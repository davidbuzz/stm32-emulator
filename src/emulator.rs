// SPDX-License-Identifier: GPL-3.0-or-later

use std::{mem::MaybeUninit, sync::atomic::{AtomicU64, Ordering, AtomicBool}, cell::RefCell};
use svd_parser::svd::Device as SvdDevice;
use unicorn_engine::{unicorn_const::{Arch, Mode, HookType, MemType}, Unicorn, RegisterARM};
use crate::{config::Config, util::UniErr, Args, system::System, framebuffers::sdl_engine::{PUMP_EVENT_INST_INTERVAL, SDL}};
use anyhow::{Context as _, Result, bail};
use capstone::prelude::*;

#[repr(C)]
struct VectorTable {
    pub sp: u32,
    pub reset: u32,
}

impl VectorTable {
    pub fn from_memory(uc: &Unicorn<()>, addr: u32) -> Result<Self> {
        unsafe {
            let mut self_ = MaybeUninit::<Self>::uninit();
            let buf = std::slice::from_raw_parts_mut(self_.as_mut_ptr() as *mut u8, std::mem::size_of::<Self>());
            uc.mem_read(addr.into(), buf).map_err(UniErr)?;
            Ok(self_.assume_init())
        }
    }
}

fn thumb(pc: u64) -> u64 {
    pc | 1
}

// PC + instruction size
pub static mut LAST_INSTRUCTION: (u32, u8) = (0,0);
pub static NUM_INSTRUCTIONS: AtomicU64 = AtomicU64::new(0);
static CONTINUE_EXECUTION: AtomicBool = AtomicBool::new(false);
static BUSY_LOOP_REACHED: AtomicBool = AtomicBool::new(false);
static STOP_REQUESTED: AtomicBool = AtomicBool::new(false);

fn disassemble_instruction(diassembler: &Capstone, uc: &Unicorn<()>, pc: u64) -> String {
    let mut instr = [0; 4];
    if uc.mem_read(pc, &mut instr).is_err() {
        return "failed to read memory at pc".to_string();
    }

    if let Ok(disasm) = diassembler.disasm_count(&instr, pc, 1) {
        if let Some(instr) = disasm.first() {
            return format!("{:5} {}", instr.mnemonic().unwrap(), instr.op_str().unwrap());
        }
    }

    return "??".to_string();
}

pub fn dump_stack(uc: &mut Unicorn<()>, count: usize) {
    let mut sp = uc.reg_read(RegisterARM::SP).unwrap();

    for _ in 0..count {
        let mut v = [0,0,0,0];
        if uc.mem_read(sp, &mut v).is_err() {
            info!("stack dump finished due to mem read error");
            return;
        }
        let v = u32::from_le_bytes(v);

        if (0x0800_0000..0x0810_0000).contains(&v) {
            // Probably a return address
            info!("*** 0x{:08x} (sp=0x{:08x})", v, sp);
        } else {
            info!("    0x{:08x} (sp=0x{:08x})", v, sp);
        }

        sp += 4;
    }
}

