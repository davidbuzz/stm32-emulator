// SPDX-License-Identifier: GPL-3.0-or-later

// STM32 name: NVIC (Nested Vectored Interrupt Controller) for Cortex-M4.
// Cortex-M private-peripheral bases: ISER starts at 0xE000E100; priority registers are at 0xE000E400.
// Key function: tracks pending interrupts/exceptions, dispatches vectors, and restores exception state.
// Critical for this emulator: ArduPilot boot relies on timer IRQ wakeups, SVC, SysTick, and DMA IRQ paths.
// This model is intentionally narrow and focuses on pending state plus exception entry/return stacking.
// It also cooperates with SCB VTOR so vectors come from the firmware-selected table.
// Still incomplete: full enable/priority/arbitration semantics and nested-interrupt behavior.
// Datasheet/reference anchor: ARMv7-M NVIC/exception architecture as used by STM32F427.

use std::sync::atomic::Ordering;

use unicorn_engine::{RegisterARM, Unicorn};

use crate::system::System;
use super::Peripheral;

#[derive(Default)]
pub struct Nvic {
    pub vector_table_addr: u32,
    pub systick_period: Option<u32>,
    pub last_systick_trigger: u64,

    // 128 different interrupts. Good enough for now
    pending: u128,
    enabled: u128,
    active_exceptions: u32,
    exc_return_stack: Vec<u32>,
    exc_stack_state: Vec<(u64, i32)>,
}

const IRQ_OFFSET: i32 = 16;

pub mod irq {
    pub const SVCALL: i32 = -5;
    pub const PENDSV: i32 = -2;
    pub const SYSTICK: i32 = -1;
}

// This is all poorly implemented. If this is not making much sense, it might be
// best to re-implement everything correctly. Right now, I'm just trying to get
// the saturn firmware to work just well enough.

impl Nvic {
    const FP_EXTENDED_FRAME_RESERVED_WORD: u32 = 0;

    fn push_word(uc: &mut Unicorn<()>, sp: &mut u64, value: u32) {
        *sp -= 4;
        uc.mem_write(*sp, &value.to_le_bytes())
            .expect("Invalid SP pointer during interrupt");
    }

    fn pop_word(uc: &mut Unicorn<()>, sp: &mut u64) -> u32 {
        let mut value = [0, 0, 0, 0];
        uc.mem_read(*sp, &mut value)
            .expect("Invalid SP pointer during interrupt return");
        *sp += 4;
        u32::from_le_bytes(value)
    }

    pub fn set_intr_pending(&mut self, irq: i32) {
        trace!("Set irq pending irq={}", irq);
        let bit = IRQ_OFFSET + irq;
        assert!(bit > 0);
        self.pending |= 1 << (IRQ_OFFSET + irq);
    }

    pub fn clear_intr_pending(&mut self, irq: i32) {
        let bit = IRQ_OFFSET + irq;
        assert!(bit > 0);
        self.pending &= !(1 << (IRQ_OFFSET + irq));
    }

    pub fn is_intr_pending(&self, irq: i32) -> bool {
        let bit = IRQ_OFFSET + irq;
        assert!(bit > 0);
        (self.pending & (1 << (IRQ_OFFSET + irq))) != 0
    }

    pub fn next_pending_intr(&self) -> Option<i32> {
        self.next_dispatchable_bit().map(|bit| (bit as i32) - IRQ_OFFSET)
    }

    pub fn get_and_clear_next_intr_pending(&mut self) -> Option<i32> {
        if let Some(bit) = self.next_dispatchable_bit() {
            self.pending &= !(1 << bit);
            let irq = (bit as i32) - IRQ_OFFSET;
            Some(irq)
        } else {
            None
        }
    }

    fn next_dispatchable_bit(&self) -> Option<u32> {
        let sys_pending_mask = (1u128 << IRQ_OFFSET) - 1;
        let sys_pending = self.pending & sys_pending_mask;
        if sys_pending != 0 {
            return Some(sys_pending.trailing_zeros());
        }

        let ext_pending = self.pending >> IRQ_OFFSET;
        let dispatchable = ext_pending & self.enabled;
        if dispatchable != 0 {
            Some(dispatchable.trailing_zeros() + IRQ_OFFSET as u32)
        } else {
            None
        }
    }

