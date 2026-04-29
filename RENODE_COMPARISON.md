# Comparative Analysis: Renode vs stm32-emulator & Fork Survey

Performed April 2026 — shallow clones under `modules/` (gitignored, read-only reference).

---

## 1. Repos surveyed

| Clone path | Upstream | Purpose |
|---|---|---|
| `modules/renode` | github.com/renode/renode | Primary: Antmicro's production emulator |
| `modules/renode/src/Infrastructure` | github.com/renode/renode-infrastructure | C# peripheral implementations |
| `modules/stm32-emulator-renode` | github.com/Gissio/stm32-emulator-renode | nviennot-inspired Renode extension for consumer STM32F1 gadgets |
| `modules/stm32-renode-inpyjama` | github.com/inpyjama/stm32-renode | Tutorial skeleton: Renode + STM32F407 Discovery |
| `modules/fork-goran-mahovlic` | github.com/goran-mahovlic/stm32-emulator | nviennot fork — 1 trivial config commit |
| `modules/fork-AZhurGIT` | github.com/AZhurGIT/stm32-emulator | nviennot fork — significant peripheral additions |
| `modules/fork-wawa19933` | github.com/wawa19933/stm32-emulator | nviennot fork — nix flake only |
| `modules/fork-SINTEF` | github.com/SINTEF-Infosec/stm32-emulator | nviennot fork — README update only |

---

## 2. Renode: architecture overview

- **Language**: C# / .NET (Mono)
- **CPU emulation**: own translation-block engine (not Unicorn); true Cortex-M4 FPU support
- **Platform description**: `.repl` declarative scripts that wire peripherals to a sysbus
- **Run scripts**: `.resc` scripts for loading firmware, wiring devices, starting simulation
- **Licence**: MIT (infrastructure submodule also MIT)
- **Maturity**: production-grade, multi-year commercially supported project
- **SVD**: applied at runtime via `ApplySVD @url` — gives register-level access without hand-coding every register

---

## 3. Renode STM32F4 peripheral inventory

Scraped from `platforms/cpus/stm32f4.repl` and `stm32f429.repl` (STM32F427 is register-compatible with F429).

| Category | Peripheral | In renode? | In our emulator? | Notes |
|---|---|---|---|---|
| Core | NVIC | ✅ full | ✅ (nvic.rs) | renode ties priorityMask, systickFrequency |
| Core | SysTick | ✅ (via NVIC) | ✅ (systick.rs) | |
| Core | SCB | ✅ (via CPU model) | ✅ (scb.rs) | |
| Core | Bit-banding (periph+SRAM) | ✅ dedicated model | ❌ not mapped | firmware uses alias addresses |
| Memory | SRAM 256KB | ✅ | ✅ (config.yaml) | |
| Memory | Flash 2MB | ✅ | ✅ (config.yaml) | |
| Memory | ROM1/ROM2 (bootloader) | ✅ mapped | ✅ (config.yaml) | |
| Memory | CCM RAM | not in renode base | ✅ (config.yaml) | |
| Memory | FSMC/FMC bank | ✅ fsmcBank1 | ✅ (fsmc.rs) | renode maps whole 256MB |
| Clock | RCC | ✅ STM32F4_RCC.cs (404L) | ✅ (rcc.rs) | renode links rtcPeripheral, Timer9Reset; richer model |
| Interrupt | EXTI | ✅ STM32F4_EXTI.cs | ❌ missing | renode fans out to nvic@6-10, 23, 40 |
| DMA | DMA1, DMA2 | ✅ STM32DMA.cs (425L) | ✅ (dma.rs) | **renode copies data via DmaEngine+sysbus; ours only handles registers** |
| GPIO | GPIOA–GPIOK | ✅ STM32_GPIOPort.cs (376L) | ✅ (gpio.rs) | renode models alternate-function pin connections to timers |
| Timer | TIM1–TIM14 | ✅ STM32_Timer.cs (658L) | ❌ missing | renode has PWM, OC, GPIO AF linkage; fires IRQs; we poll IRQ50 synthetically |
| Timer | RTC | ✅ STM32F4_RTC.cs | ❌ missing | |
| Timer | IWDG | ✅ STM32_IndependentWatchdog.cs | ❌ missing | firmware writes LSI-driven watchdog; not critical to boot |
| UART | USART1–3, UART4–5 | ✅ STM32_UART.cs (295L) | ✅ (usart.rs) | ours has TX-probe output |
| SPI | SPI1–3 | ✅ STM32SPI.cs (356L) | ✅ (spi.rs) | |
| I2C | I2C1–3 | ✅ STM32F1_I2C.cs | ✅ (i2c.rs) | |
| CAN | CAN1–2 | ✅ STMCAN | ❌ missing | |
| ADC | ADC1–3 | ✅ STM32_ADC.cs | ❌ missing | |
| Flash | Flash controller | ✅ STM32F4_FlashController.cs | ❌ missing | ACR latency bits read by startup code |
| USB | OTG FS / HS | ❌ **stubbed** with fixed tag value | ❌ partially stubbed | **renode hits the same gap we do** |
| Ethernet | MAC (Synopsys) | ✅ SynopsysEthernetMAC | ❌ missing | not needed for ArduPilot CDC console |
| Video | LTDC | ✅ (F429 only) | ❌ | not needed |
| Crypto | RNG | ✅ STM32_RNG.cs | ❌ missing | |
| CRC | CRC | ✅ STM32_CRC.cs | ❌ missing | |
| Power | PWR | ✅ STM32_PWR.cs | ❌ missing | |

