# AGENTS.md

Repository guidance for AI coding agents working in this project.

## Project intent

- Primary objective: emulate STM32 boards well enough to boot and run unmodified firmware.
- Current focus board: CubeBlack (STM32F427) and ArduPilot binaries in cubeblack/.
- Treat behavioral correctness as higher priority than perfect hardware cycle accuracy.

## Current end goal

- Current deliverable: get the CubeBlack target to boot and run ArduPilot firmware in the emulator with no firmware-side bypasses required for normal startup.
- When deciding what to work on next, prefer tasks that remove the highest-confidence blocker preventing demonstrable CubeBlack boot progress.
- Treat `FEATURE_GAP.md` as the canonical backlog and handover document for remaining work toward this deliverable.

## First steps on each task

- Read the active board config before changing emulator behavior.
- Reproduce current behavior with a bounded run before editing code.
- Capture concrete evidence: instruction count reached, stop PC, and warning classes.
- When choosing the next task, consult `FEATURE_GAP.md` first; this is the primary backlog and the default place to look for more work.

## Known working run commands

- CubeBlack firmware source of truth for emulator runs:
  - The emulator loads the ROM image named in `cubeblack/config.yaml`, currently `cubeblack/arducopter.bin`.
  - Rebuilding under `modules/ardupilot/build/CubeBlack/bin/` does not change the emulator input until you copy the rebuilt image into `cubeblack/`.
  - Refresh command:
    - `cp modules/ardupilot/build/CubeBlack/bin/arducopter.bin cubeblack/arducopter.bin`
  - Optional verification:
    - `sha256sum cubeblack/arducopter.bin modules/ardupilot/build/CubeBlack/bin/arducopter.bin`

- Setup Ubuntu dev environment:
  - ./DEV_SETUP_UBUNTU.sh
- Run CubeBlack directly:
  - cd cubeblack
  - ../target/release/stm32-emulator config.yaml -v --max-instructions 3000000
- Run CubeBlack long validation and capture evidence log:
  - cd cubeblack
  - /usr/bin/time -f 'real=%e user=%U sys=%S maxrss=%M exit=%x' timeout 120 ../target/release/stm32-emulator config.yaml -v --max-instructions 120000000 2>&1 | tee ardu.cubeblack.log
- Detect tight loops:
  - ../target/release/stm32-emulator config.yaml -v --busy-loop-stop
- `cubeblack/run.sh` should remain aligned with the long validation command above and must refresh `cubeblack/ardu.cubeblack.log` on each long-run execution.

## Execution budgeting

- Prefer wrapping emulator runs with `/usr/bin/time` so validation reports include wall-clock cost.
- Prefer `timeout` for exploratory probes that may not terminate promptly.
- Current measured runtimes on this workspace state:
  - `--max-instructions 3000000` completes in about `1.3s`
  - `--max-instructions 120000000` completes in about `47s`
  - `--busy-loop-stop` did not converge quickly in one probe and was cut off at `30s` / about `70.9M` instructions
- Practical defaults:
  - short bounded runs: `timeout 30`
  - longer bounded validation runs up to `120000000` instructions: `timeout 120`
  - `--busy-loop-stop` and other open-ended probes: always wrap in `timeout 30` unless there is a specific reason not to
- Recommended command forms:
  - `cd cubeblack && /usr/bin/time -f 'real=%e user=%U sys=%S maxrss=%M exit=%x' timeout 30 ../target/release/stm32-emulator config.yaml -v --max-instructions 3000000`
  - `cd cubeblack && /usr/bin/time -f 'real=%e user=%U sys=%S maxrss=%M exit=%x' timeout 120 ../target/release/stm32-emulator config.yaml -v --max-instructions 120000000 2>&1 | tee ardu.cubeblack.log`
  - `cd cubeblack && /usr/bin/time -f 'real=%e user=%U sys=%S maxrss=%M exit=%x' timeout 30 ../target/release/stm32-emulator config.yaml -v --busy-loop-stop`
- The long validation run is expected to leave behind `cubeblack/ardu.cubeblack.log` as the evidence artifact for that checkpoint.
- Commit updated `cubeblack/ardu.cubeblack.log` regularly alongside meaningful emulator progress so git history preserves the observed runtime evidence over time.

## Boot-debug workflow

1. Reproduce with max-instructions and record logs.
2. Classify blockers:
   - unmapped memory access
   - missing peripheral register semantics
   - missing interrupt behavior
   - firmware waits on status flags
3. Fix one blocker at a time.
4. Re-run and compare logs against baseline.
5. Keep changes minimal and board-targeted when possible.

## Continuation rule

