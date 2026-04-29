// SPDX-License-Identifier: GPL-3.0-or-later

// STM32 name: DMA1 / DMA2 (direct memory access controller).
// STM32F427 bases: DMA1=0x40026000, DMA2=0x40026400.
// Key registers: LISR/HISR/LIFCR/HIFCR plus stream windows at 0x10 + n*0x18.
// Key behavior: stream EN, NDTR countdown, PAR/MxAR addressing, TCIF status bits.
// Critical for this emulator: firmware uses DMA completion flags and IRQs for boot/runtime.
// Current model: per-beat PINC/MINC, circular-mode NDTR reload, TC IRQ from both EN=1 and step().
// Still incomplete: FIFO thresholds, HT/TE signaling, double-buffer mode, stream arbitration.
// Datasheet/reference anchors: STM32F4 RM DMA chapter and cubeblack/STM32F4_DMA.md.

use crate::util::UniErr;
use crate::system::System;
use crate::emulator::NUM_INSTRUCTIONS;
use super::Peripheral;
use super::Peripherals;

const USART_RX_IDLE_DISABLE_DELAY: u64 = 20_000;

#[derive(Default)]
pub struct Dma {
    name: String,
    streams: [Stream; 8],
    lisr: u32,
    hisr: u32,
}

impl Dma {
    pub fn new(name: &str) -> Option<Box<dyn Peripheral>> {
        if name.starts_with("DMA") {
            let name = name.to_string();
            Some(Box::new(Self { name, ..Self::default() }))
        } else {
            None
        }
    }

    fn set_tcif(&mut self, stream: usize) {
        let bit = tcif_mask(stream);
        if stream < 4 {
            self.lisr |= bit;
        } else {
            self.hisr |= bit;
        }
    }

    fn stream_irq(&self, stream: usize) -> Option<i32> {
        // STM32F427 IRQ numbers for DMA stream interrupts.
        // DMA1 Stream0..6 -> 11..17
        // DMA2 Stream0..4 -> 56..60, Stream5..7 -> 68..70
        match self.name.as_str() {
            "DMA1" => Some(11 + stream as i32),
            "DMA2" => Some(match stream {
                0 => 56,
                1 => 57,
                2 => 58,
                3 => 59,
                4 => 60,
                5 => 68,
                6 => 69,
                7 => 70,
                _ => return None,
            }),
            _ => None,
        }
    }

    fn signal_tc(&mut self, sys: &System, stream_idx: usize) {
        self.set_tcif(stream_idx);
        if self.streams[stream_idx].tcie_enabled() {
            if let Some(irq) = self.stream_irq(stream_idx) {
                sys.p.nvic.borrow_mut().set_intr_pending(irq);
            }
        }
    }
}

impl Peripheral for Dma {
    fn step(&mut self, sys: &System) {
        let name = self.name.clone();
        for i in 0..8 {
            if self.streams[i].step_deferred(&name, sys) {
                self.signal_tc(sys, i);
            }
        }
    }

    fn read(&mut self, sys: &System, offset: u32) -> u32 {
        match Access::from_offset(offset) {
            Access::Reg(Reg::Lisr) => self.lisr,
            Access::Reg(Reg::Hisr) => self.hisr,
            Access::Reg(Reg::Lifcr) | Access::Reg(Reg::Hifcr) | Access::Reg(Reg::Unknown) => 0,
            Access::StreamReg(i, offset) => self.streams[i].read(&self.name, sys, offset),
        }
    }

    fn write(&mut self, sys: &System, offset: u32, value: u32) {
        match Access::from_offset(offset) {
            Access::Reg(Reg::Lifcr) => {
                self.lisr &= !value;
            }
            Access::Reg(Reg::Hifcr) => {
                self.hisr &= !value;
            }
            Access::Reg(_) => {}
            Access::StreamReg(i, offset) => {
                if self.streams[i].write(&self.name, sys, offset, value) {
                    self.signal_tc(sys, i);
                }
            }
        }
    }
}

#[derive(Default)]
struct Stream {
    pub cr: u32,
    pub next_cr: Option<u32>,
    pub ndtr: u32,
    pub initial_ndtr: u32,
    pub par: u32,
    pub m0ar: u32,
    pub m1ar: u32,
    pub fcr: u32,
    deferred_usart_rx: bool,
    deferred_since: u64,
}

