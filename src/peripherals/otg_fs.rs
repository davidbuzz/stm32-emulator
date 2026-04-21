// SPDX-License-Identifier: GPL-3.0-or-later

// STM32 name: USB OTG FS core blocks (OTG_FS_GLOBAL / OTG_FS_DEVICE / OTG_FS_PWRCLK).
// STM32F427 bases: GLOBAL=0x50000000, HOST=0x50000400, DEVICE=0x50000800, PWRCLK=0x50000E00.
// Key registers: GUSBCFG, GRSTCTL, GINTSTS/GINTMSK, GCCFG, DCFG, DCTL, DAINTMSK, PCGCCTL.
// Key function: USB full-speed core used on CubeBlack for ChibiOS USB and USB CDC serial.
// Critical for this emulator: firmware was previously stuck polling GRSTCTL/GCCFG with generic zero reads.
// Current model is deliberately minimal and targets reset/idle and early device bring-up semantics.
// Still incomplete: endpoint state machines, FIFOs, USB interrupt detail, and CDC data bridging.
// Datasheet/reference anchor: STM32F4 RM USB OTG FS chapters and the STM32F427 SVD.

use crate::system::System;

use super::Peripheral;

const GRSTCTL_CSRST: u32 = 1 << 0;
const GRSTCTL_RXFFLSH: u32 = 1 << 4;
const GRSTCTL_TXFFLSH: u32 = 1 << 5;
const GRSTCTL_AHBIDL: u32 = 1 << 31;

const GINTSTS_RESET: u32 = 0x0400_0020;
const GNPTXSTS_RESET: u32 = 0x0008_0200;
const CID_RESET: u32 = 0x0000_1000;

#[derive(Default)]
pub struct OtgFs {
    kind: OtgKind,
    gotgctl: u32,
    gotgint: u32,
    gahbcfg: u32,
    gusbcfg: u32,
    grstctl: u32,
    gintsts: u32,
    gintmsk: u32,
    grxstsr: u32,
    grxfsiz: u32,
    dieptxf0: u32,
    gnptxsts: u32,
    gccfg: u32,
    cid: u32,
    hptxfsiz: u32,
    dieptxf: [u32; 3],
    dcfg: u32,
    dctl: u32,
    dsts: u32,
    diepmsk: u32,
    doepmsk: u32,
    daint: u32,
    daintmsk: u32,
    pcgcctl: u32,
    pending_reset_clear: bool,
}

#[derive(Default)]
enum OtgKind {
    #[default]
    Global,
    Device,
    PwrClk,
}

impl OtgFs {
    pub fn new(name: &str) -> Option<Box<dyn Peripheral>> {
        let kind = match name {
            "OTG_FS_GLOBAL" => OtgKind::Global,
            "OTG_FS_DEVICE" => OtgKind::Device,
            "OTG_FS_PWRCLK" => OtgKind::PwrClk,
            _ => return None,
        };

        Some(Box::new(Self {
            kind,
            grstctl: GRSTCTL_AHBIDL,
            gintsts: GINTSTS_RESET,
            grxfsiz: 0x0000_0200,
            dieptxf0: 0x0000_0200,
            gnptxsts: GNPTXSTS_RESET,
            cid: CID_RESET,
            hptxfsiz: 0x0200_0600,
            dieptxf: [0x0200_0400; 3],
            ..Default::default()
        }))
    }

    fn read_global(&mut self, offset: u32) -> u32 {
        if self.pending_reset_clear {
            self.grstctl &= !(GRSTCTL_CSRST | GRSTCTL_RXFFLSH | GRSTCTL_TXFFLSH);
            self.pending_reset_clear = false;
        }

        match offset {
            0x0000 => self.gotgctl,
            0x0004 => self.gotgint,
            0x0008 => self.gahbcfg,
            0x000c => self.gusbcfg,
            0x0010 => self.grstctl | GRSTCTL_AHBIDL,
            0x0014 => self.gintsts,
            0x0018 => self.gintmsk,
            0x001c | 0x0020 => self.grxstsr,
            0x0024 => self.grxfsiz,
            0x0028 => self.dieptxf0,
            0x002c => self.gnptxsts,
            0x0038 => self.gccfg,
            0x003c => self.cid,
            0x0100 => self.hptxfsiz,
            0x0104 => self.dieptxf[0],
            0x0108 => self.dieptxf[1],
            0x010c => self.dieptxf[2],
            _ => 0,
        }
    }

    fn write_global(&mut self, offset: u32, value: u32) {
        match offset {
            0x0000 => self.gotgctl = value,
            0x0004 => self.gotgint &= !value,
            0x0008 => self.gahbcfg = value,
            0x000c => self.gusbcfg = value,
            0x0010 => {
                self.grstctl = (value & !GRSTCTL_AHBIDL) | GRSTCTL_AHBIDL;
                if value & (GRSTCTL_CSRST | GRSTCTL_RXFFLSH | GRSTCTL_TXFFLSH) != 0 {
                    self.pending_reset_clear = true;
                }
            }
            0x0014 => self.gintsts &= !value,
            0x0018 => self.gintmsk = value,
            0x0024 => self.grxfsiz = value,
            0x0028 => self.dieptxf0 = value,
            0x0038 => self.gccfg = value,
            0x003c => self.cid = value,
            0x0100 => self.hptxfsiz = value,
            0x0104 => self.dieptxf[0] = value,
            0x0108 => self.dieptxf[1] = value,
            0x010c => self.dieptxf[2] = value,
            _ => {}
        }
    }

    fn read_device(&self, offset: u32) -> u32 {
        match offset {
            0x0000 => self.dcfg,
            0x0004 => self.dctl,
            0x0008 => self.dsts,
            0x0010 => self.diepmsk,
            0x0014 => self.doepmsk,
            0x0018 => self.daint,
            0x001c => self.daintmsk,
            _ => 0,
        }
    }

    fn write_device(&mut self, offset: u32, value: u32) {
        match offset {
            0x0000 => self.dcfg = value,
            0x0004 => self.dctl = value,
            0x0008 => self.dsts = value,
            0x0010 => self.diepmsk = value,
            0x0014 => self.doepmsk = value,
            0x0018 => self.daint &= !value,
            0x001c => self.daintmsk = value,
            _ => {}
        }
    }
}

impl Peripheral for OtgFs {
    fn read(&mut self, _sys: &System, offset: u32) -> u32 {
        match self.kind {
            OtgKind::Global => self.read_global(offset),
            OtgKind::Device => self.read_device(offset),
            OtgKind::PwrClk => match offset {
                0x0000 => self.pcgcctl,
                _ => 0,
            },
        }
    }

    fn write(&mut self, _sys: &System, offset: u32, value: u32) {
        match self.kind {
            OtgKind::Global => self.write_global(offset, value),
            OtgKind::Device => self.write_device(offset, value),
            OtgKind::PwrClk => {
                if offset == 0x0000 {
                    self.pcgcctl = value;
                }
            }
        }
    }
}