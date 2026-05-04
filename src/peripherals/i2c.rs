// SPDX-License-Identifier: GPL-3.0-or-later

// STM32 name: I2C1 / I2C2 / I2C3.
// STM32F427 bases: I2C1=0x40005400, I2C2=0x40005800, I2C3=0x40005C00.
// Key registers: CR1, CR2, OAR1/2, DR, SR1, SR2, CCR, TRISE, FLTR.
// Key function: sensor/configuration bus used heavily by ArduPilot board bring-up.
// Critical for this emulator: ChibiOS expects EV5/EV6/EV8_2 style SR1/SR2 sequencing.
// Current model provides a minimal master-side state machine with synthetic ACK/data behavior.
// Still incomplete: real slave device hooks, DMA requests beyond basic DR access, and full IRQ/error fidelity.
// Datasheet/reference anchor: STM32F4 RM I2C chapter.

use std::collections::{HashMap, VecDeque};

use crate::system::System;
use super::{meta::DeviceMeta, Peripheral};

const RCC_BASE: u64 = 0x4002_3800;
const RCC_CFGR_OFFSET: u64 = 0x08;

pub struct I2c {
    name: String,
    event_irq: i32,
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
    awaiting_addr_clear: bool,
    last_addr_was_read: bool,
    sr1_read_armed_for_addr_clear: bool,
    pending_event_irq: Option<u8>,
    pending_error_irq: Option<u8>,
    event_delay_div_accum: u32,
    error_delay_div_accum: u32,
    active_addr: Option<u8>,
    slaves: HashMap<u8, I2cSlave>,
}

#[derive(Clone)]
struct I2cSlave {
    regs: [u8; 256],
    pointer: u8,
    expecting_register: bool,
}

impl I2c {
    pub fn new(name: &str, meta: &DeviceMeta) -> Option<Box<dyn Peripheral>> {
        if !name.starts_with("I2C") {
            return None;
        }

        let ev_name = format!("{}_EV", name);
        let er_name = format!("{}_ER", name);
        let (event_irq, error_irq) = match name {
            "I2C1" => (meta.irq_of(&ev_name).unwrap_or(31), meta.irq_of(&er_name).unwrap_or(32)),
            "I2C2" => (meta.irq_of(&ev_name).unwrap_or(33), meta.irq_of(&er_name).unwrap_or(34)),
            "I2C3" => (meta.irq_of(&ev_name).unwrap_or(72), meta.irq_of(&er_name).unwrap_or(73)),
            _ => return None,
        };

        Some(Box::new(Self {
            name: name.to_string(),
            event_irq,
            error_irq,
            cr1: 0,
            cr2: 0,
            oar1: 0,
            oar2: 0,
            dr: 0,
            sr1: 0,
            sr2: 0,
            ccr: 0,
            trise: 0x0000_0002,
            fltr: 0,
            awaiting_address: false,
            awaiting_addr_clear: false,
            last_addr_was_read: false,
            sr1_read_armed_for_addr_clear: false,
            pending_event_irq: None,
            pending_error_irq: None,
            event_delay_div_accum: 0,
            error_delay_div_accum: 0,
            active_addr: None,
            slaves: default_i2c_slaves(name),
        }))
    }

    fn clear_master_state(&mut self) {
        self.awaiting_address = false;
        self.awaiting_addr_clear = false;
        self.sr1_read_armed_for_addr_clear = false;
        self.sr1 &= !(I2C_SR1_SB | I2C_SR1_ADDR | I2C_SR1_ADD10 | I2C_SR1_BTF | I2C_SR1_RXNE | I2C_SR1_TXE);
        self.sr2 &= !(I2C_SR2_MSL | I2C_SR2_BUSY | I2C_SR2_TRA);
        self.active_addr = None;
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
        self.awaiting_addr_clear = false;
        self.last_addr_was_read = false;
        self.sr1_read_armed_for_addr_clear = false;
        self.pending_event_irq = None;
        self.pending_error_irq = None;
        self.event_delay_div_accum = 0;
        self.error_delay_div_accum = 0;
        self.active_addr = None;
    }

