// SPDX-License-Identifier: GPL-3.0-or-later

// STM32 name: USB OTG FS core blocks (OTG_FS_GLOBAL / OTG_FS_DEVICE / OTG_FS_PWRCLK).
// STM32F427 bases: GLOBAL=0x50000000, HOST=0x50000400, DEVICE=0x50000800, PWRCLK=0x50000E00.
// Key registers: GUSBCFG, GRSTCTL, GINTSTS/GINTMSK, GCCFG, DCFG, DCTL, DAINTMSK, PCGCCTL.
// Key function: USB full-speed core used on CubeBlack for ChibiOS USB and USB CDC serial.
// Critical for this emulator: firmware was previously stuck polling GRSTCTL/GCCFG with generic zero reads.
// Current model is deliberately minimal and targets reset/idle and early device bring-up semantics.
// Still incomplete: endpoint state machines, FIFOs, USB interrupt detail, and CDC data bridging.
// Datasheet/reference anchor: STM32F4 RM USB OTG FS chapters and the STM32F427 SVD.

use std::cell::RefCell;
use std::io::{self, Write};
use std::rc::Rc;

use crate::system::System;

use super::Peripheral;

macro_rules! otg_debug {
    ($($arg:tt)*) => { debug!("OTG_FS {}", format_args!($($arg)*)) };
}

const OTG_FS_IRQ: i32 = 67;
const GAHBCFG_GINT: u32 = 1 << 0;

const GRSTCTL_CSRST: u32 = 1 << 0;
const GRSTCTL_RXFFLSH: u32 = 1 << 4;
const GRSTCTL_TXFFLSH: u32 = 1 << 5;
const GRSTCTL_AHBIDL: u32 = 1 << 31;

const GINTSTS_RESET: u32 = 0x0400_0020;
const GINTSTS_SOF: u32 = 1 << 3;
const GINTSTS_USBRST: u32 = 1 << 12;
const GINTSTS_ENUMDNE: u32 = 1 << 13;
const GINTSTS_IEPINT: u32 = 1 << 18;
const GINTSTS_OEPINT: u32 = 1 << 19;
const GINTSTS_SRQINT: u32 = 1 << 30;
const GNPTXSTS_RESET: u32 = 0x0008_0200;
const CID_RESET: u32 = 0x0000_1000;
const OTG_STARTUP_EVENT_DELAY: u32 = 2048;
const OTG_SOF_PERIOD: u32 = 168_000; // ~1ms at 168MHz
const CDC_PERIODIC_FLUSH_INTERVAL: u64 = 200_000;
const EP_COUNT: usize = 4;
const DOEPINT_XFRC: u32 = 1 << 0;
const DOEPINT_STUP: u32 = 1 << 3;
const DIEPINT_XFRC: u32 = 1 << 0;
const DIEPINT_TXFE: u32 = 1 << 7;
const DOEPCTL_EPENA: u32 = 1 << 31;
const DOEPCTL_CNAK: u32 = 1 << 26;
const DOEPCTL_SNAK: u32 = 1 << 27;
const DOEPCTL_USBAEP: u32 = 1 << 15;
const DOEPCTL_NAKSTS: u32 = 1 << 17;
const DIEPCTL_EPENA: u32 = 1 << 31;
const DIEPCTL_CNAK: u32 = 1 << 26;
const DIEPCTL_SNAK: u32 = 1 << 27;
const DIEPCTL_USBAEP: u32 = 1 << 15;
const DIEPCTL_NAKSTS: u32 = 1 << 17;
const DTXFSTS_RESET_WORDS: u32 = 0x80;
const GINTSTS_RXFLVL: u32 = 1 << 4;
// GRXSTSP PKTSTS field: bits [20:17].  6=setup data received, 4=setup complete.
const GRXSTSP_PKTSTS_OUT_DATA: u32 = 2 << 17;
const GRXSTSP_PKTSTS_OUT_COMPL: u32 = 3 << 17;
const GRXSTSP_PKTSTS_SETUP_DATA: u32 = 6 << 17;
const GRXSTSP_PKTSTS_SETUP_COMPL: u32 = 4 << 17;
const GRXSTSP_BCNT_8: u32 = 8 << 4; // BCNT=8 in bits[14:4]
const GRXSTSP_BCNT_7: u32 = 7 << 4; // BCNT=7 in bits[14:4]

// Minimal USB enumeration setup packets (2xu32 little-endian bytes).
// SET_ADDRESS 1: 00 05 01 00  00 00 00 00
const SETUP_SET_ADDRESS: [u32; 2] = [0x0001_0500, 0x0000_0000];
// SET_CONFIGURATION 1: 00 09 01 00  00 00 00 00
const SETUP_SET_CONFIG: [u32; 2] = [0x0001_0900, 0x0000_0000];
// CDC ACM SET_LINE_CODING (7-byte payload): 21 20 00 00  00 00 07 00
const SETUP_CDC_SET_LINE_CODING: [u32; 2] = [0x0000_2021, 0x0007_0000];
// CDC ACM GET_LINE_CODING: A1 21 00 00  00 00 07 00
const SETUP_CDC_GET_LINE_CODING: [u32; 2] = [0x0000_21a1, 0x0007_0000];
const CDC_LINE_CODING_115200_8N1: [u32; 2] = [0x0001_c200, 0x0008_0000];
// CDC ACM SET_CONTROL_LINE_STATE (DTR|RTS): 21 22 03 00  00 00 00 00
const SETUP_CDC_SET_CONTROL_LINE_STATE: [u32; 2] = [0x0003_2221, 0x0000_0000];

