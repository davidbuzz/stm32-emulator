// SPDX-License-Identifier: GPL-3.0-or-later

// STM32 name: DMA1 / DMA2 (direct memory access controller).
// STM32F427 bases: DMA1=0x40026000, DMA2=0x40026400.
// Key registers: LISR/HISR/LIFCR/HIFCR plus stream windows at 0x10 + n*0x18.
// Key behavior: stream EN semantics, NDTR countdown, PAR/MxAR addressing, status bits.
// Critical for this emulator: firmware uses DMA completion flags and IRQs for boot/runtime.
// Current model: per-beat PINC/MINC, circular-mode NDTR reload, EN retrigger guard,
// and TC/HT/TE/DME/FE class flags.
// Still incomplete: full RM-accurate FIFO sequencing under all edge races, double-buffer details.
// Datasheet/reference anchors: STM32F4 RM DMA chapter and cubeblack/STM32F4_DMA.md.

use crate::util::UniErr;
use crate::system::System;
use crate::emulator::NUM_INSTRUCTIONS;
use super::Peripheral;
use super::Peripherals;

const USART_RX_IDLE_DISABLE_DELAY: u64 = 20_000;
const SDIO_DMA_DEFER_DELAY: u64 = 64;
const DMA_EN_DISABLE_DELAY: u64 = 8;
const DMA_FIFO_CAPACITY_BYTES: usize = 16;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
enum FifoOccupancy {
    #[default]
    Empty = 0,           // 0 bytes
    Quarter = 1,         // 0 < fill < 1/4 of FIFO
    OneQuarter = 2,      // fill == 1/4 of FIFO
    Half = 3,            // 1/4 < fill < 1/2 of FIFO
    ThreeQuarters = 4,   // fill == 3/4 of FIFO or between 1/2 and 3/4
    Full = 5,            // fill >= capacity, ERROR state
}

impl FifoOccupancy {
    fn from_bytes(bytes: usize, capacity: usize) -> Self {
        use FifoOccupancy::*;
        if bytes >= capacity {
            Full
        } else if bytes * 4 >= capacity * 3 {
            ThreeQuarters
        } else if bytes * 2 >= capacity {
            Half
        } else if bytes * 4 == capacity {
            OneQuarter
        } else if bytes > 0 {
            Quarter
        } else {
            Empty
        }
    }
}

#[derive(Default)]
pub struct Dma {
    name: String,
    streams: [Stream; 8],
    lisr: u32,
    hisr: u32,
    arb_cursor: usize,
    deferred_stream_irq: [bool; 8],
    starvation_age: [u8; 8],
}

impl Dma {
    fn ready_without_conflict_gate(&self, sys: &System, stream_idx: usize) -> bool {
        if stream_idx >= self.streams.len() {
            return false;
        }
        self.streams[stream_idx].ready_for_step(sys)
    }

    fn update_starvation_age(&mut self, sys: &System, winner: Option<usize>) {
        for i in 0..8 {
            if Some(i) == winner {
                self.starvation_age[i] = 0;
                continue;
            }

            if self.ready_without_conflict_gate(sys, i) {
                self.starvation_age[i] = self.starvation_age[i].saturating_add(1);
            } else {
                self.starvation_age[i] = 0;
            }
        }
    }

    fn pick_step_winner(&self, sys: &System) -> Option<usize> {
        let mut best: Option<(usize, u8, u8, usize)> = None;

        for step_idx in 0..8 {
            let i = (self.arb_cursor + step_idx) % 8;
            if self.stream_blocked_by_active_owner(sys, i) {
                continue;
            }
            if !self.ready_without_conflict_gate(sys, i) {
                continue;
            }

            let pl = self.streams[i].priority();
            let age = self.starvation_age[i];
            let rank = (i + 8 - (self.arb_cursor % 8)) % 8;

            match best {
                None => best = Some((i, pl, age, rank)),
                Some((_, best_pl, best_age, best_rank)) => {
                    let better = pl > best_pl
                        || (pl == best_pl && age > best_age)
                        || (pl == best_pl && age == best_age && rank < best_rank);
                    if better {
                        best = Some((i, pl, age, rank));
                    }
                }
            }
        }

        best.map(|(idx, _, _, _)| idx)
    }

    pub fn new(name: &str) -> Option<Box<dyn Peripheral>> {
        if name.starts_with("DMA") {
            let name = name.to_string();
            Some(Box::new(Self { name, ..Self::default() }))
        } else {
            None
        }
    }

    fn set_tcif(&mut self, stream: usize) {
        let bit = tcif_mask(stream);
        let gif = gif_mask(stream);
        if stream < 4 {
            self.lisr |= bit;
            self.lisr |= gif;
        } else {
            self.hisr |= bit;
            self.hisr |= gif;
        }
    }

    fn set_htif(&mut self, stream: usize) {
        let bit = htif_mask(stream);
        let gif = gif_mask(stream);
        if stream < 4 {
            self.lisr |= bit;
            self.lisr |= gif;
        } else {
            self.hisr |= bit;
            self.hisr |= gif;
        }
    }

    fn set_teif(&mut self, stream: usize) {
        let bit = teif_mask(stream);
        let gif = gif_mask(stream);
        if stream < 4 {
            self.lisr |= bit;
            self.lisr |= gif;
        } else {
            self.hisr |= bit;
            self.hisr |= gif;
        }
    }

    fn set_dmeif(&mut self, stream: usize) {
        let bit = dmeif_mask(stream);
        let gif = gif_mask(stream);
        if stream < 4 {
            self.lisr |= bit;
            self.lisr |= gif;
        } else {
            self.hisr |= bit;
            self.hisr |= gif;
        }
    }

    fn set_feif(&mut self, stream: usize) {
        let bit = feif_mask(stream);
        let gif = gif_mask(stream);
        if stream < 4 {
            self.lisr |= bit;
            self.lisr |= gif;
        } else {
            self.hisr |= bit;
            self.hisr |= gif;
        }
    }

    fn stream_irq(&self, stream: usize) -> Option<i32> {
        // STM32F427 IRQ numbers for DMA stream interrupts.
        // DMA1 Stream0..6 -> 11..17
        // DMA2 Stream0..4 -> 56..60, Stream5..7 -> 68..70
        match self.name.as_str() {
            "DMA1" => Some(11 + stream as i32),
            "DMA2" => Some(match stream {
                0 => 56,
                1 => 57,
                2 => 58,
                3 => 59,
                4 => 60,
                5 => 68,
                6 => 69,
                7 => 70,
                _ => return None,
            }),
            _ => None,
        }
    }

    fn signal_tc(&mut self, _sys: &System, stream_idx: usize) {
        self.set_tcif(stream_idx);
        if self.streams[stream_idx].tcie_enabled() {
            self.deferred_stream_irq[stream_idx] = true;
        }
    }

    fn signal_ht(&mut self, _sys: &System, stream_idx: usize) {
        self.set_htif(stream_idx);
        if self.streams[stream_idx].htie_enabled() {
            self.deferred_stream_irq[stream_idx] = true;
        }
    }