    fn begin_start(&mut self) {
        self.awaiting_address = true;
        self.awaiting_addr_clear = false;
        self.sr1_read_armed_for_addr_clear = false;
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

    fn ack_address(&mut self, addr_byte: u8) {
        let addr_7bit = addr_byte >> 1;
        self.awaiting_address = false;
        self.awaiting_addr_clear = true;
        self.sr1_read_armed_for_addr_clear = false;
        self.last_addr_was_read = (addr_byte & 1) != 0;
        self.active_addr = Some(addr_7bit);

        self.cr1 &= !I2C_CR1_START;
        self.sr1 &= !(I2C_SR1_SB | I2C_SR1_ADD10 | I2C_SR1_AF | I2C_SR1_BTF | I2C_SR1_RXNE);
        self.sr1 |= I2C_SR1_ADDR;

        self.sr2 |= I2C_SR2_MSL | I2C_SR2_BUSY;
        if self.last_addr_was_read {
            self.sr2 &= !I2C_SR2_TRA;
            self.sr1 &= !I2C_SR1_TXE;
            self.prepare_read_byte();
        } else {
            self.sr2 |= I2C_SR2_TRA;
            self.sr1 |= I2C_SR1_TXE;
            self.sr1 &= !I2C_SR1_RXNE;
            if let Some(slave) = self.slaves.get_mut(&addr_7bit) {
                slave.expecting_register = true;
            }
        }

        self.pending_event_irq = Some(1);
    }

    fn nack_address(&mut self) {
        // AF (Address Failure): firmware will see AF flag set and address not acknowledged.
        // This allows firmware to detect and handle slave device absence or rejection.
        self.awaiting_address = false;
        self.awaiting_addr_clear = false;
        self.cr1 &= !I2C_CR1_START;
        self.clear_master_state();
        self.sr1 |= I2C_SR1_AF;
        // AF is an error condition, so signal error IRQ (not event IRQ)
        self.pending_error_irq = Some(1);
        self.pending_event_irq = None;
        debug!("{} address NACK detected (slave not responding or rejected)", self.name);
    }

    fn schedule_btf_event(&mut self) {
        self.sr1 |= I2C_SR1_TXE | I2C_SR1_BTF;
        self.pending_event_irq = Some(0);
    }

    fn prepare_read_byte(&mut self) {
        if let Some(addr) = self.active_addr {
            if let Some(slave) = self.slaves.get_mut(&addr) {
                self.dr = slave.regs[slave.pointer as usize] as u32;
                slave.pointer = slave.pointer.wrapping_add(1);
                self.sr1 |= I2C_SR1_RXNE;
                return;
            }
        }

        self.dr = 0;
        self.sr1 |= I2C_SR1_RXNE;
    }

    fn handle_data_write(&mut self, value: u8) {
        let Some(addr) = self.active_addr else {
            return;
        };
        let Some(slave) = self.slaves.get_mut(&addr) else {
            return;
        };

        if slave.expecting_register {
            slave.pointer = value;
            slave.expecting_register = false;
            return;
        }

        slave.regs[slave.pointer as usize] = value;
        slave.pointer = slave.pointer.wrapping_add(1);
    }

    fn schedule_error_irq_if_needed(&mut self) {
        if (self.cr2 & I2C_CR2_ITERREN) != 0 && (self.sr1 & I2C_ERROR_MASK) != 0 {
            self.pending_error_irq = Some(0);
        }
    }

    fn apb1_divider(&self, sys: &System) -> u32 {
        let mut cfgr = [0u8; 4];
        if sys.uc.borrow().mem_read(RCC_BASE + RCC_CFGR_OFFSET, &mut cfgr).is_err() {
            return 1;
        }

        let ppre1 = (u32::from_le_bytes(cfgr) >> 10) & 0b111;
        match ppre1 {
            0b000..=0b011 => 1,
            0b100 => 2,
            0b101 => 4,
            0b110 => 8,
            0b111 => 16,
            _ => 1,
        }
    }
}

impl Peripheral for I2c {
    fn read(&mut self, _sys: &System, offset: u32) -> u32 {
        match offset {
            0x0000 => self.cr1,
            0x0004 => self.cr2,
            0x0008 => self.oar1,
            0x000c => self.oar2,
            0x0010 => {
                let value = self.dr;

                if self.last_addr_was_read && self.active_addr.is_some() {
                    self.prepare_read_byte();
                    self.sr1 |= I2C_SR1_BTF;
                }

                if (self.sr1 & I2C_SR1_BTF) != 0 && (self.sr1 & I2C_SR1_RXNE) == 0 {
                    self.sr1 &= !I2C_SR1_BTF;
                }
                value
            }
            0x0014 => {
                if self.awaiting_addr_clear && (self.sr1 & (I2C_SR1_ADDR | I2C_SR1_ADD10)) != 0 {
                    self.sr1_read_armed_for_addr_clear = true;
                }
                self.sr1
            }
            0x0018 => {
                if self.awaiting_addr_clear && self.sr1_read_armed_for_addr_clear {
                    self.sr1 &= !(I2C_SR1_ADDR | I2C_SR1_ADD10);
                    self.awaiting_addr_clear = false;
                    self.sr1_read_armed_for_addr_clear = false;
                }
                self.sr2
            }
            0x001c => self.ccr,
            0x0020 => self.trise,
            0x0024 => self.fltr,
            _ => 0,
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

                if (old_cr1 & I2C_CR1_START) == 0 && (value & I2C_CR1_START) != 0 {
                    self.begin_start();
                }
            }
            0x0004 => self.cr2 = value,
            0x0008 => self.oar1 = value,
            0x000c => self.oar2 = value,
            0x0010 => {
                self.dr = value & 0xff;
                if self.awaiting_address {
                    let addr = self.dr as u8;
                    let addr_7bit = addr >> 1;
                    if !self.slaves.contains_key(&addr_7bit) {
                        trace!("{} address phase addr=0x{:02x}: no board hook, NACK", self.name, self.dr);
                        self.nack_address();
                    } else {
                        self.ack_address(addr);
                    }
                } else {
                    self.handle_data_write((value & 0xff) as u8);
                    self.sr1 |= I2C_SR1_TXE | I2C_SR1_BTF;
                    self.pending_event_irq = Some(0);
                }
            }
            0x0014 => {
                // Write-0-to-clear SR1 flags (BERR, ARLO, AF, OVR, TIMEOUT, etc.).
                self.sr1 &= value;
                self.schedule_error_irq_if_needed();
            }
            0x0018 => self.sr2 = value,
            0x001c => {
                // CCR: I2C clock control register. Validates that T_high/T_low timing is configured.
                // Firmware typically sets CCR based on I2C_CR2 FREQ field and desired I2C speed.
                // We just store it; actual timing is emulated elsewhere.
                if (value & 0xFFF) == 0 {
                    // CCR=0 is invalid and could indicate corrupted/uninitialized I2C state.
                    warn!("{} CCR set to 0 (invalid clock divider)", self.name);
                }
                self.ccr = value;
            }
            0x0020 => {
                // TRISE: I2C rise time. Firmware sets this based on I2C_CR2 FREQ and max SCL rise time.
                // Common pattern: TRISE = (freq_mhz + 1) where freq_mhz is in MHz.
                // We just store it; actual timing is simplified.
                if value == 0 {
                    warn!("{} TRISE set to 0 (invalid rise time)", self.name);
                }
                self.trise = value;
            }
            0x0024 => self.fltr = value,
            _ => {}
        }
    }

