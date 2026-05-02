// SPDX-License-Identifier: GPL-3.0-or-later

// STM32 name: CAN1/CAN2 (bxCAN).
// STM32F427 bases: CAN1=0x40006400, CAN2=0x40006800.
// Minimal model: register storage and basic INRQ/INAK handshake semantics.

use crate::system::System;

use super::Peripheral;

const CAN_MCR_INRQ: u32 = 1 << 0;
const CAN_MSR_INAK: u32 = 1 << 0;
const CAN_MCR_MASK: u32 =
    (1 << 0) | // INRQ
    (1 << 1) | // SLEEP
    (1 << 2) | // TXFP
    (1 << 3) | // RFLM
    (1 << 4) | // NART
    (1 << 5) | // AWUM
    (1 << 6) | // ABOM
    (1 << 7) | // TTCM
    (1 << 15); // RESET

const CAN_TSR_RQCP0: u32 = 1 << 0;
const CAN_TSR_TXOK0: u32 = 1 << 1;
const CAN_TSR_TME0: u32 = 1 << 26;
const CAN_TI0R_TXRQ: u32 = 1 << 0;

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

        match offset {
            0x00 => {
                self.regs[idx] = value & CAN_MCR_MASK;
            }
            0x08 => {
                // TSR: selected bits are write-1-to-clear.
                self.regs[idx] &= !(value & (CAN_TSR_RQCP0 | CAN_TSR_TXOK0));
                self.regs[idx] |= CAN_TSR_TME0;
            }
            0x180 => {
                // TI0R: when TXRQ is set, complete transmission immediately in this stub.
                self.regs[idx] = value;
                if (value & CAN_TI0R_TXRQ) != 0 {
                    self.regs[0x08 / 4] |= CAN_TSR_RQCP0 | CAN_TSR_TXOK0 | CAN_TSR_TME0;
                    self.regs[idx] &= !CAN_TI0R_TXRQ;
                }
            }
            _ => {
                self.regs[idx] = value;
            }
        }

        // MCR/MSR basic init mode handshake.
        if offset == 0x00 {
            if self.regs[idx] & CAN_MCR_INRQ != 0 {
                self.regs[0x04 / 4] |= CAN_MSR_INAK;
            } else {
                self.regs[0x04 / 4] &= !CAN_MSR_INAK;
            }
        }
    }
}
