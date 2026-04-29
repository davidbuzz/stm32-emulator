// SPDX-License-Identifier: GPL-3.0-or-later

// STM32 name: IWDG (independent watchdog).
// STM32F427 base: 0x40003000.
// Minimal model: expose KR/PR/RLR/SR with unlock/reload behavior so firmware init
// paths can proceed without full watchdog timing emulation.

use crate::system::System;

use super::Peripheral;

pub struct Iwdg {
    kr: u32,
    pr: u32,
    rlr: u32,
    sr: u32,
    write_unlocked: bool,
}

impl Iwdg {
    pub fn new(name: &str) -> Option<Box<dyn Peripheral>> {
        if name == "IWDG" {
            Some(Box::new(Self {
                kr: 0,
                pr: 0,
                rlr: 0x0FFF,
                sr: 0,
                write_unlocked: false,
            }))
        } else {
            None
        }
    }
}

impl Peripheral for Iwdg {
    fn read(&mut self, _sys: &System, offset: u32) -> u32 {
        match offset {
            0x00 => self.kr,
            0x04 => self.pr & 0x7,
            0x08 => self.rlr & 0x0FFF,
            0x0C => self.sr & 0x7,
            _ => 0,
        }
    }

    fn write(&mut self, _sys: &System, offset: u32, value: u32) {
        match offset {
            0x00 => {
                self.kr = value;
                match value {
                    0x5555 => self.write_unlocked = true,
                    0xAAAA => {
                        // Reload key accepted; no timing model in this stub.
                    }
                    0xCCCC => {
                        // Start key accepted; no reset countdown model yet.
                    }
                    _ => {}
                }
            }
            0x04 => {
                if self.write_unlocked {
                    self.pr = value & 0x7;
                    self.write_unlocked = false;
                }
            }
            0x08 => {
                if self.write_unlocked {
                    self.rlr = value & 0x0FFF;
                    self.write_unlocked = false;
                }
            }
            _ => {}
        }
    }
}
