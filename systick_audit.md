# SysTick Compliance Audit vs ARMv7-M / STM32 Integration

## Scope and Source of Truth
- Code audited: src/peripherals/systick.rs
- Primary spec: ARMv7-M SysTick architecture as used on Cortex-M4 in STM32F4 (CTRL, LOAD, VAL, CALIB and exception behavior).
- Integration touchpoint: NVIC/SCB handling in src/peripherals/nvic.rs and src/peripherals/scb.rs.

## What Is Already Implemented
- Basic CTRL and LOAD storage exists.
- NVIC period hook exists via nvic.systick_period.
- SysTick pending generation exists in NVIC based on instruction delta.

## Incorrect Behavior vs Architecture

### 1) CTRL enable logic is incorrect
- has_int_enabled requires both ENABLE and TICKINT bits to be set.
- Hardware counting should run when ENABLE=1 regardless of TICKINT; TICKINT only controls exception generation.
- Impact: counter effectively stops unless interrupts are enabled.

### 2) COUNTFLAG behavior is synthetic/toggled on read
- CTRL read flips bit 16 using val_toggle.
- Hardware COUNTFLAG sets when counter transitions 1->0 and clears on CTRL read.
- Impact: polling logic gets fake events unrelated to elapsed cycles.

### 3) VAL register is not an actual downcounter
- VAL read alternates between 0 and reload/2.
- Hardware VAL should reflect current decrementing value.

### 4) VAL write semantics are missing
- Writing VAL should clear current counter to 0 (and affects COUNTFLAG timing); current code ignores offset 0x0008 writes.

### 5) LOAD bit-width constraints are missing
- LOAD is 24-bit (RELOAD field), with max 0x00FF_FFFF.
- Current code stores full 32-bit without masking.

### 6) CALIB register is missing
- SysTick CALIB at offset 0x000C is not implemented.

### 7) Clock source behavior absent
- CTRL.CLKSOURCE (processor clock vs external reference) is not modeled.
- Current NVIC scheduling assumes one fixed instruction-based cadence.

### 8) ENABLE without TICKINT should still count and set COUNTFLAG
- Current architecture coupling to nvic.systick_period prevents that behavior.

## Missing or Incomplete Integration Aspects

### A) Precise SysTick exception timing
- Exception should be pending when counter wraps and TICKINT=1; COUNTFLAG should still latch on wraps independent of TICKINT.

### B) Interaction with SCB ICSR PENDSTSET/PENDSTCLR and active state
- Some paths exist in SCB, but SysTick model itself does not maintain full internal event state needed for fully accurate behavior.

### C) Reset values and side-effect fidelity
- Register reset defaults and side-effects are only partially modeled.

## High-Confidence Fix Priorities
1. Split counting-enable from interrupt-enable (ENABLE vs TICKINT).
2. Implement real 24-bit reload/current counter model with decrement on emulated clock.
3. Implement architectural COUNTFLAG set/clear behavior.
4. Add VAL write-to-clear semantics and CALIB register exposure.
5. Mask LOAD to 24 bits and validate reload=0 corner behavior per architecture.

## Code Anchors Reviewed
- src/peripherals/systick.rs:31
- src/peripherals/systick.rs:35
- src/peripherals/systick.rs:55
- src/peripherals/systick.rs:58
- src/peripherals/systick.rs:61
- src/peripherals/systick.rs:63
- src/peripherals/systick.rs:74
- src/peripherals/systick.rs:80
