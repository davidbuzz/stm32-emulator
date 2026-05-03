// SPDX-License-Identifier: GPL-3.0-or-later

// STM32 name: STK / SysTick timer (Cortex-M system timer).
// Cortex-M private-peripheral base: 0xE000E010.
// Key registers: CTRL, LOAD, VAL, CALIB.
// Key function: periodic exception source for RTOS tick and timebase services.
// Critical for this emulator: scheduler progression and timeout logic may depend on SysTick delivery.
// This model stores control/reload state and cooperates with NVIC to raise SysTick when configured.
// Still incomplete: exact decrement timing, calibration semantics, and all CTRL side effects.
// Datasheet/reference anchor: ARMv7-M SysTick architecture as used by STM32F427.

use crate::{emulator::NUM_INSTRUCTIONS, system::System};
use super::Peripheral;

#[derive(Default)]
pub struct SysTick {
    ctl: u32,
    reload: u32,
    current: u32,
    countflag: bool,
    last_clk: u64,
    clksource_div_remainder: u8,
}

impl SysTick {
    pub fn new(name: &str) -> Option<Box<dyn Peripheral>> {
        if name == "STK" {
            Some(Box::new(Self::default()))
        } else {
            None
        }
    }

    fn enabled(&self) -> bool {
        (self.ctl & 1) != 0
    }

    fn tickint_enabled(&self) -> bool {
        (self.ctl & (1 << 1)) != 0
    }

    fn clksource_ahb(&self) -> bool {
        (self.ctl & (1 << 2)) != 0
    }

    fn reload_value(&self) -> u32 {
        self.reload & 0x00ff_ffff
    }

    fn current_value(&self) -> u32 {
        self.current & 0x00ff_ffff
    }

    fn update_counter(&mut self) {
        let now = NUM_INSTRUCTIONS.load(std::sync::atomic::Ordering::Relaxed);
        let delta = now.saturating_sub(self.last_clk);
        self.last_clk = now;

        if !self.enabled() {
            return;
        }

        let tick_div: u64 = if self.clksource_ahb() { 1 } else { 8 };
        let total = delta.saturating_add(self.clksource_div_remainder as u64);
        let mut ticks = total / tick_div;
        self.clksource_div_remainder = (total % tick_div) as u8;

        let reload = self.reload_value();
        while ticks > 0 {
            if self.current == 0 {
                self.current = reload;
                self.countflag = true;
            } else {
                self.current = self.current.saturating_sub(1);
                if self.current == 0 {
                    self.countflag = true;
                }
            }
            ticks -= 1;
        }
    }

    fn set_nvic_systick_period(&self, sys: &System) {
        let nvic_systick_period = if self.enabled() && self.tickint_enabled() {
            let base = self.reload_value().saturating_add(1);
            let div = if self.clksource_ahb() { 1 } else { 8 };
            Some(base.saturating_mul(div))
        } else {
            None
        };

        trace!(
            "SysTick config ctl=0x{:08x} reload=0x{:08x} period={:?}",
            self.ctl,
            self.reload,
            nvic_systick_period
        );
        sys.p.nvic.borrow_mut().systick_period = nvic_systick_period;
    }
}

impl Peripheral for SysTick {
    fn step(&mut self, _sys: &System) {
        self.update_counter();
    }

    fn read(&mut self, _sys: &System, offset: u32) -> u32 {
        self.update_counter();

        match offset {
            0x0000 => {
                let mut ctrl = self.ctl & 0x0001_0007;
                if self.countflag {
                    ctrl |= 1 << 16;
                }
                // COUNTFLAG is cleared on CTRL read.
                self.countflag = false;
                ctrl
            }
            0x0004 => self.reload_value(),
            0x0008 => self.current_value(),
            // CALIB: no reference clock modeled (NOREF=1), calibration value unknown.
            0x000c => 1 << 31,
            _ => 0
        }
    }

    fn write(&mut self, sys: &System, offset: u32, value: u32) {
        self.update_counter();

        match offset {
            0x0000 => {
                // CTRL register
                trace!("SysTick write CTRL=0x{:08x}", value);
                self.ctl = value & 0x0001_0007;
                if self.enabled() && self.current == 0 {
                    self.current = self.reload_value();
                }
                if !self.enabled() {
                    self.clksource_div_remainder = 0;
                }
                self.set_nvic_systick_period(sys);
            }
            0x0004 => {
                // LOAD register
                trace!("SysTick write LOAD=0x{:08x}", value);
                self.reload = value & 0x00ff_ffff;
                self.set_nvic_systick_period(sys);
            }
            0x0008 => {
                // Writing VAL clears the current count and COUNTFLAG.
                self.current = 0;
                self.countflag = false;
            }
            _ => {}
        }
    }
}