/// State of the EP0 RX FIFO / GRXSTSP pop sequence for a synthetic setup packet.
#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
enum Ep0RxState {
    #[default]
    Idle,
    RxFlvlStatusPending,   // will return PKTSTS=6 on next GRXSTSP pop
    RxFlvlFifoPending,     // PKTSTS=6 returned; waiting for 2 FIFO word reads
    RxFlvlCompletePending, // 2 words consumed; will return PKTSTS=4 on next pop
    StupPending,           // PKTSTS=4 returned; will fire DOEPINT0.STUP
    OutDataStatusPending,  // will return PKTSTS=2 BCNT=7 on next GRXSTSP pop
    OutDataFifoPending,    // PKTSTS=2 returned; waiting for 2 FIFO word reads
    OutCompletePending,    // 2 words consumed; will return PKTSTS=3 on next pop
    OutXfrcPending,        // PKTSTS=3 returned; will fire DOEPINT0.XFRC
}

/// Which step of the synthetic USB enumeration sequence are we on.
#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
enum UsbEnumStage {
    #[default]
    Idle,
    DeliverSetAddress,    // send SET_ADDRESS 1 setup packet
    DeliverSetConfig,     // send SET_CONFIGURATION 1 setup packet
    DeliverGetLineCoding, // send CDC ACM GET_LINE_CODING
    DeliverSetLineCoding, // send CDC ACM SET_LINE_CODING
    DeliverSetControlLineState, // send CDC ACM SET_CONTROL_LINE_STATE (DTR/RTS)
    Configured,           // enumeration done
}

#[derive(Default)]
struct OtgFsState {
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
    diepempmsk: u32,
    daint: u32,
    daintmsk: u32,
    diepctl: [u32; EP_COUNT],
    doepctl: [u32; EP_COUNT],
    diepint: [u32; EP_COUNT],
    doepint: [u32; EP_COUNT],
    dieptsiz: [u32; EP_COUNT],
    doeptsiz: [u32; EP_COUNT],
    dtxfsts: [u32; EP_COUNT],
    pcgcctl: u32,
    pending_reset_clear: bool,
    startup_stage: StartupStage,
    event_delay: u32,
    sof_delay: u32,
    irq_latched: bool,
    ep0_rx_state: Ep0RxState,
    ep0_pending_setup: [u32; 2],
    ep0_pending_out_data: [u32; 2],
    ep0_fifo_read_count: u8,
    ep0_setup_inflight: bool,
    ep0_in_transfer_pending: bool,
    ep_in_transfer_pending: [bool; EP_COUNT], // for EP1..3 (CDC bulk/interrupt)
    ep_txfe_was_fired: [bool; EP_COUNT],      // tracks whether TXFE fired for each pending EP transfer
    enum_stage: UsbEnumStage,
    cdc_line_buf: Vec<u8>,
    cdc_last_periodic_flush_clk: u64,
    /// RX FIFO level in words: tracks pending data for enumeration and generic transfers.
    rx_fifo_level: u32,
}

pub struct OtgFs {
    kind: OtgKind,
    shared: Rc<RefCell<OtgFsState>>,
}

thread_local! {
    static OTG_FS_SHARED: Rc<RefCell<OtgFsState>> = Rc::new(RefCell::new(OtgFsState::default()));
}

#[derive(Default)]
enum OtgKind {
    #[default]
    Global,
    Device,
    PwrClk,
}

#[derive(Clone, Copy, Default, PartialEq, Eq)]
enum StartupStage {
    #[default]
    Idle,
    UsbReset,
    EnumDone,
    Running,
}

impl OtgFs {
    pub fn new(name: &str) -> Option<Box<dyn Peripheral>> {
        let kind = match name {
            "OTG_FS_GLOBAL" => OtgKind::Global,
            "OTG_FS_DEVICE" => OtgKind::Device,
            "OTG_FS_PWRCLK" => OtgKind::PwrClk,
            _ => return None,
        };

        let shared = OTG_FS_SHARED.with(|shared| shared.clone());

        if matches!(kind, OtgKind::Global) {
            *shared.borrow_mut() = OtgFsState {
                grstctl: GRSTCTL_AHBIDL,
                gintsts: GINTSTS_RESET,
                grxfsiz: 0x0000_0200,
                dieptxf0: 0x0000_0200,
                gnptxsts: GNPTXSTS_RESET,
                cid: CID_RESET,
                hptxfsiz: 0x0200_0600,
                dieptxf: [0x0200_0400; 3],
                dtxfsts: [DTXFSTS_RESET_WORDS; EP_COUNT],
                event_delay: OTG_STARTUP_EVENT_DELAY,
                sof_delay: OTG_SOF_PERIOD,
                rx_fifo_level: 0,
                ..Default::default()
            };
        }

        Some(Box::new(Self { kind, shared }))
    }
}

impl OtgFsState {
    fn tx_fifo_cfg(&self, ep: usize) -> (u32, u32) {
        match ep {
            0 => {
                let start = self.dieptxf0 & 0xFFFF;
                let depth = (self.dieptxf0 >> 16) & 0xFFFF;
                if depth != 0 {
                    (start, depth)
                } else {
                    (0, DTXFSTS_RESET_WORDS)
                }
            }
            1..=3 => {
                let start = self.dieptxf[ep - 1] & 0xFFFF;
                let depth = (self.dieptxf[ep - 1] >> 16) & 0xFFFF;
                if depth != 0 {
                    (start, depth)
                } else {
                    (0, DTXFSTS_RESET_WORDS)
                }
            }
            _ => (0, DTXFSTS_RESET_WORDS),
        }
    }

    fn tx_fifo_overlap_invalid(&self, ep: usize) -> bool {
        let (start, depth) = self.tx_fifo_cfg(ep);
        if depth == 0 {
            return true;
        }
        let end = start.saturating_add(depth);
        for other in 0..EP_COUNT {
            if other == ep {
                continue;
            }
            let (other_start, other_depth) = self.tx_fifo_cfg(other);
            if other_depth == 0 {
                continue;
            }
            let other_end = other_start.saturating_add(other_depth);
            if start < other_end && other_start < end {
                return true;
            }
        }
        false
    }

