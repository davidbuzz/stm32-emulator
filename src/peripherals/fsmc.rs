// SPDX-License-Identifier: GPL-3.0-or-later

// STM32 name: FSMC (Flexible Static Memory Controller).
// STM32F427 register base: 0xA0000000; external memory banks map into 0x60000000..0x9FFFFFFF.
// Key function: bridges external NOR/SRAM/LCD-style buses onto the AHB memory space.
// Key registers normally include BCR/BTR/BWTR bank control and timing state.
// Critical for this emulator: board-attached LCD/display devices hang off FSMC bank windows.
// This model is bank-oriented rather than register-complete and is aimed at external device hookup.
// Emulator-specific note: registration remaps the live access range to the external bank window.
// Datasheet/reference anchors: STM32F4 RM FSMC chapter and board-specific display wiring.

use std::{cell::RefCell, rc::Rc};

use crate::{system::System, ext_devices::{ExtDevices, ExtDevice}};
use super::Peripheral;

pub struct Fsmc {
    banks: [Bank; 4],
}

impl Fsmc {
    pub fn new(name: &str, ext_devices: &ExtDevices) -> Option<Box<dyn Peripheral>> {
        if name.starts_with("FSMC") {
            let banks = [
                Bank::new(0, ext_devices),
                Bank::new(1, ext_devices),
                Bank::new(2, ext_devices),
                Bank::new(3, ext_devices),
            ];
            Some(Box::new(Self { banks }))
        } else {
            None
        }
    }

    fn access(offset: u32) -> Access {
        match offset {
            0x0000_0000..=0x0fff_ffff => Access::Data(0, offset),
            0x1000_0000..=0x1fff_ffff => Access::Data(1, offset - 0x1000_0000),
            0x2000_0000..=0x2fff_ffff => Access::Data(2, offset - 0x2000_0000),
            0x3000_0000..=0x3fff_ffff => Access::Data(3, offset - 0x3000_0000),
            0x4000_0000..=0x4fff_ffff => {
                match offset - 0x4000_0000 {
                    0x0000 => Access::Register(0, Reg::BCR),
                    0x0004 => Access::Register(0, Reg::BTR),
                    0x0008 => Access::Register(1, Reg::BCR),
                    0x000C => Access::Register(1, Reg::BTR),
                    0x0010 => Access::Register(2, Reg::BCR),
                    0x0014 => Access::Register(2, Reg::BTR),
                    0x0018 => Access::Register(3, Reg::BCR),
                    0x001C => Access::Register(3, Reg::BTR),
                    0x0060 => Access::Register(1, Reg::PCR),
                    0x0064 => Access::Register(1, Reg::SR),
                    0x0068 => Access::Register(1, Reg::PMEM),
                    0x006C => Access::Register(1, Reg::PATT),
                    0x0074 => Access::Register(1, Reg::ECCR),
                    0x0080 => Access::Register(2, Reg::PCR),
                    0x0084 => Access::Register(2, Reg::SR),
                    0x0088 => Access::Register(2, Reg::PMEM),
                    0x008C => Access::Register(2, Reg::PATT),
                    0x0094 => Access::Register(2, Reg::ECCR),
                    0x00A0 => Access::Register(3, Reg::PCR),
                    0x00A4 => Access::Register(3, Reg::SR),
                    0x00A8 => Access::Register(3, Reg::PMEM),
                    0x00AC => Access::Register(3, Reg::PATT),
                    0x00B0 => Access::Register(3, Reg::PIO),
                    0x0104 => Access::Register(0, Reg::BWTR),
                    0x010C => Access::Register(1, Reg::BWTR),
                    0x0114 => Access::Register(2, Reg::BWTR),
                    0x011C => Access::Register(3, Reg::BWTR),
                    _ => Access::Register(0, Reg::Invalid),
                }
            }
            _ => unreachable!()
        }
    }
}

impl Peripheral for Fsmc {
    fn read(&mut self, sys: &System, offset: u32) -> u32 {
        match Self::access(offset) {
            Access::Data(bank, offset) => self.banks[bank].read_data(sys, offset),
            Access::Register(bank, reg) => self.banks[bank].read_reg(sys, reg),
        }
    }

    fn write(&mut self, sys: &System, offset: u32, value: u32) {
        match Self::access(offset) {
            Access::Data(bank, offset) => self.banks[bank].write_data(sys, offset, value),
            Access::Register(bank, reg) => self.banks[bank].write_reg(sys, reg, value),
        }
    }
}

