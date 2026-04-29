// SPDX-License-Identifier: GPL-3.0-or-later

// GDB Remote Serial Protocol server for the STM32 emulator.
// Uses gdbstub (blocking single-thread mode) over TCP.
// The Target wraps Unicorn via a raw pointer because gdbstub's BlockingEventLoop
// associated type cannot carry a lifetime parameter.  The invariant is that the
// Unicorn instance passed to `run_gdb_server` outlives the GdbTarget, which is
// guaranteed by the call site in `run_emulator`.

use std::cell::{Cell, RefCell};
use std::collections::HashSet;
use std::net::{TcpListener, TcpStream};
use std::rc::Rc;
use std::sync::atomic::Ordering;

use anyhow::Result;
use unicorn_engine::{RegisterARM, Unicorn};

use gdbstub::common::Signal;
use gdbstub::conn::{Connection, ConnectionExt};
use gdbstub::stub::run_blocking::{BlockingEventLoop, Event, WaitForStopReasonError};
use gdbstub::stub::{DisconnectReason, GdbStub, SingleThreadStopReason};
use gdbstub::target::ext::base::singlethread::{
    SingleThreadBase, SingleThreadResume, SingleThreadSingleStep,
};
use gdbstub::target::ext::base::BaseOps;
use gdbstub::target::ext::breakpoints::{Breakpoints, SwBreakpoint};
use gdbstub::target::{Target, TargetError, TargetResult};
use gdbstub_arch::arm::reg::ArmCoreRegs;
use gdbstub_arch::arm::{ArmBreakpointKind, Armv4t};

use crate::emulator::{CONTINUE_EXECUTION, NUM_INSTRUCTIONS};
use crate::ext_devices::ExtDevices;
use crate::peripherals::Peripherals;
use crate::system::System;