    fn tx_fifo_effective_depth_words(&self, ep: usize) -> u32 {
        let (_, depth) = self.tx_fifo_cfg(ep);
        if self.tx_fifo_overlap_invalid(ep) {
            0
        } else {
            depth
        }
    }

    fn reset_tx_fifo_level(&mut self, ep: usize) {
        if ep < EP_COUNT {
            self.dtxfsts[ep] = self.tx_fifo_effective_depth_words(ep);
        }
    }

    fn reset_all_tx_fifo_levels(&mut self) {
        for ep in 0..EP_COUNT {
            self.reset_tx_fifo_level(ep);
        }
    }

    fn enum_stage_setup_packet(stage: UsbEnumStage) -> Option<([u32; 2], &'static str)> {
        match stage {
            UsbEnumStage::DeliverSetAddress => Some((SETUP_SET_ADDRESS, "SetAddress")),
            UsbEnumStage::DeliverSetConfig => Some((SETUP_SET_CONFIG, "SetConfig")),
            UsbEnumStage::DeliverGetLineCoding => {
                Some((SETUP_CDC_GET_LINE_CODING, "GetLineCoding"))
            }
            UsbEnumStage::DeliverSetLineCoding => {
                Some((SETUP_CDC_SET_LINE_CODING, "SetLineCoding"))
            }
            UsbEnumStage::DeliverSetControlLineState => {
                Some((SETUP_CDC_SET_CONTROL_LINE_STATE, "SetControlLineState"))
            }
            UsbEnumStage::Configured | UsbEnumStage::Idle => None,
        }
    }
    fn rx_fifo_capacity_words(&self) -> u32 {
        let words = self.grxfsiz & 0xFFFF;
        if words == 0 { 1 } else { words }
    }

    fn assert_rxflvl(&mut self, words: u32) {
        self.rx_fifo_level = words.min(self.rx_fifo_capacity_words());
        if self.rx_fifo_level > 0 {
            self.gintsts |= GINTSTS_RXFLVL;
        }
    }

    fn clear_rxflvl(&mut self) {
        self.gintsts &= !GINTSTS_RXFLVL;
        self.rx_fifo_level = 0;
    }

    fn maybe_arm_startup_events(&mut self) {
        if self.startup_stage == StartupStage::Idle
            && self.gccfg != 0
            && self.gintmsk != 0
            && self.gahbcfg & GAHBCFG_GINT != 0
        {
            info!("OTG_FS: USB enumeration starting (gccfg={:#x} gintmsk={:#x} gahbcfg={:#x})", self.gccfg, self.gintmsk, self.gahbcfg);
            self.startup_stage = StartupStage::UsbReset;
            self.event_delay = OTG_STARTUP_EVENT_DELAY;
            self.sof_delay = OTG_SOF_PERIOD;
        }
    }

    fn masked_interrupts(&self) -> u32 {
        self.gintsts & self.gintmsk
    }

    fn maybe_raise_irq(&self, sys: &System) {
        if (self.gahbcfg & GAHBCFG_GINT) == 0 {
            return;
        }
        if self.masked_interrupts() == 0 {
            return;
        }
        sys.p.nvic.borrow_mut().set_intr_pending(OTG_FS_IRQ);
    }

    /// Calculate RX FIFO level: pending setup, out data, or generic transfers.
    /// Represents number of words available in GRXSTSR[15:0] (GRXFLVL).
    fn calculate_rx_fifo_level(&self) -> u32 {
        match self.ep0_rx_state {
            Ep0RxState::RxFlvlFifoPending | Ep0RxState::OutDataFifoPending => {
                // Pending FIFO read: count remaining words after ep0_fifo_read_count.
                let remaining = 2u32.saturating_sub(self.ep0_fifo_read_count as u32);
                remaining
            }
            Ep0RxState::RxFlvlStatusPending | Ep0RxState::OutDataStatusPending => {
                // Status pending: RX FIFO shows the PKTSTS/BCNT header word (1 word available).
                1
            }
            _ => self.rx_fifo_level,
        }
    }

    fn inject_startup_event(&mut self, mask: u32) {
        self.gintsts |= mask;
    }

    fn update_endpoint_summary(&mut self) {
        let in_pending = self.daint & self.daintmsk & 0x0000_FFFF;
        let out_pending = self.daint & self.daintmsk & 0xFFFF_0000;

        if in_pending != 0 {
            self.gintsts |= GINTSTS_IEPINT;
        } else {
            self.gintsts &= !GINTSTS_IEPINT;
        }

        if out_pending != 0 {
            self.gintsts |= GINTSTS_OEPINT;
        } else {
            self.gintsts &= !GINTSTS_OEPINT;
        }
    }

    fn mark_in_endpoint_interrupt(&mut self, ep: usize, mask: u32) {
        if ep >= EP_COUNT {
            return;
        }
        self.diepint[ep] |= mask;
        self.daint |= 1 << ep;
        self.update_endpoint_summary();
    }

    fn mark_out_endpoint_interrupt(&mut self, ep: usize, mask: u32) {
        if ep >= EP_COUNT {
            return;
        }
        self.doepint[ep] |= mask;
        self.daint |= 1 << (16 + ep);
        self.update_endpoint_summary();
    }

    fn clear_in_endpoint_interrupt(&mut self, ep: usize, mask: u32) {
        if ep >= EP_COUNT {
            return;
        }
        self.diepint[ep] &= !mask;
        if self.diepint[ep] == 0 {
            self.daint &= !(1 << ep);
        }
        self.update_endpoint_summary();
    }

    fn clear_out_endpoint_interrupt(&mut self, ep: usize, mask: u32) {
        if ep >= EP_COUNT {
            return;
        }
        self.doepint[ep] &= !mask;
        if self.doepint[ep] == 0 {
            self.daint &= !(1 << (16 + ep));
        }
        self.update_endpoint_summary();
    }

