# PWR Audit (RM0090 PWR Source-of-Truth)

Scope
- Peripheral audited: src/peripherals/pwr.rs
- Reference source: STM32F4 power control and status registers.

## Executive result
PWR model is useful for early polling loops (ready bits), but many bit semantics and low-power state behaviors are simplified.

Wrong
1. Ready bits are effectively immediate and synthetic.
2. Some CR/CSR writable/read-only distinctions are simplified.

Missing
1. Standby/stop mode entry and wake semantics.
2. Backup-domain and regulator transition timing nuances.
3. Full wakeup pin/status flag behavior.

Incomplete
1. Over-drive sequence constraints are simplified.
2. Bypass/regulator interaction side effects are partial.

## High-priority correction targets
1. Enforce more accurate CR/CSR bit write masks and transitions.
2. Add low-power mode state transitions at least for core polling paths.
3. Improve wake/backup domain behavior.

## Key file anchors
- src/peripherals/pwr.rs:29
- src/peripherals/pwr.rs:75
