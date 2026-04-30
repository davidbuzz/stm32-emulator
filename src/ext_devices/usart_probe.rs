// SPDX-License-Identifier: GPL-3.0-or-later

use anyhow::Result;
use serde::Deserialize;

use crate::system::System;

use super::ExtDevice;

#[derive(Debug, Deserialize, Default)]
pub struct UsartProbeConfig {
    pub peripheral: String,
}

#[derive(Default)]
pub struct UsartProbe {
    pub config: UsartProbeConfig,
    name: String,
    rx: Vec<u8>,
}

impl UsartProbe {
    pub fn new(config: UsartProbeConfig) -> Result<Self> {
        Ok(Self { config, ..Self::default() })
    }

    fn append_bytes(&mut self, mut bytes: &[u8]) {
        while !bytes.is_empty() {
            let room = 256usize.saturating_sub(self.rx.len());
            let take = room.min(bytes.len());
            self.rx.extend_from_slice(&bytes[..take]);
            bytes = &bytes[take..];

            if self.rx.len() >= 256 {
                self.flush_rx();
            }
        }
    }

    fn flush_rx(&mut self) {
        let line = String::from_utf8_lossy(&self.rx);
        let line = line.trim();
        if !line.is_empty() {
            info!("{} '{}'", self.name, line);
        }
        self.rx.clear();
    }

    fn push_byte(&mut self, v: u8) {
        if v == b'\n' || v == b'\r' {
            self.flush_rx();
            return;
        }

        self.rx.push(v);
        if self.rx.len() >= 256 {
            self.flush_rx();
        }
    }
}

impl ExtDevice<(), u8> for UsartProbe {
    fn connect_peripheral(&mut self, peri_name: &str) -> String {
        self.name = format!("{} usart-probe", peri_name);
        self.name.clone()
    }

    fn read(&mut self, _sys: &System, _addr: ()) -> u8 {
        0
    }

    fn write(&mut self, _sys: &System, _addr: (), v: u8) {
        self.push_byte(v);
    }

    fn read_batch(&mut self, _sys: &System, _addr: (), len: usize) -> Vec<u8> {
        vec![0; len]
    }

    fn write_batch(&mut self, _sys: &System, _addr: (), values: &[u8]) {
        let mut start = 0usize;

        for (idx, value) in values.iter().enumerate() {
            if *value == b'\n' || *value == b'\r' {
                if start < idx {
                    self.append_bytes(&values[start..idx]);
                }
                self.flush_rx();
                start = idx + 1;
            }
        }

        if start < values.len() {
            self.append_bytes(&values[start..]);
        }
    }
}
