// SPDX-License-Identifier: GPL-3.0-or-later

// STM32 name: TIM5 / TIM6 / TIM7 in the current model (general/basic timers).
// STM32F427 bases: TIM5=0x40000C00, TIM6=0x40001000, TIM7=0x40001400.
// Key registers: CR1, DIER, SR, CNT, PSC, ARR, and CCR1 in this implementation.
// Key function: scheduler wakeups, timeouts, compare events, and runtime pacing.
// Critical for this emulator: TIM5 compare interrupt is what breaks the early idle-loop stall.
// Current model provides a free-running software timebase stepped from the emulator main loop.
// Still incomplete: many channels/modes, update events, synchronization, and DMA request behavior.
// Datasheet/reference anchor: STM32F4 RM general-purpose/basic timer chapters.

use crate::{emulator::NUM_INSTRUCTIONS, system::System};

use super::Peripheral;

#[derive(Default)]
pub struct Tim {
    name: String,
    cr1: u32,
    dier: u32,
    sr: u32,
    cnt: u32,
    psc: u32,
    arr: u32,
    ccr1: u32,
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

    fn irq_number(&self) -> Option<i32> {
        match self.name.as_str() {
            "TIM5" => Some(50),
            "TIM6" => Some(54),
            "TIM7" => Some(55),
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

        self.cnt = self.cnt.wrapping_add(ticks);

        // Minimal compare behavior used by polling/timeout loops.
        if (self.dier & (1 << 1)) != 0 && self.cnt >= self.ccr1 {
            self.sr |= 1 << 1;
            if let Some(irq) = self.irq_number() {
                debug!("{} CC1 compare fired cnt=0x{:08x} ccr1=0x{:08x} -> IRQ {}", self.name, self.cnt, self.ccr1, irq);
                sys.p.nvic.borrow_mut().set_intr_pending(irq);
            }
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
            0x000c => self.dier,
            0x0010 => self.sr,
            0x0024 => self.cnt,
            0x0028 => self.psc,
            0x002c => self.arr,
            0x0034 => self.ccr1,
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
            0x000c => {
                debug!("{} write DIER=0x{:08x}", self.name, value);
                self.dier = value;
            }
            // Firmware often clears status flags by writing 0.
            0x0010 => self.sr &= value,
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
            _ => {}
        }
    }
}
