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
const USART_SR_ORE: u32 = 1 << 3;   // Overrun error
const USART_SR_NE: u32 = 1 << 2;    // Noise error
const USART_SR_FE: u32 = 1 << 1;    // Framing error
const USART_SR_PE: u32 = 1 << 0;    // Parity error
const USART_CR3_DMAT: u32 = 1 << 7;  // Transmit DMA enable
const USART_CR3_DMAR: u32 = 1 << 6;  // Receive DMA enable
const USART_CR3_SCEN: u32 = 1 << 5;  // Smartcard mode enable
const USART_CR3_HDSEL: u32 = 1 << 3; // Half-duplex selection
const USART_CR3_IREN: u32 = 1 << 1;  // IrDA mode enable
const USART_CR3_EIE: u32 = 1 << 0;   // Error interrupt enable
const USART_CR1_UE: u32 = 1 << 13;   // USART enable
const USART_CR1_TE: u32 = 1 << 3;    // Transmitter enable
const USART_CR1_RE: u32 = 1 << 2;    // Receiver enable
const USART_CR1_M: u32 = 1 << 12;    // Word length (0=8 data bits, 1=9 data bits)
const USART_CR1_PCE: u32 = 1 << 10;  // Parity control enable
const USART_CR1_PEIE: u32 = 1 << 8;  // PE interrupt enable
const USART_CR1_TXEIE: u32 = 1 << 7; // TXE interrupt enable
const USART_CR1_TCIE: u32 = 1 << 6;  // TC interrupt enable
const USART_CR1_RXNEIE: u32 = 1 << 5;// RXNE interrupt enable
const USART_CR2_STOP_MASK: u32 = 0b11 << 12;

