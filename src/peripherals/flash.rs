// SPDX-License-Identifier: GPL-3.0-or-later

// STM32 name: FLASH (Flash interface registers).
// STM32F427 base: 0x40023c00.
// Key registers: ACR(0x00), KEYR(0x04), OPTKEYR(0x08), SR(0x0C), CR(0x10), OPTCR(0x14).
// Key function: flash wait states, prefetch control, lock/unlock key sequence, erase/program control.
// Critical for this emulator: startup code polls ACR latency after programming wait states;
//   runtime param/EEPROM emulation paths need BSY/EOP to complete after erase/program operations.
// This model: KEYR two-step unlock (KEY1=0x45670123, KEY2=0xCDEF89AB); CR locked on reset;
//   STRT sets BSY immediately, clears BSY + sets EOP after 2 step() calls; SR write-1-to-clear.
// Reference: STM32F4 RM Flash chapter; Renode STM32F4_FlashController.cs lines 56–197.
// Datasheet/reference anchor: STM32F4 RM Flash interface registers chapter.

use crate::system::System;
use super::Peripheral;

const FLASH_KEY1: u32 = 0x4567_0123;
const FLASH_KEY2: u32 = 0xCDEF_89AB;
const FLASH_OPTKEY1: u32 = 0x0819_2A3B;
const FLASH_OPTKEY2: u32 = 0x4C5D_6E7F;

const SR_EOP: u32   = 1 << 0;   // End of operation (write-1-to-clear)
const SR_OPERR: u32 = 1 << 1;   // Operation error
const SR_PGAERR: u32 = 1 << 5;  // Programming alignment error
const SR_PGPERR: u32 = 1 << 6;  // Programming parallelism error
const SR_PGSERR: u32 = 1 << 7;  // Programming sequence error
const SR_BSY: u32   = 1 << 16;  // Busy (read-only, cleared by hardware)

const FLASH_SR_WRITECLEAR_MASK: u32 =
    SR_EOP | SR_OPERR | SR_PGAERR | SR_PGPERR | SR_PGSERR;

const FLASH_ACR_WRITABLE_MASK: u32 = 0x0000_071F;

const FLASH_CR_WRITABLE_MASK: u32 =
    CR_PG | CR_SER | CR_MER | CR_STRT | (0x1F << 3) | CR_LOCK;

const CR_PG: u32    = 1 << 0;   // Programming
const CR_SER: u32   = 1 << 1;   // Sector erase
const CR_MER: u32   = 1 << 2;   // Mass erase (bank 1)
const CR_STRT: u32  = 1 << 16;  // Start erase/program (self-clearing)
const CR_LOCK: u32  = 1 << 31;  // Lock bit (set by HW on reset, cleared by key sequence)

pub struct Flash {
    acr: u32,
    sr: u32,
    cr: u32,
    optcr: u32,
    key_seq: u8,     // 0 = expecting KEY1, 1 = expecting KEY2, 2 = unlocked
    optkey_seq: u8,  // 0 = expecting OPTKEY1, 1 = expecting OPTKEY2, 2 = unlocked
    op_countdown: u8, // steps until BSY clears and EOP fires (0 = idle)
}

impl Flash {
    pub fn new(name: &str) -> Option<Box<dyn Peripheral>> {
        if name == "FLASH" {
            Some(Box::new(Flash {
                acr: 0,
                sr: 0,
                cr: CR_LOCK, // starts locked per RM reset value
                optcr: 0x0fff_aaed,
                key_seq: 0,
                optkey_seq: 0,
                op_countdown: 0,
            }))
        } else {
            None
        }
    }
}

impl Peripheral for Flash {
    fn step(&mut self, _sys: &System) {
        if self.op_countdown > 0 {
            self.op_countdown -= 1;
            if self.op_countdown == 0 {
                // Operation complete: clear BSY, set EOP, clear STRT (STRT is self-clearing).
                self.sr = (self.sr & !SR_BSY) | SR_EOP;
                self.cr &= !CR_STRT;
                debug!("FLASH operation complete: EOP set");
            }
        }
    }

