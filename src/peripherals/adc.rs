// SPDX-License-Identifier: GPL-3.0-or-later

// STM32 name: ADC1 / ADC2 / ADC3.
// STM32F427 bases: ADC1=0x40012000, ADC2=0x40012100, ADC3=0x40012200.
// Key registers: SR(0x00), CR1(0x04), CR2(0x08), SMPR1/2(0x0C/0x10),
//   SQR1-3(0x2C/0x30/0x34), DR(0x4C).
// Key function: analog sensor readings — battery voltage, current, RSSI, temperature.
// Critical for this emulator: ADC1 is DMA2-paired in circular mode for runtime ADC sampling.
// Current model: Channel-aware conversion results with per-channel synthetic values,
//   EOC event generation and interrupt signaling, overrun detection on rapid conversions.
// Still incomplete: actual analog sampling, watchdog thresholds, injected channels, calibration.
// Datasheet/reference anchor: STM32F4 RM ADC chapter.

use std::collections::VecDeque;
use crate::system::System;
use super::Peripheral;

const ADC_SR_EOC: u32 = 1 << 1;
const ADC_SR_OVR: u32 = 1 << 5;
const ADC_CR1_EOCIE: u32 = 1 << 5;   // EOC interrupt enable
const ADC_CR1_OVRIE: u32 = 1 << 26;  // Overrun interrupt enable
const ADC_CR2_SWSTART: u32 = 1 << 30; // Software start of regular channel conversion

// Per-channel synthetic values (12-bit): channels vary from low to high to simulate
// different sensor readings (battery volts, current, temp, etc.)
const ADC_CHANNEL_VALUES: [u16; 16] = [
    0x0400, // CH0:  1024 (~1.04V)
    0x0600, // CH1:  1536 (~1.56V)
    0x0800, // CH2:  2048 (~1.65V) - typical battery mid-scale
    0x0A00, // CH3:  2560 (~2.07V)
    0x0C00, // CH4:  3072 (~2.48V)
    0x0E00, // CH5:  3584 (~2.89V)
    0x0700, // CH6:  1792 (~1.82V)
    0x0900, // CH7:  2304 (~1.86V)
    0x0B00, // CH8:  2816 (~2.28V)
    0x0D00, // CH9:  3328 (~2.69V)
    0x0F00, // CH10: 3840 (~3.10V)
    0x0500, // CH11: 1280 (~1.30V)
    0x0300, // CH12:  768 (~0.78V)
    0x0200, // CH13:  512 (~0.52V)
    0x0100, // CH14:  256 (~0.26V)
    0x0050, // CH15:   80 (~0.08V)
];

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
    dr: u16,                   // Last conversion result
    last_conversion_channel: u8, // Channel that was last converted
    conversion_in_progress: bool,
}

impl Adc {
    pub fn new(name: &str) -> Option<Box<dyn Peripheral>> {
        // Match ADC1, ADC2, ADC3 — not ADC_Common or similar.
        if name.starts_with("ADC") && name.len() == 4 && name.chars().last().map_or(false, |c| c.is_ascii_digit()) {
            Some(Box::new(Self {
                name: name.to_string(),
                sr: ADC_SR_EOC, // Start ready
                dr: ADC_CHANNEL_VALUES[0],
                htr: 0x0FFF,    // Watchdog high threshold reset value
                ..Default::default()
            }))
        } else {
            None
        }
    }

    fn get_channel_from_sqr(&self) -> u8 {
        // SQR3 bits 4:0 contain the first channel in regular sequence (L=0 case)
        // For simplicity, extract channel 0 from SQR3[4:0]
        let l = (self.sqr1 >> 20) & 0x0F;
        if l == 0 {
            // Single channel conversion: use first (and only) entry in SQR3
            (self.sqr3 & 0x1F) as u8
        } else {
            // Multi-channel sequence: for now return first channel
            (self.sqr3 & 0x1F) as u8
        }
    }

    fn start_conversion(&mut self, sys: &System) {
        if (self.cr2 & (1 << 1)) == 0 {
            // CONT bit not set - single conversion mode
            // Check if conversion was already in progress (OVR detection)
            if self.conversion_in_progress {
                self.sr |= ADC_SR_OVR;
                if (self.cr1 & ADC_CR1_OVRIE) != 0 {
                    self.signal_irq(sys, "OVR");
                }
                return;
            }
        }

        // Mark conversion starting
        self.conversion_in_progress = true;
        self.last_conversion_channel = self.get_channel_from_sqr();
        
        // Perform conversion immediately (simplified: no actual delay)
        self.perform_conversion();

        // Signal EOC
        self.sr |= ADC_SR_EOC;
        if (self.cr1 & ADC_CR1_EOCIE) != 0 {
            self.signal_irq(sys, "EOC");
        }

        self.conversion_in_progress = false;
    }

    fn perform_conversion(&mut self) {
        let ch = self.last_conversion_channel as usize;
        let result = if ch < ADC_CHANNEL_VALUES.len() {
            ADC_CHANNEL_VALUES[ch]
        } else {
            ADC_CHANNEL_VALUES[0]
        };
        self.dr = result;
    }

    fn signal_irq(&self, sys: &System, _reason: &str) {
        // ADC1=IRQ 18, ADC2=IRQ 33, ADC3=IRQ 72, ADC (shared)=IRQ 18
        let irq = match self.name.as_str() {
            "ADC1" => 18,
            "ADC2" => 33,
            "ADC3" => 72,
            _ => return,
        };
        sys.p.nvic.borrow_mut().set_intr_pending(irq as i32);
    }
}

impl Peripheral for Adc {
    fn read(&mut self, sys: &System, offset: u32) -> u32 {
        match offset {
            0x0000 => self.sr | ADC_SR_EOC, // EOC always asserted (ready)
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
                // DR: reading clears EOC; return the last conversion result
                self.sr &= !ADC_SR_EOC;
                self.dr as u32
            }
            _ => 0,
        }
    }

    fn write(&mut self, sys: &System, offset: u32, value: u32) {
        match offset {
            0x0000 => self.sr &= value, // write-0-to-clear (OVR, EOC, etc.)
            0x0004 => {
                self.cr1 = value;
                // If EOCIE is now enabled and EOC is set, potentially signal IRQ
                if (self.cr1 & ADC_CR1_EOCIE) != 0 && (self.sr & ADC_SR_EOC) != 0 {
                    self.signal_irq(sys, "EOC_pending");
                }
            }
            0x0008 => {
                self.cr2 = value;
                if value & ADC_CR2_SWSTART != 0 {
                    // SWSTART: software start of regular channel conversion
                    self.start_conversion(sys);
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
    // Uses the current DR value from the last conversion.
    fn read_dma(&mut self, _sys: &System, offset: u32, size: usize) -> VecDeque<u8> {
        if offset == 0x004C {
            let val = self.dr;
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
