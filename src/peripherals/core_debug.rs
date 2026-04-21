// SPDX-License-Identifier: GPL-3.0-or-later

// STM32 name: CoreDebug / DWT (Cortex-M4 debug and data watchpoint/trace blocks).
// Cortex-M private-peripheral addresses used here: DEMCR=0xE000EDFC, DWT_CTRL=0xE0001000, DWT_CYCCNT=0xE0001004.
// Key function: debug enable state and cycle counting used by firmware timing/profiling code.
// Critical for this emulator: ArduPilot startup reads DWT_CYCCNT and enables tracing during early init.
// This model tracks only the minimum state needed for monotonic CYCCNT behavior.
// Current behavior: CYCCNT advances with emulator instruction count when TRCENA and CYCCNTENA are set.
// Still incomplete: broader DWT/CoreDebug register coverage and real debug side effects.
// Datasheet/reference anchor: ARMv7-M CoreDebug and DWT architecture.

use std::sync::atomic::Ordering;

use crate::{emulator::NUM_INSTRUCTIONS, system::System};

#[derive(Default)]
pub struct CoreDebug {
    demcr: u32,
    dwt_ctrl: u32,
    dwt_cyccnt: u32,
    last_clk: u64,
}

impl CoreDebug {
    const DEMCR_ADDR: u32 = 0xE000_EDFC;
    const DWT_CTRL_ADDR: u32 = 0xE000_1000;
    const DWT_CYCCNT_ADDR: u32 = 0xE000_1004;
    const TRCENA: u32 = 1 << 24;
    const CYCCNTENA: u32 = 1;

    pub fn handles(&self, addr: u32) -> bool {
        matches!(addr, Self::DEMCR_ADDR | Self::DWT_CTRL_ADDR | Self::DWT_CYCCNT_ADDR)
    }

    pub fn reg_name(&self, addr: u32) -> &'static str {
        match addr {
            Self::DEMCR_ADDR => "reg=DEMCR",
            Self::DWT_CTRL_ADDR => "reg=DWT_CTRL",
            Self::DWT_CYCCNT_ADDR => "reg=DWT_CYCCNT",
            _ => "reg=????",
        }
    }

    fn reset_clock_reference(&mut self) {
        self.last_clk = NUM_INSTRUCTIONS.load(Ordering::Relaxed);
    }

    fn counter_enabled(&self) -> bool {
        (self.demcr & Self::TRCENA) != 0 && (self.dwt_ctrl & Self::CYCCNTENA) != 0
    }

    pub fn read(&mut self, sys: &System, addr: u32) -> u32 {
        self.step(sys);

        match addr {
            Self::DEMCR_ADDR => self.demcr,
            Self::DWT_CTRL_ADDR => self.dwt_ctrl,
            Self::DWT_CYCCNT_ADDR => self.dwt_cyccnt,
            _ => 0,
        }
    }

    pub fn write(&mut self, sys: &System, addr: u32, value: u32) {
        self.step(sys);

        match addr {
            Self::DEMCR_ADDR => {
                self.demcr = value;
                self.reset_clock_reference();
            }
            Self::DWT_CTRL_ADDR => {
                self.dwt_ctrl = value;
                self.reset_clock_reference();
            }
            Self::DWT_CYCCNT_ADDR => {
                self.dwt_cyccnt = value;
                self.reset_clock_reference();
            }
            _ => {}
        }
    }

    pub fn step(&mut self, _sys: &System) {
        let now = NUM_INSTRUCTIONS.load(Ordering::Relaxed);
        let delta = now.saturating_sub(self.last_clk);
        self.last_clk = now;

        if !self.counter_enabled() || delta == 0 {
            return;
        }

        self.dwt_cyccnt = self.dwt_cyccnt.wrapping_add(delta as u32);
    }
}
