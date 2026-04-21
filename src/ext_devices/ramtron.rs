// SPDX-License-Identifier: GPL-3.0-or-later

// RAMTRON FM25V02: 32KB SPI FRAM used on CubeBlack for parameter storage.
// Protocol: RDID (0x9F) returns 9-byte Cypress ID; READ (0x03) + 2-byte addr;
// WRITE (0x02) + 2-byte addr; WREN (0x06) write-enable latch; WRDI (0x04).
// CS deassert (GPIO callback) resets state machine to Idle so transactions are isolated.
// Reference: AP_RAMTRON.cpp, SIM_RAMTRON_FM25V02.h, FM25V02 datasheet.

use std::collections::VecDeque;
use anyhow::Result;
use serde::Deserialize;

use crate::system::System;
use super::ExtDevice;

const RAMTRON_SIZE: usize = 32 * 1024;

// Cypress FM25V02 RDID response: 6 manufacturer bytes + memory byte + id1 + id2
const RDID_RESPONSE: [u8; 9] = [0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0xC2, 0x22, 0x00];

#[derive(Debug, Deserialize, Default)]
pub struct RamtronConfig {
    pub peripheral: String,
    pub cs_pin: String,
}

enum State {
    Idle,
    CollectingArgs { cmd: Cmd, args: Vec<u8> },
    ReadData { addr: usize },
    WriteData { addr: usize },
    SendingReply(VecDeque<u8>),
}

#[derive(Clone, Copy)]
enum Cmd {
    Read,
    Write,
}

pub struct Ramtron {
    pub config: RamtronConfig,
    name: String,
    storage: Vec<u8>,
    state: State,
    write_enabled: bool,
}

impl Ramtron {
    pub fn new(config: RamtronConfig) -> Result<Self> {
        Ok(Self {
            config,
            name: String::new(),
            storage: vec![0xFF; RAMTRON_SIZE],
            state: State::Idle,
            write_enabled: false,
        })
    }

    pub fn reset_state(&mut self) {
        self.state = State::Idle;
    }
}

impl ExtDevice<(), u8> for Ramtron {
    fn connect_peripheral(&mut self, peri_name: &str) -> String {
        self.name = format!("{} ramtron", peri_name);
        self.name.clone()
    }

    fn read(&mut self, _sys: &System, _addr: ()) -> u8 {
        match &mut self.state {
            State::SendingReply(data) => data.pop_front().unwrap_or(0xFF),
            State::ReadData { addr } => {
                let v = self.storage[*addr % RAMTRON_SIZE];
                *addr = (*addr + 1) % RAMTRON_SIZE;
                v
            }
            _ => 0xFF,
        }
    }

    fn write(&mut self, _sys: &System, _addr: (), v: u8) {
        let prev_state = std::mem::replace(&mut self.state, State::Idle);
        self.state = match prev_state {
            State::CollectingArgs { cmd, mut args } => {
                args.push(v);
                self.collect(cmd, args)
            }
            State::WriteData { addr } => {
                if self.write_enabled {
                    self.storage[addr % RAMTRON_SIZE] = v;
                }
                State::WriteData { addr: (addr + 1) % RAMTRON_SIZE }
            }
            _ => self.new_command(v),
        };
    }
}

impl Ramtron {
    fn new_command(&mut self, v: u8) -> State {
        match v {
            0x9F => {
                debug!("{} RDID", self.name);
                State::SendingReply(RDID_RESPONSE.iter().copied().collect())
            }
            0x05 => {
                let status = if self.write_enabled { 0x02 } else { 0x00 };
                State::SendingReply(vec![status].into())
            }
            0x06 => {
                self.write_enabled = true;
                debug!("{} WREN", self.name);
                State::Idle
            }
            0x04 => {
                self.write_enabled = false;
                State::Idle
            }
            0x03 => State::CollectingArgs { cmd: Cmd::Read, args: vec![] },
            0x02 => State::CollectingArgs { cmd: Cmd::Write, args: vec![] },
            0xFF | 0x00 => State::Idle,
            _ => {
                debug!("{} unknown cmd=0x{:02x}", self.name, v);
                State::Idle
            }
        }
    }

    fn collect(&mut self, cmd: Cmd, args: Vec<u8>) -> State {
        match cmd {
            Cmd::Read if args.len() >= 2 => {
                let addr = ((args[0] as usize) << 8) | (args[1] as usize);
                debug!("{} READ addr=0x{:04x}", self.name, addr);
                State::ReadData { addr }
            }
            Cmd::Write if args.len() >= 2 => {
                let addr = ((args[0] as usize) << 8) | (args[1] as usize);
                debug!("{} WRITE addr=0x{:04x}", self.name, addr);
                // Any data bytes already collected go directly to storage
                let mut write_addr = addr;
                for &b in &args[2..] {
                    if self.write_enabled {
                        self.storage[write_addr % RAMTRON_SIZE] = b;
                    }
                    write_addr = (write_addr + 1) % RAMTRON_SIZE;
                }
                State::WriteData { addr: write_addr }
            }
            _ => State::CollectingArgs { cmd, args },
        }
    }
}