    fn update_doepctl(&mut self, ep: usize, value: u32) {
        if ep >= EP_COUNT {
            return;
        }

        let old_reg = self.doepctl[ep];
        let mut reg = old_reg;
        reg = (reg & !(DOEPCTL_EPENA | DOEPCTL_USBAEP | DOEPCTL_NAKSTS))
            | (value & (DOEPCTL_EPENA | DOEPCTL_USBAEP | 0x003F_0000 | 0x7FF));

        if value & DOEPCTL_SNAK != 0 {
            reg |= DOEPCTL_NAKSTS;
        }
        if value & DOEPCTL_CNAK != 0 {
            reg &= !DOEPCTL_NAKSTS;
        }

        self.doepctl[ep] = reg;

        // On EPENA clear, mark endpoint as no longer accepting OUT transfers.
        if (old_reg & DOEPCTL_EPENA) != 0 && (reg & DOEPCTL_EPENA) == 0 {
            otg_debug!("DOEPCTL{ep} EPENA cleared (OUT endpoint disabled)");
        }
    }

    fn update_diepctl(&mut self, ep: usize, value: u32) {
        if ep >= EP_COUNT {
            return;
        }

        let mut reg = self.diepctl[ep];
        reg = (reg & !(DIEPCTL_EPENA | DIEPCTL_USBAEP | DIEPCTL_NAKSTS))
            | (value & (DIEPCTL_EPENA | DIEPCTL_USBAEP | 0x03FF_0000 | 0x7FF));

        if value & DIEPCTL_SNAK != 0 {
            reg |= DIEPCTL_NAKSTS;
        }
        if value & DIEPCTL_CNAK != 0 {
            reg &= !DIEPCTL_NAKSTS;
        }

        self.diepctl[ep] = reg;

        if value & DIEPCTL_EPENA != 0 {
            if ep == 0 {
                self.ep0_in_transfer_pending = true;
            } else {
                otg_debug!("DIEPCTL{ep} EPENA set dieptsiz={:#010x} diepempmsk={:#010x}", self.dieptsiz[ep], self.diepempmsk);
                if ep < EP_COUNT {
                    self.ep_in_transfer_pending[ep] = true;
                    self.ep_txfe_was_fired[ep] = false;
                }
            }

            if self.diepempmsk & (1 << ep) != 0 {
                self.mark_in_endpoint_interrupt(ep, DIEPINT_TXFE);
                if ep > 0 && ep < EP_COUNT { self.ep_txfe_was_fired[ep] = true; }
            }
        }
    }

    fn read_device_endpoint(&self, offset: u32) -> u32 {
        match offset {
            0x0100..=0x017f => {
                let ep = ((offset - 0x0100) / 0x20) as usize;
                match (offset - 0x0100) % 0x20 {
                    0x00 => self.diepctl.get(ep).copied().unwrap_or(0),
                    0x08 => self.diepint.get(ep).copied().unwrap_or(0),
                    0x10 => self.dieptsiz.get(ep).copied().unwrap_or(0),
                    0x18 => {
                        // DTXFSTS: transmit FIFO status. Bits [15:0] = number of free space
                        // locations in the IN endpoint TX FIFO (in 32-bit words).
                        let fifo_space = if ep < EP_COUNT {
                            let depth = self.tx_fifo_effective_depth_words(ep);
                            let free = self.dtxfsts[ep].min(depth);
                            free
                        } else {
                            0x0080
                        };
                        fifo_space
                    }
                    _ => 0,
                }
            }
            0x0300..=0x037f => {
                let ep = ((offset - 0x0300) / 0x20) as usize;
                match (offset - 0x0300) % 0x20 {
                    0x00 => self.doepctl.get(ep).copied().unwrap_or(0),
                    0x08 => self.doepint.get(ep).copied().unwrap_or(0),
                    0x10 => self.doeptsiz.get(ep).copied().unwrap_or(0),
                    _ => 0,
                }
            }
            _ => 0,
        }
    }

