// SPDX-License-Identifier: GPL-3.0-or-later

use unicorn_engine::{RegisterARM, Unicorn};

#[derive(Default)]
pub struct TraceState {
    saw_uart_begin_impl: bool,
    saw_uart_thread_rx_init: bool,
    saw_thread_create_alloc: bool,
    saw_uart_begin_stage_8032: bool,
    saw_uart_begin_stage_8130: bool,
    saw_uart_begin_stage_8188: bool,
    saw_uart_begin_return: bool,
    saw_mainloop_post_begin: bool,
    saw_mainloop_post_analog: bool,
    saw_vehicle_setup: bool,
    saw_init_console: bool,
    saw_ap_param_setup: bool,
    saw_ap_param_setup_post_read: bool,
    saw_ap_param_erase_all: bool,
    saw_ap_param_eeprom_write_check: bool,
    saw_storageaccess_write_block: bool,
    saw_chibios_storage_write_block: bool,
    saw_storageaccess_read_block: bool,
    saw_chibios_storage_read_block: bool,
    saw_chibios_storage_open: bool,
    saw_ramtron_init: bool,
    saw_ramtron_read: bool,
    saw_ramtron_transfer_call: bool,
    saw_ramtron_transfer_return: bool,
    saw_spidevice_transfer: bool,
    saw_spidevice_do_transfer: bool,
    saw_storage_open_post_ramtron_read: bool,
    saw_storage_save_backup: bool,
    saw_storage_open_post_save_backup: bool,
    saw_save_backup_loop_start: bool,
    saw_save_backup_after_mount_wait: bool,
    saw_save_backup_open_last: bool,
    saw_save_backup_open_data: bool,
    saw_save_backup_write_data: bool,
    saw_save_backup_done_path: bool,
    save_backup_wait_seen: bool,
    save_backup_wait_last_delta: u32,
    saw_f_mount_entry: bool,
    saw_mount_volume_entry: bool,
    saw_mount_volume_after_find: bool,
    saw_mount_volume_return: bool,
    saw_mount_volume_boundary_check: bool,
    saw_mount_volume_boundary_fail: bool,
    saw_mount_volume_totsec: bool,
    saw_mount_volume_exfat_fields: bool,
    mount_volume_loop_bpb_scan_hits: u32,
    mount_volume_loop_root_scan_hits: u32,
    mount_volume_loop_fat_chain_hits: u32,
    saw_find_volume_entry: bool,
    saw_find_volume_return: bool,
    saw_move_window_entry: bool,
    saw_move_window_post_disk_read: bool,
    saw_disk_read_entry: bool,
    saw_disk_read_call_driver: bool,
    saw_disk_read_return_from_driver: bool,
    disk_read_retry_hits: u32,
    port_switch_trace_hits: u32,
}

impl TraceState {
    fn read_u32(uc: &Unicorn<()>, addr: u64) -> Option<u32> {
        let mut b = [0u8; 4]; // 32-bit little-endian word width
        if uc.mem_read(addr, &mut b).is_ok() {
            Some(u32::from_le_bytes(b))
        } else {
            None
        }
    }