// Default TX latency used when BRR has not been configured yet.
const TX_COMPLETION_DELAY_DEFAULT: u64 = 10;

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
    rx_dma_pending: VecDeque<u8>,
    
    // TX state machine: track when DR was written to trigger TXE/TC transitions
    tx_active_since: Option<u64>,
    sr_read_since_last_dr_read: bool,
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

    fn tx_completion_delay(&self) -> u64 {
        // BRR[15:4]=mantissa, BRR[3:0]=fraction (oversampling by 16 path).
        // Use a bounded instruction-latency approximation so BRR changes affect
        // TXE/TC timing without stalling execution at very low baud values.
        if self.brr == 0 {
            return TX_COMPLETION_DELAY_DEFAULT;
        }

        let mantissa = (self.brr >> 4) & 0x0fff;
        let fraction = self.brr & 0x000f;
        let usartdiv_x16 = (mantissa << 4) | fraction;
        // Base delay from BRR divider.
        let base_delay = (usartdiv_x16 as u64) / 2;

        // Fold configured frame length into the latency estimate so CR1/CR2
        // programming (word length, parity, stop bits) influences TXE/TC timing.
        let data_bits = if (self.cr1 & USART_CR1_M) != 0 { 9u64 } else { 8u64 };
        let parity_bits = if (self.cr1 & USART_CR1_PCE) != 0 { 1u64 } else { 0u64 };
        let stop_half_bits = match (self.cr2 & USART_CR2_STOP_MASK) >> 12 {
            0b00 => 2u64, // 1 stop bit
            0b01 => 1u64, // 0.5 stop bit
            0b10 => 4u64, // 2 stop bits
            0b11 => 3u64, // 1.5 stop bits
            _ => 2u64,
        };
        let frame_half_bits = 2u64 + (data_bits * 2) + (parity_bits * 2) + stop_half_bits;
        let raw_delay = (base_delay.saturating_mul(frame_half_bits)).saturating_div(20);

        raw_delay.clamp(2, 128)
    }

    fn tx_enabled(&self) -> bool {
        (self.cr1 & USART_CR1_UE) != 0 && (self.cr1 & USART_CR1_TE) != 0
    }

    fn rx_enabled(&self) -> bool {
        let enabled = (self.cr1 & USART_CR1_UE) != 0 && (self.cr1 & USART_CR1_RE) != 0;
        if !enabled {
            return false;
        }

        // In half-duplex mode, model single-wire directionality: when TX is active,
        // do not accept RX traffic on the same line.
        if (self.cr3 & USART_CR3_HDSEL) != 0 && (self.cr1 & USART_CR1_TE) != 0 {
            return false;
        }

        true
    }

    /// Service TX state machine: transition TXE/TC based on TX timing
    fn service_tx_state(&mut self, sys: &System) {
        if let Some(tx_since) = self.tx_active_since {
            let now = NUM_INSTRUCTIONS.load(std::sync::atomic::Ordering::Relaxed);
            if now.saturating_sub(tx_since) >= self.tx_completion_delay() {
                // TX completion delay expired: set both TXE and TC
                self.sr |= USART_SR_TXE | USART_SR_TC;
                self.tx_active_since = None;
                self.maybe_raise_irq(sys);
            }
        }
    }

    fn tx_data_mask(&self) -> u32 {
        let data_bits: u32 = if (self.cr1 & USART_CR1_M) != 0 { 9 } else { 8 };
        // With parity enabled, the top data bit is replaced by parity.
        let payload_bits = if (self.cr1 & USART_CR1_PCE) != 0 {
            data_bits.saturating_sub(1)
        } else {
            data_bits
        };
        if payload_bits >= 32 {
            u32::MAX
        } else {
            (1u32 << payload_bits) - 1
        }
    }

    fn rx_data_mask(&self) -> u32 {
        // Mirror RX payload width handling with TX: configured word length with
        // parity consuming the MSB when enabled.
        let data_bits: u32 = if (self.cr1 & USART_CR1_M) != 0 { 9 } else { 8 };
        let payload_bits = if (self.cr1 & USART_CR1_PCE) != 0 {
            data_bits.saturating_sub(1)
        } else {
            data_bits
        };

        if payload_bits >= 32 {
            u32::MAX
        } else {
            (1u32 << payload_bits) - 1
        }
    }

    fn service_rx_state(&mut self, sys: &System) {
        if !self.rx_enabled() {
            return;
        }

        // USART probes are TX observation endpoints and report 0 on reads.
        // Skip RX polling to avoid synthetic RXNE noise.
        if self.name.contains("usart-probe") {
            return;
        }

        let Some(dev) = &self.ext_device else {
            return;
        };

        let byte = dev.borrow_mut().read(sys, ());
        if byte == 0 {
            return;
        }

        if (self.cr3 & USART_CR3_DMAR) != 0 {
            // With DMAR enabled, stage a single pending byte for DMA consumption.
            // A second byte arriving before DMA drains the first is treated as overrun.
            if !self.rx_dma_pending.is_empty() {
                self.sr |= USART_SR_ORE;
                self.maybe_raise_irq(sys);
                return;
            }

            self.rx_dma_pending.push_back(byte);
            self.sr |= USART_SR_RXNE;
            self.sr &= !USART_SR_IDLE;
            self.maybe_raise_irq(sys);
            return;
        }

        if (self.sr & USART_SR_RXNE) != 0 {
            // Overrun: keep existing DR until firmware reads it, and latch ORE.
            self.sr |= USART_SR_ORE;
            self.maybe_raise_irq(sys);
            return;
        }

        self.dr = (byte as u32) & self.rx_data_mask();
        self.sr |= USART_SR_RXNE;
        self.sr &= !USART_SR_IDLE;
        self.maybe_raise_irq(sys);
    }

    fn maybe_raise_irq(&self, sys: &System) {
        if self.irq < 0 || (self.cr1 & USART_CR1_UE) == 0 {
            return;
        }

        let tx_enabled = (self.cr1 & USART_CR1_TE) != 0;
        let rx_enabled = (self.cr1 & USART_CR1_RE) != 0;

        let txe = (self.sr & USART_SR_TXE) != 0;
        let tc = (self.sr & USART_SR_TC) != 0;
        let rxne = (self.sr & USART_SR_RXNE) != 0;
        let pe = (self.sr & USART_SR_PE) != 0;
        let err = (self.sr & (USART_SR_ORE | USART_SR_NE | USART_SR_FE)) != 0;

        let txeie = (self.cr1 & USART_CR1_TXEIE) != 0;
        let tcie = (self.cr1 & USART_CR1_TCIE) != 0;
        let rxneie = (self.cr1 & USART_CR1_RXNEIE) != 0;
        let peie = (self.cr1 & USART_CR1_PEIE) != 0;
        let eie = (self.cr3 & USART_CR3_EIE) != 0;

        if (tx_enabled && txe && txeie)
            || (tx_enabled && tc && tcie)
            || (rx_enabled && rxne && rxneie)
            || (rx_enabled && pe && peie)
            || (rx_enabled && err && eie)
        {
            sys.p.nvic.borrow_mut().set_intr_pending(self.irq);
        }
    }
}

impl Peripheral for Usart {
    fn step(&mut self, sys: &System) {
        self.service_tx_state(sys);
        self.service_rx_state(sys);
    }

