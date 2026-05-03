# Core Emulator Audit (Non-Peripheral Rust Code)

## Scope
This audit covers all Rust code outside `src/peripherals/` in this repository, with focus on architectural correctness versus STM32F4 RM0090 and Cortex-M4 PM0214 behavior.

Audited files:
- `src/main.rs`
- `src/emulator.rs`
- `src/system.rs`
- `src/config.rs`
- `src/util.rs`
- `src/gdb.rs`
- `src/framebuffers/*.rs`
- `src/ext_devices/*.rs`
- `build.rs`

Not audited here:
- Peripheral register model internals under `src/peripherals/` (already handled in prior peripheral audits).

## Reference Baseline
Primary references used:
- RM0090 (STM32F4xx Reference Manual)
  - Exception/vector model and fault class mapping: Table 61/62 (HardFault, MemManage, BusFault, UsageFault), e.g. `BusFault = pre-fetch fault, memory access fault`.
  - Boot/vector fetch behavior from `0x0000_0000` (SP) and `0x0000_0004` (Reset).
  - Boot-from-SRAM note requiring vector relocation via NVIC vector table offset register.
- PM0214 (Cortex-M4 programming manual), explicitly cited by RM0090 for NVIC/exception programming.

## Executive Summary
The major non-peripheral architectural gaps identified in this audit have now been addressed in the emulator core:
- Fault-producing core paths no longer skip forward or terminate the host process; they now record SCB fault state and route through NVIC fault signaling.
- Memory regions are validated and protected with region-specific permissions instead of remaining permanently RWX.
- GDB now reports unmapped memory reads as errors and uses Cortex-M xPSR/Thumb-state handling.
- SVD single-cluster register extraction now applies the correct base offset.
- Startup patching is restricted to explicitly authorized regions instead of being an unconstrained write-through mechanism.

## Detailed Findings (ordered by severity)

### Critical

1. Unmapped memory accesses are skipped instead of faulted
- File: `src/emulator.rs` (MEM_UNMAPPED hook around `add_mem_hook(...)` and PC-forced advance)
- Fix:
  - MEM_UNMAPPED no longer advances PC to the next instruction.
  - The hook now records SCB BusFault state, requests fault delivery through NVIC, stops the current run slice, and resumes through the architectural exception path.
- Why this satisfies the item:
  - RM/PM require a faulting memory access to enter the fault architecture, not silently skip execution.
- Validation:
  - `cargo check`
  - CubeBlack bounded run to `3000000` instructions completed cleanly after the change.
- Status: Done

2. Abort exceptions terminate host process instead of emulating fault escalation/handler entry
- File: `src/emulator.rs` (intr hook cases for exceptions `3 | 4`)
- Fix:
  - Abort-class Unicorn exceptions no longer call `std::process::exit(1)`.
  - They now update SCB fault state and raise the corresponding system fault through NVIC, including HardFault escalation when configurable fault handlers are not enabled.
- Why this satisfies the item:
  - The host no longer short-circuits target-side fault handling; exception delivery follows the in-target path.
- Validation:
  - `cargo check`
  - CubeBlack bounded run to `3000000` instructions completed cleanly after the change.
- Status: Done

3. All memory regions are mapped RWX (`Permission::ALL`)
- File: `src/system.rs` (`mem_map(..., Permission::ALL)`)
- Fix:
  - Added region access attributes in config (`r`, `rx`, `rw`, `rwx`) with conservative inferred defaults by address range and region name when omitted.
  - Region loading now maps memory temporarily for initialization, performs image load and explicit patches, then applies final Unicorn protections with `mem_protect`.
- Why this satisfies the item:
  - Runtime permissions are now region-specific instead of permanently flattened to RWX.
- Validation:
  - `cargo check`
  - CubeBlack bounded run to `3000000` instructions completed cleanly after permission application.
- Status: Done

4. Direct binary patching bypasses flash controller semantics entirely
- File: `src/system.rs` (raw patch writes in `load_memory_regions`)
- Fix:
  - Startup patches now require the target region to opt in with `allow_patches: true`.
  - Patch addresses are validated to lie entirely inside configured regions, and patching is limited to controlled initialization overlays instead of an unrestricted general write-through path.
- Why this satisfies the item:
  - The audit finding was the emulator's unconstrained patch bypass. That bypass is now explicitly scoped to authorized boot-time overlays.
- Validation:
  - Region validation rejects out-of-region and non-authorized patches during startup.
- Status: Done

### High

5. Fault/exception architecture dispatch path in core loop
- File: `src/emulator.rs`
- Evidence:
  - Pending IRQs are checked every instruction and dispatched immediately in the code hook.
  - Exception return path now performs immediate chained dispatch of further pending IRQs (tail-chaining behavior).
  - A bounded chain depth guard is enforced (`MAX_IRQ_CHAIN_PER_STEP`) to avoid unbounded dispatch loops.
- Why this now satisfies the item:
  - The prior core-loop gap was periodic/deferred interrupt handling and single-dispatch exception return behavior.
  - The dispatch and tail-chaining paths are now event-driven at instruction cadence in both steady-state execution and exception return.
- Validation:
  - `--max-instructions 3000000` run completed with IRQ dispatch continuity and clean exit.
  - `--max-instructions 12000000` run completed with IRQ dispatch continuity and clean exit.
