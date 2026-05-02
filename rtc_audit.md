# RTC Audit (RM0090 RTC Source-of-Truth)

Scope
- Peripheral audited: src/peripherals/rtc.rs
- Reference source: STM32F4 RTC calendar/alarm/wakeup/protection behavior.

## Executive result
RTC is currently a writable register bank with basic write-protection gate; calendar clocking and alarm/wakeup semantics are largely absent.

Wrong
1. WPR unlock flow is simplified and not fully RM-sequenced.
2. ISR reset-like defaults are approximated.

Missing
1. Calendar time/date progression and shadow register synchronization.
2. Alarm A/B, wakeup timer, timestamp, tamper behavior.
3. Alarm/wakeup interrupt generation path.

Incomplete
1. INIT/INITF/RSF sequencing is minimal.
2. Backup register + protection semantics are simplified.

## High-priority correction targets
1. Implement minimal calendar tick + RSF/INIT sync behavior.
2. Add alarm/wakeup status and interrupt paths.
3. Tighten WPR/INIT write gating.

## Key file anchors
- src/peripherals/rtc.rs:13
- src/peripherals/rtc.rs:40
