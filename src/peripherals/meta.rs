// SPDX-License-Identifier: GPL-3.0-or-later

// Runtime SVD-driven peripheral metadata: register offsets and interrupt numbers.
// Builds DeviceMeta from the parsed SVD at startup so individual peripheral models
// can look up IRQ numbers by name instead of hardcoding chip-specific values.
// Ported and simplified from AZhurGIT fork (modules/fork-AZhurGIT/src/peripherals/meta.rs).
// See RENODE_COMPARISON.md §7 for motivation and FEATURE_GAP.md for status.

use std::collections::HashMap;
use svd_parser::svd::Device as SvdDevice;

#[derive(Clone, Default)]
pub struct PeripheralMeta {
    pub name: String,
    pub base: u32,
    offsets_by_name: HashMap<String, u32>,
}

impl PeripheralMeta {
    pub fn offset_of(&self, register_name: &str) -> Option<u32> {
        self.offsets_by_name.get(register_name).copied()
    }

    pub fn has_register(&self, register_name: &str) -> bool {
        self.offsets_by_name.contains_key(register_name)
    }
}

#[derive(Clone, Default)]
pub struct DeviceMeta {
    peripherals: HashMap<String, PeripheralMeta>,
    interrupts: HashMap<String, i32>,
}

impl DeviceMeta {
    /// Build from a parsed SVD device.  Must be called before the peripherals Vec is sorted.
    pub fn from_svd(svd_device: &SvdDevice) -> Self {
        let svd_peripherals = svd_device.peripherals.iter()
            .map(|d| (d.name.to_string(), d))
            .collect::<HashMap<_, _>>();

        let mut peripherals = HashMap::new();
        let mut interrupts = HashMap::new();

        for p in &svd_device.peripherals {
            let name = p.name.to_string();
            let base = p.base_address as u32;

            // Resolve derived-from source for register layout.
            let source = if let Some(derived_from) = p.derived_from.as_ref() {
                svd_peripherals.get(derived_from)
                    .as_ref()
                    .unwrap_or_else(|| panic!("meta: cannot find derived-from peripheral {}", derived_from))
            } else {
                p
            };

            let mut offsets_by_name = HashMap::new();
            for r in crate::util::extract_svd_registers(source) {
                offsets_by_name.insert(r.name.to_string(), r.address_offset);
            }

            peripherals.insert(name.clone(), PeripheralMeta { name, base, offsets_by_name });

            // Collect interrupts from both the source (where they're typically defined on the
            // first instance) and from p itself (derived peripherals sometimes add their own).
            for intr in &source.interrupt {
                interrupts.insert(intr.name.to_string(), intr.value as i32);
            }
            for intr in &p.interrupt {
                interrupts.insert(intr.name.to_string(), intr.value as i32);
            }
        }

        Self { peripherals, interrupts }
    }

    pub fn peripheral(&self, peripheral_name: &str) -> Option<&PeripheralMeta> {
        self.peripherals.get(peripheral_name)
    }

    pub fn peripheral_base(&self, peripheral_name: &str) -> Option<u32> {
        self.peripheral(peripheral_name).map(|p| p.base)
    }

    pub fn offset_of(&self, peripheral_name: &str, register_name: &str) -> Option<u32> {
        self.peripheral(peripheral_name)
            .and_then(|p| p.offset_of(register_name))
    }

    pub fn has_register(&self, peripheral_name: &str, register_name: &str) -> bool {
        self.peripheral(peripheral_name)
            .map(|p| p.has_register(register_name))
            .unwrap_or(false)
    }

    pub fn irq_of(&self, interrupt_name: &str) -> Option<i32> {
        self.interrupts.get(interrupt_name).copied()
    }

    pub fn interrupt_map(&self) -> &HashMap<String, i32> {
        &self.interrupts
    }
}