    pub fn maybe_set_systick_intr_pending(&mut self) {
        if let Some(systick_period) = self.systick_period {
            let n = crate::emulator::NUM_INSTRUCTIONS.load(Ordering::Relaxed);
            let delta_num_instructions = n - self.last_systick_trigger;
            if delta_num_instructions > (systick_period as u64) {
                trace!(
                    "SysTick matured n={} delta={} period={}",
                    n,
                    delta_num_instructions,
                    systick_period
                );
                self.last_systick_trigger = n;
                self.set_intr_pending(irq::SYSTICK);
            }
        }
    }

   fn are_interrupts_disabled(sys: &System) -> bool {
        let primask = sys.uc.borrow().reg_read(RegisterARM::PRIMASK).unwrap();
        primask != 0
    }

    fn basepri_masks_external_interrupts(sys: &System) -> bool {
        let basepri = sys.uc.borrow().reg_read(RegisterARM::BASEPRI).unwrap();
        basepri != 0
    }

    pub fn take_pending_interrupt(&mut self, sys: &System) -> Option<i32> {
        self.maybe_set_systick_intr_pending();

        let primask_disabled = Self::are_interrupts_disabled(sys);
        let basepri_masked = Self::basepri_masks_external_interrupts(sys);
        let current_exception = sys.uc.borrow().reg_read(RegisterARM::IPSR).unwrap();

        if primask_disabled || current_exception != 0 {
            trace!(
                "Interrupt dispatch blocked primask={} ipsr={} active_exceptions={} pending=0x{:032x}",
                sys.uc.borrow().reg_read(RegisterARM::PRIMASK).unwrap(),
                current_exception,
                self.active_exceptions,
                self.pending
            );
            return None;
        }

        if basepri_masked {
            let sys_pending_mask = (1u128 << IRQ_OFFSET) - 1;
            if self.pending & !sys_pending_mask != 0 {
                trace!(
                    "Interrupt dispatch blocked basepri={} pending=0x{:032x}",
                    sys.uc.borrow().reg_read(RegisterARM::BASEPRI).unwrap(),
                    self.pending,
                );
            }
        }

        if let Some(irq) = self.get_and_clear_next_intr_pending() {
            if irq >= 0 && basepri_masked {
                self.set_intr_pending(irq);
                return None;
            }
            Some(irq)
        } else {
            None
        }
    }

    fn read_vector_addr(sys: &System, vector_table_addr: u32, irq: i32) -> u32 {
        // 4 because of ptr size
        let vaddr = vector_table_addr + 4*(IRQ_OFFSET + irq) as u32;

        let mut vector = [0,0,0,0];
        sys.uc.borrow().mem_read(vaddr as u64, &mut vector).unwrap();
        u32::from_le_bytes(vector)
    }

    // SPSEL, bit[1], 0 means we use MSP, 1 means we use PSP.
    // FPCA, bit[2], if the processor includes the FP extension.

    pub fn run_interrupt(&mut self, sys: &System, irq: i32) {
        if irq == 67 || irq == 50 {
            debug!("NVIC dispatching IRQ {} vector={:#08x}", irq, Self::read_vector_addr(sys, self.vector_table_addr, irq));
        }
        let vector = Self::read_vector_addr(sys, self.vector_table_addr, irq);

        let mut uc = sys.uc.borrow_mut();
        let entry_msp = uc.reg_read(RegisterARM::MSP).unwrap();

        // SPSEL, bit[1], 0 means we use MSP, 1 means we use PSP.
        // FPCA, bit[2], if the processor includes the FP extension.
        let control_reg = uc.reg_read(RegisterARM::CONTROL).unwrap();
        let in_handler_mode = uc.reg_read(RegisterARM::IPSR).unwrap() != 0;
        let spsel = if in_handler_mode {
            // ARMv7-M uses MSP for exception entry while already in handler mode.
            false
        } else {
            control_reg & (1 << 1) != 0
        };
        let fpca = false;
        trace!("Running interrupt irq={} spsel={} fpca={} vector={:#08x}",
            irq, spsel, fpca, vector);

        Self::push_regs(&mut uc, spsel, fpca);

        // LR meaning:
        //   EXC_RETURN    Return to      Return stack Frame type
        //   0xFFFF_FFE1   Handler mode   Main         Extended
        //   0xFFFF_FFE9   Thread mode    Main         Extended
        //   0xFFFF_FFED   Thread mode    Process      Extended
        //   0xFFFF_FFF1   Handler mode   Main         Basic
        //   0xFFFF_FFF9   Thread mode    Main         Basic
        //   0xFFFF_FFFD   Thread mode    Process      Basic

        let mut lr: u32 = if in_handler_mode { 0xFFFF_FFE1 } else { 0xFFFF_FFE9 };
        if !in_handler_mode && spsel { lr |= 0b0000_0100; }
        if !fpca { lr |= 0b0001_0000; } // Yes, no fpca means the bit is set
        uc.reg_write(RegisterARM::LR, lr.into()).unwrap();

        let exception_number = (IRQ_OFFSET + irq) as u64;
        uc.reg_write(RegisterARM::IPSR, exception_number).unwrap();
        uc.reg_write(RegisterARM::PC, vector as u64).unwrap();

        self.exc_return_stack.push(lr);
        self.exc_stack_state.push((entry_msp, irq));
        self.active_exceptions = self.active_exceptions.saturating_add(1);
    }