- Do not stop after fixing one blocker if boot/runtime progress simply exposes the next actionable blocker.
- Treat "fixed one blocker, found the next blocker" as an in-progress state, not a completion state.
- Continue immediately into the next highest-confidence blocker unless a hard external blocker appears or the operator explicitly asks to pause.
- If the operator says `continue`, treat that as an explicit instruction to resume work immediately rather than a conversational checkpoint.
- Use `FEATURE_GAP.md` as a handover document only when ending is actually necessary, not as a reason to stop early.
- Do not treat a local success marker, a validation pass, or an instruction-hook reminder about `task_complete` as permission to stop while the repo TODO list still has open items or `FEATURE_GAP.md` still has an actionable next blocker.
- `task_complete` is only appropriate when the operator asked to end, or when both the current TODO list and the next actionable `FEATURE_GAP.md` work item have been exhausted or are externally blocked.

## STM32F4 reference docs in this repo

- cubeblack/STM32F4xx_Reference_Manual.md
- cubeblack/stm32f427vg-datasheet.md
- cubeblack/STM32F4_DMA.md

Use these local docs as the source of truth for register behavior and memory layout.

## ArduPilot source code

The full ArduPilot source tree is available as a git submodule:

- ArduPilot root: `./modules/ardupilot/`
- ChibiOS (RTOS for CubeBlack): `./modules/ardupilot/modules/ChibiOS/`

Use this for cross-referencing firmware behavior, ISR logic, USB driver internals (see `modules/ardupilot/modules/ChibiOS/os/hal/ports/STM32/LLD/OTGv1/`), and understanding how ChibiOS polls peripheral registers.

### Rebuilding the CubeBlack binary from source

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

If you also intend to refresh the bootloader image used by the emulator, copy `build/CubeBlack/bin/CubeBlack_bl.bin` to `cubeblack/CubeBlack_bl.bin` separately.

## Emulator implementation guidance

- Prefer implementing realistic register read/write behavior over bypass patches.
- For DMA on STM32F4:
  - Keep stream register decode aligned with offset 0x10 + n*0x18.
  - Model status/clear flag behavior (LISR/HISR and LIFCR/HIFCR) when firmware polls completion.
  - Maintain EN and NDTR transitions expected by polling loops.
- Preserve existing public interfaces unless a task requires refactoring.

## Config and memory-map guidance

- Favor adding missing memory regions in board config over skipping instructions.
- For STM32F427 boards, account for CCM RAM and system memory/UID areas when firmware touches them.
- Keep firmware patch entries in config.yaml documented with why they exist.

## Validation expectations

- After each meaningful change:
  - run a short bounded test (for fast feedback)
  - run a longer stability test (to confirm no regression)
- Capture the longer stability test output into `cubeblack/ardu.cubeblack.log` and keep that file current in git as evidence of the latest long-run result.
- Report:
  - command used
  - final instruction count or stop reason
  - whether warning classes improved, regressed, or stayed the same

## Editing guardrails

- Do not revert unrelated user changes.
- Avoid destructive git operations.
- Keep commits focused and small.
- Prefer targeted patches over broad cleanup.

## Todo list requirements

- For any non-trivial CubeBlack/emulator workstream in this repo, always keep a repo TODO list.
- When maintaining a TODO list for work in this repo, always end it with these final items in this order:
  - `git commit your changes, assess all un-comitted changes, not just your recent edits`
  - `refer to FEATURE_GAP.md afterwards to get more work to do`
- After completing the current task, use `FEATURE_GAP.md` as the default source for identifying the next useful piece of work.
- An incomplete repo TODO list means the work is still in progress; do not summarize-and-stop at an intermediate milestone while those items remain open.

## Done criteria for boot tasks

A boot-progress fix is considered demonstrably good when:

- the targeted warning/error class is removed or reduced,
- execution advances stably for a materially longer run window,
- and the change is traceable to documented STM32F4 behavior or board config requirements.
- This is evidence of progress, not permission to stop if another concrete blocker is now visible and actionable.

## Concrete success marker

Treat the CubeBlack/ArduPilot deliverable as reached only when all of the following are true:

- A bounded run to at least `100000000` instructions completes without `WARN` or `ERROR` log lines and without `peri=????` unknown peripheral accesses.
- The early idle-loop probe at `0x08161148` is no longer a terminal state: a run with interrupt tracing shows at least one wake event/return sequence (`TIM5` IRQ 50 pending, interrupt dispatch, and return).
- Firmware emits at least one recognizable runtime console line through configured USART probes, or an equivalent deterministic runtime milestone is captured and documented in `FEATURE_GAP.md`.
- The validation report records the exact command(s), stop reason or final instruction count, and why the observed marker demonstrates boot/runtime progress rather than a passive loop.
