# CubeBlack TIM5 Dual Timer Modes Analysis

## Summary

**YES**, ArduPilot/CubeBlack uses **TWO distinct TIM5 operating modes**:

1. **Free-running counter mode** – TIM5 continuously counts from 0 to 0xFFFFFFFF, providing microsecond-resolution time reference
2. **Compare-based alarm mode** – CC1 (Compare Channel 1) interrupt triggers scheduler wakeups at precise intervals

These are not separate timers, but rather two complementary functions of the **same TIM5 instance**, orchestrated by the ChibiOS ST (System Timer) driver.

---

## ChibiOS ST Driver Configuration (CubeBlack)

### Board Config Definition
**File:** `cubeblack/hwdef.h:65`
```
#define STM32_ST_USE_TIMER 5
```

This configures CubeBlack to use TIM5 for the ChibiOS system timer in **free-running mode** (`OSAL_ST_MODE_FREERUNNING`).

---

## TIM5 Free-Running Counter Initialization

**Source:** `modules/ardupilot/modules/ChibiOS/os/hal/ports/STM32/LLD/SYSTICKv1/hal_st_lld.c:550-576`

```c
void st_lld_init(void) {
  // Initialization for free-running counter mode:
  STM32_ST_TIM->PSC    = (ST_CLOCK_SRC / OSAL_ST_FREQUENCY) - 1;
  STM32_ST_TIM->ARR    = ST_ARR_INIT;     // 0xFFFFFFFF for 32-bit counter
  STM32_ST_TIM->CCMR1  = 0;
  STM32_ST_TIM->CCR[0] = 0;               // Initialize compare register
  STM32_ST_TIM->DIER   = 0;               // Start with NO interrupts enabled
  STM32_ST_TIM->CR2    = 0;
  STM32_ST_TIM->EGR    = TIM_EGR_UG;      // Trigger update event
  STM32_ST_TIM->CR1    = TIM_CR1_CEN;     // CEN = Counter enable
}
```

**Boot sequence from `ardu.cubeblack.log` (lines 37-43):**
```
[clk=00108688] DEBUG TIM5 write PSC=0x00000053
[clk=00108690] DEBUG TIM5 write ARR=0xffffffff    ← 32-bit free-running counter!
[clk=00108693] DEBUG TIM5 write CCR1=0x00000000   ← Compare register
[clk=00108694] DEBUG TIM5 write DIER=0x00000000   ← Interrupts disabled initially
[clk=00108698] DEBUG TIM5 write CR1=0x00000001    ← Timer enabled
```

---

## Alarm/Compare Mode: CC1 Interrupt

When the ArduPilot scheduler needs to set a wakeup timeout, it calls ChibiOS's `stStartAlarm(systime_t abstime)` API, which activates the CC1 (Compare Channel 1) interrupt.

### stStartAlarm() Implementation

**Source:** `modules/ardupilot/modules/ChibiOS/os/hal/ports/STM32/LLD/SYSTICKv1/hal_st_lld.h:818-828`

```c
static inline void st_lld_start_alarm(systime_t abstime) {
  STM32_ST_TIM->CCR[0] = (uint32_t)abstime;  // Set compare target
  STM32_ST_TIM->SR     = 0;                  // Clear status flags
  
  // Enable CC1 interrupt (bit 1 of DIER)
  #if ST_LLD_NUM_ALARMS == 1
    STM32_ST_TIM->DIER = STM32_TIM_DIER_CC1IE;  // 0x00000002
  #endif
}
```

**Boot sequence from `ardu.cubeblack.log` (lines 49-50, 52-53):**
```
[clk=00333307] DEBUG TIM5 write CCR1=0x0000104d  ← Set alarm target count
[clk=00333311] DEBUG TIM5 write DIER=0x00000002  ← Enable CC1IE (bit 1)
[clk=00459230] DEBUG TIM5 CC1 compare fired cnt=0x0000104d ccr1=0x0000104d -> IRQ 50
[clk=00459230] DEBUG NVIC dispatching IRQ 50 vector=0x815c389

(later, alarm is cleared:)
[clk=00459277] DEBUG TIM5 write DIER=0x00000000  ← Disable CC1IE
```

---

## DIER Register Bits

