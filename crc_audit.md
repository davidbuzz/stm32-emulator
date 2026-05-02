# CRC Audit (RM0090 CRC Source-of-Truth)

Scope
- Peripheral audited: src/peripherals/crc.rs
- Reference source: STM32F4 fixed-function CRC unit behavior.

## Executive result
CRC model is compact and mostly usable for firmware expecting basic DR/IDR/CR behavior, but peripheral edge semantics are simplified.

Wrong
1. DR write path always uses software loop without timing or bus width side effects.

Missing
1. Byte/halfword write behavior distinctions to DR in mixed-width access cases.
2. Explicit reset/read timing behavior nuances used by some codebases.

Incomplete
1. Strict access restrictions and reserved bit handling are minimal.
2. No validation for unusual access patterns.

## High-priority correction targets
1. Add width-aware DR write handling (8/16/32-bit access forms).
2. Tighten CR/IDR masking semantics to RM-defined writable bits only.

## Key file anchors
- src/peripherals/crc.rs:31
- src/peripherals/crc.rs:54
