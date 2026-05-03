// SPDX-License-Identifier: GPL-3.0-or-later

// STM32 name: SCB (System Control Block) for Cortex-M4.
// Cortex-M private-peripheral base: 0xE000ED00.
// Key registers: CPUID, ICSR, VTOR, AIRCR, SCR, CCR, SHPRx, SHCSR, CFSR, HFSR, CPACR.
// Key function: exception control, vector-table selection, system handlers, and fault status.
// Critical for this emulator: VTOR, ICSR, and system-handler state affect every interrupt path.
// This model keeps just enough architectural state for firmware boot and exception routing.
// Still incomplete: deep fault generation/escalation and many side effects of system control writes.
// Datasheet/reference anchor: ARMv7-M SCB architecture as exposed on STM32F427.

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
const HFSR_FORCED: u32 = 1 << 30;
const CFSR_IBUSERR: u32 = 1 << 8;
const CFSR_PRECISERR: u32 = 1 << 9;
const CFSR_BFARVALID: u32 = 1 << 15;
const CFSR_INVSTATE: u32 = 1 << 17;

impl Scb {
    pub fn read(&mut self, sys: &System, offset: u32) -> u32 {
        let value = match offset {
            0x0000 => CPUID_CORTEX_M4,
            0x0004 => {
                let nvic = sys.p.nvic.borrow();
                let active = sys.uc.borrow().reg_read(unicorn_engine::RegisterARM::IPSR).unwrap_or(0) as u32;
                let pending = nvic
                    .next_pending_intr()
                    .map(|irq| (16 + irq) as u32)
                    .unwrap_or(0);
                let ret_to_base = if active == 0 || nvic.active_exception_depth() <= 1 {
                    1
                } else {
                    0
                };

                active
                    | (pending << 12)
                    | (((nvic.is_intr_pending(irq::SYSTICK) as u32) << 26))
                    | (((nvic.is_intr_pending(irq::PENDSV) as u32) << 28))
                    | (ret_to_base << 11)
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

    pub fn write(&mut self, sys: &System, offset: u32, value: u32) {
        trace!("SCB write offset=0x{:04x} value=0x{:08x}", offset, value);
        match offset {
            0x0004 => {
                // ICSR register
                // bit 25: clear SysTick pending
                // bit 26: set systick pending
                // bit 27: clear PendSV pending
                // bit 28: set PendSV pending
                let control = value & ((1 << 25) | (1 << 26) | (1 << 27) | (1 << 28));
                if control & (1 << 25) != 0 {
                    sys.p.nvic.borrow_mut().clear_intr_pending(irq::SYSTICK);
                }
                if control & (1 << 26) != 0 {
                    sys.p.nvic.borrow_mut().set_intr_pending(irq::SYSTICK);
                }
                if control & (1 << 27) != 0 {
                    sys.p.nvic.borrow_mut().clear_intr_pending(irq::PENDSV);
                }
                if control & (1 << 28) != 0 {
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
            0x0018 => {
                self.shpr[0] = value;
                sys.p.nvic.borrow_mut().set_system_handler_priority_reg(0, value);
            }
            0x001c => {
                self.shpr[1] = value;
                sys.p.nvic.borrow_mut().set_system_handler_priority_reg(1, value);
            }
            0x0020 => {
                self.shpr[2] = value;
                sys.p.nvic.borrow_mut().set_system_handler_priority_reg(2, value);
            }
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

    pub fn record_bus_fault(&mut self, addr: Option<u32>, instruction_fetch: bool, escalated: bool) {
        if instruction_fetch {
            self.cfsr |= CFSR_IBUSERR;
        } else {
            self.cfsr |= CFSR_PRECISERR;
            if let Some(addr) = addr {
                self.bfar = addr;
                self.cfsr |= CFSR_BFARVALID;
            }
        }

        if escalated {
            self.hfsr |= HFSR_FORCED;
        }
    }

    pub fn record_usagefault_invstate(&mut self, escalated: bool) {
        self.cfsr |= CFSR_INVSTATE;
        if escalated {
            self.hfsr |= HFSR_FORCED;
        }
    }
}

pub struct ScbWrapper;

impl ScbWrapper {
    pub fn new(name: &str) -> Option<Box<dyn Peripheral>> {
        if name == "SCB" {
            Some(Box::new(Self))
        } else {
            None
        }
    }
}

impl Peripheral for ScbWrapper {
    fn read(&mut self, sys: &System, offset: u32) -> u32 {
        sys.p.scb.borrow_mut().read(sys, offset)
    }

    fn write(&mut self, sys: &System, offset: u32, value: u32) {
        sys.p.scb.borrow_mut().write(sys, offset, value)
    }
}