    pub fn on_instruction(&mut self, uc: &Unicorn<()>, pc32: u32) {
        let pc_aligned = pc32 & !1;
        if self.port_switch_trace_hits < 16 && pc_aligned == 0x0800_5438 { // __port_switch prologue entry PC
            self.port_switch_trace_hits += 1;
            let r0 = uc.reg_read(RegisterARM::R0).unwrap_or(0);
            let r1 = uc.reg_read(RegisterARM::R1).unwrap_or(0);
            let old_ctx_sp = Self::read_u32(uc, r1 + 12).unwrap_or(0); // thread_t.ctx.sp field offset
            let new_ctx_sp = Self::read_u32(uc, r0 + 12).unwrap_or(0); // thread_t.ctx.sp field offset
            info!(
                "TRACE __port_switch entry hit={} r0(new)=0x{r0:08x} r1(old)=0x{r1:08x} old->sp=0x{old_ctx_sp:08x} new->sp=0x{new_ctx_sp:08x}",
                self.port_switch_trace_hits
            );
        }
        if pc_aligned == 0x0800_544c { // __port_switch ldmia ...,{...,pc} site
            if self.port_switch_trace_hits < 32 { // cap detailed per-switch dumps
                self.port_switch_trace_hits += 1;
            }
            let sp = uc.reg_read(RegisterARM::SP).unwrap_or(0);
            let r4 = Self::read_u32(uc, sp).unwrap_or(0);
            let r5 = Self::read_u32(uc, sp + 4).unwrap_or(0); // stacked callee-saved slot +0x04
            let r6 = Self::read_u32(uc, sp + 8).unwrap_or(0); // stacked callee-saved slot +0x08
            let r7 = Self::read_u32(uc, sp + 12).unwrap_or(0); // stacked callee-saved slot +0x0C
            let r8 = Self::read_u32(uc, sp + 16).unwrap_or(0); // stacked callee-saved slot +0x10
            let r9 = Self::read_u32(uc, sp + 20).unwrap_or(0); // stacked callee-saved slot +0x14
            let r10 = Self::read_u32(uc, sp + 24).unwrap_or(0); // stacked callee-saved slot +0x18
            let r11 = Self::read_u32(uc, sp + 28).unwrap_or(0); // stacked callee-saved slot +0x1C
            let pc_word = Self::read_u32(uc, sp + 32).unwrap_or(0); // return PC loaded by ldmia ...,{...,pc}
            if self.port_switch_trace_hits <= 32 { // keep verbose frame dumps bounded
                info!(
                    "TRACE __port_switch ldmia hit={} sp=0x{sp:08x} r4=0x{r4:08x} r5=0x{r5:08x} r6=0x{r6:08x} r7=0x{r7:08x} r8=0x{r8:08x} r9=0x{r9:08x} r10=0x{r10:08x} r11=0x{r11:08x} pc_word=0x{pc_word:08x}",
                    self.port_switch_trace_hits
                );
            }
            if !(0x0800_0001..=0x081f_ffff).contains(&pc_word) { // expected Thumb address window for CubeBlack firmware text
                let r0 = uc.reg_read(RegisterARM::R0).unwrap_or(0);
                let r1 = uc.reg_read(RegisterARM::R1).unwrap_or(0);
                let new_ctx_sp = Self::read_u32(uc, r0 + 12).unwrap_or(0); // thread_t.ctx.sp field offset
                warn!(
                    "TRACE __port_switch invalid next PC candidate pc_word=0x{pc_word:08x} sp=0x{sp:08x} r0(new)=0x{r0:08x} r1(old)=0x{r1:08x} [r0+12]=0x{new_ctx_sp:08x}"
                );
            }
        }
        if !self.saw_uart_begin_impl && pc_aligned == 0x0814_7fd4 {
            self.saw_uart_begin_impl = true;
            let lr = uc.reg_read(RegisterARM::LR).unwrap_or(0);
            info!("TRACE UART begin impl reached pc=0x{pc32:08x} lr=0x{lr:08x}");
        }
        if !self.saw_uart_thread_rx_init && pc_aligned == 0x0814_7170 {
            self.saw_uart_thread_rx_init = true;
            let lr = uc.reg_read(RegisterARM::LR).unwrap_or(0);
            info!("TRACE UART thread_rx_init reached pc=0x{pc32:08x} lr=0x{lr:08x}");
        }
        if !self.saw_thread_create_alloc && pc_aligned == 0x0815_aa28 {
            self.saw_thread_create_alloc = true;
            let lr = uc.reg_read(RegisterARM::LR).unwrap_or(0);
            info!("TRACE thread_create_alloc reached pc=0x{pc32:08x} lr=0x{lr:08x}");
        }
        if !self.saw_uart_begin_stage_8032 && pc_aligned == 0x0814_8032 {
            self.saw_uart_begin_stage_8032 = true;
            info!("TRACE UART _begin stage reached pc=0x{pc32:08x}");
        }
        if !self.saw_uart_begin_stage_8130 && pc_aligned == 0x0814_8130 {
            self.saw_uart_begin_stage_8130 = true;
            info!("TRACE UART _begin stage reached pc=0x{pc32:08x}");
        }
        if !self.saw_uart_begin_stage_8188 && pc_aligned == 0x0814_8188 {
            self.saw_uart_begin_stage_8188 = true;
            info!("TRACE UART _begin stage reached pc=0x{pc32:08x}");
        }
        if !self.saw_uart_begin_return && pc_aligned == 0x0814_81c0 {
            self.saw_uart_begin_return = true;
            info!("TRACE UART _begin return reached pc=0x{pc32:08x}");
        }
        if !self.saw_mainloop_post_begin && pc_aligned == 0x080f_27cc {
            self.saw_mainloop_post_begin = true;
            info!("TRACE main_loop post-begin reached pc=0x{pc32:08x}");
        }
        if !self.saw_mainloop_post_analog && pc_aligned == 0x080f_27d4 {
            self.saw_mainloop_post_analog = true;
            info!("TRACE main_loop post-analoginit reached pc=0x{pc32:08x}");
        }
        if !self.saw_vehicle_setup && pc_aligned == 0x0808_2f48 {
            self.saw_vehicle_setup = true;
            info!("TRACE AP_Vehicle::setup reached pc=0x{pc32:08x}");
        }
        if !self.saw_init_console && pc_aligned == 0x0808_0f38 {
            self.saw_init_console = true;
            info!("TRACE AP_SerialManager::init_console reached pc=0x{pc32:08x}");
        }
        if !self.saw_ap_param_setup && pc_aligned == 0x0807_5748 {
            self.saw_ap_param_setup = true;
            info!("TRACE AP_Param::setup reached pc=0x{pc32:08x}");
        }
        if !self.saw_ap_param_setup_post_read && pc_aligned == 0x0807_575c {
            self.saw_ap_param_setup_post_read = true;
            info!("TRACE AP_Param::setup post-read reached pc=0x{pc32:08x}");
        }
        if !self.saw_ap_param_erase_all && pc_aligned == 0x0807_5714 {
            self.saw_ap_param_erase_all = true;
            info!("TRACE AP_Param::erase_all reached pc=0x{pc32:08x}");
        }
        if !self.saw_ap_param_eeprom_write_check && pc_aligned == 0x0807_4e34 {
            self.saw_ap_param_eeprom_write_check = true;
            info!("TRACE AP_Param::eeprom_write_check reached pc=0x{pc32:08x}");
        }
        if !self.saw_storageaccess_write_block && pc_aligned == 0x0809_737c {
            self.saw_storageaccess_write_block = true;
            info!("TRACE StorageAccess::write_block reached pc=0x{pc32:08x}");
        }
        if !self.saw_chibios_storage_write_block && pc_aligned == 0x080f_2dc4 {
            self.saw_chibios_storage_write_block = true;
            info!("TRACE ChibiOS::Storage::write_block reached pc=0x{pc32:08x}");
        }
        if !self.saw_storageaccess_read_block && pc_aligned == 0x0809_72f4 {
            self.saw_storageaccess_read_block = true;
            info!("TRACE StorageAccess::read_block reached pc=0x{pc32:08x}");
        }
        if !self.saw_chibios_storage_read_block && pc_aligned == 0x080f_2d38 {
            self.saw_chibios_storage_read_block = true;
            info!("TRACE ChibiOS::Storage::read_block reached pc=0x{pc32:08x}");
        }
        if !self.saw_chibios_storage_open && pc_aligned == 0x080f_2cdc {
            self.saw_chibios_storage_open = true;
            info!("TRACE ChibiOS::Storage::_storage_open reached pc=0x{pc32:08x}");
        }
        if !self.saw_ramtron_init && pc_aligned == 0x0813_8a34 {
            self.saw_ramtron_init = true;
            info!("TRACE AP_RAMTRON::init reached pc=0x{pc32:08x}");
        }
        if !self.saw_ramtron_read && pc_aligned == 0x0813_8bc8 {
            self.saw_ramtron_read = true;
            info!("TRACE AP_RAMTRON::read reached pc=0x{pc32:08x}");
        }
        if !self.saw_ramtron_transfer_call && pc_aligned == 0x0813_8ca6 {
            self.saw_ramtron_transfer_call = true;
            info!("TRACE AP_RAMTRON::read transfer call reached pc=0x{pc32:08x}");
        }
        if !self.saw_ramtron_transfer_return && pc_aligned == 0x0813_8ca8 {
            self.saw_ramtron_transfer_return = true;
            info!("TRACE AP_RAMTRON::read transfer return reached pc=0x{pc32:08x}");
        }
        if !self.saw_spidevice_transfer && pc_aligned == 0x0814_5904 {
            self.saw_spidevice_transfer = true;
            info!("TRACE ChibiOS::SPIDevice::transfer_fullduplex reached pc=0x{pc32:08x}");
        }
        if !self.saw_spidevice_do_transfer && pc_aligned == 0x0814_5700 {
            self.saw_spidevice_do_transfer = true;
            info!("TRACE ChibiOS::SPIDevice::do_transfer reached pc=0x{pc32:08x}");
        }
        if !self.saw_storage_open_post_ramtron_read && pc_aligned == 0x080f_2d14 {
            self.saw_storage_open_post_ramtron_read = true;
            info!("TRACE ChibiOS::Storage::_storage_open post-RAMTRON-read reached pc=0x{pc32:08x}");
        }
        if !self.saw_storage_save_backup && pc_aligned == 0x080f_2b18 {
            self.saw_storage_save_backup = true;
            info!("TRACE ChibiOS::Storage::_save_backup reached pc=0x{pc32:08x}");
        }
        if !self.saw_storage_open_post_save_backup && pc_aligned == 0x080f_2d1c {
            self.saw_storage_open_post_save_backup = true;
            info!("TRACE ChibiOS::Storage::_storage_open post-save-backup reached pc=0x{pc32:08x}");
        }
        if !self.saw_save_backup_loop_start && pc_aligned == 0x080f_2b50 {
            self.saw_save_backup_loop_start = true;
            info!("TRACE _save_backup loop start reached pc=0x{pc32:08x}");
        }
        if !self.saw_save_backup_after_mount_wait && pc_aligned == 0x080f_2b72 {
            self.saw_save_backup_after_mount_wait = true;
            info!("TRACE _save_backup after mount-wait reached pc=0x{pc32:08x}");
        }
        if !self.saw_save_backup_open_last && pc_aligned == 0x080f_2ba0 {
            self.saw_save_backup_open_last = true;
            info!("TRACE _save_backup open(last_storage_bak) reached pc=0x{pc32:08x}");
        }
        if !self.saw_save_backup_open_data && pc_aligned == 0x080f_2c88 {
            self.saw_save_backup_open_data = true;
            info!("TRACE _save_backup open(STRG*.bak) reached pc=0x{pc32:08x}");
        }
        if !self.saw_save_backup_write_data && pc_aligned == 0x080f_2cb4 {
            self.saw_save_backup_write_data = true;
            info!("TRACE _save_backup write(storage_blob) reached pc=0x{pc32:08x}");
        }
        if !self.saw_save_backup_done_path && pc_aligned == 0x080f_2c9e {
            self.saw_save_backup_done_path = true;
            info!("TRACE _save_backup done-path reached pc=0x{pc32:08x}");
        }
        if pc_aligned == 0x080f_2b5e {
            let now_ms = uc.reg_read(RegisterARM::R0).unwrap_or(0) as u32;
            let start_ms = uc.reg_read(RegisterARM::R5).unwrap_or(0) as u32;
            let delta = now_ms.wrapping_sub(start_ms);
            if !self.save_backup_wait_seen || delta >= self.save_backup_wait_last_delta.saturating_add(100) {
                self.save_backup_wait_seen = true;
                self.save_backup_wait_last_delta = delta;
                info!(
                    "TRACE _save_backup wait progress now_ms={} start_ms={} delta_ms={}",
                    now_ms,
                    start_ms,
                    delta
                );
            }
        }
        if !self.saw_f_mount_entry && pc_aligned == 0x0815_2a3c {
            self.saw_f_mount_entry = true;
            let lr = uc.reg_read(RegisterARM::LR).unwrap_or(0);
            let r2 = uc.reg_read(RegisterARM::R2).unwrap_or(0);
            info!("TRACE f_mount entry reached pc=0x{pc32:08x} lr=0x{lr:08x} opt=0x{r2:08x}");
        }
        if !self.saw_mount_volume_entry && pc_aligned == 0x0815_0ab0 {
            self.saw_mount_volume_entry = true;
            let lr = uc.reg_read(RegisterARM::LR).unwrap_or(0);
            let r0 = uc.reg_read(RegisterARM::R0).unwrap_or(0);
            let r1 = uc.reg_read(RegisterARM::R1).unwrap_or(0);
            let r2 = uc.reg_read(RegisterARM::R2).unwrap_or(0);
            info!(
                "TRACE mount_volume entry reached pc=0x{pc32:08x} lr=0x{lr:08x} path=0x{r0:08x} rfs_ptr=0x{r1:08x} mode=0x{r2:08x}"
            );
        }
        if !self.saw_mount_volume_after_find && pc_aligned == 0x0815_0b22 {
            self.saw_mount_volume_after_find = true;
            let r0 = uc.reg_read(RegisterARM::R0).unwrap_or(0);
            info!("TRACE mount_volume post-find_volume reached pc=0x{pc32:08x} find_volume_res=0x{r0:08x}");
        }
        if !self.saw_mount_volume_boundary_check && pc_aligned == 0x0815_0de2 {
            self.saw_mount_volume_boundary_check = true;
            let r7 = uc.reg_read(RegisterARM::R7).unwrap_or(0);
            let r9 = uc.reg_read(RegisterARM::R9).unwrap_or(0);
            let r3 = uc.reg_read(RegisterARM::R3).unwrap_or(0);
            info!(
                "TRACE mount_volume boundary-check reached pc=0x{pc32:08x} r7=0x{r7:08x} r9=0x{r9:08x} r3=0x{r3:08x}"
            );
        }
        if !self.saw_mount_volume_boundary_fail && pc_aligned == 0x0815_0f12 {
            self.saw_mount_volume_boundary_fail = true;
            let r7 = uc.reg_read(RegisterARM::R7).unwrap_or(0);
            let r9 = uc.reg_read(RegisterARM::R9).unwrap_or(0);
            info!(
                "TRACE mount_volume boundary-fail reached pc=0x{pc32:08x} r7=0x{r7:08x} r9=0x{r9:08x}"
            );
        }
        if !self.saw_mount_volume_totsec && pc_aligned == 0x0815_0b5a {
            self.saw_mount_volume_totsec = true;
            let r7 = uc.reg_read(RegisterARM::R7).unwrap_or(0);
            let r2 = uc.reg_read(RegisterARM::R2).unwrap_or(0);
            let r9 = uc.reg_read(RegisterARM::R9).unwrap_or(0);
            info!(
                "TRACE mount_volume FAT totals reached pc=0x{pc32:08x} totsec_r7=0x{r7:08x} secperclus_r2=0x{r2:08x} rootent_r9=0x{r9:08x}"
            );
        }
        if !self.saw_mount_volume_exfat_fields && pc_aligned == 0x0815_0c48 {
            self.saw_mount_volume_exfat_fields = true;
            let r0 = uc.reg_read(RegisterARM::R0).unwrap_or(0);
            let r1 = uc.reg_read(RegisterARM::R1).unwrap_or(0);
            let r6 = uc.reg_read(RegisterARM::R6).unwrap_or(0);
            let r7 = uc.reg_read(RegisterARM::R7).unwrap_or(0);
            info!(
                "TRACE mount_volume exfat fields reached pc=0x{pc32:08x} volofs_lo_r0=0x{r0:08x} volofs_hi_r1=0x{r1:08x} partbase_r6=0x{r6:08x} volsct_lo_r7=0x{r7:08x}"
            );
        }
        if !self.saw_find_volume_entry && pc_aligned == 0x0815_0a40 {
            self.saw_find_volume_entry = true;
            let lr = uc.reg_read(RegisterARM::LR).unwrap_or(0);
            let r1 = uc.reg_read(RegisterARM::R1).unwrap_or(0);
            info!("TRACE find_volume entry reached pc=0x{pc32:08x} lr=0x{lr:08x} mode=0x{r1:08x}");
        }
        if !self.saw_find_volume_return && pc_aligned == 0x0815_0aac {
            self.saw_find_volume_return = true;
            let r0 = uc.reg_read(RegisterARM::R0).unwrap_or(0);
            info!("TRACE find_volume return path reached pc=0x{pc32:08x} res=0x{r0:08x}");
        }
        if !self.saw_move_window_entry && pc_aligned == 0x0815_0914 {
            self.saw_move_window_entry = true;
            let r1 = uc.reg_read(RegisterARM::R1).unwrap_or(0);
            info!("TRACE move_window entry reached pc=0x{pc32:08x} sector=0x{r1:08x}");
        }
        if !self.saw_move_window_post_disk_read && pc_aligned == 0x0815_093e {
            self.saw_move_window_post_disk_read = true;
            let r0 = uc.reg_read(RegisterARM::R0).unwrap_or(0);
            info!("TRACE move_window post-disk_read reached pc=0x{pc32:08x} disk_read_res=0x{r0:08x}");
        }
        if !self.saw_disk_read_entry && pc_aligned == 0x0815_9ffc {
            self.saw_disk_read_entry = true;
            let r0 = uc.reg_read(RegisterARM::R0).unwrap_or(0);
            let r2 = uc.reg_read(RegisterARM::R2).unwrap_or(0);
            let r3 = uc.reg_read(RegisterARM::R3).unwrap_or(0);
            info!(
                "TRACE disk_read entry reached pc=0x{pc32:08x} pdrv=0x{r0:08x} sector=0x{r2:08x} count=0x{r3:08x}"
            );
        }
        if !self.saw_disk_read_call_driver && pc_aligned == 0x0815_a024 {
            self.saw_disk_read_call_driver = true;
            let r6 = uc.reg_read(RegisterARM::R6).unwrap_or(0);
            let r8 = uc.reg_read(RegisterARM::R8).unwrap_or(0);
            let r7 = uc.reg_read(RegisterARM::R7).unwrap_or(0);
            info!(
                "TRACE disk_read call driver reached pc=0x{pc32:08x} callee=0x{r6:08x} sector=0x{r8:08x} count=0x{r7:08x}"
            );
        }
        if !self.saw_disk_read_return_from_driver && pc_aligned == 0x0815_a02c {
            self.saw_disk_read_return_from_driver = true;
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
            self.disk_read_retry_hits = self.disk_read_retry_hits.saturating_add(1);
            if self.disk_read_retry_hits == 1 || self.disk_read_retry_hits % 64 == 0 {
                let r4 = uc.reg_read(RegisterARM::R4).unwrap_or(0);
                info!(
                    "TRACE disk_read retry loop hits={} pc=0x{pc32:08x} retry_idx=0x{r4:08x}",
                    self.disk_read_retry_hits
                );
            }
        }
        if pc_aligned == 0x0815_0c10 {
            self.mount_volume_loop_bpb_scan_hits = self.mount_volume_loop_bpb_scan_hits.saturating_add(1);
            if self.mount_volume_loop_bpb_scan_hits == 1 || self.mount_volume_loop_bpb_scan_hits % 1024 == 0 {
                let r3 = uc.reg_read(RegisterARM::R3).unwrap_or(0);
                info!(
                    "TRACE mount_volume loop[bpb_scan] hits={} pc=0x{pc32:08x} index_r3=0x{r3:08x}",
                    self.mount_volume_loop_bpb_scan_hits
                );
            }
        }
        if pc_aligned == 0x0815_0cf6 {
            self.mount_volume_loop_root_scan_hits = self.mount_volume_loop_root_scan_hits.saturating_add(1);
            if self.mount_volume_loop_root_scan_hits == 1 || self.mount_volume_loop_root_scan_hits % 1024 == 0 {
                let r6 = uc.reg_read(RegisterARM::R6).unwrap_or(0);
                let r7 = uc.reg_read(RegisterARM::R7).unwrap_or(0);
                info!(
                    "TRACE mount_volume loop[root_scan] hits={} pc=0x{pc32:08x} ofs_r6=0x{r6:08x} entries_r7=0x{r7:08x}",
                    self.mount_volume_loop_root_scan_hits
                );
            }
        }
        if pc_aligned == 0x0815_0d54 {
            self.mount_volume_loop_fat_chain_hits = self.mount_volume_loop_fat_chain_hits.saturating_add(1);
            if self.mount_volume_loop_fat_chain_hits == 1 || self.mount_volume_loop_fat_chain_hits % 1024 == 0 {
                let r6 = uc.reg_read(RegisterARM::R6).unwrap_or(0);
                let r1 = uc.reg_read(RegisterARM::R1).unwrap_or(0);
                info!(
                    "TRACE mount_volume loop[fat_chain] hits={} pc=0x{pc32:08x} cluster_r6=0x{r6:08x} sector_r1=0x{r1:08x}",
                    self.mount_volume_loop_fat_chain_hits
                );
            }
        }
        if !self.saw_mount_volume_return && pc_aligned == 0x0815_0e9a {
            self.saw_mount_volume_return = true;
            let r5 = uc.reg_read(RegisterARM::R5).unwrap_or(0);
            info!("TRACE mount_volume return path reached pc=0x{pc32:08x} fr=0x{r5:08x}");
        }
    }
}
