// SPDX-License-Identifier: GPL-3.0-or-later

// STM32 name: DMA1 / DMA2 (direct memory access controller).
// STM32F427 bases: DMA1=0x40026000, DMA2=0x40026400.
// Key registers: LISR/HISR/LIFCR/HIFCR plus stream windows at 0x10 + n*0x18.
// Key behavior: stream EN semantics, NDTR countdown, PAR/MxAR addressing, status bits.
// Critical for this emulator: firmware uses DMA completion flags and IRQs for boot/runtime.
// Current model: per-beat PINC/MINC, circular-mode NDTR reload, EN retrigger guard,
// and TC/HT/TE/DME/FE class flags.
// Still incomplete: FIFO thresholds/behavioral depth, double-buffer mode details, stream arbitration.
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

    fn set_htif(&mut self, stream: usize) {
        let bit = htif_mask(stream);
        if stream < 4 {
            self.lisr |= bit;
        } else {
            self.hisr |= bit;
        }
    }

    fn set_teif(&mut self, stream: usize) {
        let bit = teif_mask(stream);
        if stream < 4 {
            self.lisr |= bit;
        } else {
            self.hisr |= bit;
        }
    }

    fn set_dmeif(&mut self, stream: usize) {
        let bit = dmeif_mask(stream);
        if stream < 4 {
            self.lisr |= bit;
        } else {
            self.hisr |= bit;
        }
    }

    fn set_feif(&mut self, stream: usize) {
        let bit = feif_mask(stream);
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

    fn signal_ht(&mut self, sys: &System, stream_idx: usize) {
        self.set_htif(stream_idx);
        if self.streams[stream_idx].htie_enabled() {
            if let Some(irq) = self.stream_irq(stream_idx) {
                sys.p.nvic.borrow_mut().set_intr_pending(irq);
            }
        }
    }

    fn signal_te(&mut self, sys: &System, stream_idx: usize) {
        self.set_teif(stream_idx);
        if self.streams[stream_idx].teie_enabled() {
            if let Some(irq) = self.stream_irq(stream_idx) {
                sys.p.nvic.borrow_mut().set_intr_pending(irq);
            }
        }
    }

    fn signal_dme(&mut self, sys: &System, stream_idx: usize) {
        self.set_dmeif(stream_idx);
        if self.streams[stream_idx].dmeie_enabled() {
            if let Some(irq) = self.stream_irq(stream_idx) {
                sys.p.nvic.borrow_mut().set_intr_pending(irq);
            }
        }
    }

    fn signal_fe(&mut self, sys: &System, stream_idx: usize) {
        self.set_feif(stream_idx);
        if self.streams[stream_idx].feie_enabled() {
            if let Some(irq) = self.stream_irq(stream_idx) {
                sys.p.nvic.borrow_mut().set_intr_pending(irq);
            }
        }
    }

    fn signal_mode_error(&mut self, sys: &System, stream_idx: usize) {
        // TE is the generic transfer-error class. DME/FE are mode-specific:
        // direct mode uses DME, FIFO mode uses FE.
        self.signal_te(sys, stream_idx);
        if self.streams[stream_idx].fifo_enabled() {
            self.signal_fe(sys, stream_idx);
        } else {
            self.signal_dme(sys, stream_idx);
        }
    }

    fn find_request_conflict(&self, stream_idx: usize, channel: u8, par: u32) -> Option<usize> {
        self.streams.iter().enumerate().find_map(|(idx, s)| {
            if idx != stream_idx
                && (s.cr & 1) != 0
                && s.channel() == channel
                && s.par == par
            {
                Some(idx)
            } else {
                None
            }
        })
    }
}