    fn signal_te(&mut self, _sys: &System, stream_idx: usize) {
        self.set_teif(stream_idx);
        if self.streams[stream_idx].teie_enabled() {
            self.deferred_stream_irq[stream_idx] = true;
        }
    }

    fn signal_dme(&mut self, _sys: &System, stream_idx: usize) {
        self.set_dmeif(stream_idx);
        if self.streams[stream_idx].dmeie_enabled() {
            self.deferred_stream_irq[stream_idx] = true;
        }
    }

    fn signal_fe(&mut self, _sys: &System, stream_idx: usize) {
        self.set_feif(stream_idx);
        if self.streams[stream_idx].feie_enabled() {
            self.deferred_stream_irq[stream_idx] = true;
        }
    }

    fn stream_has_enabled_pending_event(&self, stream: usize) -> bool {
        let bank = if stream < 4 { self.lisr } else { self.hisr };
        let has_tc = self.streams[stream].tcie_enabled() && (bank & tcif_mask(stream)) != 0;
        let has_ht = self.streams[stream].htie_enabled() && (bank & htif_mask(stream)) != 0;
        let has_te = self.streams[stream].teie_enabled() && (bank & teif_mask(stream)) != 0;
        let has_dme = self.streams[stream].dmeie_enabled() && (bank & dmeif_mask(stream)) != 0;
        let has_fe = self.streams[stream].feie_enabled() && (bank & feif_mask(stream)) != 0;
        has_tc || has_ht || has_te || has_dme || has_fe
    }

    fn service_deferred_stream_irqs(&mut self, sys: &System) {
        for stream in 0..8 {
            if !self.deferred_stream_irq[stream] {
                continue;
            }

            if self.stream_has_enabled_pending_event(stream) {
                if let Some(irq) = self.stream_irq(stream) {
                    sys.p.nvic.borrow_mut().set_intr_pending(irq);
                }
            }

            self.deferred_stream_irq[stream] = false;
        }
    }

    fn refresh_stream_irq_latches(&mut self) {
        for stream in 0..8 {
            if self.stream_has_enabled_pending_event(stream) {
                self.deferred_stream_irq[stream] = true;
            }
        }
    }

    fn signal_mode_error(&mut self, sys: &System, stream_idx: usize) {
        // TE is the generic transfer-error class. DME/FE are mode-specific:
        // direct mode uses DME, FIFO mode uses FE.
        self.signal_te(sys, stream_idx);
        if self.streams[stream_idx].fifo_enabled() {
            self.signal_fe(sys, stream_idx);
        } else {
            self.signal_dme(sys, stream_idx);
        }
    }

    fn find_request_conflicts(&self, stream_idx: usize, channel: u8) -> Vec<usize> {
        self.streams
            .iter()
            .enumerate()
            .filter_map(|(idx, s)| {
                if idx == stream_idx || (s.cr & 1) == 0 || s.channel() != channel {
                    return None;
                }

                // Mem2mem streams are not tied to peripheral request lines and should
                // not participate in shared peripheral request arbitration.
                if s.dir() == Dir::MemCopy || s.par == 0 {
                    return None;
                }

                Some(idx)
            })
            .collect()
    }

    fn stream_blocked_by_active_owner(&self, sys: &System, stream_idx: usize) -> bool {
        if stream_idx >= self.streams.len() || (self.streams[stream_idx].cr & 1) == 0 {
            return false;
        }

        let candidate = &self.streams[stream_idx];
        let channel = candidate.channel();
        let cand_dir = candidate.dir();
        if cand_dir == Dir::MemCopy || candidate.par == 0 {
            return false;
        }
        let cand_pl = candidate.priority();
        let cand_peri_desc = sys.p.addr_desc(candidate.par);
        let cand_peri_name = peripheral_name_from_desc(&cand_peri_desc);

        for owner_idx in self.find_request_conflicts(stream_idx, channel) {
            let owner = &self.streams[owner_idx];
            if !owner.ready_for_step(sys) {
                // Do not let sleeping/deferred owners starve an active contender.
                continue;
            }

            let owner_peri_desc = sys.p.addr_desc(owner.par);
            let owner_peri_name = peripheral_name_from_desc(&owner_peri_desc);
            let same_peripheral_request = same_peripheral_request_target(
                sys,
                candidate.par,
                cand_peri_name,
                owner.par,
                owner_peri_name,
            );

            if !same_peripheral_request {
                continue;
            }

            let owner_dir = owner.dir();
            let full_duplex_pair =
                (cand_dir == Dir::Read && owner_dir == Dir::Write)
                || (cand_dir == Dir::Write && owner_dir == Dir::Read);
            if full_duplex_pair {
                continue;
            }

            let owner_pl = owner.priority();
            if owner_pl > cand_pl {
                return true;
            }

            if owner_pl == cand_pl {
                // Let long-waiting contenders win equal-priority conflicts.
                if self.starvation_age[stream_idx] > self.starvation_age[owner_idx] {
                    continue;
                }
                let owner_rank = (owner_idx + 8 - (self.arb_cursor % 8)) % 8;
                let cand_rank = (stream_idx + 8 - (self.arb_cursor % 8)) % 8;
                if owner_rank < cand_rank {
                    return true;
                }
            }

        }

        false
    }

    fn clear_ifcr_bank(reg: &mut u32, base_stream: usize, value: u32) {
        for local in 0..4 {
            let stream = base_stream + local;
            let cgif = gif_mask(stream);
            let stream_flags = stream_flag_mask(stream);

            if (value & cgif) != 0 {
                *reg &= !(stream_flags | cgif);
                continue;
            }

            if (value & feif_mask(stream)) != 0 {
                *reg &= !feif_mask(stream);
            }
            if (value & dmeif_mask(stream)) != 0 {
                *reg &= !dmeif_mask(stream);
            }
            if (value & teif_mask(stream)) != 0 {
                *reg &= !teif_mask(stream);
            }
            if (value & htif_mask(stream)) != 0 {
                *reg &= !htif_mask(stream);
            }
            if (value & tcif_mask(stream)) != 0 {
                *reg &= !tcif_mask(stream);
            }

            if (*reg & stream_flags) == 0 {
                *reg &= !cgif;
            }
        }
    }
}

impl Peripheral for Dma {
    fn step(&mut self, sys: &System) {
        // Re-latch IRQ delivery from currently visible status+IE combinations so
        // enable-after-flag ordering still results in an interrupt pulse.
        self.refresh_stream_irq_latches();
        self.service_deferred_stream_irqs(sys);

        let name = self.name.clone();
        let mut winner: Option<usize> = None;
        if let Some(i) = self.pick_step_winner(sys) {
            match self.streams[i].step_deferred(&name, i, sys) {
                StreamStepResult::Progress { half } => {
                    if half {
                        self.signal_ht(sys, i);
                    }
                    winner = Some(i);
                }
                StreamStepResult::Completed { half } => {
                    if half {
                        self.signal_ht(sys, i);
                    }
                    self.signal_tc(sys, i);
                    winner = Some(i);
                }
                StreamStepResult::FifoError => {
                    self.signal_fe(sys, i);
                    winner = Some(i);
                }
                StreamStepResult::TransferError => {
                    self.signal_mode_error(sys, i);
                    winner = Some(i);
                }
                StreamStepResult::Noop => {
                    winner = None;
                }
            }
        }

        self.update_starvation_age(sys, winner);
        self.arb_cursor = winner.map(|idx| (idx + 1) % 8)
            .unwrap_or((self.arb_cursor + 1) % 8);
    }