fn thumb(pc: u64) -> u64 {
    pc | 1
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ExecMode {
    Continue,
    Step,
}

// Shared state between the code hook and GdbTarget.
pub struct GdbShared {
    pub breakpoints: Rc<RefCell<HashSet<u32>>>,
    pub bp_hit: Rc<Cell<bool>>,
}

impl GdbShared {
    pub fn new() -> Self {
        Self {
            breakpoints: Rc::new(RefCell::new(HashSet::new())),
            bp_hit: Rc::new(Cell::new(false)),
        }
    }

    pub fn clone_handles(&self) -> (Rc<RefCell<HashSet<u32>>>, Rc<Cell<bool>>) {
        (self.breakpoints.clone(), self.bp_hit.clone())
    }
}

pub struct GdbTarget {
    // SAFETY: raw pointer to the Unicorn instance in run_emulator.
    // Safe because: single-threaded blocking mode; Unicorn outlives GdbTarget.
    uc: *mut Unicorn<'static, ()>,
    p: Rc<Peripherals>,
    d: Rc<ExtDevices>,
    deferred_irq: Rc<RefCell<Option<i32>>>,
    breakpoints: Rc<RefCell<HashSet<u32>>>,
    bp_hit: Rc<Cell<bool>>,
    pub exec_mode: ExecMode,
    pub current_pc: u64,
    stop_addr: Option<u32>,
}

impl GdbTarget {
    pub fn new(
        uc: &mut Unicorn<'_, ()>,
        p: Rc<Peripherals>,
        d: Rc<ExtDevices>,
        deferred_irq: Rc<RefCell<Option<i32>>>,
        shared: &GdbShared,
        current_pc: u64,
        stop_addr: Option<u32>,
    ) -> Self {
        let uc_static =
            unsafe { std::mem::transmute::<*mut Unicorn<'_, ()>, *mut Unicorn<'static, ()>>(uc) };
        Self {
            uc: uc_static,
            p,
            d,
            deferred_irq,
            breakpoints: shared.breakpoints.clone(),
            bp_hit: shared.bp_hit.clone(),
            exec_mode: ExecMode::Continue,
            current_pc,
            stop_addr,
        }
    }

    fn handle_deferred_irq(&mut self) {
        let maybe_irq = self.deferred_irq.borrow_mut().take();
        if let Some(mut irq) = maybe_irq {
            // Copy all fields we need BEFORE creating any references.
            // The `let raw = self.uc` copy ends immediately; the subsequent
            // `&mut *raw` does not borrow self.
            let p = self.p.clone();
            let d = self.d.clone();
            let fallback_pc = self.current_pc;
            let raw = self.uc;
            let uc: &mut Unicorn<'static, ()> = unsafe { &mut *raw };

            let mut sys = System { uc: RefCell::new(uc), p, d };

            let selected = sys.p.nvic.borrow_mut().take_pending_interrupt(&sys);
            if let Some(sel) = selected {
                if sel != irq {
                    sys.p.nvic.borrow_mut().set_intr_pending(irq);
                    irq = sel;
                }
            }
            if irq == 67 {
                info!("EMULATOR gdb: dispatching deferred IRQ 67 (NUM_INSTRUCTIONS={})",
                    NUM_INSTRUCTIONS.load(Ordering::Relaxed));
            }
            sys.p.nvic.borrow_mut().run_interrupt(&sys, irq);
            let new_pc = sys.uc.borrow().reg_read(RegisterARM::PC).unwrap_or(fallback_pc);
            drop(sys);
            self.current_pc = new_pc;
        }
    }

    // Run one emulation quantum.  Returns how execution stopped.
    pub fn run_quantum(&mut self, max_instructions: usize) -> QuantumResult {
        self.bp_hit.set(false);
        CONTINUE_EXECUTION.store(false, Ordering::Release);

        // Capture fields before creating the uc reference so no self-borrow conflict.
        let current_pc = self.current_pc;
        let stop_addr  = self.stop_addr;
        let raw = self.uc;
        let uc: &mut Unicorn<'static, ()> = unsafe { &mut *raw };

        let result = uc.emu_start(
            thumb(current_pc),
            stop_addr.unwrap_or(0) as u64,
            0,
            max_instructions,
        );
        let new_pc = uc.reg_read(RegisterARM::PC).unwrap_or(current_pc);
        self.current_pc = new_pc;

        self.handle_deferred_irq();

        if self.bp_hit.get() {
            return QuantumResult::Breakpoint;
        }

        if CONTINUE_EXECUTION.swap(false, Ordering::AcqRel) {
            self.current_pc = thumb(self.current_pc);
            return QuantumResult::Continue;
        }

        match result {
            Ok(_) => QuantumResult::Done,
            Err(e) => {
                warn!(
                    "GDB quantum emu_start error at pc=0x{:#010x}: {:?}",
                    self.current_pc,
                    e
                );
                QuantumResult::Halted
            }
        }
    }
}

pub enum QuantumResult {
    Breakpoint,
    Continue,
    Done,
    Halted,
}

// ── gdbstub Target impl ───────────────────────────────────────────────────────

impl Target for GdbTarget {
    type Arch = Armv4t;
    type Error = anyhow::Error;

    #[inline(always)]
    fn base_ops(&mut self) -> BaseOps<'_, Self::Arch, Self::Error> {
        BaseOps::SingleThread(self)
    }

    #[inline(always)]
    fn support_breakpoints(
        &mut self,
    ) -> Option<gdbstub::target::ext::breakpoints::BreakpointsOps<'_, Self>> {
        Some(self)
    }
}

const ARM_REGS: [RegisterARM; 13] = [
    RegisterARM::R0,  RegisterARM::R1,  RegisterARM::R2,  RegisterARM::R3,
    RegisterARM::R4,  RegisterARM::R5,  RegisterARM::R6,  RegisterARM::R7,
    RegisterARM::R8,  RegisterARM::R9,  RegisterARM::R10, RegisterARM::R11,
    RegisterARM::R12,
];

