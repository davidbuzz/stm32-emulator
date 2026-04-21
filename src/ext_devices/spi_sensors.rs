// SPDX-License-Identifier: GPL-3.0-or-later

// SPI sensor stubs for CubeBlack board validation.
//
// ArduPilot's HAL_VALIDATE_BOARD macro checks 6 SPI sensors before any driver init:
//   ms5611, ms5611_ext   — barometer (PROM CRC4 check)
//   mpu9250, mpu9250_ext — IMU (WHO_AM_I reg 0x75 → 0x71)
//   lsm9ds0_ext_g        — gyro (WHO_AM_I reg 0x0F → 0xD4)
//   lsm9ds0_ext_am       — accel/mag (WHO_AM_I reg 0x0F → 0x49)
//
// If any check fails, board_autodetect() enters an infinite error loop and never boots.
//
// SpiMux routes SPI traffic to the correct sub-device based on which CS pin is asserted
// (GPIO low). Each device's state is reset on CS deassert (GPIO high transition).
//
// Protocol conventions:
//   WhoAmI devices: SPI read = [reg | 0x80, 0x00] → [X, value]
//   MS5611: reset = [0x1E] (no reply); PROM read = [0xAn, 0x00, 0x00] → [X, HIGH, LOW]
//           PROM[7] low-nibble must be valid CRC4 of PROM[0..7]
//
// Reference: AP_BoardConfig/board_drivers.cpp, AP_Baro_MS5611.cpp,
//            AP_InertialSensor_Invensense.cpp, AP_InertialSensor_LSM9DS0.cpp

use std::collections::VecDeque;
use serde::Deserialize;
use crate::system::System;
use super::ExtDevice;

// ──────────────────────────────────────────────
// Config types (deserialized from YAML)
// ──────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct SpiSensorConfig {
    pub cs_pin: String,
    #[serde(flatten)]
    pub kind: SpiSensorKindConfig,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum SpiSensorKindConfig {
    WhoAmI {
        name: String,
        reg: u8,
        val: u8,
    },
    Ms5611 {
        name: String,
    },
}

#[derive(Debug, Deserialize)]
pub struct SpiMuxConfig {
    pub peripheral: String,
    pub sensors: Vec<SpiSensorConfig>,
}

// ──────────────────────────────────────────────
// Runtime types
// ──────────────────────────────────────────────

enum SpiSensorDevice {
    WhoAmI(WhoAmIDevice),
    Ms5611(Ms5611Device),
}

pub struct SpiSensorSlot {
    pub cs_pin: String,
    device: SpiSensorDevice,
    pub active: bool,
}

pub struct SpiMux {
    pub peripheral: String,
    pub slots: Vec<SpiSensorSlot>,
}

// ──────────────────────────────────────────────
// SpiMux construction and CS routing
// ──────────────────────────────────────────────

impl SpiMux {
    pub fn new(config: SpiMuxConfig) -> Self {
        let peripheral = config.peripheral.clone();
        let slots = config.sensors.into_iter().map(|s| {
            let device = match s.kind {
                SpiSensorKindConfig::WhoAmI { name, reg, val } => {
                    SpiSensorDevice::WhoAmI(WhoAmIDevice::new(name, reg, val))
                }
                SpiSensorKindConfig::Ms5611 { name } => {
                    SpiSensorDevice::Ms5611(Ms5611Device::new(name))
                }
            };
            SpiSensorSlot { cs_pin: s.cs_pin, device, active: false }
        }).collect();
        Self { peripheral, slots }
    }

    pub fn cs_pins(&self) -> Vec<String> {
        self.slots.iter().map(|s| s.cs_pin.clone()).collect()
    }

    /// Called by GPIO write callback for each CS pin.
    /// `value`: true = CS high (deasserted), false = CS low (asserted).
    pub fn on_cs_change(&mut self, cs_pin: &str, value: bool) {
        for slot in &mut self.slots {
            if slot.cs_pin == cs_pin {
                if slot.active && value {
                    // Deassert: reset device state for clean next transaction
                    slot.device.reset();
                }
                slot.active = !value; // CS low → active
            }
        }
    }

    fn active_device(&mut self) -> Option<&mut SpiSensorDevice> {
        self.slots.iter_mut()
            .find(|s| s.active)
            .map(|s| &mut s.device)
    }
}

impl ExtDevice<(), u8> for SpiMux {
    fn connect_peripheral(&mut self, peri_name: &str) -> String {
        // Returns name used in logs; we just use the peripheral name
        format!("{} spi-mux", peri_name)
    }

    fn read(&mut self, _sys: &System, _addr: ()) -> u8 {
        self.active_device()
            .map(|d| d.read())
            .unwrap_or(0xFF)
    }

    fn write(&mut self, _sys: &System, _addr: (), v: u8) {
        if let Some(d) = self.active_device() {
            d.write(v);
        }
    }
}

// ──────────────────────────────────────────────
// SpiSensorDevice dispatch
// ──────────────────────────────────────────────

impl SpiSensorDevice {
    fn read(&mut self) -> u8 {
        match self {
            SpiSensorDevice::WhoAmI(d) => d.read(),
            SpiSensorDevice::Ms5611(d) => d.read(),
        }
    }

    fn write(&mut self, v: u8) {
        match self {
            SpiSensorDevice::WhoAmI(d) => d.write(v),
            SpiSensorDevice::Ms5611(d) => d.write(v),
        }
    }

