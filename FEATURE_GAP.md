# Feature Gap

This file tracks emulator features that are known to be incomplete or not yet implemented.

## Current deliverable

- End goal: get the CubeBlack target to boot and run ArduPilot firmware under the emulator with realistic emulator-side behavior, rather than relying on firmware-side startup bypasses.
- This document is the primary backlog and technical handover for work that remains between the current emulator state and that deliverable.
- Prefer items that unblock demonstrable CubeBlack boot progress first, then follow-on correctness and hardware coverage work.

## Recently Fixed Items

| Feature | Status | Notes/Technical Issues |
| --- | --- | --- |
| CubeBlack target bring-up assets and board configuration | Implemented | Added in commit `f46e695`. This introduced the initial CubeBlack board files, firmware artifacts, `cubeblack/config.yaml`, `run.sh`, and STM32F427 SVD so the emulator could target CubeBlack at all. |
| CubeBlack bootloader artifact rebuilt for oscillator configuration change | Implemented | Added in commit `5797f0f`. The commit message says the upstream hardware definition changed `OSCILLATOR_HZ` to `0`; in this repo the concrete result is an updated `cubeblack/CubeBlack_bl.hex` artifact used by the emulator. |
| Startup peripheral-ready wait loops bypassed in firmware image | Implemented | Added in commit `377a746` via `drop-waiting-for-peripherals-on-startup.patch` plus rebuilt firmware artifacts. This comments out waits on RCC, PWR, FLASH, PLL, and oscillator ready bits so startup can proceed despite missing emulator-side peripheral readiness semantics. |

## Current Session Implemented Items

| Feature | Status | Notes/Technical Issues |
| --- | --- | --- |
| CubeBlack memory map updated for CCM RAM and system memory | Implemented | `cubeblack/config.yaml` now maps `RAM-CCM` at `0x10000000` and `SYSMEM` at `0x1FFF0000`, removing earlier unmapped accesses in valid STM32F427 regions. |
| STM32 unique ID bytes patched into system memory | Implemented | `cubeblack/config.yaml` now injects a 12-byte UID payload at `0x1FFF7A10`, covering firmware reads from the STM32 device identifier area. |
| DMA global register decode corrected | Implemented | `src/peripherals/dma.rs` now treats `0x00..0x0f` as `LISR/HISR/LIFCR/HIFCR` and starts stream register decode at `0x10 + n*0x18`, fixing the previous misdecode. |
| DMA transfer-complete flag and clear semantics added | Implemented | `src/peripherals/dma.rs` now tracks `lisr`/`hisr`, sets `TCIF` on completed transfers, and clears flags via `LIFCR`/`HIFCR`, matching the STM32F4 polling model more closely. |
| NVIC stack-pointer restore and IPSR numbering fixed | Implemented | `src/peripherals/nvic.rs` now writes back the selected stack pointer register (`MSP` or `PSP`) instead of always `SP`, and writes architectural exception numbers into `IPSR` using `16 + irq`. |
| Minimal general/basic TIM timebase added | Partially Implemented | `src/peripherals/tim.rs` provides a small stub for `TIM5`, `TIM6`, and `TIM7` with `CR1`, `DIER`, `SR`, `CNT`, `PSC`, `ARR`, and `CCR1`. This is enough for some polling and timeout loops, but it is not a full advanced TIM implementation. |
| Core SCB state and dynamic VTOR handling added | Implemented | `src/peripherals/scb.rs` now exposes basic Cortex-M4 `SCB` state including `CPUID`, `ICSR`, `VTOR`, `AIRCR`, `SHPRx`, and fault-status registers, while `src/peripherals/nvic.rs` now dispatches exceptions through the active `VTOR` instead of a fixed config-time vector table address. |
| Free-running TIM stepping and TIM5 compare wakeup fixed | Implemented | Timers now advance from the emulator main loop instead of only on MMIO access, and `src/peripherals/tim.rs` preserves prescaler remainder across steps. This allows the firmware-configured `TIM5` compare interrupt to fire after the initial idle-loop park, which was previously impossible because the counter froze once the firmware stopped polling it. |
| Busy-loop diagnostic capture added | Implemented | `src/emulator.rs` now logs `PC`, `SP`, `LR`, `R0-R7`, context words, and the restore frame when `--busy-loop-stop` triggers, which made the current scheduler hand-off failure traceable. |
| USART probe line flushing improved | Implemented | `src/ext_devices/usart_probe.rs` now flushes on both CR and LF and also flushes long buffered output, improving visibility of firmware text output during boot debugging. |
| CubeBlack run script hardened for local environments | Implemented | `cubeblack/run.sh` now sources Cargo environment when present and falls back to the built release binary, reducing local setup friction. |

## Current Handover Blockers

| Feature | Status | Notes/Technical Issues |
| --- | --- | --- |
| `--busy-loop-stop` still trips on an early idle-thread loop before timer wakeup | Partially Implemented | The first repeated PC is still `0x08161148` around instruction count `0x00308077`, but this is no longer a terminal stall. With free-running timer stepping enabled, `TIM5` later raises IRQ 50 and execution resumes out of that loop. For this target, `--busy-loop-stop` is now a misleading probe unless paired with interrupt traces. |
| Cortex-M debug/timing blocks remain mostly unmodeled | Unimplemented | Firmware writes to `DEMCR` at `0xE000EDFC` and `DWT` registers around `0xE0001000` during scheduler/timer initialization. Those accesses are currently tolerated but not modeled, so any code that depends on DWT cycle counting or debug-trigger side effects may still misbehave. |
| Validation snapshot for current state | Partially Implemented | After the memory-map, DMA, NVIC, SCB, and timer progression fixes, bounded runs now remain stable to 20M and 100M instructions and no longer stay parked at `0x08161148`. A 5M trace shows `TIM5` raising IRQ 50 at `clk=0x00433914`, with the interrupt vector dispatching and returning successfully, but full functional boot is still not demonstrated. |

## Advanced TIM Peripheral

| Feature | Status | Notes/Technical Issues |
| --- | --- | --- |
| TIM1/TIM8 control register coverage (`CR2`, `SMCR`, `EGR`, `CCER`, `BDTR`, `RCR`) | Unimplemented | The current timer stub only exposes a small subset of generic timer registers and does not model advanced-control timer specific state. |
| Channel state for `CCR2`, `CCR3`, `CCR4`, `CCMR1`, and `CCMR2` | Unimplemented | Only `CCR1` is currently handled, so multi-channel output compare and PWM mode decoding are missing. |
| Advanced counting modes (up, down, center-aligned) | Unimplemented | The current counter behavior is a simple monotonically increasing software timebase and does not support direction or center-aligned update rules. |
| Update, compare, commutation, trigger, and break event semantics | Unimplemented | `SR` and `DIER` handling is minimal, so firmware that expects full event flag behavior may stall or mis-handle interrupt state. |
| Complementary outputs, MOE, break input, and dead-time insertion | Unimplemented | Advanced PWM motor-control behavior for TIM1/TIM8 requires output-stage modeling that does not exist in the emulator yet. |
| Timer synchronization and master-slave trigger chaining | Unimplemented | Timer link behavior across TIM peripherals is not implemented, so synchronized startup and chained events are currently unsupported. |
| TIM1/TIM8 DMA request generation | Unimplemented | Advanced timers can raise DMA requests independently; without that behavior, DMA-driven PWM setup or polling loops may not progress correctly. |