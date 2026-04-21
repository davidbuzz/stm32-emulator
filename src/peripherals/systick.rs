// SPDX-License-Identifier: GPL-3.0-or-later

// STM32 name: STK / SysTick timer (Cortex-M system timer).
// Cortex-M private-peripheral base: 0xE000E010.
// Key registers: CTRL, LOAD, VAL, CALIB.
// Key function: periodic exception source for RTOS tick and timebase services.
// Critical for this emulator: scheduler progression and timeout logic may depend on SysTick delivery.
// This model stores control/reload state and cooperates with NVIC to raise SysTick when configured.
// Still incomplete: exact decrement timing, calibration semantics, and all CTRL side effects.
// Datasheet/reference anchor: ARMv7-M SysTick architecture as used by STM32F427.

use crate::system::System;
use super::Peripheral;

#[derive(Default)]
pub struct SysTick {
    ctl: u32,
    reload: u32,
    val_toggle: bool,
}

impl SysTick {
    pub fn new(name: &str) -> Option<Box<dyn Peripheral>> {
        if name == "STK" {
            Some(Box::new(Self::default()))
        } else {
            None
        }
    }

    fn has_int_enabled(&self) -> bool {
        (self.ctl & 0b11) == 0b11
    }

    fn set_nvic_systick_period(&self, sys: &System) {
        let nvic_systick_period = if self.has_int_enabled() {
            Some(self.reload)
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
    fn read(&mut self, _sys: &System, offset: u32) -> u32 {
        match offset {
            0x0000 => {
                self.val_toggle = !self.val_toggle;
                // toggle the count bit
                self.ctl | if self.val_toggle { 0 } else { 1 << 16 }
            }
            0x0004 => self.reload,
            0x0008 => {
                self.val_toggle = !self.val_toggle;
                if self.val_toggle { 0 } else { self.reload/2 }
            }
            _ => 0
        }
    }

    fn write(&mut self, sys: &System, offset: u32, value: u32) {
        match offset {
            0x0000 => {
                // CTRL register
                trace!("SysTick write CTRL=0x{:08x}", value);
                self.ctl = value;
                self.set_nvic_systick_period(sys);
            }
            0x0004 => {
                // LOAD register
                trace!("SysTick write LOAD=0x{:08x}", value);
                self.reload = value;
                self.set_nvic_systick_period(sys);
            }
            _ => {}
        }
    }
}