impl SingleThreadBase for GdbTarget {
    fn read_registers(&mut self, regs: &mut ArmCoreRegs) -> TargetResult<(), Self> {
        let raw = self.uc;
        let uc: &mut Unicorn<'static, ()> = unsafe { &mut *raw };
        for (i, &reg) in ARM_REGS.iter().enumerate() {
            regs.r[i] = uc.reg_read(reg)
                .map_err(|e| TargetError::Fatal(anyhow::anyhow!("reg_read r{i}: {e:?}")))?
                as u32;
        }
        regs.sp = uc.reg_read(RegisterARM::SP)
            .map_err(|e| TargetError::Fatal(anyhow::anyhow!("reg_read sp: {e:?}")))? as u32;
        regs.lr = uc.reg_read(RegisterARM::LR)
            .map_err(|e| TargetError::Fatal(anyhow::anyhow!("reg_read lr: {e:?}")))? as u32;
        // Report PC without thumb bit
        regs.pc = uc.reg_read(RegisterARM::PC)
            .map_err(|e| TargetError::Fatal(anyhow::anyhow!("reg_read pc: {e:?}")))? as u32 & !1;
        regs.cpsr = uc.reg_read(RegisterARM::CPSR)
            .map_err(|e| TargetError::Fatal(anyhow::anyhow!("reg_read cpsr: {e:?}")))? as u32;
        Ok(())
    }

    fn write_registers(&mut self, regs: &ArmCoreRegs) -> TargetResult<(), Self> {
        let raw = self.uc;
        let uc: &mut Unicorn<'static, ()> = unsafe { &mut *raw };
        for (i, &reg) in ARM_REGS.iter().enumerate() {
            uc.reg_write(reg, regs.r[i] as u64)
                .map_err(|e| TargetError::Fatal(anyhow::anyhow!("reg_write r{i}: {e:?}")))?;
        }
        uc.reg_write(RegisterARM::SP, regs.sp as u64)
            .map_err(|e| TargetError::Fatal(anyhow::anyhow!("reg_write sp: {e:?}")))?;
        uc.reg_write(RegisterARM::LR, regs.lr as u64)
            .map_err(|e| TargetError::Fatal(anyhow::anyhow!("reg_write lr: {e:?}")))?;
        uc.reg_write(RegisterARM::PC, regs.pc as u64)
            .map_err(|e| TargetError::Fatal(anyhow::anyhow!("reg_write pc: {e:?}")))?;
        self.current_pc = regs.pc as u64;
        uc.reg_write(RegisterARM::CPSR, regs.cpsr as u64)
            .map_err(|e| TargetError::Fatal(anyhow::anyhow!("reg_write cpsr: {e:?}")))?;
        Ok(())
    }

    fn read_addrs(&mut self, start_addr: u32, data: &mut [u8]) -> TargetResult<usize, Self> {
        let raw = self.uc;
        let uc: &mut Unicorn<'static, ()> = unsafe { &mut *raw };
        // Best-effort: read as much as Unicorn allows; unmapped regions return 0s.
        if uc.mem_read(start_addr as u64, data).is_err() {
            data.fill(0);
        }
        Ok(data.len())
    }

    fn write_addrs(&mut self, start_addr: u32, data: &[u8]) -> TargetResult<(), Self> {
        let raw = self.uc;
        let uc: &mut Unicorn<'static, ()> = unsafe { &mut *raw };
        uc.mem_write(start_addr as u64, data)
            .map_err(|e| TargetError::Fatal(anyhow::anyhow!("mem_write: {:?}", e)))?;
        Ok(())
    }

    #[inline(always)]
    fn support_resume(
        &mut self,
    ) -> Option<gdbstub::target::ext::base::singlethread::SingleThreadResumeOps<'_, Self>> {
        Some(self)
    }
}

impl SingleThreadResume for GdbTarget {
    fn resume(&mut self, signal: Option<Signal>) -> Result<(), Self::Error> {
        if signal.is_some() {
            // Continuing with a signal is unusual in this context; just continue.
        }
        self.exec_mode = ExecMode::Continue;
        Ok(())
    }

    #[inline(always)]
    fn support_single_step(
        &mut self,
    ) -> Option<gdbstub::target::ext::base::singlethread::SingleThreadSingleStepOps<'_, Self>>
    {
        Some(self)
    }
}

impl SingleThreadSingleStep for GdbTarget {
    fn step(&mut self, _signal: Option<Signal>) -> Result<(), Self::Error> {
        self.exec_mode = ExecMode::Step;
        Ok(())
    }
}

// ── Breakpoints ───────────────────────────────────────────────────────────────

impl Breakpoints for GdbTarget {
    #[inline(always)]
    fn support_sw_breakpoint(
        &mut self,
    ) -> Option<gdbstub::target::ext::breakpoints::SwBreakpointOps<'_, Self>> {
        Some(self)
    }
}