    fn write_device_endpoint(&mut self, offset: u32, value: u32) -> bool {
        match offset {
            0x0100..=0x017f => {
                let ep = ((offset - 0x0100) / 0x20) as usize;
                match (offset - 0x0100) % 0x20 {
                    0x00 => self.update_diepctl(ep, value),
                    0x08 => self.clear_in_endpoint_interrupt(ep, value),
                    0x10 => {
                        if ep < EP_COUNT {
                            if ep > 0 {
                                info!("OTG_FS: EP{} DIEPTSIZ write value={:#010x} (usb_lld_start_in)", ep, value);
                            }
                            self.dieptsiz[ep] = value;
                            // When DIEPTSIZ is written with valid packet count/size, restore TX FIFO space.
                            if value != 0 && value != 0xFFFF_FFFF {
                                self.reset_tx_fifo_level(ep);
                            }
                        }
                    }
                    _ => {}
                }
                true
            }
            0x0300..=0x037f => {
                let ep = ((offset - 0x0300) / 0x20) as usize;
                match (offset - 0x0300) % 0x20 {
                    0x00 => self.update_doepctl(ep, value),
                    0x08 => self.clear_out_endpoint_interrupt(ep, value),
                    0x10 => {
                        if ep < EP_COUNT {
                            self.doeptsiz[ep] = value;
                        }
                    }
                    _ => {}
                }
                true
            }
            _ => false,
        }
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
            0x001c => {
                // GRXSTSR is a status register (no pop side-effect).
                match self.ep0_rx_state {
                    Ep0RxState::RxFlvlStatusPending => GRXSTSP_PKTSTS_SETUP_DATA | GRXSTSP_BCNT_8,
                    Ep0RxState::RxFlvlCompletePending => GRXSTSP_PKTSTS_SETUP_COMPL,
                    Ep0RxState::OutDataStatusPending => GRXSTSP_PKTSTS_OUT_DATA | GRXSTSP_BCNT_7,
                    Ep0RxState::OutCompletePending => GRXSTSP_PKTSTS_OUT_COMPL,
                    _ => self.grxstsr,
                }
            }
            0x0020 => {
                // GRXSTSP is a pop register; serve the enumeration FIFO state machine.
                let result = match self.ep0_rx_state {
                    Ep0RxState::RxFlvlStatusPending => {
                        self.ep0_rx_state = Ep0RxState::RxFlvlFifoPending;
                        self.ep0_fifo_read_count = 0;
                        GRXSTSP_PKTSTS_SETUP_DATA | GRXSTSP_BCNT_8
                    }
                    Ep0RxState::RxFlvlCompletePending => {
                        self.ep0_rx_state = Ep0RxState::StupPending;
                        self.clear_rxflvl();
                        GRXSTSP_PKTSTS_SETUP_COMPL
                    }
                    Ep0RxState::OutDataStatusPending => {
                        self.ep0_rx_state = Ep0RxState::OutDataFifoPending;
                        self.ep0_fifo_read_count = 0;
                        GRXSTSP_PKTSTS_OUT_DATA | GRXSTSP_BCNT_7
                    }
                    Ep0RxState::OutCompletePending => {
                        self.ep0_rx_state = Ep0RxState::OutXfrcPending;
                        self.clear_rxflvl();
                        GRXSTSP_PKTSTS_OUT_COMPL
                    }
                    _ => self.grxstsr,
                };
                otg_debug!("GRXSTSP pop at 0x{:03x} state_after={:?} result={:#010x}", offset, self.ep0_rx_state, result);
                result
            }
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
            0x0014 => {
                // GINTSTS is mostly W1C, but RXFLVL reflects RxFIFO non-empty state and
                // should not be directly cleared by firmware writes.
                self.gintsts &= !(value & !GINTSTS_RXFLVL);
            }
            0x0018 => self.gintmsk = value,
            0x0024 => self.grxfsiz = value,
            0x0028 => {
                self.dieptxf0 = value;
                self.reset_all_tx_fifo_levels();
            }
            0x0038 => self.gccfg = value,
            0x003c => self.cid = value,
            0x0100 => self.hptxfsiz = value,
            0x0104 => {
                self.dieptxf[0] = value;
                self.reset_all_tx_fifo_levels();
            }
            0x0108 => {
                self.dieptxf[1] = value;
                self.reset_all_tx_fifo_levels();
            }
            0x010c => {
                self.dieptxf[2] = value;
                self.reset_all_tx_fifo_levels();
            }
            _ => {}
        }

        self.maybe_arm_startup_events();
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
            0x0034 => self.diepempmsk,
            _ => self.read_device_endpoint(offset),
        }
    }

    fn write_device(&mut self, offset: u32, value: u32) {
        match offset {
            0x0000 => self.dcfg = value,
            0x0004 => self.dctl = value,
            0x0008 => {
                // DSTS is read-only status in device mode.
            }
            0x0010 => self.diepmsk = value,
            0x0014 => self.doepmsk = value,
            0x0018 => self.daint &= !value,
            0x001c => self.daintmsk = value,
            0x0034 => {
                if (value ^ self.diepempmsk) & (1 << 1) != 0 {
                    info!(
                        "OTG_FS: DIEPEMPMSK write old={:#010x} new={:#010x} ep1_enabled={}",
                        self.diepempmsk,
                        value,
                        (value & (1 << 1)) != 0
                    );
                }
                self.diepempmsk = value;
                for ep in 0..EP_COUNT {
                    if value & (1 << ep) != 0 && self.diepctl[ep] & DIEPCTL_EPENA != 0 {
                        self.mark_in_endpoint_interrupt(ep, DIEPINT_TXFE);
                        if ep > 0 && ep < EP_COUNT { self.ep_txfe_was_fired[ep] = true; }
                    }
                }
            }
            _ => {
                if !self.write_device_endpoint(offset, value) {
                    return;
                }
            }
        }

        self.update_endpoint_summary();
    }
}

impl Peripheral for OtgFs {
    fn read(&mut self, _sys: &System, offset: u32) -> u32 {
        match self.kind {
            OtgKind::Global => self.shared.borrow_mut().read_global(offset),
            OtgKind::Device => self.shared.borrow().read_device(offset),
            OtgKind::PwrClk => match offset {
                0x0000 => self.shared.borrow().pcgcctl,
                _ => 0,
            },
        }
    }

    fn write(&mut self, _sys: &System, offset: u32, value: u32) {
        match self.kind {
            OtgKind::Global => self.shared.borrow_mut().write_global(offset, value),
            OtgKind::Device => self.shared.borrow_mut().write_device(offset, value),
            OtgKind::PwrClk => {
                if offset == 0x0000 {
                    self.shared.borrow_mut().pcgcctl = value;
                }
            }
        }
    }

