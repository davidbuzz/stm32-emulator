# CAN Audit (RM0090 bxCAN Source-of-Truth)

Scope
- Peripheral audited: src/peripherals/can.rs
- Reference source: STM32F4 bxCAN register and mailbox/filter behavior.

## Executive result
Current CAN is a register-storage stub with basic INRQ/INAK handshake only; it is missing almost all bxCAN runtime semantics.

Wrong
1. No read-only/write-restricted handling for many status/control bits.
2. Mailbox and status side effects are not represented.

Missing
1. Tx mailbox request/completion path and TSR flags.
2. Rx FIFO behavior, frame acceptance, and FIFO flags.
3. Filter banks and CAN2 shared-filter interactions.
4. Error counters/state machine (TEC/REC, bus-off/error passive/warning).
5. Interrupt generation mapping for TX/RX/SCE classes.

Incomplete
1. Init/sleep/normal mode transitions beyond INRQ/INAK are absent.
2. Bit timing validation and loopback/silent modes are not modeled.

## High-priority correction targets
1. Implement mailbox TX/RX FIFO minimal state machines.
2. Add filter-bank programming + acceptance path.
3. Add core interrupt/status behavior to unblock CAN users.

## Key file anchors
- src/peripherals/can.rs:11
- src/peripherals/can.rs:33
