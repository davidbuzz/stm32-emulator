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

use super::{Peripheral, meta::DeviceMeta};

#[derive(Default)]
pub struct Tim {
    name: String,
    cr1: u32,
    cr2: u32,
    smcr: u32,
    dier: u32,
    sr: u32,
    ccmr1: u32,
    ccmr2: u32,
    ccer: u32,
    cnt: u32,
    psc: u32,
    arr: u32,
    ccr1: u32,
    ccr2: u32,
    ccr3: u32,
    ccr4: u32,
    /// ARPE (CR1 bit 7): auto-reload preload enable. When set, ARR value preloads on next update event.
    arpe: bool,
    /// CR1[5:4]: CMS counting mode. 00=upcounting, 01/10=center-aligned, 11=reserved.
    cms: u8,
    /// CR1 bit 4: DIR direction (0=up, 1=down). Only meaningful when CMS=00 (edge-aligned).
    direction_up: bool,
    /// RCR (repetition counter) for advanced timers TIM1/TIM8: update fires only after RCR+1 overflows.
    rcr: u32,
    rcr_count: u32,
    /// BDTR (break and dead-time register) for advanced timers: MOE, BKE, OSSR, OSSI, DTG.
    bdtr: u32,
    /// Break condition active (BIF in SR). Set when MOE transitions from 1→0 or BRK input detected.
    break_condition: bool,
    update_irq: Option<i32>,
    cc_irq: Option<i32>,
    /// SMCR slave mode configuration: SMS bits 2:0 extracted
    smcr_sms: u8,
    /// SMCR trigger selection: TS bits 6:4 extracted  
    smcr_ts: u8,
    center_aligned_up: bool,
    arr_shadow: u32,
    psc_shadow: u32,
    arr_pending: bool,
    psc_pending: bool,
    last_clk: u64,
    psc_accum: u64,
}

impl Tim {
    pub fn new(name: &str, meta: &DeviceMeta) -> Option<Box<dyn Peripheral>> {
        if name.starts_with("TIM") {
            let (update_irq, cc_irq) = Self::resolve_irqs(name, meta);
            Some(Box::new(Self {
                name: name.to_string(),
                arr: u32::MAX,
                arr_shadow: u32::MAX,
                center_aligned_up: true,
                update_irq,
                cc_irq,
                ..Self::default()
            }))
        } else {
            None
        }
    }

    fn resolve_irqs(name: &str, meta: &DeviceMeta) -> (Option<i32>, Option<i32>) {
        let update = match name {
            "TIM1" => meta.irq_of("TIM1_UP_TIM10").or(Some(25)),
            "TIM2" => meta.irq_of("TIM2").or(Some(28)),
            "TIM3" => meta.irq_of("TIM3").or(Some(29)),
            "TIM4" => meta.irq_of("TIM4").or(Some(30)),
            "TIM5" => meta.irq_of("TIM5").or(Some(50)),
            "TIM6" => meta.irq_of("TIM6_DAC").or(Some(54)),
            "TIM7" => meta.irq_of("TIM7").or(Some(55)),
            "TIM8" => meta.irq_of("TIM8_UP_TIM13").or(Some(44)),
            "TIM9" => meta.irq_of("TIM1_BRK_TIM9").or(Some(24)),
            "TIM10" => meta.irq_of("TIM1_UP_TIM10").or(Some(25)),
            "TIM11" => meta.irq_of("TIM1_TRG_COM_TIM11").or(Some(26)),
            "TIM12" => meta.irq_of("TIM8_BRK_TIM12").or(Some(43)),
            "TIM13" => meta.irq_of("TIM8_UP_TIM13").or(Some(44)),
            "TIM14" => meta.irq_of("TIM8_TRG_COM_TIM14").or(Some(45)),
            _ => None,
        };

        let cc = match name {
            "TIM1" => meta.irq_of("TIM1_CC").or(Some(27)),
            "TIM8" => meta.irq_of("TIM8_CC").or(Some(46)),
            _ => None,
        };

        (update, cc)
    }

    fn irq_number(&self) -> Option<i32> {
        self.update_irq
    }

    fn cc_irq_number(&self) -> Option<i32> {
        self.cc_irq
    }

    fn counter_mask(&self) -> u32 {
        match self.name.as_str() {
            "TIM2" | "TIM5" => u32::MAX,
            _ => 0x0000_ffff,
        }
    }

    fn apply_width(&self, value: u32) -> u32 {
        value & self.counter_mask()
    }

    fn is_advanced_timer(&self) -> bool {
        self.name == "TIM1" || self.name == "TIM8"
    }

    fn udis(&self) -> bool {
        (self.cr1 & (1 << 1)) != 0
    }

    fn urs(&self) -> bool {
        (self.cr1 & (1 << 2)) != 0
    }