    fn step(&mut self, sys: &System) {
        if !matches!(self.kind, OtgKind::Global) {
            return;
        }

        let mut shared = self.shared.borrow_mut();

        shared.maybe_arm_startup_events();

        match shared.startup_stage {
            StartupStage::Idle => {}
            StartupStage::UsbReset => {
                if shared.gintsts & GINTSTS_USBRST == 0 {
                    if shared.event_delay > 0 {
                        shared.event_delay -= 1;
                    } else {
                        shared.inject_startup_event(GINTSTS_SRQINT | GINTSTS_USBRST);
                        shared.startup_stage = StartupStage::EnumDone;
                        shared.event_delay = OTG_STARTUP_EVENT_DELAY;
                    }
                }
            }
            StartupStage::EnumDone => {
                if shared.gintsts & (GINTSTS_SRQINT | GINTSTS_USBRST) == 0 {
                    if shared.event_delay > 0 {
                        shared.event_delay -= 1;
                    } else {
                        shared.inject_startup_event(GINTSTS_ENUMDNE);
                        shared.startup_stage = StartupStage::Running;
                        shared.sof_delay = OTG_SOF_PERIOD;
                        shared.enum_stage = UsbEnumStage::DeliverSetAddress;
                        shared.ep0_rx_state = Ep0RxState::Idle;
                        shared.ep0_setup_inflight = false;
                    }
                }
            }
            StartupStage::Running => {
                if shared.gintmsk & GINTSTS_SOF != 0 && shared.gintsts & GINTSTS_SOF == 0 {
                    if shared.sof_delay > 0 {
                        shared.sof_delay -= 1;
                    } else {
                        otg_debug!("SOF injected (gintmsk={:#010x} gintsts_before={:#010x} irq_latched={})", shared.gintmsk, shared.gintsts, shared.irq_latched);
                        shared.inject_startup_event(GINTSTS_SOF);
                        shared.sof_delay = OTG_SOF_PERIOD;
                    }
                }
            }
        }

        // RXFLVL -> GRXSTSP -> FIFO -> STUP delivery for synthetic enumeration setup packets.
        let ep0_armed = shared.startup_stage == StartupStage::Running
            && shared.doeptsiz[0] != 0
            && shared.doepctl[0] & (DOEPCTL_USBAEP | DOEPCTL_CNAK | DOEPCTL_EPENA) != 0;

        // Log once when we're waiting for ep0_armed but it isn't true yet - for key enum stages.
        if !ep0_armed && shared.ep0_rx_state == Ep0RxState::Idle
            && (shared.enum_stage == UsbEnumStage::DeliverSetConfig
                || shared.enum_stage == UsbEnumStage::DeliverSetAddress)
        {
            // Only log rarely to avoid flood; use the doeptsiz==0 case as the diagnostic.
            if shared.doeptsiz[0] == 0 {
                otg_debug!("ep0_armed=false waiting: enum_stage={:?} doeptsiz0=0 doepctl0={:#010x}",
                    shared.enum_stage, shared.doepctl[0]);
            }
        }

        if ep0_armed
            && shared.ep0_rx_state == Ep0RxState::Idle
            && shared.doepint[0] & DOEPINT_STUP == 0
            && !shared.ep0_setup_inflight
        {
            if let Some((pkt, label)) = OtgFsState::enum_stage_setup_packet(shared.enum_stage) {
                info!("OTG_FS: delivering {} SETUP packet to EP0", label);
                shared.ep0_pending_setup = pkt;
                shared.ep0_rx_state = Ep0RxState::RxFlvlStatusPending;
                shared.ep0_setup_inflight = true;
                shared.assert_rxflvl(1);
            }
        }

        // On real hardware RXFLVL stays asserted while the RxFIFO is non-empty.
        // The ChibiOS ISR clears all GINTSTS bits (including RXFLVL) at ISR entry.
        // We re-assert RXFLVL for states that still have data pending, then reset
        // irq_latched to allow the next IRQ 67 delivery. The latch is also reset
        // whenever RxFlvlCompletePending is active — this is a new epoch (firmware
        // finished reading the FIFO data and we need another IRQ to deliver the
        // PKTSTS_SETUP_COMPL status read from GRXSTSP).
        if matches!(
            shared.ep0_rx_state,
            Ep0RxState::RxFlvlFifoPending
                | Ep0RxState::RxFlvlCompletePending
                | Ep0RxState::OutDataFifoPending
                | Ep0RxState::OutCompletePending
        ) {
            let lvl = shared.calculate_rx_fifo_level().max(1);
            shared.assert_rxflvl(lvl);
        }

        // Reset irq_latched when RXFLVL has been freshly re-asserted for a new epoch,
        // or when there are genuinely no pending interrupts. Without this, the second
        // RXFLVL IRQ (PKTSTS_SETUP_COMPL) never fires because masked_interrupts() is
        // non-zero from the re-asserted RXFLVL and irq_latched is never cleared.
        if shared.gahbcfg & GAHBCFG_GINT == 0
            || shared.masked_interrupts() == 0
            || matches!(shared.ep0_rx_state, Ep0RxState::RxFlvlCompletePending | Ep0RxState::OutCompletePending)
        {
            shared.irq_latched = false;
        }

        if shared.ep0_rx_state == Ep0RxState::StupPending && ep0_armed {
            if shared.enum_stage == UsbEnumStage::DeliverSetLineCoding {
                info!("OTG_FS: StupPending fires → DOEPINT STUP (enum_stage=DeliverSetLineCoding), scheduling OUT data");
                shared.mark_out_endpoint_interrupt(0, DOEPINT_STUP);
                shared.ep0_pending_out_data = CDC_LINE_CODING_115200_8N1;
                shared.ep0_rx_state = Ep0RxState::OutDataStatusPending;
                shared.assert_rxflvl(1);
            } else if shared.enum_stage == UsbEnumStage::DeliverGetLineCoding {
                info!("OTG_FS: StupPending fires → DOEPINT STUP (enum_stage=DeliverGetLineCoding)");
                shared.mark_out_endpoint_interrupt(0, DOEPINT_STUP);
                shared.ep0_rx_state = Ep0RxState::Idle;
            } else {
                info!("OTG_FS: StupPending fires → DOEPINT STUP|XFRC  (enum_stage={:?})", shared.enum_stage);
                shared.mark_out_endpoint_interrupt(0, DOEPINT_STUP | DOEPINT_XFRC);
                shared.ep0_rx_state = Ep0RxState::Idle;
            }
            // Force irq_latched false so the new OEPINT bit triggers a fresh IRQ 67 raise.
            // Without this, irq_latched stays true from the previous RXFLVL raise and the
            // firmware never re-enters the OTG ISR to service the SETUP packet.
            shared.irq_latched = false;
        }

        if shared.ep0_rx_state == Ep0RxState::OutXfrcPending && ep0_armed {
            info!("OTG_FS: EP0 OUT data complete → DOEPINT XFRC (enum_stage={:?})", shared.enum_stage);
            shared.mark_out_endpoint_interrupt(0, DOEPINT_XFRC);
            shared.ep0_rx_state = Ep0RxState::Idle;
            shared.irq_latched = false;
        }

        // Handle non-enumeration OUT transfers on generic endpoints after enumeration completes.
        // For endpoints with EPENA and USBAEP set on DOEPCTL, simulate receiving data completion
        // and fire XFRC interrupt if firmware has armed the endpoint.
        if shared.ep0_in_transfer_pending && shared.diepctl[0] & DIEPCTL_EPENA != 0 {
            info!("OTG_FS: EP0 IN ZLP XFRC (enum_stage={:?} diepctl0={:#010x})", shared.enum_stage, shared.diepctl[0]);
            shared.clear_in_endpoint_interrupt(0, DIEPINT_TXFE);
            shared.mark_in_endpoint_interrupt(0, DIEPINT_XFRC);
            shared.diepctl[0] &= !DIEPCTL_EPENA;
            shared.ep0_in_transfer_pending = false;
            // Advance USB enumeration stage after each EP0 IN transfer completes.
            match shared.enum_stage {
                UsbEnumStage::DeliverSetAddress => {
                    info!("OTG_FS: SetAddress ZLP done → DeliverSetConfig");
                    shared.enum_stage = UsbEnumStage::DeliverSetConfig;
                    shared.ep0_setup_inflight = false;
                }
                UsbEnumStage::DeliverSetConfig  => {
                    info!("OTG_FS: SetConfig ZLP done → DeliverGetLineCoding");
                    shared.enum_stage = UsbEnumStage::DeliverGetLineCoding;
                    shared.ep0_setup_inflight = false;
                }
                UsbEnumStage::DeliverGetLineCoding => {
                    info!("OTG_FS: GetLineCoding data done → DeliverSetLineCoding");
                    shared.enum_stage = UsbEnumStage::DeliverSetLineCoding;
                    shared.ep0_setup_inflight = false;
                }
                UsbEnumStage::DeliverSetLineCoding => {
                    info!("OTG_FS: SetLineCoding ZLP done → DeliverSetControlLineState");
                    shared.enum_stage = UsbEnumStage::DeliverSetControlLineState;
                    shared.ep0_setup_inflight = false;
                }
                UsbEnumStage::DeliverSetControlLineState => {
                    info!("OTG_FS: SetControlLineState ZLP done → Configured");
                    shared.enum_stage = UsbEnumStage::Configured;
                    shared.ep0_setup_inflight = false;
                }
                _ => {
                    shared.ep0_setup_inflight = false;
                }
            }
        }

        // Fire XFRC for non-EP0 IN endpoints (CDC bulk/interrupt) after EPENA is set.
        // ChibiOS always sets DIEPEMPMSK after EPENA (TXFE-interrupt-driven path).
        // We use ep_txfe_was_fired to avoid racing XFRC against DIEPEMPMSK being written
        // in the very next CPU instruction after EPENA.
        for ep in 1..EP_COUNT {
            if shared.ep_in_transfer_pending[ep] && shared.diepctl[ep] & DIEPCTL_EPENA != 0 {
                let txfe_active = shared.diepint[ep] & DIEPINT_TXFE != 0
                    && shared.diepempmsk & (1 << ep) != 0;
                let txfe_done = shared.ep_txfe_was_fired[ep]
                    && shared.diepempmsk & (1 << ep) == 0;
                if txfe_active {
                    // TXFE is live: let the firmware ISR fill the FIFO before XFRC.
                    shared.irq_latched = false;
                } else if txfe_done || !shared.cdc_line_buf.is_empty() {
                    // TXFE handled (firmware cleared DIEPEMPMSK after filling FIFO),
                    // or direct-fill path. Fire XFRC; line buffer is flushed by
                    // newline/null bytes in fifo_write, not here, so multi-packet
                    // messages accumulate into a single line naturally.
                    shared.clear_in_endpoint_interrupt(ep, DIEPINT_TXFE);
                    shared.mark_in_endpoint_interrupt(ep, DIEPINT_XFRC);
                    shared.reset_tx_fifo_level(ep);
                    shared.diepctl[ep] &= !DIEPCTL_EPENA;
                    shared.ep_in_transfer_pending[ep] = false;
                    shared.ep_txfe_was_fired[ep] = false;
                    shared.irq_latched = false;
                }
                // else: waiting for DIEPEMPMSK to be written (TXFE not yet fired)
            }
        }

        // Emit partial CDC output periodically during long-running transfers so
        // the operator does not need to interrupt execution to see progress.
        let now_clk = crate::emulator::NUM_INSTRUCTIONS.load(std::sync::atomic::Ordering::Relaxed);
        if !shared.cdc_line_buf.is_empty()
            && now_clk.saturating_sub(shared.cdc_last_periodic_flush_clk) >= CDC_PERIODIC_FLUSH_INTERVAL
        {
            let line = String::from_utf8_lossy(&shared.cdc_line_buf).trim().to_string();
            if !line.is_empty() {
                if crate::console_only() {
                    let _ = writeln!(io::stdout(), "{}", line);
                    let _ = io::stdout().flush();
                } else {
                    info!("USB-CDC ep1 '{}'", line);
                }
            }
            shared.cdc_line_buf.clear();
            shared.cdc_last_periodic_flush_clk = now_clk;
        }

        if shared.gahbcfg & GAHBCFG_GINT == 0 || shared.masked_interrupts() == 0 {
            // Already handled above (before RXFLVL re-assertion). Repeat here to also catch
            // the case where no RXFLVL is pending but other interrupt bits were just cleared.
            shared.irq_latched = false;
        } else {
            // Edge-style raise: assert IRQ once for each newly pending epoch.
            // Event producers above clear irq_latched when they introduce new sources.
            if !shared.irq_latched {
                shared.maybe_raise_irq(sys);
                shared.irq_latched = true;
            }
        }
    }
}

