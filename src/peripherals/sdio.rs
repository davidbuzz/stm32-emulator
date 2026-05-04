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
use std::collections::VecDeque;

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
const DMA2_BASE: u64 = 0x4002_6400;
const DMA_STREAM_CR_BASE_OFFSET: u64 = 0x10;
const DMA_STREAM_STRIDE: u64 = 0x18;
const DMA_STREAM_COUNT: u64 = 8;

const CMD_GO_IDLE_STATE: u32 = 0;
const CMD_ALL_SEND_CID: u32 = 2;
const CMD_SEND_RELATIVE_ADDR: u32 = 3;
const CMD_SET_DSR_OR_SWITCH: u32 = 4;
const CMD_SWITCH_OR_SET_BUS_WIDTH: u32 = 6;
const CMD_SEL_DESEL_CARD: u32 = 7;
const CMD_SEND_IF_COND: u32 = 8;
const CMD_SEND_CSD: u32 = 9;
const CMD_SEND_STATUS: u32 = 13;
const CMD_STOP_TRANSMISSION: u32 = 12;
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

const STA_CLEARABLE_MASK: u32 =
    STA_CCRCFAIL | STA_DCRCFAIL | STA_CTIMEOUT | STA_DTIMEOUT |
    STA_TXUNDERR | STA_RXOVERR | STA_CMDREND | STA_CMDSENT |
    STA_DATAEND | STA_STBITERR;

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
    pending_data_cmd: Option<u32>,
    pending_sector: u32,
    fifo_data: VecDeque<u8>,
    data_transfer_pending: bool,
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
                // GO_IDLE_STATE (CMD0) always returns CMDSENT (no response)
                self.respcmd = cmd;
                self.sta |= STA_CMDSENT;
                true
            }
            CMD_SEND_IF_COND => {
                // SEND_IF_COND (CMD8) – R7 response (short format)
                self.set_short_response(cmd, SHORT_R7_IF_COND);
                true
            }
            CMD_APP_CMD => {
                // APP_CMD (CMD55) – R1 response (short format)
                self.set_short_response(cmd, SHORT_R1_OK);
                true
            }
            CMD_APP_OP_COND => {
                // APP_OP_COND (ACMD41) – R3 response (short format, no CRC)
                // Also set CMDSENT since APP_OP_COND doesn't require a formal command end
                self.set_short_response(cmd, SHORT_R3_OCR_READY_HC);
                self.sta |= STA_CMDSENT;
                true
            }
            CMD_ALL_SEND_CID => {
                // ALL_SEND_CID (CMD2) – R2 response (long format CID)
                self.set_long_response(cmd, FAKE_CID);
                true
            }
            CMD_SEND_RELATIVE_ADDR => {
                // SEND_RELATIVE_ADDR (CMD3) – R6 response (short format with RCA)
                self.set_short_response(cmd, SHORT_R6_RCA);
                true
            }
            CMD_SEND_CSD => {
                // SEND_CSD (CMD9) – R2 response (long format CSD)
                self.set_long_response(cmd, FAKE_CSD);
                true
            }
            CMD_SEL_DESEL_CARD => {
                // SELECT_CARD (CMD7) – R1/R1b response
                self.set_short_response(cmd, SHORT_R1_OK);
                true
            }
            CMD_SET_BLOCKLEN => {
                // SET_BLOCKLEN (CMD16) – R1 response
                self.set_short_response(cmd, SHORT_R1_OK);
                true
            }
            CMD_SWITCH_OR_SET_BUS_WIDTH => {
                // SWITCH_FUNC or SET_BUS_WIDTH (CMD6) – R1b response
                self.set_short_response(cmd, SHORT_R1_OK);
                true
            }
            CMD_SET_DSR_OR_SWITCH => {
                // SET_DSR (CMD4) – R1 response
                self.set_short_response(cmd, SHORT_R1_OK);
                true
            }
            CMD_SEND_STATUS => {
                // SEND_STATUS (CMD13) – R1 response
                self.set_short_response(cmd, SHORT_R1_TRAN);
                true
            }
            CMD_STOP_TRANSMISSION => {
                // STOP_TRANSMISSION (CMD12) – terminate multi-block transfer context.
                self.set_short_response(cmd, SHORT_R1_OK);
                self.pending_data_cmd = None;
                self.data_transfer_pending = false;
                self.fifo_data.clear();
                self.data_timeout_delay = 0;
                self.sta |= STA_DATAEND;
                true
            }
            CMD_READ_SINGLE_BLOCK | CMD_READ_MULTIPLE_BLOCK | CMD_WRITE_BLOCK | CMD_WRITE_MULTIPLE_BLOCK => {
                // Data transfer commands (CMD17, CMD18, CMD24, CMD25) – R1 response.
                // The actual transfer begins on DCTRL.DTEN; remember command context now.
                self.set_short_response(cmd, SHORT_R1_OK);
                self.pending_data_cmd = Some(cmd);
                self.pending_sector = self.arg;
                true
            }
            _ => false,
        }
    }
    fn maybe_raise_irq(&self, sys: &System) {
        if (self.sta & self.mask) != 0 {
            sys.p.nvic.borrow_mut().set_intr_pending(SDIO_IRQ_NUMBER);
        }
    }

    fn clear_sdio_dma_en_bits(&self, sys: &System) {
        let clear_en_if_sdio = |sys: &System, stream_base: u64| {
            let mut cr = [0u8; 4];
            let mut par = [0u8; 4];
            let mut uc = sys.uc.borrow_mut();
            if uc.mem_read(stream_base + 0x08, &mut par).is_err() {
                return;
            }
            if u32::from_le_bytes(par) != 0x4001_2c80 {
                return;
            }
            if uc.mem_read(stream_base, &mut cr).is_ok() {
                let new_cr = u32::from_le_bytes(cr) & !1;
                let _ = uc.mem_write(stream_base, &new_cr.to_le_bytes());
            }
        };

        for stream in 0..DMA_STREAM_COUNT {
            let stream_base = DMA2_BASE + DMA_STREAM_CR_BASE_OFFSET + stream * DMA_STREAM_STRIDE;
            clear_en_if_sdio(sys, stream_base);
        }
    }

    fn service_sdio_dma_read_stream(&mut self, sys: &System, stream_base: u64) {
        let mut uc = sys.uc.borrow_mut();

        let cr_addr = stream_base;
        let ndtr_addr = stream_base + 0x04;
        let par_addr = stream_base + 0x08;
        let m0ar_addr = stream_base + 0x0C;

        let mut b = [0u8; 4];
        if uc.mem_read(cr_addr, &mut b).is_err() {
            return;
        }
        let cr = u32::from_le_bytes(b);
        if (cr & 1) == 0 {
            return;
        }

        // DIR=00 means peripheral-to-memory.
        let dir = (cr >> 6) & 0b11;
        if dir != 0 {
            return;
        }

        if uc.mem_read(par_addr, &mut b).is_err() {
            return;
        }
        let par = u32::from_le_bytes(b);
        if par != 0x4001_2c80 {
            return;
        }

        if uc.mem_read(ndtr_addr, &mut b).is_err() {
            return;
        }
        let mut count = u32::from_le_bytes(b) & 0xFFFF;
        if count == 0 {
            let _ = uc.mem_write(cr_addr, &(cr & !1).to_le_bytes());
            return;
        }

        if uc.mem_read(m0ar_addr, &mut b).is_err() {
            return;
        }
        let mut maddr = u32::from_le_bytes(b);
        let minc = (cr & (1 << 10)) != 0;
        let msize = match (cr >> 13) & 0b11 {
            0b00 => 1u32,
            0b01 => 2u32,
            0b10 => 4u32,
            _ => 1u32,
        };

        while count > 0 {
            let mut beat = [0u8; 4];
            for i in 0..(msize as usize) {
                beat[i] = self.fifo_data.pop_front().unwrap_or(0);
            }
            let _ = uc.mem_write(maddr as u64, &beat[..msize as usize]);
            if minc {
                maddr = maddr.saturating_add(msize);
            }
            count -= 1;
        }

        let _ = uc.mem_write(ndtr_addr, &0u32.to_le_bytes());
        let _ = uc.mem_write(cr_addr, &(cr & !1).to_le_bytes());
    }

    fn service_sdio_dma_reads(&mut self, sys: &System) {
        for stream in 0..DMA_STREAM_COUNT {
            let stream_base = DMA2_BASE + DMA_STREAM_CR_BASE_OFFSET + stream * DMA_STREAM_STRIDE;
            self.service_sdio_dma_read_stream(sys, stream_base);
        }
    }

    fn synth_sector(&self, sector: u32) -> [u8; 512] {
        let mut s = [0u8; 512];
        match sector {
            0 => {
                // Minimal FAT16 boot sector accepted by FatFs sanity checks.
                s[0] = 0xEB;
                s[1] = 0x3C;
                s[2] = 0x90;
                s[3..11].copy_from_slice(b"MSDOS5.0");
                s[11..13].copy_from_slice(&512u16.to_le_bytes()); // BPB_BytsPerSec
                s[13] = 1; // BPB_SecPerClus
                s[14..16].copy_from_slice(&1u16.to_le_bytes()); // BPB_RsvdSecCnt
                s[16] = 1; // BPB_NumFATs
                s[17..19].copy_from_slice(&16u16.to_le_bytes()); // BPB_RootEntCnt
                // Keep geometry small and self-consistent so FatFs boundary checks pass.
                s[19..21].copy_from_slice(&128u16.to_le_bytes()); // BPB_TotSec16
                s[21] = 0xF8; // BPB_Media
                s[22..24].copy_from_slice(&1u16.to_le_bytes()); // BPB_FATSz16
                s[24..26].copy_from_slice(&32u16.to_le_bytes());
                s[26..28].copy_from_slice(&64u16.to_le_bytes());
                s[36] = 0x80;
                s[38] = 0x29;
                s[39..43].copy_from_slice(&0x1234_5678u32.to_le_bytes());
                s[43..54].copy_from_slice(b"NO NAME    ");
                s[54..62].copy_from_slice(b"FAT16   ");
                s[510] = 0x55;
                s[511] = 0xAA;
            }
            1 => {
                // FAT table: reserved entries + cluster 2 marked end-of-chain.
                s[0..2].copy_from_slice(&0xFFF8u16.to_le_bytes());
                s[2..4].copy_from_slice(&0xFFFFu16.to_le_bytes());
                s[4..6].copy_from_slice(&0xFFFFu16.to_le_bytes());
            }
            _ => {}
        }
        s
    }

    fn prime_fifo_for_data_transfer(&mut self) {
        self.fifo_data.clear();
        let Some(cmd) = self.pending_data_cmd else {
            return;
        };
        if cmd != CMD_READ_SINGLE_BLOCK && cmd != CMD_READ_MULTIPLE_BLOCK {
            return;
        }
        let block_count = core::cmp::max(1, self.dlen / 512);
        for i in 0..block_count {
            let sector = self.pending_sector.saturating_add(i);
            let sec = self.synth_sector(sector);
            self.fifo_data.extend(sec);
        }
    }

    fn dcount(&self) -> u32 {
        if self.pending_data_cmd.is_some() {
            (self.fifo_data.len() as u32).min(self.dlen)
        } else {
            0
        }
    }

    fn fifocnt_words(&self) -> u32 {
        (self.fifo_data.len() as u32) / 4
    }

    fn pending_is_write_transfer(&self) -> bool {
        matches!(self.pending_data_cmd, Some(CMD_WRITE_BLOCK | CMD_WRITE_MULTIPLE_BLOCK))
    }
}

