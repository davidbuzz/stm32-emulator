// SPDX-License-Identifier: GPL-3.0-or-later

// STM32 name: SDIO (SD/MMC card interface).
// STM32F427 base: SDIO=0x40012C00.
// Key registers: POWER(0x00), CLKCR(0x04), ARG(0x08), CMD(0x0C), RESPCMD(0x10),
//   RESP1-4(0x14-0x20), DTIMER(0x24), DLEN(0x28), DCTRL(0x2C), DCOUNT(0x30),
//   STA(0x34), ICR(0x38), MASK(0x3C), FIFOCNT(0x48), FIFO(0x80).
// Key behavior: firmware sends SD commands and waits for CMDSENT or CMDREND in STA.
//   No SD card present → emulate CTIMEOUT (bit 2) after CMDSENT (bit 7) for command responses.
// Datasheet/reference anchor: STM32F4 RM SDIO chapter.

use crate::system::System;
use super::Peripheral;

// SDIO_STA bits
const STA_CCRCFAIL: u32 = 1 << 0;
const STA_CTIMEOUT: u32 = 1 << 2;
const STA_CMDREND: u32  = 1 << 6;
const STA_CMDSENT: u32  = 1 << 7;

// SDIO_CMD bits
const CMD_CPSMEN: u32  = 1 << 10; // Command path state machine enable
const CMD_WAITRESP_MASK: u32 = 0b11 << 6;

#[derive(Default)]
pub struct Sdio {
    power: u32,
    clkcr: u32,
    arg: u32,
    cmd: u32,
    sta: u32,
    mask: u32,
}

impl Sdio {
    pub fn new(name: &str) -> Option<Box<dyn Peripheral>> {
        if name == "SDIO" {
            Some(Box::new(Self::default()))
        } else {
            None
        }
    }
}

impl Peripheral for Sdio {
    fn read(&mut self, _sys: &System, offset: u32) -> u32 {
        match offset {
            0x00 => self.power,
            0x04 => self.clkcr,
            0x08 => self.arg,
            0x0C => self.cmd,
            0x10 => 0, // RESPCMD
            0x14..=0x20 => 0, // RESP1-4: no response (no card)
            0x24 => 0, // DTIMER
            0x28 => 0, // DLEN
            0x2C => 0, // DCTRL
            0x30 => 0, // DCOUNT
            0x34 => self.sta,
            0x3C => self.mask,
            0x48 => 0, // FIFOCNT
            0x80..=0xFF => 0, // FIFO
            _ => 0,
        }
    }

    fn write(&mut self, _sys: &System, offset: u32, value: u32) {
        match offset {
            0x00 => self.power = value,
            0x04 => self.clkcr = value,
            0x08 => self.arg = value,
            0x0C => {
                self.cmd = value;
                if value & CMD_CPSMEN != 0 {
                    // Command path state machine enabled: simulate command sent immediately.
                    // Always set CMDSENT to unblock firmware wait loop.
                    // If firmware expects a response (WAITRESP != 0), set CTIMEOUT (no card).
                    let waitresp = (value & CMD_WAITRESP_MASK) >> 6;
                    if waitresp == 0 {
                        // No response expected: just signal CMDSENT
                        self.sta |= STA_CMDSENT;
                    } else {
                        // Response expected: signal CMDSENT then CTIMEOUT (no card present)
                        self.sta |= STA_CMDSENT | STA_CTIMEOUT;
                    }
                    debug!("SDIO CMD=0x{:02x} arg=0x{:08x} waitresp={} → sta=0x{:08x}",
                        value & 0x3F, self.arg, waitresp, self.sta);
                }
            }
            0x24 => {} // DTIMER
            0x28 => {} // DLEN
            0x2C => {} // DCTRL
            0x38 => {
                // ICR: clear indicated status bits
                self.sta &= !value;
            }
            0x3C => self.mask = value,
            _ => {}
        }
    }
}
