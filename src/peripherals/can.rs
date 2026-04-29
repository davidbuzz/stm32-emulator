// SPDX-License-Identifier: GPL-3.0-or-later

// STM32 name: CAN1/CAN2 (bxCAN).
// STM32F427 bases: CAN1=0x40006400, CAN2=0x40006800.
// Minimal model: register storage and basic INRQ/INAK handshake semantics.

use crate::system::System;

use super::Peripheral;

const CAN_MCR_INRQ: u32 = 1 << 0;
const CAN_MSR_INAK: u32 = 1 << 0;

pub struct Can {
    _name: String,
    regs: [u32; 0x80],
}

impl Can {
    pub fn new(name: &str) -> Option<Box<dyn Peripheral>> {
        if name == "CAN1" || name == "CAN2" {
            Some(Box::new(Self {
                _name: name.to_string(),
                regs: [0; 0x80],
            }))
        } else {
            None
        }
    }
}

impl Peripheral for Can {
    fn read(&mut self, _sys: &System, offset: u32) -> u32 {
        let idx = (offset / 4) as usize;
        self.regs.get(idx).copied().unwrap_or(0)
    }

    fn write(&mut self, _sys: &System, offset: u32, value: u32) {
        let idx = (offset / 4) as usize;
        if idx >= self.regs.len() {
            return;
        }

        self.regs[idx] = value;

        // MCR/MSR basic init mode handshake.
        if offset == 0x00 {
            if value & CAN_MCR_INRQ != 0 {
                self.regs[0x04 / 4] |= CAN_MSR_INAK;
            } else {
                self.regs[0x04 / 4] &= !CAN_MSR_INAK;
            }
        }
    }
}
