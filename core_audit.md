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
The core runtime is functional for bring-up and debugging, but it is not architecturally faithful in several high-impact areas:
- Fault handling is intentionally bypassed in multiple paths (unmapped memory, abort exceptions), which suppresses BusFault/HardFault behavior required by RM/PM.
- Memory region permissions are flattened to RWX, so flash/ROM and execute protections are not enforced.
- GDB memory/register integration prioritizes convenience over architectural correctness (zero-filling unmapped reads, CPSR/xPSR ambiguity, Thumb-state handling gaps).
- SVD register extraction has a concrete correctness bug for single clusters, which can misplace register offsets for any peripheral using SVD clusters.

## Detailed Findings (ordered by severity)

### Critical

1. Unmapped memory accesses are skipped instead of faulted
- File: `src/emulator.rs` (MEM_UNMAPPED hook around `add_mem_hook(...)` and PC-forced advance)
- Evidence:
  - Forces PC to next instruction on unmapped access.
  - Sets `CONTINUE_EXECUTION` and resumes.
- Why this is wrong:
  - RM vector/fault model classifies memory access and prefetch faults under BusFault/HardFault handling, not silent instruction skip.
  - This suppresses real firmware fault paths, status registers, and handlers.
- Impact:
  - False forward progress; hides real blockers; impossible to validate fault-handling firmware code.
- Status: Wrong

2. Abort exceptions terminate host process instead of emulating fault escalation/handler entry
- File: `src/emulator.rs` (intr hook cases for exceptions `3 | 4`)
- Evidence:
  - On prefetch/data abort it logs and calls `std::process::exit(1)`.
- Why this is wrong:
  - PM0214/RM exception model requires exception entry and state update semantics (CFSR/HFSR/BFAR/MMFAR paths and configurable escalation).
- Impact:
  - Fatal host exit prevents in-target recovery/fault servicing and diverges from MCU behavior.
- Status: Wrong/Incomplete

3. All memory regions are mapped RWX (`Permission::ALL`)
- File: `src/system.rs` (`mem_map(..., Permission::ALL)`)
- Why this is wrong:
  - Real STM32 memory map has region-dependent access semantics (Flash behavior, system memory behavior, SRAM behavior). Flat RWX invalidates execute/write constraints and fault generation conditions.
- Impact:
  - Can hide illegal writes/execs and invalidate boot/runtime behavior checks.
- Status: Wrong

4. Direct binary patching bypasses flash controller semantics entirely
- File: `src/system.rs` (raw patch writes in `load_memory_regions`)
- Why this is wrong:
  - RM flash programming requires controller flow/flags/latency, not arbitrary write-through to flash-mapped memory.
- Impact:
  - Any firmware that depends on flash controller behavior/flags can appear to work when it should fail.
- Status: Incomplete (acceptable for controlled patching, but architecturally divergent)

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
- Why this is incomplete:
  - Cortex-M requires T-state correctness; invalid state transitions should fault (UsageFault/INVSTATE path), not be auto-healed.
- Impact:
  - Masks invalid control-flow bugs and may prevent realistic fault discovery.
- Status: Incomplete

7. GDB memory read path masks unmapped access with zero-fill
- File: `src/gdb.rs` (`read_addrs`: if mem read fails -> fill zeros)
- Why this is wrong:
  - Debugger reads from invalid addresses should return an access error, not fabricated data.
- Impact:
  - Misleads debugger sessions and hides map/fault issues.
- Status: Wrong

8. GDB register model is not fully Cortex-M specific
- File: `src/gdb.rs` (`read_registers` / `write_registers`)
- Evidence:
  - Uses `RegisterARM::CPSR` in a Cortex-M context and direct PC writes without full architectural guardrails.
- Why this is incomplete:
  - Cortex-M debug register semantics are xPSR-centered; state bits and exception context need stricter handling.
- Impact:
  - Potential debugger-visible divergence and invalid state acceptance.
- Status: Incomplete

9. `extract_svd_registers` misses single-cluster base offset application
- File: `src/util.rs`
- Evidence:
  - For `MaybeArray::Single(c)`, code calls `collect_registers(c.all_registers(), None)`.
  - Cluster offset logic exists but is not applied for this path.
- Why this is wrong:
  - Registers within a single cluster should inherit cluster base offset.
- Impact:
  - Register address decoding can be wrong for clustered SVD structures.
- Status: Wrong

10. Oversized load files are silently truncated
- File: `src/system.rs` (`content[..min(size)]`)
- Why this is incomplete:
  - Silent truncation hides config/image issues and can cause non-obvious boot divergence.
- Impact:
  - Hard-to-diagnose runtime behavior.
- Status: Incomplete

11. Region overlap/attribute validation is absent in config ingestion
- Files: `src/config.rs`, `src/system.rs`
- Why this is incomplete:
  - No explicit validation of overlaps, alignment, intended region type, or conflicting load targets.
- Impact:
  - Invalid board configs can succeed in setup and fail later with misleading symptoms.
- Status: Incomplete

### Medium

12. `static mut` global runtime flags in main
- File: `src/main.rs` (`VERBOSE`, `CONSOLE_ONLY`, `LAST_NUM_INSTRUCTIONS`)
- Why this is risky:
  - Unsafe globals are not needed here and make behavior less robust if architecture evolves (threads, re-entrancy, tests).
- Impact:
  - Maintainability/safety risk; lower direct hardware-fidelity impact.
- Status: Incomplete

13. Hardcoded firmware-address trace probes in emulator core
- File: `src/emulator.rs` (many `if pc_aligned == 0x...` probes)
- Why this is incomplete:
  - Debug instrumentation is useful, but current placement is tightly coupled to one firmware image and can distort maintainability of core execution logic.
- Impact:
  - High technical debt in core loop; low direct RM compliance impact.
- Status: Incomplete

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

## Recommended Next Fix Order
1. Replace MEM_UNMAPPED skip logic with architectural fault injection path (BusFault/MemManage + escalation).
2. Remove host `exit(1)` on aborts; route through in-target exception model and fault status register updates.
3. Introduce memory-region permissions in config and enforce in `mem_map` (at minimum: flash RO/X, SRAM RW/X, peripheral RW/!X).
4. Fix `extract_svd_registers` single-cluster offset bug.
5. Tighten GDB behavior: propagate unmapped read errors; align register model with Cortex-M xPSR semantics.
6. Add strict config validation (overlap, alignment, truncation warnings-as-errors option).

## Bottom Line
Core runtime is effective for iterative boot debugging, but it currently trades away several architectural guarantees required by RM0090/PM0214. The most important correctness gaps are in fault handling and memory protection semantics, followed by debugger and SVD register extraction fidelity.
