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
../target/release/stm32-emulator config.yaml -v --max-instructions 20000000

# Longer stability run with exact 120-second wall-clock cutoff
( /usr/bin/time -f 'real=%e user=%U sys=%S maxrss=%M exit=%x' \
  ../target/release/stm32-emulator config.yaml -v --max-instructions 120000000 \
) > ardu.cubeblack.log 2>&1 & pid=$!
for _ in $(seq 120); do kill -0 "$pid" 2>/dev/null || break; sleep 1; done
kill -9 "$pid" 2>/dev/null || true
wait "$pid" 2>/dev/null || true
tail -40 ardu.cubeblack.log

# Detect tight loops
../target/release/stm32-emulator config.yaml -v --busy-loop-stop
```

## Execution Budgeting

- Wrap emulator runs with `/usr/bin/time` when validating so runtime cost is visible in the log.
- Wrap exploratory or potentially open-ended probes with `timeout` when they are not inside a logging pipeline.
- For exact wall-clock bounded validation runs that also capture logs, prefer a background PID plus explicit `sleep`/`kill -9` over `timeout` in a pipeline.
- Measured on the current workspace state:
  - short bounded run baseline: `--max-instructions 20000000`
  - `main` is reached in about `1.6s`
  - `--max-instructions 120000000` takes about `47s`
  - `--busy-loop-stop` did not terminate quickly in one probe and was cut off at `30s` / about `70.9M` instructions
- Recommended defaults:
  - `timeout 10` for short runs
  - `timeout 30` for busy-loop probes
  - explicit 120-second PID kill for longer bounded runs up to `120000000` instructions

```bash
cd cubeblack

# Short feedback run
/usr/bin/time -f 'real=%e user=%U sys=%S maxrss=%M exit=%x' \
  timeout 10 ../target/release/stm32-emulator config.yaml -v --max-instructions 20000000

# Longer stability run
( /usr/bin/time -f 'real=%e user=%U sys=%S maxrss=%M exit=%x' \
  ../target/release/stm32-emulator config.yaml -v --max-instructions 120000000 \
) > ardu.cubeblack.log 2>&1 & pid=$!
for _ in $(seq 120); do kill -0 "$pid" 2>/dev/null || break; sleep 1; done
kill -9 "$pid" 2>/dev/null || true
wait "$pid" 2>/dev/null || true
tail -40 ardu.cubeblack.log

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

## Continuation Rule (MANDATORY — DO NOT DEVIATE)

- **PRIMARY RULE**: Never call `task_complete` while the TODO list has any unchecked items or items marked `in-progress`.
- **COROLLARY**: Never call `task_complete` while `FEATURE_GAP.md` has unresolved items still marked `open` or `partially implemented`.
- Do not end the work merely because one blocker was fixed and a new blocker became visible.
- "One blocker fixed, next blocker identified" means continue working.
- If the operator sends `continue`, resume the next concrete step immediately; do not treat it as a prompt to summarize and stop.
- Only stop when the operator asks to stop, or when the next step is blocked by something external that cannot be resolved inside the repo.
- `FEATURE_GAP.md` is the backlog and handover document when a handoff is required; it is not a stop signal by itself.
- A clean bounded validation run or a reminder to call `task_complete` is not, by itself, a stop condition when the TODO list is still open or `FEATURE_GAP.md` still contains an actionable next blocker.
- **HARD STOP CHECK**: Before calling `task_complete`, verify:
  1. All TODO items are marked `completed` (none `not-started` or `in-progress`)
  2. All FEATURE_GAP.md items are marked `done` (none `open` or `partially implemented`)
  3. The operator explicitly asked to stop OR both conditions above are fully satisfied
  4. If any of these fail, mark the next TODO item `in-progress` and continue work immediately

## Completion-Hook Guardrail (MANDATORY)

- If any tooling message says "you must call task_complete" while TODO or `FEATURE_GAP.md` still has actionable open work, treat it as a reminder only.
- Do not end the task in that situation.
- Required response sequence:
  1. Keep one concrete TODO item `in-progress`
  2. Implement the next blocker
  3. Run bounded validation
  4. Update `FEATURE_GAP.md`
  5. Commit
  6. Continue to the next blocker
- Call `task_complete` only when HARD STOP CHECK conditions are actually true.

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

- For any non-trivial CubeBlack/emulator workstream in this repo, always keep a repo TODO list.
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