/// Read one word from the OTG FS EP data FIFO (address base 0x50001000 + ep*0x1000).
/// Called from Peripherals::read when firmware reads the FIFO region during RXFLVL handling.
pub fn fifo_read(_ep: usize) -> u32 {
    OTG_FS_SHARED.with(|shared| {
        let mut s = shared.borrow_mut();
        if s.ep0_rx_state == Ep0RxState::RxFlvlFifoPending || s.ep0_rx_state == Ep0RxState::OutDataFifoPending {
            let idx = s.ep0_fifo_read_count as usize;
            let word = if idx < 2 {
                if s.ep0_rx_state == Ep0RxState::RxFlvlFifoPending {
                    s.ep0_pending_setup[idx]
                } else {
                    s.ep0_pending_out_data[idx]
                }
            } else {
                0
            };
            s.ep0_fifo_read_count += 1;
            debug!("OTG_FS FIFO read idx={} word={:#010x} state_after={:?}", idx, word, s.ep0_rx_state);
            if s.ep0_fifo_read_count >= 2 {
                s.ep0_rx_state = if s.ep0_rx_state == Ep0RxState::RxFlvlFifoPending {
                    Ep0RxState::RxFlvlCompletePending
                } else {
                    Ep0RxState::OutCompletePending
                };
            }
            word
        } else {
            debug!("OTG_FS FIFO read (state={:?} → 0)", s.ep0_rx_state);
            0
        }
    })
}

