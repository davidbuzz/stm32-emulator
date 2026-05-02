# FSMC Audit (RM0090 FSMC Source-of-Truth)

Scope
- Peripheral audited: src/peripherals/fsmc.rs
- Reference source: STM32F4 FSMC bank/timing register behavior.

## Executive result
FSMC is bank-oriented and useful for external device hookup, but register semantics and timing effects are mostly pass-through/storage.

Wrong
1. Many register fields are accepted without bit-level legality or side effects.
2. ECC and NAND/PCCARD-specific paths are not behaviorally modeled.

Missing
1. FSMC timing impact on external read/write cycles.
2. ECC calculation and status update paths.
3. Bus turnaround/wait-state behavior.

Incomplete
1. BCR/BTR/BWTR/PCR timing fields are persisted but not enforced.
2. Status behavior (SR/ECCR) is mostly static.

## High-priority correction targets
1. Implement minimum timing-based wait behavior for bank accesses.
2. Add ECCR/ECC flow for NAND-like modes where relevant.
3. Enforce key field masks and mode-specific constraints.

## Key file anchors
- src/peripherals/fsmc.rs:22
- src/peripherals/fsmc.rs:86
- src/peripherals/fsmc.rs:169
