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
}

impl Rcc {
    pub fn new(name: &str) -> Option<Box<dyn Peripheral>> {
        if name == "RCC" {
            Some(Box::new(Rcc {}))
        } else {
            None
        }
    }
}


impl Peripheral for Rcc {
    fn read(&mut self, _sys: &System, offset: u32) -> u32 {
        match offset {
            0x0000 => {
                // CR register
                // Return all the r to true. This is where the PLL ready flags are.
                //0b0010_0000_0010_0000_0000_0000_0010
                0xFFFF_FFFF
            }
            0x0008 => {
                // CFGR register
                0b1000
            }
            _ => 0
        }
    }

    fn write(&mut self, _sys: &System, _offset: u32, _value: u32) {
    }
}
