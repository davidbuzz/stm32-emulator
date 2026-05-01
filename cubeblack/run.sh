#!/bin/bash

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/.." && pwd)"
LOG_FILE="$ROOT_DIR/cubeblack/ardu.cubeblack.log"
TIME_FORMAT='real=%e user=%U sys=%S maxrss=%M exit=%x'

if [[ $# -gt 0 ]]; then
	run_args=("$@")
else
	run_args=(config.yaml -v --max-instructions 120000000)
fi

if [[ -f "$HOME/.cargo/env" ]]; then
	# shellcheck disable=SC1090
	source "$HOME/.cargo/env"
fi

if [[ -x "$ROOT_DIR/target/release/stm32-emulator" ]]; then
	emulator_cmd=("$ROOT_DIR/target/release/stm32-emulator" --console-only "${run_args[@]}")
elif command -v cargo >/dev/null 2>&1; then
	emulator_cmd=(cargo run --release -- --console-only "${run_args[@]}")
else
	echo "error: neither cargo nor $ROOT_DIR/target/release/stm32-emulator is available" >&2
	echo "hint: run $ROOT_DIR/DEV_SETUP_UBUNTU.sh first" >&2
	exit 1
fi

mkdir -p "$(dirname "$LOG_FILE")"

run_with_logging() {
	local run_cmd=("${emulator_cmd[@]}")
	if command -v stdbuf >/dev/null 2>&1; then
		run_cmd=(stdbuf -oL -eL "${run_cmd[@]}")
	fi
	if command -v /usr/bin/time >/dev/null 2>&1; then
		/usr/bin/time -f "$TIME_FORMAT" "${run_cmd[@]}"
	else
		"${run_cmd[@]}"
	fi
}

run_with_logging >"$LOG_FILE" 2>&1 &
emulator_pid=$!

for _ in $(seq 120); do
	if ! kill -0 "$emulator_pid" 2>/dev/null; then
		break
	fi
	sleep 1
done

kill "$emulator_pid" 2>/dev/null || true
for _ in $(seq 1 5); do
	if ! kill -0 "$emulator_pid" 2>/dev/null; then
		break
	fi
	sleep 1
done
kill -9 "$emulator_pid" 2>/dev/null || true
wait "$emulator_pid" 2>/dev/null || true

cat "$LOG_FILE"
