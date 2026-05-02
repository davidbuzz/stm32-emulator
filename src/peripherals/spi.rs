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

const SPI_CR1_CPHA: u32 = 1 << 0;
const SPI_CR1_MSTR: u32 = 1 << 2;
const SPI_CR1_SPE: u32 = 1 << 6;
const SPI_CR1_SSI: u32 = 1 << 8;
const SPI_CR1_SSM: u32 = 1 << 9;

const SPI_CR2_RXDMAEN: u32 = 1 << 0;
const SPI_CR2_TXDMAEN: u32 = 1 << 1;
const SPI_CR2_ERRIE: u32 = 1 << 5;
const SPI_CR2_RXNEIE: u32 = 1 << 6;
const SPI_CR2_TXEIE: u32 = 1 << 7;

const SPI_SR_RXNE: u32 = 1 << 0;
const SPI_SR_TXE: u32 = 1 << 1;
const SPI_SR_MODF: u32 = 1 << 5;
const SPI_SR_OVR: u32 = 1 << 6;
const SPI_SR_BSY: u32 = 1 << 7;

#[derive(Default)]
pub struct Spi {
    pub name: String,
    pub cr1: u32,
    pub cr2: u32,
    pub sr: u32,
    pub rx_buffer: u32,
    pub rxne: bool,           // RXNE: receive data available
    pub ovr: bool,
    pub modf: bool,
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

    fn irq(&self) -> Option<i32> {
        Some(match self.name.as_str() {
            "SPI1" => 35,
            "SPI2" => 36,
            "SPI3" => 51,
            "SPI4" => 84,
            "SPI5" => 85,
            "SPI6" => 86,
            _ => return None,
        })
    }

    fn check_mode_fault(&mut self) {
        let master = (self.cr1 & SPI_CR1_MSTR) != 0;
        let hw_nss = (self.cr1 & SPI_CR1_SSM) == 0;
        let nss_low = (self.cr1 & SPI_CR1_SSI) == 0;
        if master && hw_nss && nss_low {
            self.modf = true;
            // Hardware clears SPE on mode fault.
            self.cr1 &= !SPI_CR1_SPE;
        }
    }

    fn build_sr(&self) -> u32 {
        let mut sr = 0;
        if (self.sr & SPI_SR_BSY) == 0 {
            sr |= SPI_SR_TXE;
        }
        if self.rxne {
            sr |= SPI_SR_RXNE;
        }
        if self.ovr {
            sr |= SPI_SR_OVR;
        }
        if self.modf {
            sr |= SPI_SR_MODF;
        }
        if (self.sr & SPI_SR_BSY) != 0 {
            sr |= SPI_SR_BSY;
        }
        sr
    }

    fn maybe_raise_irq(&self, sys: &System) {
        let Some(irq) = self.irq() else {
            return;
        };
        let sr = self.build_sr();
        let rxne_irq = (self.cr2 & SPI_CR2_RXNEIE) != 0 && (sr & SPI_SR_RXNE) != 0;
        let txe_irq = (self.cr2 & SPI_CR2_TXEIE) != 0 && (sr & SPI_SR_TXE) != 0;
        let err_irq = (self.cr2 & SPI_CR2_ERRIE) != 0 && (sr & (SPI_SR_OVR | SPI_SR_MODF)) != 0;
        if rxne_irq || txe_irq || err_irq {
            sys.p.nvic.borrow_mut().set_intr_pending(irq);
        }
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
            let txdmaen = self.cr2 & SPI_CR2_TXDMAEN != 0;
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
        self.rxne = !rx_bytes.is_empty();
        self.sr &= !SPI_SR_BSY;
        self.maybe_raise_irq(sys);
    }

    fn read(&mut self, _sys: &System, offset: u32) -> u32 {
        match offset {
            0x0000 => {
                self.cr1
            }
            0x0004 => self.cr2,
            0x0008 => {
                self.build_sr()
            }
            0x000C => {
                // DR register: reading clears RXNE
                let v = self.rx_buffer;
                self.rxne = false;
                // Simplified OVR clear path for firmware polling loops.
                self.ovr = false;
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
                self.check_mode_fault();
                self.maybe_raise_irq(sys);
            }
            0x0004 => {
                // CR2 register — track TXDMAEN (bit 1) and RXDMAEN (bit 0) for DMA mode detection
                self.cr2 = value;
                self.maybe_raise_irq(sys);
            }
            0x000C => {
                // DR register write: perform SPI exchange, set RXNE
                if (self.cr1 & SPI_CR1_SPE) == 0 {
                    // Ignore writes while disabled.
                    return;
                }
                self.check_mode_fault();
                if self.modf {
                    self.maybe_raise_irq(sys);
                    return;
                }

                // Writing while previous RX data is unread raises overrun.
                if self.rxne {
                    self.ovr = true;
                }

                self.sr |= SPI_SR_BSY;

                let cpha = (self.cr1 & SPI_CR1_CPHA) != 0;
                let (rx_buffer, tx_first) = if cpha {
                    // CPHA=1 samples later in the cycle: write first, then read response.
                    (true, true)
                } else {
                    // CPHA=0 uses existing behavior: read first, then shift out MOSI.
                    (true, false)
                };

                self.rx_buffer = self.ext_device.as_ref().map(|d| d.borrow_mut()).map(|mut d| {
                    if self.is_16bits() {
                        if tx_first {
                            d.write(sys, (), (value >> 8) as u8);
                            d.write(sys, (), value as u8);
                            let h = d.read(sys, ()) as u32;
                            let l = d.read(sys, ()) as u32;
                            (h << 8) | l
                        } else {
                            let h = d.read(sys, ()) as u32;
                            let l = d.read(sys, ()) as u32;
                            (h << 8) | l
                        }
                    } else {
                        if tx_first {
                            d.write(sys, (), value as u8);
                            d.read(sys, ()) as u32
                        } else {
                            d.read(sys, ()) as u32
                        }
                    }
                }).unwrap_or(0);

                if self.is_16bits() {
                    if !tx_first {
                        self.ext_device.as_ref().map(|d| d.borrow_mut()).map(|mut d| {
                            d.write(sys, (), (value >> 8) as u8);
                            d.write(sys, (), value as u8);
                        });
                    }

                    trace!("{} write={:04x?}", self.name, value as u16);
                } else {
                    let v = value as u8;
                    if !tx_first {
                        self.ext_device.as_ref().map(|d| d.borrow_mut().write(sys, (), v));
                    }
                    trace!("{} write={:02x?}", self.name, v);
                }

                // After exchange, RX data is available
                self.rxne = rx_buffer;
                self.sr &= !SPI_SR_BSY;
                self.maybe_raise_irq(sys);
            }
            _ => {}
        }
    }
}
