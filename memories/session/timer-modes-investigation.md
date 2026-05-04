# Session: Timer Modes Investigation

## Finding Summary
**YES - Two timer modes confirmed:**
1. Free-running counter (0 → 0xFFFFFFFF)
2. Compare-based CC1 interrupt for scheduler alarms

## Key References
- CiBiOS ST driver: `modules/ardupilot/modules/ChibiOS/os/hal/ports/STM32/LLD/SYSTICKv1/hal_st_lld.h:818-828`
- `st_lld_start_alarm()` sets CCR[0] and enables DIER bit 1 (CC1IE = 0x00000002)
- hwdef.h:65 = `#define STM32_ST_USE_TIMER 5`
- Emulator correctly fires CC1 compare interrupt

## Boot Log Evidence
- `[clk=00108690] TIM5 write ARR=0xffffffff` ← 32-bit free running
- `[clk=00108694] TIM5 write DIER=0x00000000` ← Disabled initially
- `[clk=00333311] TIM5 write DIER=0x00000002` ← CC1IE enabled for alarm
- `[clk=00459230] TIM5 CC1 compare fired` ← Interrupt works

## Output Document
- [DUAL_TIMER_MODES_ANALYSIS.md](/home/buzz/stm32-emulator/DUAL_TIMER_MODES_ANALYSIS.md)
