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
const STA_DTIMEOUT: u32 = 1 << 3;
const STA_CTIMEOUT: u32 = 1 << 2;
const STA_DATAEND: u32 = 1 << 8;
const STA_STBITERR: u32 = 1 << 9;
const STA_RXOVERR: u32 = 1 << 5;
const STA_TXUNDERR: u32 = 1 << 4;
const STA_DCRCFAIL: u32 = 1 << 1;
const STA_CCRCFAIL: u32 = 1 << 0;
const STA_CMDREND: u32 = 1 << 6;
const STA_CMDSENT: u32  = 1 << 7;

// SDIO_CMD bits
const CMD_CPSMEN: u32  = 1 << 10; // Command path state machine enable
const CMD_WAITRESP_MASK: u32 = 0b11 << 6;
const DCTRL_DTEN: u32 = 1 << 0;
const SDIO_IRQ_NUMBER: i32 = 49;
const SD_EJECT_RETRY_LIMIT: u32 = 100;
const DATA_TIMEOUT_DELAY_STEPS: u8 = 4;

const CMD_GO_IDLE_STATE: u32 = 0;
const CMD_ALL_SEND_CID: u32 = 2;
const CMD_SEND_RELATIVE_ADDR: u32 = 3;
const CMD_SET_DSR_OR_SWITCH: u32 = 4;
const CMD_SWITCH_OR_SET_BUS_WIDTH: u32 = 6;
const CMD_SEL_DESEL_CARD: u32 = 7;
const CMD_SEND_IF_COND: u32 = 8;
const CMD_SEND_CSD: u32 = 9;
const CMD_SEND_STATUS: u32 = 13;
const CMD_SET_BLOCKLEN: u32 = 16;
const CMD_READ_SINGLE_BLOCK: u32 = 17;
const CMD_READ_MULTIPLE_BLOCK: u32 = 18;
const CMD_WRITE_BLOCK: u32 = 24;
const CMD_WRITE_MULTIPLE_BLOCK: u32 = 25;
const CMD_APP_OP_COND: u32 = 41;
const CMD_APP_CMD: u32 = 55;

const SHORT_R1_OK: u32 = 0;
const SHORT_R1_TRAN: u32 = 4 << 9;
const SHORT_R3_OCR_READY_HC: u32 = 0xC000_0000;
const SHORT_R6_RCA: u32 = 0x0001_0000;
const SHORT_R7_IF_COND: u32 = 0x0000_01AA;

const FAKE_CID: [u32; 4] = [0x0353_4445, 0x4D55_3031, 0x1012_3456, 0x7801_7AFE];
const FAKE_CSD: [u32; 4] = [0x4000_0032, 0x5B59_0000, 0x003F_4000, 0x0000_80FE];

#[derive(Default)]
pub struct Sdio {
    power: u32,
    clkcr: u32,
    arg: u32,
    cmd: u32,
    respcmd: u32,
    resp: [u32; 4],
    dtimer: u32,
    dlen: u32,
    dctrl: u32,
    sta: u32,
    mask: u32,
    data_timeout_delay: u8,
    failed_retries: u32,
    is_ejected: bool,
}

impl Sdio {
    pub fn new(name: &str) -> Option<Box<dyn Peripheral>> {
        if name == "SDIO" {
            Some(Box::new(Self::default()))
        } else {
            None
        }
    }

    fn set_short_response(&mut self, cmd: u32, resp1: u32) {
        self.respcmd = cmd;
        self.resp = [resp1, 0, 0, 0];
        self.sta |= STA_CMDREND;
    }

    fn set_long_response(&mut self, cmd: u32, resp: [u32; 4]) {
        self.respcmd = cmd;
        self.resp = [resp[3], resp[2], resp[1], resp[0]];
        self.sta |= STA_CMDREND;
    }

