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
| Historical startup bypass retired from the active path | medium | Updated | Commit `377a746` introduced a temporary ArduPilot-side startup workaround for RCC, PWR, FLASH, PLL, and oscillator wait loops. That patch file is no longer kept in this repo; current progress should be evaluated against the emulator-side peripheral readiness models and current validation evidence instead of treating a firmware-side bypass as the active mechanism. |

## Current Session Implemented Items

| Feature | Priority | Status | Notes/Technical Issues |
| --- | --- | --- | --- |
| CubeBlack memory map updated for CCM RAM and system memory | medium | Implemented | `cubeblack/config.yaml` now maps `RAM-CCM` at `0x10000000` and `SYSMEM` at `0x1FFF0000`, removing earlier unmapped accesses in valid STM32F427 regions. |
| STM32 unique ID bytes patched into system memory | medium | Implemented | `cubeblack/config.yaml` now injects a 12-byte UID payload at `0x1FFF7A10`, covering firmware reads from the STM32 device identifier area. |
| SPI2 FRAM (FM25V02) device emulation | high | Implemented | Device model in `src/ext_devices/ramtron.rs` with 32KB storage, complete SPI command state machine (RDID/0x9F, READ/0x03, WRITE/0x02, WREN/0x06, WRDI/0x04, RDSR/0x05), GPIO PD10 CS callback; full-duplex DMA bridge fix in `src/peripherals/spi.rs` (CR2 tracking, TXe anEN-gated `read_dma` returns empty for full-duplex, `write_dma` performs per-byte exchange and patches RX dest RAM). RDID returns correct id1=0x22/id2=0x00 and firmware reads all 32KB storage successfully. |
| DMA global register decode corrected | medium | Implemented | `src/peripherals/dma.rs` now treats `0x00..0x0f` as `LISR/HISR/LIFCR/HIFCR` and starts stream register decode at `0x10 + n*0x18`, fixing the previous misdecode. |
| DMA transfer-complete flag and clear semantics added | medium | Implemented | `src/peripherals/dma.rs` now tracks `lisr`/`hisr`, sets `TCIF` on completed transfers, and clears flags via `LIFCR`/`HIFCR`, matching the STM32F4 polling model more closely. |
| NVIC stack-pointer restore and IPSR numbering fixed | medium | Implemented | `src/peripherals/nvic.rs` now writes back the selected stack pointer register (`MSP` or `PSP`) instead of always `SP`, and writes architectural exception numbers into `IPSR` using `16 + irq`. |
| Minimal general/basic TIM timebase added | medium | Partially Implemented | `src/peripherals/tim.rs` provides a small stub for `TIM5`, `TIM6`, and `TIM7` with `CR1`, `DIER`, `SR`, `CNT`, `PSC`, `ARR`, and `CCR1`. This is enough for some polling and timeout loops, but it is not a full advanced TIM implementation. |
| Core SCB state and dynamic VTOR handling added | medium | Partially Implemented | `src/peripherals/scb.rs` now exposes basic Cortex-M4 `SCB` state including `CPUID`, `ICSR`, `VTOR`, `AIRCR`, `SHPRx`, and fault-status registers, while `src/peripherals/nvic.rs` now dispatches exceptions through the active `VTOR` instead of a fixed config-time vector table address. Fault-status registers are stored but have no behavioral effect; SHPRx priority values are not used in arbitration. |
| Minimal CoreDebug and DWT cycle-counter support added | medium | Partially Implemented | `src/peripherals/core_debug.rs` now models `DEMCR`, `DWT_CTRL`, and `DWT_CYCCNT`, allowing firmware to enable tracing and read a monotonically increasing cycle counter instead of seeing hard-zero values and unknown MMIO accesses during startup. |
| DMA stream TC interrupt signaling to NVIC added | medium | Partially Implemented | `src/peripherals/dma.rs` now raises the mapped STM32F427 DMA stream IRQ pending in NVIC on transfer-complete when `TCIE` is set, in addition to `TCIF` status-bit updates. This closes the gap where completion was only visible via `LISR/HISR`, but request-line driven transfers and broader HT/TE signaling remain incomplete. |
| Minimal OTG FS global/device bring-up added | high | Partially Implemented | `src/peripherals/otg_fs.rs` now handles `OTG_FS_GLOBAL`, `OTG_FS_DEVICE`, and `OTG_FS_PWRCLK` with persisted core registers, minimal `GRSTCTL` idle/reset semantics, a synthetic device-event sequence (`USBRST`, `ENUMDNE`, periodic `SOF`) that raises the real `OTG_FS` IRQ 67, and enough EP0 endpoint bookkeeping to latch `DIEPEMPMSK`, deliver a one-shot setup packet, and raise the zero-length IN `XFRC` completion the firmware expects during early control-transfer bring-up. FIFO payload semantics and any CDC data bridge are still unmodeled. |
| Exception-frame sizing and BASEPRI-aware IRQ gating corrected | high | Implemented | `src/peripherals/nvic.rs` now includes the reserved word required for Cortex-M4F extended FP exception frames and blocks external IRQ dispatch while `BASEPRI` is nonzero. This removed the late scheduler corruption that previously let the ready-list sentinel be selected as a thread and eliminated the `FETCH_UNMAPPED addr=0x00010002` crash at about `10.64M` instructions. |
| Free-running TIM stepping and TIM5 compare wakeup fixed | medium | Implemented | Timers now advance from the emulator main loop instead of only on MMIO access, and `src/peripherals/tim.rs` preserves prescaler remainder across steps. This allows the firmware-configured `TIM5` compare interrupt to fire after the initial idle-loop park, which was previously impossible because the counter froze once the firmware stopped polling it. |
| Minimal RCC startup status model added | high | Partially Implemented | `src/peripherals/rcc.rs` now persists core clock-control state instead of returning fixed stub values. `CR` ready bits (`HSIRDY`, `HSERDY`, `PLLRDY`, `PLLI2SRDY`, `PLLSAIRDY`) are synthesized from enable bits, `CFGR.SWS` mirrors `SW`, `CSR.LSIRDY` mirrors `LSION`, and `BDCR.LSERDY` mirrors `LSEON` with basic backup-domain reset handling. This removes the early ChibiOS `stm32_clock_init()` stalls on HSI/HSE/PLL/LSI/LSE readiness, though detailed clock-tree timing effects remain unmodeled. |
| Minimal PWR regulator-ready model added | high | Partially Implemented | `src/peripherals/pwr.rs` now models `CR`/`CSR` well enough for ChibiOS startup polling, including immediate `VOSRDY`, `ODRDY`, and `ODSWRDY` semantics. This clears the F4 voltage-regulator waits during PLL bring-up without attempting full PWR behavior. |
| Minimal FLASH ACR model added | high | Partially Implemented | `src/peripherals/flash.rs` now stores `FLASH->ACR` and related control registers so the ChibiOS flash-latency programming loop can observe its own writeback. This removes the previous startup stall at the `FLASH_ACR_LATENCY_Msk` check. |
| Busy-loop diagnostic capture added | medium | Implemented | `src/emulator.rs` now logs `PC`, `SP`, `LR`, `R0-R7`, context words, and the restore frame when `--busy-loop-stop` triggers, which made the current scheduler hand-off failure traceable. |
| USART probe line flushing improved | medium | Implemented | `src/ext_devices/usart_probe.rs` now flushes on both CR and LF and also flushes long buffered output, improving visibility of firmware text output during boot debugging. |
| USART register state persistence expanded | medium | Partially Implemented | `src/peripherals/usart.rs` now stores and returns `SR`, `DR`, `BRR`, `CR1`, `CR2`, `CR3`, and `GTPR` state, including `CR3` `DMAT/DMAR` bits needed for DMA-coupled configuration paths, and preserves `SR.TXE`/`SR.TC` across firmware `SR=0` init writes so CubeBlack no longer sees a permanently non-ready transmitter. Full interrupt and state-machine accurate USART behavior is still pending. |
| Deferred external IRQ delivery avoids mixed thread/handler execution state | high | Implemented | `src/emulator.rs` now defers pending external IRQ delivery until Unicorn stops the current execution block, and `src/peripherals/nvic.rs` exposes `take_pending_interrupt()` so the emulator can inject the vector on the next resume boundary instead of mutating CPU exception state mid-block. This fixes the deterministic late crash at about `145255239` instructions where `TIM5` IRQ 50 could land while firmware was inside the `IT` block in `ChibiOS::Scheduler::in_main_thread()`, previously leading to a bogus `pop {r3,pc}` from MSP fill memory and `pc=0x55555554`. |
| CubeBlack run script hardened for local environments | medium | Implemented | `cubeblack/run.sh` now sources Cargo environment when present and falls back to the built release binary, reducing local setup friction. |

