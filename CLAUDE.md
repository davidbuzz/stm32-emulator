# CLAUDE.md

Project guidance for Claude Code working in this repository.

## Project intent

- Primary objective: emulate STM32 boards well enough to boot and run unmodified firmware.
- Current focus board: CubeBlack (STM32F427) and ArduPilot binaries in [cubeblack/](cubeblack/).
- Behavioral correctness is higher priority than cycle accuracy.

## Current end goal

Get the CubeBlack target to boot and run ArduPilot firmware with no firmware-side bypasses required for normal startup. [FEATURE_GAP.md](FEATURE_GAP.md) is the canonical backlog — consult it first when choosing what to work on next.

## First steps on each task

1. Read the active board config before changing emulator behavior.
2. Reproduce current behavior with a bounded run before editing code.
3. Capture concrete evidence: instruction count reached, stop PC, and warning classes.

## Run commands

```bash
# Setup Ubuntu dev environment
./DEV_SETUP_UBUNTU.sh

# Run CubeBlack
cd cubeblack
../target/release/stm32-emulator config.yaml -v --max-instructions 3000000

# Detect tight loops
../target/release/stm32-emulator config.yaml -v --busy-loop-stop
```

## Boot-debug workflow

1. Reproduce with `--max-instructions` and record logs.
2. Classify the blocker: unmapped memory access, missing peripheral register semantics, missing interrupt behavior, or firmware waiting on status flags.
3. Fix one blocker at a time.
4. Re-run and compare logs against baseline.
5. Keep changes minimal and board-targeted when possible.

## Reference docs

- [cubeblack/STM32F4xx_Reference_Manual.md](cubeblack/STM32F4xx_Reference_Manual.md)
- [cubeblack/stm32f427vg-datasheet.md](cubeblack/stm32f427vg-datasheet.md)
- [cubeblack/STM32F4_DMA.md](cubeblack/STM32F4_DMA.md)

Use these as the source of truth for register behavior and memory layout.

## ArduPilot source

Full ArduPilot source is available as a git submodule at [modules/ardupilot/](modules/ardupilot/). ChibiOS lives at [modules/ardupilot/modules/ChibiOS/](modules/ardupilot/modules/ChibiOS/). Use it to cross-reference firmware behavior, ISR logic, and USB driver internals (`os/hal/ports/STM32/LLD/OTGv1/`).

### Rebuilding the CubeBlack binary

```bash
cd modules/ardupilot
./waf configure --board=CubeBlack --debug
./waf copter
cp build/CubeBlack/bin/* ../../cubeblack/
cd ../..
```

## Implementation guidance

- Prefer realistic register read/write behavior over bypass patches.
- For DMA on STM32F4: keep stream register decode aligned with offset `0x10 + n*0x18`; model LISR/HISR and LIFCR/HIFCR flag behavior; maintain EN and NDTR transitions expected by polling loops.
- Preserve existing public interfaces unless a task requires refactoring.
- Favor adding missing memory regions in board config over skipping instructions.
- For STM32F427, account for CCM RAM and system memory/UID areas.
- Keep firmware patch entries in `config.yaml` documented with why they exist.

## Validation after each change

Run a short bounded test for fast feedback, then a longer stability test to confirm no regression. Report:
- Command used
- Final instruction count or stop reason
- Whether warning classes improved, regressed, or stayed the same

## Editing guardrails

- Do not revert unrelated user changes.
- Avoid destructive git operations.
- Keep commits focused and small.
- Prefer targeted patches over broad cleanup.

## Todo list requirements

When maintaining a TODO list, always end it with:
- `git commit your changes`
- `refer to FEATURE_GAP.md afterwards to get more work to do`

## Done criteria for boot tasks

A boot-progress fix is considered good when:
- the targeted warning/error class is removed or reduced,
- execution advances stably for a materially longer run window,
- and the change is traceable to documented STM32F4 behavior or board config requirements.

## Concrete success marker

The CubeBlack/ArduPilot deliverable is reached only when all of the following are true:

- A bounded run to at least `100000000` instructions completes without `WARN` or `ERROR` lines and without `peri=????` unknown peripheral accesses.
- The early idle-loop probe at `0x08161148` is no longer terminal: interrupt tracing shows at least one wake event/return sequence (TIM5 IRQ 50 pending, dispatch, return).
- Firmware emits at least one recognizable runtime console line through USART probes, or an equivalent deterministic runtime milestone is documented in `FEATURE_GAP.md`.
- The validation report records the exact command(s), stop reason or final instruction count, and why the observed marker demonstrates boot/runtime progress.
