// SPDX-License-Identifier: GPL-3.0-or-later

// STM32 name: CRC (cyclic redundancy check calculation unit).
// STM32F427 base: 0x40023000.
// Key registers: DR, IDR, CR.
// This model is intentionally minimal: it provides register presence, reset behavior,
// and a deterministic software CRC32 update on DR writes.

use crate::system::System;

use super::Peripheral;

const CRC_CR_RESET: u32 = 1 << 0;

pub struct Crc {
    dr: u32,
    idr: u32,
    cr: u32,
}

impl Crc {
    pub fn new(name: &str) -> Option<Box<dyn Peripheral>> {
        if name == "CRC" {
            Some(Box::new(Self {
                dr: 0xFFFF_FFFF,
                idr: 0,
                cr: 0,
            }))
        } else {
            None
        }
    }

    fn update_crc_word(mut crc: u32, data: u32) -> u32 {
        // Standard STM32 CRC polynomial (0x04C11DB7), MSB-first over 32 bits.
        let mut x = data;
        for _ in 0..32 {
            let bit = ((x >> 31) ^ (crc >> 31)) & 1;
            crc <<= 1;
            if bit != 0 {
                crc ^= 0x04C11DB7;
            }
            x <<= 1;
        }
        crc
    }
}

impl Peripheral for Crc {
    fn read(&mut self, _sys: &System, offset: u32) -> u32 {
        match offset {
            0x00 => self.dr,
            0x04 => self.idr & 0xFF,
            0x08 => self.cr,
            _ => 0,
        }
    }

    fn write(&mut self, _sys: &System, offset: u32, value: u32) {
        match offset {
            0x00 => {
                self.dr = Self::update_crc_word(self.dr, value);
            }
            0x04 => self.idr = value & 0xFF,
            0x08 => {
                self.cr = value & CRC_CR_RESET;
                if value & CRC_CR_RESET != 0 {
                    self.dr = 0xFFFF_FFFF;
                }
            }
            _ => {}
        }
    }
}