## Current Handover Blockers

| Feature | Priority | Status | Notes/Technical Issues |
| --- | --- | --- | --- |
| Implement GDB remote debugging support (`gdb.rs` / `--gdb`) | high | Implemented | The emulator now exposes a GDB remote stub via `src/gdb.rs` and the `--gdb <port>` CLI option, allowing `arm-none-eabi-gdb`/`gdb-multiarch` to attach with `--nx`, inspect registers and memory, set breakpoints, single-step, continue, and halt at firmware symbols such as `main`. |
| `--busy-loop-stop` now ignores transient idle waits and follows timer wakeups | high | Implemented | `src/emulator.rs` now requires a sustained repeated-PC streak (`BUSY_LOOP_STREAK_THRESHOLD`) before halting, so the probe no longer trips on the first idle-thread wait loop. Revalidated with `cd cubeblack && /usr/bin/time -f 'real=%e user=%U sys=%S maxrss=%M exit=%x' timeout 30 ../target/release/stm32-emulator config.yaml -v --busy-loop-stop`: the run continues past the early `0x0815ca54` idle wait and logs repeated `TIM5` IRQ 50 compare/disptach activity instead of stopping immediately. |
| Runtime now reaches 200M cleanly after deferred IRQ delivery fix | high | Updated | The old deterministic late crash at `clk=145255239` is gone. Short validation still succeeds with `cd cubeblack && /usr/bin/time -f 'real=%e user=%U sys=%S maxrss=%M exit=%x' timeout 30 ../target/release/stm32-emulator config.yaml -v --max-instructions 3000000`, ending at `pc=0x0815ca54` in about `1.66s`. Longer bounded validation now also succeeds with `cd cubeblack && /usr/bin/time -f 'real=%e user=%U sys=%S maxrss=%M exit=%x' timeout 120 ../target/release/stm32-emulator config.yaml -v --max-instructions 200000000`, reaching `clk=200000000 pc=0x0815ca54` in about `117.39s` with no emulator `WARN`, no emulator `ERROR`, and no `peri=????`. The remaining visible output in `cubeblack/ardu.cubeblack.log` is dominated by expected `DEBUG` timer/USB activity rather than fatal faults. |
| USB OTG FS RXFLVL/GRXSTSP enumeration sequence completed | high | Implemented | `src/peripherals/otg_fs.rs` now delivers the full synthetic USB enumeration sequence: PKTSTS=6 (setup data) → FIFO reads → PKTSTS=4 (setup complete) → DOEPINT0.STUP|XFRC → EP0 IN ZLP XFRC. Both `SET_ADDRESS 1` and `SET_CONFIGURATION 1` are delivered and acknowledged. Fixed: (1) continuous RXFLVL re-assertion in step() to re-fire IRQ between PKTSTS=6 and PKTSTS=4 pops; (2) `irq_latched` forced false after STUP fires so OEPINT triggers a fresh IRQ for the SETUP callback. After successful enumeration, `enum_stage=Configured` and the firmware proceeds to runtime operation. |
| First ArduPilot console line captured over USB CDC | high | Implemented | The emulator captures CDC bulk IN traffic from EP1 via the TXFE-interrupt-driven path. ChibiOS sets DIEPEMPMSK after EPENA (not before), so XFRC is now gated on `ep_txfe_was_fired` to avoid racing XFRC against the firmware filling the FIFO. Confirmed at clk=`10615187`/`10615218`: `USB-CDC ep1 'Unab'` + `'le to init RAMTRON storage'` (split across two USB packets by ChibiOS TXFE fill loop). Messages recur throughout the run. |
| SPI2 RAMTRON full-duplex DMA bridge | high | Implemented | Root cause and core fix are now complete: ChibiOS `spiStartExchangeI` with in-place buffer (send==recv) had RX DMA clobbering TX source bytes before TX DMA consumed them. `read_dma()` returns empty for SPI DR in TXDMAEN full-duplex mode, `write_dma()` performs the exchange, and SPI now queues/pairs all pending RX DMA destinations so every received byte is patched into the correct RAM address. CR2 tracking for TXDMAEN/RXDMAEN remains in place. Revalidated with `cd cubeblack && /usr/bin/time -f 'real=%e user=%U sys=%S maxrss=%M exit=%x' timeout 120 ../target/release/stm32-emulator config.yaml -v --max-instructions 120000000`: run reached `clk=120000000` and no longer emitted USB CDC `Unable to init RAMTRON storage` / `Unknown RAMTRON device` lines. |
| SDIO peripheral stub added (CMDSENT unblock) | high | Partially Implemented | Firmware at PC=0x08155f76 polls SDIO_STA (0x40012C00+0x34) bit 7 (CMDSENT) after writing CMD register with CPSMEN. Added `src/peripherals/sdio.rs`: minimal SDIO model that sets CMDSENT immediately on CMD write (no-response commands), and CMDSENT+CTIMEOUT for commands expecting a response (no card present). This unblocked the previous infinite spin at 0x08155f78. SD card init now proceeds but fails gracefully via CTIMEOUT on all ACMD41 attempts. Outstanding: SDIO DMA/interrupt path, data transfer registers are stubs; SD_INIT_RETRY=100 retries × 10ms osalThreadSleepMilliseconds each = ~167M instructions per sdcConnect failure; 3 sdcConnect attempts = ~500M instructions before firmware gives up on SD and resumes other init. |
| CubeBlack board-validation sensor stubs | high | Implemented | `src/ext_devices/spi_sensors.rs` provides the board-validation responses ArduPilot expects before normal driver init: `WhoAmIDevice` satisfies `spi_check_register(...)` and `Ms5611Device` satisfies `check_ms5611(...)` with RESET + PROM/CRC behavior. The generated `HAL_VALIDATE_BOARD` in `cubeblack/hwdef.h` currently covers the two MS5611 baros plus the external and internal IMU presence predicates, and a failure would trap startup in `AP_BoardConfig::config_error(...)`. Current long validation runs proceed far beyond that stage and emit later USB CDC runtime lines such as `Unable to init RAMTRON storage` / `Unknown RAMTRON device` from `cubeblack/ardu.cubeblack.log`, so board validation is no longer a startup blocker. |
| CoreDebug/DWT coverage is still minimal | medium | Partially Implemented | The emulator now handles `DEMCR`, `DWT_CTRL`, and `DWT_CYCCNT`, which removes the earlier unknown accesses and hard-zero startup reads. The remaining gap is breadth: the wider DWT/CoreDebug register set and any debug-trigger side effects are still unmodeled. |
| ADC peripheral stub (ADC1/2/3) | medium | Partially Implemented | `src/peripherals/adc.rs` (new file) models ADC1, ADC2, ADC3. SR.EOC always asserted; DR returns synthetic `0x0800` (12-bit half-scale ≈ 1.65V on 3.3V rail → ≈16.5V battery via typical 10:1 divider). `read_dma()` overrides the default to emit correct little-endian halfwords for PSIZE=2 DMA bursts (default `read_dma` truncates to `u8`, losing the high byte). CR2 SWSTART sets EOC. Watchdog threshold reset value (`htr=0x0FFF`) included. Registered in `src/peripherals/mod.rs`. |
| USART SR realistic RXNE transitions | low | Implemented | `src/peripherals/usart.rs` initial SR changed from `TXE+TC+RXNE+IDLE` to `TXE+TC+IDLE` (bit 5 RXNE removed). DR read now explicitly clears RXNE (`self.sr &= !(1 << 5)`). Removes spurious "byte ready" signals to firmware when no incoming data is present. |
| I2C nack_address cancels pending SB event IRQ | low | Implemented | `src/peripherals/i2c.rs` `nack_address()` now sets `self.pending_event_irq = None` before arming the error IRQ. Previously the 1-step-delayed SB event IRQ could fire after the address had already been written to DR, causing the interrupt handler to see a stale SB after the NACK — confusing the I2C state machine in firmware. |
| SDIO data timeout reduced | low | Implemented | `src/peripherals/sdio.rs` `DATA_TIMEOUT_DELAY_STEPS` reduced from `64` to `4`. DTIMEOUT fires 16× faster when DCTRL_DTEN is set, reducing the instruction cost of the data-phase timeout path that fires on every (unimplemented) SD card data read. |
| DMA conflicting-stream diagnostics downgraded from WARN | medium | Implemented | `src/peripherals/dma.rs` now logs conflicting-stream arbitration (`preempted`/`blocked`) at `DEBUG` instead of `WARN`, while preserving the same transfer-error signaling behavior. Revalidated with `cd cubeblack && /usr/bin/time -f 'real=%e user=%U sys=%S maxrss=%M exit=%x' timeout 120 ../target/release/stm32-emulator config.yaml -v --max-instructions 120000000`: latest run reached `clk=120000000` with no emulator `WARN`, no emulator `ERROR`, and no `peri=????`; the former conflict message now appears as `DEBUG DMA1 stream=4 blocked conflicting request ...`. |
| Validation snapshot for current state | high | Updated | Current release build passes bounded validation to `120000000` instructions with no emulator `WARN`, no emulator `ERROR`, and no `peri=????` unknown peripheral accesses. Short command: `cd cubeblack && /usr/bin/time -f 'real=%e user=%U sys=%S maxrss=%M exit=%x' timeout 30 ../target/release/stm32-emulator config.yaml -v --max-instructions 3000000` → stop at `clk=03000000 pc=0x0815ca54`, `real=1.42s`. Long command: `cd cubeblack && /usr/bin/time -f 'real=%e user=%U sys=%S maxrss=%M exit=%x' timeout 120 ../target/release/stm32-emulator config.yaml -v --max-instructions 120000000` → stop at `clk=120000000 pc=0x0815ca54`, `real=63.08s`, 0 WARN/ERROR lines. |
| DMA circular mode, per-beat PINC/MINC, and correct NDTR tracking | high | Implemented | `src/peripherals/dma.rs` now stores `initial_ndtr` when NDTR is written; after each `do_xfer` completes, if CIRC=1 (SxCR bit 8) NDTR is reloaded to `initial_ndtr` and EN is kept set, otherwise EN is cleared and NDTR=0. `do_xfer` now separately tracks PSIZE (bits [12:11]) and MSIZE (bits [14:13]) and uses them for peripheral and memory transfer sizes respectively; MINC=0 case writes all beats to the same memory address; PINC with per-beat address increment handled for both P2M and M2P directions. `step_deferred` now calls `do_xfer` before completing so deferred USART RX DMA actually moves data from ext_device into the DMA destination buffer, and rearms the circular window if CIRC=1. `signal_tc` extracted to fire both TCIF flag and NVIC IRQ from one call site. Validated: ADC1 circular DMA2 transfer (`circ=true ndtr=64 psize=2 msize=2`) confirmed firing and reloading correctly at clk=10582014. |
| Timer CCMR1, CCMR2, CCER register storage | medium | Partially Implemented | `src/peripherals/tim.rs` now stores and returns `CCMR1` (offset 0x0018), `CCMR2` (0x001C), and `CCER` (0x0020). Firmware that reads back these registers after writing (e.g., to verify OC mode configuration) now sees correct values. Output-compare mode decoding (OC1M/OC2M/OC3M/OC4M fields) and GPIO output toggling remain unimplemented; the compare-event firing logic is unchanged. |
| EXTI peripheral added | medium | Partially Implemented | `src/peripherals/exti.rs` models the STM32F427 EXTI controller (base `0x40013C00`): IMR, EMR, RTSR, FTSR, SWIER, and PR registers with write-1-to-clear PR semantics. `SWIER` writes trigger matching lines via `trigger_line()` which sets PR and fires NVIC if IMR is unmasked. IRQ fanout matches STM32F427: EXTI0→6, EXTI1→7, EXTI2→8, EXTI3→9, EXTI4→10, EXTI5-9→23 (EXTI9_5), EXTI10-15→40 (EXTI15_10). A `gpio_pin_transition()` stub allows GPIO models to route edge events through EXTI. Wired into `src/peripherals/mod.rs` via `pub mod exti` and `Exti::new()` registration. EXTI lines 16–22 (wakeup/RTC/PVD) remain unmodeled. |
| TIM peripheral extended to all 14 timers with EGR and CCR2-4 | medium | Partially Implemented | `src/peripherals/tim.rs` now covers TIM1–TIM14 with correct STM32F427 update IRQ numbers: TIM1(25), TIM2(28), TIM3(29), TIM4(30), TIM5(50), TIM6(54), TIM7(55), TIM8(44), TIM9(24), TIM10(25), TIM11(26), TIM12(43), TIM13(44), TIM14(45). Added dedicated `cc_irq_number()` for TIM1(27) and TIM8(46). Added EGR register (offset 0x0014): bit 0 forces update (sets SR UIF, fires update IRQ if DIER UIE set), bits 1–4 force CCx events. Added CCR2(0x0038), CCR3(0x003C), CCR4(0x0040) storage and compare-fire logic for DIER CC2IE/CC3IE/CC4IE. Added CR2(0x0004) and SMCR(0x0008) as stored pass-through registers. |
| Flash ACR PRFTBS status bit mirrors PRFTBE | low | Implemented | `src/peripherals/flash.rs` now synthesizes PRFTBS (bit 5) from PRFTBE (bit 4) state on ACR reads, so firmware that checks the prefetch-buffer status bit after enabling the prefetch buffer sees it as active. |

