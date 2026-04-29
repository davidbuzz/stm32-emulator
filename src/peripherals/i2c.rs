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
use super::{Peripheral, meta::DeviceMeta};

pub struct I2c {
    name: String,
    event_irq: i32, // resolved from SVD via DeviceMeta at construction
    error_irq: i32,
    cr1: u32,
    cr2: u32,
    oar1: u32,
    oar2: u32,
    dr: u32,
    sr1: u32,
    sr2: u32,
    ccr: u32,
    trise: u32,
    fltr: u32,
    awaiting_address: bool,
    pending_event_irq: Option<u8>,
    pending_error_irq: Option<u8>,
}

impl I2c {
    pub fn new(name: &str, meta: &DeviceMeta) -> Option<Box<dyn Peripheral>> {
        if name.starts_with("I2C") {
            // Look up IRQ numbers from SVD.  Fall back to STM32F427 RM values if SVD
            // doesn't list them (e.g. derived peripherals that inherit interrupt entries).
            let ev_name = format!("{}_EV", name);
            let er_name = format!("{}_ER", name);
            let (event_irq, error_irq) = match name {
                "I2C1" => (meta.irq_of(&ev_name).unwrap_or(31), meta.irq_of(&er_name).unwrap_or(32)),
                "I2C2" => (meta.irq_of(&ev_name).unwrap_or(33), meta.irq_of(&er_name).unwrap_or(34)),
                "I2C3" => (meta.irq_of(&ev_name).unwrap_or(72), meta.irq_of(&er_name).unwrap_or(73)),
                _      => return None, // unknown I2C instance — don't register
            };
            Some(Box::new(Self {
                name: name.to_string(),
                event_irq,
                error_irq,
                trise: 0x0000_0002,
                cr1: 0, cr2: 0, oar1: 0, oar2: 0, dr: 0,
                sr1: 0, sr2: 0, ccr: 0, fltr: 0,
                awaiting_address: false,
                pending_event_irq: None,
                pending_error_irq: None,
            }))
        } else {
            None
        }
    }

    fn clear_master_state(&mut self) {
        self.awaiting_address = false;
        self.sr1 &= !(I2C_SR1_SB | I2C_SR1_ADDR | I2C_SR1_ADD10 | I2C_SR1_BTF | I2C_SR1_RXNE | I2C_SR1_TXE);
        self.sr2 &= !(I2C_SR2_MSL | I2C_SR2_BUSY | I2C_SR2_TRA);
    }

    fn reset(&mut self) {
        self.cr1 = 0;
        self.cr2 = 0;
        self.oar1 = 0;
        self.oar2 = 0;
        self.dr = 0;
        self.sr1 = 0;
        self.sr2 = 0;
        self.ccr = 0;
        self.trise = 0x0000_0002;
        self.fltr = 0;
        self.awaiting_address = false;
        self.pending_event_irq = None;
        self.pending_error_irq = None;
    }

    fn begin_start(&mut self) {
        self.awaiting_address = true;
        self.sr1 = (self.sr1 & I2C_ERROR_MASK) | I2C_SR1_SB;
        self.sr2 |= I2C_SR2_MSL | I2C_SR2_BUSY;
        self.sr2 &= !I2C_SR2_TRA;
        self.pending_event_irq = Some(1);
    }

    fn finish_stop(&mut self) {
        self.clear_master_state();
        self.pending_event_irq = None;
        self.pending_error_irq = None;
        self.cr1 &= !I2C_CR1_STOP;
    }

    fn nack_address(&mut self) {
        self.awaiting_address = false;
        self.cr1 &= !I2C_CR1_START;
        self.clear_master_state();
        self.sr1 |= I2C_SR1_AF;
        // Cancel the pending SB event IRQ — firmware already consumed SB by writing DR.
        // Without this, the SB event fires after the NACK, confusing the interrupt handler.
        self.pending_event_irq = None;
        self.pending_error_irq = Some(1);
    }
}

