# DMA (DMA1/DMA2) Compliance Audit vs STM32F4 RM0090

## Scope and Source of Truth
- Code audited: src/peripherals/dma.rs
- Primary spec: RM0090 Chapter 10 (DMA), especially register map/bit behavior and Table 42/43 request mapping.
- Supplemental spec: AN4031 (STM32F2/F4/F7 DMA controller behavior, enable/disable sequences, FIFO/direct mode constraints, arbitration notes).

## What Is Already Implemented Correctly (or mostly correctly)
- Global register decode and stream window layout follow the STM32F4 register map shape (LISR/HISR/LIFCR/HIFCR + 8 streams at 0x10 + n*0x18).
- Main status classes are modeled: TCIF, HTIF, TEIF, DMEIF, FEIF.
- Interrupt gating by TCIE/HTIE/TEIE/DMEIE/FEIE is present.
- Direct-mode burst restriction check exists (PBURST/MBURST invalid when DMDIS=0).
- Circular mode and DBM CT bit toggling are partially modeled.
- Basic stream/channel request mapping checks exist for many CubeBlack-relevant peripherals.

## Incorrect Behavior vs RM0090/AN4031

### 1) Flags are cleared with raw bitmask inversion instead of IFCR semantics
- Code: LIFCR/HIFCR handling uses lisr &= !value and hisr &= !value.
- Problem: IFCR bits are not positional aliases of SR registers for all flags in a naive linear way; they are dedicated clear bits with group layouts. Clearing should decode CFEIFx/CDMEIFx/CTEIFx/CHTIFx/CTCIFx fields and clear corresponding status bits exactly.
- Impact: firmware can clear unintended bits or fail to clear expected bits, causing wait loops and IRQ storms.

### 2) Half-transfer event generation is wrong
- Code emits HT whenever initial_ndtr > 1.
- Problem: HTIF should assert when half of programmed data has actually transferred (exact milestone semantics), not just on any multi-beat completion.
- Impact: premature HT IRQs and incorrect driver state transitions.

### 3) NDTR semantics are incorrect for non-circular transfers
- Code often performs all beats in one shot and then forces ndtr = 0 immediately.
- Problem: NDTR must decrement as transfers occur and reflect in-progress residue. In peripheral-paced transfers, depletion is request-driven, not instant completion at EN set.
- Impact: polling code reading NDTR sees unrealistic behavior and timing.

### 4) FIFO status and threshold behavior is not RM-compliant
- Code computes FS from synthetic chunk math and internal counters, with threshold checks based on a custom interpretation.
- Problem: STM32 FIFO is fixed-depth 4-word with specific FS state transitions and FEIF conditions. Current model is heuristic and can report wrong FS/FEIF states.
- Impact: drivers using FCR/FS/FEIF for flow control can misbehave.

### 5) Request conflict handling does not match hardware arbitration
- Code preempts other enabled streams based on custom priority rule and same-peripheral heuristic.
- Problem: hardware stream arbitration does not disable peer streams this way; conflicts and priorities are handled by DMA arbiter and request routing constraints.
- Impact: non-hardware side effects, hidden regressions across peripherals.

### 6) Forced SPI TC interrupt behavior is non-hardware
- Code can force TC IRQ on SPI DR descriptor regardless of strict TCIE path.
- Problem: IRQ generation should be strictly according to DMA interrupt enable bits and status events.
- Impact: false-positive interrupts, masking real configuration bugs.

### 7) Disable/enable timing model is ad-hoc
- Code uses fixed instruction delays (DMA_EN_DISABLE_DELAY) and next_cr toggles, including firmware-specific hacks for NDTR=0.
- Problem: STM32 EN clear/set sequencing has precise visible semantics; current behavior mixes useful workaround logic with non-portable hacks.
- Impact: behavior divergence from RM, hard-to-debug corner cases.

## Missing or Incomplete DMA Features

### A) PFCTRL (flow controller) semantics not implemented
- No full support for peripheral flow controller behavior and its constraints on NDTR/transfer pacing.

### B) True peripheral-request pacing
- Transfers are largely executed as bursty software copies, not one request per peripheral event for many modes.

### C) Double-buffer restrictions and runtime rules
- DBM constraints (e.g., legal mode combinations, safe CT switching timing, register write constraints while EN=1) are only partially enforced.

### D) Register write restrictions while stream enabled
- RM imposes which fields are writable only when EN=0 (many SxCR/SxNDTR/SxPAR/SxM0AR/SxM1AR/FCR details).
- Current code allows broad writes without full enforcement.

### E) Complete request matrix coverage
- request_mapping_allows includes many mappings but is incomplete relative to full RM tables.
- Unsupported requests are permissive by default, which deviates from strict hardware behavior.

### F) Direct mode corner cases and FE/DME signaling nuance
- DMEIF/FEIF generation and interaction with DMDIS/FTH/MBURST/PBURST is simplified.

### G) Bus errors / alignment / size constraints
- RM/AN4031 constraints (data alignment, legal NDT versus PSIZE/MSIZE combinations, burst packing behavior) are only partially represented.

### H) FIFO drain/fill and memory/peripheral port concurrency
- Hardware dual-port behavior, arbitration latency, and partial FIFO transactions are approximated but not cycle-faithful.

## High-Confidence Fix Priorities
1. Implement exact IFCR-to-ISR flag-clear decoding.
2. Rework HTIF to trigger at real half-transfer threshold, not post-hoc heuristic.
3. Make NDTR decrement per actual transfer beat/request and expose intermediate values.
4. Enforce write-while-EN restrictions per stream register/field.
5. Remove non-hardware stream preemption logic and replace with realistic arbiter behavior.
6. Tighten request mapping to RM tables with explicit reject paths for invalid combinations.

## Code Anchors Reviewed
- src/peripherals/dma.rs:211
- src/peripherals/dma.rs:214
- src/peripherals/dma.rs:465
- src/peripherals/dma.rs:552
- src/peripherals/dma.rs:827
- src/peripherals/dma.rs:831
- src/peripherals/dma.rs:836
- src/peripherals/dma.rs:939
