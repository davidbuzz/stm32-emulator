// SPDX-License-Identifier: GPL-3.0-or-later

// STM32 name: I2C1 / I2C2 / I2C3.
// STM32F427 bases: I2C1=0x40005400, I2C2=0x40005800, I2C3=0x40005C00.
// Key registers: CR1, CR2, OAR1/2, DR, SR1, SR2, CCR, TRISE, FLTR.
// Key function: sensor/configuration bus used heavily by ArduPilot board bring-up.
// Critical for this emulator: real boot/runtime progress likely depends on realistic SR1/SR2 state.
// Current model is a minimal stub with toggled status behavior, not a transaction state machine.
// Still incomplete: START/ADDR/BTF/TXE/RXNE sequencing, DMA requests, interrupt behavior.
// Datasheet/reference anchor: STM32F4 RM I2C chapter.

use crate::system::System;
use super::Peripheral;

#[derive(Default)]
pub struct I2c {
    name: String,
    toggle: u8,
}

impl I2c {
    pub fn new(name: &str) -> Option<Box<dyn Peripheral>> {
        if name.starts_with("I2C") {
            let name = name.to_string();
            Some(Box::new(Self { name, ..I2c::default() }))
        } else {
            None
        }
    }
}

impl Peripheral for I2c {
    fn read(&mut self, _sys: &System, offset: u32) -> u32 {
        match offset {
            0x0010 => {
                // DR
                debug!("{} READ", self.name);
                0
            }
            0x0014 => {
                // SR1
                self.toggle = (self.toggle + 1) % 5;
                if self.toggle & 2 != 0 { 0xFFFFFFFF } else { 0 }
            }
            0x0018 => {
                // SR2
                self.toggle = (self.toggle + 1) % 5;
                if self.toggle & 1  != 0{ 0xFFFFFFFF } else { 0 }
            }
            _ => 0
        }
    }

    fn write(&mut self, _sys: &System, offset: u32, value: u32) {
        match offset {
            0x0010 => {
                debug!("{} WRITE value=0x{:08x}", self.name, value);
            }
            _ => {}
        }
    }
}