pub trait FsmcDevice {
    fn name(&self, fsmc_bank_name: &str) -> String;
    fn read_data(&mut self, bank: &mut Bank, offset: u32) -> u32;
    fn write_data(&mut self, bank: &mut Bank, offset: u32, value: u32);
}

pub struct Bank {
    pub name: String,
    ext_device: Option<Rc<RefCell<dyn ExtDevice<u32, u32>>>>,
    // Control/timing register state.  Reset values from STM32F4 RM FSMC chapter.
    bcr: u32,   // Bank Control Register (BCR1 reset=0x000030D2, BCR2-4 reset=0x000030D0)
    btr: u32,   // Bank Timing Register  (reset=0xFFFFFFFF)
    bwtr: u32,  // Bank Write Timing Register (reset=0x0FFF_FFFF)
    pcr: u32,   // NAND/PCCARD Control Register (reset=0x0000_0018)
    sr: u32,    // FIFO Status Register (FEMPT=bit6=1 always means FIFO empty)
    pmem: u32,  // Common memory timing (reset=0xFCFCFCFC)
    patt: u32,  // Attribute memory timing (reset=0xFCFCFCFC)
    pio: u32,   // I/O space timing (reset=0xFCFCFCFC)
}

impl Bank {
    pub fn new(bank: usize, ext_devices: &ExtDevices) -> Self {
        let name = format!("FSMC.BANK{}", bank+1);

        let ext_device = ext_devices.find_mem_device(&name);
        let name = ext_device.as_ref()
            .map(|d| d.borrow_mut().connect_peripheral(&name))
            .unwrap_or(name);

        // BCR1 has FACCEN bit set (bit 6); BCR2-4 do not.
        let bcr = if bank == 0 { 0x0000_30D2 } else { 0x0000_30D0 };

        Self {
            name,
            ext_device,
            bcr,
            btr: 0xFFFF_FFFF,
            bwtr: 0x0FFF_FFFF,
            pcr: 0x0000_0018,
            sr: 0x0000_0040, // FEMPT=1
            pmem: 0xFCFC_FCFC,
            patt: 0xFCFC_FCFC,
            pio: 0xFCFC_FCFC,
        }
    }

    fn read_data(&mut self, sys: &System, offset: u32) -> u32 {
        let v = self.ext_device.as_ref().map(|d|
            d.borrow_mut().read(sys, offset)
        ).unwrap_or_default();

        trace!("{} data read at offset=0x{:08x} value=0x{:08x}", self.name, offset, v);

        v
    }

    fn write_data(&mut self, sys: &System, offset: u32, value: u32) {
        self.ext_device.as_ref().map(|d|
            d.borrow_mut().write(sys, offset, value)
        );

        trace!("{} data write at offset=0x{:08x} value=0x{:08x}", self.name, offset, value);
    }

    fn read_reg(&mut self, _sys: &System, reg: Reg) -> u32 {
        let v = match reg {
            Reg::BCR  => self.bcr,
            Reg::BTR  => self.btr,
            Reg::BWTR => self.bwtr,
            Reg::PCR  => self.pcr,
            Reg::SR   => self.sr | 0x40, // FEMPT always set
            Reg::PMEM => self.pmem,
            Reg::PATT => self.patt,
            Reg::PIO  => self.pio,
            Reg::ECCR | Reg::Invalid => 0,
        };
        trace!("{} read reg={:?} value=0x{:08x}", self.name, reg, v);
        v
    }

    fn write_reg(&mut self, _sys: &System, reg: Reg, value: u32) {
        trace!("{} write reg={:?} value=0x{:08x}", self.name, reg, value);
        match reg {
            Reg::BCR  => self.bcr  = value,
            Reg::BTR  => self.btr  = value,
            Reg::BWTR => self.bwtr = value,
            Reg::PCR  => self.pcr  = value,
            Reg::SR   => {} // SR is read-only (FEMPT is status)
            Reg::PMEM => self.pmem = value,
            Reg::PATT => self.patt = value,
            Reg::PIO  => self.pio  = value,
            Reg::ECCR | Reg::Invalid => {}
        }
    }
}

enum Access {
    Data(usize, u32),
    Register(usize, Reg),
}

#[derive(Debug)]
enum Reg {
    BCR,
    BTR,
    PMEM,
    PATT,
    ECCR,
    PCR,
    SR,
    BWTR,
    PIO,
    Invalid,
}
