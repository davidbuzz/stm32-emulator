// SPDX-License-Identifier: GPL-3.0-or-later

// STM32 name: SPI1..SPI6.
// STM32F427 bases: SPI1=0x40013000, SPI2=0x40003800, SPI3=0x40003C00, SPI4=0x40013400, SPI5=0x40015000, SPI6=0x40015400.
// Key registers: CR1, CR2, SR, DR, CRCPR, RXCRCR, TXCRCR, I2SCFGR, I2SPR.
// Key function: synchronous serial traffic for sensors, flash, and board-attached devices.
// Critical for this emulator: ArduPilot runtime reaches SPI2 DMA-backed traffic on CubeBlack.
// Current model is intentionally simple and focuses on DR transfers plus a minimal ready/busy illusion.
// Still incomplete: stateful SR bits, DMA request generation, error flags, and detailed mode semantics.
// Datasheet/reference anchor: STM32F4 RM SPI/I2S chapter.

use crate::{system::System, ext_devices::ExtDevice};
use super::Peripheral;

use crate::ext_devices::ExtDevices;

use std::{rc::Rc, cell::RefCell};

#[derive(Default)]
pub struct Spi {
    pub name: String,
    pub cr1: u32,
    pub rx_buffer: u32,
    pub ready_toggle: bool,
    pub ext_device: Option<Rc<RefCell<dyn ExtDevice<(), u8>>>>,
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
    fn read(&mut self, _sys: &System, offset: u32) -> u32 {
        match offset {
            0x0000 => {
                self.cr1
            }
            0x0008 => {
                // SR register
                // receive buffer not empty
                // transmit buffer empty
                self.ready_toggle = !self.ready_toggle;
                if self.ready_toggle { 0b11 } else { 0 }
            }
            0x000C => {
                // DR register
                let v = self.rx_buffer;
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
            0x000C => {
                // DR register

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
            }
            _ => {}
        }
    }
}
