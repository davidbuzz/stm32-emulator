# TIM5 Dual-Mode Operation

## Key Finding
CubeBlack uses TIM5 in **two complementary modes simultaneously**:
1. **Free-running 32-bit counter** (`ARR=0xFFFFFFFF`, `DIER.UIE=0`, read via CNT)
2. **CC1 compare interrupt** (`DIER.CC1IE=bit1`, triggers IRQ 50 when CNT==CCR[0])

This is standard ChibiOS ST driver pattern (`OSAL_ST_MODE_FREERUNNING`).

## Hardware Configuration
- **Prescaler**: PSC depends on ST_CLOCK_SRC and OSAL_ST_FREQUENCY
- **ARR**: 0xFFFFFFFF (32-bit wrap)
- **CR1.CEN**: Always 1 (counter enabled)
- **DIER**: Toggle bit 1 only (0x00000000 or 0x00000002)

## ChibiOS APIs
- `osalOsGetCounter()` → `st_lld_get_counter()` → reads `TIM5->CNT`
- `osalOsStartAlarm(abstime)` → `st_lld_start_alarm()` → writes `CCR[0]=abstime; DIER|=CC1IE`
- `osalOsStopAlarm()` → `st_lld_stop_alarm()` → `DIER&=~CC1IE`

## Source
`modules/ardupilot/modules/ChibiOS/os/hal/ports/STM32/LLD/SYSTICKv1/hal_st_lld.h` lines 818-841

## Emulator Notes
- Free-running counter must increment each cycle (or at PS rate)
- CC1 interrupt fires when CNT==CCR[0] AND DIER.CC1IE=1
- Current emulator correctly models this behavior
- Debug output confirms ARR, CCR, DIER transitions match expected pattern
