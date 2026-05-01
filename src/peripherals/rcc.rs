// SPDX-License-Identifier: GPL-3.0-or-later

// STM32 name: RCC (Reset and Clock Control).
// STM32F427 base: 0x40023800.
// Key registers: CR, PLLCFGR, CFGR, AHBxENR, APBxENR, AHBxRSTR, CSR, and backup-clock control.
// Key function: peripheral reset, clock gating, oscillator/PLL status, and bus-frequency setup.
// Critical for this emulator: almost every peripheral bring-up depends on RCC enable/reset semantics.
// The current model is intentionally permissive and mostly acts like clocks are immediately available.
// Still incomplete: realistic ready bits, PLL timing, and derived clock-rate effects.
// Datasheet/reference anchor: STM32F4 RM RCC chapter.

use crate::system::System;
use super::Peripheral;

pub struct Rcc {
    cr: u32,
    pllcfgr: u32,
    cfgr: u32,
    cir: u32,
    ahb1rstr: u32,
    ahb2rstr: u32,
    ahb3rstr: u32,
    apb1rstr: u32,
    apb2rstr: u32,
    ahb1enr: u32,
    ahb2enr: u32,
    ahb3enr: u32,
    apb1enr: u32,
    apb2enr: u32,
    ahb1lpenr: u32,
    ahb2lpenr: u32,
    ahb3lpenr: u32,
    apb1lpenr: u32,
    apb2lpenr: u32,
    bdcr: u32,
    csr: u32,
    sscgr: u32,
    plli2scfgr: u32,
    pllsaicfgr: u32,
    dckcfgr: u32,
}

impl Rcc {
    pub fn new(name: &str) -> Option<Box<dyn Peripheral>> {
        if name == "RCC" {
            Some(Box::new(Rcc {
                // Reset values chosen to satisfy STM32F4 startup polling with immediate-ready semantics.
                cr: 0x0000_0083,
                pllcfgr: 0x2400_3010,
                cfgr: 0,
                cir: 0,
                ahb1rstr: 0,
                ahb2rstr: 0,
                ahb3rstr: 0,
                apb1rstr: 0,
                apb2rstr: 0,
                ahb1enr: 0,
                ahb2enr: 0,
                ahb3enr: 0,
                apb1enr: 0,
                apb2enr: 0,
                ahb1lpenr: 0,
                ahb2lpenr: 0,
                ahb3lpenr: 0,
                apb1lpenr: 0,
                apb2lpenr: 0,
                bdcr: 0,
                csr: 0x0e00_0000,
                sscgr: 0,
                plli2scfgr: 0x2000_3000,
                pllsaicfgr: 0x2400_3000,
                dckcfgr: 0,
            }))
        } else {
            None
        }
    }

    fn update_cr_ready_bits(&mut self) {
        const HSION: u32 = 1 << 0;
        const HSIRDY: u32 = 1 << 1;
        const HSEON: u32 = 1 << 16;
        const HSERDY: u32 = 1 << 17;
        const PLLON: u32 = 1 << 24;
        const PLLRDY: u32 = 1 << 25;
        const PLLI2SON: u32 = 1 << 26;
        const PLLI2SRDY: u32 = 1 << 27;
        const PLLSAION: u32 = 1 << 28;
        const PLLSAIRDY: u32 = 1 << 29;

        if self.cr & HSION != 0 {
            self.cr |= HSIRDY;
        } else {
            self.cr &= !HSIRDY;
        }

        if self.cr & HSEON != 0 {
            self.cr |= HSERDY;
        } else {
            self.cr &= !HSERDY;
        }

        if self.cr & PLLON != 0 {
            self.cr |= PLLRDY;
        } else {
            self.cr &= !PLLRDY;
        }

        if self.cr & PLLI2SON != 0 {
            self.cr |= PLLI2SRDY;
        } else {
            self.cr &= !PLLI2SRDY;
        }

        if self.cr & PLLSAION != 0 {
            self.cr |= PLLSAIRDY;
        } else {
            self.cr &= !PLLSAIRDY;
        }
    }

    fn update_cfgr_status_bits(&mut self) {
        const SW_MASK: u32 = 0b11;
        const SWS_MASK: u32 = 0b11 << 2;
        let sw = self.cfgr & SW_MASK;
        self.cfgr = (self.cfgr & !SWS_MASK) | (sw << 2);
    }

