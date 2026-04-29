---
name: cubeblack-hardware
description: "CubeBlack (STM32F427) emulator interaction for the ArduPilot port. Use when: building and deploying firmware to the emulator; connecting GDB to the emulator; live inspecting memory/registers; diagnosing USB CDC serial silence; adding DEV_PRINTF/print statements to trace boot crashes; diagnosing UART/GPIO issues; halting vs resetting; using --nx with gdb; reading /dev/ttyACM* output."
argument-hint: "What do you need to do? (build / gdb / uart-debug / monitor-serial / print-debug)"
---

# CubeBlack Hardware Skill

## Repository Safety

`git worktree` commands are approval-gated.

Before running any `git worktree add`, `git worktree remove`, `git worktree move`, `git worktree prune`, or any equivalent worktree-management command, the agent must first tell the human:
- the exact directory/path that will be created, removed, or changed
- the branch or commit that worktree will use
- the reason for using a worktree instead of the current checkout

Do not run the command until the user/operator/admin explicitly approves it.

## Flash Workflows

The emulator loads `arducopter.bin` directly at `0x08004000` and jumps to its reset
vector — the bootloader binary is mapped into memory but never executed.

Before any step that requires touching the target board, the agent must stop and
explicitly prompt the human for that action. Do not assume the human will unplug,
re-plug, or press buttons without being asked.

---

### 1. App Firmware — build and copy

Build the app and copy the binary into the emulator config directory:

```bash
cd /home/buzz/ardupilot
./waf configure --board=CubeBlack --debug
./waf copter -j12
cp build/CubeBlack/bin/arducopter.bin /home/buzz/stm32-emulator/cubeblack/arducopter.bin
```

---

## GDB Live Diagnosis

### CRITICAL: always use `--nx`

`/home/buzz/ardupilot/.gdbinit` contains `mon reset halt`.  
**This resets the board on every standard GDB connect**, masking the real live state.  
Always use `--nx` for diagnostics:

```bash
gdb-multiarch --nx build/CubeBlack/bin/arducopter
# then manually:
(gdb) target extended-remote :3333
(gdb) mon halt          # halt without reset
```

### Useful GDB inspection commands

```gdb
# General state
info threads
bt                          # backtrace current thread

# Memory: read a struct field
p serial0Driver._writebuf.available()
p serial1Driver._device_initialised
p serial1Driver.uart_thread_ctx

# Read raw memory address
x/4wx 0x20000000

# Show USB CDC state
p SDU1.state              # SDU_READY = good
p SDU1.config->usbp->state  # USB_ACTIVE = good
```

---

## USB CDC Serial Debugging (print statements)

USB CDC serial is the primary debugging tool for boot crashes.
The device emits both MAVLink frames and plain-text `DEV_PRINTF` / `hal.console->printf` output.
The port changes ACM number on every reboot (ttyACM1 → ttyACM2 → etc.), so use the stable
symlink `/dev/serial/by-id/usb-ArduPilot_CubeBlack_*-if00` instead.

### Read boot output across reboots

```python
import serial, time, glob, os

BY_ID_GLOB = '/dev/serial/by-id/usb-ArduPilot_CubeBlack_*-if00'

def find_port():
    matches = glob.glob(BY_ID_GLOB)
    if matches:
        return matches[0]
    ports = sorted(glob.glob('/dev/ttyACM*'))
    return ports[0] if ports else None

all_data = b''
t_end = time.time() + 30          # monitor for 30 s across reboots
while time.time() < t_end:
    port = find_port()
    if not port:
        time.sleep(0.1); continue
    try:
        s = serial.Serial(port, 115200, timeout=0.3)
        t_close = time.time() + 5  # read one cycle then reconnect
        while time.time() < t_close and time.time() < t_end:
            d = s.read(256)
            if d:
                all_data += d
                print(d.decode('utf-8', errors='replace'), end='', flush=True)
        s.close()
    except Exception as e:
        time.sleep(0.2)
```

### Boot sequence visible on USB serial

During normal startup the following text appears in this order:
```
\n                          # ChibiOS/ArduPilot boot marker
<MAVLink binary frames>     # HEARTBEAT etc. (binary, not text)
No Compass backends available
Init Gyro                   # AP_InertialSensor gyro calibration starts
<\n>                        # calibration loop DEV_PRINTF("\n") at end
ArduCopter Vx.x.x ...       # version banner = successful full boot
```

If the device reboots after `Init Gyro` but before the version banner, the crash is
happening inside `AP_InertialSensor::_init_gyro()` or something it calls.

### Add print statements to narrow down a crash

Use `DEV_PRINTF` for debug prints in ArduPilot C++ code (compiled out on final builds,
always emits to hal.console in debug builds). Use `hal.console->printf` when you need
it to always emit regardless of build type.