    pub fn return_from_interrupt(&mut self, sys: &System) {
        let mut uc = sys.uc.borrow_mut();
        let live_lr = uc.reg_read(RegisterARM::LR).unwrap() as u32;
        let lr = self.exc_return_stack.pop().unwrap_or(live_lr) as u64;
        let (entry_msp, entry_irq) = self
            .exc_stack_state
            .pop()
            .unwrap_or((uc.reg_read(RegisterARM::MSP).unwrap(), -999));
        let restored_xpsr;
        if lr & 0xFFFF_FF00 == 0xFFFF_FF00 {
            let spsel = lr & 0b0000_0100 != 0;
            let fpca = false;

            Self::pop_regs(&mut uc, spsel, fpca);
            restored_xpsr = uc.reg_read(RegisterARM::XPSR).unwrap() as u32;

            trace!("Return from interrupt spsel={} fpca={} pc=0x{:08x}",
                spsel, fpca, uc.reg_read(RegisterARM::PC).unwrap());

            // SPSEL, bit[1], 0 means we use MSP, 1 means we use PSP.
            // FPCA, bit[2], if the processor includes the FP extension.
            let mut control_reg = uc.reg_read(RegisterARM::CONTROL).unwrap() as u32 & 0x1;
            if spsel { control_reg |= 1 << 1; }
            if fpca { control_reg |= 2 << 1; }
            uc.reg_write(RegisterARM::CONTROL, control_reg.into()).unwrap();
        } else {
            let control_reg = uc.reg_read(RegisterARM::CONTROL).unwrap();
            let spsel = control_reg & (1 << 1) != 0;
            let fpca = false;
            Self::pop_regs(&mut uc, spsel, fpca);
            restored_xpsr = uc.reg_read(RegisterARM::XPSR).unwrap() as u32;

            trace!(
                "Return from interrupt spsel={} fpca={} pc=0x{:08x} -- saved LR unavailable live_lr=0x{:08x}",
                spsel,
                fpca,
                uc.reg_read(RegisterARM::PC).unwrap(),
                live_lr
            );
        }

        let restored_ipsr = restored_xpsr & 0x1ff;
        uc.reg_write(RegisterARM::IPSR, restored_ipsr as u64).unwrap();

        let current_msp = uc.reg_read(RegisterARM::MSP).unwrap();
        if current_msp != entry_msp {
            warn!(
                "Exception return MSP mismatch irq={} lr=0x{:08x} msp 0x{:08x}->0x{:08x}; restoring saved MSP",
                entry_irq,
                lr as u32,
                entry_msp,
                current_msp,
            );
            uc.reg_write(RegisterARM::MSP, entry_msp).unwrap();
        }

        self.active_exceptions = self.active_exceptions.saturating_sub(1);
    }

    const CONTEXT_REGS_EXTENDED: [RegisterARM; 17] = [
        RegisterARM::FPSCR,
        RegisterARM::S15,
        RegisterARM::S14,
        RegisterARM::S13,
        RegisterARM::S12,
        RegisterARM::S11,
        RegisterARM::S10,
        RegisterARM::S9,
        RegisterARM::S8,
        RegisterARM::S7,
        RegisterARM::S6,
        RegisterARM::S5,
        RegisterARM::S4,
        RegisterARM::S3,
        RegisterARM::S2,
        RegisterARM::S1,
        RegisterARM::S0,
    ];