pub fn run_emulator(config: Config, svd_device: SvdDevice, args: Args) -> Result<()> {
    let mut uc = Unicorn::new(Arch::ARM, Mode::MCLASS | Mode::LITTLE_ENDIAN)
        .map_err(UniErr).context("Failed to initialize Unicorn instance")?;

    let vector_table_addr = config.cpu.vector_table;

    let (sys, framebuffers) = crate::system::prepare(&mut uc, config, svd_device)?;
    sys.p.nvic.borrow_mut().vector_table_addr = vector_table_addr;

    let diassembler = Capstone::new()
        .arm()
        .mode(arch::arm::ArchMode::Thumb)
        .build()
        .expect("failed to initialize capstone");

    // We hook on each instructions, but we could skip this.
    // The slowdown is less than 50%. It's okay for now.
    {
        let trace_instructions = crate::verbose() >= 4;
        let busy_loop_stop = args.busy_loop_stop;
        let p = sys.p.clone();
        let d = sys.d.clone();
        let interrupt_period = args.interrupt_period;
        sys.uc.borrow_mut().add_code_hook(0, u64::MAX, move |uc, pc, size| {
            unsafe {
                if busy_loop_stop && LAST_INSTRUCTION.0 == pc as u32 {
                    let sp = uc.reg_read(RegisterARM::SP).unwrap_or(0);
                    let lr = uc.reg_read(RegisterARM::LR).unwrap_or(0);
                    let r0 = uc.reg_read(RegisterARM::R0).unwrap_or(0);
                    let r1 = uc.reg_read(RegisterARM::R1).unwrap_or(0);
                    let r2 = uc.reg_read(RegisterARM::R2).unwrap_or(0);
                    let r3 = uc.reg_read(RegisterARM::R3).unwrap_or(0);
                    let r4 = uc.reg_read(RegisterARM::R4).unwrap_or(0);
                    let r5 = uc.reg_read(RegisterARM::R5).unwrap_or(0);
                    let r6 = uc.reg_read(RegisterARM::R6).unwrap_or(0);
                    let r7 = uc.reg_read(RegisterARM::R7).unwrap_or(0);
                    let primask = uc.reg_read(RegisterARM::PRIMASK).unwrap_or(0);
                    let basepri = uc.reg_read(RegisterARM::BASEPRI).unwrap_or(0);
                    let control = uc.reg_read(RegisterARM::CONTROL).unwrap_or(0);
                    let ipsr = uc.reg_read(RegisterARM::IPSR).unwrap_or(0);
                    info!("Busy loop reached pc=0x{:08x} sp=0x{:08x} lr=0x{:08x} r0=0x{:08x} r1=0x{:08x} r2=0x{:08x} r3=0x{:08x} r4=0x{:08x} r5=0x{:08x} r6=0x{:08x} r7=0x{:08x} primask=0x{:08x} basepri=0x{:08x} control=0x{:08x} ipsr=0x{:08x}",
                        pc, sp, lr, r0, r1, r2, r3, r4, r5, r6, r7, primask, basepri, control, ipsr);
                    if r1 != 0 {
                        let mut buf = [0u8; 64];
                        if uc.mem_read(r1, &mut buf).is_ok() {
                            let words = (0..16)
                                .map(|i| {
                                    let o = i * 4;
                                    u32::from_le_bytes([buf[o], buf[o + 1], buf[o + 2], buf[o + 3]])
                                })
                                .collect::<Vec<_>>();
                            info!(
                                "Busy loop ctx @r1=0x{:08x}: [{:08x} {:08x} {:08x} {:08x} {:08x} {:08x} {:08x} {:08x} {:08x} {:08x} {:08x} {:08x} {:08x} {:08x} {:08x} {:08x}]",
                                r1,
                                words[0], words[1], words[2], words[3],
                                words[4], words[5], words[6], words[7],
                                words[8], words[9], words[10], words[11],
                                words[12], words[13], words[14], words[15]
                            );
                        }
                    }

                    let ctx_base = sp.saturating_sub(100);
                    let mut ctx = [0u8; 104];
                    if uc.mem_read(ctx_base, &mut ctx).is_ok() {
                        let read_word = |off: usize| -> u32 {
                            u32::from_le_bytes([ctx[off], ctx[off + 1], ctx[off + 2], ctx[off + 3]])
                        };
                        info!(
                            "Busy loop restore frame base=0x{:08x} r4=0x{:08x} r5=0x{:08x} r6=0x{:08x} r7=0x{:08x} r8=0x{:08x} sb=0x{:08x} sl=0x{:08x} fp=0x{:08x} pc=0x{:08x}",
                            ctx_base,
                            read_word(64), read_word(68), read_word(72), read_word(76),
                            read_word(80), read_word(84), read_word(88), read_word(92), read_word(96)
                        );
                    }
                    uc.emu_stop().unwrap();
                    BUSY_LOOP_REACHED.store(true, Ordering::Release);
                }
                LAST_INSTRUCTION = (pc as u32, size as u8);
            }

            let n = NUM_INSTRUCTIONS.fetch_add(1, Ordering::Acquire);

            if trace_instructions {
                info!("{}", disassemble_instruction(&diassembler, uc, pc));
            }

            let sys = System { uc: RefCell::new(uc), p: p.clone(), d: d.clone() };
            p.step(&sys);

            if n % interrupt_period as u64 == 0 {
                p.nvic.borrow_mut().run_pending_interrupts(&sys);
            }

            if n & PUMP_EVENT_INST_INTERVAL == 0 {
                for fb in &framebuffers.sdls {
                    fb.borrow_mut().maybe_redraw();
                }
                if !SDL.lock().unwrap().pump_events(&framebuffers.sdls) {
                    STOP_REQUESTED.store(true, Ordering::Relaxed);
                    uc.emu_stop().unwrap();
                }
            }
        }).expect("add_code_hook failed");
    }

    {
        let p = sys.p.clone();
        let d = sys.d.clone();
        sys.uc.borrow_mut().add_intr_hook(move |uc, exception| {
            match exception {
                /*
                    EXCP_UDEF            1   /* undefined instruction */
                    EXCP_SWI             2   /* software interrupt */
                    EXCP_PREFETCH_ABORT  3
                    EXCP_DATA_ABORT      4
                    EXCP_IRQ             5
                    EXCP_FIQ             6
                    EXCP_BKPT            7
                    EXCP_EXCEPTION_EXIT  8   /* Return from v7M exception.  */
                    EXCP_KERNEL_TRAP     9   /* Jumped to kernel code page.  */
                    EXCP_HVC            11   /* HyperVisor Call */
                    EXCP_HYP_TRAP       12
                    EXCP_SMC            13   /* Secure Monitor Call */
                    EXCP_VIRQ           14
                    EXCP_VFIQ           15
                    EXCP_SEMIHOST       16   /* semihosting call */
                    EXCP_NOCP           17   /* v7M NOCP UsageFault */
                    EXCP_INVSTATE       18   /* v7M INVSTATE UsageFault */
                    EXCP_STKOF          19   /* v8M STKOF UsageFault */
                    EXCP_LAZYFP         20   /* v7M fault during lazy FP stacking */
                    EXCP_LSERR          21   /* v8M LSERR SecureFault */
                    EXCP_UNALIGNED      22   /* v7M UNALIGNED UsageFault */
                    */
                2 => {
                    let sys = System { uc: RefCell::new(uc), p: p.clone(), d: d.clone() };
                    p.nvic.borrow_mut().run_interrupt(&sys, crate::peripherals::nvic::irq::SVCALL);
                }
                8 => {
                    // Return from interrupt
                    let sys = System { uc: RefCell::new(uc), p: p.clone(), d: d.clone() };
                    p.nvic.borrow_mut().return_from_interrupt(&sys);
                    p.nvic.borrow_mut().run_pending_interrupts(&sys);
                }
                3 => {
                    error!("intr_hook intno={:08x}", exception);
                }
                _ => {
                    error!("intr_hook intno={:08x}", exception);
                    std::process::exit(1);
                }
            }
        }).expect("add_intr_hook failed");
    }

    uc.add_mem_hook(HookType::MEM_UNMAPPED, 0, u64::MAX, |uc, type_, addr, size, value| {
        if type_ == MemType::WRITE_UNMAPPED {
            warn!("{:?} addr=0x{:08x} size={} value=0x{:08x}", type_, addr, size, value);
        } else {
            warn!("{:?} addr=0x{:08x} size={}", type_, addr, size);
        }

        unsafe {
            let pc = uc.reg_read(RegisterARM::PC).expect("failed to get pc");
            if pc as u32 == LAST_INSTRUCTION.0 {
                uc.reg_write(RegisterARM::PC, thumb(pc as u64 + LAST_INSTRUCTION.1 as u64)).unwrap();
            } else {
                // Branch targets can fault on fetch after PC has already changed.
                // In that case, advance from the last known executed instruction.
                warn!(
                    "unmapped hook pc mismatch pc=0x{:08x} last_pc=0x{:08x} size={} advancing from last instruction",
                    pc as u32,
                    LAST_INSTRUCTION.0,
                    LAST_INSTRUCTION.1,
                );
                let next = (LAST_INSTRUCTION.0 as u64).saturating_add(LAST_INSTRUCTION.1 as u64);
                uc.reg_write(RegisterARM::PC, thumb(next)).unwrap();
            }
        }

        CONTINUE_EXECUTION.store(true, Ordering::Release);

        false
    }).expect("add_mem_hook failed");

    let vector_table = VectorTable::from_memory(&uc, vector_table_addr)?;
    let mut pc = vector_table.reset as u64;
    uc.reg_write(RegisterARM::SP, vector_table.sp.into()).map_err(UniErr)?;
    //uc.reg_write(RegisterARM::LR, 0xFFFF_FFFF).map_err(UniErr)?;

    info!("Starting emulation");

    loop {
        let max_instructions = args.max_instructions.map(|c|
            // yes, we want to panic if this goes negative.
            c - NUM_INSTRUCTIONS.load(Ordering::Relaxed)
        );
        if max_instructions == Some(0) {
            info!("Reached target number of instructions. Done");
            break;
        }

        let result = uc.emu_start(
            pc,
            args.stop_addr.unwrap_or(0) as u64,
            0,
            max_instructions.unwrap_or(0) as usize,
        ).map_err(UniErr);
        pc = uc.reg_read(RegisterARM::PC).expect("failed to get pc");

        if STOP_REQUESTED.load(Ordering::Relaxed) {
            info!("Stop requested");
            break;
        }

        if let Err(e) = result {
            if CONTINUE_EXECUTION.swap(false, Ordering::AcqRel) {
                // This was a bad memory access, we keep going.
                if crate::verbose() >= 3 {
                    trace!("Resuming execution pc={:08x}", pc);
                }
                pc = thumb(pc);
                continue;
            } else {
                bail!(e);
            }
        }

        if args.stop_addr == Some(pc as u32) {
            info!("Stop address reached, stopping");
            break;
        }

        if BUSY_LOOP_REACHED.load(Ordering::Relaxed) {
            break;
        }
    }

    if let Some(n) = args.dump_stack {
        dump_stack(&mut uc, n);
    }

    for fb in framebuffers.images {
        fb.borrow().write_to_disk()?;
    }

    Ok(())
}
