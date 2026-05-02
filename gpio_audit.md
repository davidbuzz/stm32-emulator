# GPIO Audit (RM0090 GPIO Source-of-Truth)

Scope
- Peripheral audited: src/peripherals/gpio.rs
- Reference source: STM32F4 GPIO port register and alternate-function behavior.

## Executive result
GPIO model is strong for callback-driven board wiring and EXTI triggering, but AF/system integration and locking semantics are simplified.

Wrong
1. IDR reads derive from callback system only; true input latch behavior is not modeled.
2. LCKR lock sequence semantics are not implemented.

Missing
1. Alternate-function effects on peripheral signal routing.
2. OSPEED/OTYPE/PUPD electrical behavior impact.
3. Proper per-bit write constraints for some register fields.

Incomplete
1. BSRR/ODR behavior is adequate for wiring but not full hardware timing.
2. EXTI interactions bypass SYSCFG line mux constraints.

## High-priority correction targets
1. Implement LCKR lock-key sequence and locked-write behavior.
2. Route AF state to peripheral mux hooks where needed.
3. Improve IDR vs ODR separation and input-latch realism.

## Key file anchors
- src/peripherals/gpio.rs:86
- src/peripherals/gpio.rs:124
- src/peripherals/gpio.rs:213
