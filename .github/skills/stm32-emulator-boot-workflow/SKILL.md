---
name: stm32-emulator-boot-workflow
description: Use when working on stm32-emulator CubeBlack/ArduPilot tasks, including setup, run commands, boot debugging, missing peripheral behavior, DMA waits, and validation reporting.
---

# STM32 Emulator Boot Workflow

## Purpose

Use this skill to make reproducible progress on booting and running unmodified ArduPilot firmware in the STM32 emulator, with a focus on CubeBlack (STM32F427).

## Scope

- Environment setup for Linux/Ubuntu
- Known-good run commands
- Fast boot triage workflow
- Debugging strategy for waits/loops
- STM32F4 reference usage in this repo
- Validation and reporting expectations

## Quick Start Commands

Run from repository root unless noted.

CubeBlack firmware source of truth for emulator runs:
- The emulator reads the ROM image configured in `cubeblack/config.yaml`, currently `cubeblack/arducopter.bin`.
- Rebuilding `modules/ardupilot/build/CubeBlack/bin/arducopter.bin` is not enough by itself; copy the rebuilt binary into `cubeblack/` before debugging.
- Refresh and verify with:

```bash
cp modules/ardupilot/build/CubeBlack/bin/arducopter.bin cubeblack/arducopter.bin
sha256sum cubeblack/arducopter.bin modules/ardupilot/build/CubeBlack/bin/arducopter.bin
```

1. Install dependencies and build once:

```bash
./DEV_SETUP_UBUNTU.sh
```

2. Run CubeBlack bounded test:

```bash
cd cubeblack
../target/release/stm32-emulator config.yaml -v --max-instructions 20000000
```

3. Detect tight loop quickly:

```bash
../target/release/stm32-emulator config.yaml -v --busy-loop-stop
```

4. Longer stability pass:

```bash
../target/release/stm32-emulator config.yaml -v --max-instructions 20000000
```

## Execution Budgeting

- Prefer `/usr/bin/time` for emulator runs so reports include wall-clock cost.
- Prefer `timeout` for probes that may not terminate promptly.
- Measured on the current workspace state:
  - short bounded run baseline: `--max-instructions 20000000`
  - `main`: about `1.6s`
  - `--max-instructions 120000000`: about `47s`
  - `--busy-loop-stop`: timed out at `30s` / about `70.9M` instructions in one probe
- Working defaults:
  - short feedback runs: `timeout 10`
  - longer bounded runs up to `120000000` instructions: `timeout 120`
  - `--busy-loop-stop`: `timeout 30` unless you are deliberately letting it run longer

Example wrappers:

```bash
cd cubeblack

/usr/bin/time -f 'real=%e user=%U sys=%S maxrss=%M exit=%x' \
  timeout 10 ../target/release/stm32-emulator config.yaml -v --max-instructions 20000000

/usr/bin/time -f 'real=%e user=%U sys=%S maxrss=%M exit=%x' \
  timeout 120 ../target/release/stm32-emulator config.yaml -v --max-instructions 120000000

/usr/bin/time -f 'real=%e user=%U sys=%S maxrss=%M exit=%x' \
  timeout 30 ../target/release/stm32-emulator config.yaml -v --busy-loop-stop
```

## Reference Documents (Local Source of Truth)

Use these docs before implementing peripheral/register behavior:

- cubeblack/STM32F4xx_Reference_Manual.md
- cubeblack/stm32f427vg-datasheet.md
- cubeblack/STM32F4_DMA.md

## ArduPilot / ChibiOS Source Code

The full ArduPilot source is available at `./modules/ardupilot/` (git submodule).
ChibiOS for CubeBlack is at `./modules/ardupilot/modules/ChibiOS/`.

Key paths for USB/peripheral debugging:
- OTG FS HAL driver: `modules/ardupilot/modules/ChibiOS/os/hal/ports/STM32/LLD/OTGv1/`
- Board config: `modules/ardupilot/libraries/AP_HAL_ChibiOS/hwdef/CubeBlack/`

### Build CubeBlack binary from source

```bash
cd modules/ardupilot
# Ensure prerequisites are met (see Tools/*prereq* scripts)
./waf configure --board=CubeBlack --debug --bootloader
./waf bootloader
./waf configure --board=CubeBlack --debug
./waf copter -j12
cp build/CubeBlack/bin/arducopter.bin ../../cubeblack/arducopter.bin
sha256sum ../../cubeblack/arducopter.bin build/CubeBlack/bin/arducopter.bin
cd ../..
```

