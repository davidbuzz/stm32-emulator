#!/bin/bash

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/.." && pwd)"

if [[ -f "$HOME/.cargo/env" ]]; then
	# shellcheck disable=SC1090
	source "$HOME/.cargo/env"
fi

if command -v cargo >/dev/null 2>&1; then
	cargo run --release -- config.yaml -v
elif [[ -x "$ROOT_DIR/target/release/stm32-emulator" ]]; then
	"$ROOT_DIR/target/release/stm32-emulator" config.yaml -v
else
	echo "error: neither cargo nor $ROOT_DIR/target/release/stm32-emulator is available" >&2
	echo "hint: run $ROOT_DIR/DEV_SETUP_UBUNTU.sh first" >&2
	exit 1
fi