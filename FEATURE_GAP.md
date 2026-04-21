# Feature Gap

This file tracks emulator features that are known to be incomplete or not yet implemented.

## Current deliverable

- End goal: get the CubeBlack target to boot and run ArduPilot firmware under the emulator with realistic emulator-side behavior, rather than relying on firmware-side startup bypasses.
- This document is the primary backlog and technical handover for work that remains between the current emulator state and that deliverable.
- Prefer items that unblock demonstrable CubeBlack boot progress first, then follow-on correctness and hardware coverage work.
- Priority values used in item tables: `high` = active ArduPilot runtime blockers only, `medium` = useful but not immediate blockers, `low` = defer until proven needed.

## Concrete success marker

Consider the CubeBlack/ArduPilot deliverable achieved only when all of the following are satisfied:

- Run `cd cubeblack && ../target/release/stm32-emulator config.yaml -v --max-instructions 100000000` and finish with no `WARN`, no `ERROR`, and no `peri=????` unknown peripheral accesses.
- Confirm the `0x08161148` idle-loop probe is not terminal by showing at least one wakeup path (`TIM5` IRQ 50 pending/dispatch/return) in a traced run.
- Capture at least one recognizable firmware runtime marker (for example USART-probe output) or another deterministic runtime milestone and document it in this file.
- Record commands, final instruction count/stop reason, and why the captured marker proves active runtime progress instead of a passive loop.

## Recently Fixed Items

| Feature | Priority | Status | Notes/Technical Issues |
| --- | --- | --- | --- |
| CubeBlack target bring-up assets and board configuration | medium | Implemented | Added in commit `f46e695`. This introduced the initial CubeBlack board files, firmware artifacts, `cubeblack/config.yaml`, `run.sh`, and STM32F427 SVD so the emulator could target CubeBlack at all. |
| CubeBlack bootloader artifact rebuilt for oscillator configuration change | medium | Implemented | Added in commit `5797f0f`. The commit message says the upstream hardware definition changed `OSCILLATOR_HZ` to `0`; in this repo the concrete result is an updated `cubeblack/CubeBlack_bl.hex` artifact used by the emulator. |
| Startup peripheral-ready wait loops bypassed in firmware image | medium | Implemented | Added in commit `377a746` via `drop-waiting-for-peripherals-on-startup.patch` plus rebuilt firmware artifacts. This comments out waits on RCC, PWR, FLASH, PLL, and oscillator ready bits so startup can proceed despite missing emulator-side peripheral readiness semantics. |

## Current Session Implemented Items