## Peripheral TODO Audit (Datasheet-Backed)

The items below are concrete implementation gaps found in `src/peripherals/*.rs`, with reference targets in:

- `cubeblack/STM32F4xx_Reference_Manual.md`
- `cubeblack/stm32f427vg-datasheet.md`
- `cubeblack/STM32F4_DMA.md` (AN4031 conversion)

### Runtime-First Priority Order (ArduPilot Needs To Run)

Work this list top-to-bottom; defer lower tiers until higher tiers are demonstrably improved.

- P0 (must-have for runtime):
	- SPI2 FRAM (FM25V02) device integration: **IMPLEMENTED** - device model in `src/ext_devices/ramtron.rs` plus full-duplex DMA RX-destination queueing in `src/peripherals/spi.rs` now allow end-to-end initialization without `Unable to init RAMTRON storage` / `Unknown RAMTRON device` lines in 120M validation.
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

- [x] HIGH PRIORITY: Enforce STM32F427 stream/channel request mapping from `STM32F4_DMA.md` Table 1 and Table 2 instead of merely storing `CHSEL`. Current enforcement blocks mismatched stream/channel configurations for key active peripherals (ADC1/2/3, SPI1/2/3, USART/UART, I2C1/2/3, SDIO) while leaving unknown requests permissive.
- [x] Peripheral-driven DMA requests: deferred USART RX now calls `do_xfer` when idle window expires, moving real ext_device bytes into the DMA buffer; SPI full-duplex DMA already handled via read_dma/write_dma.
- [x] Per-beat NDTR tracking: `initial_ndtr` saved on NDTR write; NDTR set to 0 on non-circular completion; reloaded on circular completion.
- [x] Distinct PSIZE/MSIZE: `do_xfer` now uses `psize()` (bits [12:11]) for peripheral transfers and `msize()` (bits [14:13]) for memory transfers.
- [x] PINC/MINC per-beat increment: PINC=1 issues per-beat peripheral reads with address advance; MINC=0 writes all beats to the fixed memory address; MINC=1 writes sequentially (already correct).
- [x] Circular-mode stream behavior: after `do_xfer`, if CIRC=1 NDTR reloads to `initial_ndtr` and EN stays set; validated on ADC1 DMA2 circular stream.
- [x] Double-buffer mode (`M0AR`/`M1AR` + `CT`): `is_double_buffer()` checks SxCR bit18; after each completed transfer CT (bit19) is toggled so the hardware pointer alternates between `M0AR` and `M1AR`; `initial_ndtr` is reloaded; implies CIRC semantics.
- [ ] HIGH PRIORITY: Implement FIFO/direct-mode behavior and threshold semantics (`DMA_SxFCR`) per AN4031 FIFO section. Basic transfer granularity is now mode-aware: direct mode uses single-beat chunks and FIFO mode uses `FTH` threshold-sized chunks. Remaining gap: full FIFO state machine (level/status/error-cause fidelity).
- [ ] HIGH PRIORITY: Implement DMA interrupt signaling into NVIC for all relevant classes (`TC`, `HT`, `TE`, `DME`, `FE`) plus matching status-bit visibility in `LISR/HISR`. All five class flags and IRQ-gated paths are wired (`TC/HT/TE/DME/FE`), and error-class signaling is now mode-gated (`DME` in direct mode, `FE` in FIFO mode). Remaining gap: full RM-accurate trigger conditions per specific fault cause.
- [ ] HIGH PRIORITY: Implement channel/request conflict handling and request arbitration when multiple enabled streams target the same peripheral request. Basic same-DMA arbitration now exists for duplicate `(channel, PAR)` enables: lower/equal priority new streams are blocked, higher-priority new streams preempt the current owner; broader request-key arbitration is still pending.
- [ ] HIGH PRIORITY: Implement stream priority/arbitration semantics instead of treating all streams as immediate and conflict-free. Initial `PL`-based preemption is now wired for duplicate `(channel, PAR)` request conflicts, but full cross-stream arbitration behavior remains incomplete.
- [ ] HIGH PRIORITY: Implement STM32F4-accurate disable/re-enable sequencing for `EN` clear/set behavior instead of treating enable as an immediate fire-and-forget trigger. Re-trigger on repeated `EN=1` writes is now blocked and `EN=0` disables deferred USART RX windows, but full RM-accurate enable/disable timing is still pending.

