# SCB Audit (ARMv7-M SCB Source-of-Truth)

Scope
- Peripheral audited: src/peripherals/scb.rs
- Reference source: ARMv7-M System Control Block behavior on Cortex-M4.

## Executive result
SCB model covers practical VTOR/ICSR control and status register storage, but deep fault and control side effects are incomplete.

Wrong
1. ICSR active/pending field composition is simplified.
2. Some writable/read-only and key-protected behaviors are not fully enforced.

Missing
1. Full SHCSR/CFSR/HFSR fault-generation coupling with execution faults.
2. AIRCR SYSRESETREQ and priority-grouping side effects.
3. Complete system-handler priority integration with NVIC arbitration.

Incomplete
1. Fault address/status lifecycle is mostly storage-oriented.
2. CPACR effects on floating-point usage are not enforced.

## High-priority correction targets
1. Tie SCB fault registers to real exception escalation paths.
2. Implement AIRCR reset/grouping side effects.
3. Improve ICSR pending/active semantics parity with ARM behavior.

## Key file anchors
- src/peripherals/scb.rs:44
- src/peripherals/scb.rs:83