| Feature | Priority | Status | Notes/Technical Issues |
| --- | --- | --- | --- |
| CubeBlack memory map updated for CCM RAM and system memory | medium | Implemented | `cubeblack/config.yaml` now maps `RAM-CCM` at `0x10000000` and `SYSMEM` at `0x1FFF0000`, removing earlier unmapped accesses in valid STM32F427 regions. |
| STM32 unique ID bytes patched into system memory | medium | Implemented | `cubeblack/config.yaml` now injects a 12-byte UID payload at `0x1FFF7A10`, covering firmware reads from the STM32 device identifier area. |
| DMA global register decode corrected | medium | Implemented | `src/peripherals/dma.rs` now treats `0x00..0x0f` as `LISR/HISR/LIFCR/HIFCR` and starts stream register decode at `0x10 + n*0x18`, fixing the previous misdecode. |
| DMA transfer-complete flag and clear semantics added | medium | Implemented | `src/peripherals/dma.rs` now tracks `lisr`/`hisr`, sets `TCIF` on completed transfers, and clears flags via `LIFCR`/`HIFCR`, matching the STM32F4 polling model more closely. |
| NVIC stack-pointer restore and IPSR numbering fixed | medium | Implemented | `src/peripherals/nvic.rs` now writes back the selected stack pointer register (`MSP` or `PSP`) instead of always `SP`, and writes architectural exception numbers into `IPSR` using `16 + irq`. |
| Minimal general/basic TIM timebase added | medium | Partially Implemented | `src/peripherals/tim.rs` provides a small stub for `TIM5`, `TIM6`, and `TIM7` with `CR1`, `DIER`, `SR`, `CNT`, `PSC`, `ARR`, and `CCR1`. This is enough for some polling and timeout loops, but it is not a full advanced TIM implementation. |
| Core SCB state and dynamic VTOR handling added | medium | Implemented | `src/peripherals/scb.rs` now exposes basic Cortex-M4 `SCB` state including `CPUID`, `ICSR`, `VTOR`, `AIRCR`, `SHPRx`, and fault-status registers, while `src/peripherals/nvic.rs` now dispatches exceptions through the active `VTOR` instead of a fixed config-time vector table address. |
| Minimal CoreDebug and DWT cycle-counter support added | medium | Partially Implemented | `src/peripherals/core_debug.rs` now models `DEMCR`, `DWT_CTRL`, and `DWT_CYCCNT`, allowing firmware to enable tracing and read a monotonically increasing cycle counter instead of seeing hard-zero values and unknown MMIO accesses during startup. |
| DMA stream TC interrupt signaling to NVIC added | medium | Partially Implemented | `src/peripherals/dma.rs` now raises the mapped STM32F427 DMA stream IRQ pending in NVIC on transfer-complete when `TCIE` is set, in addition to `TCIF` status-bit updates. This closes the gap where completion was only visible via `LISR/HISR`, but request-line driven transfers and broader HT/TE signaling remain incomplete. |
| Minimal OTG FS global/device bring-up added | high | Partially Implemented | `src/peripherals/otg_fs.rs` now handles `OTG_FS_GLOBAL`, `OTG_FS_DEVICE`, and `OTG_FS_PWRCLK` with persisted core registers, minimal `GRSTCTL` idle/reset semantics, a synthetic device-event sequence (`USBRST`, `ENUMDNE`, periodic `SOF`) that raises the real `OTG_FS` IRQ 67, and enough EP0 endpoint bookkeeping to latch `DIEPEMPMSK`, deliver a one-shot setup packet, and raise the zero-length IN `XFRC` completion the firmware expects during early control-transfer bring-up. FIFO payload semantics and any CDC data bridge are still unmodeled. |
| Exception-frame sizing and BASEPRI-aware IRQ gating corrected | high | Implemented | `src/peripherals/nvic.rs` now includes the reserved word required for Cortex-M4F extended FP exception frames and blocks external IRQ dispatch while `BASEPRI` is nonzero. This removed the late scheduler corruption that previously let the ready-list sentinel be selected as a thread and eliminated the `FETCH_UNMAPPED addr=0x00010002` crash at about `10.64M` instructions. |
| Free-running TIM stepping and TIM5 compare wakeup fixed | medium | Implemented | Timers now advance from the emulator main loop instead of only on MMIO access, and `src/peripherals/tim.rs` preserves prescaler remainder across steps. This allows the firmware-configured `TIM5` compare interrupt to fire after the initial idle-loop park, which was previously impossible because the counter froze once the firmware stopped polling it. |
| Busy-loop diagnostic capture added | medium | Implemented | `src/emulator.rs` now logs `PC`, `SP`, `LR`, `R0-R7`, context words, and the restore frame when `--busy-loop-stop` triggers, which made the current scheduler hand-off failure traceable. |
| USART probe line flushing improved | medium | Implemented | `src/ext_devices/usart_probe.rs` now flushes on both CR and LF and also flushes long buffered output, improving visibility of firmware text output during boot debugging. |
| USART register state persistence expanded | medium | Partially Implemented | `src/peripherals/usart.rs` now stores and returns `SR`, `DR`, `BRR`, `CR1`, `CR2`, `CR3`, and `GTPR` state, including `CR3` `DMAT/DMAR` bits needed for DMA-coupled configuration paths. Full interrupt and state-machine accurate USART behavior is still pending. |
| CubeBlack run script hardened for local environments | medium | Implemented | `cubeblack/run.sh` now sources Cargo environment when present and falls back to the built release binary, reducing local setup friction. |

## Current Handover Blockers