| Bit | Name | Meaning |
|-----|------|---------|
| 0   | UIE  | Update Interrupt Enable (overflow, not used in CubeBlack) |
| 1   | CC1IE | **Compare 1 Match Interrupt Enable** ← Used for scheduler alarms |
| 2   | CC2IE | Compare 2 Match Interrupt Enable |
| 3   | CC3IE | Compare 3 Match Interrupt Enable |
| 4   | CC4IE | Compare 4 Match Interrupt Enable |

**DIER=0x00000000** → No interrupts (free-running counter only)
**DIER=0x00000002** → CC1IE enabled (alarm active)

---

## Interrupt Handler

When a CC1 match occurs, the ChibiOS system timer ISR fires:

**Source:** `modules/ardupilot/modules/ChibiOS/os/hal/ports/STM32/LLD/SYSTICKv1/hal_st_lld.c:617-645`

```c
void st_lld_serve_interrupt(void) {
  uint32_t sr = STM32_ST_TIM->SR;
  sr &= STM32_ST_TIM->DIER & STM32_TIM_DIER_IRQ_MASK;
  STM32_ST_TIM->SR = ~sr;  // Acknowledge interrupt

  if ((sr & TIM_SR_CC1IF) != 0U) {        // Check if CC1 match occurred
    osalSysLockFromISR();
    osalOsTimerHandlerI();                 // Call ChibiOS scheduler
    osalSysUnlockFromISR();
  }
}
```

---

## Summary: Dual-Mode Operation

| Mode | Register | Behavior | API |
|------|----------|----------|-----|
| **Free-Running Counter** | CNT (read-only) | Continuously counts 0 → 0xFFFFFFFF → 0 | `stGetCounter()` → reads TIM5->CNT |
| **Compare Alarm** | CCR[0] + DIER.CC1IE | Triggers IRQ when CNT == CCR[0] | `stStartAlarm(target_time)` → set CCR[0], enable CC1IE |

### API Calls in ArduPilot
- **`osalOsGetCounter()`** → reads free-running counter for timing
- **`osalOsStartAlarm(abstime)`** → schedules next wakeup using CC1 compare interrupt

---

## Hardware Reference

**STM32F427 TIM5 Base:** `0x40000C00`

**Registers:**
- `0x00` – CR1 (Control Register 1) – bit 0 = CEN (counter enable)
- `0x08` – DIER (Interrupt Enable Register) – bit 1 = CC1IE
- `0x0C` – SR (Status Register) – bit 1 = CC1IF (captures when CNT == CCR1)
- `0x14` – CNT (Counter Value)
- `0x18` – PSC (Prescaler)
- `0x1C` – ARR (Auto-Reload Register)
- `0x34` – CCR1 (Capture/Compare Register 1)

**Reference Manuals:**
- [cubeblack/STM32F4xx_Reference_Manual.md](cubeblack/STM32F4xx_Reference_Manual.md) – Section 18 (General-purpose timers TIM2-TIM5)
- [cubeblack/stm32f427vg-datasheet.md](cubeblack/stm32f427vg-datasheet.md) – Timer specifications

---

## Emulator Implementation Implications

The emulator must correctly model:

1. **Free-running counter** – CNT increments each cycle (or at prescaler rate), wraps at ARR
2. **CC1 conflict detection** – IRQ 50 fires when CNT == CCR[0] and both CC1IE bit is set in DIER
3. **DIER bit toggling** – CC1IE (bit 1) is the only bit toggled during normal operation
4. **ISR handler** – Must route to the TIM5_CC interrupt vector (address 0x815c389 in this binary)

The current emulator trace shows correct CC1 compare behavior, confirming the dual-mode understanding is correct.

---

## Verification

The dual-mode hypothesis is confirmed by:
- ✅ Initialization with `ARR=0xFFFFFFFF` (free-running)
- ✅ Early `DIER=0x00000000` (no interrupts during boot)
- ✅ Later `DIER=0x00000002` (CC1IE enabled for first alarm)
- ✅ `[clk=00459230] CC1 compare fired` when `CNT==CCR1`
- ✅ ChibiOS ST (SYSTICKv1) source code confirms this pattern
- ✅ hwdef.h confirms `STM32_ST_USE_TIMER 5`

