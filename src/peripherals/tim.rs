// SPDX-License-Identifier: GPL-3.0-or-later

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
        self.cnt = self.cnt.wrapping_add(delta / step.max(1));

        // Minimal compare behavior used by polling/timeout loops.
        if (self.dier & (1 << 1)) != 0 && self.cnt >= self.ccr1 {
            self.sr |= 1 << 1;
            if let Some(irq) = self.irq_number() {
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
            0x0000 => self.cr1 = value,
            0x000c => self.dier = value,
            // Firmware often clears status flags by writing 0.
            0x0010 => self.sr &= value,
            0x0024 => self.cnt = value,
            0x0028 => self.psc = value,
            0x002c => self.arr = value,
            0x0034 => self.ccr1 = value,
            _ => {}
        }
    }
}