    fn validate_pll_config(&self) {
        // PLLCFGR validation: ensure multiplier (PLLN) is in valid range [50..432].
        const PLLN_MASK: u32 = 0x1FC0;
        const PLLN_SHIFT: u32 = 6;
        let plln = (self.pllcfgr & PLLN_MASK) >> PLLN_SHIFT;
        
        if plln < 50 || plln > 432 {
            warn!("RCC invalid PLLN value: {} (must be 50-432)", plln);
        }

        // PLLM (input divider) must be in range [2..63].
        let pllm = self.pllcfgr & 0x3F;
        if pllm < 2 || pllm > 63 {
            warn!("RCC invalid PLLM value: {} (must be 2-63)", pllm);
        }

        // PLLP (output divider) must be 2, 4, 6, or 8.
        const PLLP_MASK: u32 = 0x00030000;
        const PLLP_SHIFT: u32 = 16;
        let pllp_enc = (self.pllcfgr & PLLP_MASK) >> PLLP_SHIFT;
        let pllp = match pllp_enc {
            0b00 => 2,
            0b01 => 4,
            0b10 => 6,
            0b11 => 8,
            _ => 0,
        };
        if pllp == 0 {
            warn!("RCC invalid PLLP encoding: {}", pllp_enc);
        }

        // STM32F427 main PLL does not use PLLR in PLLCFGR; bits [30:28] are reserved.
    }

    fn update_csr_ready_bits(&mut self) {
        const LSION: u32 = 1 << 0;
        const LSIRDY: u32 = 1 << 1;

        if self.csr & LSION != 0 {
            self.csr |= LSIRDY;
        } else {
            self.csr &= !LSIRDY;
        }
    }

    fn update_bdcr_ready_bits(&mut self) {
        const LSEON: u32 = 1 << 0;
        const LSERDY: u32 = 1 << 1;

        if self.bdcr & LSEON != 0 {
            self.bdcr |= LSERDY;
        } else {
            self.bdcr &= !LSERDY;
        }
    }
}


impl Peripheral for Rcc {
    fn read(&mut self, _sys: &System, offset: u32) -> u32 {
        match offset {
            0x0000 => {
                self.update_cr_ready_bits();
                self.cr
            }
            0x0004 => self.pllcfgr,
            0x0008 => {
                self.update_cfgr_status_bits();
                self.cfgr
            }
            0x000c => self.cir,
            0x0010 => self.ahb1rstr,
            0x0014 => self.ahb2rstr,
            0x0018 => self.ahb3rstr,
            0x0020 => self.apb1rstr,
            0x0024 => self.apb2rstr,
            0x0030 => self.ahb1enr,
            0x0034 => self.ahb2enr,
            0x0038 => self.ahb3enr,
            0x0040 => self.apb1enr,
            0x0044 => self.apb2enr,
            0x0050 => self.ahb1lpenr,
            0x0054 => self.ahb2lpenr,
            0x0058 => self.ahb3lpenr,
            0x0060 => self.apb1lpenr,
            0x0064 => self.apb2lpenr,
            0x0070 => {
                self.update_bdcr_ready_bits();
                self.bdcr
            }
            0x0074 => {
                self.update_csr_ready_bits();
                self.csr
            }
            0x0080 => self.sscgr,
            0x0084 => self.plli2scfgr,
            0x0088 => self.pllsaicfgr,
            0x008c => self.dckcfgr,
            _ => 0
        }
    }

    fn write(&mut self, _sys: &System, offset: u32, value: u32) {
        match offset {
            0x0000 => {
                self.cr = value;
                self.update_cr_ready_bits();
            }
            0x0004 => {
                // PLLCFGR write: validate configuration
                self.pllcfgr = value;
                self.validate_pll_config();
            }
            0x0008 => {
                self.cfgr = value;
                self.update_cfgr_status_bits();
            }
            0x000c => self.cir = value,
            0x0010 => self.ahb1rstr = value,
            0x0014 => self.ahb2rstr = value,
            0x0018 => self.ahb3rstr = value,
            0x0020 => self.apb1rstr = value,
            0x0024 => self.apb2rstr = value,
            0x0030 => self.ahb1enr = value,
            0x0034 => self.ahb2enr = value,
            0x0038 => self.ahb3enr = value,
            0x0040 => self.apb1enr = value,
            0x0044 => self.apb2enr = value,
            0x0050 => self.ahb1lpenr = value,
            0x0054 => self.ahb2lpenr = value,
            0x0058 => self.ahb3lpenr = value,
            0x0060 => self.apb1lpenr = value,
            0x0064 => self.apb2lpenr = value,
            0x0070 => {
                const BDRST: u32 = 1 << 16;
                self.bdcr = value;
                if self.bdcr & BDRST != 0 {
                    self.bdcr = BDRST;
                }
                self.update_bdcr_ready_bits();
            }
            0x0074 => {
                self.csr = value;
                self.update_csr_ready_bits();
            }
            0x0080 => self.sscgr = value,
            0x0084 => self.plli2scfgr = value,
            0x0088 => self.pllsaicfgr = value,
            0x008c => self.dckcfgr = value,
            _ => {}
        }
    }
}