    const CONTEXT_REGS: [RegisterARM; 8] = [
        RegisterARM::XPSR,
        RegisterARM::PC,
        RegisterARM::LR,
        RegisterARM::R12,
        RegisterARM::R3,
        RegisterARM::R2,
        RegisterARM::R1,
        RegisterARM::R0,
    ];

    fn push_regs(uc: &mut Unicorn<()>, spsel: bool, fpca: bool) {
        let sp_reg = if spsel { RegisterARM::PSP } else { RegisterARM::MSP };
        let mut sp = uc.reg_read(sp_reg).unwrap();

        if fpca {
            // ARMv7-M extended frame layout (low→high): R0..XPSR | S0..S15, FPSCR | Reserved
            // Push Reserved first so it lands at the highest address in the FP section.
            Self::push_word(uc, &mut sp, Self::FP_EXTENDED_FRAME_RESERVED_WORD);

            for reg in Self::CONTEXT_REGS_EXTENDED {
                let value = uc.reg_read(reg).unwrap() as u32;
                Self::push_word(uc, &mut sp, value);
            }
        }
        for reg in Self::CONTEXT_REGS {
            let value = uc.reg_read(reg).unwrap() as u32;
            Self::push_word(uc, &mut sp, value);
        }
        uc.reg_write(sp_reg, sp).unwrap();
    }

    fn pop_regs(uc: &mut Unicorn<()>, spsel: bool, fpca: bool) {
        let sp_reg = if spsel { RegisterARM::PSP } else { RegisterARM::MSP };
        let mut sp = uc.reg_read(sp_reg).unwrap();

        for reg in Self::CONTEXT_REGS.iter().rev() {
            let value = Self::pop_word(uc, &mut sp);
            uc.reg_write(*reg, value as u64).unwrap();
        }
        if fpca {
            for reg in Self::CONTEXT_REGS_EXTENDED.iter().rev() {
                let value = Self::pop_word(uc, &mut sp);
                uc.reg_write(*reg, value as u64).unwrap();
            }

            // Reserved word is at the top of the FP section (highest address)
            let _reserved = Self::pop_word(uc, &mut sp);
        }
        uc.reg_write(sp_reg, sp).unwrap();
    }
}

impl Peripheral for Nvic {
    fn read(&mut self, _sys: &System, offset: u32) -> u32 {
        match offset {
            // ISER0..ISER3
            0x0000..=0x000c => {
                let idx = (offset / 4) as u32;
                ((self.enabled >> (idx * 32)) & 0xFFFF_FFFF) as u32
            }
            // ICER0..ICER3 reflects enable state too
            0x0080..=0x008c => {
                let idx = ((offset - 0x80) / 4) as u32;
                ((self.enabled >> (idx * 32)) & 0xFFFF_FFFF) as u32
            }
            // ISPR0..ISPR3
            0x0100..=0x010c => {
                let idx = ((offset - 0x100) / 4) as u32;
                let ext_pending = self.pending >> IRQ_OFFSET;
                ((ext_pending >> (idx * 32)) & 0xFFFF_FFFF) as u32
            }
            // ICPR0..ICPR3 reflects pending state
            0x0180..=0x018c => {
                let idx = ((offset - 0x180) / 4) as u32;
                let ext_pending = self.pending >> IRQ_OFFSET;
                ((ext_pending >> (idx * 32)) & 0xFFFF_FFFF) as u32
            }
            _ => 0,
        }
    }

    fn write(&mut self, _sys: &System, offset: u32, value: u32) {
        match offset {
            // ISER0..ISER3
            0x0000..=0x000c => {
                let idx = (offset / 4) as u32;
                debug!("NVIC ISER{} write value={:#010x} (enables irqs {}-{})", idx, value, idx*32, idx*32+31);
                self.enabled |= (value as u128) << (idx * 32);
            }
            // ICER0..ICER3
            0x0080..=0x008c => {
                let idx = ((offset - 0x80) / 4) as u32;
                debug!("NVIC ICER{} write value={:#010x} (disables irqs {}-{})", idx, value, idx*32, idx*32+31);
                self.enabled &= !((value as u128) << (idx * 32));
            }
            // ISPR0..ISPR3
            0x0100..=0x010c => {
                let idx = ((offset - 0x100) / 4) as u32;
                self.pending |= (value as u128) << (IRQ_OFFSET as u32 + idx * 32);
            }
            // ICPR0..ICPR3
            0x0180..=0x018c => {
                let idx = ((offset - 0x180) / 4) as u32;
                self.pending &= !((value as u128) << (IRQ_OFFSET as u32 + idx * 32));
            }
            _ => {}
        }
    }
}