**In C++ (libraries or vehicle code):**
```cpp
// Breadcrumb prints — add around the suspected crash site
DEV_PRINTF("STAGE A\n");
// ... suspect code ...
DEV_PRINTF("STAGE B val=%d\n", (int)some_value);
```

**In ChibiOS HAL (AP_HAL_ChibiOS/):**
```cpp
// Use AP_HAL::panic() for fatal errors — prints to console then halts+resets
AP_HAL::panic("UART: SM%u failed to start\n", (unsigned)sm_idx);
```

**After adding prints, always:**
1. `./waf configure --board=CubeBlack --debug && ./waf copter -j12`
2. Flash via SWD ( GDB `load`)
3. Monitor USB output with the Python script above — port changes each reboot

### Example: tracing a boot-time crash

```cpp
// In AP_InertialSensor.cpp, _init_gyro():
DEV_PRINTF("Init Gyro A\n");   // before loop
for (int16_t j = 0; ...) {
    DEV_PRINTF("Init Gyro B j=%d\n", j);  // each calibration iteration
    update();   // <-- if crash is here, B never prints for j > first
}
DEV_PRINTF("Init Gyro done\n");
```

Then monitor USB — last print before reboot identifies the exact line.

---

## Serial / USB Monitor

ArduPilot maps:
| Port | Device | Notes |
|---|---|---|
| SERIAL0 / USB | `/dev/ttyACM0` or `ttyACM1` | USB connector |
| SERIAL1 | UART | TX/RX via carrier board |
| SERIAL2 | UART | TX/RX via carrier board |

```bash
# Detect USB CDC port (stable symlink — survives reboots)
ls /dev/serial/by-id/usb-ArduPilot_CubeBlack*

# Quick health check — should see "Init ArduCopter" text
timeout 5 cat /dev/ttyACM1 | head -c 200 | xxd

# MAVLink stream (human readable)
mavproxy.py --master=/dev/ttyACM1 --baudrate=115200
```

---

## MAVLink SERIAL_CONTROL — Correct Flag Values

**Pymavlink constants** (confirmed from `pymavlink.dialects.v20.common`):

| Flag | Value | Meaning |
|---|---|---|
| `REPLY` | 1 | FC-originated response — **if set, FC returns immediately without acting** |
| `RESPOND` | 2 | FC echoes received data back |
| `EXCLUSIVE` | 4 | Lock port exclusively |
| `BLOCKING` | 8 | Block waiting for data |
| `MULTI` | 16 | Multi-packet response |

**Common mistake:** `EXCLUSIVE=1<<0=1` sets the REPLY bit → `handle_serial_control()` hits early return at line ~40 of `GCS_MAVLink/GCS_serial_control.cpp`:
```cpp
if (packet.flags & SERIAL_CONTROL_FLAG_REPLY) { return; }
```
This means `begin()` is **never called** on the target UART — completely silent failure.

**Correct flags for open+respond**: `EXCLUSIVE | RESPOND = 4 | 2 = 6`

**Correct device IDs** (from `common.xml`):
```python
SERIAL_CONTROL_SERIAL0 = 100   # SERIAL0/USB
SERIAL_CONTROL_SERIAL1 = 101   # SERIAL1
SERIAL_CONTROL_SERIAL2 = 102   # SERIAL2
```

---

## Build Commands

```bash
cd /home/buzz/ardupilot

# Configure (first time or after hwdef changes)
./waf configure --board=CubeBlack --debug

# Build copter
./waf copter -j12

# Bootloader only
./waf configure --board=CubeBlack --debug --bootloader
./waf bootloader -j12
```

Build output: `build/CubeBlack/bin/arducopter` (ELF)

---

## Common Pitfalls

| Symptom | Cause | Fix |
|---|---|---|
| USB CDC goes silent after SERIAL_CONTROL | Wrong REPLY flag set (bit0=1) triggers early return | Use `flags = EXCLUSIVE\|RESPOND = 6` |
| GDB connect resets board unexpectedly | `.gdbinit` has `mon reset halt` | Use `gdb --nx` for live diagnosis |
| No heartbeats on USB after test | Write buffer empty (not a freeze) | Board is running fine; check MAVLink connection |
| Device reboots after "Init Gyro" | Crash inside `_init_gyro()` / IMU driver | Add `DEV_PRINTF` breadcrumbs around each calibration step; monitor USB serial |
| ACM number changes each reboot | USB re-enumeration on every boot | Use `/dev/serial/by-id/usb-ArduPilot_CubeBlack_*-if00` stable symlink |
| USB print output mixed with MAVLink binary | Both share SERIAL0/USB | Use `xxd` or Python `serial` to view raw; text prints are readable between binary frames |