    fn update_register_preloads(&mut self) {
        if self.arr_pending {
            self.arr = self.apply_width(self.arr_shadow);
            self.arr_pending = false;
        }
        if self.psc_pending {
            self.psc = self.apply_width(self.psc_shadow);
            self.psc_pending = false;
        }
    }

    fn trigger_update_event(&mut self, sys: &System, from_counter_overflow: bool) {
        if !from_counter_overflow && self.urs() {
            return;
        }
        if self.udis() {
            return;
        }

        if self.is_advanced_timer() {
            self.rcr_count = self.rcr_count.saturating_add(1);
            if self.rcr_count < self.rcr.saturating_add(1) {
                return;
            }
            self.rcr_count = 0;
        }

        self.update_register_preloads();
        self.sr |= 1;
        if (self.dier & 1) != 0 {
            if let Some(irq) = self.irq_number() {
                sys.p.nvic.borrow_mut().set_intr_pending(irq);
            }
        }
        if (self.dier & (1 << 8)) != 0 {
            self.trigger_update_dma_request(sys);
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

        let width_mask = self.counter_mask();
        let arr = self.arr.max(1) & width_mask;
        let old_cnt = self.cnt;
        
        // Determine counting direction based on DIR bit (CR1[4]) and CMS mode (CR1[6:5])
        // For edge-aligned mode (CMS=00): up if DIR=0, down if DIR=1
        // For center-aligned modes: counter alternates direction
        let is_downcounting = if self.cms == 0 {
            !self.direction_up
        } else {
            !self.center_aligned_up
        };
        
        if is_downcounting {
            // Down-counting mode: decrement counter
            self.cnt = self.cnt.saturating_sub(ticks) & width_mask;
        } else if self.cms == 0 {
            // Edge-aligned up-counting mode (default)
            self.cnt = self.cnt.wrapping_add(ticks) & width_mask;
        } else if self.cms == 1 || self.cms == 2 {
            // Center-aligned mode: emulate a coarse up/down reversal around ARR/0.
            if self.center_aligned_up {
                self.cnt = self.cnt.wrapping_add(ticks) & width_mask;
                if self.cnt >= arr {
                    self.cnt = arr;
                    self.center_aligned_up = false;
                }
            } else {
                self.cnt = self.cnt.saturating_sub(ticks) & width_mask;
                if self.cnt == 0 {
                    self.center_aligned_up = true;
                }
            }
        } else {
            // CMS=3 is reserved
            return;
        }

        // Fire compare once when the counter crosses CCR1-CCR4; flags remain set until cleared.
        let check_cc = |old: u32, new: u32, ccr: u32| -> bool {
            if is_downcounting {
                if old >= new { old > ccr && ccr >= new } else { old > ccr || ccr >= new }
            } else {
                if old <= new { old < ccr && ccr <= new } else { old < ccr || ccr <= new }
            }
        };

        // CCR1 – DIER bit 1, SR bit 1, CCER CC1E bit 0
        if (self.ccer & (1 << 0)) != 0
            && (self.dier & (1 << 1)) != 0
            && (self.sr & (1 << 1)) == 0
            && check_cc(old_cnt, self.cnt, self.ccr1)
        {
            self.sr |= 1 << 1;
            let irq = self.cc_irq_number().or_else(|| self.irq_number());
            if let Some(irq) = irq {
                debug!("{} CC1 compare fired cnt=0x{:08x} ccr1=0x{:08x} -> IRQ {}", self.name, self.cnt, self.ccr1, irq);
                sys.p.nvic.borrow_mut().set_intr_pending(irq);
            }
            // CC1 DMA request (DIER bit 9)
            if (self.dier & (1 << 9)) != 0 {
                self.trigger_cc_dma_request(sys, 1);
            }
        } else if (self.ccer & (1 << 0)) != 0 && (self.sr & (1 << 1)) != 0 && check_cc(old_cnt, self.cnt, self.ccr1) {
            // CC1 overflow: flag already set, but counter crossed threshold again
            self.sr |= 1 << 9; // set CC1OF
        }
        // CCR2 – DIER bit 2, SR bit 2, CCER CC2E bit 4
        if (self.ccer & (1 << 4)) != 0
            && (self.dier & (1 << 2)) != 0
            && (self.sr & (1 << 2)) == 0
            && check_cc(old_cnt, self.cnt, self.ccr2)
        {
            self.sr |= 1 << 2;
            let irq = self.cc_irq_number().or_else(|| self.irq_number());
            if let Some(irq) = irq { sys.p.nvic.borrow_mut().set_intr_pending(irq); }
            // CC2 DMA request (DIER bit 10)
            if (self.dier & (1 << 10)) != 0 {
                self.trigger_cc_dma_request(sys, 2);
            }
        } else if (self.ccer & (1 << 4)) != 0 && (self.sr & (1 << 2)) != 0 && check_cc(old_cnt, self.cnt, self.ccr2) {
            // CC2 overflow: flag already set, but counter crossed threshold again
            self.sr |= 1 << 10; // set CC2OF
        }
        // CCR3 – DIER bit 3, SR bit 3, CCER CC3E bit 8
        if (self.ccer & (1 << 8)) != 0
            && (self.dier & (1 << 3)) != 0
            && (self.sr & (1 << 3)) == 0
            && check_cc(old_cnt, self.cnt, self.ccr3)
        {
            self.sr |= 1 << 3;
            let irq = self.cc_irq_number().or_else(|| self.irq_number());
            if let Some(irq) = irq { sys.p.nvic.borrow_mut().set_intr_pending(irq); }
            // CC3 DMA request (DIER bit 11)
            if (self.dier & (1 << 11)) != 0 {
                self.trigger_cc_dma_request(sys, 3);
            }
        } else if (self.ccer & (1 << 8)) != 0 && (self.sr & (1 << 3)) != 0 && check_cc(old_cnt, self.cnt, self.ccr3) {
            // CC3 overflow: flag already set, but counter crossed threshold again
            self.sr |= 1 << 11; // set CC3OF
        }
        // CCR4 – DIER bit 4, SR bit 4, CCER CC4E bit 12
        if (self.ccer & (1 << 12)) != 0
            && (self.dier & (1 << 4)) != 0
            && (self.sr & (1 << 4)) == 0
            && check_cc(old_cnt, self.cnt, self.ccr4)
        {
            self.sr |= 1 << 4;
            let irq = self.cc_irq_number().or_else(|| self.irq_number());
            if let Some(irq) = irq { sys.p.nvic.borrow_mut().set_intr_pending(irq); }
            // CC4 DMA request (DIER bit 12)
            if (self.dier & (1 << 12)) != 0 {
                self.trigger_cc_dma_request(sys, 4);
            }
        } else if (self.ccer & (1 << 12)) != 0 && (self.sr & (1 << 4)) != 0 && check_cc(old_cnt, self.cnt, self.ccr4) {
            // CC4 overflow: flag already set, but counter crossed threshold again
            self.sr |= 1 << 12; // set CC4OF
        }

        // Check for overflow/underflow condition
        let overflow = if self.cms == 0 && is_downcounting {
            old_cnt > 0 && self.cnt == 0  // Counter underflowed to 0
        } else if self.cms == 0 {
            self.cnt >= arr  // Counter overflowed past ARR
        } else {
            // In center-aligned mode, update happens at both limits.
            self.cnt == 0 || self.cnt == arr
        };

        if overflow {
            if self.cms == 0 {
                if is_downcounting {
                    self.cnt = arr;
                } else {
                    self.cnt = 0;
                }
            } else {
                // Center-aligned update point keeps counter at edge and flips direction.
                if self.cnt == arr {
                    self.center_aligned_up = false;
                } else if self.cnt == 0 {
                    self.center_aligned_up = true;
                }
            }

            self.trigger_update_event(sys, true);
        }
    }

    fn trigger_cc_dma_request(&self, sys: &System, cc: u8) {
        // Determine which address and channel to trigger DMA for
        // This is simplified: actual firmware would have configured a specific stream
        // For now, we log that a CC DMA request occurred
        debug!("{} CC{} DMA request triggered (DIER bit {} set)", self.name, cc, 8 + cc);
        // TODO: enumerate active DMA streams for this timer and fire any that are waiting
    }

    fn trigger_update_dma_request(&self, sys: &System) {
        debug!("{} Update DMA request triggered (DIER bit 8 set)", self.name);
        // TODO: enumerate active DMA streams for this timer update events
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
            0x0018 => self.ccmr1,
            0x001c => self.ccmr2,
            0x0020 => self.ccer,
            0x0024 => self.apply_width(self.cnt),
            0x0028 => self.apply_width(self.psc),
            0x002c => self.apply_width(self.arr),
            0x0030 => {
                // RCR (1xH): Repetition Counter for TIM1 TIM8 only
                // Firmware reads to check remaining repetitions before update
                if self.name == "TIM1" || self.name == "TIM8" {
                    self.rcr as u32
                } else {
                    0
                }
            }
            0x0034 => self.ccr1,
            0x0038 => self.ccr2,
            0x003c => self.ccr3,
            0x0040 => self.ccr4,
            0x0044 => {
                // BDTR (44H): Break and Dead-Time Register for TIM1 TIM8
                if self.name == "TIM1" || self.name == "TIM8" {
                    self.bdtr
                } else {
                    0
                }
            }
            _ => 0,
        }
    }