/// Write one word to the OTG FS EP TX FIFO (address base 0x50001000 + ep*0x1000).
/// Called from Peripherals::write when firmware writes bulk IN data during CDC transmission.
/// EP1 is the CubeBlack CDC data bulk IN endpoint (console output).
pub fn fifo_write(ep: usize, word: u32) {
    // Only capture EP1 (CDC data bulk IN = console) and EP2 (CDC interrupt IN).
    // EP0 TX FIFO is used for control responses (descriptors, ZLPs) — skip those.
    debug!("OTG FIFO write ep={} word=0x{:08x}", ep, word);
    if ep == 0 {
        return;
    }
    OTG_FS_SHARED.with(|shared| {
        let mut s = shared.borrow_mut();

        if ep < EP_COUNT {
            let ep_active = (s.diepctl[ep] & (DIEPCTL_EPENA | DIEPCTL_USBAEP))
                == (DIEPCTL_EPENA | DIEPCTL_USBAEP);
            let depth_words = s.tx_fifo_effective_depth_words(ep);
            if !ep_active || depth_words == 0 {
                // Endpoint/FIFO ownership not valid: keep TXFE clear and ignore this write.
                s.clear_in_endpoint_interrupt(ep, DIEPINT_TXFE);
                return;
            }

            // Each 32-bit write consumes one FIFO word when capacity is available.
            s.dtxfsts[ep] = s.dtxfsts[ep].min(depth_words).saturating_sub(1);
            let free_words = s.dtxfsts[ep];
            let txfe_threshold_words = std::cmp::max(1, depth_words / 2);
            if free_words == 0 {
                // FIFO full: no empty interrupt should remain asserted.
                s.clear_in_endpoint_interrupt(ep, DIEPINT_TXFE);
            } else if (s.diepempmsk & (1 << ep)) != 0
                && (s.diepctl[ep] & DIEPCTL_EPENA) != 0
                && free_words >= txfe_threshold_words
            {
                // Keep TXFE aligned with live free-space visibility while endpoint is active.
                s.mark_in_endpoint_interrupt(ep, DIEPINT_TXFE);
            } else {
                s.clear_in_endpoint_interrupt(ep, DIEPINT_TXFE);
            }
        }

        // In console-only mode, stream EP1 bytes immediately so output is visible
        // even if firmware has not emitted a newline-terminated record yet.
        let bytes = word.to_le_bytes();
        if crate::console_only() && ep == 1 {
            for &b in &bytes {
                if b != 0 {
                    let _ = io::stdout().write_all(&[b]);
                }
            }
            let _ = io::stdout().flush();
            return;
        }

        // Accumulate bytes into the cdc_rx buffer, flush on newline.
        for &b in &bytes {
            if b == b'\n' || b == b'\r' || b == 0 {
                // Flush on LF, CR, or null (end-of-transfer padding).
                if !s.cdc_line_buf.is_empty() {
                    let line = String::from_utf8_lossy(&s.cdc_line_buf);
                    let line_owned = line.trim().to_string();
                    if !line_owned.is_empty() {
                        if crate::console_only() {
                            let _ = writeln!(io::stdout(), "{}", line_owned);
                            let _ = io::stdout().flush();
                        } else {
                            info!("USB-CDC ep{} '{}'", ep, line_owned);
                        }
                    }
                    s.cdc_line_buf.clear();
                }
            } else {
                s.cdc_line_buf.push(b);
                if s.cdc_line_buf.len() >= 256 {
                    let line = String::from_utf8_lossy(&s.cdc_line_buf).trim().to_string();
                    if crate::console_only() {
                        let _ = writeln!(io::stdout(), "{}", line);
                        let _ = io::stdout().flush();
                    } else {
                        info!("USB-CDC ep{} '{}'", ep, line);
                    }
                    s.cdc_line_buf.clear();
                }
            }
        }
    })
}

/// Flush any pending CDC line-buffer bytes that did not end with LF/CR/NUL yet.
/// This allows bounded runs to emit the final partial console line on normal exit.
pub fn flush_cdc_pending_output() {
    OTG_FS_SHARED.with(|shared| {
        let mut s = shared.borrow_mut();
        if s.cdc_line_buf.is_empty() {
            return;
        }

        let line = String::from_utf8_lossy(&s.cdc_line_buf).trim().to_string();
        if !line.is_empty() {
            if crate::console_only() {
                let _ = writeln!(io::stdout(), "{}", line);
                let _ = io::stdout().flush();
            } else {
                info!("USB-CDC ep1 '{}'", line);
            }
        }

        s.cdc_line_buf.clear();
    })
}