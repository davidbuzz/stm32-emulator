// SPDX-License-Identifier: GPL-3.0-or-later

// STM32 name: SPI1..SPI6.
// STM32F427 bases: SPI1=0x40013000, SPI2=0x40003800, SPI3=0x40003C00, SPI4=0x40013400, SPI5=0x40015000, SPI6=0x40015400.
// Key registers: CR1, CR2, SR, DR, CRCPR, RXCRCR, TXCRCR, I2SCFGR, I2SPR.
// Key function: synchronous serial traffic for sensors, flash, and board-attached devices.
// Critical for this emulator: ArduPilot runtime reaches SPI2 DMA-backed traffic on CubeBlack.
// Current model is intentionally simple and focuses on DR transfers plus a minimal ready/busy illusion.
// Still incomplete: stateful SR bits, DMA request generation, error flags, and detailed mode semantics.
// Datasheet/reference anchor: STM32F4 RM SPI/I2S chapter.

use crate::{system::System, ext_devices::ExtDevice, util::UniErr};
use super::Peripheral;

use crate::ext_devices::ExtDevices;

use std::{rc::Rc, cell::RefCell};
use std::collections::VecDeque;

#[derive(Default)]
pub struct Spi {
    pub name: String,
    pub cr1: u32,
    pub cr2: u32,
    pub rx_buffer: u32,
    pub rxne: bool,           // RXNE: receive data available
    pub ext_device: Option<Rc<RefCell<dyn ExtDevice<(), u8>>>>,
    /// Pending RX DMA destination addresses collected during RX DMA bursts.
    /// TX DMA consumes these addresses and patches RAM with real MISO bytes.
    pending_rx_dest: VecDeque<u32>,
}

impl Spi {
    pub fn new(name: &str, ext_devices: &ExtDevices) -> Option<Box<dyn Peripheral>> {
        if name.starts_with("SPI") {
            let ext_device = ext_devices.find_serial_device(name);
            let name = ext_device.as_ref()
                .map(|d| d.borrow_mut().connect_peripheral(name))
                .unwrap_or_else(|| name.to_string());
            Some(Box::new(Self { name, ext_device, ..Default::default() }))
        } else {
            None
        }
    }

    pub fn is_16bits(&self) -> bool {
        self.cr1 & (1 << 11) != 0
    }
}

impl Peripheral for Spi {
    fn set_dma_rx_dest(&mut self, dest_addr: u32) {
        self.pending_rx_dest.push_back(dest_addr);
    }

    /// For full-duplex DMA (TXDMAEN set in CR2), the RX DMA fires first.
    /// Return empty to avoid overwriting the TX source buffer; write_dma handles the exchange.
    /// For receive-only DMA (TXDMAEN clear), generate MISO by sending dummy 0xFF writes.
    fn read_dma(&mut self, sys: &System, offset: u32, size: usize) -> std::collections::VecDeque<u8> {
        if offset == 0x000C {
            let txdmaen = self.cr2 & (1 << 1) != 0;
            if txdmaen {
                // Full-duplex exchange: write_dma will handle the actual exchange
                return std::collections::VecDeque::new();
            } else if let Some(dev) = &self.ext_device {
                // Receive-only: send dummy 0xFF bytes and collect MISO
                let dev = dev.clone();
                let mut miso = std::collections::VecDeque::new();
                for _ in 0..size {
                    dev.borrow_mut().write(sys, (), 0xFF);
                    miso.push_back(dev.borrow_mut().read(sys, ()));
                }
                return miso;
            }
        }
        std::collections::VecDeque::new()
    }

    /// Execute the full-duplex SPI exchange. Each byte in `value` is MOSI; we read MISO from
    /// the ext_device first (one byte behind, matching real SPI timing), then write MOSI.
    /// If a pending RX DMA destination was set by set_dma_rx_dest, patch that RAM location
    /// with the collected MISO bytes so the firmware sees the correct response.
    fn write_dma(&mut self, sys: &System, offset: u32, value: std::collections::VecDeque<u8>) {
        if offset != 0x000C {
            return;
        }
        let rx_bytes: Vec<u8> = value.into_iter().map(|v| {
            let rx = self.ext_device.as_ref()
                .map(|d| d.borrow_mut().read(sys, ()) as u8)
                .unwrap_or(0xFF);
            if let Some(d) = &self.ext_device {
                d.borrow_mut().write(sys, (), v);
            }
            rx
        }).collect();
        if !self.pending_rx_dest.is_empty() {
            for rx in rx_bytes.iter().copied() {
                let Some(dest) = self.pending_rx_dest.pop_front() else {
                    break;
                };
                if let Err(e) = sys.uc.borrow_mut().mem_write(dest.into(), &[rx]) {
                    warn!("{} DMA full-duplex patch failed dest=0x{:08x}: {}", self.name, dest, UniErr(e));
                }
            }
        } else {
            // No pending RX DMA: update rx_buffer with the last received byte (polling compat)
            if let Some(&last) = rx_bytes.last() {
                self.rx_buffer = last as u32;
            }
        }
    }

    fn read(&mut self, _sys: &System, offset: u32) -> u32 {
        match offset {
            0x0000 => {
                self.cr1
            }
            0x0004 => self.cr2,
            0x0008 => {
                // SR register: TXE(1)=1 always (DR empty), BSY(7)=0 always,
                // RXNE(0) reflects whether data is available to read.
                let rxne = if self.rxne { 1 } else { 0 };
                let txe = 1 << 1;  // TXE always set: DR is always ready for next write
                rxne | txe
            }
            0x000C => {
                // DR register: reading clears RXNE
                let v = self.rx_buffer;
                self.rxne = false;
                if self.is_16bits() {
                    trace!("{} read={:04x?}", self.name, v as u16);
                } else {
                    trace!("{} read={:02x?}", self.name, v as u8);
                }

                v
            }
            _ => 0
        }
    }

    fn write(&mut self, sys: &System, offset: u32, value: u32) {
        match offset {
            0x0000 => {
                // CR1 register
                self.cr1 = value;
            }
            0x0004 => {
                // CR2 register — track TXDMAEN (bit 1) and RXDMAEN (bit 0) for DMA mode detection
                self.cr2 = value;
            }
            0x000C => {
                // DR register write: perform SPI exchange, set RXNE

                self.rx_buffer = self.ext_device.as_ref().map(|d| d.borrow_mut()).map(|mut d| {
                    if self.is_16bits() {
                        let h = d.read(sys, ()) as u32;
                        let l = d.read(sys, ()) as u32;
                        (h << 8) | l
                    } else {
                        d.read(sys, ()) as u32
                    }
                }).unwrap_or(0);

                if self.is_16bits() {
                    self.ext_device.as_ref().map(|d| d.borrow_mut()).map(|mut d| {
                        d.write(sys, (), (value >> 8) as u8);
                        d.write(sys, (), value as u8);
                    });

                    trace!("{} write={:04x?}", self.name, value as u16);
                } else {
                    let v = value as u8;
                    self.ext_device.as_ref().map(|d| d.borrow_mut().write(sys, (), v));
                    trace!("{} write={:02x?}", self.name, v);
                }

                // After exchange, RX data is available
                self.rxne = true;
            }
            _ => {}
        }
    }
}
