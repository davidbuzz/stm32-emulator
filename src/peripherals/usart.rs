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
const USART_SR_TC: u32 = 1 << 6;
const USART_SR_TXE: u32 = 1 << 7;
const USART_CR3_DMAT: u32 = 1 << 6;  // Transmit DMA enable
const USART_CR3_DMAR: u32 = 1 << 5;  // Receive DMA enable

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
}

impl Usart {
    pub fn new(name: &str, ext_devices: &ExtDevices) -> Option<Box<dyn Peripheral>> {
        if name.starts_with("USART") || name.starts_with("UART") {
            let ext_device = ext_devices.find_serial_device(&name);
            let name = ext_device.as_ref()
                .map(|d| d.borrow_mut().connect_peripheral(name))
                .unwrap_or_else(|| name.to_string());
            Some(Box::new(Self {
                name,
                ext_device,
                // TXE(7) and TC(6) always set — transmitter immediately ready.
                // IDLE(4) set — line is idle since no incoming data is modeled.
                // RXNE(5) cleared — no incoming byte until ext_device provides one.
                sr: (1 << 7) | (1 << 6) | (1 << 4),
                ..Default::default()
            }))
        } else {
            None
        }
    }
}

impl Peripheral for Usart {
    fn read(&mut self, sys: &System, offset: u32) -> u32 {
        match offset {
            0x0000 => self.sr,
            0x0004 => {
                // DR register: reading clears RXNE
                let v = self.ext_device.as_ref()
                    .map(|d| d.borrow_mut().read(sys, ()))
                    .unwrap_or(self.dr as u8) as u32;

                self.dr = v;
                self.sr &= !(1 << 5); // clear RXNE after read

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
        match offset {
            0x0000 => {
                // TXE/TC are transmitter state bits; keep them asserted in this minimal model
                // instead of letting firmware clear them permanently through SR writes.
                self.sr = (value & !(USART_SR_TXE | USART_SR_TC)) | USART_SR_TXE | USART_SR_TC;
            }
            0x0004 => {
                // DR register - handling depends on whether DMA TX is enabled
                self.dr = value & 0xFF;
                
                // If DMA TX is not enabled, write directly to ext_device
                // (If DMAT is set, DMA controller handles writes via write_dma())
                if (self.cr3 & USART_CR3_DMAT) == 0 {
                    self.ext_device.as_ref().map(|d|
                        d.borrow_mut().write(sys, (), value as u8)
                    );
                }

                // TX is complete immediately in this minimal model.
                self.sr |= USART_SR_TXE | USART_SR_TC;

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