    fn step(&mut self, sys: &System) {
        let apb_div = self.apb1_divider(sys).max(1);

        if let Some(delay) = self.pending_event_irq.as_mut() {
            self.event_delay_div_accum = self.event_delay_div_accum.saturating_add(1);
            if self.event_delay_div_accum >= apb_div {
                self.event_delay_div_accum = 0;
                if *delay > 0 {
                    *delay -= 1;
                }
            }
            if *delay == 0 && (self.cr2 & I2C_CR2_ITEVTEN) != 0 {
                self.pending_event_irq = None;
                self.event_delay_div_accum = 0;
                sys.p.nvic.borrow_mut().set_intr_pending(self.event_irq);
            }
        } else {
            self.event_delay_div_accum = 0;
        }

        if let Some(delay) = self.pending_error_irq.as_mut() {
            self.error_delay_div_accum = self.error_delay_div_accum.saturating_add(1);
            if self.error_delay_div_accum >= apb_div {
                self.error_delay_div_accum = 0;
                if *delay > 0 {
                    *delay -= 1;
                }
            }
            if *delay == 0 && (self.cr2 & I2C_CR2_ITERREN) != 0 {
                self.pending_error_irq = None;
                self.error_delay_div_accum = 0;
                sys.p.nvic.borrow_mut().set_intr_pending(self.error_irq);
            }
        } else {
            self.error_delay_div_accum = 0;
        }
    }

