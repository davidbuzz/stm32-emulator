// SPDX-License-Identifier: GPL-3.0-or-later

use std::{rc::Rc, cell::RefCell};
use unicorn_engine::{Unicorn, unicorn_const::Permission};
use crate::{peripherals::{Peripherals, gpio::GpioPorts}, ext_devices::ExtDevices, util::{UniErr, round_up, self}, config::{Config, Region, RegionAccess}, framebuffers::Framebuffers};
use anyhow::{Context as _, Result, bail};
use svd_parser::svd::Device as SvdDevice;

// System is passed around during read/write hooks. It's more convenient than passing each thing individually.
// Maybe it should just be a global variable, and we call it a day.
pub struct System<'a, 'b> {
    // not sure how to not have a refcell here
    pub uc: RefCell<&'a mut Unicorn<'b, ()>>,
    // Sorry for the single letter variables, it just gets too verbose at times.
    pub p: Rc<Peripherals>,
    pub d: Rc<ExtDevices>,
}

impl<'a, 'b> System<'a, 'b> {
    fn new(uc: &'a mut Unicorn<'b, ()>, p: Peripherals,  d: ExtDevices) -> Self {
        Self {
            uc: RefCell::new(uc),
            p: Rc::new(p),
            d: Rc::new(d),
        }
    }

    fn bind_peripherals_to_unicorn(&mut self) -> Result<()> {
        for (start, end) in Peripherals::MEMORY_MAPS {
            let read_cb = {
                let p = self.p.clone();
                let d = self.d.clone();
                move |uc: &mut Unicorn<'_, ()>, addr, size| {
                    let mut sys = System { uc: RefCell::new(uc), p: p.clone(), d: d.clone() };
                    p.read(&mut sys, start + addr as u32, size as u8) as u64
                }
            };

            let write_cb = {
                let p = self.p.clone();
                let d = self.d.clone();
                move |uc: &mut Unicorn<'_, ()>, addr, size, value| {
                    let mut sys = System { uc: RefCell::new(uc), p: p.clone(), d: d.clone() };
                    p.write(&mut sys, start + addr as u32, size as u8, value as u32)
                }
            };

            self.uc.borrow_mut().mmio_map(start as u64, (end-start) as usize, Some(read_cb), Some(write_cb))
                .map_err(UniErr).context("Failed to mmio_map()")?;
        }

        Ok(())
    }
}

fn inferred_access(region: &Region) -> RegionAccess {
    if let Some(access) = region.access {
        return access;
    }

    match region.start {
        0x0800_0000..=0x080F_FFFF => RegionAccess::Rx,
        0x1FFF_0000..=0x1FFF_FFFF => RegionAccess::Rx,
        0x1000_0000..=0x100F_FFFF => RegionAccess::Rwx,
        0x2000_0000..=0x3FFF_FFFF => RegionAccess::Rwx,
        _ => {
            if region.name == "NULL_forgiveness" {
                RegionAccess::Rw
            } else {
                RegionAccess::Rwx
            }
        }
    }
}

fn unicorn_permission(access: RegionAccess) -> Permission {
    match access {
        RegionAccess::R => Permission::READ,
        RegionAccess::Rx => Permission::READ | Permission::EXEC,
        RegionAccess::Rw => Permission::READ | Permission::WRITE,
        RegionAccess::Rwx => Permission::ALL,
    }
}

fn validate_memory_layout(config: &Config) -> Result<()> {
    let mut regions: Vec<&Region> = config.regions.iter().collect();
    regions.sort_by_key(|region| region.start);

    for window in regions.windows(2) {
        let left = window[0];
        let right = window[1];
        let left_end = left.start.checked_add(left.size)
            .with_context(|| format!("Region {} overflows address space", left.name))?;
        if left_end > right.start {
            bail!(
                "Region overlap: {} [0x{:08x}, 0x{:08x}) overlaps {} [0x{:08x}, 0x{:08x})",
                left.name,
                left.start,
                left_end,
                right.name,
                right.start,
                right.start + right.size,
            );
        }
    }

    for patch in config.patches.as_ref().unwrap_or(&vec![]) {
        let patch_end = patch.start.checked_add(patch.data.len() as u32)
            .with_context(|| format!("Patch at 0x{:08x} overflows address space", patch.start))?;

        let region = config.regions.iter().find(|region| {
            let region_end = region.start.saturating_add(region.size);
            patch.start >= region.start && patch_end <= region_end
        }).ok_or_else(|| anyhow::anyhow!(
            "Patch at [0x{:08x}, 0x{:08x}) is outside all configured regions",
            patch.start,
            patch_end,
        ))?;

        if !region.allow_patches.unwrap_or(false) {
            bail!(
                "Patch at 0x{:08x} targets region {} but allow_patches is not enabled",
                patch.start,
                region.name,
            );
        }
    }

    Ok(())
}

fn load_memory_regions(uc: &mut Unicorn<()>, config: &Config) -> Result<()> {
    validate_memory_layout(config)?;

    for region in &config.regions {
        debug!("Mapping region start=0x{:08x} len=0x{:x} name={}",
            region.start, region.size, region.name);

        let size = round_up(region.size as usize, 4096); // magic number is from mem_map() documentation
        uc.mem_map(region.start.into(), size, Permission::ALL)
            .map_err(UniErr).with_context(||
                format!("Memory mapping of peripheral={} failed", region.name))?;

        if let Some(ref load) = region.load {
            info!("Loading file={} at base=0x{:08x}", load, region.start);
            let content = util::read_file(load)?;
            if content.len() > region.size as usize {
                bail!(
                    "Load file {} ({} bytes) exceeds region {} size 0x{:x}",
                    load,
                    content.len(),
                    region.name,
                    region.size,
                );
            }
            let content = &content[0..content.len().min(size)];
            uc.mem_write(region.start.into(), content).map_err(UniErr)?;
        }
    }

    for patch in config.patches.as_ref().unwrap_or(&vec![]) {
        uc.mem_write(patch.start.into(), &patch.data)
            .map_err(UniErr).with_context(||
                format!("Failed to apply patch at addr={}", patch.start))?;
    }

    for region in &config.regions {
        let size = round_up(region.size as usize, 4096);
        let perms = unicorn_permission(inferred_access(region));
        uc.mem_protect(region.start.into(), size, perms)
            .map_err(UniErr)
            .with_context(|| format!("Failed to protect region {}", region.name))?;
    }

    Ok(())
}

pub fn prepare<'a, 'b>(uc: &'a mut Unicorn<'b, ()>, config: Config, svd_device: SvdDevice)
-> Result<(System<'a, 'b>, Framebuffers)>
  {
    load_memory_regions(uc, &config)?;

    let framebuffers = Framebuffers::from_config(config.framebuffers.unwrap_or_default());
    let mut gpio: GpioPorts = Default::default();
    let ext_devices = config.devices.unwrap_or_default().into_ext_devices(&mut gpio, &framebuffers)?;
    let peripherals = Peripherals::from_svd(svd_device, config.peripherals.unwrap_or_default(), gpio, &ext_devices);

    let mut system = System::new(uc, peripherals, ext_devices);
    system.bind_peripherals_to_unicorn()?;
    Ok((system, framebuffers))
}
