# UART/USART Audit (RM0090 Ch.30 Source-of-Truth)

Scope
- Peripheral audited: src/peripherals/usart.rs
- Reference source: STM32F4 RM0090 USART/UART chapter (SR/DR/BRR/CR1/CR2/CR3/GTPR behavior, interrupt and DMA interactions).
- Classification:
  - wrong: contradicts RM behavior
  - missing: absent feature/bit/register path
  - incomplete: partial implementation only

## Executive result
The model provides useful baseline TX/RX register behavior for emulation, but many status flags, error conditions, and side effects are simplified. It is functional for simple serial and DMA paths, not fully RM-accurate.

## Register and flag coverage

Implemented (partial)
- Register map coverage: SR, DR, BRR, CR1, CR2, CR3, GTPR.
- Core TX behavior: DR write clears TXE/TC, delayed completion sets TXE/TC.
- Basic interrupt generation for TXE/TC/RXNE based on CR1 enables.
- DMA hooks through CR3.DMAT/CR3.DMAR for DR access.
- Data width/parity payload masking applied for TX data extraction.

Wrong
1. SR write semantics are incomplete/inexact
- RM defines specific clear sequences for some flags (many require SR then DR read/write sequences, or are hardware-only).
- Current SR write path clears TC and RXNE by direct bit write logic, which is not fully RM-compliant across flag classes.

2. Error/status flag model is missing
- ORE, NE, FE, PE, LBD, CTS, and related status transitions are not modeled.
- As a result, firmware that depends on robust error-path handling is misrepresented.

3. IDLE behavior is simplified
- IDLE set/clear conditions are not modeled with full line-state timing; it is mostly static/cleared on DR read.

4. TX timing formula is heuristic
- BRR and frame fields influence delay, but not with RM-accurate baud-clock timing and oversampling details.

Missing
1. Full CR1/CR2/CR3 bit semantics
- LIN, clocked mode, smartcard, IrDA, half-duplex, wakeup/mute features, hardware flow-control behavior, and many control side effects are absent.

2. Fractional baud and oversampling behavior completeness
- BRR is stored and used heuristically; full baud generator semantics are not implemented.

3. 9-bit/Parity receive-path semantics
- TX masking exists, but receive data/flag interactions for parity/word-length combinations are not complete.

4. Interrupt source completeness
- Only subset of IRQ causes are generated; many RM interrupt paths are absent.

5. GTPR effects
- Register is stored but guard/prescaler behavior for specific modes is not functionally applied.

Incomplete
1. RX path realism
- Data comes from ext_device abstraction; flag transitions and framing/error detection are simplified.

2. DMA interaction realism
- DMA transfers move bytes, but full request pacing and flag race behavior with TXE/TC/RXNE is simplified.

3. UE/TE/RE gating details
- Basic gating exists, but transitional side effects when toggling UE/TE/RE are not fully modeled.

## High-priority correction targets
1. Implement RM-accurate SR flag clear rules per flag class.
2. Add major error flags (ORE/NE/FE/PE) and their set/clear transitions.
3. Expand IRQ cause generation to full CRx/SR matrix used by firmware.
4. Tighten DMA/USART handshakes to reduce race mismatches around TXE/TC/RXNE.
5. Add mode-gating checks and explicit unsupported-mode behavior for LIN/Smartcard/IrDA/HD.

## Key file anchors
- src/peripherals/usart.rs:24
- src/peripherals/usart.rs:130
- src/peripherals/usart.rs:172
- src/peripherals/usart.rs:199
- src/peripherals/usart.rs:251
- src/peripherals/usart.rs:279