impl Peripheral for Dma {
    fn step(&mut self, sys: &System) {
        let name = self.name.clone();
        for i in 0..8 {
            match self.streams[i].step_deferred(&name, i, sys) {
                StreamStepResult::Completed { half } => {
                    if half {
                        self.signal_ht(sys, i);
                    }
                    self.signal_tc(sys, i);
                }
                StreamStepResult::TransferError => {
                    self.signal_mode_error(sys, i);
                }
                StreamStepResult::Noop => {}
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
                if offset == 0x0000 && (value & 1) != 0 {
                    let channel = ((value >> 25) & 0b111) as u8;
                    let par = self.streams[i].par;
                    if let Some(owner) = self.find_request_conflict(i, channel, par) {
                        // Allow a read/write stream pair to share the same peripheral request.
                        // This is the normal SPI full-duplex DMA pattern (RX + TX streams).
                        let new_dir = match (value >> 6) & 0b11 {
                            0b00 => Dir::Read,
                            0b01 => Dir::Write,
                            0b10 => Dir::MemCopy,
                            _ => Dir::Invalid,
                        };
                        let owner_dir = self.streams[owner].dir();
                        let full_duplex_pair =
                            (new_dir == Dir::Read && owner_dir == Dir::Write)
                            || (new_dir == Dir::Write && owner_dir == Dir::Read);
                        if full_duplex_pair {
                            self.streams[i].write(&self.name, i, sys, offset, value);
                            return;
                        }

                        let new_pl = ((value >> 16) & 0b11) as u8;
                        let owner_pl = self.streams[owner].priority();

                        if new_pl > owner_pl {
                            self.streams[owner].cr &= !1;
                            self.streams[owner].deferred_usart_rx = false;
                            debug!(
                                "{} stream={} preempted stream={} on conflicting request channel={} par=0x{:08x} (pl {} > {})",
                                self.name,
                                i,
                                owner,
                                channel,
                                par,
                                new_pl,
                                owner_pl
                            );
                        } else {
                            self.signal_mode_error(sys, i);
                            debug!(
                                "{} stream={} blocked conflicting request channel={} par=0x{:08x} (pl {} <= {})",
                                self.name,
                                i,
                                channel,
                                par,
                                new_pl,
                                owner_pl
                            );
                            return;
                        }
                    }
                }

                match self.streams[i].write(&self.name, i, sys, offset, value) {
                    StreamWriteResult::Completed { half } => {
                        if half {
                            self.signal_ht(sys, i);
                        }
                        self.signal_tc(sys, i);
                    }
                    StreamWriteResult::TransferError => {
                        self.signal_mode_error(sys, i);
                    }
                    StreamWriteResult::Noop => {}
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

    fn htie_enabled(&self) -> bool {
        self.cr & (1 << 3) != 0
    }

    fn teie_enabled(&self) -> bool {
        self.cr & (1 << 2) != 0
    }

    fn dmeie_enabled(&self) -> bool {
        self.cr & (1 << 1) != 0
    }

    fn feie_enabled(&self) -> bool {
        self.fcr & (1 << 7) != 0
    }

    fn fifo_enabled(&self) -> bool {
        self.fcr & (1 << 2) != 0
    }

    fn channel(&self) -> u8 {
        ((self.cr >> 25) & 0b111) as u8
    }

    fn priority(&self) -> u8 {
        ((self.cr >> 16) & 0b11) as u8
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

    fn is_double_buffer(&self) -> bool {
        self.cr & (1 << 18) != 0
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

    fn data_addr(&self) -> u32 {
        if (self.cr >> 19) & 1 != 0 {
            self.m1ar
        } else {
            self.m0ar
        }
    }

    fn fifo_threshold_words(&self) -> usize {
        // FTH (SxFCR bits [1:0]): 1/4, 1/2, 3/4, full of 4-word FIFO.
        match self.fcr & 0b11 {
            0b00 => 1,
            0b01 => 2,
            0b10 => 3,
            0b11 => 4,
            _ => 1,
        }
    }

    fn transfer_beats_per_chunk(&self) -> usize {
        if !self.fifo_enabled() {
            // Direct mode: effectively one beat per request window.
            return 1;
        }

        // FIFO mode: transfer one FIFO threshold worth of beats at a time.
        // FIFO is 4 words = 16 bytes.
        let fifo_bytes = self.fifo_threshold_words() * 4;
        let beat_bytes = std::cmp::max(self.psize(), self.msize());
        std::cmp::max(1, fifo_bytes / std::cmp::max(1, beat_bytes))
    }

    /// Perform one complete DMA transfer respecting PINC/MINC.
    /// For P2M (Read): NDTR beats, each reading psize bytes from peripheral (PINC) into
    ///   msize bytes at memory (MINC). Total bytes: psize*NDTR from peri, msize*NDTR to mem.
    /// For M2P (Write): NDTR beats, reading msize bytes from memory (MINC), writing psize
    ///   bytes to peripheral (PINC).
    /// For MemCopy: source and destination both increment by msize per beat.
    fn do_xfer(&self, dma_name: &str, stream_idx: usize, sys: &System) -> bool {
        let dir = self.dir();
        let mut mem_addr = self.data_addr();
        let mut peri_addr = self.par;
        let ndtr = self.ndtr as usize;
        let psize = self.psize();
        let msize = self.msize();
        let minc = self.minc();
        let pinc = self.pinc();

        if ndtr == 0 { return false; }

        let peri = Peripherals::get_peripheral(&sys.p.peripherals, peri_addr);
        let peri_desc = sys.p.addr_desc(peri_addr);
        let peri_name = peripheral_name_from_desc(&peri_desc);

        if !request_mapping_allows(dma_name, stream_idx as u8, self.channel(), peri_name, dir) {
            warn!(
                "{} stream={} channel={} blocked request mapping for {} dir={:?}",
                dma_name,
                stream_idx,
                self.channel(),
                peri_desc,
                dir
            );
            return false;
        }

        let mut ok = true;
        let chunk_beats = self.transfer_beats_per_chunk();

        if log::log_enabled!(log::Level::Debug) {
            debug!("{} xfer channel={} peri_{} dir={:?} mem=0x{:08x} ndtr={} psize={} msize={} circ={} minc={} pinc={} fifo={} fth_words={} chunk_beats={}",
                dma_name, self.channel(), peri_desc, dir, mem_addr, ndtr,
                psize, msize, self.is_circular(), minc, pinc, self.fifo_enabled(), self.fifo_threshold_words(), chunk_beats);
        }

        let mut remaining_beats = ndtr;
        while remaining_beats > 0 {
            let beats = std::cmp::min(remaining_beats, chunk_beats);
            let peri_total = psize * beats;
            let mem_total = msize * beats;

            match dir {
                Dir::Read => {
                    // P2M: read from peripheral into memory
                    let mut buf = if pinc {
                        let mut v = std::collections::VecDeque::with_capacity(peri_total);
                        for beat in 0..beats {
                            let beat_addr = peri_addr + (beat * psize) as u32;
                            let beat_offset = beat_addr - peri_addr;
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

                    if minc {
                        if let Err(e) = sys.uc.borrow_mut().mem_write(mem_addr.into(), buf.make_contiguous()) {
                            warn!("DMA P2M write failed addr=0x{:08x} size={} e={}", mem_addr, mem_total, UniErr(e));
                            ok = false;
                        }
                    } else {
                        let bytes = buf.make_contiguous();
                        if let Err(e) = sys.uc.borrow_mut().mem_write(mem_addr.into(), &bytes[bytes.len().saturating_sub(msize)..]) {
                            warn!("DMA P2M write (MINC=0) failed addr=0x{:08x} e={}", mem_addr, UniErr(e));
                            ok = false;
                        }
                    }
                }

                Dir::Write => {
                    // M2P: read from memory into peripheral
                    let peri_offset = peri_addr - peri.map(|p| p.start).unwrap_or(peri_addr);

                    let buf = if minc {
                        sys.uc.borrow().mem_read_as_vec(mem_addr.into(), mem_total)
                            .map_err(|e| warn!("DMA M2P read failed addr=0x{:08x} size={} e={}", mem_addr, mem_total, UniErr(e)))
                            .map(|v| v.into())
                            .ok()
                    } else {
                        sys.uc.borrow().mem_read_as_vec(mem_addr.into(), msize)
                            .ok()
                            .map(|v| {
                                let mut out = std::collections::VecDeque::new();
                                for _ in 0..beats {
                                    out.extend(v.iter());
                                }
                                out
                            })
                    };

                    let buf = buf.unwrap_or_else(|| {
                        ok = false;
                        let mut v = std::collections::VecDeque::new();
                        v.extend(std::iter::repeat(0u8).take(mem_total));
                        v
                    });

                    if pinc {
                        let bytes: Vec<u8> = buf.into_iter().collect();
                        for beat in 0..beats {
                            let beat_offset = peri_offset + (beat * psize) as u32;
                            let start = beat * psize;
                            let end = (start + psize).min(bytes.len());
                            let slice = &bytes[start..end];
                            if let Some(p) = peri {
                                p.peripheral.borrow_mut().write_dma(sys, beat_offset, slice.iter().copied().collect());
                            } else {
                                ok = false;
                            }
                        }
                    } else if let Some(p) = peri {
                        p.peripheral.borrow_mut().write_dma(sys, peri_offset, buf);
                    } else {
                        ok = false;
                    }
                }

                Dir::MemCopy => {
                    let src = peri_addr;
                    let dst = mem_addr;

                    let buf = sys.uc.borrow().mem_read_as_vec(src.into(), mem_total)
                        .map_err(|e| warn!("DMA MemCopy read failed src=0x{:08x} size={} e={}", src, mem_total, UniErr(e)))
                        .ok();

                    if let Some(buf) = buf {
                        if let Err(e) = sys.uc.borrow_mut().mem_write(dst.into(), &buf) {
                            warn!("DMA MemCopy write failed dst=0x{:08x} size={} e={}", dst, mem_total, UniErr(e));
                            ok = false;
                        }
                    } else {
                        ok = false;
                    }
                }

                Dir::Invalid => {}
            }

            if minc {
                mem_addr = mem_addr.wrapping_add((beats * msize) as u32);
            }
            if pinc {
                peri_addr = peri_addr.wrapping_add((beats * psize) as u32);
            }

            remaining_beats -= beats;
        }

        ok
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

    pub fn write(&mut self, dma_name: &str, stream_idx: usize, sys: &System, offset: u32, mut value: u32) -> StreamWriteResult {
        match offset {
            0x0000 => {
                let was_enabled = self.cr & 1 != 0;
                self.cr = value;

                // EN set while already enabled should not retrigger a new transfer.
                if (value & 1) != 0 && was_enabled {
                    return StreamWriteResult::Noop;
                }

                // EN clear requests disable; keep it simple for now by canceling deferred RX window.
                if (value & 1) == 0 {
                    self.deferred_usart_rx = false;
                    return StreamWriteResult::Noop;
                }

                self.deferred_usart_rx = false;

                if self.is_deferred_usart_rx(sys) {
                    self.deferred_usart_rx = true;
                    self.deferred_since = NUM_INSTRUCTIONS.load(std::sync::atomic::Ordering::Relaxed);
                    return StreamWriteResult::Noop;
                }

                let ok = self.do_xfer(dma_name, stream_idx, sys);
                let half = self.ndtr > 1;

                if self.is_double_buffer() {
                    // DBM: toggle CT (bit 19) to switch between M0AR and M1AR, reload NDTR
                    self.cr ^= 1 << 19;
                    self.ndtr = self.initial_ndtr;
                } else if self.is_circular() {
                    // Circular without DBM: reload NDTR, keep same buffer
                    self.ndtr = self.initial_ndtr;
                } else {
                    value &= !1;
                    self.ndtr = 0;
                    self.next_cr = Some(value);
                }
                if ok {
                    return StreamWriteResult::Completed { half };
                }
                return StreamWriteResult::TransferError;
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

        StreamWriteResult::Noop
    }

    fn is_deferred_usart_rx(&self, sys: &System) -> bool {
        self.dir() == Dir::Read && is_usart_dr_request(&sys.p.addr_desc(self.par))
    }

    /// Called from Dma::step(). Returns true if a transfer completed and TC should be signaled.
    fn step_deferred(&mut self, dma_name: &str, stream_idx: usize, sys: &System) -> StreamStepResult {
        if !self.deferred_usart_rx || self.cr & 1 == 0 {
            return StreamStepResult::Noop;
        }

        let now = NUM_INSTRUCTIONS.load(std::sync::atomic::Ordering::Relaxed);
        if now.saturating_sub(self.deferred_since) < USART_RX_IDLE_DISABLE_DELAY {
            return StreamStepResult::Noop;
        }

        // Idle window expired: perform the transfer (reads available bytes from USART ext_device)
        // then decide based on circular mode whether to reload or finish.
        let ok = self.do_xfer(dma_name, stream_idx, sys);
        let half = self.ndtr > 1;
        self.deferred_usart_rx = false;

        if self.is_double_buffer() {
            self.cr ^= 1 << 19;
            self.ndtr = self.initial_ndtr;
            self.deferred_usart_rx = true;
            self.deferred_since = now;
        } else if self.is_circular() {
            self.ndtr = self.initial_ndtr;
            self.deferred_usart_rx = true;
            self.deferred_since = now;
        } else {
            self.cr &= !1;
            self.ndtr = 0;
            self.next_cr = None;
        }

        if ok {
            StreamStepResult::Completed { half }
        } else {
            StreamStepResult::TransferError
        }
    }
}

enum StreamWriteResult {
    Noop,
    Completed { half: bool },
    TransferError,
}

enum StreamStepResult {
    Noop,
    Completed { half: bool },
    TransferError,
}

fn is_usart_dr_request(peri_desc: &str) -> bool {
    (peri_desc.contains("peri=USART") || peri_desc.contains("peri=UART")) && peri_desc.contains("reg=DR")
}

fn peripheral_name_from_desc(desc: &str) -> Option<&str> {
    let (_, tail) = desc.split_once("peri=")?;
    Some(tail.split_whitespace().next().unwrap_or_default())
}

fn request_mapping_allows(
    dma_name: &str,
    stream: u8,
    channel: u8,
    peri_name: Option<&str>,
    dir: Dir,
) -> bool {
    let Some(peri_name) = peri_name else {
        return true;
    };

    let req = match (peri_name, dir) {
        ("ADC1", Dir::Read) => Some(&[("DMA2", 0u8, 0u8), ("DMA2", 4, 0)][..]),
        ("ADC2", Dir::Read) => Some(&[("DMA2", 2u8, 1u8), ("DMA2", 3, 1)][..]),
        ("ADC3", Dir::Read) => Some(&[("DMA2", 0u8, 2u8), ("DMA2", 1, 2)][..]),

        ("SPI1", Dir::Read) => Some(&[("DMA2", 0u8, 3u8), ("DMA2", 2, 3)][..]),
        ("SPI1", Dir::Write) => Some(&[("DMA2", 3u8, 3u8), ("DMA2", 5, 3)][..]),
        ("SPI2", Dir::Read) => Some(&[("DMA1", 3u8, 0u8)][..]),
        ("SPI2", Dir::Write) => Some(&[("DMA1", 4u8, 0u8)][..]),
        ("SPI3", Dir::Read) => Some(&[("DMA1", 0u8, 0u8), ("DMA1", 2, 0)][..]),
        ("SPI3", Dir::Write) => Some(&[("DMA1", 5u8, 0u8), ("DMA1", 7, 0)][..]),

        ("USART1", Dir::Read) => Some(&[("DMA2", 2u8, 4u8), ("DMA2", 5, 4)][..]),
        ("USART1", Dir::Write) => Some(&[("DMA2", 7u8, 4u8)][..]),
        ("USART2", Dir::Read) => Some(&[("DMA1", 5u8, 4u8)][..]),
        ("USART2", Dir::Write) => Some(&[("DMA1", 6u8, 4u8)][..]),
        ("USART3", Dir::Read) => Some(&[("DMA1", 1u8, 4u8)][..]),
        ("USART3", Dir::Write) => Some(&[("DMA1", 3u8, 4u8), ("DMA1", 4, 7)][..]),
        ("UART4", Dir::Read) => Some(&[("DMA1", 2u8, 4u8)][..]),
        ("UART4", Dir::Write) => Some(&[("DMA1", 4u8, 4u8)][..]),
        ("UART5", Dir::Read) => Some(&[("DMA1", 0u8, 4u8)][..]),
        ("UART5", Dir::Write) => Some(&[("DMA1", 7u8, 4u8)][..]),
        ("USART6", Dir::Read) => Some(&[("DMA2", 1u8, 5u8), ("DMA2", 2, 5)][..]),
        ("USART6", Dir::Write) => Some(&[("DMA2", 6u8, 5u8), ("DMA2", 7, 5)][..]),

        ("I2C1", Dir::Read) => Some(&[("DMA1", 0u8, 1u8), ("DMA1", 5, 1)][..]),
        ("I2C1", Dir::Write) => Some(&[("DMA1", 6u8, 1u8), ("DMA1", 7, 1)][..]),
        ("I2C2", Dir::Read) => Some(&[("DMA1", 2u8, 7u8), ("DMA1", 3, 7)][..]),
        ("I2C2", Dir::Write) => Some(&[("DMA1", 7u8, 7u8)][..]),
        ("I2C3", Dir::Read) => Some(&[("DMA1", 2u8, 3u8)][..]),
        ("I2C3", Dir::Write) => Some(&[("DMA1", 4u8, 3u8)][..]),

        // SDIO can be used in both directions through the same request entries.
        ("SDIO", _) => Some(&[("DMA2", 3u8, 4u8), ("DMA2", 6, 4)][..]),

        // Keep unsupported peripheral request IDs permissive for now.
        _ => None,
    };

    req.map(|entries| {
        entries.iter().any(|(dma, s, ch)| dma_name == *dma && stream == *s && channel == *ch)
    }).unwrap_or(true)
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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

fn htif_mask(stream: usize) -> u32 {
    match stream % 4 {
        0 => 1 << 4,
        1 => 1 << 10,
        2 => 1 << 20,
        3 => 1 << 26,
        _ => 0,
    }
}

fn teif_mask(stream: usize) -> u32 {
    match stream % 4 {
        0 => 1 << 3,
        1 => 1 << 9,
        2 => 1 << 19,
        3 => 1 << 25,
        _ => 0,
    }
}

fn dmeif_mask(stream: usize) -> u32 {
    match stream % 4 {
        0 => 1 << 2,
        1 => 1 << 8,
        2 => 1 << 18,
        3 => 1 << 24,
        _ => 0,
    }
}

fn feif_mask(stream: usize) -> u32 {
    match stream % 4 {
        0 => 1 << 0,
        1 => 1 << 6,
        2 => 1 << 16,
        3 => 1 << 22,
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