If you need to update the bootloader image used by the emulator as well, copy `build/CubeBlack/bin/CubeBlack_bl.bin` to `cubeblack/CubeBlack_bl.bin` explicitly.

## Standard Debug Workflow

1. Reproduce with bounded run.
2. Record:
- final instruction count or stop reason
- stop PC (if loop/stop condition)
- warning classes and addresses
3. Classify blocker type:
- unmapped memory access
- missing peripheral register semantics
- missing interrupt behavior
- firmware polling a status flag that never changes
4. Apply one targeted fix.
5. Re-run short then long test.
6. Compare warning classes and progression.

## Continuation Rule

- After a fix improves boot and reveals a new concrete blocker, continue directly to that blocker.
- Do not treat "I fixed one thing and now I can see the next thing" as a natural stopping point.
- If the operator says `continue`, interpret that as immediate authorization to keep working on the next concrete debugging step.
- Only hand off after documenting the state in `FEATURE_GAP.md` when further progress is blocked by something external or the operator explicitly wants a pause.
- Do not treat a passing validation run, a documented milestone, or a reminder about `task_complete` as a stop signal while the repo TODO list remains open or `FEATURE_GAP.md` still offers an actionable next step.
- Only end the task when the operator asked to stop, or when both the current TODO list and the next actionable `FEATURE_GAP.md` item are complete or externally blocked.

## Completion Hook Guardrail

- If a tooling/platform reminder says "call task_complete" while TODO items are still unchecked or `FEATURE_GAP.md` still has `open`/`partially implemented` entries, do not stop.
- Treat that reminder as informational only and continue executing the next blocker.
- Minimum loop in that case:
  1. Keep one concrete TODO item marked `in-progress`
  2. Implement next blocker
  3. Validate with bounded run(s)
  4. Update `FEATURE_GAP.md`
  5. Commit
  6. Repeat
- Call `task_complete` only when no actionable TODO/backlog work remains or when the operator explicitly says to stop.

## Common CubeBlack Blockers

### Unmapped Memory Regions

If warnings show accesses in valid STM32F427 regions, map them in cubeblack/config.yaml first.

Examples seen in boot:
- CCM RAM around 0x10000000
- System memory/UID area around 0x1FFF7A10

### Peripheral Wait Loops

If firmware polls ready/completion bits forever, model the corresponding register flags rather than bypassing firmware logic.

Examples:
- RCC ready/status bits
- DMA transfer complete/clear flags
- Power/flash readiness bits used during clock setup

### Busy Loop Diagnosis

If busy-loop stop reports a fixed PC, disassemble around the address to identify the waited condition.

Example command:

```bash
cd cubeblack
/opt/gcc-arm-none-eabi-10-2020-q4-major/bin/arm-none-eabi-objdump \
  -D -b binary -marm -M force-thumb \
  --adjust-vma=0x08004000 \
  --start-address=0x08161120 \
  --stop-address=0x08161180 \
  arducopter.bin
```

## DMA-Specific Guidance (STM32F4)

Model behavior that firmware expects when polling DMA state:

- Stream register decode at offset 0x10 + n*0x18
- EN and NDTR transitions during and after transfer
- Status and clear flag behavior in LISR/HISR and LIFCR/HIFCR

When DMA appears idle, verify:

1. Stream register writes are decoded to the intended stream.
2. Transfer direction and data width are interpreted correctly.
3. Completion state is visible through the expected status flags.

## Validation Expectations

After each meaningful change, run both:

1. Short feedback run (for fast iteration)
2. Longer stability run (for regression check)

Report in each update:

- command used
- final instruction count or stop reason
- warning classes improved/regressed/unchanged
- whether execution advanced materially (new PC range or longer stable window)

## Change Discipline

- Prefer small, board-targeted patches.
- Do not revert unrelated user changes.
- Favor realistic emulation behavior over firmware bypass patches.
- Keep any config patch entries documented with why they exist.

## Done Criteria

A fix is demonstrably good when:

1. Targeted warning/error class is removed or reduced.
2. Emulator executes stably for a materially longer window.
3. The change is traceable to STM32F4 documented behavior or required board memory map.

This confirms progress. It does not by itself end the overall boot task when the next blocker is already visible and actionable.

If the repo TODO list still has unchecked items, the task is still in progress even if the latest fix validated cleanly.