/// The next part is glue. Maybe we could have a better architecture.

pub struct NvicWrapper;

impl NvicWrapper {
    pub fn new(name: &str) -> Option<Box<dyn Peripheral>> {
        if name == "NVIC" {
            Some(Box::new(Self))
        } else {
            None
        }
    }
}

impl Peripheral for NvicWrapper {
    fn read(&mut self, sys: &System, offset: u32) -> u32 {
        sys.p.nvic.borrow_mut().read(sys, offset)
    }

    fn write(&mut self, sys: &System, offset: u32, value: u32) {
        sys.p.nvic.borrow_mut().write(sys, offset, value)
    }
}


/*
0xE000E100 B  REGISTER ISER0 (rw): Interrupt Set-Enable Register
0xE000E104 B  REGISTER ISER1 (rw): Interrupt Set-Enable Register
0xE000E108 B  REGISTER ISER2 (rw): Interrupt Set-Enable Register

0xE000E180 B  REGISTER ICER0 (rw): Interrupt Clear-Enable Register
0xE000E184 B  REGISTER ICER1 (rw): Interrupt Clear-Enable Register
0xE000E188 B  REGISTER ICER2 (rw): Interrupt Clear-Enable Register

0xE000E200 B  REGISTER ISPR0 (rw): Interrupt Set-Pending Register
0xE000E204 B  REGISTER ISPR1 (rw): Interrupt Set-Pending Register
0xE000E208 B  REGISTER ISPR2 (rw): Interrupt Set-Pending Register

0xE000E280 B  REGISTER ICPR0 (rw): Interrupt Clear-Pending Register
0xE000E284 B  REGISTER ICPR1 (rw): Interrupt Clear-Pending Register
0xE000E288 B  REGISTER ICPR2 (rw): Interrupt Clear-Pending Register

0xE000E300 B  REGISTER IABR0 (ro): Interrupt Active Bit Register
0xE000E304 B  REGISTER IABR1 (ro): Interrupt Active Bit Register
0xE000E308 B  REGISTER IABR2 (ro): Interrupt Active Bit Register

0xE000E400 B  REGISTER IPR0 (rw): Interrupt Priority Register
0xE000E404 B  REGISTER IPR1 (rw): Interrupt Priority Register
0xE000E408 B  REGISTER IPR2 (rw): Interrupt Priority Register
0xE000E40C B  REGISTER IPR3 (rw): Interrupt Priority Register
0xE000E410 B  REGISTER IPR4 (rw): Interrupt Priority Register
0xE000E414 B  REGISTER IPR5 (rw): Interrupt Priority Register
0xE000E418 B  REGISTER IPR6 (rw): Interrupt Priority Register
0xE000E41C B  REGISTER IPR7 (rw): Interrupt Priority Register
0xE000E420 B  REGISTER IPR8 (rw): Interrupt Priority Register
0xE000E424 B  REGISTER IPR9 (rw): Interrupt Priority Register
0xE000E428 B  REGISTER IPR10 (rw): Interrupt Priority Register
0xE000E42C B  REGISTER IPR11 (rw): Interrupt Priority Register
0xE000E430 B  REGISTER IPR12 (rw): Interrupt Priority Register
0xE000E434 B  REGISTER IPR13 (rw): Interrupt Priority Register
0xE000E438 B  REGISTER IPR14 (rw): Interrupt Priority Register
0xE000E43C B  REGISTER IPR15 (rw): Interrupt Priority Register
0xE000E440 B  REGISTER IPR16 (rw): Interrupt Priority Register
0xE000E444 B  REGISTER IPR17 (rw): Interrupt Priority Register
0xE000E448 B  REGISTER IPR18 (rw): Interrupt Priority Register
0xE000E44C B  REGISTER IPR19 (rw): Interrupt Priority Register
*/
