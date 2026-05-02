# IWDG Audit (RM0090 IWDG Source-of-Truth)

Scope
- Peripheral audited: src/peripherals/iwdg.rs
- Reference source: STM32F4 independent watchdog behavior.

## Executive result
IWDG supports key unlock/reload/start register interactions, but it does not implement watchdog countdown/reset semantics.

Wrong
1. No timebase-based decrement/reset behavior after start key.

Missing
1. Prescaler/reload-driven timeout counting.
2. Update-status bits in SR during PR/RLR updates.
3. System reset trigger on underflow.

Incomplete
1. Register-write timing and synchronization delays are absent.
2. Start/refresh side effects are mostly stubbed.

## High-priority correction targets
1. Implement countdown based on PR/RLR and emulation clock.
2. Model SR PVU/RVU update-busy flags.
3. Trigger reset path on timeout expiration.

## Key file anchors
- src/peripherals/iwdg.rs:33
- src/peripherals/iwdg.rs:45
