// SPDX-License-Identifier: GPL-3.0-or-later

// STM32 name: RTC (real-time clock).
// STM32F427 base: 0x40002800.
// Minimal model: register storage with write-protection gate for core setup paths.

use crate::system::System;

use super::Peripheral;

pub struct Rtc {
    regs: [u32; 0x24],
    wpr_unlocked: bool,
    wpr_stage: u8,
}

impl Rtc {
    pub fn new(name: &str) -> Option<Box<dyn Peripheral>> {
        if name == "RTC" {
            let mut rtc = Self {
                regs: [0; 0x24],
                wpr_unlocked: false,
                wpr_stage: 0,
            };
            // ISR reset-like state with INITF clear and RSF set approximation.
            rtc.regs[0x0C / 4] = 1 << 5;
            Some(Box::new(rtc))
        } else {
            None
        }
    }
}

impl Peripheral for Rtc {
    fn read(&mut self, _sys: &System, offset: u32) -> u32 {
        let idx = (offset / 4) as usize;
        self.regs.get(idx).copied().unwrap_or(0)
    }

    fn write(&mut self, _sys: &System, offset: u32, value: u32) {
        // WPR is at 0x24 and is byte-wide; handle it separately.
        if offset == 0x24 {
            match value & 0xFF {
                0xCA => {
                    self.wpr_unlocked = false;
                    self.wpr_stage = 1;
                }
                0x53 if self.wpr_stage == 1 => {
                    self.wpr_unlocked = true;
                    self.wpr_stage = 0;
                }
                _ => {
                    self.wpr_unlocked = false;
                    self.wpr_stage = 0;
                }
            }
            return;
        }

        let idx = (offset / 4) as usize;
        if idx >= self.regs.len() {
            return;
        }

        // Backup registers are writable regardless of WPR in this stub. Core regs require unlock.
        let is_bkp = (0x50..=0x9C).contains(&offset);
        if self.wpr_unlocked || is_bkp {
            self.regs[idx] = value;
        }
    }
}
