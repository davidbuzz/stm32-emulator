// SPDX-License-Identifier: GPL-3.0-or-later

use crate::system::System;
use super::{Peripheral, nvic::irq};

#[derive(Default)]
pub struct Scb {
    aircr: u32,
    scr: u32,
    ccr: u32,
    shpr: [u32; 3],
    shcsr: u32,
    cfsr: u32,
    hfsr: u32,
    dfsr: u32,
    mmfar: u32,
    bfar: u32,
    afsr: u32,
    cpacr: u32,
}

const CPUID_CORTEX_M4: u32 = 0x410f_c241;
const AIRCR_VECTKEY: u32 = 0x05fa << 16;

impl Scb {
    pub fn new(name: &str) -> Option<Box<dyn Peripheral>> {
        if name == "SCB" {
            Some(Box::new(Self::default()))
        } else {
            None
        }
    }
}

impl Peripheral for Scb {
    fn read(&mut self, sys: &System, offset: u32) -> u32 {
        let value = match offset {
            0x0000 => CPUID_CORTEX_M4,
            0x0004 => {
                let nvic = sys.p.nvic.borrow();
                let active = sys.uc.borrow().reg_read(unicorn_engine::RegisterARM::IPSR).unwrap_or(0) as u32;
                let pending = nvic
                    .next_pending_intr()
                    .map(|irq| (16 + irq) as u32)
                    .unwrap_or(0);

                active
                    | (pending << 12)
                    | (((nvic.is_intr_pending(irq::SYSTICK) as u32) << 26))
                    | (((nvic.is_intr_pending(irq::PENDSV) as u32) << 28))
                    | (1 << 11)
            }
            0x0008 => sys.p.nvic.borrow().vector_table_addr,
            0x000c => self.aircr | AIRCR_VECTKEY,
            0x0010 => self.scr,
            0x0014 => self.ccr,
            0x0018 => self.shpr[0],
            0x001c => self.shpr[1],
            0x0020 => self.shpr[2],
            0x0024 => self.shcsr,
            0x0028 => self.cfsr,
            0x002c => self.hfsr,
            0x0030 => self.dfsr,
            0x0034 => self.mmfar,
            0x0038 => self.bfar,
            0x003c => self.afsr,
            0x0088 => self.cpacr,
            _ => 0,
        };

        trace!("SCB read offset=0x{:04x} value=0x{:08x}", offset, value);
        value
    }

    fn write(&mut self, sys: &System, offset: u32, value: u32) {
        trace!("SCB write offset=0x{:04x} value=0x{:08x}", offset, value);
        match offset {
            0x0004 => {
                // ICSR register
                // bit 25: clear SysTick pending
                // bit 26: set systick pending
                // bit 27: clear PendSV pending
                // bit 28: set PendSV pending
                if value & (1 << 25) != 0 {
                    sys.p.nvic.borrow_mut().clear_intr_pending(irq::SYSTICK);
                }
                if value & (1 << 26) != 0 {
                    sys.p.nvic.borrow_mut().set_intr_pending(irq::SYSTICK);
                }
                if value & (1 << 27) != 0 {
                    sys.p.nvic.borrow_mut().clear_intr_pending(irq::PENDSV);
                }
                if value & (1 << 28) != 0 {
                    sys.p.nvic.borrow_mut().set_intr_pending(irq::PENDSV);
                }
            }
            0x0008 => {
                sys.p.nvic.borrow_mut().vector_table_addr = value & 0xffff_ff80;
            }
            0x000c => {
                if value & 0xffff_0000 == AIRCR_VECTKEY {
                    self.aircr = value & 0x0000_ffff;
                }
            }
            0x0010 => self.scr = value,
            0x0014 => self.ccr = value,
            0x0018 => self.shpr[0] = value,
            0x001c => self.shpr[1] = value,
            0x0020 => self.shpr[2] = value,
            0x0024 => self.shcsr = value,
            0x0028 => self.cfsr &= !value,
            0x002c => self.hfsr &= !value,
            0x0030 => self.dfsr &= !value,
            0x0034 => self.mmfar = value,
            0x0038 => self.bfar = value,
            0x003c => self.afsr = value,
            0x0088 => self.cpacr = value,
            _ => {}
        }
    }
}
