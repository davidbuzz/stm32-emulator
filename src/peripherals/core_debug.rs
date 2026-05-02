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
    /// DWT_CPICNT (E0001008): CPI (Cycles Per Instruction) counter
    dwt_cpicnt: u32,
    /// DWT_EXCCNT (E000100C): Exception overhead counter
    dwt_exccnt: u32,
    /// DWT_SLEEPCNT (E0001010): Sleep cycle counter
    dwt_sleepcnt: u32,
    /// DWT_LSUCNT (E0001014): Load/Store Unit counter
    dwt_lsucnt: u32,
    /// DWT_FOLDCNT (E0001018): Folding counter (unused instructions)
    dwt_foldcnt: u32,
    /// DWT_PCSR (E000101C): Program Counter Sampling Register
    dwt_pcsr: u32,
    /// DWT_COMP0-3 (E0001020+): Comparator value registers
    dwt_comp: [u32; 4],
    /// DWT_MASK0-3 (E0001040+): Comparator address mask registers
    dwt_mask: [u32; 4],
    /// DWT_FUNCTION0-3 (E0001060+): Comparator function selection
    dwt_function: [u32; 4],
    last_clk: u64,
}

impl CoreDebug {
    const DEMCR_ADDR: u32 = 0xE000_EDFC;
    const DWT_CTRL_ADDR: u32 = 0xE000_1000;
    const DWT_CYCCNT_ADDR: u32 = 0xE000_1004;
    const DWT_CPICNT_ADDR: u32 = 0xE000_1008;
    const DWT_EXCCNT_ADDR: u32 = 0xE000_100C;
    const DWT_SLEEPCNT_ADDR: u32 = 0xE000_1010;
    const DWT_LSUCNT_ADDR: u32 = 0xE000_1014;
    const DWT_FOLDCNT_ADDR: u32 = 0xE000_1018;
    const DWT_PCSR_ADDR: u32 = 0xE000_101C;
    const DWT_COMP0_ADDR: u32 = 0xE000_1020;
    const DWT_MASK0_ADDR: u32 = 0xE000_1040;
    const DWT_FUNCTION0_ADDR: u32 = 0xE000_1060;
    const TRCENA: u32 = 1 << 24;
    const CYCCNTENA: u32 = 1;
    const DEMCR_WRITABLE_MASK: u32 = (1 << 24) | (1 << 16);

    pub fn handles(&self, addr: u32) -> bool {
        matches!(addr,
            Self::DEMCR_ADDR | Self::DWT_CTRL_ADDR | Self::DWT_CYCCNT_ADDR |
            Self::DWT_CPICNT_ADDR | Self::DWT_EXCCNT_ADDR | Self::DWT_SLEEPCNT_ADDR |
            Self::DWT_LSUCNT_ADDR | Self::DWT_FOLDCNT_ADDR | Self::DWT_PCSR_ADDR |
            0xE000_1020..=0xE000_102F | // COMP0-3
            0xE000_1040..=0xE000_104F | // MASK0-3
            0xE000_1060..=0xE000_106F   // FUNCTION0-3
        )
    }

    pub fn reg_name(&self, addr: u32) -> &'static str {
        match addr {
            Self::DEMCR_ADDR => "reg=DEMCR",
            Self::DWT_CTRL_ADDR => "reg=DWT_CTRL",
            Self::DWT_CYCCNT_ADDR => "reg=DWT_CYCCNT",
            Self::DWT_CPICNT_ADDR => "reg=DWT_CPICNT",
            Self::DWT_EXCCNT_ADDR => "reg=DWT_EXCCNT",
            Self::DWT_SLEEPCNT_ADDR => "reg=DWT_SLEEPCNT",
            Self::DWT_LSUCNT_ADDR => "reg=DWT_LSUCNT",
            Self::DWT_FOLDCNT_ADDR => "reg=DWT_FOLDCNT",
            Self::DWT_PCSR_ADDR => "reg=DWT_PCSR",
            0xE000_1020..=0xE000_102F => "reg=DWT_COMPx",
            0xE000_1040..=0xE000_104F => "reg=DWT_MASKx",
            0xE000_1060..=0xE000_106F => "reg=DWT_FUNCTIONx",
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
            Self::DWT_CPICNT_ADDR => self.dwt_cpicnt,
            Self::DWT_EXCCNT_ADDR => self.dwt_exccnt,
            Self::DWT_SLEEPCNT_ADDR => self.dwt_sleepcnt,
            Self::DWT_LSUCNT_ADDR => self.dwt_lsucnt,
            Self::DWT_FOLDCNT_ADDR => self.dwt_foldcnt,
            Self::DWT_PCSR_ADDR => self.dwt_pcsr,
            // DWT comparator registers: 4 sets at offsets 0x20-0x2F, 0x40-0x4F, 0x60-0x6F
            0xE000_1020..=0xE000_102F => {
                let idx = ((addr - 0xE000_1020) >> 2) as usize;
                if idx < 4 { self.dwt_comp[idx] } else { 0 }
            }
            0xE000_1040..=0xE000_104F => {
                let idx = ((addr - 0xE000_1040) >> 2) as usize;
                if idx < 4 { self.dwt_mask[idx] } else { 0 }
            }
            0xE000_1060..=0xE000_106F => {
                let idx = ((addr - 0xE000_1060) >> 2) as usize;
                if idx < 4 { self.dwt_function[idx] } else { 0 }
            }
            _ => 0,
        }
    }

    pub fn write(&mut self, sys: &System, addr: u32, value: u32) {
        self.step(sys);

        match addr {
            Self::DEMCR_ADDR => {
                self.demcr = (self.demcr & !Self::DEMCR_WRITABLE_MASK)
                    | (value & Self::DEMCR_WRITABLE_MASK);
                self.reset_clock_reference();
            }
            Self::DWT_CTRL_ADDR => {
                // Keep this narrow for now; CYCCNTENA is the main consumed control bit.
                self.dwt_ctrl = value & Self::CYCCNTENA;
                self.reset_clock_reference();
            }
            Self::DWT_CYCCNT_ADDR => {
                self.dwt_cyccnt = value;
                self.reset_clock_reference();
            }
            Self::DWT_CPICNT_ADDR => {
                self.dwt_cpicnt = value & 0xFF;
                self.reset_clock_reference();
            }
            Self::DWT_EXCCNT_ADDR => {
                self.dwt_exccnt = value & 0xFF;
                self.reset_clock_reference();
            }
            Self::DWT_SLEEPCNT_ADDR => {
                self.dwt_sleepcnt = value & 0xFF;
                self.reset_clock_reference();
            }
            Self::DWT_LSUCNT_ADDR => {
                self.dwt_lsucnt = value & 0xFF;
                self.reset_clock_reference();
            }
            Self::DWT_FOLDCNT_ADDR => {
                self.dwt_foldcnt = value & 0xFF;
                self.reset_clock_reference();
            }
            Self::DWT_PCSR_ADDR => {
                self.dwt_pcsr = value;
            }
            0xE000_1020..=0xE000_102F => {
                let idx = ((addr - 0xE000_1020) >> 2) as usize;
                if idx < 4 { self.dwt_comp[idx] = value; }
            }
            0xE000_1040..=0xE000_104F => {
                let idx = ((addr - 0xE000_1040) >> 2) as usize;
                if idx < 4 { self.dwt_mask[idx] = value; }
            }
            0xE000_1060..=0xE000_106F => {
                let idx = ((addr - 0xE000_1060) >> 2) as usize;
                if idx < 4 { self.dwt_function[idx] = value; }
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
