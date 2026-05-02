# SPI Audit (RM0090 SPI/I2S Source-of-Truth)

Scope
- Peripheral audited: src/peripherals/spi.rs
- Reference source: STM32F4 SPI status/control/DMA/interrupt behavior.

## Executive result
SPI model is functional for ext-device exchange and DMA-driven boot flows, but status flags and bus timing semantics remain simplified.

Wrong
1. TXE is effectively always asserted in build_sr path except during brief local updates.
2. OVR/MODF clear semantics are simplified compared with RM-defined read/write sequences.

Missing
1. Full CRC/I2S mode behavior.
2. NSS hardware management nuances and full master/slave timing interaction.
3. Detailed BSY/RXNE/TXE timing with frame boundaries.

Incomplete
1. CPOL/CPHA and frame format effects are partial.
2. DMA request pacing and thresholds are simplified.
3. Interrupt source behavior is practical but not exhaustive.

## High-priority correction targets
1. Improve SR flag lifecycle (TXE/RXNE/BSY/OVR/MODF) to RM sequences.
2. Expand DMA handshake timing coupling to transfer events.
3. Add stricter mode constraints and I2S separation.

## Key file anchors
- src/peripherals/spi.rs:36
- src/peripherals/spi.rs:90
- src/peripherals/spi.rs:196
