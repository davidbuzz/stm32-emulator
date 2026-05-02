// SPDX-License-Identifier: GPL-3.0-or-later

// STM32 name: Ethernet MAC (Media Access Control for Gigabit Ethernet interface).
// STM32F427 base: ETH=0x40028000.
// Key registers: MACCR (config), MACMIIAR (MII address), MACMIIDR (MII data), MACRWUFFR (RX watchdog).
// Key function: on-chip gigabit Ethernet controller for network I/O (rarely used in embedded flight control).
// Critical for this emulator: CubeBlack doesn't require Ethernet; this is a stub to prevent crashes on MMIO.
// Current model: generic register persistence with no real networking or MAC behavior.
// Still incomplete: PHY communication, packet transmission/reception, DMA integration.
// Datasheet/reference anchor: STM32F4 RM Ethernet MAC chapters.

use crate::system::System;
use super::Peripheral;

#[derive(Default)]
pub struct Ethernet {
    /// MACCR (000H): MAC configuration register
    maccr: u32,
    /// MACFFR (004H): MAC frame filter register
    macffr: u32,
    /// MACHTHR (008H): MAC hash table high register
    machthr: u32,
    /// MACHTLR (00CH): MAC hash table low register
    machtlr: u32,
    /// MACMIIAR (010H): MAC MII address register
    macmiiar: u32,
    /// MACMIIDR (014H): MAC MII data register
    macmiidr: u32,
    /// MACFCR (018H): MAC flow control register
    macfcr: u32,
    /// MACVLANTR (01CH): MAC VLAN tag register
    macvlantr: u32,
    /// MACRWUFFR (028H): MAC remote wakeup frame filter register
    macrwuffr: u32,
    /// MACPMTCSR (02CH): MAC PMT control and status register
    macpmtcsr: u32,
    /// MACDBGR (034H): MAC debug register
    macdbgr: u32,
    /// MACSR (038H): MAC interrupt status register
    macsr: u32,
    /// MACIMR (03CH): MAC interrupt mask register
    macimr: u32,
    /// MACA0HR (040H): MAC address 0 high register
    maca0hr: u32,
    /// MACA0LR (044H): MAC address 0 low register
    maca0lr: u32,
}

impl Ethernet {
    pub fn new(_name: &str) -> Option<Box<dyn Peripheral>> {
        Some(Box::new(Self::default()))
    }
}

impl Peripheral for Ethernet {
    fn read(&mut self, _sys: &System, offset: u32) -> u32 {
        match offset {
            0x0000 => self.maccr,
            0x0004 => self.macffr,
            0x0008 => self.machthr,
            0x000C => self.machtlr,
            0x0010 => self.macmiiar,
            0x0014 => self.macmiidr,
            0x0018 => self.macfcr,
            0x001C => self.macvlantr,
            0x0028 => self.macrwuffr,
            0x002C => self.macpmtcsr,
            0x0034 => self.macdbgr,
            0x0038 => self.macsr,
            0x003C => self.macimr,
            0x0040 => self.maca0hr,
            0x0044 => self.maca0lr,
            _ => 0,
        }
    }

    fn write(&mut self, _sys: &System, offset: u32, value: u32) {
        match offset {
            0x0000 => self.maccr = value,
            0x0004 => self.macffr = value,
            0x0008 => self.machthr = value,
            0x000C => self.machtlr = value,
            0x0010 => self.macmiiar = value,
            0x0014 => self.macmiidr = value,
            0x0018 => self.macfcr = value,
            0x001C => self.macvlantr = value,
            0x0028 => self.macrwuffr = value,
            0x002C => self.macpmtcsr = value,
            0x0034 => self.macdbgr = value,
            0x0038 => {
                // MACSR status bits are generally cleared by writing 1.
                self.macsr &= !value;
            }
            0x003C => self.macimr = value,
            0x0040 => self.maca0hr = value,
            0x0044 => self.maca0lr = value,
            _ => {}
        }
    }

    fn step(&mut self, _sys: &System) {
        // No time-based behavior for Ethernet stub
    }
}
