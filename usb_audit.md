# USB OTG FS Audit (RM0090 Ch.34 Source-of-Truth)

Scope
- Peripheral audited: src/peripherals/otg_fs.rs
- Reference manual source: STM32F4 RM0090 OTG_FS chapter (global/device/power-clock blocks), plus endpoint/interrupt semantics.
- Classification used:
  - wrong: behavior conflicts with RM
  - missing: feature/bit/register path absent
  - incomplete: partially modeled but not to RM behavior

## Executive result
The implementation is intentionally minimal and boot-focused, and it is not register-complete or protocol-complete versus OTG_FS hardware. It is good enough for a narrow CDC bring-up path, but many bits/flags/side effects are either synthetic, incomplete, or absent.

## Global block (OTG_FS_GLOBAL)

Implemented (partial)
- Basic register surfaces for GOTGCTL/GOTGINT/GAHBCFG/GUSBCFG/GRSTCTL/GINTSTS/GINTMSK/GRXFSIZ/GNPTXFSIZ/GNPTXSTS/GCCFG/CID and several FIFO sizing regs are present.
- GRSTCTL AHBIDL exposed and CSRST/RXFFLSH/TXFFLSH clear-after-delay behavior approximated.
- GINTSTS W1C behavior partially modeled.

Wrong
1. GINTSTS model is synthetic for several events
- The implementation injects SRQINT/USBRST/ENUMDNE/SOF based on internal timers/state machine, not on full core state transitions.
- This diverges from RM-defined event generation and timing dependencies.

2. GRXSTSP/GRXSTSR behavior is synthetic
- Pop sequence is driven by custom EP0 state machine and fixed setup payloads.
- Real GRXSTSP packet-status stream, endpoint/byte-count behavior, and FIFO-originated packet metadata are broader and not faithfully represented.

3. GOTG/GUSBCFG/GRSTCTL side effects are incomplete or not enforced
- Many control bits are stored but do not trigger the RM-defined mode/clock/PHY/timeout effects.

Missing
1. Host-mode block behavior
- Host registers/path are not implemented (model is device-focused).

2. Core global interrupt detail coverage
- Not all GINTSTS/GINTMSK sources and interactions are represented (for example full wakeup/suspend/session/OTG events).

3. FIFO architecture fidelity
- Non-periodic/periodic Tx FIFO and Rx FIFO ownership/arbitration/threshold behavior is not modeled per RM.

Incomplete
1. GRSTCTL reset/flush sequences
- Implemented as delayed clear only; full constraints/handshakes/busy states are not complete.

2. GINTSTS RXFLVL handling
- Correctly treated as level-like in places, but RM-level interaction with actual FIFO fill/drain events is still synthetic.

## Device block (OTG_FS_DEVICE)

Implemented (partial)
- DCFG/DCTL/DSTS/DIEPMSK/DOEPMSK/DAINT/DAINTMSK/DIEPEMPMSK surfaces exist.
- Endpoint register windows for IN/OUT EP0..EP3 partially implemented.
- Basic DOEPINT/DIEPINT assertion/clear paths for STUP/XFRC/TXFE exist.

Wrong
1. Endpoint transfer engine is synthetic
- EP0 setup/data/status progression is scripted through fixed CDC setup packets.
- This bypasses generic token parsing and real endpoint transfer descriptors.

2. DIEPCTL/DOEPCTL bit semantics are incomplete
- Selected bits (EPENA/CNAK/SNAK/USBAEP/NAKSTS + coarse masks) are tracked, but RM-defined behavior for many fields is not enforced.

3. DAINT/DAINTMSK and endpoint interrupt summarization are simplified
- Summary behavior is approximated and may not match exact IRQ edge/level behavior for all endpoint combinations.

Missing
1. Full endpoint state machines
- Data PID handling, packet count semantics, NAK/STALL/HALT details, and isochronous/interrupt/bulk differences are not fully implemented.

2. Control transfer completeness
- Address/configuration requests are emulated with canned payloads; full chapter-9 control machinery is absent.

3. Dedicated endpoint/FIFO sizing constraints
- Register programming legality/constraints for FIFO partitioning and endpoint max packet interactions are not fully validated.

Incomplete
1. TXFE/XFRC choreography
- Enough for ChibiOS path, but not generalized for all endpoint traffic patterns.

2. OUT endpoint completion
- OUT path is largely synthetic for EP0 and partial for others.

## Power/Clock block (OTG_FS_PWRCLK)

Implemented (partial)
- PCGCCTL register storage/readback path exists.

Missing / Incomplete
- RM-defined power/clock gating side effects are largely absent.

## CDC / FIFO bridging behavior

Implemented
- FIFO write capture path for EP1 console output with line buffering and periodic flush.

Wrong / Incomplete
- This is transport convenience logic, not hardware behavior.
- Useful for observability, but not equivalent to OTG_FS data path timing and interrupt causality in RM.

## High-priority correction targets
1. Separate “bring-up helper” logic from hardware-accurate mode (feature flag).
2. Expand GINTSTS/GINTMSK source modeling with RM-consistent causes.
3. Replace scripted EP0 setup delivery with generic FIFO/token-driven path.
4. Tighten DIEP/DOEP bit semantics and endpoint transfer state progression.
5. Add legality checks and side effects for key global/device control bits.

## Key file anchors
- src/peripherals/otg_fs.rs:477
- src/peripherals/otg_fs.rs:534
- src/peripherals/otg_fs.rs:566
- src/peripherals/otg_fs.rs:580
- src/peripherals/otg_fs.rs:643
- src/peripherals/otg_fs.rs:883
- src/peripherals/otg_fs.rs:916