| Feature | Priority | Status | Notes/Technical Issues |
| --- | --- | --- | --- |
| `--busy-loop-stop` still trips on an early idle-thread loop before timer wakeup | high | Partially Implemented | The first repeated PC is still `0x08161148` around instruction count `0x00308077`, but this is no longer a terminal stall. With free-running timer stepping enabled, `TIM5` later raises IRQ 50 and execution resumes out of that loop. For this target, `--busy-loop-stop` is now a misleading probe unless paired with interrupt traces. |
| USB OTG FS RXFLVL/GRXSTSP enumeration sequence completed | high | Implemented | `src/peripherals/otg_fs.rs` now delivers the full synthetic USB enumeration sequence: PKTSTS=6 (setup data) → FIFO reads → PKTSTS=4 (setup complete) → DOEPINT0.STUP|XFRC → EP0 IN ZLP XFRC. Both `SET_ADDRESS 1` and `SET_CONFIGURATION 1` are delivered and acknowledged. Fixed: (1) continuous RXFLVL re-assertion in step() to re-fire IRQ between PKTSTS=6 and PKTSTS=4 pops; (2) `irq_latched` forced false after STUP fires so OEPINT triggers a fresh IRQ for the SETUP callback. After successful enumeration, `enum_stage=Configured` and the firmware proceeds to runtime operation. |
| First ArduPilot console line captured over USB CDC | high | Implemented | The emulator now captures CDC bulk IN traffic from EP1 and surfaces it as host-side log output. Confirmed working runtime marker: `USB-CDC ep1 'Unable to init RAMTRON storage'` at clk=`10615218`, proving ArduPilot reached active runtime code and emitted console text through the emulated USB serial path. |
| CoreDebug/DWT coverage is still minimal | medium | Partially Implemented | The emulator now handles `DEMCR`, `DWT_CTRL`, and `DWT_CYCCNT`, which removes the earlier unknown accesses and hard-zero startup reads. The remaining gap is breadth: the wider DWT/CoreDebug register set and any debug-trigger side effects are still unmodeled. |
| Validation snapshot for current state | high | Implemented | The current release build completes bounded runs to `100M` instructions with no `WARN`, no `ERROR`, and no `peri=????` unknown peripheral accesses. Command: `cd cubeblack && ../target/release/stm32-emulator config.yaml -v --max-instructions 100000000`. Stop reason: reached 100M instructions at pc=0x08005458. Runtime milestones confirmed in this run: (1) USB OTG FS IRQ 67 fired and USB enumeration (SET_ADDRESS + SET_CONFIGURATION) completed at ~440K instructions; (2) DMA2 ADC1 read initiated at clk=10552392 and DMA1 SPI2 read/write initiated at clk=10603522, confirming the firmware is actively polling sensors during runtime; (3) periodic TIM5 and OTG SOF IRQ service remained visible throughout the run; (4) the first ArduPilot console line was captured over USB CDC: `USB-CDC ep1 'Unable to init RAMTRON storage'` at clk=`10615218`. |

## Peripheral TODO Audit (Datasheet-Backed)

The items below are concrete implementation gaps found in `src/peripherals/*.rs`, with reference targets in:

- `cubeblack/STM32F4xx_Reference_Manual.md`
- `cubeblack/stm32f427vg-datasheet.md`
- `cubeblack/STM32F4_DMA.md` (AN4031 conversion)

### Runtime-First Priority Order (ArduPilot Needs To Run)

Work this list top-to-bottom; defer lower tiers until higher tiers are demonstrably improved.

- P0 (must-have for runtime):
	- DMA request-line mapping and DMA interrupt signaling (TC/HT/TE to NVIC)
	- UART/USART DMA coupling (`DMAT/DMAR`) and realistic SR/interrupt behavior
	- SPI DMA request generation and stateful SR behavior for sensor traffic
	- I2C transaction-state machine and status semantics needed for sensor bring-up
	- TIM channels/modes actually used by scheduler and runtime device drivers
- P1 (likely runtime-sensitive next):
	- RCC clock-state transitions and clock-frequency effects on peripheral timing
	- NVIC priority/enable behavior improvements when multiple interrupts compete
	- Expanded CoreDebug/DWT counters only if firmware actively consumes them during runtime
- P2 (defer until runtime markers are present):
	- Advanced TIM breadth not used by current CubeBlack runtime path
	- Wider SCB fault-model depth not currently hit in boot/runtime traces
	- Non-critical peripheral features and generalized completeness work

### DMA and Peripheral-DMA Coupling

- [ ] Implement DMA request-line mapping (stream/channel to peripheral request), using `STM32F4_DMA.md` Table 1 and Table 2 for STM32F427/429 mappings.
- [ ] Implement peripheral-driven DMA requests instead of only synchronous transfer-on-EN behavior (`DMA_SxCR.EN` currently causes immediate do_xfer).
- [ ] Implement DMA interrupt signaling into NVIC for TC/HT/TE classes (not only flag bits in `LISR/HISR`).
- [ ] Implement FIFO/direct-mode behavior and threshold semantics (`DMA_SxFCR`) per AN4031 FIFO section.
- [ ] Implement channel/request conflict handling (AN4031 notes that multiple enabled streams must not serve the same peripheral request).

### USART/UART