    fn read(&mut self, sys: &System, offset: u32) -> u32 {
        match Access::from_offset(offset) {
            Access::Reg(Reg::Lisr) => self.lisr,
            Access::Reg(Reg::Hisr) => self.hisr,
            Access::Reg(Reg::Lifcr) | Access::Reg(Reg::Hifcr) | Access::Reg(Reg::Unknown) => 0,
            Access::StreamReg(i, offset) => self.streams[i].read(&self.name, sys, offset),
        }
    }

    fn write(&mut self, sys: &System, offset: u32, value: u32) {
        match Access::from_offset(offset) {
            Access::Reg(Reg::Lifcr) => {
                Self::clear_ifcr_bank(&mut self.lisr, 0, value);
            }
            Access::Reg(Reg::Hifcr) => {
                Self::clear_ifcr_bank(&mut self.hisr, 4, value);
            }
            Access::Reg(_) => {}
            Access::StreamReg(i, offset) => {
                if offset == 0x0000 && (value & 1) != 0 {
                    let channel = ((value >> 25) & 0b111) as u8;
                    let par = self.streams[i].par;
                    let peri_desc = sys.p.addr_desc(par);
                    let peri_name = peripheral_name_from_desc(&peri_desc);
                    let new_dir = match (value >> 6) & 0b11 {
                        0b00 => Dir::Read,
                        0b01 => Dir::Write,
                        0b10 => Dir::MemCopy,
                        _ => Dir::Invalid,
                    };
                    let new_pl = ((value >> 16) & 0b11) as u8;

                    if new_dir == Dir::MemCopy || par == 0 {
                        match self.streams[i].write(&self.name, i, sys, offset, value) {
                            StreamWriteResult::Completed { half } => {
                                if half {
                                    self.signal_ht(sys, i);
                                }
                                self.signal_tc(sys, i);
                            }
                            StreamWriteResult::FifoError => {
                                self.signal_fe(sys, i);
                            }
                            StreamWriteResult::ModeError => {
                                self.signal_mode_error(sys, i);
                            }
                            StreamWriteResult::TransferError => {
                                self.signal_te(sys, i);
                            }
                            StreamWriteResult::Noop => {}
                        }
                        return;
                    }

                    let owners = self.find_request_conflicts(i, channel);
                    let mut blocking_owners = Vec::new();
                    let mut preempted_owners = Vec::new();
                    for owner in owners {
                        if !self.streams[owner].ready_for_step(sys) {
                            // Keep enable-time arbitration aligned with deferred runtime:
                            // sleeping/deferred owners do not block ready contenders.
                            continue;
                        }

                        let owner_peri_desc = sys.p.addr_desc(self.streams[owner].par);
                        let owner_peri_name = peripheral_name_from_desc(&owner_peri_desc);
                        let same_peripheral_request = same_peripheral_request_target(
                            sys,
                            self.streams[i].par,
                            peri_name,
                            self.streams[owner].par,
                            owner_peri_name,
                        );

                        if !same_peripheral_request {
                            continue;
                        }

                        let owner_dir = self.streams[owner].dir();
                        let owner_pl = self.streams[owner].priority();
                        // Keep opposite-direction stream sharing for peripherals with
                        // independent RX/TX request paths.
                        let full_duplex_pair = supports_bidirectional_dma_sharing(peri_name)
                            && ((new_dir == Dir::Read && owner_dir == Dir::Write)
                                || (new_dir == Dir::Write && owner_dir == Dir::Read));
                        if full_duplex_pair {
                            continue;
                        }

                        // Priority-aware conflict resolution for same request owner:
                        // - Higher owner priority blocks this stream.
                        // - Lower owner priority is preempted by disabling EN.
                        // - Equal priority uses stream index as tiebreaker (lower stream wins).
                        if owner_pl > new_pl {
                            blocking_owners.push(owner);
                        } else if owner_pl < new_pl {
                            preempted_owners.push(owner);
                        } else {
                            if self.starvation_age[i] > self.starvation_age[owner] {
                                preempted_owners.push(owner);
                                continue;
                            }
                            let owner_rank = (owner + 8 - (self.arb_cursor % 8)) % 8;
                            let new_rank = (i + 8 - (self.arb_cursor % 8)) % 8;
                            if owner_rank < new_rank {
                                blocking_owners.push(owner);
                            } else {
                                preempted_owners.push(owner);
                            }
                        }

                    }

                    if !blocking_owners.is_empty() {
                        self.signal_mode_error(sys, i);
                        debug!(
                            "{} stream={} blocked conflicting request channel={} peri={} (pl {})",
                            self.name,
                            i,
                            channel,
                            peri_desc,
                            new_pl
                        );
                        return;
                    }

                    for owner in preempted_owners {
                        self.streams[owner].cr &= !1;
                        self.streams[owner].next_cr = None;
                        self.streams[owner].deferred_usart_rx = false;
                        self.streams[owner].disable_requested_at = None;
                        debug!(
                            "{} stream={} preempted lower-priority owner stream={} channel={} peri={} (new_pl={} owner_pl={})",
                            self.name,
                            i,
                            owner,
                            channel,
                            peri_desc,
                            new_pl,
                            self.streams[owner].priority()
                        );
                    }
                }

                match self.streams[i].write(&self.name, i, sys, offset, value) {
                    StreamWriteResult::Completed { half } => {
                        if half {
                            self.signal_ht(sys, i);
                        }
                        self.signal_tc(sys, i);
                    }
                    StreamWriteResult::FifoError => {
                        self.signal_fe(sys, i);
                    }
                    StreamWriteResult::ModeError => {
                        self.signal_mode_error(sys, i);
                    }
                    StreamWriteResult::TransferError => {
                        self.signal_te(sys, i);
                    }
                    StreamWriteResult::Noop => {}
                }
            }
        }
    }
}

#[derive(Default)]
struct Stream {
    pub cr: u32,
    pub next_cr: Option<u32>,
    pub ndtr: u32,
    pub initial_ndtr: u32,
    pub par: u32,
    pub m0ar: u32,
    pub m1ar: u32,
    pub fcr: u32,
    deferred_usart_rx: bool,
    deferred_since: u64,
    disable_requested_at: Option<u64>,
    // FIFO state tracking: bytes filled and occupancy classification.
    fifo_bytes: usize,
    fifo_occupancy: FifoOccupancy,
    fifo_error_pending: bool,
}