impl Peripheral for Sdio {
    fn step(&mut self, sys: &System) {
        if self.data_transfer_pending {
            self.service_sdio_dma_reads(sys);
            if self.fifo_data.is_empty() {
                self.sta |= STA_DATAEND;
                self.data_timeout_delay = 0;
                self.data_transfer_pending = false;
                self.clear_sdio_dma_en_bits(sys);
                self.maybe_raise_irq(sys);
            }
        }

        if self.data_timeout_delay > 0 {
            self.data_timeout_delay -= 1;
            if self.data_timeout_delay == 0 && self.mask != 0 {
                if self.pending_is_write_transfer() {
                    self.sta |= STA_DCRCFAIL;
                } else {
                    self.sta |= STA_DTIMEOUT;
                }
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
            0x30 => self.dcount(),
            0x34 => self.sta,
            0x3C => self.mask,
            0x48 => self.fifocnt_words(),
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
                        self.maybe_raise_irq(_sys);
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
                            self.maybe_raise_irq(_sys);
                        } else {
                            // Unknown command with expected response: model a CRC-fail
                            // response path before eventual ejection timeout behavior.
                            self.sta |= STA_CCRCFAIL;
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
                            self.maybe_raise_irq(_sys);
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
                    // Malformed write transfer setup: signal data CRC failure.
                    if self.pending_is_write_transfer() && (self.dlen == 0 || (self.dlen % 512) != 0) {
                        self.sta |= STA_DCRCFAIL;
                        self.data_transfer_pending = false;
                        self.data_timeout_delay = 0;
                        self.maybe_raise_irq(_sys);
                        return;
                    }

                    // Data transfer enabled: prepare synthetic payload and complete when DMA consumes it.
                    self.prime_fifo_for_data_transfer();
                    self.service_sdio_dma_reads(_sys);
                    if self.fifo_data.is_empty() {
                        self.sta |= STA_DATAEND;
                        self.data_timeout_delay = 0;
                        self.data_transfer_pending = false;
                        self.clear_sdio_dma_en_bits(_sys);
                        self.maybe_raise_irq(_sys);
                    } else {
                        self.data_transfer_pending = true;
                        // Keep timeout path active while waiting for DMA stream enable.
                        self.data_timeout_delay = DATA_TIMEOUT_DELAY_STEPS;
                    }
                } else {
                    self.data_transfer_pending = false;
                    self.data_timeout_delay = 0;
                }
            }
            0x38 => {
                // ICR: clear indicated status bits
                self.sta &= !(value & STA_CLEARABLE_MASK);
            }
            0x3C => {
                self.mask = value;
                // If status bits were already set before the firmware unmasked IRQs,
                // raise pending now so waiters are not stranded.
                self.maybe_raise_irq(_sys);
            }
            _ => {}
        }
    }

    fn read_dma(&mut self, sys: &System, offset: u32, size: usize) -> VecDeque<u8> {
        if offset != 0x80 {
            return super::Peripheral::read_dma(self, sys, offset, size);
        }

        let mut out = VecDeque::with_capacity(size);
        for _ in 0..size {
            out.push_back(self.fifo_data.pop_front().unwrap_or(0));
        }
        out
    }
}
