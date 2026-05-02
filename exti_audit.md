# EXTI Audit (RM0090 EXTI Source-of-Truth)

Scope
- Peripheral audited: src/peripherals/exti.rs
- Reference source: STM32F4 EXTI line routing and pending/trigger behavior.

## Executive result
EXTI model covers core pending/mask/trigger flow and IRQ fanout, but omits critical SYSCFG routing and several event-path details.

Wrong
1. GPIO transition path ignores SYSCFG EXTICR port-to-line muxing.
2. SWIER handling is edge-based in model, while hardware semantics are bit-set driven.

Missing
1. Accurate EXTI line source routing via SYSCFG EXTICR.
2. Event path behavior (EMR) integration beyond storage.
3. Full line-specific behavior for lines 16-22 nuances.

Incomplete
1. Pending/clear behavior is simplified for combined IRQ lines.
2. Wakeup/event-only behavior is not modeled.

## High-priority correction targets
1. Implement SYSCFG EXTICR-driven line source selection.
2. Separate interrupt vs event behavior with clearer EMR handling.
3. Tighten SWIER semantics and pending-bit lifecycle per line.

## Key file anchors
- src/peripherals/exti.rs:31
- src/peripherals/exti.rs:78
- src/peripherals/exti.rs:124
