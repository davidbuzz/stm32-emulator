// SPDX-License-Identifier: GPL-3.0-or-later

use std::{mem::MaybeUninit, sync::atomic::{AtomicU64, Ordering, AtomicBool}, cell::RefCell, rc::Rc};
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
pub(crate) static CONTINUE_EXECUTION: AtomicBool = AtomicBool::new(false);
static BUSY_LOOP_REACHED: AtomicBool = AtomicBool::new(false);
static STOP_REQUESTED: AtomicBool = AtomicBool::new(false);

const BUSY_LOOP_STREAK_THRESHOLD: u32 = 1_000_000;

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

pub fn dump_stack_from(uc: &mut Unicorn<()>, mut sp: u64, count: usize, label: &str) {
    info!("{} stack dump starting at sp=0x{:08x}", label, sp);

    for _ in 0..count {
        let mut v = [0, 0, 0, 0];
        if uc.mem_read(sp, &mut v).is_err() {
            info!("{} stack dump finished due to mem read error", label);
            return;
        }
        let v = u32::from_le_bytes(v);

        if (0x0800_0000..0x0818_0000).contains(&v) {
            info!("{} *** 0x{:08x} (sp=0x{:08x})", label, v, sp);
        } else {
            info!("{}     0x{:08x} (sp=0x{:08x})", label, v, sp);
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

    // GDB shared state: breakpoints set and hit flag.  Both are Rc so they can be
    // captured by the code hook closure (single-threaded) and owned by GdbTarget.
    let gdb_shared = crate::gdb::GdbShared::new();
    let (gdb_breakpoints, gdb_bp_hit) = gdb_shared.clone_handles();

    let diassembler = Capstone::new()
        .arm()
        .mode(arch::arm::ArchMode::Thumb)
        .build()
        .expect("failed to initialize capstone");

    // We hook on each instructions, but we could skip this.
    // The slowdown is less than 50%. It's okay for now.
    let deferred_irq = Rc::new(RefCell::new(None::<i32>));
    {
        let trace_instructions = crate::verbose() >= 4;
        let busy_loop_stop = args.busy_loop_stop;
        let mut busy_loop_pc: Option<u32> = None;
        let mut busy_loop_streak: u32 = 0;
        let mut saw_uart_begin_impl = false;
        let mut saw_uart_thread_rx_init = false;
        let mut saw_thread_create_alloc = false;
        let mut saw_uart_begin_stage_8032 = false;
        let mut saw_uart_begin_stage_8130 = false;
        let mut saw_uart_begin_stage_8188 = false;
        let mut saw_uart_begin_return = false;
        let mut saw_mainloop_post_begin = false;
        let mut saw_mainloop_post_analog = false;
        let mut saw_vehicle_setup = false;
        let mut saw_init_console = false;
        let mut saw_ap_param_setup = false;
        let mut saw_ap_param_setup_post_read = false;
        let mut saw_ap_param_erase_all = false;
        let mut saw_ap_param_eeprom_write_check = false;
        let mut saw_storageaccess_write_block = false;
        let mut saw_chibios_storage_write_block = false;
        let mut saw_storageaccess_read_block = false;
        let mut saw_chibios_storage_read_block = false;
        let mut saw_chibios_storage_open = false;
        let mut saw_ramtron_init = false;
        let mut saw_ramtron_read = false;
        let mut saw_ramtron_transfer_call = false;
        let mut saw_ramtron_transfer_return = false;
        let mut saw_spidevice_transfer = false;
        let mut saw_spidevice_do_transfer = false;
        let mut saw_storage_open_post_ramtron_read = false;
        let mut saw_storage_save_backup = false;
        let mut saw_storage_open_post_save_backup = false;
        let mut saw_save_backup_loop_start = false;
        let mut saw_save_backup_after_mount_wait = false;
        let mut saw_save_backup_open_last = false;
        let mut saw_save_backup_open_data = false;
        let mut saw_save_backup_write_data = false;
        let mut saw_save_backup_done_path = false;
        let mut save_backup_wait_seen = false;
        let mut save_backup_wait_last_delta: u32 = 0;
        let mut saw_f_mount_entry = false;
        let mut saw_mount_volume_entry = false;
        let mut saw_mount_volume_after_find = false;
        let mut saw_mount_volume_return = false;
        let mut saw_mount_volume_boundary_check = false;
        let mut saw_mount_volume_boundary_fail = false;
        let mut saw_mount_volume_totsec = false;
        let mut saw_mount_volume_exfat_fields = false;
        let mut mount_volume_loop_bpb_scan_hits: u32 = 0;
        let mut mount_volume_loop_root_scan_hits: u32 = 0;
        let mut mount_volume_loop_fat_chain_hits: u32 = 0;
        let mut saw_find_volume_entry = false;
        let mut saw_find_volume_return = false;
        let mut saw_move_window_entry = false;
        let mut saw_move_window_post_disk_read = false;
        let mut saw_disk_read_entry = false;
        let mut saw_disk_read_call_driver = false;
        let mut saw_disk_read_return_from_driver = false;
        let mut disk_read_retry_hits: u32 = 0;
        let p = sys.p.clone();
        let d = sys.d.clone();
        let interrupt_period = args.interrupt_period;
        let deferred_irq = deferred_irq.clone();
        let gdb_breakpoints_hook = gdb_breakpoints.clone();
        let gdb_bp_hit_hook = gdb_bp_hit.clone();
        sys.uc.borrow_mut().add_code_hook(0, u64::MAX, move |uc, pc, size| {
            // GDB software breakpoint check — must run before other hook logic.
            {
                let bps = gdb_breakpoints_hook.borrow();
                if !bps.is_empty() && bps.contains(&(pc as u32)) {
                    gdb_bp_hit_hook.set(true);
                    uc.emu_stop().unwrap();
                    return;
                }
            }
            unsafe {
                let pc32 = pc as u32;
                let pc_aligned = pc32 & !1;
                if !saw_uart_begin_impl && pc_aligned == 0x0814_7fd4 {
                    saw_uart_begin_impl = true;
                    let lr = uc.reg_read(RegisterARM::LR).unwrap_or(0);
                    info!("TRACE UART begin impl reached pc=0x{pc32:08x} lr=0x{lr:08x}");
                }
                if !saw_uart_thread_rx_init && pc_aligned == 0x0814_7170 {
                    saw_uart_thread_rx_init = true;
                    let lr = uc.reg_read(RegisterARM::LR).unwrap_or(0);
                    info!("TRACE UART thread_rx_init reached pc=0x{pc32:08x} lr=0x{lr:08x}");
                }
                if !saw_thread_create_alloc && pc_aligned == 0x0815_aa28 {
                    saw_thread_create_alloc = true;
                    let lr = uc.reg_read(RegisterARM::LR).unwrap_or(0);
                    info!("TRACE thread_create_alloc reached pc=0x{pc32:08x} lr=0x{lr:08x}");
                }
                if !saw_uart_begin_stage_8032 && pc_aligned == 0x0814_8032 {
                    saw_uart_begin_stage_8032 = true;
                    info!("TRACE UART _begin stage reached pc=0x{pc32:08x}");
                }
                if !saw_uart_begin_stage_8130 && pc_aligned == 0x0814_8130 {
                    saw_uart_begin_stage_8130 = true;
                    info!("TRACE UART _begin stage reached pc=0x{pc32:08x}");
                }
                if !saw_uart_begin_stage_8188 && pc_aligned == 0x0814_8188 {
                    saw_uart_begin_stage_8188 = true;
                    info!("TRACE UART _begin stage reached pc=0x{pc32:08x}");
                }
                if !saw_uart_begin_return && pc_aligned == 0x0814_81c0 {
                    saw_uart_begin_return = true;
                    info!("TRACE UART _begin return reached pc=0x{pc32:08x}");
                }
                if !saw_mainloop_post_begin && pc_aligned == 0x080f_27cc {
                    saw_mainloop_post_begin = true;
                    info!("TRACE main_loop post-begin reached pc=0x{pc32:08x}");
                }
                if !saw_mainloop_post_analog && pc_aligned == 0x080f_27d4 {
                    saw_mainloop_post_analog = true;
                    info!("TRACE main_loop post-analoginit reached pc=0x{pc32:08x}");
                }
                if !saw_vehicle_setup && pc_aligned == 0x0808_2f48 {
                    saw_vehicle_setup = true;
                    info!("TRACE AP_Vehicle::setup reached pc=0x{pc32:08x}");
                }
                if !saw_init_console && pc_aligned == 0x0808_0f38 {
                    saw_init_console = true;
                    info!("TRACE AP_SerialManager::init_console reached pc=0x{pc32:08x}");
                }
                if !saw_ap_param_setup && pc_aligned == 0x0807_5748 {
                    saw_ap_param_setup = true;
                    info!("TRACE AP_Param::setup reached pc=0x{pc32:08x}");
                }
                if !saw_ap_param_setup_post_read && pc_aligned == 0x0807_575c {
                    saw_ap_param_setup_post_read = true;
                    info!("TRACE AP_Param::setup post-read reached pc=0x{pc32:08x}");
                }
                if !saw_ap_param_erase_all && pc_aligned == 0x0807_5714 {
                    saw_ap_param_erase_all = true;
                    info!("TRACE AP_Param::erase_all reached pc=0x{pc32:08x}");
                }
                if !saw_ap_param_eeprom_write_check && pc_aligned == 0x0807_4e34 {
                    saw_ap_param_eeprom_write_check = true;
                    info!("TRACE AP_Param::eeprom_write_check reached pc=0x{pc32:08x}");
                }
                if !saw_storageaccess_write_block && pc_aligned == 0x0809_737c {
                    saw_storageaccess_write_block = true;
                    info!("TRACE StorageAccess::write_block reached pc=0x{pc32:08x}");
                }
                if !saw_chibios_storage_write_block && pc_aligned == 0x080f_2dc4 {
                    saw_chibios_storage_write_block = true;
                    info!("TRACE ChibiOS::Storage::write_block reached pc=0x{pc32:08x}");
                }
                if !saw_storageaccess_read_block && pc_aligned == 0x0809_72f4 {
                    saw_storageaccess_read_block = true;
                    info!("TRACE StorageAccess::read_block reached pc=0x{pc32:08x}");
                }
                if !saw_chibios_storage_read_block && pc_aligned == 0x080f_2d38 {
                    saw_chibios_storage_read_block = true;
                    info!("TRACE ChibiOS::Storage::read_block reached pc=0x{pc32:08x}");
                }
                if !saw_chibios_storage_open && pc_aligned == 0x080f_2cdc {
                    saw_chibios_storage_open = true;
                    info!("TRACE ChibiOS::Storage::_storage_open reached pc=0x{pc32:08x}");
                }
                if !saw_ramtron_init && pc_aligned == 0x0813_8a34 {
                    saw_ramtron_init = true;
                    info!("TRACE AP_RAMTRON::init reached pc=0x{pc32:08x}");
                }
                if !saw_ramtron_read && pc_aligned == 0x0813_8bc8 {
                    saw_ramtron_read = true;
                    info!("TRACE AP_RAMTRON::read reached pc=0x{pc32:08x}");
                }
                if !saw_ramtron_transfer_call && pc_aligned == 0x0813_8ca6 {
                    saw_ramtron_transfer_call = true;
                    info!("TRACE AP_RAMTRON::read transfer call reached pc=0x{pc32:08x}");
                }
                if !saw_ramtron_transfer_return && pc_aligned == 0x0813_8ca8 {
                    saw_ramtron_transfer_return = true;
                    info!("TRACE AP_RAMTRON::read transfer return reached pc=0x{pc32:08x}");
                }
                if !saw_spidevice_transfer && pc_aligned == 0x0814_5904 {
                    saw_spidevice_transfer = true;
                    info!("TRACE ChibiOS::SPIDevice::transfer_fullduplex reached pc=0x{pc32:08x}");
                }
                if !saw_spidevice_do_transfer && pc_aligned == 0x0814_5700 {
                    saw_spidevice_do_transfer = true;
                    info!("TRACE ChibiOS::SPIDevice::do_transfer reached pc=0x{pc32:08x}");
                }
                if !saw_storage_open_post_ramtron_read && pc_aligned == 0x080f_2d14 {
                    saw_storage_open_post_ramtron_read = true;
                    info!("TRACE ChibiOS::Storage::_storage_open post-RAMTRON-read reached pc=0x{pc32:08x}");
                }
                if !saw_storage_save_backup && pc_aligned == 0x080f_2b18 {
                    saw_storage_save_backup = true;
                    info!("TRACE ChibiOS::Storage::_save_backup reached pc=0x{pc32:08x}");
                }
                if !saw_storage_open_post_save_backup && pc_aligned == 0x080f_2d1c {
                    saw_storage_open_post_save_backup = true;
                    info!("TRACE ChibiOS::Storage::_storage_open post-save-backup reached pc=0x{pc32:08x}");
                }
                if !saw_save_backup_loop_start && pc_aligned == 0x080f_2b50 {
                    saw_save_backup_loop_start = true;
                    info!("TRACE _save_backup loop start reached pc=0x{pc32:08x}");
                }
                if !saw_save_backup_after_mount_wait && pc_aligned == 0x080f_2b72 {
                    saw_save_backup_after_mount_wait = true;
                    info!("TRACE _save_backup after mount-wait reached pc=0x{pc32:08x}");
                }
                if !saw_save_backup_open_last && pc_aligned == 0x080f_2ba0 {
                    saw_save_backup_open_last = true;
                    info!("TRACE _save_backup open(last_storage_bak) reached pc=0x{pc32:08x}");
                }
                if !saw_save_backup_open_data && pc_aligned == 0x080f_2c88 {
                    saw_save_backup_open_data = true;
                    info!("TRACE _save_backup open(STRG*.bak) reached pc=0x{pc32:08x}");
                }
                if !saw_save_backup_write_data && pc_aligned == 0x080f_2cb4 {
                    saw_save_backup_write_data = true;
                    info!("TRACE _save_backup write(storage_blob) reached pc=0x{pc32:08x}");
                }
                if !saw_save_backup_done_path && pc_aligned == 0x080f_2c9e {
                    saw_save_backup_done_path = true;
                    info!("TRACE _save_backup done-path reached pc=0x{pc32:08x}");
                }
                if pc_aligned == 0x080f_2b5e {
                    let now_ms = uc.reg_read(RegisterARM::R0).unwrap_or(0) as u32;
                    let start_ms = uc.reg_read(RegisterARM::R5).unwrap_or(0) as u32;
                    let delta = now_ms.wrapping_sub(start_ms);
                    if !save_backup_wait_seen || delta >= save_backup_wait_last_delta.saturating_add(100) {
                        save_backup_wait_seen = true;
                        save_backup_wait_last_delta = delta;
                        info!(
                            "TRACE _save_backup wait progress now_ms={} start_ms={} delta_ms={}",
                            now_ms,
                            start_ms,
                            delta
                        );
                    }
                }
                if !saw_f_mount_entry && pc_aligned == 0x0815_2a3c {
                    saw_f_mount_entry = true;
                    let lr = uc.reg_read(RegisterARM::LR).unwrap_or(0);
                    let r2 = uc.reg_read(RegisterARM::R2).unwrap_or(0);
                    info!("TRACE f_mount entry reached pc=0x{pc32:08x} lr=0x{lr:08x} opt=0x{r2:08x}");
                }
                if !saw_mount_volume_entry && pc_aligned == 0x0815_0ab0 {
                    saw_mount_volume_entry = true;
                    let lr = uc.reg_read(RegisterARM::LR).unwrap_or(0);
                    let r0 = uc.reg_read(RegisterARM::R0).unwrap_or(0);
                    let r1 = uc.reg_read(RegisterARM::R1).unwrap_or(0);
                    let r2 = uc.reg_read(RegisterARM::R2).unwrap_or(0);
                    info!(
                        "TRACE mount_volume entry reached pc=0x{pc32:08x} lr=0x{lr:08x} path=0x{r0:08x} rfs_ptr=0x{r1:08x} mode=0x{r2:08x}"
                    );
                }
                if !saw_mount_volume_after_find && pc_aligned == 0x0815_0b22 {
                    saw_mount_volume_after_find = true;
                    let r0 = uc.reg_read(RegisterARM::R0).unwrap_or(0);
                    info!("TRACE mount_volume post-find_volume reached pc=0x{pc32:08x} find_volume_res=0x{r0:08x}");
                }
                if !saw_mount_volume_boundary_check && pc_aligned == 0x0815_0de2 {
                    saw_mount_volume_boundary_check = true;
                    let r7 = uc.reg_read(RegisterARM::R7).unwrap_or(0);
                    let r9 = uc.reg_read(RegisterARM::R9).unwrap_or(0);
                    let r3 = uc.reg_read(RegisterARM::R3).unwrap_or(0);
                    info!(
                        "TRACE mount_volume boundary-check reached pc=0x{pc32:08x} r7=0x{r7:08x} r9=0x{r9:08x} r3=0x{r3:08x}"
                    );
                }
                if !saw_mount_volume_boundary_fail && pc_aligned == 0x0815_0f12 {
                    saw_mount_volume_boundary_fail = true;
                    let r7 = uc.reg_read(RegisterARM::R7).unwrap_or(0);
                    let r9 = uc.reg_read(RegisterARM::R9).unwrap_or(0);
                    info!(
                        "TRACE mount_volume boundary-fail reached pc=0x{pc32:08x} r7=0x{r7:08x} r9=0x{r9:08x}"
                    );
                }
                if !saw_mount_volume_totsec && pc_aligned == 0x0815_0b5a {
                    saw_mount_volume_totsec = true;
                    let r7 = uc.reg_read(RegisterARM::R7).unwrap_or(0);
                    let r2 = uc.reg_read(RegisterARM::R2).unwrap_or(0);
                    let r9 = uc.reg_read(RegisterARM::R9).unwrap_or(0);
                    info!(
                        "TRACE mount_volume FAT totals reached pc=0x{pc32:08x} totsec_r7=0x{r7:08x} secperclus_r2=0x{r2:08x} rootent_r9=0x{r9:08x}"
                    );
                }
                if !saw_mount_volume_exfat_fields && pc_aligned == 0x0815_0c48 {
                    saw_mount_volume_exfat_fields = true;
                    let r0 = uc.reg_read(RegisterARM::R0).unwrap_or(0);
                    let r1 = uc.reg_read(RegisterARM::R1).unwrap_or(0);
                    let r6 = uc.reg_read(RegisterARM::R6).unwrap_or(0);
                    let r7 = uc.reg_read(RegisterARM::R7).unwrap_or(0);
                    info!(
                        "TRACE mount_volume exfat fields reached pc=0x{pc32:08x} volofs_lo_r0=0x{r0:08x} volofs_hi_r1=0x{r1:08x} partbase_r6=0x{r6:08x} volsct_lo_r7=0x{r7:08x}"
                    );
                }
                if !saw_find_volume_entry && pc_aligned == 0x0815_0a40 {
                    saw_find_volume_entry = true;
                    let lr = uc.reg_read(RegisterARM::LR).unwrap_or(0);
                    let r1 = uc.reg_read(RegisterARM::R1).unwrap_or(0);
                    info!("TRACE find_volume entry reached pc=0x{pc32:08x} lr=0x{lr:08x} mode=0x{r1:08x}");
                }
                if !saw_find_volume_return && pc_aligned == 0x0815_0aac {
                    saw_find_volume_return = true;
                    let r0 = uc.reg_read(RegisterARM::R0).unwrap_or(0);
                    info!("TRACE find_volume return path reached pc=0x{pc32:08x} res=0x{r0:08x}");
                }
                if !saw_move_window_entry && pc_aligned == 0x0815_0914 {
                    saw_move_window_entry = true;
                    let r1 = uc.reg_read(RegisterARM::R1).unwrap_or(0);
                    info!("TRACE move_window entry reached pc=0x{pc32:08x} sector=0x{r1:08x}");
                }
                if !saw_move_window_post_disk_read && pc_aligned == 0x0815_093e {
                    saw_move_window_post_disk_read = true;
                    let r0 = uc.reg_read(RegisterARM::R0).unwrap_or(0);
                    info!("TRACE move_window post-disk_read reached pc=0x{pc32:08x} disk_read_res=0x{r0:08x}");
                }
                if !saw_disk_read_entry && pc_aligned == 0x0815_9ffc {
                    saw_disk_read_entry = true;
                    let r0 = uc.reg_read(RegisterARM::R0).unwrap_or(0);
                    let r2 = uc.reg_read(RegisterARM::R2).unwrap_or(0);
                    let r3 = uc.reg_read(RegisterARM::R3).unwrap_or(0);
                    info!(
                        "TRACE disk_read entry reached pc=0x{pc32:08x} pdrv=0x{r0:08x} sector=0x{r2:08x} count=0x{r3:08x}"
                    );
                }
                if !saw_disk_read_call_driver && pc_aligned == 0x0815_a024 {
                    saw_disk_read_call_driver = true;
                    let r6 = uc.reg_read(RegisterARM::R6).unwrap_or(0);
                    let r8 = uc.reg_read(RegisterARM::R8).unwrap_or(0);
                    let r7 = uc.reg_read(RegisterARM::R7).unwrap_or(0);
                    info!(
                        "TRACE disk_read call driver reached pc=0x{pc32:08x} callee=0x{r6:08x} sector=0x{r8:08x} count=0x{r7:08x}"
                    );
                }
                if !saw_disk_read_return_from_driver && pc_aligned == 0x0815_a02c {
                    saw_disk_read_return_from_driver = true;
                    let r0 = uc.reg_read(RegisterARM::R0).unwrap_or(0);
                    let r5 = uc.reg_read(RegisterARM::R5).unwrap_or(0);
                    let mut hdr = [0u8; 16];
                    let mut sig = [0u8; 2];
                    let mut bps = [0u8; 2];
                    let hdr_ok = uc.mem_read(r5, &mut hdr).is_ok();
                    let sig_ok = uc.mem_read(r5 + 510, &mut sig).is_ok();
                    let bps_ok = uc.mem_read(r5 + 11, &mut bps).is_ok();
                    let sig_u16 = u16::from_le_bytes(sig);
                    let bps_u16 = u16::from_le_bytes(bps);
                    info!(
                        "TRACE disk_read return from driver reached pc=0x{pc32:08x} driver_res=0x{r0:08x} buf=0x{r5:08x} hdr_ok={} sig_ok={} sig=0x{:04x} bps_ok={} bps={} hdr={:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
                        hdr_ok,
                        sig_ok,
                        sig_u16,
                        bps_ok,
                        bps_u16,
                        hdr[0], hdr[1], hdr[2], hdr[3], hdr[4], hdr[5], hdr[6], hdr[7],
                        hdr[8], hdr[9], hdr[10], hdr[11], hdr[12], hdr[13], hdr[14], hdr[15]
                    );
                }
                if pc_aligned == 0x0815_a016 {
                    disk_read_retry_hits = disk_read_retry_hits.saturating_add(1);
                    if disk_read_retry_hits == 1 || disk_read_retry_hits % 64 == 0 {
                        let r4 = uc.reg_read(RegisterARM::R4).unwrap_or(0);
                        info!(
                            "TRACE disk_read retry loop hits={} pc=0x{pc32:08x} retry_idx=0x{r4:08x}",
                            disk_read_retry_hits
                        );
                    }
                }
                if pc_aligned == 0x0815_0c10 {
                    mount_volume_loop_bpb_scan_hits = mount_volume_loop_bpb_scan_hits.saturating_add(1);
                    if mount_volume_loop_bpb_scan_hits == 1 || mount_volume_loop_bpb_scan_hits % 1024 == 0 {
                        let r3 = uc.reg_read(RegisterARM::R3).unwrap_or(0);
                        info!(
                            "TRACE mount_volume loop[bpb_scan] hits={} pc=0x{pc32:08x} index_r3=0x{r3:08x}",
                            mount_volume_loop_bpb_scan_hits
                        );
                    }
                }
                if pc_aligned == 0x0815_0cf6 {
                    mount_volume_loop_root_scan_hits = mount_volume_loop_root_scan_hits.saturating_add(1);
                    if mount_volume_loop_root_scan_hits == 1 || mount_volume_loop_root_scan_hits % 1024 == 0 {
                        let r6 = uc.reg_read(RegisterARM::R6).unwrap_or(0);
                        let r7 = uc.reg_read(RegisterARM::R7).unwrap_or(0);
                        info!(
                            "TRACE mount_volume loop[root_scan] hits={} pc=0x{pc32:08x} ofs_r6=0x{r6:08x} entries_r7=0x{r7:08x}",
                            mount_volume_loop_root_scan_hits
                        );
                    }
                }
                if pc_aligned == 0x0815_0d54 {
                    mount_volume_loop_fat_chain_hits = mount_volume_loop_fat_chain_hits.saturating_add(1);
                    if mount_volume_loop_fat_chain_hits == 1 || mount_volume_loop_fat_chain_hits % 1024 == 0 {
                        let r6 = uc.reg_read(RegisterARM::R6).unwrap_or(0);
                        let r1 = uc.reg_read(RegisterARM::R1).unwrap_or(0);
                        info!(
                            "TRACE mount_volume loop[fat_chain] hits={} pc=0x{pc32:08x} cluster_r6=0x{r6:08x} sector_r1=0x{r1:08x}",
                            mount_volume_loop_fat_chain_hits
                        );
                    }
                }
                if !saw_mount_volume_return && pc_aligned == 0x0815_0e9a {
                    saw_mount_volume_return = true;
                    let r5 = uc.reg_read(RegisterARM::R5).unwrap_or(0);
                    info!("TRACE mount_volume return path reached pc=0x{pc32:08x} fr=0x{r5:08x}");
                }

                if busy_loop_stop {
                    let current_pc = pc as u32;
                    if busy_loop_pc == Some(current_pc) {
                        busy_loop_streak = busy_loop_streak.saturating_add(1);
                    } else {
                        busy_loop_pc = Some(current_pc);
                        busy_loop_streak = 1;
                    }
                }

                if busy_loop_stop && busy_loop_streak >= BUSY_LOOP_STREAK_THRESHOLD {
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
                    info!("Busy loop reached pc=0x{:08x} streak={} sp=0x{:08x} lr=0x{:08x} r0=0x{:08x} r1=0x{:08x} r2=0x{:08x} r3=0x{:08x} r4=0x{:08x} r5=0x{:08x} r6=0x{:08x} r7=0x{:08x} primask=0x{:08x} basepri=0x{:08x} control=0x{:08x} ipsr=0x{:08x}",
                        pc, busy_loop_streak, sp, lr, r0, r1, r2, r3, r4, r5, r6, r7, primask, basepri, control, ipsr);
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
                let pending_irq = {
                    p.nvic.borrow_mut().take_pending_interrupt(&sys)
                };
                if let Some(irq) = pending_irq {
                    if irq == 67 {
                        debug!("EMULATOR code_hook: deferred IRQ 67, will dispatch after emu_stop");
                    }
                    let deferred_was_empty = {
                        let mut deferred = deferred_irq.borrow_mut();
                        if deferred.is_none() {
                            *deferred = Some(irq);
                            true
                        } else {
                            false
                        }
                    };
                    if !deferred_was_empty {
                        p.nvic.borrow_mut().set_intr_pending(irq);
                    }
                    CONTINUE_EXECUTION.store(true, Ordering::Release);
                    uc.emu_stop().unwrap();
                    return;
                }
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
        let deferred_irq = deferred_irq.clone();
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
                    let pending_irq = {
                        p.nvic.borrow_mut().take_pending_interrupt(&sys)
                    };
                    if let Some(irq) = pending_irq {
                        let deferred_was_empty = {
                            let mut deferred = deferred_irq.borrow_mut();
                            if deferred.is_none() {
                                *deferred = Some(irq);
                                true
                            } else {
                                false
                            }
                        };
                        if !deferred_was_empty {
                            p.nvic.borrow_mut().set_intr_pending(irq);
                        }
                        CONTINUE_EXECUTION.store(true, Ordering::Release);
                        uc.emu_stop().unwrap();
                    }
                }
                3 | 4 => {
                    // EXCP_PREFETCH_ABORT (3) or EXCP_DATA_ABORT (4): fatal fault in emulated CPU.
                    // Logging and continuing causes Unicorn to retry the faulting instruction
                    // forever, producing unbounded output. Exit immediately.
                    // pc=0x55555554 means stack overflow (ChibiOS fill pattern 0x55555555 loaded as PC).
                    let pc = uc.reg_read(unicorn_engine::RegisterARM::PC).unwrap_or(0);
                    let sp = uc.reg_read(unicorn_engine::RegisterARM::SP).unwrap_or(0);
                    let msp = uc.reg_read(unicorn_engine::RegisterARM::MSP).unwrap_or(0);
                    let psp = uc.reg_read(unicorn_engine::RegisterARM::PSP).unwrap_or(0);
                    let lr = uc.reg_read(unicorn_engine::RegisterARM::LR).unwrap_or(0);
                    let r0 = uc.reg_read(unicorn_engine::RegisterARM::R0).unwrap_or(0);
                    let r1 = uc.reg_read(unicorn_engine::RegisterARM::R1).unwrap_or(0);
                    let r2 = uc.reg_read(unicorn_engine::RegisterARM::R2).unwrap_or(0);
                    let r3 = uc.reg_read(unicorn_engine::RegisterARM::R3).unwrap_or(0);
                    let r12 = uc.reg_read(unicorn_engine::RegisterARM::R12).unwrap_or(0);
                    let primask = uc.reg_read(unicorn_engine::RegisterARM::PRIMASK).unwrap_or(0);
                    let basepri = uc.reg_read(unicorn_engine::RegisterARM::BASEPRI).unwrap_or(0);
                    let faultmask = uc.reg_read(unicorn_engine::RegisterARM::FAULTMASK).unwrap_or(0);
                    let control = uc.reg_read(unicorn_engine::RegisterARM::CONTROL).unwrap_or(0);
                    let ipsr = uc.reg_read(unicorn_engine::RegisterARM::IPSR).unwrap_or(0);
                    error!(
                        "intr_hook intno={:08x} (fatal fault) pc=0x{:08x} sp=0x{:08x} msp=0x{:08x} psp=0x{:08x} lr=0x{:08x} r0=0x{:08x} r1=0x{:08x} r2=0x{:08x} r3=0x{:08x} r12=0x{:08x} primask=0x{:08x} basepri=0x{:08x} faultmask=0x{:08x} control=0x{:08x} ipsr=0x{:08x}",
                        exception, pc, sp, msp, psp, lr, r0, r1, r2, r3, r12, primask, basepri, faultmask, control, ipsr
                    );
                    dump_stack_from(uc, sp, 24, "active");
                    if msp != sp {
                        dump_stack_from(uc, msp, 24, "msp");
                    }
                    if psp != sp && psp != msp {
                        dump_stack_from(uc, psp, 24, "psp");
                    }
                    if pc == 0x55555554 {
                        error!("Stack overflow detected (ChibiOS fill pattern at PC). Thread stack exhausted.");
                    }
                    std::process::exit(1);
                }
                _ => {
                    error!("intr_hook intno={:08x}", exception);
                    std::process::exit(1);
                }
            }
        }).expect("add_intr_hook failed");
    }

    sys.uc.borrow_mut().add_mem_hook(HookType::MEM_UNMAPPED, 0, u64::MAX, |uc, type_, addr, size, value| {
        if type_ == MemType::WRITE_UNMAPPED {
            warn!("{:?} addr=0x{:08x} size={} value=0x{:08x}", type_, addr, size, value);
        } else {
            warn!("{:?} addr=0x{:08x} size={}", type_, addr, size);
        }

        let pc = uc.reg_read(RegisterARM::PC).expect("failed to get pc");

        unsafe {
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

    let vector_table = VectorTable::from_memory(&sys.uc.borrow(), vector_table_addr)?;
    let mut pc = vector_table.reset as u64;
    sys.uc.borrow_mut().reg_write(RegisterARM::SP, vector_table.sp.into()).map_err(UniErr)?;
    // Keep architectural state coherent for debugger attach before first emu_start.
    sys.uc.borrow_mut().reg_write(RegisterARM::PC, thumb(pc)).map_err(UniErr)?;

    // ── GDB mode: hand off to GDB server instead of running the normal loop ──
    if let Some(gdb_port) = args.gdb {
        let p = sys.p.clone();
        let d = sys.d.clone();
        // Drop sys to release the &mut borrow on uc before we pass uc to GdbTarget.
        drop(sys);

        let target = crate::gdb::GdbTarget::new(
            &mut uc,
            p,
            d,
            deferred_irq,
            &gdb_shared,
            pc,
            args.stop_addr,
        );

        info!("GDB mode: emulator paused at reset vector 0x{:08x}", pc);
        crate::gdb::run_gdb_server(target, gdb_port)?;

        for fb in framebuffers.images {
            fb.borrow().write_to_disk()?;
        }
        return Ok(());
    }
    // ── Normal run loop ───────────────────────────────────────────────────────

    info!("Starting emulation");

    loop {
        let max_instructions = args.max_instructions.map(|c|
            // yes, we want to panic if this goes negative.
            c - NUM_INSTRUCTIONS.load(Ordering::Relaxed)
        );
        if max_instructions == Some(0) {
            crate::peripherals::otg_fs::flush_cdc_pending_output();
            info!("Reached target number of instructions. Done");
            break;
        }

        let result = {
            let mut uc = sys.uc.borrow_mut();
            uc.emu_start(
                pc,
                args.stop_addr.unwrap_or(0) as u64,
                0,
                max_instructions.unwrap_or(0) as usize,
            ).map_err(UniErr)
        };
        pc = sys.uc.borrow().reg_read(RegisterARM::PC).expect("failed to get pc");

        if let Some(mut irq) = deferred_irq.borrow_mut().take() {
            let selected_irq = {
                sys.p.nvic.borrow_mut().take_pending_interrupt(&sys)
            };
            if let Some(selected) = selected_irq {
                if selected != irq {
                    sys.p.nvic.borrow_mut().set_intr_pending(irq);
                    irq = selected;
                }
            }

            if irq == 67 {
                debug!("EMULATOR outer_loop: dispatching deferred IRQ 67 (NUM_INSTRUCTIONS={})", NUM_INSTRUCTIONS.load(Ordering::Relaxed));
            }
            sys.p.nvic.borrow_mut().run_interrupt(&sys, irq);
            pc = sys.uc.borrow().reg_read(RegisterARM::PC).expect("failed to get pc after deferred irq");
        }

        if CONTINUE_EXECUTION.swap(false, Ordering::AcqRel) {
            if crate::verbose() >= 3 {
                trace!("Resuming execution pc={:08x}", pc);
            }
            pc = thumb(pc);
            continue;
        }

        if STOP_REQUESTED.load(Ordering::Relaxed) {
            crate::peripherals::otg_fs::flush_cdc_pending_output();
            info!("Stop requested");
            break;
        }

        if let Err(e) = result {
            bail!(e);
        }

        if args.stop_addr == Some(pc as u32) {
            let uc = sys.uc.borrow();
            let r0 = uc.reg_read(RegisterARM::R0).unwrap_or(0);
            let r1 = uc.reg_read(RegisterARM::R1).unwrap_or(0);
            let r2 = uc.reg_read(RegisterARM::R2).unwrap_or(0);
            let r3 = uc.reg_read(RegisterARM::R3).unwrap_or(0);
            let r4 = uc.reg_read(RegisterARM::R4).unwrap_or(0);
            let lr = uc.reg_read(RegisterARM::LR).unwrap_or(0);

            info!(
                "Stop address reached at pc=0x{pc:08x} r0=0x{r0:08x} r1=0x{r1:08x} r2=0x{r2:08x} r3=0x{r3:08x} r4=0x{r4:08x} lr=0x{lr:08x}"
            );

            // UART begin probe helper: when halted near AP_HAL::UARTDriver::begin(),
            // decode the virtual target from object vtable slot +0x84.
            if pc == 0x0804_59e4 || pc == 0x0804_59e8 {
                let mut vtbl_buf = [0u8; 4];
                if uc.mem_read(r0, &mut vtbl_buf).is_ok() {
                    let vtbl = u32::from_le_bytes(vtbl_buf) as u64;
                    let mut fn_buf = [0u8; 4];
                    if uc.mem_read(vtbl + 0x84, &mut fn_buf).is_ok() {
                        let callee = u32::from_le_bytes(fn_buf);
                        info!(
                            "UART begin dispatch: obj=0x{r0:08x} vtbl=0x{vtbl:08x} callee@+0x84=0x{callee:08x}"
                        );
                    }
                }
            }

            if pc == 0x080c_213c || pc == 0x080c_2140 {
                let read_u32 = |addr: u64| -> Option<u32> {
                    let mut b = [0u8; 4];
                    if uc.mem_read(addr, &mut b).is_ok() {
                        Some(u32::from_le_bytes(b))
                    } else {
                        None
                    }
                };
                if let Some(fs_obj) = read_u32(r0) {
                    if let Some(vtbl) = read_u32(fs_obj as u64) {
                        if let Some(callee) = read_u32(vtbl as u64 + 0x44) {
                            info!(
                                "Filesystem retry_mount dispatch: fs_obj_ptr=0x{r0:08x} obj=0x{fs_obj:08x} vtbl=0x{vtbl:08x} callee@+0x44=0x{callee:08x}"
                            );
                        }
                    }
                }
            }

            let pc_aligned = (pc as u32) & !1;
            if (0x0807_709c..=0x0807_7110).contains(&pc_aligned) {
                let read_u16 = |addr: u64| -> Option<u16> {
                    let mut b = [0u8; 2];
                    if uc.mem_read(addr, &mut b).is_ok() {
                        Some(u16::from_le_bytes(b))
                    } else {
                        None
                    }
                };
                let read_u32 = |addr: u64| -> Option<u32> {
                    let mut b = [0u8; 4];
                    if uc.mem_read(addr, &mut b).is_ok() {
                        Some(u32::from_le_bytes(b))
                    } else {
                        None
                    }
                };
                let num_vars = read_u16(0x2000_d330);
                let first_free = read_u16(0x2000_d304);
                let var_info = read_u32(0x2000_d314);
                let defaults_info = read_u32(0x2000_d334);
                info!(
                    "AP_Param loop state: num_vars={:?} first_free={:?} var_info_ptr={:?} defaults_ptr={:?}",
                    num_vars,
                    first_free,
                    var_info,
                    defaults_info
                );
            }

            info!("Stop address reached, stopping");
            break;
        }

        if BUSY_LOOP_REACHED.load(Ordering::Relaxed) {
            break;
        }
    }

    if let Some(n) = args.dump_stack {
        dump_stack(&mut sys.uc.borrow_mut(), n);
    }

    for fb in framebuffers.images {
        fb.borrow().write_to_disk()?;
    }

    Ok(())
}
