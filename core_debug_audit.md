# CoreDebug DWT Audit (ARMv7-M CoreDebug/DWT Source-of-Truth)

Scope
- Peripheral audited: src/peripherals/core_debug.rs
- Reference source: ARMv7-M CoreDebug + DWT architecture as used on STM32F4.

## Executive result
Cycle counting support is good for firmware progress, but most debug/watchpoint behaviors are storage-only and non-functional.

Wrong
1. Many writable debug fields are accepted without access restrictions.
2. Counter increments are tied only to NUM_INSTRUCTIONS, not architectural event classes.

Missing
1. Comparator match logic and action from DWT FUNCTION/COMP/MASK registers.
2. Exception/sleep/LSU/folded counter semantics.
3. Debug monitor interactions and halt/debug side effects.

Incomplete
1. DEMCR gating exists but full trace/debug behavior is absent.
2. PC sampling register behavior is static.

## High-priority correction targets
1. Implement comparator match path with at least watchpoint signaling hooks.
2. Model non-CYCCNT counters according to relevant events.
3. Tighten access masks for reserved/RO bits.

## Key file anchors
- src/peripherals/core_debug.rs:42
- src/peripherals/core_debug.rs:102
- src/peripherals/core_debug.rs:187