impl Stream {
    fn defer_delay_for_desc(&self, peri_desc: &str) -> u64 {
        if self.pfctrl() {
            0
        } else if is_usart_dr_request(peri_desc) {
            USART_RX_IDLE_DISABLE_DELAY
        } else if self.fifo_enabled() && self.dir() != Dir::MemCopy {
            // FIFO-enabled peripheral streams are request paced and should be serviced
            // promptly in arbitration rounds rather than waiting on SDIO-style idle windows.
            0
        } else {
            SDIO_DMA_DEFER_DELAY
        }
    }

    fn ready_for_step(&self, sys: &System) -> bool {
        if (self.cr & 1) == 0 || self.ndtr == 0 {
            return false;
        }

        if self.disable_requested_at.is_some() {
            return false;
        }

        if !self.deferred_usart_rx {
            return true;
        }

        let now = NUM_INSTRUCTIONS.load(std::sync::atomic::Ordering::Relaxed);
        let peri_desc = sys.p.addr_desc(self.par);
        let defer_delay = self.defer_delay_for_desc(&peri_desc);

        now.saturating_sub(self.deferred_since) >= defer_delay
    }

    fn tcie_enabled(&self) -> bool {
        self.cr & (1 << 4) != 0
    }

    fn htie_enabled(&self) -> bool {
        self.cr & (1 << 3) != 0
    }

    fn teie_enabled(&self) -> bool {
        self.cr & (1 << 2) != 0
    }

    fn dmeie_enabled(&self) -> bool {
        self.cr & (1 << 1) != 0
    }

    fn feie_enabled(&self) -> bool {
        self.fcr & (1 << 7) != 0
    }

    fn fifo_enabled(&self) -> bool {
        self.fcr & (1 << 2) != 0
    }

    fn channel(&self) -> u8 {
        ((self.cr >> 25) & 0b111) as u8
    }

    fn priority(&self) -> u8 {
        ((self.cr >> 16) & 0b11) as u8
    }

    fn dir(&self) -> Dir {
        match (self.cr >> 6) & 0b11 {
            0b00 => Dir::Read,
            0b01 => Dir::Write,
            0b10 => Dir::MemCopy,
            _ => Dir::Invalid,
        }
    }

    fn pfctrl(&self) -> bool {
        self.cr & (1 << 5) != 0
    }

    fn is_circular(&self) -> bool {
        self.cr & (1 << 8) != 0
    }

    fn is_double_buffer(&self) -> bool {
        self.cr & (1 << 18) != 0
    }

    fn minc(&self) -> bool {
        self.cr & (1 << 10) != 0
    }

    fn pburst(&self) -> u8 {
        ((self.cr >> 21) & 0b11) as u8
    }

    fn mburst(&self) -> u8 {
        ((self.cr >> 23) & 0b11) as u8
    }

    fn burst_beats(field: u8) -> usize {
        match field & 0b11 {
            0b00 => 1,
            0b01 => 4,
            0b10 => 8,
            0b11 => 16,
            _ => 1,
        }
    }

    fn pinc(&self) -> bool {
        self.cr & (1 << 9) != 0
    }

    // Peripheral data size in bytes (PSIZE field, SxCR bits [12:11]).
    fn psize(&self) -> usize {
        match (self.cr >> 11) & 0b11 {
            0b00 => 1,
            0b01 => 2,
            0b10 => 4,
            _ => 1,
        }
    }

    // Memory data size in bytes (MSIZE field, SxCR bits [14:13]).
    fn msize(&self) -> usize {
        match (self.cr >> 13) & 0b11 {
            0b00 => 1,
            0b01 => 2,
            0b10 => 4,
            _ => 1,
        }
    }

    fn data_addr(&self) -> u32 {
        if (self.cr >> 19) & 1 != 0 {
            self.m1ar
        } else {
            self.m0ar
        }
    }

    /// Service EN disable request: if disable was requested and enough cycles have passed,
    /// clear the EN bit (set bit 0 to 0) in CR. This enforces STM32F4 timing for EN transitions.
    fn service_disable_delay(&mut self) {
        if let Some(disable_at) = self.disable_requested_at {
            let now = NUM_INSTRUCTIONS.load(std::sync::atomic::Ordering::Relaxed);
            if now.saturating_sub(disable_at) >= DMA_EN_DISABLE_DELAY {
                self.cr &= !1;  // Clear EN (bit 0)
                self.disable_requested_at = None;
            }
        }
    }

    fn fifo_threshold_words(&self) -> usize {
        // FTH (SxFCR bits [1:0]): 1/4, 1/2, 3/4, full of 4-word FIFO.
        match self.fcr & 0b11 {
            0b00 => 1,
            0b01 => 2,
            0b10 => 3,
            0b11 => 4,
            _ => 1,
        }
    }

    fn fifo_threshold_bytes(&self) -> usize {
        self.fifo_threshold_words() * 4
    }

    fn transfer_beats_per_chunk(&self) -> usize {
        if self.pfctrl() {
            // Peripheral-flow-controller mode should pace transfers in smaller slices.
            return 1;
        }

        if self.fifo_enabled() && self.dir() != Dir::MemCopy {
            // Peripheral request-driven streams are paced beat-by-beat so FIFO fill/drain
            // progresses across arbitration rounds instead of draining an entire threshold
            // window in a single service pass.
            return 1;
        }

        if !self.fifo_enabled() {
            // Direct mode: effectively one beat per request window.
            return 1;
        }

        // FIFO mode: transfer one FIFO threshold worth of beats at a time.
        // FIFO is 4 words = 16 bytes.
        let fifo_bytes = self.fifo_threshold_bytes();
        let beat_bytes = std::cmp::max(self.psize(), self.msize());
        std::cmp::max(1, fifo_bytes / std::cmp::max(1, beat_bytes))
    }

    /// Check if a transfer would violate FIFO threshold constraints.
    /// For Read (P2M): check if incoming data would exceed FIFO capacity.
    /// For Write (M2P): check if FIFO has enough data for the requested transfer.
    /// Returns true if threshold would be violated (should signal FE).
    fn would_violate_fifo_threshold(&self, beats_to_transfer: usize) -> bool {
        if !self.fifo_enabled() || beats_to_transfer == 0 {
            return false;
        }

        let beat_bytes = std::cmp::max(self.psize(), self.msize());
        let incoming_bytes = beats_to_transfer * beat_bytes;

        // FIFO threshold (FTH) is not the FIFO capacity. Capacity is fixed at 16 bytes.
        // We treat oversized chunk requests as FIFO errors while allowing smaller-than-threshold
        // chunks to proceed (threshold is a service watermark, not a hard minimum transfer size).
        incoming_bytes > DMA_FIFO_CAPACITY_BYTES
    }

