# FEATURE_GAP Unresolved Master TODO

Generated from FEATURE_GAP.md rows where Completion is open or partially implemented.

## High
- [ ] DMA Implement full FIFO direct-mode threshold semantics and FIFO state machine. (partially implemented): FCR FS status field now dynamic (001=busy when EN=1, 100=idle). HT signal uses initial_ndtr > 1 fix. Full threshold underrun/overrun trigger conditions still missing.
- [ ] DMA Implement full DMA interrupt signaling classes with matching status visibility. (partially implemented): TC HT TE DME FE all wired with NVIC pending via stream_irq. signal_mode_error dispatches FE vs DME based on FIFO mode. RM-accurate trigger threshold and burst-error conditions remain edge cases.
- [ ] DMA Implement broader request conflict handling and arbitration for shared requests. (partially implemented): Arbitration now resolves channel-wide stream conflicts (not only identical PAR), preserves read/write full-duplex sharing, and applies mode-error signaling when blocked.
- [ ] DMA Implement full stream priority arbitration semantics. (partially implemented): New stream compares PL against all conflicting owners and preempts lower-priority owners; equal-or-higher owner priority blocks with TE/DME/FE path. RM-complete fairness and dynamic re-arbitration remain.
- [ ] USART Implement CR1 CR2 CR3 behavior beyond stubs. (partially implemented): UE TE RE gating plus CR2 STOP and CR1 parity/word-length aware TX frame timing are modeled; CR3 still persists for DMA and interrupt gating. Hardware-accurate RX framing and full CR3 side effects remain.
- [ ] Minimal OTG FS global and device bring-up. (partially implemented): Synthetic USBRST ENUMDNE SOF and EP0 setup plus ZLP completion modeled; FIFO payload semantics and CDC bridge remain partial.
- [ ] Minimal RCC startup status model. (partially implemented): Ready-bit synthesis and basic CFGR CSR BDCR mirroring remove early clock-init stalls; full clock-tree timing not modeled.
- [ ] Minimal PWR regulator-ready model. (partially implemented): CR CSR semantics with immediate VOSRDY ODRDY ODSWRDY support startup polling.
- [ ] Minimal FLASH ACR model. (partially implemented): ACR and related control registers stored for latency-programming loops.
- [ ] SDIO CMDSENT unblock stub. (partially implemented): CMDSENT and CTIMEOUT paths unblock spin but full SDIO DMA interrupt and data-transfer behavior still stubbed.