    fn reset(&mut self) {
        match self {
            SpiSensorDevice::WhoAmI(d) => d.reset(),
            SpiSensorDevice::Ms5611(d) => d.reset(),
        }
    }
}

// ──────────────────────────────────────────────
// WhoAmI device
// Passes spi_check_register(name, reg, val) by responding to [reg|0x80, 0x00] → [X, val]
// ──────────────────────────────────────────────

struct WhoAmIDevice {
    name: String,
    who_am_i_reg: u8,
    who_am_i_val: u8,
    reply: VecDeque<u8>,
}

impl WhoAmIDevice {
    fn new(name: String, reg: u8, val: u8) -> Self {
        Self { name, who_am_i_reg: reg, who_am_i_val: val, reply: VecDeque::new() }
    }

    fn read(&mut self) -> u8 {
        self.reply.pop_front().unwrap_or(0xFF)
    }

    fn write(&mut self, v: u8) {
        if !self.reply.is_empty() {
            // Dummy MOSI bytes during reply phase — ignore
            return;
        }
        if v == (self.who_am_i_reg | 0x80) {
            debug!("{} WHO_AM_I read → 0x{:02x}", self.name, self.who_am_i_val);
            self.reply.push_back(self.who_am_i_val);
        }
        // All other register reads/writes: return 0xFF (safe default)
    }

    fn reset(&mut self) {
        self.reply.clear();
    }
}

// ──────────────────────────────────────────────
// MS5611 barometer device
// Passes check_ms5611(name): reset (0x1E) + 8× PROM word reads (0xA0..0xAE) with CRC4 valid
//
// Transaction format per PROM word:
//   DMA sends [0xA0+(word*2), 0x00, 0x00]
//   Device queues [PROM_HIGH, PROM_LOW] after command byte
//   RX buffer: [0xFF, PROM_HIGH, PROM_LOW]  ← firmware uses indices [1],[2]
// ──────────────────────────────────────────────

const MS5611_CMD_RESET: u8 = 0x1E;
const MS5611_CMD_PROM_READ_BASE: u8 = 0xA0;

struct Ms5611Device {
    name: String,
    prom: [u16; 8],
    reply: VecDeque<u8>,
}

impl Ms5611Device {
    fn new(name: String) -> Self {
        let prom = Self::make_valid_prom();
        Self { name, prom, reply: VecDeque::new() }
    }

    /// Build PROM[0..7] with typical calibration constants and compute CRC4 into PROM[7].
    fn make_valid_prom() -> [u16; 8] {
        let mut prom: [u16; 8] = [
            0x0000, // PROM[0]: reserved / manufacturer bits (masked to 0x0FFF for CRC)
            0x77E0, // C1: SENS_T1
            0x7B00, // C2: OFF_T1
            0x6D00, // C3: TCS
            0x6300, // C4: TCO
            0x5E50, // C5: T_REF
            0x5A00, // C6: TEMPSENS
            0x0000, // C7: will hold the CRC4
        ];
        let crc = Self::compute_crc4(&prom);
        prom[7] = crc as u16; // store in low nibble; top nibble stays 0
        prom
    }

    /// CRC4 per MS5611 datasheet (application note AN520).
    /// Operates on an 8-word PROM with PROM[7] zeroed and PROM[0] top nibble masked.
    fn compute_crc4(prom_in: &[u16; 8]) -> u8 {
        let mut prom = *prom_in;
        prom[7] = 0;
        prom[0] &= 0x0FFF;

        let mut n_rem: u32 = 0;
        for cnt in 0..16usize {
            let byte = if cnt % 2 == 1 {
                (prom[cnt >> 1] & 0x00FF) as u32
            } else {
                ((prom[cnt >> 1] >> 8) & 0xFF) as u32
            };
            n_rem ^= byte;
            for _ in 0..8 {
                if n_rem & 0x8000 != 0 {
                    n_rem = (n_rem << 1) ^ 0x3000;
                } else {
                    n_rem <<= 1;
                }
            }
        }
        ((n_rem >> 12) & 0x0F) as u8
    }

    fn read(&mut self) -> u8 {
        self.reply.pop_front().unwrap_or(0xFF)
    }

    fn write(&mut self, v: u8) {
        if !self.reply.is_empty() {
            // Dummy MOSI bytes during reply — ignore
            return;
        }

        if v == MS5611_CMD_RESET {
            debug!("{} RESET", self.name);
            return;
        }

        // PROM read commands: 0xA0, 0xA2, 0xA4, 0xA6, 0xA8, 0xAA, 0xAC, 0xAE
        // Note: 0xA0|0x80 = 0xA0, so ArduPilot's read_registers() leaves these unchanged.
        if v >= MS5611_CMD_PROM_READ_BASE && v <= 0xAE {
            let offset = v - MS5611_CMD_PROM_READ_BASE;
            if offset % 2 == 0 {
                let word_idx = (offset / 2) as usize;
                let word = self.prom[word_idx];
                debug!("{} PROM[{}] = 0x{:04x}", self.name, word_idx, word);
                self.reply.push_back((word >> 8) as u8);
                self.reply.push_back((word & 0xFF) as u8);
                return;
            }
        }

        // Convert D1/D2 (0x40..0x4A, 0x50..0x5A) and ADC read (0x00):
        // Not needed for board validation; silently ignore.
    }

    fn reset(&mut self) {
        self.reply.clear();
    }
}