    fn handle_known_command(&mut self, cmd: u32) -> bool {
        match cmd {
            CMD_GO_IDLE_STATE => {
                self.respcmd = cmd;
                self.sta |= STA_CMDSENT;
                true
            }
            CMD_SEND_IF_COND => {
                self.set_short_response(cmd, SHORT_R7_IF_COND);
                true
            }
            CMD_APP_CMD => {
                self.set_short_response(cmd, SHORT_R1_OK);
                true
            }
            CMD_APP_OP_COND => {
                self.set_short_response(cmd, SHORT_R3_OCR_READY_HC);
                true
            }
            CMD_ALL_SEND_CID => {
                self.set_long_response(cmd, FAKE_CID);
                true
            }
            CMD_SEND_RELATIVE_ADDR => {
                self.set_short_response(cmd, SHORT_R6_RCA);
                true
            }
            CMD_SEND_CSD => {
                self.set_long_response(cmd, FAKE_CSD);
                true
            }
            CMD_SEL_DESEL_CARD | CMD_SET_BLOCKLEN | CMD_SWITCH_OR_SET_BUS_WIDTH | CMD_SET_DSR_OR_SWITCH => {
                self.set_short_response(cmd, SHORT_R1_OK);
                true
            }
            CMD_SEND_STATUS => {
                self.set_short_response(cmd, SHORT_R1_TRAN);
                true
            }
            CMD_READ_SINGLE_BLOCK | CMD_READ_MULTIPLE_BLOCK | CMD_WRITE_BLOCK | CMD_WRITE_MULTIPLE_BLOCK => {
                self.set_short_response(cmd, SHORT_R1_OK);
                true
            }
            _ => false,
        }
    }
}

impl Peripheral for Sdio {
    fn step(&mut self, sys: &System) {
        if self.data_timeout_delay > 0 {
            self.data_timeout_delay -= 1;
            if self.data_timeout_delay == 0 && self.mask != 0 {
                self.sta |= STA_DTIMEOUT;
                sys.p.nvic.borrow_mut().set_intr_pending(SDIO_IRQ_NUMBER);
            }
        }
    }

    fn read(&mut self, _sys: &System, offset: u32) -> u32 {
        match offset {
            0x00 => self.power,
            0x04 => self.clkcr,
            0x08 => self.arg,
            0x0C => self.cmd,
            0x10 => self.respcmd,
            0x14 => self.resp[0],
            0x18 => self.resp[1],
            0x1C => self.resp[2],
            0x20 => self.resp[3],
            0x24 => self.dtimer,
            0x28 => self.dlen,
            0x2C => self.dctrl,
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
                self.sta &= !(STA_CMDSENT | STA_CMDREND | STA_CTIMEOUT | STA_CCRCFAIL);
                if value & CMD_CPSMEN != 0 {
                    let cmd = value & 0x3F;
                    if self.handle_known_command(cmd) {
                        debug!(
                            "SDIO CMD=0x{:02x} arg=0x{:08x} → sta=0x{:08x}",
                            cmd,
                            self.arg,
                            self.sta
                        );
                    } else {
                        let waitresp = (value & CMD_WAITRESP_MASK) >> 6;
                        if waitresp == 0 {
                            self.respcmd = cmd;
                            self.sta |= STA_CMDSENT;
                        } else {
                            self.sta |= STA_CMDSENT | STA_CTIMEOUT;
                            if !self.is_ejected {
                                self.failed_retries = self.failed_retries.saturating_add(1);
                            }
                            if !self.is_ejected && self.failed_retries >= SD_EJECT_RETRY_LIMIT {
                                self.is_ejected = true;
                                info!(
                                    "SDIO marked ejected after {} failed command retries",
                                    self.failed_retries
                                );
                            }
                        }
                        debug!(
                            "SDIO{} CMD=0x{:02x} arg=0x{:08x} waitresp={} retries={} → sta=0x{:08x}",
                            if self.is_ejected { "[ejected]" } else { "" },
                            cmd,
                            self.arg,
                            waitresp,
                            self.failed_retries,
                            self.sta
                        );
                    }
                }
            }
            0x24 => self.dtimer = value,
            0x28 => self.dlen = value,
            0x2C => {
                self.dctrl = value;
                self.sta &= !(STA_DTIMEOUT | STA_DATAEND | STA_STBITERR | STA_RXOVERR | STA_TXUNDERR | STA_DCRCFAIL);
                if (value & DCTRL_DTEN) != 0 {
                    self.data_timeout_delay = DATA_TIMEOUT_DELAY_STEPS;
                } else {
                    self.data_timeout_delay = 0;
                }
            }
            0x38 => {
                // ICR: clear indicated status bits
                self.sta &= !value;
            }
            0x3C => self.mask = value,
            _ => {}
        }
    }
}
