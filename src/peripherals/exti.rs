// SPDX-License-Identifier: GPL-3.0-or-later

// STM32 name: EXTI (External interrupt/event controller).
// STM32F427 base: 0x40013C00.
// Key registers: IMR, EMR, RTSR, FTSR, SWIER, PR.
// Key function: routes GPIO pin transitions and software events to NVIC interrupt lines.
// Critical for this emulator: IMU DRDY and other GPIO-driven interrupts fan out through EXTI.
// IRQ fanout: EXTI0->6, EXTI1->7, EXTI2->8, EXTI3->9, EXTI4->10, EXTI5-9->23, EXTI10-15->40.
// Still incomplete: wakeup event routing (EMR), full EXTI16-22 lines (PVD, TAMPER, RTC, etc.).
// Datasheet/reference anchor: STM32F4 RM EXTI chapter.

use crate::system::System;
use super::Peripheral;

#[derive(Default)]
pub struct Exti {
    imr: u32,   // interrupt mask: 1=unmasked (enabled)
    emr: u32,   // event mask
    rtsr: u32,  // rising-edge trigger select
    ftsr: u32,  // falling-edge trigger select
    swier: u32, // software interrupt event register
    pr: u32,    // pending register (write 1 to clear)
}

impl Exti {
    pub fn new(name: &str) -> Option<Box<dyn Peripheral>> {
        if name == "EXTI" {
            Some(Box::new(Self::default()))
        } else {
            None
        }
    }

    /// Map EXTI line number to NVIC IRQ number (STM32F427 mapping).
    fn irq_for_line(line: u8) -> Option<i32> {
        match line {
            0  => Some(6),
            1  => Some(7),
            2  => Some(8),
            3  => Some(9),
            4  => Some(10),
            5..=9  => Some(23),   // EXTI9_5
            10..=15 => Some(40),  // EXTI15_10
            // Lines 16-22 route to: PVD(1), RTC_ALARM(41), USB_OTG_FS_WAKEUP(42),
            // ETH_WAKEUP(62), USB_OTG_HS_WAKEUP(76), RTC_TAMPER(2), RTC_WUT(3).
            // These are not wired through the standard GPIO EXTI path, so leave unimplemented.
            _ => None,
        }
    }

    fn trigger_line(&mut self, sys: &System, line: u8) {
        let bit = 1u32 << line;
        self.pr |= bit;
        if (self.imr & bit) != 0 {
            if let Some(irq) = Self::irq_for_line(line) {
                debug!("EXTI line {} pending -> IRQ {}", line, irq);
                sys.p.nvic.borrow_mut().set_intr_pending(irq);
            }
        }
    }
}

impl Peripheral for Exti {
    fn read(&mut self, _sys: &System, offset: u32) -> u32 {
        match offset {
            0x00 => self.imr,
            0x04 => self.emr,
            0x08 => self.rtsr,
            0x0c => self.ftsr,
            0x10 => self.swier,
            0x14 => self.pr,
            _ => 0,
        }
    }

    fn write(&mut self, sys: &System, offset: u32, value: u32) {
        match offset {
            0x00 => {
                self.imr = value;
            }
            0x04 => self.emr = value,
            0x08 => self.rtsr = value,
            0x0c => self.ftsr = value,
            0x10 => {
                // Writing a bit triggers the corresponding EXTI line (software interrupt).
                // Only trigger newly-set bits (rising edge on SWIER bit).
                let newly_set = value & !self.swier;
                self.swier = value;
                for line in 0u8..=22 {
                    if (newly_set >> line) & 1 != 0 {
                        self.trigger_line(sys, line);
                    }
                }
            }
            // Write 1 to clear the corresponding pending bit.
            0x14 => self.pr &= !value,
            _ => {}
        }
    }
}

// ---------------------------------------------------------------------------
// GPIO → EXTI interface
// ---------------------------------------------------------------------------

/// Called by GPIO emulation when a pin's input level transitions.
/// `port` is 0=A, 1=B, ... 10=K.  `pin` is 0..15.
/// Only fires EXTI if the EXTI line is configured to that port's SYSCFG channel —
/// in this simplified model we route all GPIO write callbacks through without
/// checking SYSCFG EXTICR, since CubeBlack wires each significant pin to a unique
/// port anyway and getting the exact SYSCFG routing wrong only matters for
/// multi-port-on-same-line cases.
pub fn gpio_pin_transition(sys: &System, _port: u8, pin: u8, prev: bool, curr: bool) {
    let rising  = !prev && curr;
    let falling =  prev && !curr;
    if !rising && !falling {
        return;
    }

    // Read current EXTI trigger configuration via the peripheral read path.
    let exti_base = 0x4001_3C00u32;
    let rtsr = sys.p.read(sys, exti_base + 0x08, 4);
    let ftsr = sys.p.read(sys, exti_base + 0x0C, 4);
    let bit = 1u32 << pin;
    let trigger = (rising && (rtsr & bit) != 0) || (falling && (ftsr & bit) != 0);
    if trigger {
        // Write to SWIER to trigger the line (goes through Exti::write which calls trigger_line).
        sys.p.write(sys, exti_base + 0x10, 4, bit);
    }
}
