// SPDX-License-Identifier: GPL-3.0-or-later

// STM32 name: RTC (real-time clock).
// STM32F427 base: 0x40002800.
// Minimal model: register storage with write-protection gate for core setup paths.

use crate::system::System;

use super::Peripheral;

const RTC_TR_OFFSET: usize = 0x00 / 4;
const RTC_DR_OFFSET: usize = 0x04 / 4;
const RTC_ISR_OFFSET: usize = 0x0C / 4;

const RTC_ISR_RSF: u32 = 1 << 5;
const RTC_ISR_INITF: u32 = 1 << 6;
const RTC_ISR_INIT: u32 = 1 << 7;

// Coarse LSI/LSE-like pacing for calendar progression.
const RTC_TICK_STEPS: u32 = 32_768;

pub struct Rtc {
    regs: [u32; 0x24],
    wpr_unlocked: bool,
    wpr_stage: u8,
    tick_accum: u32,
    rsf_resync_delay: u8,
}

impl Rtc {
    pub fn new(name: &str) -> Option<Box<dyn Peripheral>> {
        if name == "RTC" {
            let mut rtc = Self {
                regs: [0; 0x24],
                wpr_unlocked: false,
                wpr_stage: 0,
                tick_accum: 0,
                rsf_resync_delay: 0,
            };
            // ISR reset-like state with INITF clear and RSF set approximation.
            rtc.regs[RTC_ISR_OFFSET] = RTC_ISR_RSF;
            // Default date/time: 2000-01-01 00:00:00 (weekday=1)
            rtc.regs[RTC_TR_OFFSET] = 0;
            rtc.regs[RTC_DR_OFFSET] = 0x0000_2101;
            Some(Box::new(rtc))
        } else {
            None
        }
    }

    fn bcd_get(value: u32, ones_shift: u32, tens_shift: u32, tens_mask: u32) -> u32 {
        let ones = (value >> ones_shift) & 0xF;
        let tens = (value >> tens_shift) & tens_mask;
        tens * 10 + ones
    }

    fn bcd_set(dst: &mut u32, num: u32, ones_shift: u32, tens_shift: u32, tens_mask: u32) {
        let ones = num % 10;
        let tens = (num / 10) & tens_mask;
        let ones_mask = 0xF << ones_shift;
        let tens_mask_bits = tens_mask << tens_shift;
        *dst &= !(ones_mask | tens_mask_bits);
        *dst |= (ones << ones_shift) | (tens << tens_shift);
    }

    fn is_leap_year(year: u32) -> bool {
        // RTC stores year as two digits; use modulo-4 approximation for leap years.
        year % 4 == 0
    }

    fn days_in_month(month: u32, year: u32) -> u32 {
        match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => {
                if Self::is_leap_year(year) { 29 } else { 28 }
            }
            _ => 30,
        }
    }

    fn increment_calendar_second(&mut self) {
        let mut tr = self.regs[RTC_TR_OFFSET];
        let mut dr = self.regs[RTC_DR_OFFSET];

        let mut sec = Self::bcd_get(tr, 0, 4, 0x7);
        let mut min = Self::bcd_get(tr, 8, 12, 0x7);
        let mut hour = Self::bcd_get(tr, 16, 20, 0x3);

        let mut day = Self::bcd_get(dr, 0, 4, 0x3).max(1);
        let mut month = Self::bcd_get(dr, 8, 12, 0x1).clamp(1, 12);
        let mut year = Self::bcd_get(dr, 16, 20, 0xF);
        let mut weekday = ((dr >> 13) & 0x7).max(1);

        sec += 1;
        if sec >= 60 {
            sec = 0;
            min += 1;
            if min >= 60 {
                min = 0;
                hour += 1;
                if hour >= 24 {
                    hour = 0;
                    day += 1;
                    weekday = if weekday >= 7 { 1 } else { weekday + 1 };
                    let dim = Self::days_in_month(month, year);
                    if day > dim {
                        day = 1;
                        month += 1;
                        if month > 12 {
                            month = 1;
                            year = (year + 1) % 100;
                        }
                    }
                }
            }
        }

        Self::bcd_set(&mut tr, sec, 0, 4, 0x7);
        Self::bcd_set(&mut tr, min, 8, 12, 0x7);
        Self::bcd_set(&mut tr, hour, 16, 20, 0x3);

        Self::bcd_set(&mut dr, day, 0, 4, 0x3);
        Self::bcd_set(&mut dr, month, 8, 12, 0x1);
        Self::bcd_set(&mut dr, year, 16, 20, 0xF);
        dr &= !(0x7 << 13);
        dr |= (weekday & 0x7) << 13;

        self.regs[RTC_TR_OFFSET] = tr;
        self.regs[RTC_DR_OFFSET] = dr;
    }
}

impl Peripheral for Rtc {
    fn step(&mut self, _sys: &System) {
        if self.rsf_resync_delay > 0 {
            self.rsf_resync_delay -= 1;
            if self.rsf_resync_delay == 0 {
                self.regs[RTC_ISR_OFFSET] |= RTC_ISR_RSF;
            }
        }

        if (self.regs[RTC_ISR_OFFSET] & RTC_ISR_INIT) != 0 {
            return;
        }

        self.tick_accum = self.tick_accum.saturating_add(1);
        if self.tick_accum >= RTC_TICK_STEPS {
            self.tick_accum = 0;
            self.increment_calendar_second();
        }
    }

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
            if offset == 0x0C {
                // ISR write handling for INIT/INITF and RSF resync behavior.
                let mut isr = self.regs[RTC_ISR_OFFSET];
                if (value & RTC_ISR_INIT) != 0 {
                    isr |= RTC_ISR_INIT | RTC_ISR_INITF;
                } else {
                    isr &= !(RTC_ISR_INIT | RTC_ISR_INITF);
                }

                if (value & RTC_ISR_RSF) == 0 {
                    isr &= !RTC_ISR_RSF;
                    self.rsf_resync_delay = 2;
                }

                self.regs[RTC_ISR_OFFSET] = isr;
                return;
            }

            self.regs[idx] = value;
        }
    }
}