    fn write(&mut self, sys: &System, offset: u32, value: u32) {
        self.tick(sys);

        match offset {
            0x0000 => {
                debug!("{} write CR1=0x{:08x}", self.name, value);
                self.cr1 = value;
                // Extract control bits from CR1
                self.arpe = (value >> 7) & 1 != 0;  // CR1 bit 7: ARPE
                self.cms = ((value >> 5) & 0x3) as u8;  // CR1[6:5]: CMS
                self.direction_up = (value >> 4) & 1 == 0;  // CR1 bit 4: DIR (0=up, 1=down)
                if self.cms != 0 {
                    self.center_aligned_up = true;
                }
            }
            0x0004 => self.cr2 = value,
            0x0008 => {
                self.smcr = value;
                self.smcr_sms = (value & 0x7) as u8;  // SMS bits 2:0
                self.smcr_ts = ((value >> 4) & 0x7) as u8;  // TS bits 6:4
                if self.smcr_sms != 0 {
                    debug!("{} write SMCR=0x{:08x} (SMS={} TS={})", self.name, value, self.smcr_sms, self.smcr_ts);
                }
            }
            0x000c => {
                debug!("{} write DIER=0x{:08x}", self.name, value);
                self.dier = value;
            }
            // Firmware often clears status flags by writing 0 (write-0-to-clear).
            0x0010 => self.sr &= value,
            0x0014 => { // EGR
                // EGR (Event Generation Register): writing bit 0 forces an update event.
                if value & 1 != 0 {
                    self.trigger_update_event(sys, false);
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
                // Trigger event generation: bit 6 sets TIF and fires trigger IRQ (if enabled).
                if (value >> 6) & 1 != 0 {
                    self.sr |= 1 << 6; // set TIF
                    if (self.dier & (1 << 6)) != 0 {
                        if let Some(irq) = self.irq_number() {
                            debug!("{} EGR TG -> trigger event -> IRQ {}", self.name, irq);
                            sys.p.nvic.borrow_mut().set_intr_pending(irq);
                        }
                    }
                }
                // Break event generation: bit 7 sets BIF for TIM1/TIM8.
                if (value >> 7) & 1 != 0 && (self.name == "TIM1" || self.name == "TIM8") {
                    self.sr |= 1 << 7; // set BIF
                    self.break_condition = true;
                    if (self.dier & (1 << 7)) != 0 {
                        if let Some(irq) = self.irq_number() {
                            debug!("{} EGR BG -> break event -> IRQ {}", self.name, irq);
                            sys.p.nvic.borrow_mut().set_intr_pending(irq);
                        }
                    }
                }
            }
            0x0018 => self.ccmr1 = value,
            0x001c => self.ccmr2 = value,
            0x0020 => self.ccer = value,
            0x0024 => {
                debug!("{} write CNT=0x{:08x}", self.name, value);
                self.cnt = self.apply_width(value);
            }
            0x0028 => {
                debug!("{} write PSC=0x{:08x}", self.name, value);
                self.psc_shadow = self.apply_width(value);
                self.psc_pending = true;
            }
            0x002c => {
                debug!("{} write ARR=0x{:08x}", self.name, value);
                let arr = self.apply_width(value);
                self.arr_shadow = arr;
                if self.arpe {
                    self.arr_pending = true;
                } else {
                    self.arr = arr;
                    self.arr_pending = false;
                }
            }
            0x0030 => {
                // RCR (1xH): Repetition Counter for TIM1 TIM8
                if self.name == "TIM1" || self.name == "TIM8" {
                    self.rcr = value & 0xff;
                    self.rcr_count = 0;
                    debug!("{} write RCR=0x{:08x}", self.name, value);
                }
            }
            0x0034 => {
                debug!("{} write CCR1=0x{:08x}", self.name, value);
                self.ccr1 = self.apply_width(value);
            }
            0x0038 => {
                debug!("{} write CCR2=0x{:08x}", self.name, value);
                self.ccr2 = self.apply_width(value);
            }
            0x003c => {
                debug!("{} write CCR3=0x{:08x}", self.name, value);
                self.ccr3 = self.apply_width(value);
            }
            0x0040 => {
                debug!("{} write CCR4=0x{:08x}", self.name, value);
                self.ccr4 = self.apply_width(value);
            }
            0x0044 => {
                // BDTR (44H): Break and Dead-Time Register for TIM1 TIM8
                if self.name == "TIM1" || self.name == "TIM8" {
                    self.bdtr = value;
                    debug!("{} write BDTR=0x{:08x} (MOE={} BKE={})",
                        self.name, value, 
                        (value >> 15) & 1, (value >> 12) & 1);
                }
            }
            _ => {}
        }
    }
}
