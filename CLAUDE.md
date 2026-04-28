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

CubeBlack firmware source of truth for emulator runs:
- The emulator loads the image referenced by [cubeblack/config.yaml](cubeblack/config.yaml), which currently points at [cubeblack/arducopter.bin](cubeblack/arducopter.bin).
- A rebuild under [modules/ardupilot/build/CubeBlack/bin/](modules/ardupilot/build/CubeBlack/bin/) does not affect emulator runs until you copy the rebuilt file into [cubeblack/](cubeblack/).
- Refresh command:

```bash
cp modules/ardupilot/build/CubeBlack/bin/arducopter.bin cubeblack/arducopter.bin
sha256sum cubeblack/arducopter.bin modules/ardupilot/build/CubeBlack/bin/arducopter.bin
```

```bash
# Setup Ubuntu dev environment
./DEV_SETUP_UBUNTU.sh

# Run CubeBlack
cd cubeblack
../target/release/stm32-emulator config.yaml -v --max-instructions 3000000

# Detect tight loops
../target/release/stm32-emulator config.yaml -v --busy-loop-stop
```

## Execution Budgeting

- Wrap emulator runs with `/usr/bin/time` when validating so runtime cost is visible in the log.
- Wrap exploratory or potentially open-ended probes with `timeout`.
- Measured on the current workspace state:
  - `--max-instructions 3000000` takes about `1.3s`
  - `--max-instructions 120000000` takes about `47s`
  - `--busy-loop-stop` did not terminate quickly in one probe and was cut off at `30s` / about `70.9M` instructions
- Recommended defaults:
  - `timeout 30` for short runs and busy-loop probes
  - `timeout 120` for longer bounded runs up to `120000000` instructions

```bash
cd cubeblack

# Short feedback run
/usr/bin/time -f 'real=%e user=%U sys=%S maxrss=%M exit=%x' \
  timeout 30 ../target/release/stm32-emulator config.yaml -v --max-instructions 3000000

# Longer stability run
/usr/bin/time -f 'real=%e user=%U sys=%S maxrss=%M exit=%x' \
  timeout 120 ../target/release/stm32-emulator config.yaml -v --max-instructions 120000000

# Busy-loop probe
/usr/bin/time -f 'real=%e user=%U sys=%S maxrss=%M exit=%x' \
  timeout 30 ../target/release/stm32-emulator config.yaml -v --busy-loop-stop
```

## Boot-debug workflow

1. Reproduce with `--max-instructions` and record logs.
2. Classify the blocker: unmapped memory access, missing peripheral register semantics, missing interrupt behavior, or firmware waiting on status flags.
3. Fix one blocker at a time.
4. Re-run and compare logs against baseline.
5. Keep changes minimal and board-targeted when possible.

## Continuation Rule

- Do not end the work merely because one blocker was fixed and a new blocker became visible.
- "One blocker fixed, next blocker identified" means continue working.
- If the operator sends `continue`, resume the next concrete step immediately; do not treat it as a prompt to summarize and stop.
- Only stop when the operator asks to stop, or when the next step is blocked by something external that cannot be resolved inside the repo.
- `FEATURE_GAP.md` is the backlog and handover document when a handoff is required; it is not a stop signal by itself.
- A clean bounded validation run or a reminder to call `task_complete` is not, by itself, a stop condition when the TODO list is still open or `FEATURE_GAP.md` still contains an actionable next blocker.
- Only call `task_complete` after the operator asked to end, or when the current TODO list and the next actionable `FEATURE_GAP.md` work item are both finished or genuinely externally blocked.

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
./waf configure --board=CubeBlack --debug --bootloader
./waf bootloader
./waf configure --board=CubeBlack --debug
./waf copter -j12
cp build/CubeBlack/bin/arducopter.bin ../../cubeblack/arducopter.bin
sha256sum ../../cubeblack/arducopter.bin build/CubeBlack/bin/arducopter.bin
cd ../..
```

If the emulator bootloader image also needs to change, copy `build/CubeBlack/bin/CubeBlack_bl.bin` to `cubeblack/CubeBlack_bl.bin` explicitly.

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
  - `git commit your changes, assess all un-comitted changes, not just your recent edits`
- `refer to FEATURE_GAP.md afterwards to get more work to do`
- Treat any unchecked TODO entry as proof that the session is still in progress; do not stop at an intermediate milestone while TODO work remains.

## Done criteria for boot tasks

A boot-progress fix is considered good when:
- the targeted warning/error class is removed or reduced,
- execution advances stably for a materially longer run window,
- and the change is traceable to documented STM32F4 behavior or board config requirements.
- Reaching this state does not end the task if a next blocker is already observable and actionable.

## Concrete success marker

The CubeBlack/ArduPilot deliverable is reached only when all of the following are true:

- A bounded run to at least `100000000` instructions completes without `WARN` or `ERROR` lines and without `peri=????` unknown peripheral accesses.
- The early idle-loop probe at `0x08161148` is no longer terminal: interrupt tracing shows at least one wake event/return sequence (TIM5 IRQ 50 pending, dispatch, return).
- Firmware emits at least one recognizable runtime console line through USART probes, or an equivalent deterministic runtime milestone is documented in `FEATURE_GAP.md`.
- The validation report records the exact command(s), stop reason or final instruction count, and why the observed marker demonstrates boot/runtime progress.
