// SPDX-License-Identifier: GPL-3.0-or-later

use crate::util::UniErr;
use crate::system::System;
use super::Peripheral;
use super::Peripherals;

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
}

impl Peripheral for Dma {
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
                    self.set_tcif(i);
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
    pub par: u32,
    pub m0ar: u32,
    pub m1ar: u32,
    pub fcr: u32,
}

impl Stream {
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

    // 1, 2, 4 (8bit, 16bit, 32bit)
    fn word_size(&self) -> usize {
        match (self.cr >> 11) & 0b11 {
            0b00 => 1,
            0b01 => 2,
            0b10 => 4,
            _ => 1,
        }
    }

    fn data_size(&self) -> usize {
        self.word_size() * self.ndtr as usize
    }

    fn data_addr(&self) -> u32 {
        if (self.cr >> 19) & 1 != 0 {
            self.m1ar
        } else {
            self.m0ar
        }
    }

    fn do_xfer(&self, name: &str, sys: &System) {
        let dir = self.dir();
        let data_addr = self.data_addr();
        let size = self.data_size();
        let peri_addr = self.par;

        let peri = Peripherals::get_peripheral(&sys.p.peripherals, peri_addr);

        let (src, dst) = match dir {
            Dir::Read => (peri_addr, data_addr),
            Dir::Write => (data_addr, peri_addr),
            Dir::MemCopy => (peri_addr, data_addr),
            Dir::Invalid => (0,0),
        };

        if log::log_enabled!(log::Level::Debug) {
            let peri_desc = sys.p.addr_desc(peri_addr);
            debug!("{} xfer initiated channel={} peri_{} dir={:?} addr=0x{:08x} size={}",
                name, self.channel(), peri_desc, dir, data_addr, size);
        }

        let buf = match dir {
            Dir::Read => {
                peri.map(|p| p.peripheral.borrow_mut().read_dma(sys, peri_addr-p.start, size))
            }
            Dir::Write | Dir::MemCopy => {
                sys.uc.borrow().mem_read_as_vec(src.into(), size)
                    .map_err(|e| warn!("DMA read failed addr=0x{:08x} size={} e={}", src, size, UniErr(e)))
                    .map(|v| v.into())
                    .ok()
            }
            Dir::Invalid => Some(vec![].into()),
        };

        let mut buf = buf.unwrap_or_else(|| {
            let mut rx = vec![];
            rx.resize(size, 0);
            rx.into()
        });

        trace!("{} xfer buf={:x?}", name, buf);

        match dir {
            Dir::Write => {
                peri.map(|p| p.peripheral.borrow_mut().write_dma(sys, peri_addr-p.start, buf));
            }
            Dir::Read | Dir::MemCopy => {
                if let Err(e) = sys.uc.borrow_mut().mem_write(dst.into(), buf.make_contiguous()) {
                    warn!("DMA read failed addr=0x{:08x} size={} e={}", dst, size, UniErr(e));
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
                if self.dir() == Dir::Write && self.data_size() == 0 {
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

                // CRx register
                if value & 1 != 0 {
                    // Enable is on. do the transfer.
                    self.do_xfer(name, sys);

                    value &= !1;
                    self.ndtr = 0;
                    self.next_cr = Some(value);
                    return true;
                }
            }
            0x0004 => { self.ndtr = value & 0xFFFF; }
            0x0008 => { self.par = value; }
            0x000c => { self.m0ar = value; }
            0x0010 => { self.m1ar = value; }
            0x0014 => { self.fcr = value; }
            _ => {}
        }

        false
    }
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
    /// CR0, CR1, etc.
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