- [ ] Implement `CR1/CR2/CR3` behavior beyond DR/SR stubs (UE/TE/RE, stop bits, parity, interrupt enable bits).
- [ ] Implement realistic `SR` flag transitions (`TXE`, `TC`, `RXNE`, `IDLE`, error bits) instead of always-ready reads.
- [ ] Implement DMA coupling for UART/USART (`DMAT/DMAR` in `CR3`) so serial TX/RX can be DMA-driven.
- [ ] Implement UART/USART interrupt generation (TXE/RXNE/TC/error paths) and clearing rules.
- [ ] Implement baud-rate effects (`BRR`) enough for firmware timing assumptions.

### SPI/I2S

- [ ] Implement stateful SPI status flags (`TXE`, `RXNE`, `BSY`, OVR/MODF paths) instead of synthetic toggle behavior.
- [ ] Implement SPI DMA request generation for RX/TX paths.
- [ ] Implement core control semantics (`CPOL`, `CPHA`, frame format, NSS/master-slave effects) required by CubeBlack peripherals.
- [ ] Implement SPI error/interrupt signaling (RXNE/TXE/ERR interrupt paths).

### I2C

- [ ] Replace toggled `SR1/SR2` stubs with a real transaction state machine (start/address/data/stop progression).
- [ ] Implement key I2C status/control bit semantics (`SB`, `ADDR`, `BTF`, `RXNE`, `TXE`, `AF`, `BERR`, `ARLO`), including clear sequencing.
- [ ] Implement I2C DMA request generation and interrupt paths.
- [ ] Add board-level I2C device behavior hooks for sensor bring-up paths used by ArduPilot.

### TIM

- [ ] Extend timer coverage beyond minimal TIM5/6/7 subset; prioritize timer instances/channels used by ArduPilot runtime.
- [ ] Implement additional capture/compare channels (`CCR2/3/4`) and mode registers (`CCMR1/2`, `CCER`) semantics.
- [ ] Implement event-generation and synchronization semantics (`EGR`, `SMCR`, update/compare event behavior).
- [ ] Implement timer DMA-request generation paths where firmware expects DMA-triggered operation.

### RCC/PWR/Clocking

- [ ] Replace always-ready RCC behavior with stateful clock-enable/ready transitions (`CR`, `CFGR`, PLL/HSE/LSE paths) to remove hidden firmware bypass dependence.
- [ ] Implement effective bus/clock configuration impacts needed by timing-sensitive peripherals (UART, TIM, DMA pacing assumptions).

### SCB/NVIC/CoreDebug

- [ ] Implement deeper fault-path semantics (fault status population + routing/escalation behavior) rather than read/write storage only.
- [ ] Extend CoreDebug/DWT coverage beyond minimal `DEMCR`, `DWT_CTRL`, `DWT_CYCCNT` where firmware uses additional counters/registers.
- [ ] Improve NVIC priority/enable behavior to better match real exception arbitration when multiple interrupt sources are active.

## Advanced TIM Peripheral

| Feature | Priority | Status | Notes/Technical Issues |
| --- | --- | --- | --- |
| TIM1/TIM8 control register coverage (`CR2`, `SMCR`, `EGR`, `CCER`, `BDTR`, `RCR`) | low | Unimplemented | The current timer stub only exposes a small subset of generic timer registers and does not model advanced-control timer specific state. |
| Channel state for `CCR2`, `CCR3`, `CCR4`, `CCMR1`, and `CCMR2` | low | Unimplemented | Only `CCR1` is currently handled, so multi-channel output compare and PWM mode decoding are missing. |
| Advanced counting modes (up, down, center-aligned) | low | Unimplemented | The current counter behavior is a simple monotonically increasing software timebase and does not support direction or center-aligned update rules. |
| Update, compare, commutation, trigger, and break event semantics | low | Unimplemented | `SR` and `DIER` handling is minimal, so firmware that expects full event flag behavior may stall or mis-handle interrupt state. |
| Complementary outputs, MOE, break input, and dead-time insertion | low | Unimplemented | Advanced PWM motor-control behavior for TIM1/TIM8 requires output-stage modeling that does not exist in the emulator yet. |
| Timer synchronization and master-slave trigger chaining | low | Unimplemented | Timer link behavior across TIM peripherals is not implemented, so synchronized startup and chained events are currently unsupported. |
| TIM1/TIM8 DMA request generation | low | Unimplemented | Advanced timers can raise DMA requests independently; without that behavior, DMA-driven PWM setup or polling loops may not progress correctly. |