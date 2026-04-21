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

- Setup Ubuntu dev environment:
  - ./DEV_SETUP_UBUNTU.sh
- Run CubeBlack directly:
  - cd cubeblack
  - ../target/release/stm32-emulator config.yaml -v --max-instructions 3000000
- Detect tight loops:
  - ../target/release/stm32-emulator config.yaml -v --busy-loop-stop

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

## STM32F4 reference docs in this repo

- cubeblack/STM32F4xx_Reference_Manual.md
- cubeblack/stm32f427vg-datasheet.md
- cubeblack/STM32F4_DMA.md

Use these local docs as the source of truth for register behavior and memory layout.

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

- When maintaining a TODO list for work in this repo, always end it with these final items in this order:
  - `git commit your changes`
  - `refer to FEATURE_GAP.md afterwards to get more work to do`
- After completing the current task, use `FEATURE_GAP.md` as the default source for identifying the next useful piece of work.

## Done criteria for boot tasks

A boot-progress fix is considered demonstrably good when:

- the targeted warning/error class is removed or reduced,
- execution advances stably for a materially longer run window,
- and the change is traceable to documented STM32F4 behavior or board config requirements.

## Concrete success marker

Treat the CubeBlack/ArduPilot deliverable as reached only when all of the following are true:

- A bounded run to at least `100000000` instructions completes without `WARN` or `ERROR` log lines and without `peri=????` unknown peripheral accesses.
- The early idle-loop probe at `0x08161148` is no longer a terminal state: a run with interrupt tracing shows at least one wake event/return sequence (`TIM5` IRQ 50 pending, interrupt dispatch, and return).
- Firmware emits at least one recognizable runtime console line through configured USART probes, or an equivalent deterministic runtime milestone is captured and documented in `FEATURE_GAP.md`.
- The validation report records the exact command(s), stop reason or final instruction count, and why the observed marker demonstrates boot/runtime progress rather than a passive loop.