## Medium
- [ ] RCC Implement effective bus and clock configuration impacts on peripheral timing. (open): Affects UART baud-rate accuracy and TIM prescaler alignment. Not a gate for CubeBlack boot; reduces timing accuracy.
- [ ] USB OTG FS remains partial for full CDC-accurate endpoint FIFO and interrupt behavior. (partially implemented): Full synthetic enumeration: USBRST ENUMDNE SOF EP0 SETUP ZLP SET_ADDRESS SET_CONFIG CDC ACM handshake. EP1 bulk IN TXFE path captures console output. FIFO threshold semantics and non-enumeration OUT transfer handling remain.
- [ ] USART Implement baud-rate effects from BRR for timing assumptions. (partially implemented): BRR now influences TXE/TC completion latency via bounded delay derived from mantissa/fraction. RX sampling/parity framing timing remains simplified.
- [ ] SPI Implement control semantics for CPOL CPHA frame format and NSS master-slave effects. (partially implemented): CPHA now changes transfer ordering and master+HW-NSS mode-fault path clears SPE with MODF set; deeper CPOL timing and full NSS pin routing remain simplified.
- [ ] SPI Implement SPI error and interrupt signaling paths. (partially implemented): RXNEIE TXEIE ERRIE now raise NVIC pending; OVR and MODF flags modeled with simplified clear behavior.
- [ ] TIM Extend timer coverage beyond current subset based on runtime use. (open): Prioritize runtime-used instances and channels first.
- [ ] TIM Timer DMA request generation paths. (open): Needed where firmware expects DMA-triggered operation.
- [ ] TIM Counting modes preload and slave synchronization behavior. (open): Down center-aligned ARPE and sync behavior still missing.
- [ ] NVIC Implement deeper fault-path semantics and escalation behavior. (open): Current implementation is mostly register storage.
- [ ] NVIC Extend CoreDebug DWT coverage beyond minimal counters and controls. (open): Expand only where firmware consumption proves needed.
- [ ] NVIC Improve NVIC priority and enable arbitration with multiple active sources. (partially implemented): External arbitration now bounds pending-bit iteration to representable IRQ range and uses safe u128 pending-bit shifts; priority competition remains simplified versus full ARM nesting semantics.
- [ ] Broader AZhurGIT meta adoption not complete. (open): Benefits maintainability but is not immediate top runtime blocker.
- [ ] meta adoption remains partial beyond IRQ lookup in I2C and TIM. (open): Broader register-offset migration pending.
- [ ] Ethernet MAC peripheral coverage is still missing versus reference baseline. (open): Coverage gap remains.
- [ ] Minimal TIM5 TIM6 TIM7 timebase. (partially implemented): Provides CR1 DIER SR CNT PSC ARR CCR1 subset, not full advanced timer model.
- [ ] Core SCB state and dynamic VTOR handling. (partially implemented): SCB register state and VTOR-based exception dispatch present, but fault semantics and SHPR arbitration behavior remain limited.
- [ ] Minimal CoreDebug and DWT cycle-counter support. (partially implemented): DEMCR DWT_CTRL DWT_CYCCNT modeled with monotonic cycle counter.
- [ ] DMA stream transfer-complete IRQ signaling to NVIC. (partially implemented): TCIE path now raises STM32F427 stream IRQ; wider request-line and HT TE signaling gaps remain.
- [ ] USART register state persistence expansion. (partially implemented): SR DR BRR CR1 CR2 CR3 GTPR persisted; TXE and TC preserved over init writes; full interrupt and state-machine behavior pending.
- [ ] CoreDebug and DWT coverage breadth remains minimal. (partially implemented): Wider DWT CoreDebug register set and debug-trigger side effects still unmodeled.
- [ ] ADC peripheral stub for ADC1 ADC2 ADC3. (partially implemented): EOC always set and DR synthetic half-scale output; DMA read emits correct halfword bytes.
- [ ] I2C transaction sequencing baseline with board-level hooks. (partially implemented): EV5 EV6 EV8_2 style sequencing plus address-scoped slave hooks and unknown-address NACK; full RM fault and timing fidelity still pending.
- [ ] Timer CCMR1 CCMR2 CCER register storage. (partially implemented): Registers persist and read back, but output-compare mode decode and GPIO toggling remain unimplemented.
- [ ] EXTI peripheral model added and wired. (partially implemented): Core EXTI registers and IRQ fanout modeled, including lines 16 through 22 wake/tamper/RTC routing; SYSCFG line-port mux fidelity and event-only behavior remain simplified.
- [ ] TIM peripheral extended to TIM1 through TIM14 with EGR and CCR2-4. (partially implemented): Update and compare events expanded, with remaining OC mode decoding gaps.

## Low
- [ ] FLASH sector and mass erase not yet tracked previously. (partially implemented): SER and MER plus STRT now launch deferred operations; SNB is decoded and invalid sector numbers flag OPERR. Full backing-flash data mutation is still simplified.
- [ ] FLASH status flags BSY EOP and error bits not fully modeled. (partially implemented): BSY and EOP transition through deferred completion; OPERR is raised on invalid STRT and locked CR writes. Wider error-bit classes remain minimal.
- [ ] FMC FSMC 4-bank abstraction with external routing is missing. (open): Current fsmc model is stub compared to fork reference implementation.
- [ ] LTDC video support remains unimplemented. (open): Not currently required for CubeBlack runtime milestones.
- [ ] TIM Advanced counting modes up down and center-aligned. (open): Current behavior is monotonic software timebase without direction or center-aligned rules.
- [ ] TIM Update compare commutation trigger and break event semantics. (open): SR and DIER behavior remains minimal.
- [ ] TIM Complementary outputs MOE break input and dead-time insertion. (open): Required for advanced PWM motor-control fidelity.
- [ ] TIM Timer synchronization and master-slave trigger chaining. (open): Cross-timer link behavior is not implemented.
- [ ] TIM TIM1 TIM8 DMA request generation. (open): Advanced timer DMA request behavior still missing.
- [ ] TIM TIM1 TIM8 advanced control register coverage for CR2 SMCR EGR CCER BDTR RCR. (partially implemented): CR2 SMCR EGR present; CCER BDTR RCR and complementary-output control remain missing.
- [ ] TIM Channel state for CCR2 CCR3 CCR4 CCMR1 and CCMR2. (partially implemented): CCR2 through CCR4 storage and compare firing are present; CCMR decode and CCER polarity enable remain incomplete.

## Required session tail items
- [ ] git commit your changes, assess all un-comitted changes, not just your recent edits
- [ ] refer to FEATURE_GAP.md afterwards to get more work to do
