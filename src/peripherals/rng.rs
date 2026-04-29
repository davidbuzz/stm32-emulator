// SPDX-License-Identifier: GPL-3.0-or-later

// STM32 name: RNG (random number generator).
// STM32F427 base: 0x50060800.
// Key registers: CR, SR, DR.
// This model is minimal: when RNGEN is set, SR.DRDY is asserted and DR returns a
// deterministic pseudo-random stream.

use crate::system::System;

use super::Peripheral;

const RNG_CR_RNGEN: u32 = 1 << 2;
const RNG_SR_DRDY: u32 = 1 << 0;
const RNG_SR_SECS: u32 = 1 << 2;
const RNG_SR_CECS: u32 = 1 << 1;

pub struct Rng {
    cr: u32,
    sr: u32,
    lfsr: u32,
}

impl Rng {
    pub fn new(name: &str) -> Option<Box<dyn Peripheral>> {
        if name == "RNG" {
            Some(Box::new(Self {
                cr: 0,
                sr: 0,
                lfsr: 0x1234_5678,
            }))
        } else {
            None
        }
    }

    fn next_word(&mut self) -> u32 {
        // xorshift32: deterministic, cheap, sufficient for firmware progress.
        let mut x = self.lfsr;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.lfsr = x;
        x
    }
}

impl Peripheral for Rng {
    fn read(&mut self, _sys: &System, offset: u32) -> u32 {
        match offset {
            0x00 => self.cr,
            0x04 => {
                let mut sr = self.sr & (RNG_SR_SECS | RNG_SR_CECS);
                if self.cr & RNG_CR_RNGEN != 0 {
                    sr |= RNG_SR_DRDY;
                }
                sr
            }
            0x08 => {
                if self.cr & RNG_CR_RNGEN != 0 {
                    self.next_word()
                } else {
                    0
                }
            }
            _ => 0,
        }
    }

    fn write(&mut self, _sys: &System, offset: u32, value: u32) {
        match offset {
            0x00 => self.cr = value & RNG_CR_RNGEN,
            0x04 => {
                // SR is status-only on hardware; keep only sticky error bits clearable by write-1.
                self.sr &= !(value & (RNG_SR_SECS | RNG_SR_CECS));
            }
            _ => {}
        }
    }
}