    fn read(&mut self, sys: &System, offset: u32) -> u32 {
        // Service TX state before reading SR
        if offset == 0x0000 {
            self.service_tx_state(sys);
        }

        match offset {
            0x0000 => {
                self.sr_read_since_last_dr_read = true;
                self.sr
            }
            0x0004 => {
                // DR register: reading clears RXNE and IDLE
                if !self.rx_enabled() {
                    self.sr_read_since_last_dr_read = false;
                    return self.dr;
                }

                // Return the latched DR value. Incoming data is sampled in service_rx_state(),
                // not during DR reads, so repeated DR accesses do not consume new bytes.
                self.service_rx_state(sys);
                self.dr &= self.rx_data_mask();
                if self.sr_read_since_last_dr_read {
                    // RM-style SR->DR sequence clears receive and line-status flags.
                    self.sr &= !(USART_SR_RXNE | USART_SR_IDLE | USART_SR_ORE | USART_SR_NE | USART_SR_FE | USART_SR_PE);
                }
                self.sr_read_since_last_dr_read = false;

                trace!("{} read={:02x}", self.name, self.dr);
                self.dr
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
                // SR write does not directly clear status flags in this model.
                // Receive/error classes are cleared by SR->DR reads.
                // TC is cleared when a new transmission starts (DR write with TX enabled).
                let _ = value;
                self.maybe_raise_irq(sys);
            }
            0x0004 => {
                // DR register write: indicates TX data write
                self.dr = value & self.tx_data_mask();

                // Ignore transmission side effects when UE/TE do not permit TX.
                if !self.tx_enabled() {
                    return;
                }

                // Clear TXE on write (DR now full), clear TC (new transmission starting)
                self.sr &= !(USART_SR_TXE | USART_SR_TC);
                // Start TX completion timer
                self.tx_active_since = Some(NUM_INSTRUCTIONS.load(std::sync::atomic::Ordering::Relaxed));

                // If DMA TX is not enabled, write directly to ext_device
                if (self.cr3 & USART_CR3_DMAT) == 0 {
                    self.ext_device.as_ref().map(|d|
                        d.borrow_mut().write(sys, (), self.dr as u8)
                    );
                }

                trace!("{} write={:02x}", self.name, self.dr as u8);
            }
            0x0008 => self.brr = value & 0x0000_ffff,
            0x000c => {
                let old_ue = (self.cr1 & USART_CR1_UE) != 0;
                let old_te = (self.cr1 & USART_CR1_TE) != 0;
                let old_re = (self.cr1 & USART_CR1_RE) != 0;
                self.cr1 = value;
                let new_ue = (self.cr1 & USART_CR1_UE) != 0;
                let new_te = (self.cr1 & USART_CR1_TE) != 0;
                let new_re = (self.cr1 & USART_CR1_RE) != 0;

                if old_ue && !new_ue {
                    self.tx_active_since = None;
                    self.sr &= !(USART_SR_RXNE | USART_SR_ORE | USART_SR_NE | USART_SR_FE | USART_SR_PE);
                    self.rx_dma_pending.clear();
                }

                if old_te && !new_te {
                    self.tx_active_since = None;
                    self.sr |= USART_SR_TXE | USART_SR_TC;
                }

                if old_re && !new_re {
                    self.sr &= !(USART_SR_RXNE | USART_SR_ORE | USART_SR_NE | USART_SR_FE | USART_SR_PE);
                    self.rx_dma_pending.clear();
                }

                self.maybe_raise_irq(sys);
            }
            0x0010 => {
                // Persist full CR2 state, but keep only defined stop-bit field in behavior.
                self.cr2 = value;
            }
            0x0014 => {
                // Persist CR3 state; DMAT/DMAR are consumed in DMA hooks.
                self.cr3 = value;

                // Basic STM32 mode interaction modeling:
                // - HDSEL, SCEN, and IREN are mutually exclusive families.
                // - IrDA/Smartcard paths are modeled as non-DMA serial paths here.
                if (self.cr3 & USART_CR3_HDSEL) != 0 {
                    self.cr3 &= !(USART_CR3_SCEN | USART_CR3_IREN);
                } else if (self.cr3 & (USART_CR3_SCEN | USART_CR3_IREN)) != 0 {
                    self.cr3 &= !USART_CR3_HDSEL;
                    self.cr3 &= !(USART_CR3_DMAR | USART_CR3_DMAT);
                }

                if !self.rx_enabled() {
                    self.sr &= !(USART_SR_RXNE | USART_SR_ORE | USART_SR_NE | USART_SR_FE | USART_SR_PE);
                    self.rx_dma_pending.clear();
                }
                self.maybe_raise_irq(sys);
            }
            0x0018 => self.gtpr = value,
            _ => {}
        }
    }

    fn read_dma(&mut self, sys: &System, offset: u32, size: usize) -> VecDeque<u8> {
        // DMA reads should use the batched ext_device path to avoid per-byte virtual-call overhead.
        let mut result = VecDeque::with_capacity(size);

        if offset == 0x0004 && (self.cr3 & USART_CR3_DMAR) != 0 && self.rx_enabled() {
            while result.len() < size {
                let Some(byte) = self.rx_dma_pending.pop_front() else {
                    break;
                };
                result.push_back(byte);
            }

            if result.len() < size {
                if let Some(dev) = &self.ext_device {
                    for byte in dev.borrow_mut().read_batch(sys, (), size - result.len()) {
                        result.push_back(byte);
                    }
                } else {
                    while result.len() < size {
                        result.push_back(self.dr as u8);
                    }
                }
            }

            if self.rx_dma_pending.is_empty() {
                self.sr &= !USART_SR_RXNE;
            }
            self.sr_read_since_last_dr_read = false;
        }

        // For non-DR offsets or when DMAR is not enabled, return empty rather than
        // delegating to the trait default, which resolves back to this override.
        result
    }

    fn write_dma(&mut self, sys: &System, offset: u32, mut value: VecDeque<u8>) {
        if offset == 0x0004 && (self.cr3 & USART_CR3_DMAT) != 0 && self.tx_enabled() {
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
            self.maybe_raise_irq(sys);
            trace!("{} dma_write {} bytes", self.name, value.len());
        } else {
            super::Peripheral::write_dma(self, sys, offset, value);
        }
    }
}