    fn fifo_status_bits(&self) -> u32 {
        // FS encoding (SxFCR bits [5:3]):
        // 000 <1/4, 001 1/4, 010 1/2, 011 3/4, 100 empty, 101 full
        if !self.fifo_enabled() || (self.cr & 1) == 0 || self.ndtr == 0 {
            return 0b100;
        }

        use FifoOccupancy::*;
        match self.fifo_occupancy {
            Empty => 0b100,
            Quarter | OneQuarter => 0b000,
            Half => 0b001,
            ThreeQuarters => 0b010,
            Full => 0b101,
        }
    }

    fn update_fifo_occupancy(&mut self) {
        self.fifo_occupancy = FifoOccupancy::from_bytes(self.fifo_bytes, DMA_FIFO_CAPACITY_BYTES);
    }

    /// Perform one complete DMA transfer respecting PINC/MINC.
    /// For P2M (Read): NDTR beats, each reading psize bytes from peripheral (PINC) into
    ///   msize bytes at memory (MINC). Total bytes: psize*NDTR from peri, msize*NDTR to mem.
    /// For M2P (Write): NDTR beats, reading msize bytes from memory (MINC), writing psize
    ///   bytes to peripheral (PINC).
    /// For MemCopy: source and destination both increment by msize per beat.
    fn do_xfer(&mut self, dma_name: &str, stream_idx: usize, sys: &System, max_beats: Option<usize>) -> XferOutcome {
        let dir = self.dir();
        let mut mem_addr = self.data_addr();
        let mut peri_addr = self.par;
        let ndtr = self.ndtr as usize;
        let psize = self.psize();
        let msize = self.msize();
        let minc = self.minc();
        let pinc = self.pinc();

        if ndtr == 0 {
            return XferOutcome { ok: false, half: false };
        }

        let peri = Peripherals::get_peripheral(&sys.p.peripherals, peri_addr);
        let peri_desc = sys.p.addr_desc(peri_addr);
        let peri_name = peripheral_name_from_desc(&peri_desc);

        if !request_mapping_allows(dma_name, stream_idx as u8, self.channel(), peri_name, dir) {
            warn!(
                "{} stream={} channel={} blocked request mapping for {} dir={:?}",
                dma_name,
                stream_idx,
                self.channel(),
                peri_desc,
                dir
            );
            return XferOutcome { ok: false, half: false };
        }

        let mut ok = true;
        let chunk_beats = self.transfer_beats_per_chunk();

        // Check FIFO threshold constraints before starting transfer
        if self.would_violate_fifo_threshold(chunk_beats) {
            self.fifo_error_pending = true;
            debug!(
                "{} stream={} FIFO threshold violation: would_violate_threshold={} fifo_bytes={} dir={:?}",
                dma_name,
                stream_idx,
                chunk_beats,
                self.fifo_bytes,
                dir
            );
            // Signal FEIF but still attempt transfer (RM behavior)
            return XferOutcome { ok: false, half: false };
        }

        if log::log_enabled!(log::Level::Debug) {
            debug!("{} xfer channel={} peri_{} dir={:?} mem=0x{:08x} ndtr={} psize={} msize={} circ={} minc={} pinc={} fifo={} fth_words={} chunk_beats={}",
                dma_name, self.channel(), peri_desc, dir, mem_addr, ndtr,
                psize, msize, self.is_circular(), minc, pinc, self.fifo_enabled(), self.fifo_threshold_words(), chunk_beats);
        }

        let mut remaining_beats = ndtr;
        let mut remaining_budget = max_beats.unwrap_or(usize::MAX);
        while remaining_beats > 0 {
            if remaining_budget == 0 {
                break;
            }

            let beats = std::cmp::min(remaining_beats, chunk_beats);
            let beats = std::cmp::min(beats, remaining_budget);
            let peri_total = psize * beats;
            let mem_total = msize * beats;
            let fifo_chunk_bytes = beats * std::cmp::max(psize, msize);

            if self.fifo_enabled() {
                // Model FIFO fill/drain around each serviced chunk instead of forcing
                // a fixed occupancy snapshot. This keeps FS transitions direction-aware.
                match dir {
                    Dir::Read => {
                        self.fifo_bytes = std::cmp::min(
                            DMA_FIFO_CAPACITY_BYTES,
                            self.fifo_bytes.saturating_add(fifo_chunk_bytes),
                        );
                    }
                    Dir::Write => {
                        self.fifo_bytes = std::cmp::min(
                            DMA_FIFO_CAPACITY_BYTES,
                            self.fifo_bytes.saturating_add(fifo_chunk_bytes),
                        );
                    }
                    Dir::MemCopy | Dir::Invalid => {}
                }
            }

            match dir {
                Dir::Read => {
                    // P2M: read from peripheral into memory
                    let mut buf = if pinc {
                        let mut v = std::collections::VecDeque::with_capacity(peri_total);
                        for beat in 0..beats {
                            let beat_addr = peri_addr + (beat * psize) as u32;
                            let beat_offset = beat_addr - peri_addr;
                            if let Some(p) = peri {
                                let mut slice = p.peripheral.borrow_mut().read_dma(sys, beat_offset, psize);
                                v.extend(slice.drain(..));
                            } else {
                                v.extend(std::iter::repeat(0u8).take(psize));
                            }
                        }
                        if let Some(p) = peri {
                            p.peripheral.borrow_mut().set_dma_rx_dest(mem_addr);
                        }
                        v
                    } else {
                        let peri_offset = peri_addr - peri.map(|p| p.start).unwrap_or(peri_addr);
                        let b = peri.map(|p| {
                            p.peripheral.borrow_mut().set_dma_rx_dest(mem_addr);
                            p.peripheral.borrow_mut().read_dma(sys, peri_offset, peri_total)
                        });
                        b.unwrap_or_else(|| {
                            let mut v = std::collections::VecDeque::new();
                            v.extend(std::iter::repeat(0u8).take(peri_total));
                            v
                        })
                    };

                    if minc {
                        if let Err(e) = sys.uc.borrow_mut().mem_write(mem_addr.into(), buf.make_contiguous()) {
                            warn!("DMA P2M write failed addr=0x{:08x} size={} e={}", mem_addr, mem_total, UniErr(e));
                            ok = false;
                        }
                    } else {
                        let bytes = buf.make_contiguous();
                        if let Err(e) = sys.uc.borrow_mut().mem_write(mem_addr.into(), &bytes[bytes.len().saturating_sub(msize)..]) {
                            warn!("DMA P2M write (MINC=0) failed addr=0x{:08x} e={}", mem_addr, UniErr(e));
                            ok = false;
                        }
                    }

                    if self.fifo_enabled() {
                        // After draining FIFO to memory, retain a small non-zero occupancy
                        // between deferred slices to expose intermediate FS states.
                        let drained = std::cmp::min(self.fifo_bytes, fifo_chunk_bytes);
                        self.fifo_bytes = self.fifo_bytes.saturating_sub(drained);
                        if self.ndtr > beats as u32 && self.fifo_bytes == 0 {
                            self.fifo_bytes = std::cmp::min(DMA_FIFO_CAPACITY_BYTES, std::cmp::max(1, psize));
                        }
                        self.update_fifo_occupancy();
                    }
                }

                Dir::Write => {
                    // M2P: read from memory into peripheral
                    let peri_offset = peri_addr - peri.map(|p| p.start).unwrap_or(peri_addr);

                    let buf = if minc {
                        sys.uc.borrow().mem_read_as_vec(mem_addr.into(), mem_total)
                            .map_err(|e| warn!("DMA M2P read failed addr=0x{:08x} size={} e={}", mem_addr, mem_total, UniErr(e)))
                            .map(|v| v.into())
                            .ok()
                    } else {
                        sys.uc.borrow().mem_read_as_vec(mem_addr.into(), msize)
                            .ok()
                            .map(|v| {
                                let mut out = std::collections::VecDeque::new();
                                for _ in 0..beats {
                                    out.extend(v.iter());
                                }
                                out
                            })
                    };

                    let buf = buf.unwrap_or_else(|| {
                        ok = false;
                        let mut v = std::collections::VecDeque::new();
                        v.extend(std::iter::repeat(0u8).take(mem_total));
                        v
                    });

                    if pinc {
                        let bytes: Vec<u8> = buf.into_iter().collect();
                        for beat in 0..beats {
                            let beat_offset = peri_offset + (beat * psize) as u32;
                            let start = beat * psize;
                            let end = (start + psize).min(bytes.len());
                            let slice = &bytes[start..end];
                            if let Some(p) = peri {
                                p.peripheral.borrow_mut().write_dma(sys, beat_offset, slice.iter().copied().collect());
                            } else {
                                ok = false;
                            }
                        }
                    } else if let Some(p) = peri {
                        p.peripheral.borrow_mut().write_dma(sys, peri_offset, buf);
                    } else {
                        ok = false;
                    }

                    if self.fifo_enabled() {
                        let drained = std::cmp::min(self.fifo_bytes, fifo_chunk_bytes);
                        self.fifo_bytes = self.fifo_bytes.saturating_sub(drained);
                        if self.ndtr > beats as u32 && self.fifo_bytes == 0 {
                            self.fifo_bytes = std::cmp::min(DMA_FIFO_CAPACITY_BYTES, std::cmp::max(1, msize));
                        }
                        self.update_fifo_occupancy();
                    }
                }

                Dir::MemCopy => {
                    let src = peri_addr;
                    let dst = mem_addr;

                    let buf = sys.uc.borrow().mem_read_as_vec(src.into(), mem_total)
                        .map_err(|e| warn!("DMA MemCopy read failed src=0x{:08x} size={} e={}", src, mem_total, UniErr(e)))
                        .ok();

                    if let Some(buf) = buf {
                        if let Err(e) = sys.uc.borrow_mut().mem_write(dst.into(), &buf) {
                            warn!("DMA MemCopy write failed dst=0x{:08x} size={} e={}", dst, mem_total, UniErr(e));
                            ok = false;
                        }
                    } else {
                        ok = false;
                    }
                }

                Dir::Invalid => {}
            }

            if minc {
                mem_addr = mem_addr.wrapping_add((beats * msize) as u32);
            }
            if pinc {
                peri_addr = peri_addr.wrapping_add((beats * psize) as u32);
            }

            remaining_beats -= beats;
            remaining_budget -= beats;
            self.ndtr = remaining_beats as u32;
        }

        // Update FIFO byte tracking after transfer
        if ok && self.fifo_enabled() {
            if self.ndtr == 0 {
                self.fifo_bytes = 0;
                self.update_fifo_occupancy();
            }
        }

        let half_threshold = self.initial_ndtr / 2;
        let half = self.initial_ndtr > 1
            && (ndtr as u32) > half_threshold
            && self.ndtr <= half_threshold;

        XferOutcome { ok, half }
    }

