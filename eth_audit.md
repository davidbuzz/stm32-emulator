# Ethernet MAC Audit (RM0090 ETH Source-of-Truth)

Scope
- Peripheral audited: src/peripherals/eth.rs
- Reference source: STM32F4 Ethernet MAC + DMA controller chapters.

## Executive result
Ethernet is presently a pure register-stub; no packet path or PHY transaction logic is implemented.

Wrong
1. Read/write-only and W1C semantics for status/mask registers are not enforced.

Missing
1. MAC DMA descriptor engine and buffer ownership behavior.
2. MII/MDIO transaction sequencing through MACMIIAR/MACMIIDR.
3. RX/TX packet flow, frame filtering, and interrupt signaling.
4. Integration with ETH IRQ and DMA status bits.

Incomplete
1. Power management and wake behavior is storage-only.
2. Debug/status register fields are not dynamically updated.

## High-priority correction targets
1. Add minimal MDIO state machine for PHY probing.
2. Implement stub RX/TX descriptor completion + IRQ for bring-up paths.
3. Enforce key W1C/RO semantics on MACSR-like registers.

## Key file anchors
- src/peripherals/eth.rs:17
- src/peripherals/eth.rs:57
