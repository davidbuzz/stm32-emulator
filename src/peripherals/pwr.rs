// SPDX-License-Identifier: GPL-3.0-or-later

// STM32 name: PWR (Power control).
// STM32F427 base: 0x40007000.
// Key registers: CR and CSR.
// Key function: voltage scaling, backup-domain access, and over-drive readiness status.
// Critical for this emulator: ChibiOS polls CSR ready bits during early clock setup.
// The current model is intentionally minimal and reports regulator transitions as immediate.

use crate::system::System;
use super::Peripheral;

pub struct Pwr {
    cr: u32,
    csr: u32,
}

impl Pwr {
    pub fn new(name: &str) -> Option<Box<dyn Peripheral>> {
        if name == "PWR" {
            let mut pwr = Pwr {
                cr: 0,
                csr: 0,
            };
            pwr.update_ready_bits();
            Some(Box::new(pwr))
        } else {
            None
        }
    }

    fn update_ready_bits(&mut self) {
        const DBP: u32 = 1 << 8;
        const BRE: u32 = 1 << 9;
        const EWUP_MASK: u32 = 0x7 << 8;
        const BRR: u32 = 1 << 3;
        const VOSRDY: u32 = 1 << 14;
        const ODEN: u32 = 1 << 16;
        const ODSWEN: u32 = 1 << 17;
        const ODRDY: u32 = 1 << 16;
        const ODSWRDY: u32 = 1 << 17;

        self.csr |= VOSRDY;

        if self.cr & ODEN != 0 {
            self.csr |= ODRDY;
        } else {
            self.csr &= !ODRDY;
        }

        if self.cr & ODSWEN != 0 {
            self.csr |= ODSWRDY;
        } else {
            self.csr &= !ODSWRDY;
        }

        if self.cr & DBP != 0 {
            self.csr |= DBP;
        } else {
            self.csr &= !DBP;
        }

        if self.csr & BRE != 0 {
            self.csr |= BRR;
        } else {
            self.csr &= !BRR;
        }

        self.csr = (self.csr & !EWUP_MASK) | (self.cr & EWUP_MASK);
    }
}

impl Peripheral for Pwr {
    fn read(&mut self, _sys: &System, offset: u32) -> u32 {
        match offset {
            0x00 => self.cr,
            0x04 => self.csr,
            _ => 0,
        }
    }

    fn write(&mut self, _sys: &System, offset: u32, value: u32) {
        match offset {
            0x00 => {
                self.cr = value;
                self.update_ready_bits();
            }
            0x04 => {
                let writable_mask = (1 << 3) | (0x7 << 8);
                self.csr = (self.csr & !writable_mask) | (value & writable_mask);
                self.update_ready_bits();
            }
            _ => {}
        }
    }
}