    fn read_dma(&mut self, _sys: &System, offset: u32, size: usize) -> VecDeque<u8> {
        if offset != 0x0010 || (self.cr2 & I2C_CR2_DMAEN) == 0 {
            return VecDeque::new();
        }

        // DMA receiver path: bytes are consumed directly into memory, so RXNE remains clear.
        // Mark transfer progression with BTF and schedule an EV IRQ if enabled.
        self.sr1 &= !I2C_SR1_RXNE;
        self.sr1 |= I2C_SR1_BTF;
        self.pending_event_irq = Some(0);

        let mut out = VecDeque::with_capacity(size);
        if self.last_addr_was_read {
            if let Some(addr) = self.active_addr {
                if let Some(slave) = self.slaves.get_mut(&addr) {
                    for _ in 0..size {
                        out.push_back(slave.regs[slave.pointer as usize]);
                        slave.pointer = slave.pointer.wrapping_add(1);
                    }
                }
            }
        }

        while out.len() < size {
            out.push_back(0);
        }
        out
    }

    fn write_dma(&mut self, _sys: &System, offset: u32, value: VecDeque<u8>) {
        if offset != 0x0010 || (self.cr2 & I2C_CR2_DMAEN) == 0 {
            return;
        }

        if let Some(last) = value.back().copied() {
            self.dr = last as u32;
        }
        self.schedule_btf_event();
    }
}

fn default_i2c_slaves(_name: &str) -> HashMap<u8, I2cSlave> {
    let mut slaves = HashMap::new();

    // Common CubeBlack bring-up probe targets on external I2C paths.
    slaves.insert(0x1e, make_slave(&[(0x0A, b'H'), (0x0B, b'4'), (0x0C, b'3')]));
    slaves.insert(0x0e, make_slave(&[(0x00, 0x10)]));
    slaves.insert(0x0c, make_slave(&[(0x00, 0x48), (0x01, 0x09)]));
    slaves.insert(0x76, make_slave(&[(0xD0, 0x58)]));
    slaves.insert(0x77, make_slave(&[(0xD0, 0x58)]));
    slaves.insert(0x68, make_slave(&[(0x75, 0x71)]));
    slaves.insert(0x69, make_slave(&[(0x75, 0x71)]));

    slaves
}

fn make_slave(seed: &[(u8, u8)]) -> I2cSlave {
    let mut regs = [0u8; 256];
    for (reg, value) in seed {
        regs[*reg as usize] = *value;
    }

    I2cSlave {
        regs,
        pointer: 0,
        expecting_register: true,
    }
}

const I2C_CR1_PE: u32 = 1 << 0;
const I2C_CR1_START: u32 = 1 << 8;
const I2C_CR1_STOP: u32 = 1 << 9;
const I2C_CR1_SWRST: u32 = 1 << 15;

const I2C_CR2_ITERREN: u32 = 1 << 8;
const I2C_CR2_ITEVTEN: u32 = 1 << 9;
const I2C_CR2_DMAEN: u32 = 1 << 11;

const I2C_SR1_SB: u32 = 1 << 0;
const I2C_SR1_ADDR: u32 = 1 << 1;
const I2C_SR1_BTF: u32 = 1 << 2;
const I2C_SR1_ADD10: u32 = 1 << 3;
const I2C_SR1_RXNE: u32 = 1 << 6;
const I2C_SR1_TXE: u32 = 1 << 7;
const I2C_SR1_BERR: u32 = 1 << 8;
const I2C_SR1_ARLO: u32 = 1 << 9;
const I2C_SR1_AF: u32 = 1 << 10;
const I2C_SR1_OVR: u32 = 1 << 11;
const I2C_SR1_TIMEOUT: u32 = 1 << 14;
const I2C_SR1_SMBALERT: u32 = 1 << 15;

const I2C_SR2_MSL: u32 = 1 << 0;
const I2C_SR2_BUSY: u32 = 1 << 1;
const I2C_SR2_TRA: u32 = 1 << 2;

const I2C_ERROR_MASK: u32 =
    I2C_SR1_BERR | I2C_SR1_ARLO | I2C_SR1_AF | I2C_SR1_OVR | (1 << 12) | I2C_SR1_TIMEOUT | I2C_SR1_SMBALERT;