---

## 4. Key renode DMA findings (STM32DMA.cs)

Renode's DMA model is more complete than ours in one critical respect: it **actually performs the data copy** via `DmaEngine.IssueCopy(request)` using the sysbus. This means a peripheral DMA transfer genuinely moves bytes in the simulated address space.

Our emulator's `dma.rs` models the register interface and fires the completion IRQ but does **not** copy data. For peripherals that rely on DMA to deliver received data into RAM (e.g., SPI-RX, UART-RX with DMA), our model would not deliver the data — firmware would read stale zero-filled memory.

Important detail: renode issues the transfer synchronously on `EN=1` for MemoryToMemory and MemoryToPeripheral, and via GPIO signal (peripheral triggers) for PeripheralToMemory. NDTR decrements to 0, TCIF is set, IRQ fires.

**Actionable for us**: add actual sysbus memory copy in our DMA completion path.

---

## 5. Key renode Timer findings (STM32_Timer.cs, 658 lines)

Renode's timer model is substantial:
- CNT counts at `frequency` Hz (configurable), wraps at ARR
- Update IRQ fires when CNT wraps
- 4 output-compare channels with alternate-function GPIO pin connections
- PWM output to GPIO
- PSC (prescaler) logic
- TIM1/TIM8 break/commutation interrupts

Our emulator has no TIM1–TIM14 model. We synthesise IRQ50 (TIM5) directly in NVIC. This works for the scheduler tick but breaks any code that reads TIM5 status registers (SR, CNT, ARR, DIER).

**Actionable**: add a `tim.rs` peripheral — see AZhurGIT fork for a directly reusable Rust base.

---

## 6. USB/OTG gap — renode is in the same position as us

Renode's STM32F4 USB handling:
```
sysbus:
  init:
    Tag <0x50000010, 0x5000003f> "USB:RESET" 0x80000000
```
This applies a fixed-value read tag to USB_OTG_FS GINTSTS. The comment says: *"FLASH and USB tags are required for CubeMX-based projects to pass the initialization phase."*

There is **no STM32 OTG USB implementation** anywhere in renode's STM32F4 peripheral set. The same peripheral class gap exists in renode that we're working around with our partial OTG stub.

This means switching to renode would not help with the USB CDC console milestone.

---

## 7. Fork analysis

### fork-AZhurGIT ⭐⭐⭐ (high value)

Single big PR adding:

| New file | What it adds |
|---|---|
| `exti.rs` | Full EXTI controller: IMR/EMR/RTSR/FTSR/SWIER/PR registers, handles 7 IRQ fanout groups (EXTI0–4, EXTI9_5, EXTI15_10). Triggers NVIC interrupts on rising/falling edge config. Handles SWIER software triggers. |
| `flash.rs` | Flash ACR register with PRFTBE/PRFTBS prefetch status bit, plus KEYR/OPTKEYR/SR/CR/AR. Handles firmware ACR polls after clock switch. |
| `tim.rs` | Timer CR1/CR2/SMCR/DIER/SR/EGR/CNT/PSC/ARR. TIM4 update interrupt fires through NVIC. EGR=1 forces UIF+IRQ immediately. |
| `meta.rs` | Parses SVD at runtime, builds `DeviceMeta` with `offset_of("REG")` lookups and `irq_of("IRQNAME")`. Eliminates hardcoded register offsets. Multi-platform: `PlatformFamily::Stm32F1` or `Stm32F4`. |