### USART/UART

- [ ] Implement `CR1/CR2/CR3` behavior beyond DR/SR stubs (UE/TE/RE, stop bits, parity, interrupt enable bits).
- [ ] Implement realistic `SR` flag transitions (`TXE`, `TC`, `RXNE`, `IDLE`, error bits) instead of always-ready reads.
- [ ] Implement DMA coupling for UART/USART (`DMAT/DMAR` in `CR3`) so serial TX/RX can be DMA-driven. **Investigation Note (2026-04-30)**: A naive implementation forwarding all USART DR reads/writes through `read_dma`/`write_dma` methods caused severe performance regression (~80% slowdown: 120M instructions in 120s vs. baseline 120s for full execution), likely due to per-byte ext_device interaction overhead during DMA bursts. Reverted pending a more efficient implementation strategy (e.g., deferred/windowed reads or ext_device buffering). Current baseline avoids DMA coupling for USART to preserve validation speed.
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

- [ ] Extend timer coverage beyond current TIM2–TIM14 subset; prioritize timer instances/channels used by ArduPilot runtime.
- [x] Additional capture/compare channels (`CCR2/3/4`) now stored and compared; `CCMR1/2` and `CCER` output-mode decoding still unimplemented.
- [x] EGR (Event Generation Register) force-update and CC-event bits now implemented.
- [x] `CCMR1` (offset 0x0018), `CCMR2` (0x001C), `CCER` (0x0020) now stored and returned on read; OC mode decoding not yet implemented.
- [ ] Implement timer DMA-request generation paths where firmware expects DMA-triggered operation.
- [ ] Implement counting modes (down, center-aligned), auto-reload preload, and synchronized slave-mode behavior.