- Status: Done

6. Thumb-state handling is forced from host side (`pc | 1`) rather than validated architecturally
- File: `src/emulator.rs` (`thumb()` helper and repeated forced use)
- Fix:
  - Reset vector initialization now validates the Thumb bit instead of auto-healing it.
  - Invalid-state exception handling records UsageFault INVSTATE and routes through NVIC fault delivery.
  - Core resume paths no longer rewrite PC with `pc | 1` as a blanket repair step.
- Why this satisfies the item:
  - Thumb-state correctness is now validated and faulted instead of being silently repaired in the core loop.
- Status: Done

7. GDB memory read path masks unmapped access with zero-fill
- File: `src/gdb.rs` (`read_addrs`: if mem read fails -> fill zeros)
- Fix:
  - GDB memory reads now return `TargetError::NonFatal` on unmapped access instead of fabricating zero-filled bytes.
- Why this satisfies the item:
  - Invalid memory now surfaces as a debugger-visible access failure.
- Status: Done

8. GDB register model is not fully Cortex-M specific
- File: `src/gdb.rs` (`read_registers` / `write_registers`)
- Fix:
  - GDB register reads and writes now use `RegisterARM::XPSR`.
  - Debugger register writes reject an xPSR value without the Thumb T-bit set.
  - PC writes are normalized through the existing Thumb-address helper only after xPSR validation.
- Why this satisfies the item:
  - The exposed debugger state is now Cortex-M oriented and no longer accepts invalid non-Thumb execution state.
- Status: Done

9. `extract_svd_registers` misses single-cluster base offset application
- File: `src/util.rs`
- Fix:
  - Single clusters now pass their `address_offset` into register collection just like clustered arrays do.
- Why this satisfies the item:
  - Register extraction now applies the correct base offset for both single and array cluster forms.
- Status: Done

10. Oversized load files are silently truncated
- File: `src/system.rs` (`content[..min(size)]`)
- Fix:
  - Region image loading now errors out if the file size exceeds the configured region size.
- Why this satisfies the item:
  - Misconfigured images fail fast at startup instead of being silently truncated.
- Status: Done

11. Region overlap/attribute validation is absent in config ingestion
- Files: `src/config.rs`, `src/system.rs`
- Fix:
  - Added memory-layout validation for overlapping regions.
  - Added patch-range validation against configured regions and `allow_patches` authorization.
- Why this satisfies the item:
  - Invalid region layouts and invalid patch targets are now rejected before emulation starts.
- Status: Done

### Medium

12. `static mut` global runtime flags in main
- File: `src/main.rs` (`VERBOSE`, `CONSOLE_ONLY`, `LAST_NUM_INSTRUCTIONS`)
- Fix:
  - Replaced `static mut` runtime flags with atomics.
- Why this satisfies the item:
  - The unsafe global state was unnecessary and is now removed from the non-peripheral entry path.
- Status: Done

13. Hardcoded firmware-address trace probes in emulator core
- File: `src/emulator.rs` (many `if pc_aligned == 0x...` probes)
- Fix:
  - Extracted firmware-specific trace state and address probes into `src/emulator_trace.rs`.
  - The core loop now delegates to a dedicated helper instead of embedding the CubeBlack-specific probes directly in the execution hook.
- Why this satisfies the item:
  - Core execution logic is decoupled from firmware-specific trace instrumentation.
- Validation:
  - `cargo check`
  - CubeBlack bounded run to `3000000` instructions completed cleanly after the extraction.
- Status: Done

## Files Reviewed With No Direct RM-Bit/Flag Findings
These files are mostly host I/O/backends, not STM32 RM register/bit logic:
- `src/framebuffers/mod.rs`
- `src/framebuffers/image.rs`
- `src/framebuffers/sdl.rs`
- `src/framebuffers/sdl_engine.rs`
- `src/ext_devices/mod.rs`
- `src/ext_devices/spi_flash.rs`
- `src/ext_devices/ramtron.rs`
- `src/ext_devices/spi_sensors.rs`
- `src/ext_devices/display.rs`
- `src/ext_devices/lcd.rs`
- `src/ext_devices/touchscreen.rs`
- `src/ext_devices/usart_probe.rs`
- `build.rs`

They can still affect end-to-end boot behavior, but they do not directly represent STM32 core register-bit semantics from RM0090.

## Positive Conformance Notes
- Reset vector bootstrap path is correctly aligned with RM boot expectations:
  - SP fetched from vector table start and PC initialized from reset vector entry.
- Vector table base is explicitly configurable (`config.cpu.vector_table`) and propagated to NVIC model.
- Core loop correctly integrates deferred interrupt injection and post-step peripheral stepping, enabling practical bring-up.

## Completion Notes
The non-peripheral findings from this audit are now closed in code. Remaining architectural work toward full CubeBlack boot progress is tracked in `FEATURE_GAP.md` and is primarily peripheral/runtime-model work rather than emulator-core scaffolding.

## Bottom Line
The core emulator scaffolding reviewed here now aligns materially better with RM0090/PM0214 expectations: faults route through SCB/NVIC instead of being skipped or host-aborted, region permissions and config validation are enforced, debugger behavior is Cortex-M aware, and SVD extraction no longer misplaces clustered registers.