    pub fn read(&mut self, _name: &str, _sys: &System, offset: u32) -> u32 {
        // Service disable delay before reading CR to ensure EN bit is properly cleared
        if offset == 0x0000 {
            self.service_disable_delay();
        }

        match offset {
            0x0000 => {
                let v = self.cr;
                if let Some(next_cr) = self.next_cr.take() {
                    self.cr = next_cr;
                }

                // The saturn firmware is a bit buggy. When doing a DMA write
                // with size=0, they don't enable the DMA channel, but they
                // wait for it to go to 1 and then 0, with a timeout. So they
                // are consistently hitting the timeout.
                // We'll do toggles on the ready flag to speed things up avoiding the timeout.
                v
            }
            0x0004 => self.ndtr,
            0x0008 => self.par,
            0x000c => self.m0ar,
            0x0010 => self.m1ar,
            0x0014 => {
                // FCR: return stored value but update FIFO status bits [5:3].
                // FS=100 (FIFO empty) when stream is idle; FS=001 (quarter full) when
                // a transfer recently completed (data may still be in flight).
                let fs = self.fifo_status_bits();
                (self.fcr & !(0b111 << 3)) | (fs << 3)
            }
            _ => 0
        }
    }

    pub fn write(&mut self, dma_name: &str, stream_idx: usize, sys: &System, offset: u32, mut value: u32) -> StreamWriteResult {
        match offset {
            0x0000 => {
                // Service any pending disable delay before processing new CR write
                self.service_disable_delay();

                let was_enabled = self.cr & 1 != 0;
                let new_enabled = value & 1 != 0;

                // If EN=0 is being set, record disable request timestamp
                if was_enabled && !new_enabled {
                    self.disable_requested_at = Some(NUM_INSTRUCTIONS.load(std::sync::atomic::Ordering::Relaxed));
                    self.cr = value;
                    self.deferred_usart_rx = false;
                    self.fifo_bytes = 0;
                    self.update_fifo_occupancy();
                    return StreamWriteResult::Noop;
                }

                // If EN=1 and disable is pending, block the write
                if new_enabled && self.disable_requested_at.is_some() {
                    return StreamWriteResult::Noop;
                }

                self.cr = value;

                // EN set while already enabled should not retrigger a new transfer.
                if new_enabled && was_enabled {
                    return StreamWriteResult::Noop;
                }

                // EN clear request (without pending disable) - shouldn't reach here but be safe
                if !new_enabled {
                    self.deferred_usart_rx = false;
                    self.fifo_bytes = 0;
                    self.update_fifo_occupancy();
                    return StreamWriteResult::Noop;
                }

                self.deferred_usart_rx = false;

                if self.dir() == Dir::Invalid {
                    return StreamWriteResult::ModeError;
                }

                // In STM32F4 direct mode, burst transfers require FIFO mode.
                // Reject this configuration and route to DME/FE signaling path.
                if !self.fifo_enabled() && (self.pburst() != 0 || self.mburst() != 0) {
                    return StreamWriteResult::ModeError;
                }

                if self.pfctrl() {
                    // PFCTRL mode is peripheral-driven and not valid for memory-to-memory
                    // or direct mode operation in this model.
                    if self.dir() == Dir::MemCopy || !self.fifo_enabled() {
                        return StreamWriteResult::ModeError;
                    }
                }

                // Burst transfers require address incrementing on the corresponding side.
                if (self.pburst() != 0 && !self.pinc()) || (self.mburst() != 0 && !self.minc()) {
                    return StreamWriteResult::ModeError;
                }

                // Basic burst consistency check: transfer length should align to burst beats.
                let burst_beats = std::cmp::max(Self::burst_beats(self.pburst()), Self::burst_beats(self.mburst()));
                if burst_beats > 1 && (self.ndtr as usize) % burst_beats != 0 {
                    return StreamWriteResult::ModeError;
                }

                if self.is_deferred_peripheral_rx(sys) {
                    self.deferred_usart_rx = true;
                    self.deferred_since = NUM_INSTRUCTIONS.load(std::sync::atomic::Ordering::Relaxed);
                    return StreamWriteResult::Noop;
                }

                let ok = self.do_xfer(dma_name, stream_idx, sys, None);
                
                // If FIFO threshold was violated, signal FE and return error
                if self.fifo_error_pending {
                    self.fifo_error_pending = false;
                    return StreamWriteResult::FifoError;
                }
                
                // HT should fire if initial_ndtr > 1 (multi-beat transfer crosses half-way point)
                let half = ok.half;

                if self.is_double_buffer() {
                    // DBM: toggle CT (bit 19) to switch between M0AR and M1AR, reload NDTR
                    self.cr ^= 1 << 19;
                    self.ndtr = self.initial_ndtr;
                } else if self.is_circular() {
                    // Circular without DBM: reload NDTR, keep same buffer
                    self.ndtr = self.initial_ndtr;
                } else {
                    value &= !1;
                    self.ndtr = 0;
                    self.next_cr = Some(value);
                }
                if ok.ok {
                    return StreamWriteResult::Completed { half };
                }
                return StreamWriteResult::TransferError;
            }
            0x0004 => {
                if (self.cr & 1) != 0 {
                    return StreamWriteResult::Noop;
                }
                self.ndtr = value & 0xFFFF;
                self.initial_ndtr = self.ndtr;
            }
            0x0008 => {
                if (self.cr & 1) != 0 {
                    return StreamWriteResult::Noop;
                }
                self.par = value;
            }
            0x000c => {
                if (self.cr & 1) != 0 {
                    return StreamWriteResult::Noop;
                }
                self.m0ar = value;
            }
            0x0010 => {
                if (self.cr & 1) != 0 {
                    return StreamWriteResult::Noop;
                }
                self.m1ar = value;
            }
            0x0014 => {
                if (self.cr & 1) != 0 {
                    return StreamWriteResult::Noop;
                }
                // Writable bits: FEIE(7), DMDIS(2), FTH(1:0). FS is read-only.
                self.fcr = value & 0x87;
            }
            _ => {}
        }

        StreamWriteResult::Noop
    }

