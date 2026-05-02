# I2C Audit (RM0090 I2C Source-of-Truth)

Scope
- Peripheral audited: src/peripherals/i2c.rs
- Reference source: STM32F4 I2C master/slave event/error sequencing.

## Executive result
The model has a practical EV5/EV6/EV8-style state machine and board-hook slave map, but full RM timing/error/DMA behavior remains incomplete.

Wrong
1. Several status transitions are synthetic and not fully tied to exact bus conditions.
2. SR1/SR2 clear ordering is approximated and may diverge in corner cases.

Missing
1. Multi-master arbitration details and clock stretching behavior.
2. ACK/NACK and STOP timing subtleties for reads/writes.
3. Full error-path generation (BERR/ARLO/OVR/TIMEOUT) from real conditions.

Incomplete
1. DMA request pacing and byte-level handshakes are simplified.
2. Addressing modes and repeated-start edge cases are partial.
3. Timing derived from CCR/TRISE values is not enforced.

## High-priority correction targets
1. Tighten SR1/SR2 flag lifecycle to RM event ordering.
2. Expand error-condition generation and IRQ routing.
3. Improve DMA request timing coupling to TXE/RXNE/BTF.

## Key file anchors
- src/peripherals/i2c.rs:47
- src/peripherals/i2c.rs:124
- src/peripherals/i2c.rs:221
- src/peripherals/i2c.rs:355
