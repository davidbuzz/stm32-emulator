# ADC Audit (RM0090 ADC Chapters Source-of-Truth)

Scope
- Peripheral audited: src/peripherals/adc.rs
- Reference source: STM32F4 RM ADC1/2/3 regular + injected conversion behavior.

## Executive result
The model is useful for boot/runtime progress and DMA-friendly reads, but conversion timing, sequencing, and ADC feature coverage are highly simplified.

Wrong
1. SR read path forces EOC set via `self.sr | ADC_SR_EOC`, which can hide real EOC clear timing.
2. OVR/EOC behavior is partially synthetic and not fully coupled to exact DR read/write and conversion overlap rules.

Missing
1. Injected conversion path (JSQR/JDR trigger semantics and JEOC handling).
2. Scan sequence stepping across SQR ranks with realistic per-rank progression.
3. Analog watchdog threshold logic and AWD flags/interrupts.
4. External trigger edge/source behavior and timer-trigger coupling.

Incomplete
1. Sampling-time impact and conversion latency are not modeled.
2. Continuous/discontinuous mode semantics are partial.
3. Multi-ADC common behavior is absent.

## High-priority correction targets
1. Make EOC/OVR transitions RM-accurate (no forced EOC on SR reads).
2. Implement regular sequence rank stepping and EOS-like completion behavior.
3. Add injected conversion + JEOC path used by advanced firmware.

## Key file anchors
- src/peripherals/adc.rs:82
- src/peripherals/adc.rs:153
- src/peripherals/adc.rs:185
