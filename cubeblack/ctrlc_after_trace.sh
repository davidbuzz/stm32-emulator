#!/bin/bash

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/.." && pwd)"
PIPE_PATH="/tmp/stm32-emulator-ctrlc.pipe"
LOG_PATH="/tmp/stm32-emulator-ctrlc.log"
TARGET_LINE="TRACE StorageAccess::write_block reached pc=0x0809737c"
DELAY_SECONDS=5

rm -f "$PIPE_PATH" "$LOG_PATH"
mkfifo "$PIPE_PATH"
trap 'rm -f "$PIPE_PATH"' EXIT

setsid bash -lc "
    exec /usr/bin/time -f 'real=%e user=%U sys=%S maxrss=%M exit=%x' \
        stdbuf -oL -eL '$ROOT_DIR/target/release/stm32-emulator' \
        '$ROOT_DIR/cubeblack/config.yaml' -v --max-instructions 23000000 \
        > '$PIPE_PATH' 2>&1
" &
emu_pid=$!

matched=0
signal_sent=0

while IFS= read -r line; do
    printf '%s\n' "$line" | tee -a "$LOG_PATH" >/dev/null
    #echo "$line" >&2

    if [[ $matched -eq 0 && "$line" == *"$TARGET_LINE"* ]]; then
        matched=1
        (
            sleep "$DELAY_SECONDS"
            kill -INT -- "-$emu_pid" 2>/dev/null || true
        ) &
        signal_sent=1
    fi
done < "$PIPE_PATH"

set +e
wait "$emu_pid"
status=$?
set -e

printf '%s\n' '--- summary ---'
printf 'matched=%s signal_sent=%s status=%s\n' "$matched" "$signal_sent" "$status"
printf '%s\n' '--- key ---'
rg -n "StorageAccess::write_block reached pc=0x0809737c|USB-CDC ep1|ArduCopter|Free RAM|Firmware change|done\.|Stop requested|Reached target" "$LOG_PATH" | head -160 || true
printf '%s\n' '--- tail ---'
tail -25 "$LOG_PATH" || true
