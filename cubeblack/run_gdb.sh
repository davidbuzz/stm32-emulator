#!/bin/bash
# Start the emulator with GDB server on port 3333 and connect arm-none-eabi-gdb.
# The emulator halts at the reset vector; GDB connects and is ready to step/continue.
#
# Usage:
#   ./run_gdb.sh                  # uses default port 3333
#   ./run_gdb.sh 3334             # uses custom port
#   ./run_gdb.sh 3333 extra args  # passes extra args to the emulator

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/.." && pwd)"
CUBEBLACK_DIR="$ROOT_DIR/cubeblack"
GDB_PORT="${1:-3333}"

# ── Locate emulator ───────────────────────────────────────────────────────────

if [[ -f "$HOME/.cargo/env" ]]; then
    # shellcheck disable=SC1090
    source "$HOME/.cargo/env"
fi

if [[ -x "$ROOT_DIR/target/release/stm32-emulator" ]]; then
    EMU_CMD=("$ROOT_DIR/target/release/stm32-emulator")
elif command -v cargo >/dev/null 2>&1; then
    EMU_CMD=(cargo run --manifest-path "$ROOT_DIR/Cargo.toml" --release --)
else
    echo "error: stm32-emulator binary not found and cargo is unavailable" >&2
    echo "hint:  run $ROOT_DIR/DEV_SETUP_UBUNTU.sh, then cargo build --release" >&2
    exit 1
fi

# ── Locate GDB ────────────────────────────────────────────────────────────────

if command -v arm-none-eabi-gdb >/dev/null 2>&1; then
    GDB_CMD=arm-none-eabi-gdb
elif command -v gdb-multiarch >/dev/null 2>&1; then
    GDB_CMD=gdb-multiarch
else
    echo "error: neither arm-none-eabi-gdb nor gdb-multiarch found in PATH" >&2
    exit 1
fi

# ── Locate debug ELF (for symbols) ───────────────────────────────────────────

ELF="$ROOT_DIR/modules/ardupilot/build/CubeBlack/bin/arducopter"
#ELF="$ROOT_DIR/cubeblack/arducopter.elf"
if [[ ! -f "$ELF" ]]; then
    echo "warning: debug ELF not found at $ELF" >&2
    echo "hint:    run: cd modules/ardupilot && ./waf configure --board=CubeBlack --debug && ./waf copter" >&2
    ELF=""
fi

# ── Start emulator in background ─────────────────────────────────────────────

echo "Starting emulator on GDB port $GDB_PORT ..."
"${EMU_CMD[@]}" "$CUBEBLACK_DIR/config.yaml" -v --gdb "$GDB_PORT" &
EMU_PID=$!

# Kill the emulator when this script exits.
trap 'kill "$EMU_PID" 2>/dev/null; wait "$EMU_PID" 2>/dev/null' EXIT

# Give the emulator a moment to bind the port before GDB tries to connect.
sleep 0.5

# ── Launch GDB ────────────────────────────────────────────────────────────────

GDB_ARGS=()
if [[ -n "$ELF" ]]; then
    GDB_ARGS+=("$ELF")
fi

GDB_ARGS+=(
    -ex "set architecture armv7e-m"
    -ex "set remotetimeout 10"
    -ex "target remote localhost:$GDB_PORT"
)

echo "Connecting $GDB_CMD to localhost:$GDB_PORT ..."
"$GDB_CMD" --nx "${GDB_ARGS[@]}"
