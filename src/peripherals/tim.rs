// SPDX-License-Identifier: GPL-3.0-or-later

// STM32 name: TIM1-TIM14 (advanced/general-purpose/basic timers).
// STM32F427 bases: TIM1=0x40010000, TIM2=0x40000000, TIM3=0x40000400, TIM4=0x40000800,
//   TIM5=0x40000C00, TIM6=0x40001000, TIM7=0x40001400, TIM8=0x40010400,
//   TIM9=0x40014000, TIM10=0x40014400, TIM11=0x40014800,
//   TIM12=0x40001800, TIM13=0x40001C00, TIM14=0x40002000.
// Key registers: CR1, CR2, SMCR, DIER, SR, EGR, CNT, PSC, ARR, CCR1-CCR4.
// Key function: scheduler wakeups, timeouts, compare events, and runtime pacing.
// Critical for this emulator: TIM5 compare interrupt is what breaks the early idle-loop stall.
// Current model provides a free-running software timebase stepped from the emulator main loop.
// Still incomplete: input-capture, PWM, synchronization, DMA-request, and advanced TIM1/TIM8 features.
// Datasheet/reference anchor: STM32F4 RM TIM chapters.

use crate::{emulator::NUM_INSTRUCTIONS, system::System};

use super::Peripheral;

#[derive(Default)]
pub struct Tim {
    name: String,
    cr1: u32,
    cr2: u32,
    smcr: u32,
    dier: u32,
    sr: u32,
    cnt: u32,
    psc: u32,
    arr: u32,
    ccr1: u32,
    ccr2: u32,
    ccr3: u32,
    ccr4: u32,
    last_clk: u64,
    psc_accum: u64,
}

impl Tim {
    pub fn new(name: &str) -> Option<Box<dyn Peripheral>> {
        if name.starts_with("TIM") {
            Some(Box::new(Self {
                name: name.to_string(),
                arr: u32::MAX,
                ..Self::default()
            }))
        } else {
            None
        }
    }

    /// Return the primary (update) IRQ number for this timer.
    /// Advanced timers (TIM1/TIM8) have 4 separate IRQs; we use the update IRQ.
    fn irq_number(&self) -> Option<i32> {
        match self.name.as_str() {
            // Advanced timers – update IRQ
            "TIM1" => Some(25),
            "TIM8" => Some(44),
            // General-purpose 32-bit / 16-bit
            "TIM2"  => Some(28),
            "TIM3"  => Some(29),
            "TIM4"  => Some(30),
            "TIM5"  => Some(50),
            // Basic timers
            "TIM6"  => Some(54),
            "TIM7"  => Some(55),
            // APB2 advanced-control family
            "TIM9"  => Some(24),
            "TIM10" => Some(25),
            "TIM11" => Some(26),
            "TIM12" => Some(43),
            "TIM13" => Some(44),
            "TIM14" => Some(45),
            _ => None,
        }
    }

    /// Return an additional capture/compare IRQ if the timer has a dedicated CC irq.
    fn cc_irq_number(&self) -> Option<i32> {
        match self.name.as_str() {
            "TIM1" => Some(27),
            "TIM8" => Some(46),
            _ => None,
        }
    }