impl Peripheral for I2c {
    fn read(&mut self, _sys: &System, offset: u32) -> u32 {
        match offset {
            0x0000 => self.cr1,
            0x0004 => self.cr2,
            0x0008 => self.oar1,
            0x000c => self.oar2,
            0x0010 => self.dr,
            0x0014 => self.sr1,
            0x0018 => self.sr2,
            0x001c => self.ccr,
            0x0020 => self.trise,
            0x0024 => self.fltr,
            _ => 0
        }
    }

    fn write(&mut self, _sys: &System, offset: u32, value: u32) {
        match offset {
            0x0000 => {
                let old_cr1 = self.cr1;
                self.cr1 = value;

                if value & I2C_CR1_SWRST != 0 {
                    self.reset();
                    return;
                }

                if value & I2C_CR1_PE == 0 {
                    self.finish_stop();
                    self.cr1 = value & !I2C_CR1_STOP;
                    return;
                }

                if value & I2C_CR1_STOP != 0 {
                    self.finish_stop();
                }

                if old_cr1 & I2C_CR1_START == 0 && value & I2C_CR1_START != 0 {
                    self.begin_start();
                }
            }
            0x0004 => {
                self.cr2 = value;
            }
            0x0008 => {
                self.oar1 = value;
            }
            0x000c => {
                self.oar2 = value;
            }
            0x0010 => {
                self.dr = value & 0xff;

                if self.awaiting_address {
                    trace!("{} address phase addr=0x{:02x}: synthetic NACK", self.name, self.dr);
                    self.nack_address();
                }
            }
            0x0014 => {
                self.sr1 &= value;
            }
            0x0018 => {
                self.sr2 = value;
            }
            0x001c => {
                self.ccr = value;
            }
            0x0020 => {
                self.trise = value;
            }
            0x0024 => {
                self.fltr = value;
            }
            _ => {}
        }
    }

    fn step(&mut self, sys: &System) {
        if let Some(delay) = self.pending_event_irq.as_mut() {
            if *delay > 0 {
                *delay -= 1;
            }
            if *delay == 0 {
                self.pending_event_irq = None;
                if self.cr2 & I2C_CR2_ITEVTEN != 0 {
                    sys.p.nvic.borrow_mut().set_intr_pending(self.event_irq);
                }
            }
        }

        if let Some(delay) = self.pending_error_irq.as_mut() {
            if *delay > 0 {
                *delay -= 1;
            }
            if *delay == 0 {
                self.pending_error_irq = None;
                if self.cr2 & I2C_CR2_ITERREN != 0 {
                    sys.p.nvic.borrow_mut().set_intr_pending(self.error_irq);
                }
            }
        }
    }
}

const I2C_CR1_PE: u32 = 1 << 0;
const I2C_CR1_START: u32 = 1 << 8;
const I2C_CR1_STOP: u32 = 1 << 9;
const I2C_CR1_SWRST: u32 = 1 << 15;

const I2C_CR2_ITEVTEN: u32 = 1 << 9;
const I2C_CR2_ITERREN: u32 = 1 << 8;

const I2C_SR1_SB: u32 = 1 << 0;
const I2C_SR1_ADDR: u32 = 1 << 1;
const I2C_SR1_BTF: u32 = 1 << 2;
const I2C_SR1_ADD10: u32 = 1 << 3;
const I2C_SR1_RXNE: u32 = 1 << 6;
const I2C_SR1_TXE: u32 = 1 << 7;
const I2C_SR1_AF: u32 = 1 << 10;

const I2C_SR2_MSL: u32 = 1 << 0;
const I2C_SR2_BUSY: u32 = 1 << 1;
const I2C_SR2_TRA: u32 = 1 << 2;

const I2C_ERROR_MASK: u32 = (1 << 8) | (1 << 9) | I2C_SR1_AF | (1 << 11) | (1 << 12) | (1 << 14) | (1 << 15);
