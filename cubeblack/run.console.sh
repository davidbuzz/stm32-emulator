#!/bin/bash

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/.." && pwd)"
LOG_FILE="$ROOT_DIR/cubeblack/ardu.cubeblack.log"
EMULATOR="$ROOT_DIR/target/release/stm32-emulator"
TIMEOUT_SECONDS=30

mkdir -p "$(dirname "$LOG_FILE")"

# Truncate previous log and run in the foreground so Ctrl-C stops emulation immediately.
: >"$LOG_FILE"

if [[ ! -x "$EMULATOR" ]]; then
	echo "error: missing executable: $EMULATOR" >&2
	exit 1
fi

set +e
if command -v stdbuf >/dev/null 2>&1; then
	timeout -k 1 "$TIMEOUT_SECONDS" stdbuf -oL -eL "$EMULATOR" --console-only "$ROOT_DIR/cubeblack/config.yaml" --max-instructions 200000000 2>&1 | tee "$LOG_FILE"
else
	timeout -k 1 "$TIMEOUT_SECONDS" "$EMULATOR" --console-only "$ROOT_DIR/cubeblack/config.yaml" --max-instructions 200000000 2>&1 | tee "$LOG_FILE"
fi
run_status=${PIPESTATUS[0]}
set -e

if [[ "$run_status" -eq 124 || "$run_status" -eq 137 ]]; then
	echo "info: bounded test hit ${TIMEOUT_SECONDS}s timeout"
	exit 0
fi

exit "$run_status"