    fn tick(&mut self, sys: &System) {
        let now = NUM_INSTRUCTIONS.load(std::sync::atomic::Ordering::Relaxed);
        let delta = now.saturating_sub(self.last_clk) as u32;
        self.last_clk = now;

        if self.cr1 & 1 == 0 {
            return;
        }

        if delta == 0 {
            return;
        }

        let step = self.psc.saturating_add(1);
        self.psc_accum = self.psc_accum.saturating_add(delta as u64);

        let step = step.max(1) as u64;
        let ticks = (self.psc_accum / step) as u32;
        self.psc_accum %= step;

        if ticks == 0 {
            return;
        }

        let old_cnt = self.cnt;
        self.cnt = self.cnt.wrapping_add(ticks);

        // Fire compare once when the counter crosses CCR1-CCR4; flags remain set until cleared.
        let check_cc = |old: u32, new: u32, ccr: u32| -> bool {
            if old <= new { old < ccr && ccr <= new } else { old < ccr || ccr <= new }
        };

        // CCR1 – DIER bit 1, SR bit 1
        if (self.dier & (1 << 1)) != 0 && (self.sr & (1 << 1)) == 0 && check_cc(old_cnt, self.cnt, self.ccr1) {
            self.sr |= 1 << 1;
            let irq = self.cc_irq_number().or_else(|| self.irq_number());
            if let Some(irq) = irq {
                debug!("{} CC1 compare fired cnt=0x{:08x} ccr1=0x{:08x} -> IRQ {}", self.name, self.cnt, self.ccr1, irq);
                sys.p.nvic.borrow_mut().set_intr_pending(irq);
            }
        }
        // CCR2 – DIER bit 2, SR bit 2
        if (self.dier & (1 << 2)) != 0 && (self.sr & (1 << 2)) == 0 && check_cc(old_cnt, self.cnt, self.ccr2) {
            self.sr |= 1 << 2;
            let irq = self.cc_irq_number().or_else(|| self.irq_number());
            if let Some(irq) = irq { sys.p.nvic.borrow_mut().set_intr_pending(irq); }
        }
        // CCR3 – DIER bit 3, SR bit 3
        if (self.dier & (1 << 3)) != 0 && (self.sr & (1 << 3)) == 0 && check_cc(old_cnt, self.cnt, self.ccr3) {
            self.sr |= 1 << 3;
            let irq = self.cc_irq_number().or_else(|| self.irq_number());
            if let Some(irq) = irq { sys.p.nvic.borrow_mut().set_intr_pending(irq); }
        }
        // CCR4 – DIER bit 4, SR bit 4
        if (self.dier & (1 << 4)) != 0 && (self.sr & (1 << 4)) == 0 && check_cc(old_cnt, self.cnt, self.ccr4) {
            self.sr |= 1 << 4;
            let irq = self.cc_irq_number().or_else(|| self.irq_number());
            if let Some(irq) = irq { sys.p.nvic.borrow_mut().set_intr_pending(irq); }
        }

        if self.cnt >= self.arr {
            self.sr |= 1;
            self.cnt = 0;
            if (self.dier & 1) != 0 {
                if let Some(irq) = self.irq_number() {
                    sys.p.nvic.borrow_mut().set_intr_pending(irq);
                }
            }
        }
    }
}

impl Peripheral for Tim {
    fn step(&mut self, sys: &System) {
        self.tick(sys);
    }

    fn read(&mut self, sys: &System, offset: u32) -> u32 {
        self.tick(sys);

        match offset {
            0x0000 => self.cr1,
            0x0004 => self.cr2,
            0x0008 => self.smcr,
            0x000c => self.dier,
            0x0010 => self.sr,
            0x0014 => 0,   // EGR is write-only
            0x0024 => self.cnt,
            0x0028 => self.psc,
            0x002c => self.arr,
            0x0034 => self.ccr1,
            0x0038 => self.ccr2,
            0x003c => self.ccr3,
            0x0040 => self.ccr4,
            _ => 0,
        }
    }

    fn write(&mut self, sys: &System, offset: u32, value: u32) {
        self.tick(sys);

        match offset {
            0x0000 => {
                debug!("{} write CR1=0x{:08x}", self.name, value);
                self.cr1 = value;
            }
            0x0004 => self.cr2 = value,
            0x0008 => self.smcr = value,
            0x000c => {
                debug!("{} write DIER=0x{:08x}", self.name, value);
                self.dier = value;
            }
            // Firmware often clears status flags by writing 0 (write-0-to-clear).
            0x0010 => self.sr &= value,
            0x0014 => {
                // EGR (Event Generation Register): writing bit 0 forces an update event.
                if value & 1 != 0 {
                    self.sr |= 1; // set UIF
                    if (self.dier & 1) != 0 {
                        if let Some(irq) = self.irq_number() {
                            debug!("{} EGR UG -> update event -> IRQ {}", self.name, irq);
                            sys.p.nvic.borrow_mut().set_intr_pending(irq);
                        }
                    }
                }
                // CC event generation: bits 1-4 set corresponding CCxIF and fire CC IRQ.
                for cc in 1u32..=4 {
                    if (value >> cc) & 1 != 0 {
                        self.sr |= 1 << cc;
                        let irq_num = self.cc_irq_number().or_else(|| self.irq_number());
                        if let Some(irq) = irq_num {
                            if (self.dier >> cc) & 1 != 0 {
                                sys.p.nvic.borrow_mut().set_intr_pending(irq);
                            }
                        }
                    }
                }
            }
            0x0024 => {
                debug!("{} write CNT=0x{:08x}", self.name, value);
                self.cnt = value;
            }
            0x0028 => {
                debug!("{} write PSC=0x{:08x}", self.name, value);
                self.psc = value;
            }
            0x002c => {
                debug!("{} write ARR=0x{:08x}", self.name, value);
                self.arr = value;
            }
            0x0034 => {
                debug!("{} write CCR1=0x{:08x} (cnt=0x{:08x})", self.name, value, self.cnt);
                self.ccr1 = value;
            }
            0x0038 => self.ccr2 = value,
            0x003c => self.ccr3 = value,
            0x0040 => self.ccr4 = value,
            _ => {}
        }
    }
}
