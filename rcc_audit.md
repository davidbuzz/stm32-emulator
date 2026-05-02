# RCC Audit (RM0090 RCC Source-of-Truth)

Scope
- Peripheral audited: src/peripherals/rcc.rs
- Reference source: STM32F4 reset/clock control behavior.

## Executive result
RCC has good startup-oriented ready-bit mirroring and register persistence, but clock-tree timing and many side effects remain permissive.

Wrong
1. Oscillator/PLL ready timing is immediate rather than stateful with delays.
2. SW->SWS mapping is forced and may skip invalid/intermediate states.

Missing
1. Derived bus/peripheral clock propagation to dependent peripheral timings.
2. Full clock-source constraints and failure modes.
3. Comprehensive reset side effects across all peripheral domains.

Incomplete
1. PLL validation exists but does not enforce full operational behavior.
2. Backup/reset interactions are partial.

## High-priority correction targets
1. Add stateful oscillator/PLL startup delays and failure behavior.
2. Propagate effective clock rates to time-sensitive peripherals.
3. Tighten reset/enable side effects across buses.

## Key file anchors
- src/peripherals/rcc.rs:72
- src/peripherals/rcc.rs:118
- src/peripherals/rcc.rs:181