Architecture compatibility: same `nviennot/stm32-emulator` base as us. The `Peripheral` trait, `System`, `Peripherals` struct are identical. Direct porting is straightforward — the main adaptation is replacing `meta.rs` lookups with hardcoded STM32F4 offsets (or adopting `meta.rs` wholesale).

The `meta.rs` SVD-driven approach is architecturally superior to hardcoding. It is worth adopting: it would let our peripheral code tolerate SVD-driven offset changes and work across F1/F4 without code forks.

### fork-goran-mahovlic (low value)

One commit. Adds `fmc.rs` — a more complete Flexible Memory Controller with 4 banks, BCR/BTR/BWTR registers, PCR/SR/PMEM/PATT NAND registers, and per-bank ECCR. Better than our `fsmc.rs` for bus-width and timing control. Low priority for CubeBlack boot.

### fork-wawa19933 / fork-SINTEF (no value)

Dev environment / documentation only. No peripheral additions.

---

## 8. Gissio/stm32-emulator-renode

Targets STM32F1-based **consumer devices** (FNIRSI oscilloscopes, GQ Geiger counter). Adds C# Renode extensions:
- `STM32F1_ADC.cs`, `STM32F1_GPIO.cs`, `STM32F1_RCC.cs`, `STM32F1_RTC.cs`, `STM32F1_SPI.cs`, `STM32F1_TIM.cs`
- `ST7789.cs` — LCD display driver over SPI
- `.repl` + `.resc` per device

**Not relevant to STM32F4/CubeBlack** — all STM32F1 specific. But it demonstrates the pattern of extending Renode with custom C# peripheral classes loaded at runtime from a `.resc` script, which is Renode's standard extension mechanism.

---

## 9. inpyjama/stm32-renode

Basic STM32F407 Discovery "hello blink" under Renode. Tutorial-level, nothing novel. Not actionable.

---

## 10. Should we switch to Renode?

**No, not for the current CubeBlack/ArduPilot goal.** Reasons:

1. **Same USB gap**: Renode stubs USB OTG FS with a fixed tag. Switching would not bring us closer to the USB CDC console milestone.
2. **Porting cost**: The CubeBlack board config (config.yaml with regions, patches, UID data, SPI-flash, probe USART) would all need to be rewritten as a `.repl` + `.resc` pair.
3. **IRQ timing control**: The deferred-IRQ-delivery fix we just landed (`da7d774`) is straightforward in our Unicorn-loop model. Doing equivalent work in Renode's framework would be opaque and would require C# changes.
4. **Debug fidelity**: Our instruction-level hooks and PC-targeted patches are native to the Unicorn loop. Renode operates at a higher level.

**Yes, use renode as a reference blueprint:**
- Register behavior documentation is baked into the C# implementations.
- The STM32F4_RCC, STM32_Timer, STM32F4_EXTI, STM32F4_FlashController implementations are tested, production-quality references.
- Read the C# code when implementing a new peripheral to get the correct reset values, field widths, and state machine behavior.

---

## 11. Priority action items from this survey

Ranked by impact on the CubeBlack/ArduPilot boot goal:

| # | Item | Source for reference |
|---|---|---|
| 1 | **Port `tim.rs`** — TIM5 register model so firmware TIM5 CR1/DIER/SR/CNT/PSC/ARR reads/writes work correctly | AZhurGIT `tim.rs`, renode `STM32_Timer.cs` |
| 2 | **Port `exti.rs`** — EXTI controller for GPIO-based external interrupts (IMU DRDY, etc.) | AZhurGIT `exti.rs`, renode `STM32F4_EXTI.cs` |
| 3 | **Port `flash.rs`** — Flash ACR latency register so startup code ACR polls resolve | AZhurGIT `flash.rs`, renode `STM32F4_FlashController.cs` |
| 4 | **DMA data copy** — Actual sysbus memory copy in DMA completion path | renode `STM32DMA.cs` → `PerformTransfer()` + `DmaEngine.IssueCopy()` |
| 5 | **USB OTG FS** — Full OTG FS register model for CDC ACM console | ChibiOS `os/hal/ports/STM32/LLD/OTGv1/` as firmware reference |
| 6 | **`meta.rs` SVD approach** — Replace hardcoded register offsets with runtime SVD lookup | AZhurGIT `meta.rs` |
| 7 | **Bit-banding** — Map peripheral and SRAM bit-band alias regions | renode `BitBanding` model as reference |

Items 1–3 are self-contained Rust ports with direct AZhurGIT reference code; they can be done incrementally without risk to existing peripherals. Item 4 (DMA copy) requires care around bus arbitration but would fix a silent correctness gap. Item 5 (USB OTG) remains the hardest and highest-value remaining piece.
