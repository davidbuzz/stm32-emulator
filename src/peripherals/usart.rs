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
const USART_SR_LBD: u32 = 1 << 8;   // LIN break detection flag
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
const USART_CR1_OVER8: u32 = 1 << 15;// Oversampling mode (0=16, 1=8)
const USART_CR1_TE: u32 = 1 << 3;    // Transmitter enable
const USART_CR1_RE: u32 = 1 << 2;    // Receiver enable
const USART_CR1_M: u32 = 1 << 12;    // Word length (0=8 data bits, 1=9 data bits)
const USART_CR1_PCE: u32 = 1 << 10;  // Parity control enable
const USART_CR1_PS: u32 = 1 << 9;    // Parity selection (0=even, 1=odd)
const USART_CR1_PEIE: u32 = 1 << 8;  // PE interrupt enable
const USART_CR1_TXEIE: u32 = 1 << 7; // TXE interrupt enable
const USART_CR1_TCIE: u32 = 1 << 6;  // TC interrupt enable
const USART_CR1_RXNEIE: u32 = 1 << 5;// RXNE interrupt enable
const USART_CR2_STOP_MASK: u32 = 0b11 << 12;
const USART_CR2_LINEN: u32 = 1 << 14;
const USART_CR2_LBDIE: u32 = 1 << 6;
const USART_CR2_CLKEN: u32 = 1 << 11;

