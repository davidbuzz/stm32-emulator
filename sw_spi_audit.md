# Software SPI Helper Audit (Emulator-Specific)

Scope
- Peripheral-like helper audited: src/peripherals/sw_spi.rs
- Note: this is emulator-only glue, not an STM32F4 memory-mapped peripheral.

## Executive result
Software SPI helper is fit for board wiring and ext-device transfer, but intentionally omits electrical/timing fidelity.

Wrong
1. No strict phase/polarity timing model beyond basic edge handling.

Missing
1. Configurable CPOL/CPHA behavior (CPOL marked TODO).
2. Bus contention/multi-device arbitration modeling.

Incomplete
1. Chip-select and clock timing tolerances are simplified.
2. Error signaling paths do not exist (by design).

## High-priority correction targets
1. Add CPOL/CPHA options and consistent edge sampling rules.
2. Add optional timing constraints for stricter protocol testing.

## Key file anchors
- src/peripherals/sw_spi.rs:13
- src/peripherals/sw_spi.rs:74