    fn read(&mut self, _sys: &System, offset: u32) -> u32 {
        match offset {
            0x00 => {
                // PRFTBS (bit 5) mirrors PRFTBE (bit 4).
                let prftbs = if (self.acr & (1 << 4)) != 0 { 1 << 5 } else { 0 };
                (self.acr & !(1 << 5)) | prftbs
            }
            0x04 => 0, // KEYR is write-only
            0x08 => 0, // OPTKEYR is write-only
            0x0c => self.sr,
            0x10 => self.cr,
            0x14 => self.optcr,
            _ => 0,
        }
    }

    fn write(&mut self, _sys: &System, offset: u32, value: u32) {
        match offset {
            0x00 => self.acr = value & FLASH_ACR_WRITABLE_MASK,
            0x04 => {
                // KEYR: two-step unlock sequence.  Wrong key re-locks (bus fault on real HW,
                // but we just reset the sequence silently to avoid breaking firmware that retries).
                match self.key_seq {
                    0 if value == FLASH_KEY1 => self.key_seq = 1,
                    1 if value == FLASH_KEY2 => {
                        self.key_seq = 2;
                        self.cr &= !CR_LOCK;
                        debug!("FLASH unlocked (CR=0x{:08x})", self.cr);
                    }
                    _ => {
                        self.key_seq = 0;
                        self.cr |= CR_LOCK;
                        debug!("FLASH wrong key sequence: re-locked");
                    }
                }
            }
            0x08 => {
                match self.optkey_seq {
                    0 if value == FLASH_OPTKEY1 => self.optkey_seq = 1,
                    1 if value == FLASH_OPTKEY2 => {
                        self.optkey_seq = 2;
                        debug!("FLASH option bytes unlocked");
                    }
                    _ => self.optkey_seq = 0,
                }
            }
            0x0c => {
                // SR: write-1-to-clear; BSY is read-only (firmware cannot clear BSY by writing SR).
                self.sr &= !(value & FLASH_SR_WRITECLEAR_MASK);
            }
            0x10 => {
                if self.cr & CR_LOCK != 0 {
                    // CR writes are ignored while locked (hardware behavior).
                    self.sr |= SR_OPERR | SR_PGSERR;
                    trace!("FLASH CR write ignored (locked): value=0x{:08x}", value);
                    return;
                }
                self.cr = value & FLASH_CR_WRITABLE_MASK;

                // Firmware locking: writing LOCK=1 re-locks the controller.
                if value & CR_LOCK != 0 {
                    self.key_seq = 0;
                    debug!("FLASH re-locked by firmware");
                }

                // STRT triggers erase or program; begin deferred BSY→EOP sequence.
                if value & CR_STRT != 0 && value & (CR_SER | CR_MER | CR_PG) != 0 {
                    self.sr |= SR_BSY;
                    self.op_countdown = if value & CR_MER != 0 {
                        8
                    } else if value & CR_SER != 0 {
                        4
                    } else {
                        2
                    };
                    let op = if value & CR_MER != 0 { "mass erase" }
                             else if value & CR_SER != 0 {
                                 let snb = (value >> 3) & 0x1F;
                                 if snb > 11 {
                                     self.sr |= SR_OPERR | SR_PGSERR;
                                     self.sr &= !SR_BSY;
                                     self.op_countdown = 0;
                                     self.cr &= !CR_STRT;
                                     warn!("FLASH invalid sector erase request SNB={} (CR=0x{:08x})", snb, value);
                                     return;
                                 }
                                 // Log SNB and return early so the borrow is clean
                                 debug!("FLASH sector erase started (SNB={} CR=0x{:08x})", snb, value);
                                 return;
                             }
                             else { "program" };
                    debug!("FLASH {} started (CR=0x{:08x})", op, value);
                } else if value & CR_STRT != 0 {
                    // START with no selected operation is invalid.
                    self.sr |= SR_OPERR | SR_PGSERR;
                    self.cr &= !CR_STRT;
                    warn!("FLASH STRT without PG/SER/MER (CR=0x{:08x})", value);
                }
            }
            0x14 => self.optcr = value,
            _ => {}
        }
    }
}