// Default TX latency used when BRR has not been configured yet.
const TX_COMPLETION_DELAY_DEFAULT: u64 = 10;
const RCC_BASE: u64 = 0x4002_3800;
const RCC_CFGR_OFFSET: u64 = 0x08;

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
    rx_staged_byte: Option<u8>,
    rx_active_since: Option<u64>,
    
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

    fn effective_stop_half_bits(&self) -> u64 {
        if (self.cr3 & USART_CR3_SCEN) != 0 {
            // Smartcard mode uses 1.5 stop bits regardless of programmed STOP field.
            return 3;
        }

        match (self.cr2 & USART_CR2_STOP_MASK) >> 12 {
            0b00 => 2u64, // 1 stop bit
            0b01 => 1u64, // 0.5 stop bit
            0b10 => 4u64, // 2 stop bits
            0b11 => 3u64, // 1.5 stop bits
            _ => 2u64,
        }
    }

    fn normalize_mode_registers(&mut self) {
        // LIN is asynchronous-only in this model: no synchronous clock output and 1 stop bit.
        if (self.cr2 & USART_CR2_LINEN) != 0 {
            self.cr2 &= !USART_CR2_CLKEN;
            self.cr2 &= !USART_CR2_STOP_MASK;
        }

        // HDSEL conflicts with Smartcard and IrDA families.
        if (self.cr3 & USART_CR3_HDSEL) != 0 {
            self.cr3 &= !(USART_CR3_SCEN | USART_CR3_IREN);
        }

        // Smartcard/IrDA are non-DMA serial paths in this emulator.
        if (self.cr3 & (USART_CR3_SCEN | USART_CR3_IREN)) != 0 {
            self.cr3 &= !USART_CR3_HDSEL;
            self.cr3 &= !(USART_CR3_DMAR | USART_CR3_DMAT);
            self.cr2 &= !USART_CR2_LINEN;
        }

        if (self.cr3 & USART_CR3_SCEN) != 0 {
            // Smartcard mode enforces synchronous clock and 1.5 stop bits.
            self.cr2 |= USART_CR2_CLKEN;
            self.cr2 = (self.cr2 & !USART_CR2_STOP_MASK) | (0b11 << 12);
        }

        if (self.cr3 & USART_CR3_IREN) != 0 {
            // IrDA mode is asynchronous and does not use CLKEN.
            self.cr2 &= !USART_CR2_CLKEN;
        }
    }

    fn tx_completion_delay(&self, sys: &System) -> u64 {
        // BRR[15:4]=mantissa, BRR fraction depends on OVER8:
        // - OVER8=0: BRR[3:0] fraction/16
        // - OVER8=1: BRR[2:0] fraction/8 (bit3 ignored)
        // Use a bounded instruction-latency approximation so BRR changes affect
        // TXE/TC timing without stalling execution at very low baud values.
        if self.brr == 0 {
            return TX_COMPLETION_DELAY_DEFAULT;
        }

        let mantissa = (self.brr >> 4) & 0x0fff;
        let usartdiv_x16 = if (self.cr1 & USART_CR1_OVER8) != 0 {
            let fraction_over8 = self.brr & 0x0007;
            (mantissa << 4) | (fraction_over8 << 1)
        } else {
            let fraction_over16 = self.brr & 0x000f;
            (mantissa << 4) | fraction_over16
        };
        // Base delay from BRR divider.
        let base_delay = (usartdiv_x16 as u64) / 2;

        // Fold configured frame length into the latency estimate so CR1/CR2
        // programming (word length, parity, stop bits) influences TXE/TC timing.
        let data_bits = if (self.cr1 & USART_CR1_M) != 0 { 9u64 } else { 8u64 };
        let parity_bits = if (self.cr1 & USART_CR1_PCE) != 0 { 1u64 } else { 0u64 };
        let stop_half_bits = self.effective_stop_half_bits();
        let frame_half_bits = 2u64 + (data_bits * 2) + (parity_bits * 2) + stop_half_bits;
        let mut raw_delay = (base_delay.saturating_mul(frame_half_bits)).saturating_div(20);

        if (self.cr3 & USART_CR3_SCEN) != 0 {
            // Smartcard mode honors guard-time in GTPR[15:8].
            let guard_time = ((self.gtpr >> 8) & 0xFF) as u64;
            raw_delay = raw_delay.saturating_add(base_delay.saturating_mul(guard_time).saturating_div(16));
        }

        if (self.cr3 & USART_CR3_IREN) != 0 {
            // IrDA pulse-shaping path has a small additional serialization delay.
            raw_delay = raw_delay.saturating_add(base_delay.saturating_div(8));
        }

        let apb_div = self.apb_clock_divider(sys) as u64;
        raw_delay.saturating_mul(apb_div).clamp(2, 256)
    }

    fn rx_completion_delay(&self, sys: &System) -> u64 {
        // Use a bounded derivative of TX timing for receive sampling latency.
        // This keeps RX path BRR-sensitive without creating long stalls.
        let mut delay = self.tx_completion_delay(sys).saturating_div(2);
        if (self.cr3 & USART_CR3_SCEN) != 0 {
            delay = delay.saturating_add(((self.gtpr >> 8) & 0xFF) as u64);
        }
        if (self.cr3 & USART_CR3_IREN) != 0 {
            delay = delay.saturating_add(1);
        }
        delay.clamp(1, 128)
    }

    fn apb_clock_divider(&self, sys: &System) -> u32 {
        let mut cfgr = [0u8; 4];
        if sys.uc.borrow().mem_read(RCC_BASE + RCC_CFGR_OFFSET, &mut cfgr).is_err() {
            return 1;
        }
        let cfgr = u32::from_le_bytes(cfgr);
        let ppre1 = (cfgr >> 10) & 0b111;
        let ppre2 = (cfgr >> 13) & 0b111;

        let decode = |ppre: u32| -> u32 {
            match ppre {
                0b000..=0b011 => 1,
                0b100 => 2,
                0b101 => 4,
                0b110 => 8,
                0b111 => 16,
                _ => 1,
            }
        };

        if self.name.starts_with("USART1") || self.name.starts_with("USART6") {
            decode(ppre2)
        } else {
            decode(ppre1)
        }
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
            if now.saturating_sub(tx_since) >= self.tx_completion_delay(sys) {
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

    fn rx_decode_with_status(&self, raw_byte: u8) -> (u32, u32) {
        let mut status = 0u32;
        let mut data = (raw_byte as u32) & self.rx_data_mask();

        if (self.cr1 & USART_CR1_PCE) != 0 {
            let data_bits: u32 = if (self.cr1 & USART_CR1_M) != 0 { 9 } else { 8 };
            let payload_bits = data_bits.saturating_sub(1);

            if payload_bits <= 7 {
                let payload_mask = (1u32 << payload_bits) - 1;
                let payload = (raw_byte as u32) & payload_mask;
                let parity_bit = ((raw_byte as u32) >> payload_bits) & 1;
                let parity_ones = (payload.count_ones() & 1) as u32;
                let expected_parity_bit = if (self.cr1 & USART_CR1_PS) != 0 {
                    parity_ones ^ 1
                } else {
                    parity_ones
                };

                if parity_bit != expected_parity_bit {
                    status |= USART_SR_PE;
                }

                data = payload;
            }
        }

        // LIN break frame samples as all-zero payload in this simplified model.
        if (self.cr2 & USART_CR2_LINEN) != 0 && raw_byte == 0 {
            status |= USART_SR_FE;
            status |= USART_SR_LBD;
        }

        // IrDA receives are pulse-shaped and more susceptible to mark-noise in this model.
        // Treat an all-ones sample as a conservative noise indication.
        if (self.cr3 & USART_CR3_IREN) != 0 && raw_byte == 0xFF {
            status |= USART_SR_NE;
        }

        (data & self.rx_data_mask(), status)
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

        let now = NUM_INSTRUCTIONS.load(std::sync::atomic::Ordering::Relaxed);
        let byte = if let (Some(staged), Some(since)) = (self.rx_staged_byte, self.rx_active_since) {
            if now.saturating_sub(since) < self.rx_completion_delay(sys) {
                return;
            }
            self.rx_staged_byte = None;
            self.rx_active_since = None;
            staged
        } else {
            let b = dev.borrow_mut().read(sys, ());
            if b == 0 {
                return;
            }
            self.rx_staged_byte = Some(b);
            self.rx_active_since = Some(now);
            return;
        };

        let (decoded_data, error_status) = self.rx_decode_with_status(byte);
        if error_status != 0 {
            self.sr |= error_status;
        }

        if (self.cr3 & USART_CR3_DMAR) != 0 {
            // With DMAR enabled, stage a single pending byte for DMA consumption.
            // A second byte arriving before DMA drains the first is treated as overrun.
            if !self.rx_dma_pending.is_empty() {
                self.sr |= USART_SR_ORE;
                self.maybe_raise_irq(sys);
                return;
            }

            self.rx_dma_pending.push_back(decoded_data as u8);
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

        self.dr = decoded_data;
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
        let lbd = (self.sr & USART_SR_LBD) != 0;
        let lbdie = (self.cr2 & USART_CR2_LBDIE) != 0;

        if (tx_enabled && txe && txeie)
            || (tx_enabled && tc && tcie)
            || (rx_enabled && rxne && rxneie)
            || (rx_enabled && pe && peie)
            || (rx_enabled && err && eie)
            || (rx_enabled && lbd && lbdie)
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
                    self.sr &= !(USART_SR_LBD | USART_SR_RXNE | USART_SR_IDLE | USART_SR_ORE | USART_SR_NE | USART_SR_FE | USART_SR_PE);
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
                // TC can also be cleared by writing 0 to SR.TC.
                if (value & USART_SR_TC) == 0 {
                    self.sr &= !USART_SR_TC;
                }
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
                    self.rx_active_since = None;
                    self.rx_staged_byte = None;
                    self.sr &= !(USART_SR_LBD | USART_SR_RXNE | USART_SR_ORE | USART_SR_NE | USART_SR_FE | USART_SR_PE);
                    self.rx_dma_pending.clear();
                }

                if old_te && !new_te {
                    self.tx_active_since = None;
                    self.sr |= USART_SR_TXE | USART_SR_TC;
                }

                if old_re && !new_re {
                    self.rx_active_since = None;
                    self.rx_staged_byte = None;
                    self.sr &= !(USART_SR_LBD | USART_SR_RXNE | USART_SR_ORE | USART_SR_NE | USART_SR_FE | USART_SR_PE);
                    self.rx_dma_pending.clear();
                }

                self.maybe_raise_irq(sys);
            }
            0x0010 => {
                // Persist full CR2 state, but keep only defined stop-bit field in behavior.
                self.cr2 = value;
                self.normalize_mode_registers();
            }
            0x0014 => {
                // Persist CR3 state; DMAT/DMAR are consumed in DMA hooks.
                let old_dmar = (self.cr3 & USART_CR3_DMAR) != 0;
                self.cr3 = value;
                self.normalize_mode_registers();

                let new_dmar = (self.cr3 & USART_CR3_DMAR) != 0;
                if old_dmar && !new_dmar {
                    self.rx_dma_pending.clear();
                    self.sr &= !USART_SR_RXNE;
                }

                if !self.rx_enabled() {
                    self.rx_active_since = None;
                    self.rx_staged_byte = None;
                    self.sr &= !(USART_SR_LBD | USART_SR_RXNE | USART_SR_ORE | USART_SR_NE | USART_SR_FE | USART_SR_PE);
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
