// SPDX-License-Identifier: GPL-3.0-or-later

// STM32 name: FLASH (Flash interface registers).
// STM32F427 base: 0x40023c00.
// Key registers: ACR, KEYR, OPTKEYR, SR, CR, OPTCR.
// Key function: flash wait states, caches, erase/program control, and option bytes.
// Critical for this emulator: startup code polls ACR latency after programming wait states.
// The current model is minimal and only needs to preserve register writes for firmware polling.

use crate::system::System;
use super::Peripheral;

pub struct Flash {
    acr: u32,
    keyr: u32,
    optkeyr: u32,
    sr: u32,
    cr: u32,
    optcr: u32,
}

impl Flash {
    pub fn new(name: &str) -> Option<Box<dyn Peripheral>> {
        if name == "FLASH" {
            Some(Box::new(Flash {
                acr: 0,
                keyr: 0,
                optkeyr: 0,
                sr: 0,
                cr: 0x8000_0000,
                optcr: 0x0fff_aaed,
            }))
        } else {
            None
        }
    }
}

impl Peripheral for Flash {
    fn read(&mut self, _sys: &System, offset: u32) -> u32 {
        match offset {
            0x00 => self.acr,
            0x04 => self.keyr,
            0x08 => self.optkeyr,
            0x0c => self.sr,
            0x10 => self.cr,
            0x14 => self.optcr,
            _ => 0,
        }
    }

    fn write(&mut self, _sys: &System, offset: u32, value: u32) {
        match offset {
            0x00 => self.acr = value,
            0x04 => self.keyr = value,
            0x08 => self.optkeyr = value,
            0x0c => self.sr &= !value,
            0x10 => self.cr = value,
            0x14 => self.optcr = value,
            _ => {}
        }
    }
}