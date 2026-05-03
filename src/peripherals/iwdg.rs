// SPDX-License-Identifier: GPL-3.0-or-later

// STM32 name: IWDG (independent watchdog).
// STM32F427 base: 0x40003000.
// Minimal model: expose KR/PR/RLR/SR with unlock/reload behavior so firmware init
// paths can proceed without full watchdog timing emulation.

use crate::system::System;
use unicorn_engine::RegisterARM;

use super::Peripheral;

const IWDG_KR_UNLOCK: u32 = 0x5555;
const IWDG_KR_RELOAD: u32 = 0xAAAA;
const IWDG_KR_START: u32 = 0xCCCC;
const IWDG_SR_PVU: u32 = 1 << 0;
const IWDG_SR_RVU: u32 = 1 << 1;
const IWDG_UPDATE_DELAY_STEPS: u8 = 4;

pub struct Iwdg {
    kr: u32,
    pr: u32,
    rlr: u32,
    sr: u32,
    write_unlocked: bool,
    running: bool,
    counter: u16,
    prescaler_accum: u32,
    pvu_delay: u8,
    rvu_delay: u8,
    pending_pr: Option<u32>,
    pending_rlr: Option<u32>,
}

impl Iwdg {
    pub fn new(name: &str) -> Option<Box<dyn Peripheral>> {
        if name == "IWDG" {
            Some(Box::new(Self {
                kr: 0,
                pr: 0,
                rlr: 0x0FFF,
                sr: 0,
                write_unlocked: false,
                running: false,
                counter: 0x0FFF,
                prescaler_accum: 0,
                pvu_delay: 0,
                rvu_delay: 0,
                pending_pr: None,
                pending_rlr: None,
            }))
        } else {
            None
        }
    }

    fn prescaler_div(&self) -> u32 {
        // PR[2:0] -> divide by {4,8,16,32,64,128,256}
        match self.pr & 0x7 {
            0 => 4,
            1 => 8,
            2 => 16,
            3 => 32,
            4 => 64,
            5 => 128,
            _ => 256,
        }
    }

    fn apply_pending_updates(&mut self) {
        if self.pvu_delay > 0 {
            self.pvu_delay -= 1;
            if self.pvu_delay == 0 {
                if let Some(pr) = self.pending_pr.take() {
                    self.pr = pr & 0x7;
                }
                self.sr &= !IWDG_SR_PVU;
            }
        }

        if self.rvu_delay > 0 {
            self.rvu_delay -= 1;
            if self.rvu_delay == 0 {
                if let Some(rlr) = self.pending_rlr.take() {
                    self.rlr = rlr & 0x0FFF;
                }
                self.sr &= !IWDG_SR_RVU;
            }
        }
    }

    fn perform_system_reset(&mut self, sys: &System) {
        let vtor = sys.p.nvic.borrow().vector_table_addr;
        let mut msp_raw = [0u8; 4];
        let mut reset_raw = [0u8; 4];
        if sys.uc.borrow().mem_read(vtor as u64, &mut msp_raw).is_err()
            || sys.uc.borrow().mem_read((vtor + 4) as u64, &mut reset_raw).is_err()
        {
            warn!("IWDG reset: failed to read vectors from VTOR=0x{:08x}", vtor);
            return;
        }

        let msp = u32::from_le_bytes(msp_raw);
        let mut pc = u32::from_le_bytes(reset_raw);
        if (pc & 1) == 0 {
            pc |= 1;
        }

        {
            let mut uc = sys.uc.borrow_mut();
            let _ = uc.reg_write(RegisterARM::MSP, msp as u64);
            let _ = uc.reg_write(RegisterARM::SP, msp as u64);
            let _ = uc.reg_write(RegisterARM::PC, pc as u64);
            let _ = uc.reg_write(RegisterARM::IPSR, 0);
            let _ = uc.reg_write(RegisterARM::CONTROL, 0);
            let _ = uc.reg_write(RegisterARM::PRIMASK, 0);
            let _ = uc.reg_write(RegisterARM::BASEPRI, 0);
            let _ = uc.reg_write(RegisterARM::FAULTMASK, 0);
        }

        sys.p.nvic.borrow_mut().system_reset();

        // Reset watchdog state to power-on defaults after reset event.
        self.kr = 0;
        self.pr = 0;
        self.rlr = 0x0FFF;
        self.sr = 0;
        self.write_unlocked = false;
        self.running = false;
        self.counter = 0x0FFF;
        self.prescaler_accum = 0;
        self.pvu_delay = 0;
        self.rvu_delay = 0;
        self.pending_pr = None;
        self.pending_rlr = None;
    }
}

impl Peripheral for Iwdg {
    fn step(&mut self, sys: &System) {
        self.apply_pending_updates();

        if !self.running {
            return;
        }

        self.prescaler_accum = self.prescaler_accum.saturating_add(1);
        if self.prescaler_accum < self.prescaler_div() {
            return;
        }
        self.prescaler_accum = 0;

        if self.counter > 0 {
            self.counter -= 1;
            return;
        }

        warn!("IWDG timeout: triggering system reset");
        self.perform_system_reset(sys);
    }

    fn read(&mut self, _sys: &System, offset: u32) -> u32 {
        match offset {
            0x00 => self.kr,
            0x04 => self.pr & 0x7,
            0x08 => self.rlr & 0x0FFF,
            0x0C => self.sr & 0x7,
            _ => 0,
        }
    }

    fn write(&mut self, _sys: &System, offset: u32, value: u32) {
        match offset {
            0x00 => {
                self.kr = value;
                match value {
                    IWDG_KR_UNLOCK => self.write_unlocked = true,
                    IWDG_KR_RELOAD => {
                        self.counter = (self.rlr & 0x0FFF) as u16;
                    }
                    IWDG_KR_START => {
                        self.running = true;
                        self.prescaler_accum = 0;
                        self.counter = (self.rlr & 0x0FFF) as u16;
                    }
                    _ => {}
                }
            }
            0x04 => {
                if self.write_unlocked {
                    self.pending_pr = Some(value & 0x7);
                    self.sr |= IWDG_SR_PVU;
                    self.pvu_delay = IWDG_UPDATE_DELAY_STEPS;
                    self.write_unlocked = false;
                }
            }
            0x08 => {
                if self.write_unlocked {
                    self.pending_rlr = Some(value & 0x0FFF);
                    self.sr |= IWDG_SR_RVU;
                    self.rvu_delay = IWDG_UPDATE_DELAY_STEPS;
                    self.write_unlocked = false;
                }
            }
            _ => {}
        }
    }
}