impl Stream {
    fn tcie_enabled(&self) -> bool {
        self.cr & (1 << 4) != 0
    }

    fn channel(&self) -> u8 {
        ((self.cr >> 25) & 0b111) as u8
    }

    fn dir(&self) -> Dir {
        match (self.cr >> 6) & 0b11 {
            0b00 => Dir::Read,
            0b01 => Dir::Write,
            0b10 => Dir::MemCopy,
            _ => Dir::Invalid,
        }
    }

    fn is_circular(&self) -> bool {
        self.cr & (1 << 8) != 0
    }

    fn minc(&self) -> bool {
        self.cr & (1 << 10) != 0
    }

    fn pinc(&self) -> bool {
        self.cr & (1 << 9) != 0
    }

    // Peripheral data size in bytes (PSIZE field, SxCR bits [12:11]).
    fn psize(&self) -> usize {
        match (self.cr >> 11) & 0b11 {
            0b00 => 1,
            0b01 => 2,
            0b10 => 4,
            _ => 1,
        }
    }

    // Memory data size in bytes (MSIZE field, SxCR bits [14:13]).
    fn msize(&self) -> usize {
        match (self.cr >> 13) & 0b11 {
            0b00 => 1,
            0b01 => 2,
            0b10 => 4,
            _ => 1,
        }
    }

    fn peri_data_size(&self) -> usize {
        self.psize() * self.ndtr as usize
    }

    fn mem_data_size(&self) -> usize {
        self.msize() * self.ndtr as usize
    }

    fn data_addr(&self) -> u32 {
        if (self.cr >> 19) & 1 != 0 {
            self.m1ar
        } else {
            self.m0ar
        }
    }