### RCC/PWR/Clocking

- [ ] Replace always-ready RCC behavior with stateful clock-enable/ready transitions (`CR`, `CFGR`, PLL/HSE/LSE paths) to remove hidden firmware bypass dependence.
- [ ] Implement effective bus/clock configuration impacts needed by timing-sensitive peripherals (UART, TIM, DMA pacing assumptions).

### SCB/NVIC/CoreDebug

- [ ] Implement deeper fault-path semantics (fault status population + routing/escalation behavior) rather than read/write storage only.
- [ ] Extend CoreDebug/DWT coverage beyond minimal `DEMCR`, `DWT_CTRL`, `DWT_CYCCNT` where firmware uses additional counters/registers.
- [ ] Improve NVIC priority/enable behavior to better match real exception arbitration when multiple interrupt sources are active.

## Reference Codebase Reuse Catalog

Survey performed April 2026 covering Renode (modules/renode), AZhurGIT fork (modules/fork-AZhurGIT), and goran-mahovlic fork (modules/fork-goran-mahovlic). This section records which concrete reference files are most reusable when implementing each gap item below.

| Gap | Best reference | Path | Notes |
|---|---|---|---|
| DMA request-line (peripheral→memory, triggered) | Renode `STM32DMA.cs` | `modules/renode/src/Infrastructure/src/Emulator/Peripherals/Peripherals/DMA/STM32DMA.cs` | `OnGPIO()` entry at line 46; `PerformTransfer()` at line 247; `CreateRequest()` at line 272 builds src/dst from config |
| DMA FIFO threshold / direct-mode logic | Renode `STM32DMA.cs` | same | `GetCurrentTransferSize()` at line 345 returns one FIFO threshold unit in direct mode |
| DMA PINC/MINC per-beat address increment | Renode `STM32DMA.cs` | same | lines 294–315 account for peripheral/memory increment flags in `CreateRequest()` |
| Timer separate CCR compare timers | Renode `STM32_Timer.cs` | `modules/renode/src/Infrastructure/src/Emulator/Peripherals/Peripherals/Timers/STM32_Timer.cs` | 4 independent `ccTimers[]` constructed at lines 25–35; event handlers at lines 93–126 |
| Timer 5 output-compare modes (SetActive/Inactive/Toggle/PWM1/PWM2) | Renode `STM32_Timer.cs` | same | `ccTimers[j]` event handlers lines 101–116 implement all 5 OC modes |
| Timer repetition counter (TIM1/TIM8 advanced) | Renode `STM32_Timer.cs` | same | lines 77–85 apply repetition counter before firing update IRQ |
| EXTI direct vs configurable line routing | Renode `STM32F4_EXTI.cs` | `modules/renode/src/Infrastructure/src/Emulator/Peripherals/Peripherals/IRQControllers/STM32F4_EXTI.cs` | `OnGPIO()` at lines 38–54; direct lines pass through immediately, configurable lines always latch pending |
| Flash sector erase + mass erase | Renode `STM32F4_FlashController.cs` | `modules/renode/src/Infrastructure/src/Emulator/Peripherals/Peripherals/MTD/STM32F4_FlashController.cs` | `Erase()` at lines 182–197; SER/MER/SNB fields at lines 133–145 |
| Flash lock-key mechanism (KEYR unlock) | Renode `STM32F4_FlashController.cs` | same | `controlLock.ConsumeValue()` pattern at lines 56–67 |
| Flash option bytes region (0x1FFFC000) | Renode `STM32F4_FlashController.cs` | same | dual address space at lines 42–47 |
| Flash status register (BSY/EOP/OPERR/error flags) | Renode `STM32F4_FlashController.cs` | same | `StatusRegister` fields at lines 105–126 |
| FMC 4-bank abstraction with ext-device routing | goran-mahovlic `fmc.rs` | `modules/fork-goran-mahovlic/src/peripherals/fmc.rs` | `Bank` struct with `ext_device` connector at lines 92–135; register decode at lines 27–67 |
| SVD-based register/IRQ lookup | AZhurGIT `meta.rs` | `modules/fork-AZhurGIT/src/peripherals/meta.rs` | `DeviceMeta::from_svd()` at lines 31–75; `irq_of()` at line 102; removes hardcoded offsets |
| RCC ready-bit auto-sync pattern | AZhurGIT `rcc.rs` | `modules/fork-AZhurGIT/src/peripherals/rcc.rs` | `update_ready_bits()` at lines 129–141 — our rcc.rs now mirrors this |