    fn is_deferred_peripheral_rx(&self, sys: &System) -> bool {
        if self.pfctrl() {
            return true;
        }

        // FIFO-enabled peripheral transfers are request-paced in hardware.
        if self.fifo_enabled() && self.dir() != Dir::MemCopy {
            return true;
        }

        if self.dir() != Dir::Read {
            return false;
        }
        let peri_desc = sys.p.addr_desc(self.par);
        is_usart_dr_request(&peri_desc) || is_sdio_fifo_request(&peri_desc)
    }

    /// Called from Dma::step(). Returns true if a transfer completed and TC should be signaled.
    fn step_deferred(&mut self, dma_name: &str, stream_idx: usize, sys: &System) -> StreamStepResult {
        if !self.deferred_usart_rx || self.cr & 1 == 0 {
            return StreamStepResult::Noop;
        }

        let now = NUM_INSTRUCTIONS.load(std::sync::atomic::Ordering::Relaxed);
        let peri_desc = sys.p.addr_desc(self.par);
        let defer_delay = self.defer_delay_for_desc(&peri_desc);
        if now.saturating_sub(self.deferred_since) < defer_delay {
            return StreamStepResult::Noop;
        }

        // Idle window expired: perform the transfer (reads available bytes from USART ext_device)
        // then decide based on circular mode whether to reload or finish.
        let chunk_beats = self.transfer_beats_per_chunk();
        let ok = self.do_xfer(dma_name, stream_idx, sys, Some(chunk_beats));

        if self.fifo_error_pending {
            self.fifo_error_pending = false;
            self.deferred_usart_rx = false;
            return StreamStepResult::FifoError;
        }

        // HT fires if multi-beat transfer (initial_ndtr > 1)
        let half = ok.half;

        if !ok.ok {
            self.deferred_usart_rx = false;
            return StreamStepResult::TransferError;
        }

        if self.ndtr > 0 {
            self.deferred_usart_rx = true;
            self.deferred_since = now;
            return StreamStepResult::Progress { half };
        }

        self.deferred_usart_rx = false;

        if self.is_double_buffer() {
            self.cr ^= 1 << 19;
            self.ndtr = self.initial_ndtr;
            self.deferred_usart_rx = true;
            self.deferred_since = now;
        } else if self.is_circular() {
            self.ndtr = self.initial_ndtr;
            self.deferred_usart_rx = true;
            self.deferred_since = now;
        } else {
            self.cr &= !1;
            self.ndtr = 0;
            self.next_cr = None;
        }

        StreamStepResult::Completed { half }
    }
}

struct XferOutcome {
    ok: bool,
    half: bool,
}

enum StreamWriteResult {
    Noop,
    Completed { half: bool },
    FifoError,
    TransferError,
    ModeError,
}

enum StreamStepResult {
    Noop,
    Progress { half: bool },
    Completed { half: bool },
    FifoError,
    TransferError,
}

fn is_usart_dr_request(peri_desc: &str) -> bool {
    (peri_desc.contains("peri=USART") || peri_desc.contains("peri=UART")) && peri_desc.contains("reg=DR")
}

fn is_sdio_fifo_request(peri_desc: &str) -> bool {
    peri_desc.contains("peri=SDIO") && peri_desc.contains("reg=FIFO")
}

fn peripheral_name_from_desc(desc: &str) -> Option<&str> {
    let (_, tail) = desc.split_once("peri=")?;
    Some(tail.split_whitespace().next().unwrap_or_default())
}

fn supports_bidirectional_dma_sharing(peri_name: Option<&str>) -> bool {
    match peri_name {
        Some(name)
            if name.starts_with("SPI")
                || name.starts_with("USART")
                || name.starts_with("UART")
                || name.starts_with("I2C") => true,
        _ => false,
    }
}

fn same_peripheral_request_target(
    sys: &System,
    par_a: u32,
    name_a: Option<&str>,
    par_b: u32,
    name_b: Option<&str>,
) -> bool {
    if let (Some(a), Some(b)) = (name_a, name_b) {
        return a == b;
    }

    let slot_a = Peripherals::get_peripheral(&sys.p.peripherals, par_a).map(|p| p.start);
    let slot_b = Peripherals::get_peripheral(&sys.p.peripherals, par_b).map(|p| p.start);
    match (slot_a, slot_b) {
        (Some(a), Some(b)) => a == b,
        _ => par_a == par_b,
    }
}

