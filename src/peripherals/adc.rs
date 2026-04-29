// SPDX-License-Identifier: GPL-3.0-or-later

// STM32 name: ADC1 / ADC2 / ADC3.
// STM32F427 bases: ADC1=0x40012000, ADC2=0x40012100, ADC3=0x40012200.
// Key registers: SR(0x00), CR1(0x04), CR2(0x08), SMPR1/2(0x0C/0x10),
//   SQR1-3(0x2C/0x30/0x34), DR(0x4C).
// Key function: analog sensor readings — battery voltage, current, RSSI, temperature.
// Critical for this emulator: ADC1 is DMA2-paired in circular mode for runtime ADC sampling.
// This stub: SR.EOC always set, DR returns 0x0800 (12-bit half-scale ~1.65V on 3.3V rail),
//   read_dma overridden to emit correct little-endian halfwords for PSIZE=2 DMA bursts.
// Still incomplete: actual channel scanning, watchdog thresholds, injected channels, calibration.
// Datasheet/reference anchor: STM32F4 RM ADC chapter.

use std::collections::VecDeque;
use crate::system::System;
use super::Peripheral;

const ADC_SR_EOC: u32 = 1 << 1;
const ADC_SR_OVR: u32 = 1 << 5;

// Synthetic 12-bit ADC result: 0x0800 = 2048 = half-scale on a 3.3V rail ≈ 1.65V.
// Maps to ~16.5V battery (via typical 1:10 divider) — valid 4S LiPo range.
const ADC_SYNTHETIC_VALUE: u16 = 0x0800;

#[derive(Default)]
pub struct Adc {
    name: String,
    sr: u32,
    cr1: u32,
    cr2: u32,
    smpr1: u32,
    smpr2: u32,
    jofr: [u32; 4],
    htr: u32,
    ltr: u32,
    sqr1: u32,
    sqr2: u32,
    sqr3: u32,
    jsqr: u32,
    jdr: [u32; 4],
}

impl Adc {
    pub fn new(name: &str) -> Option<Box<dyn Peripheral>> {
        // Match ADC1, ADC2, ADC3 — not ADC_Common or similar.
        if name.starts_with("ADC") && name.len() == 4 && name.chars().last().map_or(false, |c| c.is_ascii_digit()) {
            Some(Box::new(Self {
                name: name.to_string(),
                sr: ADC_SR_EOC, // Start ready
                htr: 0x0FFF,   // Watchdog high threshold reset value
                ..Default::default()
            }))
        } else {
            None
        }
    }
}

impl Peripheral for Adc {
    fn read(&mut self, _sys: &System, offset: u32) -> u32 {
        match offset {
            0x0000 => self.sr | ADC_SR_EOC, // EOC always asserted
            0x0004 => self.cr1,
            0x0008 => self.cr2,
            0x000C => self.smpr1,
            0x0010 => self.smpr2,
            0x0014 => self.jofr[0],
            0x0018 => self.jofr[1],
            0x001C => self.jofr[2],
            0x0020 => self.jofr[3],
            0x0024 => self.htr,
            0x0028 => self.ltr,
            0x002C => self.sqr1,
            0x0030 => self.sqr2,
            0x0034 => self.sqr3,
            0x0038 => self.jsqr,
            0x003C => self.jdr[0],
            0x0040 => self.jdr[1],
            0x0044 => self.jdr[2],
            0x0048 => self.jdr[3],
            0x004C => {
                // DR: reading clears EOC; we re-assert immediately for single-conversion compat.
                self.sr &= !ADC_SR_EOC;
                self.sr |= ADC_SR_EOC;
                ADC_SYNTHETIC_VALUE as u32
            }
            _ => 0,
        }
    }

    fn write(&mut self, _sys: &System, offset: u32, value: u32) {
        match offset {
            0x0000 => self.sr &= value, // write-0-to-clear (OVR, etc.)
            0x0004 => self.cr1 = value,
            0x0008 => {
                self.cr2 = value;
                if value & (1 << 30) != 0 {
                    // SWSTART: software start of regular channel conversion
                    self.sr |= ADC_SR_EOC;
                }
            }
            0x000C => self.smpr1 = value,
            0x0010 => self.smpr2 = value,
            0x0014 => self.jofr[0] = value,
            0x0018 => self.jofr[1] = value,
            0x001C => self.jofr[2] = value,
            0x0020 => self.jofr[3] = value,
            0x0024 => self.htr = value,
            0x0028 => self.ltr = value,
            0x002C => self.sqr1 = value,
            0x0030 => self.sqr2 = value,
            0x0034 => self.sqr3 = value,
            0x0038 => self.jsqr = value,
            _ => {}
        }
    }

    // Override read_dma to emit correct little-endian halfwords for PSIZE=2 DMA bursts.
    // Default read_dma calls read() per byte and truncates to u8, which loses the high byte.
    fn read_dma(&mut self, _sys: &System, offset: u32, size: usize) -> VecDeque<u8> {
        if offset == 0x004C {
            let val = ADC_SYNTHETIC_VALUE;
            let lo = (val & 0xFF) as u8;
            let hi = (val >> 8) as u8;
            let mut v = VecDeque::with_capacity(size);
            let beats = size / 2;
            for _ in 0..beats {
                v.push_back(lo);
                v.push_back(hi);
            }
            if size % 2 != 0 {
                v.push_back(lo);
            }
            v
        } else {
            VecDeque::new()
        }
    }
}
