| Section | Technical Item | Priority | Completion | Notes |
| --- | --- | --- | --- | --- |
| Document Scope | Track emulator features that are incomplete or not yet implemented. | medium | active | This table is the only format used in the document. |
| Deliverable | End goal is CubeBlack boot plus ArduPilot runtime under emulator with realistic emulator-side behavior. | high | active | Avoid reliance on firmware-side startup bypasses. |
| Deliverable | This file is the primary backlog and technical handover for remaining work to reach the deliverable. | high | active | Treat as canonical source of remaining work. |
| Deliverable | Prioritize items that unblock demonstrable CubeBlack boot progress first. | high | active | Follow with correctness and hardware coverage items. |
| Deliverable | Priority scale definition. | medium | active | high means runtime blockers, medium means useful non-immediate blockers, low means defer until proven needed. |
| Recently Fixed | CubeBlack target bring-up assets and board configuration. | medium | implemented | Commit f46e695 introduced board files, firmware artifacts, cubeblack config, run script, and STM32F427 SVD. |
| Recently Fixed | CubeBlack bootloader artifact rebuilt for oscillator configuration change. | medium | implemented | Commit 5797f0f updated cubeblack CubeBlack_bl.hex artifact used by emulator. |
| Recently Fixed | Historical startup bypass retired from active path. | medium | updated | Commit 377a746 had temporary firmware workaround; current progress should be judged on emulator-side models and latest validation evidence. |
| Implemented | CubeBlack memory map updated for CCM RAM and system memory. | medium | implemented | cubeblack config maps RAM-CCM at 0x10000000 and SYSMEM at 0x1FFF0000. |
| Implemented | STM32 unique ID bytes patched into system memory. | medium | implemented | 12-byte UID payload injected at 0x1FFF7A10 via config. |
| Implemented | SPI2 FRAM FM25V02 emulation. | high | implemented | ramtron model plus SPI full-duplex DMA bridge fixes with correct RDID and full 32KB reads. |
| Implemented | DMA global register decode corrected. | medium | implemented | dma model decodes global regs at 0x00..0x0f and streams at 0x10 plus n times 0x18. |
| Implemented | DMA transfer-complete flag and clear semantics. | medium | implemented | lisr and hisr tracking with TCIF set and LIFCR/HIFCR clear behavior. |
| Implemented | NVIC stack-pointer restore and IPSR numbering corrected. | medium | implemented | Restores selected stack pointer and writes architectural exception number 16 plus irq to IPSR. |
| Implemented | Minimal TIM5 TIM6 TIM7 timebase. | medium | partially implemented | Provides CR1 DIER SR CNT PSC ARR CCR1 subset, not full advanced timer model. |
| Implemented | Core SCB state and dynamic VTOR handling. | medium | partially implemented | SCB register state and VTOR-based exception dispatch present, but fault semantics and SHPR arbitration behavior remain limited. |
| Implemented | Minimal CoreDebug and DWT cycle-counter support. | medium | partially implemented | DEMCR DWT_CTRL DWT_CYCCNT modeled with monotonic cycle counter. |
| Implemented | DMA stream transfer-complete IRQ signaling to NVIC. | medium | partially implemented | TCIE path now raises STM32F427 stream IRQ; wider request-line and HT TE signaling gaps remain. |
| Implemented | Minimal OTG FS global and device bring-up. | high | partially implemented | Synthetic USBRST ENUMDNE SOF and EP0 setup plus ZLP completion modeled; FIFO payload semantics and CDC bridge remain partial. |
| Implemented | Exception-frame sizing and BASEPRI-aware IRQ gating corrected. | high | implemented | Fixed late scheduler corruption and removed FETCH_UNMAPPED crash around 10.64M instructions. |
| Implemented | Free-running TIM stepping and TIM5 compare wakeup fixed. | medium | implemented | Timers advance from main loop with prescaler remainder preserved. |
| Implemented | Minimal RCC startup status model. | high | partially implemented | Ready-bit synthesis and basic CFGR CSR BDCR mirroring remove early clock-init stalls; full clock-tree timing not modeled. |
| Implemented | Minimal PWR regulator-ready model. | high | partially implemented | CR CSR semantics with immediate VOSRDY ODRDY ODSWRDY support startup polling. |
| Implemented | Minimal FLASH ACR model. | high | partially implemented | ACR and related control registers stored for latency-programming loops. |
| Implemented | Busy-loop diagnostic capture. | medium | implemented | busy-loop stop now logs PC SP LR R0-R7 context words and restore frame. |
| Implemented | USART probe line flushing improvements. | medium | implemented | Flush on CR and LF and on long buffers for better boot-text visibility. |
| Implemented | USART register state persistence expansion. | medium | partially implemented | SR DR BRR CR1 CR2 CR3 GTPR persisted; TXE and TC preserved over init writes; full interrupt and state-machine behavior pending. |
| Implemented | Deferred external IRQ delivery to block boundary. | high | implemented | Prevents mixed thread-handler execution state and fixes deterministic late crash around 145255239 instructions. |
| Implemented | CubeBlack run script hardened for local environments. | medium | implemented | run script now sources Cargo env when present and falls back to release binary. |
| Handover Blockers | GDB remote debugging support in gdb path and CLI flag. | high | implemented | Remote stub supports attach inspect breakpoints step continue and halt at firmware symbols. |
| Handover Blockers | busy-loop-stop ignores transient idle waits and follows timer wakeups. | high | implemented | Requires sustained repeated-PC streak before halting and now shows TIM5 wake activity. |
| Handover Blockers | Runtime reaches 200M cleanly after deferred IRQ delivery fix. | high | updated | Late crash removed; bounded runs reach 200000000 with no WARN ERROR or unknown peripheral accesses in cited validation. |
| Handover Blockers | USB OTG FS RXFLVL and GRXSTSP enumeration sequence completion. | high | implemented | Full synthetic setup data, setup complete, STUP and XFRC sequence with set-address and set-configuration acknowledgement. |
| Handover Blockers | First ArduPilot console line captured over USB CDC. | high | implemented | EP1 bulk IN via TXFE path captured expected runtime text fragments. |
| Handover Blockers | SPI2 RAMTRON full-duplex DMA bridge. | high | implemented | In-place buffer corruption fixed by RX destination queueing and write_dma exchange semantics. |
| Handover Blockers | SDIO CMDSENT unblock stub. | high | partially implemented | CMDSENT and CTIMEOUT paths unblock spin but full SDIO DMA interrupt and data-transfer behavior still stubbed. |
| Handover Blockers | CubeBlack board-validation sensor stubs. | high | implemented | SPI whoami and MS5611 checks satisfy board validation predicates so startup no longer traps there. |
| Handover Blockers | CoreDebug and DWT coverage breadth remains minimal. | medium | partially implemented | Wider DWT CoreDebug register set and debug-trigger side effects still unmodeled. |
| Handover Blockers | ADC peripheral stub for ADC1 ADC2 ADC3. | medium | partially implemented | EOC always set and DR synthetic half-scale output; DMA read emits correct halfword bytes. |
| Handover Blockers | USART SR realistic RXNE transitions. | low | implemented | RXNE removed from default idle SR and DR read clears RXNE. |
| Handover Blockers | I2C transaction sequencing baseline with board-level hooks. | medium | partially implemented | EV5 EV6 EV8_2 style sequencing plus address-scoped slave hooks and unknown-address NACK; full RM fault and timing fidelity still pending. |
| Handover Blockers | SDIO data timeout reduced for faster failure path. | low | implemented | DATA_TIMEOUT_DELAY_STEPS reduced from 64 to 4. |
| Handover Blockers | DMA conflicting-stream diagnostics downgraded from WARN to DEBUG. | medium | implemented | Behavior preserved while warning noise removed. |
| Handover Blockers | Validation snapshot for current state. | high | updated | Short and long bounded validation cited with no WARN ERROR or unknown peripheral accesses. |
| Handover Blockers | DMA circular mode PINC MINC and NDTR tracking correctness. | high | implemented | initial_ndtr, circular reload, per-beat increment semantics, deferred USART RX transfer, and unified signal_tc are in place. |
| Handover Blockers | Timer CCMR1 CCMR2 CCER register storage. | medium | partially implemented | Registers persist and read back, but output-compare mode decode and GPIO toggling remain unimplemented. |
| Handover Blockers | EXTI peripheral model added and wired. | medium | partially implemented | Core EXTI registers and IRQ fanout modeled; lines 16 through 22 remain unmodeled. |
| Handover Blockers | TIM peripheral extended to TIM1 through TIM14 with EGR and CCR2-4. | medium | partially implemented | Update and compare events expanded, with remaining OC mode decoding gaps. |
| Handover Blockers | FLASH ACR PRFTBS mirrors PRFTBE. | low | implemented | Prefetch status now appears active after prefetch enable. |
| Audit Scope | Peripheral TODO audit is datasheet-backed and targets concrete gaps in src peripherals. | high | active | Reference targets include STM32F4 reference manual datasheet and converted DMA application note. |
| Runtime Priority | P0 work should be completed first for runtime viability. | high | active | Includes DMA mapping and signaling, USART DMA and SR behavior, SPI DMA and SR behavior, I2C transaction semantics, and runtime timer channels and modes. |
| Runtime Priority | P1 work is likely runtime-sensitive but secondary to P0. | medium | active | Includes RCC timing effects, NVIC contention behavior, and broader CoreDebug DWT only when consumed. |
| Runtime Priority | P2 work is deferred until runtime markers are present. | low | active | Includes advanced timer breadth, deeper SCB fault model, and generalized non-critical completeness. |
| DMA Todo | Enforce STM32F427 stream and channel request mapping from DMA reference tables. | high | done | Implemented with permissive fallback for unknown requests. |
| DMA Todo | Peripheral-driven DMA requests for deferred USART RX and SPI full-duplex path. | high | done | Deferred USART RX calls do_xfer and SPI full-duplex handled via dma read and write hooks. |
| DMA Todo | Per-beat NDTR tracking. | high | done | initial_ndtr saved and NDTR behavior differs for circular versus non-circular completion. |
| DMA Todo | Distinct PSIZE and MSIZE handling. | high | done | Transfer paths now use independent peripheral and memory data sizes. |
| DMA Todo | PINC and MINC per-beat increment behavior. | high | done | PINC and MINC handling now modeled per beat. |
| DMA Todo | Circular-mode stream behavior. | high | done | Circular mode keeps EN set and reloads NDTR from initial_ndtr. |
| DMA Todo | Double-buffer mode M0AR M1AR and CT toggle behavior. | high | done | DBM path toggles CT and reloads NDTR, implying circular semantics. |
| DMA Todo | Implement full FIFO direct-mode threshold semantics and FIFO state machine. | high | open | Basic chunk granularity implemented; full FIFO level status and error-cause fidelity still missing. |
| DMA Todo | Implement full DMA interrupt signaling classes with matching status visibility. | high | open | TC HT TE DME FE class paths wired; RM-accurate trigger conditions remain incomplete. |
| DMA Todo | Implement broader request conflict handling and arbitration for shared requests. | high | open | Current arbitration handles duplicate channel and PAR conflicts only. |
| DMA Todo | Implement full stream priority arbitration semantics. | high | open | Initial PL-based preemption exists but wider cross-stream arbitration is incomplete. |
| DMA Todo | Implement STM32F4-accurate EN disable and re-enable sequencing timing. | high | open | Re-trigger blocking and EN-clear behavior improved but full timing fidelity still pending. |
| USART Todo | Implement CR1 CR2 CR3 behavior beyond stubs. | high | open | Needs UE TE RE stop bits parity and interrupt-enable semantics. |
| USART Todo | Implement realistic SR transitions. | high | open | Needs TXE TC RXNE IDLE and error-flag state machine behavior. |
| USART Todo | Implement UART USART DMA coupling via DMAT and DMAR in CR3. | high | done | DMAT DMAR gated DMA flow with batched ext-device helpers validated in 120M run. |
| USART Todo | Implement UART USART interrupt generation and clearing rules. | high | open | TXE RXNE TC and error paths still pending. |
| USART Todo | Implement baud-rate effects from BRR for timing assumptions. | medium | open | Timing-sensitive firmware may depend on BRR effects. |
| SPI Todo | Implement stateful SPI status flags instead of synthetic toggles. | high | open | Needs TXE RXNE BSY and OVR MODF paths. |
| SPI Todo | Implement SPI DMA request generation for RX and TX. | high | open | Required for realistic sensor traffic behavior. |
| SPI Todo | Implement control semantics for CPOL CPHA frame format and NSS master-slave effects. | medium | open | Needed by CubeBlack peripheral behavior. |
| SPI Todo | Implement SPI error and interrupt signaling paths. | medium | open | Needs RXNE TXE and ERR interrupt behavior. |
| I2C Todo | Replace toggled SR1 SR2 stubs with transaction state machine. | high | done | Start address data stop progression modeled with ADDR clear sequencing. |
| I2C Todo | Implement key status and control semantics and clear ordering. | high | done | SB ADDR BTF RXNE TXE AF modeled with simplified broader fault fidelity. |
| I2C Todo | Implement I2C DMA request generation and interrupt paths. | high | done | CR2 DMAEN gating and BTF event scheduling implemented; request timing fidelity remains partial. |
| I2C Todo | Add board-level I2C device hooks for ArduPilot sensor bring-up paths. | high | done | Address-scoped models with register-pointer reads and writes and unknown-address NACK behavior are in place. |
| TIM Todo | Extend timer coverage beyond current subset based on runtime use. | medium | open | Prioritize runtime-used instances and channels first. |
| TIM Todo | Additional CCR2 CCR3 CCR4 channels. | medium | done | Stored and compared with known CCMR and CCER decode gaps. |
| TIM Todo | Event generation register force-update and CC-event bits. | medium | done | EGR support added. |
| TIM Todo | CCMR1 CCMR2 CCER readback support. | medium | done | OC mode decoding still pending. |
| TIM Todo | Timer DMA request generation paths. | medium | open | Needed where firmware expects DMA-triggered operation. |
| TIM Todo | Counting modes preload and slave synchronization behavior. | medium | open | Down center-aligned ARPE and sync behavior still missing. |
| RCC and Clocking Todo | Replace always-ready RCC behavior with stateful transitions for CR CFGR and oscillator and PLL paths. | high | open | Remove hidden firmware bypass dependence. |
| RCC and Clocking Todo | Implement effective bus and clock configuration impacts on peripheral timing. | high | open | Affects UART TIM and DMA pacing assumptions. |
| SCB NVIC CoreDebug Todo | Implement deeper fault-path semantics and escalation behavior. | medium | open | Current implementation is mostly register storage. |
| SCB NVIC CoreDebug Todo | Extend CoreDebug DWT coverage beyond minimal counters and controls. | medium | open | Expand only where firmware consumption proves needed. |
| SCB NVIC CoreDebug Todo | Improve NVIC priority and enable arbitration with multiple active sources. | medium | open | Needed for realistic exception competition behavior. |
| Reference Reuse | Best reference for DMA request-line triggered transfer model. | medium | cataloged | Renode STM32DMA.cs uses OnGPIO, PerformTransfer, and CreateRequest patterns. |
| Reference Reuse | Best reference for DMA FIFO and direct-mode transfer-size logic. | medium | cataloged | Renode STM32DMA.cs GetCurrentTransferSize pattern. |
| Reference Reuse | Best reference for DMA PINC MINC per-beat increment logic. | medium | cataloged | Renode STM32DMA.cs CreateRequest increment handling. |
| Reference Reuse | Best reference for timer separate CCR compare timers. | medium | cataloged | Renode STM32_Timer.cs ccTimers construction and event handlers. |
| Reference Reuse | Best reference for timer output-compare modes including PWM variants. | medium | cataloged | Renode STM32_Timer.cs compare handlers implement five OC modes. |
| Reference Reuse | Best reference for repetition counter behavior in advanced timers. | medium | cataloged | Renode STM32_Timer.cs applies repetition counter before update IRQ. |
| Reference Reuse | Best reference for EXTI direct and configurable line routing. | medium | cataloged | Renode STM32F4_EXTI.cs OnGPIO behavior. |
| Reference Reuse | Best reference for FLASH sector erase and mass erase logic. | low | cataloged | Renode STM32F4_FlashController.cs Erase path with control bits. |
| Reference Reuse | Best reference for FLASH lock key mechanism. | low | cataloged | Renode flash controller KEYR unlock sequence pattern. |
| Reference Reuse | Best reference for FLASH option-bytes region handling. | low | cataloged | Renode flash controller dual address-space model. |
| Reference Reuse | Best reference for FLASH status register flags. | low | cataloged | Renode flash controller status bits model. |
| Reference Reuse | Best reference for FMC bank abstraction and external-device routing. | medium | cataloged | goran-mahovlic fmc model has bank plus ext-device connector pattern. |
| Reference Reuse | Best reference for SVD-driven register and IRQ lookup meta layer. | medium | cataloged | AZhurGIT meta layer includes from_svd and irq_of support. |
| Reference Reuse | Best reference for RCC ready-bit auto-sync pattern. | medium | cataloged | AZhurGIT rcc update_ready_bits approach mirrored in current rcc model. |
| Audit Revealed | FLASH sector and mass erase not yet tracked previously. | low | open | Missing SER MER SNB STRT behavior can block firmware erase flows. |
| Audit Revealed | FLASH lock unlock key sequence not fully modeled. | low | open | Locked hardware should ignore writes until correct key sequence. |
| Audit Revealed | FLASH status flags BSY EOP and error bits not fully modeled. | low | open | Firmware polling BSY may stall without accurate status behavior. |
| Audit Revealed | FMC FSMC 4-bank abstraction with external routing is missing. | low | open | Current fsmc model is stub compared to fork reference implementation. |
| Audit Revealed | Broader AZhurGIT meta adoption not complete. | medium | open | Benefits maintainability but is not immediate top runtime blocker. |
| Migration Status | This file is canonical backlog after migration from former comparison write-up. | medium | done | Old comparison content migrated into this table. |
| Migration Open | USB OTG FS remains partial for full CDC-accurate endpoint FIFO and interrupt behavior. | high | open | Further USB behavior depth still needed. |
| Migration Open | meta adoption remains partial beyond IRQ lookup in I2C and TIM. | medium | open | Broader register-offset migration pending. |
| Migration Open | Ethernet MAC peripheral coverage is still missing versus reference baseline. | medium | open | Coverage gap remains. |
| Migration Open | LTDC video support remains unimplemented. | low | open | Not currently required for CubeBlack runtime milestones. |
| Migration Completed | TIM model coverage now includes TIM1 through TIM14 with update and CC event handling. | medium | done | Core timer breadth significantly expanded. |
| Migration Completed | EXTI peripheral model is implemented and wired. | medium | done | Basic EXTI interrupt model present. |
| Migration Completed | FLASH control and register behavior goes beyond simple ACR stubs. | medium | done | Expanded flash model now present. |
| Migration Completed | DMA performs real memory movement rather than register-only completion. | high | done | mem_read and mem_write transfer behavior implemented. |
| Migration Completed | PWR and ADC peripheral models are present. | medium | done | Baseline peripheral coverage expanded. |
| Migration Completed | Bit-band alias support includes peripheral and SRAM alias mappings. | medium | done | Both alias regions are covered. |
| Migration Completed | Missing-peripheral gap stubs for CAN RTC IWDG RNG and CRC are wired in main registry. | medium | done | Registration and basic models are present. |
| Advanced TIM | TIM1 TIM8 advanced control register coverage for CR2 SMCR EGR CCER BDTR RCR. | low | partially implemented | CR2 SMCR EGR present; CCER BDTR RCR and complementary-output control remain missing. |
| Advanced TIM | Channel state for CCR2 CCR3 CCR4 CCMR1 and CCMR2. | low | partially implemented | CCR2 through CCR4 storage and compare firing are present; CCMR decode and CCER polarity enable remain incomplete. |
| Advanced TIM | Advanced counting modes up down and center-aligned. | low | open | Current behavior is monotonic software timebase without direction or center-aligned rules. |
| Advanced TIM | Update compare commutation trigger and break event semantics. | low | open | SR and DIER behavior remains minimal. |
| Advanced TIM | Complementary outputs MOE break input and dead-time insertion. | low | open | Required for advanced PWM motor-control fidelity. |
| Advanced TIM | Timer synchronization and master-slave trigger chaining. | low | open | Cross-timer link behavior is not implemented. |
| Advanced TIM | TIM1 TIM8 DMA request generation. | low | open | Advanced timer DMA request behavior still missing. |