fn request_mapping_allows(
    dma_name: &str,
    stream: u8,
    channel: u8,
    peri_name: Option<&str>,
    dir: Dir,
) -> bool {
    let Some(peri_name) = peri_name else {
        return true;
    };

    let req = match (peri_name, dir) {
        ("ADC1", Dir::Read) => Some(&[("DMA2", 0u8, 0u8), ("DMA2", 4, 0)][..]),
        ("ADC2", Dir::Read) => Some(&[("DMA2", 2u8, 1u8), ("DMA2", 3, 1)][..]),
        ("ADC3", Dir::Read) => Some(&[("DMA2", 0u8, 2u8), ("DMA2", 1, 2)][..]),

        ("SPI1", Dir::Read) => Some(&[("DMA2", 0u8, 3u8), ("DMA2", 2, 3)][..]),
        ("SPI1", Dir::Write) => Some(&[("DMA2", 3u8, 3u8), ("DMA2", 5, 3)][..]),
        ("SPI2", Dir::Read) => Some(&[("DMA1", 3u8, 0u8)][..]),
        ("SPI2", Dir::Write) => Some(&[("DMA1", 4u8, 0u8)][..]),
        ("SPI3", Dir::Read) => Some(&[("DMA1", 0u8, 0u8), ("DMA1", 2, 0)][..]),
        ("SPI3", Dir::Write) => Some(&[("DMA1", 5u8, 0u8), ("DMA1", 7, 0)][..]),

        ("USART1", Dir::Read) => Some(&[("DMA2", 2u8, 4u8), ("DMA2", 5, 4)][..]),
        ("USART1", Dir::Write) => Some(&[("DMA2", 7u8, 4u8)][..]),
        ("USART2", Dir::Read) => Some(&[("DMA1", 5u8, 4u8)][..]),
        ("USART2", Dir::Write) => Some(&[("DMA1", 6u8, 4u8)][..]),
        ("USART3", Dir::Read) => Some(&[("DMA1", 1u8, 4u8)][..]),
        ("USART3", Dir::Write) => Some(&[("DMA1", 3u8, 4u8), ("DMA1", 4, 7)][..]),
        ("UART4", Dir::Read) => Some(&[("DMA1", 2u8, 4u8)][..]),
        ("UART4", Dir::Write) => Some(&[("DMA1", 4u8, 4u8)][..]),
        ("UART5", Dir::Read) => Some(&[("DMA1", 0u8, 4u8)][..]),
        ("UART5", Dir::Write) => Some(&[("DMA1", 7u8, 4u8)][..]),
        ("USART6", Dir::Read) => Some(&[("DMA2", 1u8, 5u8), ("DMA2", 2, 5)][..]),
        ("USART6", Dir::Write) => Some(&[("DMA2", 6u8, 5u8), ("DMA2", 7, 5)][..]),

        ("I2C1", Dir::Read) => Some(&[("DMA1", 0u8, 1u8), ("DMA1", 5, 1)][..]),
        ("I2C1", Dir::Write) => Some(&[("DMA1", 6u8, 1u8), ("DMA1", 7, 1)][..]),
        ("I2C2", Dir::Read) => Some(&[("DMA1", 2u8, 7u8), ("DMA1", 3, 7)][..]),
        ("I2C2", Dir::Write) => Some(&[("DMA1", 7u8, 7u8)][..]),
        ("I2C3", Dir::Read) => Some(&[("DMA1", 2u8, 3u8)][..]),
        ("I2C3", Dir::Write) => Some(&[("DMA1", 4u8, 3u8)][..]),

        // SDIO can be used in both directions through the same request entries.
        ("SDIO", _) => Some(&[("DMA2", 3u8, 4u8), ("DMA2", 6, 4)][..]),

        // Keep unsupported peripheral request IDs permissive for now.
        _ => None,
    };

    req.map(|entries| {
        entries.iter().any(|(dma, s, ch)| dma_name == *dma && stream == *s && channel == *ch)
    }).unwrap_or(true)
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Dir {
    Read,
    Write,
    MemCopy,
    Invalid,
}

#[derive(Debug, Clone, Copy)]
enum Reg {
    Lisr,
    Hisr,
    Lifcr,
    Hifcr,
    Unknown,
}

enum Access {
    Reg(Reg),
    StreamReg(usize, u32),
}

fn tcif_mask(stream: usize) -> u32 {
    // STM32F4 DMA status flag layout in LISR/HISR repeats for stream groups.
    // Per stream, TCIF is at bit offset +5 inside each group.
    match stream % 4 {
        0 => 1 << 5,
        1 => 1 << 11,
        2 => 1 << 21,
        3 => 1 << 27,
        _ => 0,
    }
}

fn htif_mask(stream: usize) -> u32 {
    match stream % 4 {
        0 => 1 << 4,
        1 => 1 << 10,
        2 => 1 << 20,
        3 => 1 << 26,
        _ => 0,
    }
}

fn teif_mask(stream: usize) -> u32 {
    match stream % 4 {
        0 => 1 << 3,
        1 => 1 << 9,
        2 => 1 << 19,
        3 => 1 << 25,
        _ => 0,
    }
}

fn dmeif_mask(stream: usize) -> u32 {
    match stream % 4 {
        0 => 1 << 2,
        1 => 1 << 8,
        2 => 1 << 18,
        3 => 1 << 24,
        _ => 0,
    }
}

fn feif_mask(stream: usize) -> u32 {
    match stream % 4 {
        0 => 1 << 0,
        1 => 1 << 6,
        2 => 1 << 16,
        3 => 1 << 22,
        _ => 0,
    }
}

fn gif_mask(stream: usize) -> u32 {
    match stream % 4 {
        0 => 1 << 1,
        1 => 1 << 7,
        2 => 1 << 17,
        3 => 1 << 23,
        _ => 0,
    }
}

fn stream_flag_mask(stream: usize) -> u32 {
    feif_mask(stream) | dmeif_mask(stream) | teif_mask(stream) | htif_mask(stream) | tcif_mask(stream)
}

impl Access {
    pub fn from_offset(offset: u32) -> Self {
        // Global DMA registers are at 0x00..0x0f (LISR/HISR/LIFCR/HIFCR).
        // Stream registers start at 0x10 and are 0x18 bytes apart.
        if offset < 0x10 {
            let reg = match offset {
                0x00 => Reg::Lisr,
                0x04 => Reg::Hisr,
                0x08 => Reg::Lifcr,
                0x0c => Reg::Hifcr,
                _ => Reg::Unknown,
            };
            Access::Reg(reg)
        } else {
            let stride = 0x18;
            let start = 0x10;

            let offset = offset - start;
            let stream = (offset / stride) as usize;
            if stream < 8 {
                Access::StreamReg(stream, offset % stride)
            } else {
                Access::Reg(Reg::Unknown)
            }
        }
    }
}
