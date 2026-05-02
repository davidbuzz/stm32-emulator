# FLASH Audit (RM0090 FLASH Interface Source-of-Truth)

Scope
- Peripheral audited: src/peripherals/flash.rs
- Reference source: STM32F4 FLASH ACR/SR/CR/OPTCR and erase/program behavior.

## Executive result
Model has useful lock/unlock and deferred BSY/EOP behavior, but operation semantics and error classes remain simplified.

Wrong
1. Operation completion timing is fixed-step synthetic rather than operation-dependent.
2. Partial operation validity checks do not cover full RM constraints.

Missing
1. Real flash memory mutation path for program/erase effects.
2. Full error-bit classes (WRPERR, PGAERR, PGPERR, PGSERR, RDERR) behavior.
3. Option bytes programming and side effects.

Incomplete
1. Sector erase constraints and bank-specific nuances are partial.
2. ACR latency/cache/prefetch interactions are mostly storage.

## High-priority correction targets
1. Tie CR operation requests to real backing flash content changes.
2. Expand SR error-bit generation and clear semantics.
3. Improve operation-duration model by operation type and size.

## Key file anchors
- src/peripherals/flash.rs:52
- src/peripherals/flash.rs:79
- src/peripherals/flash.rs:127