    /// Perform one complete DMA transfer respecting PINC/MINC.
    /// For P2M (Read): NDTR beats, each reading psize bytes from peripheral (PINC) into
    ///   msize bytes at memory (MINC). Total bytes: psize*NDTR from peri, msize*NDTR to mem.
    /// For M2P (Write): NDTR beats, reading msize bytes from memory (MINC), writing psize
    ///   bytes to peripheral (PINC).
    /// For MemCopy: source and destination both increment by msize per beat.
    fn do_xfer(&self, name: &str, sys: &System) {
        let dir = self.dir();
        let mem_addr = self.data_addr();
        let peri_addr = self.par;
        let ndtr = self.ndtr as usize;
        let psize = self.psize();
        let msize = self.msize();
        let minc = self.minc();
        let pinc = self.pinc();

        if ndtr == 0 { return; }

        let peri = Peripherals::get_peripheral(&sys.p.peripherals, peri_addr);

        if log::log_enabled!(log::Level::Debug) {
            let peri_desc = sys.p.addr_desc(peri_addr);
            debug!("{} xfer channel={} peri_{} dir={:?} mem=0x{:08x} ndtr={} psize={} msize={} circ={} minc={} pinc={}",
                name, self.channel(), peri_desc, dir, mem_addr, ndtr,
                psize, msize, self.is_circular(), minc, pinc);
        }

        match dir {
            Dir::Read => {
                // P2M: read from peripheral into memory
                let peri_total = self.peri_data_size();
                let mem_total = self.mem_data_size();

                let mut buf = if pinc {
                    // Peripheral address increments: read psize bytes per beat with address advance
                    let mut v = std::collections::VecDeque::with_capacity(peri_total);
                    for beat in 0..ndtr {
                        let beat_offset = (peri_addr + (beat * psize) as u32) - peri_addr;
                        if let Some(p) = peri {
                            let mut slice = p.peripheral.borrow_mut().read_dma(sys, beat_offset, psize);
                            v.extend(slice.drain(..));
                        } else {
                            v.extend(std::iter::repeat(0u8).take(psize));
                        }
                    }
                    if let Some(p) = peri {
                        p.peripheral.borrow_mut().set_dma_rx_dest(mem_addr);
                    }
                    v
                } else {
                    // Fixed peripheral address (normal case: DR register)
                    let peri_offset = peri_addr - peri.map(|p| p.start).unwrap_or(peri_addr);
                    let b = peri.map(|p| {
                        p.peripheral.borrow_mut().set_dma_rx_dest(mem_addr);
                        p.peripheral.borrow_mut().read_dma(sys, peri_offset, peri_total)
                    });
                    b.unwrap_or_else(|| {
                        let mut v = std::collections::VecDeque::new();
                        v.extend(std::iter::repeat(0u8).take(peri_total));
                        v
                    })
                };

                trace!("{} xfer P2M buf_len={}", name, buf.len());

                if minc {
                    // Write sequentially to memory
                    if let Err(e) = sys.uc.borrow_mut().mem_write(mem_addr.into(), buf.make_contiguous()) {
                        warn!("DMA P2M write failed addr=0x{:08x} size={} e={}", mem_addr, mem_total, UniErr(e));
                    }
                } else {
                    // Fixed memory address: write last psize bytes repeatedly (or just write once for simplicity)
                    let bytes = buf.make_contiguous();
                    if let Err(e) = sys.uc.borrow_mut().mem_write(mem_addr.into(), &bytes[bytes.len().saturating_sub(msize)..]) {
                        warn!("DMA P2M write (MINC=0) failed addr=0x{:08x} e={}", mem_addr, UniErr(e));
                    }
                }
            }

            Dir::Write => {
                // M2P: read from memory into peripheral
                let mem_total = self.mem_data_size();
                let peri_offset = peri_addr - peri.map(|p| p.start).unwrap_or(peri_addr);

                let buf = if minc {
                    sys.uc.borrow().mem_read_as_vec(mem_addr.into(), mem_total)
                        .map_err(|e| warn!("DMA M2P read failed addr=0x{:08x} size={} e={}", mem_addr, mem_total, UniErr(e)))
                        .map(|v| v.into())
                        .ok()
                } else {
                    // Fixed memory: read one item and repeat for NDTR beats
                    sys.uc.borrow().mem_read_as_vec(mem_addr.into(), msize)
                        .ok()
                        .map(|v| {
                            let mut buf = std::collections::VecDeque::new();
                            for _ in 0..ndtr { buf.extend(v.iter()); }
                            buf
                        })
                };

                let buf = buf.unwrap_or_else(|| {
                    let mut v = std::collections::VecDeque::new();
                    v.extend(std::iter::repeat(0u8).take(mem_total));
                    v
                });

                trace!("{} xfer M2P buf_len={}", name, buf.len());

                if pinc {
                    // Peripheral address increments: write psize bytes per beat
                    let bytes: Vec<u8> = buf.into_iter().collect();
                    for beat in 0..ndtr {
                        let beat_offset = peri_offset + (beat * psize) as u32;
                        let slice = &bytes[beat*psize..(beat*psize+psize).min(bytes.len())];
                        if let Some(p) = peri {
                            p.peripheral.borrow_mut().write_dma(sys, beat_offset, slice.iter().copied().collect());
                        }
                    }
                } else if let Some(p) = peri {
                    p.peripheral.borrow_mut().write_dma(sys, peri_offset, buf);
                }
            }

            Dir::MemCopy => {
                let mem_total = self.mem_data_size();
                let src = peri_addr;
                let dst = mem_addr;

                let buf = sys.uc.borrow().mem_read_as_vec(src.into(), mem_total)
                    .map_err(|e| warn!("DMA MemCopy read failed src=0x{:08x} size={} e={}", src, mem_total, UniErr(e)))
                    .ok();

                if let Some(buf) = buf {
                    if let Err(e) = sys.uc.borrow_mut().mem_write(dst.into(), &buf) {
                        warn!("DMA MemCopy write failed dst=0x{:08x} size={} e={}", dst, mem_total, UniErr(e));
                    }
                }
            }

            Dir::Invalid => {}
        }
    }

    pub fn read(&mut self, _name: &str, _sys: &System, offset: u32) -> u32 {
        match offset {
            0x0000 => {
                let v = self.cr;
                if let Some(next_cr) = self.next_cr.take() {
                    self.cr = next_cr;
                }

                // The saturn firmware is a bit buggy. When doing a DMA write
                // with size=0, they don't enable the DMA channel, but they
                // wait for it to go to 1 and then 0, with a timeout. So they
                // are consistently hitting the timeout.
                // We'll do toggles on the ready flag to speed things up avoiding the timeout.
                if self.dir() == Dir::Write && self.ndtr == 0 {
                    self.next_cr = Some(self.cr ^ 1)
                }

                v
            }
            0x0004 => self.ndtr,
            0x0008 => self.par,
            0x000c => self.m0ar,
            0x0010 => self.m1ar,
            0x0014 => self.fcr,
            _ => 0
        }
    }

