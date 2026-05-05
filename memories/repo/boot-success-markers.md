## CubeBlack bounded-run success markers

- Use a bounded CubeBlack run as the quick validation probe:
  - `cd cubeblack && ../target/release/stm32-emulator config.yaml --max-instructions 20000000`
- Expected ordered marker subsequence in emulator output:
  - `[clk=00467227 pc=0x0815bc4a] INFO  OTG_FS: SetLineCoding ZLP done -> DeliverSetControlLineState`
  - `[clk=10645971 pc=0x080f27f8] INFO  TRACE AP_Vehicle::setup reached pc=0x08082f48`
  - `[clk=10659256 pc=0x08138ca6] INFO  TRACE AP_RAMTRON::read transfer return reached pc=0x08138ca8`
- Expected follow-on runtime marker:
  - output should later contain a line with at least `ArduCopter`
- Purpose:
  - Treat the three trace lines as proof that USB CDC enumeration, vehicle setup, and RAMTRON-backed parameter read all progressed in the same bounded run.
  - Treat an `ArduCopter` console line as the next user-visible runtime milestone after those setup markers.