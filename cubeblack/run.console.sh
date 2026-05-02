#!/bin/bash

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/.." && pwd)"
LOG_FILE="$ROOT_DIR/cubeblack/ardu.cubeblack.log"
EMULATOR="$ROOT_DIR/target/release/stm32-emulator"

mkdir -p "$(dirname "$LOG_FILE")"

# Truncate previous log and run in the foreground so Ctrl-C stops emulation immediately.
: >"$LOG_FILE"

if [[ ! -x "$EMULATOR" ]]; then
	echo "error: missing executable: $EMULATOR" >&2
	exit 1
fi

if command -v stdbuf >/dev/null 2>&1; then
	stdbuf -oL -eL "$EMULATOR" --console-only "$ROOT_DIR/cubeblack/config.yaml" --max-instructions 120000000 2>&1 | tee "$LOG_FILE"
else
	"$EMULATOR" --console-only "$ROOT_DIR/cubeblack/config.yaml" --max-instructions 120000000 2>&1 | tee "$LOG_FILE"
fi