    pub fn write(&mut self, name: &str, sys: &System, offset: u32, mut value: u32) -> bool {
        match offset {
            0x0000 => {
                self.cr = value;
                self.deferred_usart_rx = false;

                if value & 1 != 0 {
                    if self.is_deferred_usart_rx(sys) {
                        self.deferred_usart_rx = true;
                        self.deferred_since = NUM_INSTRUCTIONS.load(std::sync::atomic::Ordering::Relaxed);
                        return false;
                    }

                    self.do_xfer(name, sys);

                    if self.is_circular() {
                        // Circular: reload NDTR, keep EN=1, signal TC
                        self.ndtr = self.initial_ndtr;
                    } else {
                        value &= !1;
                        self.ndtr = 0;
                        self.next_cr = Some(value);
                    }
                    return true;
                }
            }
            0x0004 => {
                self.ndtr = value & 0xFFFF;
                self.initial_ndtr = self.ndtr;
            }
            0x0008 => { self.par = value; }
            0x000c => { self.m0ar = value; }
            0x0010 => { self.m1ar = value; }
            0x0014 => { self.fcr = value; }
            _ => {}
        }

        false
    }

    fn is_deferred_usart_rx(&self, sys: &System) -> bool {
        self.dir() == Dir::Read && is_usart_dr_request(&sys.p.addr_desc(self.par))
    }

    /// Called from Dma::step(). Returns true if a transfer completed and TC should be signaled.
    fn step_deferred(&mut self, name: &str, sys: &System) -> bool {
        if !self.deferred_usart_rx || self.cr & 1 == 0 {
            return false;
        }

        let now = NUM_INSTRUCTIONS.load(std::sync::atomic::Ordering::Relaxed);
        if now.saturating_sub(self.deferred_since) < USART_RX_IDLE_DISABLE_DELAY {
            return false;
        }

        // Idle window expired: perform the transfer (reads available bytes from USART ext_device)
        // then decide based on circular mode whether to reload or finish.
        self.do_xfer(name, sys);
        self.deferred_usart_rx = false;

        if self.is_circular() {
            self.ndtr = self.initial_ndtr;
            // Restart the deferred timer so the next batch fires after another idle window
            self.deferred_usart_rx = true;
            self.deferred_since = now;
        } else {
            self.cr &= !1;
            self.ndtr = 0;
            self.next_cr = None;
        }

        true
    }
}

fn is_usart_dr_request(peri_desc: &str) -> bool {
    (peri_desc.contains("peri=USART") || peri_desc.contains("peri=UART")) && peri_desc.contains("reg=DR")
}

#[derive(Debug, PartialEq, Eq)]
enum Dir {
    Read,
    Write,
    MemCopy,
    Invalid,
}

#[derive(Debug, Clone, Copy)]
enum Reg {
    Lisr,
    Hisr,
    Lifcr,
    Hifcr,
    Unknown,
}

enum Access {
    Reg(Reg),
    StreamReg(usize, u32),
}

fn tcif_mask(stream: usize) -> u32 {
    // STM32F4 DMA status flag layout in LISR/HISR repeats for stream groups.
    // Per stream, TCIF is at bit offset +5 inside each group.
    match stream % 4 {
        0 => 1 << 5,
        1 => 1 << 11,
        2 => 1 << 21,
        3 => 1 << 27,
        _ => 0,
    }
}

impl Access {
    pub fn from_offset(offset: u32) -> Self {
        // Global DMA registers are at 0x00..0x0f (LISR/HISR/LIFCR/HIFCR).
        // Stream registers start at 0x10 and are 0x18 bytes apart.
        if offset < 0x10 {
            let reg = match offset {
                0x00 => Reg::Lisr,
                0x04 => Reg::Hisr,
                0x08 => Reg::Lifcr,
                0x0c => Reg::Hifcr,
                _ => Reg::Unknown,
            };
            Access::Reg(reg)
        } else {
            let stride = 0x18;
            let start = 0x10;

            let offset = offset - start;
            let stream = (offset / stride) as usize;
            if stream < 8 {
                Access::StreamReg(stream, offset % stride)
            } else {
                Access::Reg(Reg::Unknown)
            }
        }
    }
}
