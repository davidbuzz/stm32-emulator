// SPDX-License-Identifier: GPL-3.0-or-later

// STM32 name: USART1 / USART2 / USART3 / USART6 and UART4 / UART5 / UART7 / UART8.
// STM32F427 bases: USART1=0x40011000, USART2=0x40004400, USART3=0x40004800, UART4=0x40004C00, UART5=0x40005000, USART6=0x40011400, UART7=0x40007800, UART8=0x40007C00.
// Key registers: SR, DR, BRR, CR1, CR2, CR3, GTPR.
// Key function: asynchronous serial links for telemetry, console, crash output, and board I/O.
// Critical for this emulator: some boards use USART probes directly, while CubeBlack prefers USB CDC.
// Current model persists core register state and forwards DR bytes to emulator external serial devices.
// Still incomplete: realistic flag transitions, interrupt delivery, baud effects, and DMA-driven traffic.
// Datasheet/reference anchor: STM32F4 RM USART/UART chapter.

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::VecDeque;

use crate::ext_devices::{ExtDevices, ExtDevice};
use super::Peripheral;

use crate::system::System;
use super::meta::DeviceMeta;
use crate::emulator::NUM_INSTRUCTIONS;

const USART_SR_RXNE: u32 = 1 << 5;  // Receive data not empty
const USART_SR_TC: u32 = 1 << 6;    // Transmission complete
const USART_SR_TXE: u32 = 1 << 7;   // Transmit data register empty
const USART_SR_IDLE: u32 = 1 << 4;  // Idle line detected
const USART_CR3_DMAT: u32 = 1 << 6;  // Transmit DMA enable
const USART_CR3_DMAR: u32 = 1 << 5;  // Receive DMA enable
const USART_CR1_UE: u32 = 1 << 13;   // USART enable
const USART_CR1_TE: u32 = 1 << 3;    // Transmitter enable
const USART_CR1_RE: u32 = 1 << 2;    // Receiver enable
const USART_CR1_TXEIE: u32 = 1 << 7; // TXE interrupt enable
const USART_CR1_TCIE: u32 = 1 << 6;  // TC interrupt enable
const USART_CR1_RXNEIE: u32 = 1 << 5;// RXNE interrupt enable

// TX state transitions: TXE clears on write to DR, TC clears on new DR write, then both set after "transmission"
const TX_COMPLETION_DELAY: u64 = 10;

#[derive(Default)]
pub struct Usart {
    pub name: String,
    pub ext_device: Option<Rc<RefCell<dyn ExtDevice<(), u8>>>>,
    sr: u32,
    dr: u32,
    brr: u32,
    cr1: u32,
    cr2: u32,
    cr3: u32,
    gtpr: u32,
    
    // TX state machine: track when DR was written to trigger TXE/TC transitions
    tx_active_since: Option<u64>,
    irq: i32,
}

impl Usart {
    pub fn new(name: &str, ext_devices: &ExtDevices, meta: &DeviceMeta) -> Option<Box<dyn Peripheral>> {
        if name.starts_with("USART") || name.starts_with("UART") {
            let ext_device = ext_devices.find_serial_device(&name);
            let name = ext_device.as_ref()
                .map(|d| d.borrow_mut().connect_peripheral(name))
                .unwrap_or_else(|| name.to_string());
            let irq = meta.irq_of(&name).unwrap_or_else(|| match name.as_str() {
                "USART1" => 37, "USART2" => 38, "USART3" => 39,
                "UART4" => 52, "UART5" => 53, "USART6" => 71,
                "UART7" => 82, "UART8" => 83, _ => -1,
            });
            Some(Box::new(Self {
                name,
                ext_device,
                // Initial state: TXE(7) and TC(6) set (transmitter ready), IDLE(4) set, RXNE(5) cleared
                sr: USART_SR_TXE | USART_SR_TC | USART_SR_IDLE,
                tx_active_since: None,
                irq,
                ..Default::default()
            }))
        } else {
            None
        }
    }

    /// Service TX state machine: transition TXE/TC based on TX timing
    fn service_tx_state(&mut self, sys: &System) {
        if let Some(tx_since) = self.tx_active_since {
            let now = NUM_INSTRUCTIONS.load(std::sync::atomic::Ordering::Relaxed);
            if now.saturating_sub(tx_since) >= TX_COMPLETION_DELAY {
                // TX completion delay expired: set both TXE and TC
                self.sr |= USART_SR_TXE | USART_SR_TC;
                self.tx_active_since = None;
                // Signal interrupts if enabled
                if self.irq >= 0 && (self.cr1 & USART_CR1_UE) != 0 {
                    let txeie = (self.cr1 & USART_CR1_TXEIE) != 0;
                    let tcie  = (self.cr1 & USART_CR1_TCIE) != 0;
                    if txeie || tcie {
                        sys.p.nvic.borrow_mut().set_intr_pending(self.irq);
                    }
                }
            }
        }
    }
}