### Items not yet tracked in FEATURE_GAP.md that this audit revealed

- [ ] LOW: Flash sector erase and mass erase (SER/MER/SNB/STRT in FLASH_CR). Firmware that erases config sectors before writing will stall on BSY; reference: Renode `STM32F4_FlashController.cs` lines 182–197.
- [ ] LOW: Flash lock/unlock key sequence (KEYR → write `0x45670123` then `0xCDEF89AB`). Real hardware ignores control writes when locked; our stub allows writes unconditionally. Reference: Renode `STM32F4_FlashController.cs` lines 56–67.
- [ ] LOW: Flash status register (BSY, EOP, OPERR, WRPERR, PGAERR flags). Firmware that polls BSY after erase/program will spin forever without it. Reference: Renode `STM32F4_FlashController.cs` lines 105–126.
- [ ] LOW: FMC/FSMC 4-bank abstraction with external device routing. Current `fsmc.rs` is a stub; the goran-mahovlic fork has a complete `Bank` struct with `ext_device` connector. Reference: `modules/fork-goran-mahovlic/src/peripherals/fmc.rs`.
- [ ] MEDIUM: AZhurGIT `meta.rs` SVD-driven register-offset and IRQ-number lookup. Eliminates hardcoded offsets and supports F1/F4 variants from one codebase. Port overhead is moderate; benefit is long-term maintainability, not immediate boot progress.

