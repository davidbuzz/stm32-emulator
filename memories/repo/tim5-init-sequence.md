# TIM5 ChibiOS Initialization Sequence (STM32F427)

## Source References
- ChibiOS Low-Level Driver: `./modules/ardupilot/modules/ChibiOS/os/hal/ports/STM32/LLD/TIMv1/hal_gpt_lld.c`
- Timer Register Definitions: `./modules/ardupilot/modules/ChibiOS/os/hal/ports/STM32/LLD/TIMv1/stm32_tim.h`

## Configuration Structure
```c
typedef struct {
  gptfreq_t       frequency;      // Timer clock frequency (e.g., 1000000 Hz = 1 MHz for 1kHz ticks)
  gptcallback_t   callback;       // Callback pointer (can be NULL)
  uint32_t        cr2;            // CR2 register initialization (typically 0)
  uint32_t        dier;           // DIER register DMA-related bits (typically 0)
} GPTConfig;
```

## Exact Register Write Sequence

### Phase 1: `gpt_lld_start()` (Clock enable and peripheral setup)
Called once when the timer driver is initialized.

**Register operations in order:**
1. **Clock enable** via RCC (rccEnableTIM5)
2. **Clock reset** via RCC (rccResetTIM5)
3. **NVIC interrupt enable** for TIM5 IRQ 50 at configured priority
4. **Calculate prescaler:**
   - `psc = (gptp->clock / gptp->config->frequency) - 1`
   - Example: if clock = 84 MHz and frequency = 1 MHz, then PSC = 83
5. **Write CR1 = 0x0000** (initially stopped)
6. **Write CR2** = config value (typically 0x0000)
7. **Write PSC** = prescaler value
8. **Write SR = 0x0000** (clear all pending flags)
9. **Write DIER** = config->dier & ~STM32_TIM_DIER_IRQ_MASK (only DMA bits, mask off interrupt bits)

### Phase 2: `gpt_lld_start_timer()` (Interval setup and timer start)
Called when starting the timer with a specific interval.

**Register operations in order:**
1. **Write ARR** = (interval - 1)
   - For 1 kHz timer with 1 MHz prescaled clock: ARR = 999
   - ARR is loaded into the auto-reload shadow register (preload enabled)
2. **Write EGR** = STM32_TIM_EGR_UG (0x00000001 = Update Generation)
   - This triggers an update event to:
     - Transfer PSC shadow register → PSC active register
     - Transfer ARR shadow register → ARR active register
     - Reset CNT to 0
3. **Write CNT = 0x0000** (reset counter, done after EGR for timing reasons)
4. **Comment notes:** "After generating the UG event it takes several clock cycles before SR bit 0 goes to 1. This is why the clearing of CNT has been inserted before the clearing of SR, to give it some time."
5. **Write SR = 0x0000** (clear all flags again after UG)
6. **If callback is registered:**
   - **Write DIER |= STM32_TIM_DIER_UIE** (enable Update Interrupt)
7. **Write CR1** = STM32_TIM_CR1_ARPE | STM32_TIM_CR1_URS | STM32_TIM_CR1_CEN
   - **ARPE (bit 7 = 1):** Auto-reload preload enabled
   - **URS (bit 2 = 1):** Update Request Source - only counter overflow/underflow generates update interrupt
   - **CEN (bit 0 = 1):** Counter enable - timer starts counting

## Key Behaviors

### Auto-Reload Preload (ARPE = 1)
- PSC and ARR values are written to shadow registers first
- EGR update event transfers shadow → active registers
- Prevents glitches when ARR/PSC are updated during operation

### No CCER Writes for GPT Mode
- CCER (Capture/Compare Enable Reg) is NOT written by the GPT driver
- TIM5 in GPT (general-purpose timer) mode doesn't use capture/compare
- Compare channels remain disabled

### Update Generation (EGR) Timing
- Immediately transfers PSC_shadow → PSC_active and ARR_shadow → ARR_active
- Generates UIF flag (but must wait several cycles before SR reflects it)
- Used to synchronize shadow registers to active registers

### Interrupt Setup Timing
- NVIC vector enabled BEFORE any register writes
- DIER interrupt bits set AFTER timer is configured but BEFORE CEN=1
- UIE (Update Interrupt Enable) written only if callback is defined
- All flag-clearing IRQs masked initially, only UIE enabled if needed

## CCRx and CCER Handling
- **NOT written by GPT driver for basic interval timing**
- Compare channels remain unconfigured and disabled
- CCER register untouched by gpt_lld_start or gpt_lld_start_timer

## Register Width Consideration
- **TIM5 is 32-bit on STM32F427:**
  - CNT, ARR, PSC all support 32-bit values
  - No masking to 16-bit (unlike TIM3, TIM4, TIM6-TIM14)
  - Allows longer reload periods and prescaler values

## After Timer Starts (CR1.CEN = 1)
1. Timer begins counting from 0 with clock source = fCK / (PSC + 1)
2. When CNT reaches ARR, the following happen on same cycle:
   - CNT is reset to 0
   - UIF flag is set in SR (if URS=0 would also update shadow registers)
   - Update interrupt pending if UIE=1
3. ISR handler calls user callback and clears UIF
4. Cycle repeats

## Example Concrete Sequence for 1 kHz Timer at 84 MHz APB1 Clock
```
APB1 clock frequency (TIM5):  84 MHz
Desired frequency:             1 MHz (1 kHz interrupt)
Desired period:                1 millisecond

PSC calculation:  psc = (84000000 / 1000000) - 1 = 83
ARR calculation:  arr = (1000000 / 1000) - 1 = 999  (1ms period)

Register writes:
1. RCC: Enable TIM5 clock
2. RCC: Reset TIM5
3. NVIC: Enable IRQ 50 at priority level
4. CR1 = 0x0000
5. CR2 = 0x0000 (config->cr2, default 0)
6. PSC = 0x00000053 (83 decimal)
7. SR = 0x0000
8. DIER = 0x0000 & ~0x00FF = 0x0000 (DMA bits only, no interrupts yet)

Then gpt_lld_start_timer:
9. ARR = 0x000003E7 (999 decimal)
10. EGR = 0x00000001 (UG bit set)
11. CNT = 0x00000000
12. SR = 0x0000 (clear flags after UG)
13. DIER |= 0x0001 (set UIE if callback defined)
14. CR1 = 0x00000087 (CEN|URS|ARPE = bits 0,2,7)

Then timer counts 0→999 over 84000 clock cycles (1ms at 84MHz).
Every 1ms: CNT resets, UIF flag set, interrupt fires if UIE=1
```

## Emulator Validation Checklist
- [ ] CR1 starts at 0x0000, not some partial value
- [ ] PSC written before ARR in init phase
- [ ] ARR written in start_timer phase, not init phase
- [ ] EGR written immediately after ARR with UG bit set
- [ ] CNT reset after EGR written
- [ ] SR cleared after CNT reset (to allow time for UIF to propagate)
- [ ] CR1 final value written with ARPE|URS|CEN (0x87)
- [ ] DIER UIE bit only set if callback != NULL
- [ ] CCER never written for basic GPT operation
- [ ] CCRx values never written for basic GPT operation
- [ ] Timer interrupt dispatch happens on UIF flag if UIE=1

## ArduPilot Use
- CubeBlack uses TIM5 for the scheduler (1 kHz interrupt)
- Configured frequency likely 1 MHz (for 1 kHz timer interrupt)
- Callback installed for scheduler tick
- No capture/compare used
- Expected PSC = 83 (for 84 MHz APB1 at CubeBlack)