impl SwBreakpoint for GdbTarget {
    fn add_sw_breakpoint(
        &mut self,
        addr: u32,
        _kind: ArmBreakpointKind,
    ) -> TargetResult<bool, Self> {
        self.breakpoints.borrow_mut().insert(addr);
        Ok(true)
    }

    fn remove_sw_breakpoint(
        &mut self,
        addr: u32,
        _kind: ArmBreakpointKind,
    ) -> TargetResult<bool, Self> {
        Ok(self.breakpoints.borrow_mut().remove(&addr))
    }
}

// ── Blocking event loop ───────────────────────────────────────────────────────

const CONTINUE_QUANTUM: usize = 10_000;

enum EmuEventLoop {}

impl BlockingEventLoop for EmuEventLoop {
    type Target = GdbTarget;
    type Connection = TcpStream;
    type StopReason = SingleThreadStopReason<u32>;

    fn wait_for_stop_reason(
        target: &mut GdbTarget,
        conn: &mut TcpStream,
    ) -> Result<
        Event<SingleThreadStopReason<u32>>,
        WaitForStopReasonError<
            <GdbTarget as Target>::Error,
            <TcpStream as Connection>::Error,
        >,
    > {
        loop {
            // Poll for incoming GDB data (Ctrl-C, commands, etc.)
            if conn
                .peek()
                .map(|b| b.is_some())
                .unwrap_or(true)
            {
                let byte = conn
                    .read()
                    .map_err(WaitForStopReasonError::Connection)?;
                return Ok(Event::IncomingData(byte));
            }

            let quantum = match target.exec_mode {
                ExecMode::Step => 1,
                ExecMode::Continue => CONTINUE_QUANTUM,
            };

            match target.run_quantum(quantum) {
                QuantumResult::Breakpoint => {
                    return Ok(Event::TargetStopped(SingleThreadStopReason::SwBreak(())));
                }
                QuantumResult::Continue => {
                    // Emulator needs to continue (IRQ dispatch, etc.) - loop again
                }
                QuantumResult::Done if target.exec_mode == ExecMode::Step => {
                    return Ok(Event::TargetStopped(SingleThreadStopReason::DoneStep));
                }
                QuantumResult::Done => {
                    // Quantum exhausted in Continue mode; keep running
                }
                QuantumResult::Halted => {
                    return Ok(Event::TargetStopped(SingleThreadStopReason::Terminated(
                        Signal::SIGTERM,
                    )));
                }
            }
        }
    }

    fn on_interrupt(
        _target: &mut GdbTarget,
    ) -> Result<Option<SingleThreadStopReason<u32>>, <GdbTarget as Target>::Error> {
        Ok(Some(SingleThreadStopReason::Signal(Signal::SIGINT)))
    }
}

// ── Public entry point ────────────────────────────────────────────────────────

pub fn run_gdb_server(mut target: GdbTarget, port: u16) -> Result<()> {
    let addr = format!("0.0.0.0:{port}");
    info!("GDB: waiting for connection on {addr}");
    let listener = TcpListener::bind(&addr)?;
    let (stream, peer) = listener.accept()?;
    info!("GDB: debugger connected from {peer}");

    // Disable Nagle to reduce round-trip latency for small RSP packets.
    stream.set_nodelay(true).ok();

    let gdb = GdbStub::new(stream);

    match gdb.run_blocking::<EmuEventLoop>(&mut target) {
        Ok(reason) => match reason {
            DisconnectReason::Disconnect => {
                info!("GDB: client disconnected");
            }
            DisconnectReason::TargetExited(code) => {
                info!("GDB: target exited with code {code}");
            }
            DisconnectReason::TargetTerminated(sig) => {
                info!("GDB: target terminated with signal {sig}");
            }
            DisconnectReason::Kill => {
                info!("GDB: kill command received");
            }
        },
        Err(e) => {
            if e.is_target_error() {
                return Err(e.into_target_error().unwrap());
            } else if e.is_connection_error() {
                let (io_err, _kind) = e.into_connection_error().unwrap();
                return Err(io_err.into());
            } else {
                return Err(anyhow::anyhow!("gdbstub: {e}"));
            }
        }
    }

    Ok(())
}
