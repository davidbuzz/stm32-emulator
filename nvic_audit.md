# NVIC Audit (ARMv7-M + STM32F4 Integration Source-of-Truth)

Scope
- Peripheral audited: src/peripherals/nvic.rs
- Reference source:
  - ARMv7-M architecture (NVIC/exception model)
  - STM32F4 integration expectations for external IRQ enable/pending/priority behavior.
- Classification:
  - wrong: architecture mismatch
  - missing: unimplemented required feature
  - incomplete: partial behavior only

## Executive result
The NVIC model supports basic enable/pending/priority and exception entry/return, and is useful for boot progression. It is not fully architecture-accurate for masking/preemption/system-exception policy and has at least one unsafe edge condition previously observable in debug runs.

## Register coverage

Implemented (partial)
- ISER/ICER, ISPR/ICPR, IPR read/write ranges.
- External IRQ pending/enable/priority bookkeeping.
- Basic vector fetch and exception stack push/pop paths.
- SysTick pending injection via periodic counter hook.

Wrong
1. Potential shift overflow hazard around bit math (edge condition)
- Pending-bit calculations depend on IRQ_OFFSET + irq and bit shifts.
- While guarded by asserts in main helpers, architecture-negative/system exceptions and conversion paths are fragile and have shown panic behavior in some execution modes.

2. System exception handling is overly simplified
- System exceptions are chosen by lowest pending bit from a combined low-bit mask, without full ARM priority/enable semantics.
- Real behavior depends on configurable priorities, handler enables, and fault status/control interactions.

3. Interrupt dispatch blocked whenever IPSR != 0
- Current take_pending_interrupt path generally blocks dispatch if already in handler mode.
- ARM supports nested preemption when priority rules permit; this is a significant behavioral simplification.

4. Priority model lacks full ARM priority field handling
- Raw byte values are used; no consistent priority grouping/subpriority behavior.

Missing
1. Active bit registers and full architectural status exposure
- IABR behavior is not surfaced in implementation despite comments listing it.

2. Full fault/system handler control integration
- SHPR/SHCSR/ICSR/fault escalation fields are present in struct but mostly not integrated into real dispatch decisions.

3. Precise PRIMASK/BASEPRI/FAULTMASK semantics
- BASEPRI is considered for external IRQs, but broader architecture semantics are incomplete.

4. Tail-chaining/late-arrival fidelity
- Exception entry/return model does not provide full tail-chain behavior expected by ARMv7-M.

Incomplete
1. Exception return paths
- Robust for common path, but still simplified relative to complete EXC_RETURN matrix and FP context conditions.

2. Nested exception tracking
- active_exceptions and saved stack state exist, but preemption policy is intentionally constrained.

3. SysTick coupling
- SysTick event insertion is instruction-count based and not integrated with full architectural timer/execution conditions.

## High-priority correction targets
1. Harden pending-bit arithmetic for all negative/system IRQ paths and eliminate shift-panic risk.
2. Implement proper nested-preemption eligibility in handler mode.
3. Integrate system exception priority/enable semantics with SCB state.
4. Add IABR and richer active/pending visibility.
5. Improve EXC_RETURN + tail-chaining fidelity.

## Key file anchors
- src/peripherals/nvic.rs:105
- src/peripherals/nvic.rs:120
- src/peripherals/nvic.rs:216
- src/peripherals/nvic.rs:246
- src/peripherals/nvic.rs:334
- src/peripherals/nvic.rs:527
- src/peripherals/nvic.rs:560