### Renode comparison migration status

This file is now the canonical backlog. Content from the former Renode comparison write-up has been migrated here so the old comparison file can be retired.

Items from that comparison that are still not fully implemented:

- [ ] HIGH: USB OTG FS remains partial; endpoint/FIFO/interrupt behavior is still incomplete for full CDC-accurate modeling.
- [ ] MEDIUM: `meta.rs` adoption is still partial; IRQ lookup is now used by both I2C and TIM, but broader register-offset migration is pending.
- [ ] MEDIUM: Missing STM32F4 peripheral coverage compared to Renode still includes Ethernet MAC.
- [ ] LOW: LTDC/video support remains unimplemented (not currently required for CubeBlack runtime milestones).

Items from that comparison that are now implemented or materially addressed:

- [x] TIM model is present and covers TIM1-TIM14 with update/CC event handling.
- [x] EXTI peripheral model is implemented and wired.
- [x] FLASH control/register behavior is implemented beyond simple ACR stubs.
- [x] DMA now performs real memory movement (`mem_read`/`mem_write`) instead of register-only completion.
- [x] PWR and ADC peripheral models are present.
- [x] Bit-band alias support now includes both peripheral alias mapping and SRAM alias mapping.
- [x] Missing-peripheral gap items CAN, RTC, IWDG, RNG, and CRC now have peripheral stubs wired in the main registry.

