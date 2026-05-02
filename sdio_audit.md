# SDIO Audit (RM0090 SDIO Source-of-Truth)

Scope
- Peripheral audited: src/peripherals/sdio.rs
- Reference source: STM32F4 SDIO command/data path and IRQ/status behavior.

## Executive result
SDIO has a useful synthetic command/response and DMA-fed data path for boot progress, but many protocol and status semantics are intentionally simplified.

Wrong
1. Command handling is command-number scripted, not card-state/protocol complete.
2. Several STA/ICR/MASK interactions are simplified versus RM details.

Missing
1. Complete SD command set/state machine and error responses.
2. Accurate FIFO/FIFOCNT behavior and data-path error classes.
3. Multi-block transfer stop/continuation semantics.

Incomplete
1. DMA integration is pragmatic but not fully aligned to all stream/mode combinations.
2. Timeout/CRC/stbit error generation is simplified.

## High-priority correction targets
1. Improve STA bit lifecycle and ICR clear behavior fidelity.
2. Expand command/state handling beyond fixed known-command scripts.
3. Tighten data transfer/error timing for DMA and non-DMA paths.

## Key file anchors
- src/peripherals/sdio.rs:84
- src/peripherals/sdio.rs:189
- src/peripherals/sdio.rs:362