impl Peripheral for Usart {
    fn read(&mut self, sys: &System, offset: u32) -> u32 {
        // Service TX state before reading SR
        if offset == 0x0000 {
            self.service_tx_state(sys);
        }

        match offset {
            0x0000 => self.sr,
            0x0004 => {
                // DR register: reading clears RXNE and IDLE
                let v = self.ext_device.as_ref()
                    .map(|d| d.borrow_mut().read(sys, ()))
                    .unwrap_or(self.dr as u8) as u32;

                self.dr = v;
                self.sr &= !(USART_SR_RXNE | USART_SR_IDLE); // clear RXNE and IDLE after read

                trace!("{} read={:02x}", self.name, v);
                v
            }
            0x0008 => self.brr,
            0x000c => self.cr1,
            0x0010 => self.cr2,
            0x0014 => self.cr3,
            0x0018 => self.gtpr,
            _ => 0
        }
    }

    fn write(&mut self, sys: &System, offset: u32, value: u32) {
        // Service TX state before SR write
        if offset == 0x0000 {
            self.service_tx_state(sys);
        }

        match offset {
            0x0000 => {
                // SR write: per RM, TC can be cleared by writing 0 to bit 6.
                // TXE is HW-only (set by hardware after DR is shifted out), cannot be forced.
                // RXNE is cleared by reading DR; firmware should not write it.
                // Only allow TC clear via explicit 0-write to that bit.
                if (value & USART_SR_TC) == 0 {
                    self.sr &= !USART_SR_TC;
                }
                // Allow firmware to clear RXNE via SR write (some ChibiOS patterns do this)
                if (value & USART_SR_RXNE) == 0 {
                    self.sr &= !USART_SR_RXNE;
                }
            }
            0x0004 => {
                // DR register write: indicates TX data write
                self.dr = value & 0xFF;

                // Only transmit if USART enabled (UE) and transmitter enabled (TE)
                let tx_enabled = (self.cr1 & USART_CR1_UE) != 0 && (self.cr1 & USART_CR1_TE) != 0;

                // Clear TXE on write (DR now full), clear TC (new transmission starting)
                self.sr &= !(USART_SR_TXE | USART_SR_TC);
                // Start TX completion timer
                self.tx_active_since = Some(NUM_INSTRUCTIONS.load(std::sync::atomic::Ordering::Relaxed));

                // If DMA TX is not enabled, write directly to ext_device
                if tx_enabled && (self.cr3 & USART_CR3_DMAT) == 0 {
                    self.ext_device.as_ref().map(|d|
                        d.borrow_mut().write(sys, (), value as u8)
                    );
                }

                trace!("{} write={:02x}", self.name, value as u8);
            }
            0x0008 => self.brr = value,
            0x000c => self.cr1 = value,
            0x0010 => self.cr2 = value,
            0x0014 => self.cr3 = value,
            0x0018 => self.gtpr = value,
            _ => {}
        }
    }

    fn read_dma(&mut self, sys: &System, offset: u32, size: usize) -> VecDeque<u8> {
        // DMA reads should use the batched ext_device path to avoid per-byte virtual-call overhead.
        let mut result = VecDeque::with_capacity(size);

        if offset == 0x0004 && (self.cr3 & USART_CR3_DMAR) != 0 {
            if let Some(dev) = &self.ext_device {
                for byte in dev.borrow_mut().read_batch(sys, (), size) {
                    result.push_back(byte);
                }
            } else {
                for _ in 0..size {
                    result.push_back(self.dr as u8);
                }
            }

            self.sr &= !(1 << 5);
        } else {
            return super::Peripheral::read_dma(self, sys, offset, size);
        }

        result
    }

    fn write_dma(&mut self, sys: &System, offset: u32, mut value: VecDeque<u8>) {
        if offset == 0x0004 && (self.cr3 & USART_CR3_DMAT) != 0 {
            if let Some(dev) = &self.ext_device {
                let bytes = value.make_contiguous();
                if let Some(last) = bytes.last().copied() {
                    self.dr = last as u32;
                }
                dev.borrow_mut().write_batch(sys, (), bytes);
            } else if let Some(last) = value.back().copied() {
                self.dr = last as u32;
            }

            self.sr |= USART_SR_TXE | USART_SR_TC;
            trace!("{} dma_write {} bytes", self.name, value.len());
        } else {
            super::Peripheral::write_dma(self, sys, offset, value);
        }
    }
}