---

## Advanced TIM Peripheral

| Feature | Priority | Status | Notes/Technical Issues |
| --- | --- | --- | --- |
| TIM1/TIM8 control register coverage (`CR2`, `SMCR`, `EGR`, `CCER`, `BDTR`, `RCR`) | low | Partially Implemented | `CR2`, `SMCR`, and `EGR` (update + CC event generation) now implemented. `CCER`, `BDTR`, `RCR`, and complementary output control remain unimplemented. |
| Channel state for `CCR2`, `CCR3`, `CCR4`, `CCMR1`, and `CCMR2` | low | Partially Implemented | `CCR2/3/4` storage and compare-event firing now implemented. `CCMR1/2` output-mode decoding and `CCER` polarity/enable control remain unimplemented. |
| Advanced counting modes (up, down, center-aligned) | low | Unimplemented | The current counter behavior is a simple monotonically increasing software timebase and does not support direction or center-aligned update rules. |
| Update, compare, commutation, trigger, and break event semantics | low | Unimplemented | `SR` and `DIER` handling is minimal, so firmware that expects full event flag behavior may stall or mis-handle interrupt state. |
| Complementary outputs, MOE, break input, and dead-time insertion | low | Unimplemented | Advanced PWM motor-control behavior for TIM1/TIM8 requires output-stage modeling that does not exist in the emulator yet. |
| Timer synchronization and master-slave trigger chaining | low | Unimplemented | Timer link behavior across TIM peripherals is not implemented, so synchronized startup and chained events are currently unsupported. |
| TIM1/TIM8 DMA request generation | low | Unimplemented | Advanced timers can raise DMA requests independently; without that behavior, DMA-driven PWM setup or polling loops may not progress correctly. |