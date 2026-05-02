# TIM (TIM1-TIM14) Compliance Audit vs STM32F4 RM0090

## Scope and Source of Truth
- Code audited: src/peripherals/tim.rs
- Primary spec: RM0090 Chapters 17, 18, 19, 20 (TIM1/8, TIM2-5, TIM9-14, TIM6/7).
- Focus areas: CR1/CR2/SMCR/DIER/SR/EGR/CNT/PSC/ARR/RCR/CCR/BDTR semantics, update/compare IRQ and DMA-request behavior.

## What Is Already Implemented
- Basic register set for common timer operation exists.
- Counter stepping with prescaler approximation exists.
- Update and compare flag paths (UIF, CCxIF, CCxOF) are partially modeled.
- EGR software event generation for UG/CCxG/TG/BG is partially present.
- RCR/BDTR are exposed for TIM1/TIM8.

## Incorrect Behavior vs RM0090

### 1) Timer widths are not per-instance accurate
- TIM2/TIM5 are 32-bit, many others are 16-bit (CNT/ARR/CCR/PSC width constraints).
- Current model stores most counters/registers as unbounded u32 behavior.
- Impact: overflow timing and compare behavior diverge for 16-bit timers.

### 2) Center-aligned counting is incorrect
- Code states CMS 1/2 then effectively always counts upward.
- RM requires up/down counting with direction changes at limits and specific compare/update timing.
- Impact: incorrect PWM/compare/update event cadence.

### 3) Update-event policy bits are not implemented
- CR1 bits UDIS and URS are not enforced.
- RM requires UG/overflow/update-source filtering through URS/UDIS.
- Impact: UIF/UIE and update-DMA behavior can be wrong.

### 4) ARR preload behavior not implemented despite ARPE tracking
- ARPE is latched into a boolean but no shadow register/update-event transfer path exists.
- RM requires ARR preload transfer on update event when ARPE=1.

### 5) PSC preload/update semantics are simplified
- PSC writes should transfer through update-event behavior.
- Current handling applies PSC immediately as a direct divider.

### 6) Advanced-timer RCR logic is off by one
- Code compares rcr_count >= rcr after increment.
- RM update event is every RCR+1 counter overflows.
- Impact: update periodicity mismatch.

### 7) SR clearing semantics are over-simplified
- Code uses sr &= value for writes.
- Many timer status bits are cleared via specific write patterns and have nuanced behavior per bit class.
- This generic mask can accidentally preserve/clear wrong subsets across mixed flag writes.

### 8) TIM5 CEN sticky workaround is non-hardware
- Code makes TIM5 CEN permanently enabled once set.
- RM does not define this behavior.
- Impact: firmware that legitimately stops TIM5 cannot be represented.

### 9) BDTR break behavior is speculative
- BIF generation on MOE 1->0 transition is modeled as a generic break source.
- RM break sourcing is tied to break inputs/configuration and advanced-timer logic; MOE transitions are not a blanket break trigger.

## Missing or Incomplete TIM Features

### A) Slave mode controller behavior is not implemented
- SMCR SMS/TS fields are stored but no trigger-mode/gated/reset/external-clock behavior is executed.

### B) OC/IC mode logic is missing
- CCMR/CCER fields are mostly just stored; output compare modes, input capture filters/prescalers, and polarity/enable effects are absent.

### C) One-pulse mode, repetition/advanced dead-time detail
- OPM, dead-time insertion, OSSR/OSSI, lock levels, break filters and automatic output are incomplete.

### D) Trigger/commutation interrupts and DMA requests
- TDE, COMDE, CCxDE, UDE request routing to DMA engine is only logged (no actual DMA request dispatch).

### E) Master/slave synchronization network
- Internal trigger routing (ITR links), TRGO generation, and chained-timer behavior are absent.

### F) UIF remap and some family-specific subtleties
- No modeling of specific status/control subtleties needed by tighter driver expectations.

## High-Confidence Fix Priorities
1. Implement per-timer width masks and overflow behavior (16-bit vs 32-bit).
2. Correct CMS up/down state machine and update/compare timing in center-aligned modes.
3. Implement URS/UDIS/ARPE/PSC preload semantics around update events.
4. Replace RCR update logic with exact RCR+1 overflow cadence.
5. Remove TIM5 forced-CEN quirk or gate it behind explicit compatibility option.
6. Wire DIER DMA request bits to actual DMA request signaling paths.

## Code Anchors Reviewed
- src/peripherals/tim.rs:149
- src/peripherals/tim.rs:152
- src/peripherals/tim.rs:177
- src/peripherals/tim.rs:234
- src/peripherals/tim.rs:281
- src/peripherals/tim.rs:289
- src/peripherals/tim.rs:346
- src/peripherals/tim.rs:373
- src/peripherals/tim.rs:428
- src/peripherals/tim.rs:432
