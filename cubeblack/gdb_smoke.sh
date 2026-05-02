#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
EMULATOR="$ROOT_DIR/target/release/stm32-emulator"
CONFIG="$ROOT_DIR/cubeblack/config.yaml"
ELF="$ROOT_DIR/modules/ardupilot/build/CubeBlack/bin/arducopter"
GDB_BIN="$(command -v arm-none-eabi-gdb || command -v gdb-multiarch)"
TIMEOUT_SECONDS=20

if [[ ! -x "$EMULATOR" ]]; then
  echo "ERROR: emulator binary not found at $EMULATOR"
  exit 1
fi
if [[ ! -f "$ELF" ]]; then
  echo "ERROR: firmware ELF not found at $ELF"
  exit 1
fi

echo "[1/3] Symbol check"
SYMS_OUT="$("$GDB_BIN" --nx -q "$ELF" \
  -ex 'info address __start' \
  -ex 'info address Reset_Handler' \
  -ex 'info address __port_thread_start' \
  -ex 'info address main' \
  -ex quit 2>&1)"
echo "$SYMS_OUT"
echo "$SYMS_OUT" | rg -q 'No symbol "__start" in current context\.'
echo "$SYMS_OUT" | rg -q 'Symbol "Reset_Handler" is at 0x'
echo "$SYMS_OUT" | rg -q 'Symbol "__port_thread_start" is at 0x'
echo "$SYMS_OUT" | rg -q 'Symbol "main\(int, char\* const\*\)" is a function at address 0x'

echo "[2/3] Early breakpoint / step / detach"
PORT_EARLY=3360
EMULOG_EARLY=/tmp/gdb_smoke_early_server.log
GDBLOG_EARLY=/tmp/gdb_smoke_early_client.log
(
  cd "$ROOT_DIR/cubeblack"
  timeout -k 1 "$TIMEOUT_SECONDS" "$EMULATOR" "$CONFIG" -v --gdb "$PORT_EARLY" >"$EMULOG_EARLY" 2>&1
) &
PID_EARLY=$!
sleep 1
"$GDB_BIN" --nx -q "$ELF" \
  -ex 'set pagination off' \
  -ex 'set confirm off' \
  -ex "target remote :$PORT_EARLY" \
  -ex 'info registers pc sp lr' \
  -ex 'x/i $pc' \
  -ex 'x/4wx 0x08004000' \
  -ex 'tbreak __port_thread_start' \
  -ex 'continue' \
  -ex 'bt' \
  -ex 'stepi' \
  -ex 'info registers pc' \
  -ex 'detach' \
  -ex 'quit' >"$GDBLOG_EARLY" 2>&1 || true
kill "$PID_EARLY" 2>/dev/null || true
wait "$PID_EARLY" 2>/dev/null || true
rg -q 'Remote debugging using' "$GDBLOG_EARLY"
rg -q 'pc\s+0x8005314\s+0x8005314 <Reset_Handler>' "$GDBLOG_EARLY"
rg -q 'Temporary breakpoint .*__port_thread_start' "$GDBLOG_EARLY"
rg -q 'Detaching from program:' "$GDBLOG_EARLY"

echo "[3/3] Continue to main"
PORT_MAIN=3361
EMULOG_MAIN=/tmp/gdb_smoke_main_server.log
GDBLOG_MAIN=/tmp/gdb_smoke_main_client.log
(
  cd "$ROOT_DIR/cubeblack"
  timeout -k 1 "$TIMEOUT_SECONDS" "$EMULATOR" "$CONFIG" -v --gdb "$PORT_MAIN" >"$EMULOG_MAIN" 2>&1
) &
PID_MAIN=$!
sleep 1
"$GDB_BIN" --nx -q "$ELF" \
  -ex 'set architecture armv7e-m' \
  -ex 'set pagination off' \
  -ex 'set confirm off' \
  -ex "target remote :$PORT_MAIN" \
  -ex 'tbreak main' \
  -ex 'continue' \
  -ex 'bt' \
  -ex 'detach' \
  -ex 'quit' >"$GDBLOG_MAIN" 2>&1 || true
kill "$PID_MAIN" 2>/dev/null || true
wait "$PID_MAIN" 2>/dev/null || true
rg -q 'Temporary breakpoint .*main' "$GDBLOG_MAIN"
rg -q '#0\s+main ' "$GDBLOG_MAIN"
rg -q 'Detaching from program:' "$GDBLOG_MAIN"

echo "PASS: GDB smoke checks succeeded"
echo "- Early log: $GDBLOG_EARLY"
echo "- Main log:  $GDBLOG_MAIN"
