# STM32F427xx STM32F429xx Datasheet - 32-bit Arm Cortex-M4 MCU

> Converted from PDF. Source: `stm32f427vg-datasheet.pdf`

---


<!-- Page 1 -->


|  |  |
| --- | --- |
|  |  |

STM32F427xx STM32F429xx
® ®
32b Arm Cortex -M4 MCU+FPU, 225DMIPS, up to 2MB flash/256+4KB RAM, USB
OTG HS/FS, Ethernet, 17 TIMs, 3 ADCs, 20 com. interfaces, camera and LCD-TFT
Datasheet - production data
Features
• Includes ST state-of-the-art patented
technology.
• Core: Arm® 32-bit Cortex®-M4 CPU with FPU,
Adaptive real-time accelerator (ART
Accelerator™) allowing 0-wait state execution LQFP100 (14 × 14 mm) UFBGA176 (10 x 10 mm)
from flash memory, frequency up to 180MHz, L L Q Q F F P P 1 1 4 7 4 6 ( ( 2 2 0 4 × × 2 2 0 4 m m m m ) ) UFBGA169 (7 × 7 mm) WLCSP143
TFBGA216 (13 x 13 mm)
MPU, 225DMIPS/1.25DMIPS/MHz LQFP208 (28 x 28 mm)
(Dhrystone 2.1), and DSP instructions
bit timers up to 180MHz, each with up to
• Memories four IC/OC/PWM or pulse counter and
– 512 bytes of OTP memory quadrature (incremental) encoder input
– Up to 2 MB of flash memory organized into
• Debug mode
two banks allowing read-while-write
– Up to 256+4KB of SRAM including 64 KB – SWD & JTAG interfaces
of CCM (core coupled memory) data RAM – Cortex®-M4 Trace Macrocell™
– Flexible external memory controller with up • Up to 168 I/O ports with interrupt capability
to 32-bit data bus: SRAM, PSRAM,
– Up to 164 fast I/Os up to 90MHz
SDRAM/LPSDR SDRAM, compact
– Up to 166 5V-tolerant I/Os
flash/NOR/NAND memories
• LCD parallel interface, 8080/6800 modes • Up to 21 communication interfaces
• LCD-TFT controller with fully programmable – Up to 3 × I2C interfaces (SMBus/PMBus)
– Up to four USARTs/4 UARTs (11.25Mbit/s,
resolution (total width up to 4096 pixels, total
ISO7816 interface, LIN, IrDA, modem
height up to 2048 lines and pixel clock up to
control)
83MHz)
– Up to 6 SPIs (45Mbit/s), 2 with muxed full-
• Chrom-ART Accelerator™ for enhanced duplex I2S for audio class accuracy via
graphic content creation (DMA2D) internal audio PLL or external clock
• Clock, reset, and supply management – 1 x SAI (serial audio interface)
– 2 × CAN (2.0B active) and SDIO interface
– 1.7V to 3.6V application supply and I/Os
• Advanced connectivity
– POR, PDR, PVD, and BOR
– USB 2.0 full-speed device/host/OTG
– 4-to-26MHz crystal oscillator
controller with on-chip PHY
– Internal 16MHz factory-trimmed RC (1%
– USB 2.0 high-speed/full-speed
accuracy)
device/host/OTG controller with dedicated
– 32 kHz oscillator for RTC with calibration
DMA, on-chip full-speed PHY and ULPI
– Internal 32 kHz RC with calibration
– 10/100 Ethernet MAC with dedicated DMA:
• Low power supports IEEE 1588v2 hardware, MII/RMII
– Sleep, Stop, and Standby modes • 8- to 14-bit parallel camera interface up to
– V supply for RTC, 20×32-bit backup 54Mbytes/s
BAT
registers + optional 4 KB backup SRAM • True random number generator
• 3×12-bit, 2.4MSPS ADC: up to 24 channels • CRC calculation unit
and 7.2MSPS in triple interleaved mode
• RTC: subsecond accuracy, hardware calendar
• 2×12-bit D/A converters
• 96-bit unique ID.
• General-purpose DMA: 16-stream DMA
controller with FIFOs and burst support • ECOPACK2 compliant packages.
• Up to 17 timers: up to twelve 16-bit and two 32-
February 2026 DS9405 Rev 13 1/240
This is information on a product in full production. www.st.com

<!-- Page 2 -->


| Reference | Part number |
| --- | --- |
| STM32F427xx | STM32F427VG, STM32F427ZG, STM32F427IG, STM32F427AG, STM32F427VI, STM32F427ZI, STM32F427II, STM32F427AI |
| STM32F429xx | STM32F429VG, STM32F429ZG, STM32F429IG, STM32F429BG, STM32F429NG, STM32F429AG, STM32F429VI, STM32F429ZI, STM32F429II,, STM32F429BI, STM32F429NI, STM32F429AI, STM32F429VE, STM32F429ZE, STM32F429IE, STM32F429BE, STM32F429NE |

STM32F427xx STM32F429xx
• Table 1. Device summary
Reference Part number
STM32F427VG, STM32F427ZG, STM32F427IG, STM32F427AG, STM32F427VI, STM32F427ZI,
STM32F427xx
STM32F427II, STM32F427AI
STM32F429VG, STM32F429ZG, STM32F429IG, STM32F429BG, STM32F429NG,
STM32F429xx STM32F429AG, STM32F429VI, STM32F429ZI, STM32F429II,, STM32F429BI, STM32F429NI,
STM32F429AI, STM32F429VE, STM32F429ZE, STM32F429IE, STM32F429BE, STM32F429NE
2/240 DS9405 Rev 13

<!-- Page 3 -->

STM32F427xx STM32F429xx Contents
Contents
1 Introduction . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 13
2 Description . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 14
2.1 Full compatibility throughout the family . . . . . . . . . . . . . . . . . . . . . . . . . . 18
3 Functional overview . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 21
3.1 Arm® Cortex®-M4 with FPU and embedded flash and SRAM . . . . . . . . . 21
3.2 Adaptive real-time memory accelerator (ART Accelerator™) . . . . . . . . . 21
3.3 Memory protection unit . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 21
3.4 Embedded flash memory . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 22
3.5 CRC (cyclic redundancy check) calculation unit . . . . . . . . . . . . . . . . . . . 22
3.6 Embedded SRAM . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 22
3.7 Multi-AHB bus matrix . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 22
3.8 DMA controller (DMA) . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 23
3.9 Flexible memory controller (FMC) . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 24
3.10 LCD-TFT controller (available only on STM32F429xx) . . . . . . . . . . . . . . 24
3.11 Chrom-ART Accelerator™ (DMA2D) . . . . . . . . . . . . . . . . . . . . . . . . . . . . 25
3.12 Nested vectored interrupt controller (NVIC) . . . . . . . . . . . . . . . . . . . . . . . 25
3.13 External interrupt/event controller (EXTI) . . . . . . . . . . . . . . . . . . . . . . . . . 25
3.14 Clocks and startup . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 25
3.15 Boot modes . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 26
3.16 Power supply schemes . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 26
3.17 Power supply supervisor . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 26
3.17.1 Internal reset ON . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 26
3.17.2 Internal reset OFF . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 27
3.18 Voltage regulator . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 28
3.18.1 Regulator ON . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 28
3.18.2 Regulator OFF . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 29
3.18.3 Regulator ON/OFF and internal reset ON/OFF availability . . . . . . . . . . 32
3.19 Real-time clock (RTC), backup SRAM, and backup registers . . . . . . . . . 32
3.20 Low-power modes . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 33
3.21 V operation . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 34
BAT
DS9405 Rev 13 3/240
6

<!-- Page 4 -->

Contents STM32F427xx STM32F429xx
3.22 Timers and watchdogs . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 34
3.22.1 Advanced-control timers (TIM1, TIM8) . . . . . . . . . . . . . . . . . . . . . . . . . 36
3.22.2 General-purpose timers (TIMx) . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 36
3.22.3 Basic timers TIM6 and TIM7 . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 36
3.22.4 Independent watchdog . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 37
3.22.5 Window watchdog . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 37
3.22.6 SysTick timer . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 37
3.23 Inter-integrated circuit interface ( I2C) . . . . . . . . . . . . . . . . . . . . . . . . . . . 37
3.24 Universal synchronous/asynchronous receiver transmitters (USART) . . 37
3.25 Serial peripheral interface (SPI) . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 38
3.26 Inter-integrated sound (I2S) . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 39
3.27 Serial Audio interface (SAI1) . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 39
3.28 Audio PLL (PLLI2S) . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 39
3.29 Audio and LCD PLL(PLLSAI) . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 39
3.30 Secure digital input/output interface (SDIO) . . . . . . . . . . . . . . . . . . . . . . . 40
3.31 Ethernet MAC interface with dedicated DMA and IEEE 1588 support . . . 40
3.32 Controller area network (bxCAN) . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 40
3.33 Universal serial bus on-the-go full-speed (OTG_FS) . . . . . . . . . . . . . . . . 41
3.34 Universal serial bus on-the-go high-speed (OTG_HS) . . . . . . . . . . . . . . . 41
3.35 Digital camera interface (DCMI) . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 42
3.36 True random number generator (RNG) . . . . . . . . . . . . . . . . . . . . . . . . . . 42
3.37 General-purpose input/outputs (GPIOs) . . . . . . . . . . . . . . . . . . . . . . . . . . 42
3.38 Analog-to-digital converters (ADCs) . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 42
3.39 Temperature sensor . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 43
3.40 Digital-to-analog converter (DAC) . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 43
3.41 Serial wire JTAG debug port (SWJ-DP) . . . . . . . . . . . . . . . . . . . . . . . . . . 43
3.42 Embedded Trace Macrocell™ . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 44
4 Pinouts and pin description . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 45
5 Memory mapping . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 86
6 Electrical characteristics . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 91
6.1 Parameter conditions . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 91
6.1.1 Minimum and maximum values . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 91
4/240 DS9405 Rev 13

<!-- Page 5 -->

STM32F427xx STM32F429xx Contents
6.1.2 Typical values . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 91
6.1.3 Typical curves . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 91
6.1.4 Loading capacitor . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 91
6.1.5 Pin input voltage . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 91
6.1.6 Power supply scheme . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 92
6.1.7 Current consumption measurement . . . . . . . . . . . . . . . . . . . . . . . . . . . 93
6.2 Absolute maximum ratings . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 93
6.3 Operating conditions . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 95
6.3.1 General operating conditions . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 95
6.3.2 VCAP1/VCAP2 external capacitor . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 97
6.3.3 Operating conditions at power-up / power-down (regulator ON) . . . . . . 98
6.3.4 Operating conditions at power-up / power-down (regulator OFF) . . . . . 98
6.3.5 Reset and power control block characteristics . . . . . . . . . . . . . . . . . . . 99
6.3.6 Overdrive switching characteristics . . . . . . . . . . . . . . . . . . . . . . . . . . . 100
6.3.7 Supply current characteristics . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 101
6.3.8 Wake-up time from low-power modes . . . . . . . . . . . . . . . . . . . . . . . . . 117
6.3.9 External clock source characteristics . . . . . . . . . . . . . . . . . . . . . . . . . . 118
6.3.10 Internal clock source characteristics . . . . . . . . . . . . . . . . . . . . . . . . . . 122
6.3.11 PLL characteristics . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 124
6.3.12 PLL spread spectrum clock generation (SSCG) characteristics . . . . . 127
6.3.13 Memory characteristics . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 129
6.3.14 EMC characteristics . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 131
6.3.15 Absolute maximum ratings (electrical sensitivity) . . . . . . . . . . . . . . . . 133
6.3.16 I/O current injection characteristics . . . . . . . . . . . . . . . . . . . . . . . . . . . 134
6.3.17 I/O port characteristics . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 135
6.3.18 NRST pin characteristics . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 141
6.3.19 TIM timer characteristics . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 142
6.3.20 Communications interfaces . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 142
6.3.21 12-bit ADC characteristics . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 158
6.3.22 Temperature sensor characteristics . . . . . . . . . . . . . . . . . . . . . . . . . . . 164
6.3.23 V monitoring characteristics . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 165
BAT
6.3.24 Reference voltage . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 165
6.3.25 DAC electrical characteristics . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 166
6.3.26 FMC characteristics . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 169
6.3.27 Camera interface (DCMI) timing specifications . . . . . . . . . . . . . . . . . . 193
6.3.28 LCD-TFT controller (LTDC) characteristics . . . . . . . . . . . . . . . . . . . . . 194
6.3.29 SD/SDIO MMC card host interface (SDIO) characteristics . . . . . . . . . 196
DS9405 Rev 13 5/240
6

<!-- Page 6 -->

Contents STM32F427xx STM32F429xx
6.3.30 RTC characteristics . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 197
7 Package information . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 198
7.1 Device marking . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 198
7.2 LQFP100 package information (1L) . . . . . . . . . . . . . . . . . . . . . . . . . . . . 199
7.3 WLCSP143 package information . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 202
7.3.1 Device marking for WLCSP143 . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 204
7.4 LQFP144 package information (1A) . . . . . . . . . . . . . . . . . . . . . . . . . . . . 205
7.5 LQFP176 package information (1T) . . . . . . . . . . . . . . . . . . . . . . . . . . . . 209
7.6 LQFP208 package information . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 213
7.7 UFBGA169 package information (A0YV) . . . . . . . . . . . . . . . . . . . . . . . . 216
7.8 UFBGA(176+25) package information (A0E7) . . . . . . . . . . . . . . . . . . . . 219
7.9 TFBGA216 package information (A0L2) . . . . . . . . . . . . . . . . . . . . . . . . 221
7.10 Thermal characteristics . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 224
8 Ordering information . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 225
Appendix A Recommendations when using internal reset OFF . . . . . . . . . . . 226
A.1 Operating conditions. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 226
Appendix B Application block diagrams . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 227
B.1 USB OTG full speed (FS) interface solutions . . . . . . . . . . . . . . . . . . . . . 227
B.2 USB OTG high speed (HS) interface solutions. . . . . . . . . . . . . . . . . . . . 229
B.3 Ethernet interface solutions. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 230
9 Important security notice . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 232
10 Revision history . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 233
6/240 DS9405 Rev 13

<!-- Page 7 -->

STM32F427xx STM32F429xx List of tables
List of tables
Table 1. Device summary. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 2
Table 2. STM32F427xx and STM32F429xx features and peripheral counts . . . . . . . . . . . . . . . . . . 16
Table 3. Voltage regulator configuration mode versus device operating mode . . . . . . . . . . . . . . . . 29
Table 4. Regulator ON/OFF and internal reset ON/OFF availability. . . . . . . . . . . . . . . . . . . . . . . . . 32
Table 5. Voltage regulator modes in stop mode. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 33
Table 6. Timer feature comparison. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 35
Table 7. Comparison of I2C analog and digital filters. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 37
Table 8. USART feature comparison . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 38
Table 9. Legend/abbreviations used in the pinout table. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 53
Table 10. STM32F427xx and STM32F429xx pin and ball definitions . . . . . . . . . . . . . . . . . . . . . . . . 53
Table 11. FMC pin definition. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 72
Table 12. STM32F427xx and STM32F429xx alternate function mapping . . . . . . . . . . . . . . . . . . . . . 75
Table 13. STM32F427xx and STM32F429xx register boundary addresses. . . . . . . . . . . . . . . . . . . . 87
Table 14. Voltage characteristics . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 93
Table 15. Current characteristics . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 94
Table 16. Thermal characteristics. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 94
Table 17. General operating conditions . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 95
Table 18. Limitations depending on the operating power supply range. . . . . . . . . . . . . . . . . . . . . . . 97
Table 19. VCAP1/VCAP2 operating conditions . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 97
Table 20. Operating conditions at power-up / power-down (regulator ON) . . . . . . . . . . . . . . . . . . . . 98
Table 21. Operating conditions at power-up / power-down (regulator OFF). . . . . . . . . . . . . . . . . . . . 98
Table 22. Reset and power control block characteristics . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 99
Table 23. Over-drive switching characteristics. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 100
Table 24. Typical and maximum current consumption in Run mode, code with data processing
running from Flash memory (ART accelerator enabled except prefetch) or RAM. . . . . . 102
Table 25. Typical and maximum current consumption in Run mode, code with data processing
running from Flash memory (ART accelerator disabled). . . . . . . . . . . . . . . . . . . . . . . . . 103
Table 26. Typical and maximum current consumption in Sleep mode. . . . . . . . . . . . . . . . . . . . . . . 104
Table 27. Typical and maximum current consumptions in Stop mode. . . . . . . . . . . . . . . . . . . . . . . 105
Table 28. Typical and maximum current consumptions in Standby mode . . . . . . . . . . . . . . . . . . . . 106
Table 29. Typical and maximum current consumptions in V mode. . . . . . . . . . . . . . . . . . . . . . . 106
BAT
Table 30. Typical current consumption in Run mode, code with data processing running from
Flash memory or RAM, regulator ON (ART accelerator enabled except prefetch),
VDD=1.7 V . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 108
Table 31. Typical current consumption in Run mode, code with data processing running
from Flash memory, regulator OFF (ART accelerator enabled except prefetch). . . . . . . 109
Table 32. Typical current consumption in Sleep mode, regulator ON, VDD=1.7 V . . . . . . . . . . . . . 110
Table 33. Tyical current consumption in Sleep mode, regulator OFF. . . . . . . . . . . . . . . . . . . . . . . . 111
Table 34. Switching output I/O current consumption . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 113
Table 35. Peripheral current consumption . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 114
Table 36. Low-power mode wakeup timings . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 117
Table 37. High-speed external user clock characteristics. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 118
Table 38. Low-speed external user clock characteristics. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 119
Table 39. HSE 4-26 MHz oscillator characteristics . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 120
Table 40. LSE oscillator characteristics (f = 32.768 kHz) . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 121
LSE
Table 41. HSI oscillator characteristics . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 122
Table 42. LSI oscillator characteristics . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 123
Table 43. Main PLL characteristics. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 124
DS9405 Rev 13 7/240
9

<!-- Page 8 -->

List of tables STM32F427xx STM32F429xx
Table 44. PLLI2S (audio PLL) characteristics . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 125
Table 45. PLLISAI (audio and LCD-TFT PLL) characteristics . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 126
Table 46. SSCG parameters constraint . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 127
Table 47. Flash memory characteristics. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 129
Table 48. Flash memory programming. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 129
Table 49. Flash memory programming with V
PP . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .130
Table 50. Flash memory endurance and data retention. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 131
Table 51. EMS characteristics . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 131
Table 52. EMI characteristics for fHSE= 25 MHz and fCPU= 168 MHz. . . . . . . . . . . . . . . . . . . . . . 132
Table 53. EMI characteristics for HSE= 25 MHz and fCPU= 180 MHz . . . . . . . . . . . . . . . . . . . . . . 133
Table 54. ESD absolute maximum ratings. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 133
Table 55. Electrical sensitivities . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 134
Table 56. I/O current injection susceptibility. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 134
Table 57. I/O static characteristics . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 135
Table 58. Output voltage characteristics . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 138
Table 59. I/O AC characteristics. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 139
Table 60. NRST pin characteristics . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 141
Table 61. TIMx characteristics . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 142
Table 62. I2C analog filter characteristics. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 143
Table 63. SPI dynamic characteristics . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 143
Table 64. I2S dynamic characteristics . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 146
Table 65. SAI characteristics . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 149
Table 66. USB OTG full speed startup time. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 151
Table 67. USB OTG full speed DC electrical characteristics . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 151
Table 68. USB OTG full speed electrical characteristics . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 152
Table 69. USB HS DC electrical characteristics. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 152
Table 70. USB HS clock timing parameters. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 153
Table 71. Dynamic characteristics: USB ULPI. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 154
Table 72. Dynamics characteristics: Ethernet MAC signals for SMI. . . . . . . . . . . . . . . . . . . . . . . . . 155
Table 73. Dynamics characteristics: Ethernet MAC signals for RMII . . . . . . . . . . . . . . . . . . . . . . . . 156
Table 74. Dynamics characteristics: Ethernet MAC signals for MII . . . . . . . . . . . . . . . . . . . . . . . . . 157
Table 75. ADC characteristics . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 158
Table 76. ADC static accuracy at f = 18 MHz. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 159
ADC
Table 77. ADC static accuracy at f = 30 MHz. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 160
ADC
Table 78. ADC static accuracy at f = 36 MHz. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 160
ADC
Table 79. ADC dynamic accuracy at f = 18 MHz - limited test conditions . . . . . . . . . . . . . . . . . 160
ADC
Table 80. ADC dynamic accuracy at f = 36 MHz - limited test conditions . . . . . . . . . . . . . . . . . 160
ADC
Table 81. Temperature sensor characteristics. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 164
Table 82. Temperature sensor calibration values. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 164
Table 83. V monitoring characteristics . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 165
BAT
Table 84. internal reference voltage . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 165
Table 85. Internal reference voltage calibration values . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 165
Table 86. DAC characteristics . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 166
Table 87. Asynchronous non-multiplexed SRAM/PSRAM/NOR -
read timings . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 170
Table 88. Asynchronous non-multiplexed SRAM/PSRAM/NOR read -
NWAIT timings . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 171
Table 89. Asynchronous non-multiplexed SRAM/PSRAM/NOR write timings . . . . . . . . . . . . . . . . . 172
Table 90. Asynchronous non-multiplexed SRAM/PSRAM/NOR write -
NWAIT timings . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 173
Table 91. Asynchronous multiplexed PSRAM/NOR read timings. . . . . . . . . . . . . . . . . . . . . . . . . . . 174
Table 92. Asynchronous multiplexed PSRAM/NOR read-NWAIT timings . . . . . . . . . . . . . . . . . . . . 174
8/240 DS9405 Rev 13

<!-- Page 9 -->

STM32F427xx STM32F429xx List of tables
Table 93. Asynchronous multiplexed PSRAM/NOR write timings . . . . . . . . . . . . . . . . . . . . . . . . . . 175
Table 94. Asynchronous multiplexed PSRAM/NOR write-NWAIT timings. . . . . . . . . . . . . . . . . . . . 176
Table 95. Synchronous multiplexed NOR/PSRAM read timings . . . . . . . . . . . . . . . . . . . . . . . . . . . 177
Table 96. Synchronous multiplexed PSRAM write timings. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 179
Table 97. Synchronous non-multiplexed NOR/PSRAM read timings. . . . . . . . . . . . . . . . . . . . . . . . 180
Table 98. Synchronous non-multiplexed PSRAM write timings . . . . . . . . . . . . . . . . . . . . . . . . . . . . 181
Table 99. Switching characteristics for PC Card/CF read and write cycles
in attribute/common space. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 186
Table 100. Switching characteristics for PC Card/CF read and write cycles
in I/O space . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 187
Table 101. Switching characteristics for NAND Flash read cycles. . . . . . . . . . . . . . . . . . . . . . . . . . . 188
Table 102. Switching characteristics for NAND Flash write cycles. . . . . . . . . . . . . . . . . . . . . . . . . . . 189
Table 103. SDRAM read timings . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 190
Table 104. LPSDR SDRAM read timings. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 190
Table 105. SDRAM write timings . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 192
Table 106. LPSDR SDRAM write timings. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 192
Table 107. DCMI characteristics. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 193
Table 108. LTDC characteristics . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 194
Table 109. Dynamic characteristics: SD / MMC characteristics. . . . . . . . . . . . . . . . . . . . . . . . . . . . . 197
Table 110. RTC characteristics . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 197
Table 111. LQFP100 - Mechanical data. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 200
Table 112. WLCSP143 - 143-ball, 4.521x 5.547 mm, 0.4 mm pitch wafer level chip scale
package mechanical data . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 202
Table 113. WLCSP143 recommended PCB design rules . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 204
Table 114. LQFP144 - Mechanical data. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 206
Table 115. LQFP176 - Mechanical data. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 210
Table 116. LQFP208 - Mechanical data. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 214
Table 117. UFBGA169 - Mechanical data . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 217
Table 118. UFBGA169 - Example of PCB design rules (0.5 mm pitch BGA). . . . . . . . . . . . . . . . . . . 218
Table 119. UFBGA(176+25) - Mechanical data . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 219
Table 120. UFBGA(176+25) - Example of PCB design rules (0.65 mm pitch BGA) . . . . . . . . . . . . . 220
Table 121. TFBGA216 - Mechanical data . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 222
Table 122. TFBGA216 - Example of PCB design rules (0.8 mm pitch) . . . . . . . . . . . . . . . . . . . . . . . 223
Table 123. Package thermal characteristics. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 224
Table 124. Limitations depending on the operating power supply range. . . . . . . . . . . . . . . . . . . . . . 226
Table 125. Document revision history . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 233
DS9405 Rev 13 9/240
9

<!-- Page 10 -->

List of figures STM32F427xx STM32F429xx
List of figures
Figure 1. Compatible board design STM32F10xx/STM32F2xx/STM32F4xx
for LQFP100 package. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 18
Figure 2. Compatible board design between STM32F10xx/STM32F2xx/STM32F4xx
for LQFP144 package. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 19
Figure 3. Compatible board design between STM32F2xx and STM32F4xx
for LQFP176 and UFBGA176 packages. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 19
Figure 4. STM32F427xx and STM32F429xx block diagram . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 20
Figure 5. STM32F427xx and STM32F429xx Multi-AHB matrix. . . . . . . . . . . . . . . . . . . . . . . . . . . . . 23
Figure 6. Power supply supervisor interconnection with internal reset OFF . . . . . . . . . . . . . . . . . . . 27
Figure 7. PDR_ON control with internal reset OFF . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 28
Figure 8. Regulator OFF . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 30
Figure 9. Startup in regulator OFF: slow V slope
DD
- power-down reset risen after V /V stabilization. . . . . . . . . . . . . . . . . . . . . . . . 31
CAP_1 CAP_2
Figure 10. Startup in regulator OFF mode: fast V slope
DD
- power-down reset risen before V /V stabilization . . . . . . . . . . . . . . . . . . . . . . 31
CAP_1 CAP_2
Figure 11. STM32F42x LQFP100 pinout. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 45
Figure 12. STM32F42x WLCSP143 ballout. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 46
Figure 13. STM32F42x LQFP144 pinout. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 47
Figure 14. STM32F42x LQFP176 pinout. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 48
Figure 15. STM32F42x LQFP208 pinout. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 49
Figure 16. STM32F42x UFBGA169 ballout. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 50
Figure 17. STM32F42x UFBGA176 ballout. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 51
Figure 18. STM32F42x TFBGA216 ballout . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 52
Figure 19. Memory map. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 86
Figure 20. Pin loading conditions. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 91
Figure 21. Pin input voltage. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 91
Figure 22. Power supply scheme . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 92
Figure 23. Current consumption measurement scheme . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 93
Figure 24. External capacitor C . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 97
EXT
Figure 25. Typical V current consumption (LSE and RTC ON/backup RAM OFF) . . . . . . . . . . . 107
BAT
Figure 26. Typical V current consumption (LSE and RTC ON/backup RAM ON) . . . . . . . . . . . . 107
BAT
Figure 27. High-speed external clock source AC timing diagram . . . . . . . . . . . . . . . . . . . . . . . . . . . 119
Figure 28. Low-speed external clock source AC timing diagram. . . . . . . . . . . . . . . . . . . . . . . . . . . . 120
Figure 29. Typical application with an 8 MHz crystal. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 121
Figure 30. Typical application with a 32.768 kHz crystal. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 122
Figure 31. ACCHSI accuracy versus temperature. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 123
Figure 32. ACC versus temperature . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 124
LSI
Figure 33. PLL output clock waveforms in center spread mode . . . . . . . . . . . . . . . . . . . . . . . . . . . . 128
Figure 34. PLL output clock waveforms in down spread mode. . . . . . . . . . . . . . . . . . . . . . . . . . . . . 128
Figure 35. FT I/O input characteristics. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 137
Figure 36. I/O AC characteristics definition . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 140
Figure 37. Recommended NRST pin protection . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 141
Figure 38. SPI timing diagram - slave mode and CPHA = 0 . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 145
Figure 39. SPI timing diagram - slave mode and CPHA = 1 . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 145
Figure 40. SPI timing diagram - master mode. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 146
Figure 41. I2S slave timing diagram (Philips protocol)(1) . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 148
Figure 42. I2S master timing diagram (Philips protocol)(1). . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 148
Figure 43. SAI master timing waveforms. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 150
10/240 DS9405 Rev 13

<!-- Page 11 -->

STM32F427xx STM32F429xx List of figures
Figure 44. SAI slave timing waveforms . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 150
Figure 45. USB OTG full speed timings: definition of data signal rise and fall time. . . . . . . . . . . . . . 152
Figure 46. ULPI timing diagram. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 153
Figure 47. Ethernet SMI timing diagram . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 155
Figure 48. Ethernet RMII timing diagram. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 156
Figure 49. Ethernet MII timing diagram . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 157
Figure 50. ADC accuracy characteristics. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 161
Figure 51. Typical connection diagram when using the ADC with FT/TT pins
featuring the analog switch function. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 162
Figure 52. Power supply and reference decoupling (V not connected to V ). . . . . . . . . . . . . 163
REF+ DDA
Figure 53. Power supply and reference decoupling (V connected to V ). . . . . . . . . . . . . . . . 164
REF+ DDA
Figure 54. 12-bit buffered /non-buffered DAC . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 168
Figure 55. Asynchronous non-multiplexed SRAM/PSRAM/NOR read waveforms . . . . . . . . . . . . . . 170
Figure 56. Asynchronous non-multiplexed SRAM/PSRAM/NOR write waveforms . . . . . . . . . . . . . . 172
Figure 57. Asynchronous multiplexed PSRAM/NOR read waveforms. . . . . . . . . . . . . . . . . . . . . . . . 173
Figure 58. Asynchronous multiplexed PSRAM/NOR write waveforms . . . . . . . . . . . . . . . . . . . . . . . 175
Figure 59. Synchronous multiplexed NOR/PSRAM read timings . . . . . . . . . . . . . . . . . . . . . . . . . . . 177
Figure 60. Synchronous multiplexed PSRAM write timings. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 178
Figure 61. Synchronous non-multiplexed NOR/PSRAM read timings. . . . . . . . . . . . . . . . . . . . . . . . 180
Figure 62. Synchronous non-multiplexed PSRAM write timings . . . . . . . . . . . . . . . . . . . . . . . . . . . . 181
Figure 63. PC Card/CompactFlash controller waveforms for common memory read access. . . . . . 183
Figure 64. PC Card/CompactFlash controller waveforms for common memory write access. . . . . . 183
Figure 65. PC Card/CompactFlash controller waveforms for attribute memory
read access . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 184
Figure 66. PC Card/CompactFlash controller waveforms for attribute memory
write access. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 185
Figure 67. PC Card/CompactFlash controller waveforms for I/O space read access . . . . . . . . . . . . 185
Figure 68. PC Card/CompactFlash controller waveforms for I/O space write access. . . . . . . . . . . . 186
Figure 69. NAND controller waveforms for read access . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 188
Figure 70. NAND controller waveforms for write access . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 188
Figure 71. SDRAM read access waveforms (CL = 1) . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 189
Figure 72. SDRAM write access waveforms . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 191
Figure 73. DCMI timing diagram . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 193
Figure 74. LCD-TFT horizontal timing diagram . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 195
Figure 75. LCD-TFT vertical timing diagram . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 195
Figure 76. SDIO high-speed mode . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 196
Figure 77. SD default mode. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 196
Figure 78. LQFP100 - Outline(15). . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 199
Figure 79. LQFP100 - Footprint example . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 201
Figure 80. WLCSP143 - 143-ball, 4.521x 5.547 mm, 0.4 mm pitch wafer level chip scale
package outline . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 202
Figure 81. WLCSP143 - 143-ball, 4.521x 5.547 mm, 0.4 mm pitch wafer level chip scale
package recommended footprint . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 203
Figure 82. WLCSP143 marking example (package top view) . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 204
Figure 83. LQFP144 - Outline(15). . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 205
Figure 84. LQFP144 - Footprint example . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 208
Figure 85. LQFP176 - Outline(15). . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 209
Figure 86. LQFP176 - Footprint example . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 212
Figure 87. LQFP208 - Outline(15). . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 213
Figure 88. LQFP208 - footprint example . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 215
Figure 89. UFBGA169 - Outline. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 216
Figure 90. UFBGA169 - Footprint example. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 218
DS9405 Rev 13 11/240
12

<!-- Page 12 -->

List of figures STM32F427xx STM32F429xx
Figure 91. UFBGA(176+25) - Outline . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 219
Figure 92. UFBGA(176+25) - Footprint example. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 220
Figure 93. TFBGA216 - Outline. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 221
Figure 94. TFBGA216 - Footprint example . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 223
Figure 95. USB controller configured as peripheral-only and used
in Full speed mode. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 227
Figure 96. USB controller configured as host-only and used in full speed mode. . . . . . . . . . . . . . . . 227
Figure 97. USB controller configured in dual mode and used in full speed mode. . . . . . . . . . . . . . . 228
Figure 98. USB controller configured as peripheral, host, or dual-mode
and used in high speed mode. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 229
Figure 99. MII mode using a 25 MHz crystal. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 230
Figure 100. RMII with a 50 MHz oscillator. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 230
Figure 101. RMII with a 25 MHz crystal and PHY with PLL. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 231
12/240 DS9405 Rev 13

<!-- Page 13 -->

STM32F427xx STM32F429xx Introduction
1 Introduction
This datasheet provides the description of the STM32F427xx and STM32F429xx line of
microcontrollers. For more details on the whole STMicroelectronics STM32 family, refer to
Section2.1: Full compatibility throughout the family.
The STM32F427xx and STM32F429xx datasheet should be read with the STM32F4xx
reference manual.
For information on the Cortex®-M4 core, refer to the Cortex®-M4 programming manual
(PM0214), available from www.st.com.
For information on the device errata with respect to the datasheet and reference manual,
refer to the STM32F427/437xx and STM32F429/439xx errata sheet (ES0206), available
from www.st.com.
Note: Arm and Cortex are registered trademarks of Arm Limited (or its subsidiaries or affiliates) in
the US and/or elsewhere.
The Arm word and logo are trademarks of Arm Limited (or its subsidiaries) in the US and/or
elsewhere. All rights reserved.
DS9405 Rev 13 13/240
44

<!-- Page 14 -->

Description STM32F427xx STM32F429xx
2 Description
The STM32F427xx and STM32F429xx devices are based on the high-performance Arm®
Cortex®-M4 32-bit RISC core operating at a frequency of up to 180MHz. The Cortex®-M4
core features a floating-point unit (FPU) single precision, which supports all Arm® single-
precision data-processing instructions and data types. It also implements a full set of DSP
instructions and a memory protection unit (MPU) which enhances application security.
The STM32F427xx and STM32F429xx devices incorporate high-speed embedded
memories (Flash memory up to 2 Mbyte, up to 256 Kbytes of SRAM), up to 4Kbytes of
backup SRAM, and an extensive range of enhanced I/Os and peripherals connected to two
APB buses, two AHB buses and a 32-bit multi-AHB bus matrix.
All devices offer three 12-bit ADCs, two DACs, a low-power RTC, 12 general-purpose 16-bit
timers including two PWM timers for motor control, two general-purpose 32-bit timers. They
also feature standard and advanced communication interfaces.
• Up to three I2Cs
• Six SPIs, two I2Ss full duplex. To achieve audio class accuracy, the I2S peripherals can
be clocked via a dedicated internal audio PLL or via an external clock to allow
synchronization.
• Four USARTs plus four UARTs
• An USB OTG full-speed and a USB OTG high-speed with full-speed capability (with the
ULPI),
• Two CANs
• One SAI serial audio interface
• An SDIO/MMC interface
• Ethernet and camera interface
• LCD-TFT display controller
• Chrom-ART Accelerator™.
Advanced peripherals include an SDIO, a flexible memory control (FMC) interface, a
camera interface for CMOS image sensors. Refer to Table2: STM32F427xx and
STM32F429xx features and peripheral counts for the list of peripherals available on each
part number.
The STM32F427xx and STM32F429xx devices operates in the –40 to +105°C temperature
range from a 1.7 to 3.6V power supply.
The supply voltage can drop to 1.7V with the use of an external power supply supervisor
(refer to Section3.17.2: Internal reset OFF). A comprehensive set of power-saving mode
allows the design of low-power applications.
The STM32F427xx and STM32F429xx devices offer devices in 8 packages ranging from
100 pins to 216 pins. The set of included peripherals changes with the device chosen.
14/240 DS9405 Rev 13

<!-- Page 15 -->

STM32F427xx STM32F429xx Description
These features make the STM32F427xx and STM32F429xx microcontrollers suitable for a
wide range of applications:
• Motor drive and application control
• Medical equipment
• Industrial applications: PLC, inverters, circuit breakers
• Printers, and scanners
• Alarm systems, video intercom, and HVAC
• Home audio appliances
Figure4 shows the general block diagram of the device family.
DS9405 Rev 13 15/240
44

<!-- Page 16 -->


| Peripherals |  | STM32F427 Vx |  | STM32F429Vx |  |  | STM32F427 Zx |  | STM32F429Zx |  |  | STM32F427 Ax |  | STM32F429 Ax |  | STM32F427 Ix |  | STM32F429Ix |  |  | STM32F429Bx |  |  | STM32F429Nx |  |  |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Flash memory in Kbytes |  | 1024 | 2048 | 512 | 1024 | 2048 | 1024 | 2048 | 512 | 1024 | 2048 | 1024 | 2048 | 1024 | 2048 | 1024 | 2048 | 512 | 1024 | 2048 | 512 | 1024 | 2048 | 512 | 1024 | 2048 |
| SRAM in Kbytes | System | 256(112+16+64+64) |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  | Backup | 4 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| FMC memory controller |  | Yes(1) |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Ethernet |  | Yes |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Timers | General- purpose | 10 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  | Advanced -control | 2 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  | Basic | 2 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Random number generator |  | Yes |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Communication interfaces | SPI / I2S | 4/2 (full duplex)(2) |  |  |  |  | 6/2 (full duplex)(2) |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  | I2C | 3 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  | USART/ UART | 4/4 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  | USB OTG FS | Yes |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  | USB OTG HS | Yes |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  | CAN | 2 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  | SAI | 1 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  | SDIO | Yes |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Camera interface |  | Yes |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| LCD-TFT (STM32F429xx only) |  | No |  | Yes |  |  | No |  | Yes |  |  | No |  | Yes |  | No |  | Yes |  |  |  |  |  |  |  |  |
| Chrom-ART Accelerator™ |  | Yes |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| GPIOs |  | 82 |  |  |  |  | 114 |  |  |  |  | 130 |  |  |  | 140 |  |  |  |  | 168 |  |  |  |  |  |
| 12-bit ADC Number of channels |  | 3 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  | 16 |  |  |  |  | 24 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |

16/240
DS9405
Rev
13
Description
STM32F427xx
STM32F429xx
Table 2. STM32F427xx and STM32F429xx features and peripheral counts
STM32F427 STM32F427 STM32F427 STM32F429 STM32F427
Peripherals STM32F429Vx STM32F429Zx STM32F429Ix STM32F429Bx STM32F429Nx
Vx Zx Ax Ax Ix
Flash memory in Kbytes 1024 2048 512 1024 2048 1024 2048 512 1024 2048 1024 2048 1024 2048 1024 2048 512 1024 2048 512 1024 2048 512 1024 2048
System 256(112+16+64+64)
SRAM in
Kbytes
Backup 4
FMC memory controller Yes(1)
Ethernet Yes
General-
10
purpose
Timers Advanced
2
-control
Basic 2
Random number generator Yes
SPI / I2S 4/2 (full duplex)(2) 6/2 (full duplex)(2)
I2C 3
USART/
4/4
UART
USB OTG
Yes
Communication FS
interfaces
USB OTG
Yes
HS
CAN 2
SAI 1
SDIO Yes
Camera interface Yes
LCD-TFT (STM32F429xx
No Yes No Yes No Yes No Yes
only)
Chrom-ART Accelerator™ Yes
GPIOs 82 114 130 140 168
3
12-bit ADC
Number of channels
16 24

<!-- Page 17 -->


| Peripherals | STM32F427 Vx | STM32F429Vx | STM32F427 Zx | STM32F429Zx | STM32F427 Ax | STM32F429 Ax | STM32F427 Ix | STM32F429Ix | STM32F429Bx | STM32F429Nx |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 12-bit DAC Number of channels | Yes 2 |  |  |  |  |  |  |  |  |  |
| Maximum CPU frequency | 180MHz |  |  |  |  |  |  |  |  |  |
| Operating voltage | 1.8 to 3.6 V(3) |  |  |  |  |  |  |  |  |  |
| Operating temperatures | Ambient temperatures: –40 to +85 °C /–40 to +105 °C |  |  |  |  |  |  |  |  |  |
|  | Junction temperature: –40 to + 125 °C |  |  |  |  |  |  |  |  |  |
| Packages | LQFP100 |  | WLCSP143 LQFP144 |  | UFBGA169 |  | UFBGA176 LQFP176 |  | LQFP208 | TFBGA216 |

DS9405
Rev
13
17/240
STM32F427xx
STM32F429xx
Description
Table 2. STM32F427xx and STM32F429xx features and peripheral counts (continued)
STM32F427 STM32F427 STM32F427 STM32F429 STM32F427
Peripherals STM32F429Vx STM32F429Zx STM32F429Ix STM32F429Bx STM32F429Nx
Vx Zx Ax Ax Ix
12-bit DAC Yes
Number of channels 2
Maximum CPU frequency 180MHz
Operating voltage 1.8 to 3.6 V(3)
Ambient temperatures: –40 to +85 °C /–40 to +105 °C
Operating temperatures
Junction temperature: –40 to + 125 °C
WLCSP143 UFBGA176
Packages LQFP100 UFBGA169 LQFP208 TFBGA216
LQFP144 LQFP176
1. For the LQFP100 package, only FMC Bank1 or Bank2 are available. Bank1 can only support a multiplexed NOR/PSRAM memory using the NE1 Chip Select. Bank2 can only support a 16- or 8-bit
NAND Flash memory using the NCE2 Chip Select. The interrupt line cannot be used since Port G is not available in this package. For UFBGA169 package, only SDRAM, NAND and multiplexed
static memories are supported.
2. The SPI2 and SPI3 interfaces give the flexibility to work in an exclusive way in either the SPI mode or the I2S audio mode.
3. VDD/VDDA minimum value of 1.7V is obtained when the device operates in reduced temperature range, and with the use of an external power supply supervisor (refer to Section3.17.2: Internal reset
OFF).

<!-- Page 18 -->


|  |
| --- |
| VSS 75 51 76 73 50 49 VSS VSS (cid:19)(cid:3)(cid:159)(cid:3)(cid:85)(cid:72)(cid:86)(cid:76)(cid:86)(cid:87)(cid:82)(cid:85)(cid:3)(cid:82)(cid:85)(cid:3)(cid:86)(cid:82)(cid:79)(cid:71)(cid:72)(cid:85)(cid:76)(cid:81)(cid:74)(cid:3)(cid:69)(cid:85)(cid:76)(cid:71)(cid:74)(cid:72) (cid:83)(cid:85)(cid:72)(cid:86)(cid:72)(cid:81)(cid:87)(cid:3)(cid:73)(cid:82)(cid:85)(cid:3)(cid:87)(cid:75)(cid:72)(cid:3)(cid:54)(cid:55)(cid:48)32F10xxx 99 (VSS) (cid:70)(cid:82)(cid:81)(cid:73)(cid:76)(cid:74)(cid:88)(cid:85)(cid:68)(cid:87)(cid:76)(cid:82)(cid:81)(cid:15)(cid:3)(cid:81)(cid:82)(cid:87)(cid:3)(cid:83)(cid:85)(cid:72)(cid:86)(cid:72)(cid:81)(cid:87)(cid:3)(cid:76)(cid:81)(cid:3)(cid:87)(cid:75)(cid:72) 100 19 20 26 (cid:54)(cid:55)(cid:48)(cid:22)(cid:21)(cid:41)(cid:23)(cid:91)(cid:91)(cid:3)(cid:70)(cid:82)(cid:81)(cid:73)(cid:76)(cid:74)(cid:88)(cid:85)(cid:68)(cid:87)(cid:76)(cid:82)(cid:81)(cid:3)(cid:3) 1 25 VSS VSS (cid:57)(cid:54)(cid:54)(cid:3)(cid:73)(cid:82)(cid:85)(cid:3)(cid:54)(cid:55)(cid:48)(cid:22)(cid:21)(cid:41)(cid:20)(cid:19)(cid:91)(cid:91) (cid:55)(cid:90)(cid:82)(cid:3)(cid:19)(cid:3)(cid:159)(cid:3)(cid:85)(cid:72)(cid:86)(cid:76)(cid:86)(cid:87)(cid:82)(cid:85)(cid:86)(cid:3)(cid:70)(cid:82)(cid:81)(cid:81)(cid:72)(cid:70)(cid:87)(cid:72)(cid:71)(cid:3)(cid:87)(cid:82)(cid:29)(cid:3)(cid:3) VDDVSS (cid:57)(cid:39)(cid:39)(cid:3)(cid:73)(cid:82)(cid:85)(cid:3)(cid:54)(cid:55)(cid:48)(cid:22)(cid:21)(cid:41)(cid:23)(cid:91)(cid:91) -(cid:3)(cid:57)(cid:54)(cid:54)(cid:3)(cid:73)(cid:82)(cid:85)(cid:3)(cid:87)(cid:75)(cid:72)(cid:3)(cid:54)(cid:55)(cid:48)(cid:22)(cid:21)(cid:41)(cid:20)(cid:19)(cid:91)(cid:91) -(cid:3)(cid:57)(cid:54)(cid:54)(cid:3)(cid:73)(cid:82)(cid:85)(cid:3)(cid:87)(cid:75)(cid:72)(cid:3)(cid:54)(cid:55)(cid:48)(cid:22)(cid:21)(cid:41)(cid:23)(cid:91)(cid:91) -(cid:3)(cid:57)(cid:54)(cid:54)(cid:3)(cid:82)(cid:85)(cid:3)(cid:49)(cid:38)(cid:3)(cid:73)(cid:82)(cid:85)(cid:3)(cid:87)(cid:75)(cid:72)(cid:3)(cid:54)(cid:55)(cid:48)(cid:22)(cid:21)(cid:41)(cid:21)(cid:91)(cid:91) ai18488d |

Description STM32F427xx STM32F429xx
2.1 Full compatibility throughout the family
The STM32F427xx and STM32F429xx devices are part of the STM32F4 family. They are
fully pin-to-pin, software and feature compatible with the STM32F2xx devices, allowing the
user to try different memory densities, peripherals, and performances (FPU, higher
frequency) for a greater degree of freedom during the development cycle.
The STM32F427xx and STM32F429xx devices maintain a close compatibility with the
whole STM32F10xx family. All functional pins are pin-to-pin compatible. The STM32F427xx
and STM32F429xx, however, are not drop-in replacements for the STM32F10xx devices:
the two families do not have the same power scheme, and so their power pins are different.
Nonetheless, the transition from the STM32F10xx to the STM32F42x family remains simple
as only a few pins are impacted.
Figure1, Figure2, and Figure3, give compatible board designs between the STM32F4xx,
STM32F2xx, and STM32F10xx families.
Figure 1. Compatible board design STM32F10xx/STM32F2xx/STM32F4xx
for LQFP100 package
75
VSS
51
76 73 50
49
VSS
VSS
(cid:19)(cid:3)(cid:159)(cid:3)(cid:85)(cid:72)(cid:86)(cid:76)(cid:86)(cid:87)(cid:82)(cid:85)(cid:3)(cid:82)(cid:85)(cid:3)(cid:86)(cid:82)(cid:79)(cid:71)(cid:72)(cid:85)(cid:76)(cid:81)(cid:74)(cid:3)(cid:69)(cid:85)(cid:76)(cid:71)(cid:74)(cid:72)
(cid:83)(cid:85)(cid:72)(cid:86)(cid:72)(cid:81)(cid:87)(cid:3)(cid:73)(cid:82)(cid:85)(cid:3)(cid:87)(cid:75)(cid:72)(cid:3)(cid:54)(cid:55)(cid:48)32F10xxx
99 (VSS) (cid:70)(cid:82)(cid:81)(cid:73)(cid:76)(cid:74)(cid:88)(cid:85)(cid:68)(cid:87)(cid:76)(cid:82)(cid:81)(cid:15)(cid:3)(cid:81)(cid:82)(cid:87)(cid:3)(cid:83)(cid:85)(cid:72)(cid:86)(cid:72)(cid:81)(cid:87)(cid:3)(cid:76)(cid:81)(cid:3)(cid:87)(cid:75)(cid:72)
100 19 20 26 (cid:54)(cid:55)(cid:48)(cid:22)(cid:21)(cid:41)(cid:23)(cid:91)(cid:91)(cid:3)(cid:70)(cid:82)(cid:81)(cid:73)(cid:76)(cid:74)(cid:88)(cid:85)(cid:68)(cid:87)(cid:76)(cid:82)(cid:81)(cid:3)(cid:3)
1 25
VSS
VSS
(cid:57)(cid:54)(cid:54)(cid:3)(cid:73)(cid:82)(cid:85)(cid:3)(cid:54)(cid:55)(cid:48)(cid:22)(cid:21)(cid:41)(cid:20)(cid:19)(cid:91)(cid:91)
(cid:55)(cid:90)(cid:82)(cid:3)(cid:19)(cid:3)(cid:159)(cid:3)(cid:85)(cid:72)(cid:86)(cid:76)(cid:86)(cid:87)(cid:82)(cid:85)(cid:86)(cid:3)(cid:70)(cid:82)(cid:81)(cid:81)(cid:72)(cid:70)(cid:87)(cid:72)(cid:71)(cid:3)(cid:87)(cid:82)(cid:29)(cid:3)(cid:3)
-(cid:3)(cid:57)(cid:54)(cid:54)(cid:3)(cid:73)(cid:82)(cid:85)(cid:3)(cid:87)(cid:75)(cid:72)(cid:3)(cid:54)(cid:55)(cid:48)(cid:22)(cid:21)(cid:41)(cid:20)(cid:19)(cid:91)(cid:91)
VDDVSS (cid:57)(cid:39)(cid:39)(cid:3)(cid:73)(cid:82)(cid:85)(cid:3)(cid:54)(cid:55)(cid:48)(cid:22)(cid:21)(cid:41)(cid:23)(cid:91)(cid:91)
-(cid:3)(cid:57)(cid:54)(cid:54)(cid:3)(cid:73)(cid:82)(cid:85)(cid:3)(cid:87)(cid:75)(cid:72)(cid:3)(cid:54)(cid:55)(cid:48)(cid:22)(cid:21)(cid:41)(cid:23)(cid:91)(cid:91)
-(cid:3)(cid:57)(cid:54)(cid:54)(cid:3)(cid:82)(cid:85)(cid:3)(cid:49)(cid:38)(cid:3)(cid:73)(cid:82)(cid:85)(cid:3)(cid:87)(cid:75)(cid:72)(cid:3)(cid:54)(cid:55)(cid:48)(cid:22)(cid:21)(cid:41)(cid:21)(cid:91)(cid:91)
ai18488d
18/240 DS9405 Rev 13

<!-- Page 19 -->

STM32F427xx STM32F429xx Description
Figure 2. Compatible board design between STM32F10xx/STM32F2xx/STM32F4xx
for LQFP144 package
(cid:19)(cid:3)(cid:159)(cid:3)(cid:85)(cid:72)(cid:86)(cid:76)(cid:86)(cid:87)(cid:82)(cid:85)(cid:3)(cid:82)(cid:85)(cid:3)(cid:86)(cid:82)(cid:79)(cid:71)(cid:72)(cid:85)(cid:76)(cid:81)(cid:74)(cid:3)(cid:69)(cid:85)(cid:76)(cid:71)(cid:74)(cid:72)
(cid:83)(cid:85)(cid:72)(cid:86)(cid:72)(cid:81)(cid:87)(cid:3)(cid:73)(cid:82)(cid:85)(cid:3)(cid:87)(cid:75)(cid:72)(cid:3)(cid:54)(cid:55)(cid:48)32F10xx
108 V SS 73 (cid:70)(cid:82)(cid:81)(cid:73)(cid:76)(cid:74)(cid:88)(cid:85)(cid:68)(cid:87)(cid:76)(cid:82)(cid:81)(cid:15)(cid:3)(cid:81)(cid:82)(cid:87)(cid:3)(cid:83)(cid:85)(cid:72)(cid:86)(cid:72)(cid:81)(cid:87)(cid:3)(cid:76)(cid:81)(cid:3)(cid:87)(cid:75)(cid:72)(cid:3)
(cid:54)(cid:55)(cid:48)(cid:22)(cid:21)(cid:41)(cid:23)(cid:91)(cid:91)(cid:3)(cid:70)(cid:82)(cid:81)(cid:73)(cid:76)(cid:74)(cid:88)(cid:85)(cid:68)(cid:87)(cid:76)(cid:82)(cid:81)(cid:3)(cid:3)
106
109 72
71
V
SS
(cid:49)(cid:82)(cid:87)(cid:3)(cid:83)(cid:82)(cid:83)(cid:88)(cid:79)(cid:68)(cid:87)(cid:72)(cid:71)(cid:3)(cid:90)(cid:75)(cid:72)(cid:81)(cid:3)(cid:19)(cid:3)(cid:159)(cid:3)
V
SS (cid:85)(cid:72)(cid:86)(cid:76)(cid:86)(cid:87)(cid:82)(cid:85)(cid:3)(cid:82)(cid:85)(cid:3)(cid:86)(cid:82)(cid:79)(cid:71)(cid:72)(cid:85)(cid:76)(cid:81)(cid:74)(cid:3)
(cid:69)(cid:85)(cid:76)(cid:71)(cid:74)(cid:72)(cid:3)(cid:83)(cid:85)(cid:72)(cid:86)(cid:72)(cid:81)(cid:87)(cid:3)
(cid:54)(cid:76)(cid:74)(cid:81)(cid:68)(cid:79)(cid:3)(cid:73)(cid:85)(cid:82)(cid:80)(cid:3)
(cid:72)(cid:91)(cid:87)(cid:72)(cid:85)(cid:81)(cid:68)(cid:79)(cid:3)(cid:83)(cid:82)(cid:90)(cid:72)(cid:85)(cid:3)
(cid:20)(cid:23)(cid:22)(cid:3)(cid:11)(cid:51)(cid:39)(cid:53)(cid:66)(cid:50)(cid:49)(cid:12)
(cid:86)(cid:88)(cid:83)(cid:83)(cid:79)(cid:92)(cid:3)
(cid:86)(cid:88)(cid:83)(cid:72)(cid:85)(cid:89)(cid:76)(cid:86)(cid:82)(cid:85) 144 30 31 37
1 36
V
SS
V V
DD SS
(cid:49)(cid:82)(cid:87)(cid:3)(cid:83)(cid:82)(cid:83)(cid:88)(cid:79)(cid:68)(cid:87)(cid:72)(cid:71)(cid:3)(cid:73)(cid:82)(cid:85)(cid:3)(cid:54)(cid:55)(cid:48)32F10xx
(cid:55)(cid:90)(cid:82)(cid:3)(cid:19)(cid:3)(cid:159)(cid:3)(cid:3)(cid:85)(cid:72)(cid:86)(cid:76)(cid:86)(cid:87)(cid:82)(cid:85)(cid:86)(cid:3)(cid:70)(cid:82)(cid:81)(cid:81)(cid:72)(cid:70)(cid:87)(cid:72)(cid:71)(cid:3)(cid:87)(cid:82)(cid:29)(cid:3)
- - V V S S S S , (cid:73) (cid:82) V (cid:85) D (cid:3) D (cid:87)(cid:75) (cid:82) (cid:72) (cid:85) (cid:3) (cid:3) (cid:54) (cid:49) (cid:55) (cid:38) (cid:48) (cid:3)(cid:73) (cid:22) (cid:82) (cid:21) (cid:85)(cid:3) (cid:41) (cid:87)(cid:75) (cid:20) (cid:72) (cid:19) (cid:3) (cid:91) (cid:54) (cid:91) (cid:55)(cid:48)(cid:22)(cid:21)(cid:41)(cid:21)(cid:91)(cid:91) V DD V SS V V S D S D (cid:73) (cid:73) (cid:82) (cid:82) (cid:85) (cid:85) (cid:3) (cid:3) (cid:54) (cid:54) (cid:55) (cid:55) (cid:48) (cid:48) (cid:22) (cid:22) (cid:21) (cid:21) (cid:41) (cid:41) (cid:23) (cid:20) (cid:91) (cid:19) (cid:91) (cid:91)(cid:91)
- V DD (cid:82)(cid:85)(cid:3)(cid:86)(cid:76)(cid:74)(cid:81)(cid:68)(cid:79)(cid:3)(cid:73)(cid:85)(cid:82)(cid:80)(cid:3)(cid:72)(cid:91)(cid:87)(cid:72)(cid:85)(cid:81)(cid:68)(cid:79)(cid:3)(cid:83)(cid:82)(cid:90)(cid:72)(cid:85)(cid:3)(cid:86)(cid:88)(cid:83)(cid:83)(cid:79)(cid:92)(cid:3)(cid:86)(cid:88)(cid:83)(cid:72)(cid:85)(cid:89)(cid:76)(cid:86)(cid:82)(cid:85)(cid:3)(cid:73)(cid:82)(cid:85)(cid:3)(cid:87)(cid:75)(cid:72)(cid:3)(cid:54)(cid:55)(cid:48)(cid:22)(cid:21)(cid:41)(cid:23)(cid:91)(cid:91)
ai18487d
Figure 3. Compatible board design between STM32F2xx and STM32F4xx
for LQFP176 and UFBGA176 packages
132 89
133 88
48 (cid:16)(cid:3)(cid:42)(cid:49)(cid:39)(cid:3)(cid:73)(cid:82)(cid:85)(cid:3)(cid:54)(cid:55)(cid:48)(cid:22)(cid:21)(cid:41)(cid:21)(cid:91)(cid:91)
(cid:16)(cid:3)(cid:37)(cid:60)(cid:51)(cid:36)(cid:54)(cid:54)(cid:66)(cid:53)(cid:40)(cid:42)(cid:3)(cid:73)(cid:82)(cid:85)(cid:3)(cid:54)(cid:55)(cid:48)(cid:22)(cid:21)(cid:41)(cid:23)(cid:91)(cid:91)
(cid:54)(cid:76)(cid:74)(cid:81)(cid:68)(cid:79)(cid:3)(cid:73)(cid:85)(cid:82)(cid:80)(cid:3)(cid:72)(cid:91)(cid:87)(cid:72)(cid:85)(cid:81)(cid:68)(cid:79)(cid:3)
(cid:83)(cid:82)(cid:90)(cid:72)(cid:85)(cid:3)(cid:86)(cid:88)(cid:83)(cid:83)(cid:79)(cid:92)(cid:3) (cid:20)(cid:26)(cid:20)(cid:3)(cid:11)(cid:51)(cid:39)(cid:53)(cid:66)(cid:50)(cid:49)(cid:12)
(cid:86)(cid:88)(cid:83)(cid:72)(cid:85)(cid:89)(cid:76)(cid:86)o(cid:85) 176 45
1 44
V V
DD SS
(cid:55)(cid:90)(cid:82)(cid:3)(cid:19)(cid:3)(cid:159)(cid:3)(cid:3)(cid:85)(cid:72)(cid:86)(cid:76)(cid:86)(cid:87)(cid:82)(cid:85)(cid:86)(cid:3)(cid:70)(cid:82)(cid:81)(cid:81)(cid:72)(cid:70)(cid:87)(cid:72)(cid:71)(cid:3)(cid:87)(cid:82)(cid:29)(cid:3)(cid:3)(cid:3)(cid:3)
- V SS , V DD (cid:82)(cid:85)(cid:3)(cid:49)(cid:38)(cid:3)(cid:73)(cid:82)(cid:85)(cid:3)(cid:87)(cid:75)(cid:72)(cid:3)(cid:54)(cid:55)(cid:48)(cid:22)(cid:21)(cid:41)(cid:21)(cid:91)(cid:91)
- V
DD
(cid:82)(cid:85)(cid:3)(cid:86)(cid:76)(cid:74)(cid:81)(cid:68)(cid:79)(cid:3)(cid:73)(cid:85)(cid:82)(cid:80)(cid:3)(cid:72)(cid:91)(cid:87)(cid:72)(cid:85)(cid:81)(cid:68)(cid:79)(cid:3)(cid:83)(cid:82)(cid:90)(cid:72)(cid:85)(cid:3)(cid:86)(cid:88)(cid:83)(cid:83)(cid:79)(cid:92)(cid:3)(cid:86)(cid:88)(cid:83)(cid:72)(cid:85)(cid:89)(cid:76)(cid:86)o(cid:85)(cid:3)(cid:73)(cid:82)(cid:85)(cid:3)(cid:87)(cid:75)(cid:72)(cid:3)(cid:54)(cid:55)(cid:48)(cid:22)(cid:21)(cid:41)(cid:23)(cid:91)(cid:91)
MS31835V1
DS9405 Rev 13 19/240
44

<!-- Page 20 -->


| CLK, NE [3:0], A[23:0], External memory controller (FMC) CCM data RAM 64 KB JTAG & SW MPU D[31:0], NOEN, NWEN, NBL[3:0], SDCLKE[1:0], SDNE[1:0], SDNWE, NL SRAM, SDRAM, PSRAM, NOR Flash, PC Card, NJTRST, JTDI, JTCK/SWCLK AHB3 ETM NVIC NAND Flash JTDO/SWD, JTDO NRAS, NCAS, NADV TRACECLK NWAIT/NIORD, NREG, CD TRACED[3:0] Arm Cortex-M4 INTR D-BUS 180 MHz I-BUS 1MB Flash FPU /LECCA EHCAC S-BUS MII or RMII as AF Ethernet MAC DMA/ TRA RNG M7S8 1MB Flash MDIO as AF 10/100 FIFO Camera HSYNC, VSYNC xirtam-sub SRAM 112 KB interface PUIXCLK, D[13:0] OFIF DP, DM USB ULPI:CK, D[7:0], DIR, STP, NXT OTG HS ID, VBUS, SOF YHP USB DP DM OFIF YHP BHA DMA/ SRAM 16 KB FIFO SRAM 64 KB DMA2 OTG FS ID, VBUS, SOF 8 Streams AHB2 180 MHz FIFO 8 Streams FIFO AHB1 180 MHz VDD Power managmt DMA1 3.3 r V e t o g o l u t 1 a la . g 2 t e o V r V VS D S D = 1.8 to 3.6 V LCD-TFT FIFO LCD_R[7:0], LCD_G[7:0], LCD_B[7:0], LCD_HSYNC, LCD_VSYNC, LCD_DE, VCAP1, VCAP2 LCD_CLK @VDDA RC HS P re O s R et sup S e u r p v p is l i y on @VDD FIFO CHROM-ART DMA2D PA[15:0] GPIO PORT A VDDA, VSSA RC LS Int POR/PDR BOR NRST PLL1,2,3 PVD PB[15:0] GPIO PORT B @VDDA @VDD PC[15:0] GPIO PORT C OSC_IN XTAL OSC PD[15:0] GPIO PORT D OSC_OUT 4- 26MHz Reset & IWDG MAclNockAGT PE[15:0] GPIO PORT E VBAT = 1.65 to 3.6 V control Standby interface PF[15:0] GPIO PORT F OSC32_IN @VBAT PG[15:0] GPIO PORT G OSC32_OUT XTAL 32 kHz xKLCP PH[15:0] GPIO PORT H SL xKLCH RTC RTC_AF1 AWU Backup register RTC_AF1 SL PI[15:0] GPIO PORT I RTC_50HZ 4 KB BKPSRAM PJ[15:0] GPIO PORT J TIM2 32b 4 channels, ETR as AF PK[7:0] GPIO PORT K TIM3 16b 4 channels, ETR as AF 168 AF EXT IT. WKUP DMA2 TIM4 DMA1 16b 4 channels, ETR as AF D[7:0] AHB/APB2 CMD, CK as AF SDIO / MMC OFIF TIM5 AHB/APB1 32b 4 channels 4 compl. chan. (TIM1_CH1[1:4]N), TIM12 16b 2 channels as AF 4 chan. (TIM1_CH1[1:4]ETR, TIM1 / PWM 16b BKIN as AF TIM13 16b 1 channel as AF 4 compl. chan.(TIM8_CH1[1:4]N), TIM8 / PWM 16b TIM14 4 chan. (TIM8_CH1[1:4], ETR, 16b 1 channel as AF BKIN as AF 2 channels as AF TIM9 16b smcard RX, TX as AF USART2 irDA CTS, RTS as AF WWDG 1 1 c c h h a a n n n n e e l l a a s s A A F F T T I I M M 1 1 0 1 1 1 6 6 b b USART3 smc i a r r D d A R CT X S , , T R X T a S s a A s F AF CTS, R R X T , S T X a , s C A K F , USART1 UART4 smcard irDA zHM RX, TX as AF UART5 RX, TX as AF zHM031BPA 09 2BPA CTS, R R X T , S T X a , s C A K F , s ir m DA card USART6 UART7 RX, TX as AF SC M K, O N S S I, S M as IS A O F , SPI1 zHM062BPA TIM6 TIM7 )xam( zHM 54 16b 16b SP2/I2S2 M NS O S S /W I/S S D , , M M C I K S O a / s S A D F _ext, SCK/CK UART8 RX, TX as AF SC M K, O N S S I, S M as IS A O F , SPI4 SP3/I2S3 MOSI/SD, MISO/SD_ext, SCK/CK 1BPA SC M K, O N S S I, S M as IS A O F , SPI5 NSS/WS, MCK as AF I2C1/SMBUS SCL, SDA, SMBA as AF SC M K, O N S S I, S M as IS A O F , SPI6 I2C2/SMBUS SCL, SDA, SMBA as AF SD, SCK, FS, MCLK as AF SAI1 OFIF retlif @VDDA latigiD VDDREF_ADC I2C3/SMBUS SCL, SDA, SMBA as AF 8 analog inputs common ITF to the 3 ADCs TUemSpAeRratTur2e MsenBspors ADC1 DAC1 DAC2 bxCAN1 TX, RX ADC2 OFIF 8 analog inputs common bxCAN2 TX, RX IIFF ADC3 to the ADC1 & 2 8 analog inputs for ADC3 DAC1_OUT as AF @VDDA DAC2_OUT as AF MSv30420V5.svg |
| --- |
|  |


| CCM data RAM 64 KB External memory controller (FMC) SRAM, SDRAM, PSRAM, JTAG & SW MPU AHB3 NOR Flash, PC Card, ETM NVIC NAND Flash D-BUS Arm Cortex-M4 180 MHz I-BUS FPU 1MB Flash /LECCA EHCAC S-BUS Ethernet MAC DMA/ TRA RNG M7S8 1MB Flash 10/100 FIFO Camera xirtam-sub SRAM 112 KB interface OFIF USB OTG HS YHP USB OFIF YHP BHA DMA/ SRAM 16 KB FIFO SRAM 64 KB DMA2 OTG FS 8 Streams AHB2 180 MHz FIFO 8 Streams FIFO AHB1 180 MHz VDD Power managmt DMA1 Voltage regulator 3.3 to 1.2 V LCD-TFT FIFO @VDDA RC HS P re O s R et sup S e u r p v p is l i y on @VDD CHROM-ART DMA2D FIFO RC LS Int POR/PDR GPIO PORT A BOR PLL1,2,3 PVD GPIO PORT B @VDDA @VDD GPIO PORT C XTAL OSC GPIO PORT D 4- 26MHz Reset & IWDG MAclNockAGT GPIO PORT E control Standby GPIO PORT F interface @VBAT GPIO PORT G XTAL 32 kHz xKLCP xKLCH GPIO PORT H SL RTC AWU Backup register GPIO PORT I SL 4 KB BKPSRAM GPIO PORT J TIM2 32b GPIO PORT K TIM3 16b EXT IT. WKUP DMA2 DMA1 TIM4 16b SDIO / MMC AHB/APB2 OFIF AHB/APB1 TIM5 32b |
| --- |
| TIM12 16b TIM1 / PWM 16b TIM13 16b TIM8 / PWM 16b TIM14 16b TIM9 16b smcard USART2 irDA WWDG TIM10 16b USART3 smc i a r r D d A TIM11 16b UART4 USART1 smcard irDA zHM UART5 zHM031BPA 09 2BPA s ir m DA card USART6 UART7 )xam( SPI1 TIM6 16b UART8 zHM062BPA zHM TIM7 54 SPI4 16b SP2/I2S2 SP3/I2S3 1BPA SPI5 SPI6 I2C1/SMBUS SAI1 I2C2/SMBUS OFIF retlif @VDDA latigiD TUemSpAeRratTur2e MsenBspors I2C3/SMBUS ITF DAC1 ADC1 DAC2 bxCAN1 ADC2 OFIF ADC3 IIFF bxCAN2 DAC1_OUT @VDDA as AF DAC2_OUT as AF |

Description STM32F427xx STM32F429xx
Figure 4. STM32F427xx and STM32F429xx block diagram
PA[15:0] GPIO PORT A
168 AF EXT IT. WKUP
AHB/APB2
4 compl. chan. (TIM1_CH1[1:4]N),
4 chan. (TIM1_CH1[1:4]ETR, TIM1 / PWM
BKIN as AF
CTS, R R X T , S T X a , s C A K F , USART1
SC M K, O N S S I, S M as IS A O F , SPI1
MSv30420V5.svg
1. The timers connected to APB2 are clocked from TIMxCLK up to 180 MHz, while the timers connected to APB1 are clocked
from TIMxCLK either up to 90MHz or 180MHz depending on TIMPRE bit configuration in the RCC_DCKCFGR register.
2. The LCD-TFT is available only on STM32F429xx devices.
20/240 DS9405 Rev 13
zHM062BPA
zHM031BPA
JTAG & SW MPU
ETM NVIC
TRACECLK
TRACED[3:0] Arm Cortex-M4
180 MHz
MII or RMII as AF Ethernet MAC DMA/
MDIO as AF 10/100 FIFO
DP, DM USB ULPI:CK, D[7:0], DIR, STP, NXT OTG HS ID, VBUS, SOF
DMA2
8 Streams FIFO
VDDA, VSSA
NRST
VBAT = 1.65 to 3.6 V
OSC32_IN
OSC32_OUT
RTC_AF1
4 KB BKPSRAM
DMA2
D[7:0]
CMD, CK as AF SDIO / MMC
16b
WWDG
UART4 smcard irDA
SP3/I2S3 MOSI/SD, MISO/SD_ext, SCK/CK
NSS/WS, MCK as AF
VDDREF_ADC I2C3/SMBUS SCL, SDA, SMBA as AF 8 analog inputs common ITF to the 3 ADCs
bxCAN2 TX, RX
DAC1_OUT as AF
/LECCA
TRA
EHCAC
CLK, NE [3:0], A[23:0],
D[31:0], NOEN, NWEN, NBL[3:0], SDCLKE[1:0], SDNE[1:0], SDNWE, NL
NWAIT/NIORD, NREG, CD
INTR
RNG
Camera HSYNC, VSYNC
SRAM 112 KB interface PUIXCLK, D[13:0]
YHP USB DP DM
OTG FS ID, VBUS, SOF
OFIF
AHB1 180 MHz
YHP
OFIF
@VDDA RC HS P re O s R et sup S e u r p v p is l i y on
RC LS Int POR/PDR BOR
PLL1,2,3
PVD
@VDDA @VDD
Reset & IWDG MAclNockAGT
control Standby interface
@VBAT
XTAL 32 kHz
RTC
AWU
@VDDA
TUemSpAeRratTur2e MsenBspors ADC1
ADC2
IIFF ADC3
@VDDA
xKLCP
VDD Power managmt
3.3 r V e t o g o l u t 1 a la . g 2 t e o V r V VS D S D = 1.8 to 3.6 V
VCAP1, VCAP2
@VDD
Backup register RTC_AF1
zHM
09 2BPA
M7S8
xirtam-sub
BHA
SL
External memory controller (FMC)
SRAM, SDRAM, PSRAM, NOR Flash, PC Card,
NAND Flash
D-BUS
1MB Flash
TIM2
TIM3
TIM4
TIM5
TIM12
TIM14
2 channels as AF TIM9
TIM6 TIM7
DAC1 DAC2
OFIF
FPU
)xam( zHM 54
1BPA
CCM data RAM 64 KB
NJTRST, JTDI, JTCK/SWCLK AHB3
JTDO/SWD, JTDO
I-BUS
S-BUS
DMA/ SRAM 16 KB FIFO
8 Streams AHB2 180 MHz
FIFO
DMA1
PB[15:0] GPIO PORT B
PC[15:0] GPIO PORT C
PD[15:0] GPIO PORT D
PE[15:0] GPIO PORT E
PF[15:0] GPIO PORT F
PG[15:0] GPIO PORT G
PH[15:0] GPIO PORT H
PI[15:0] GPIO PORT I
32b 4 channels, ETR as AF
16b 4 channels, ETR as AF
DMA1 16b 4 channels, ETR as AF
AHB/APB1 32b 4 channels
16b 2 channels as AF
TIM13 16b 1 channel as AF
4 compl. chan.(TIM8_CH1[1:4]N), TIM8 / PWM 16b
4 chan. (TIM8_CH1[1:4], ETR, 16b 1 channel as AF
BKIN as AF
16b smcard RX, TX as AF USART2 irDA CTS, RTS as AF
1 1 c c h h a a n n n n e e l l a a s s A A F F T T I I M M 1 1 0 1 1 1 6 6 b b USART3 smc i a r r D d A R CT X S , , T R X T a S s a A s F AF
RX, TX as AF UART5 RX, TX as AF
CTS, R R X T , S T X a , s C A K F , s ir m DA card USART6
16b 16b SP2/I2S2 M NS O S S /W I/S S D , , M M C I K S O a / s S A D F _ext, SCK/CK
I2C1/SMBUS SCL, SDA, SMBA as AF
I2C2/SMBUS SCL, SDA, SMBA as AF
bxCAN1 TX, RX
8 analog inputs common
to the ADC1 & 2 8 analog inputs for ADC3
DAC2_OUT as AF
SL
OSC_IN
OSC_OUT
xKLCH
XTAL OSC
4- 26MHz
OFIF
1MB Flash
SRAM 64 KB
LCD-TFT FIFO
FIFO
PJ[15:0] GPIO PORT J
PK[7:0] GPIO PORT K
UART7 RX, TX as AF
UART8 RX, TX as AF SC M K, O N S S I, S M as IS A O F , SPI4
SC M K, O N S S I, S M as IS A O F , SPI5
SC M K, O N S S I, S M as IS A O F , SPI6
SD, SCK, FS, MCLK as AF SAI1 OFIF retlif
latigiD
NRAS, NCAS, NADV
LCD_R[7:0], LCD_G[7:0], LCD_B[7:0], LCD_HSYNC, LCD_VSYNC, LCD_DE,
LCD_CLK
CHROM-ART DMA2D
RTC_50HZ

<!-- Page 21 -->

STM32F427xx STM32F429xx Functional overview
3 Functional overview
® ®
3.1 Arm Cortex -M4 with FPU and embedded flash and SRAM
The Arm® Cortex®-M4 with FPU processor is the latest generation of Arm® processors for
embedded systems. It was developed to provide a low-cost platform that meets the needs of
MCU implementation, with a reduced pin count and low-power consumption, while
delivering outstanding computational performance and an advanced response to interrupts.
The Arm® Cortex®-M4 with FPU core is a 32-bit RISC processor that features exceptional
code-efficiency, delivering the high-performance expected from an Arm® core in the
memory size usually associated with 8- and 16-bit devices.
The processor supports a set of DSP instructions, which allow efficient signal processing
and complex algorithm execution.
Its single-precision FPU (floating-point unit) speeds up software development by using
metalanguage development tools, while avoiding saturation.
The STM32F42x family is compatible with all Arm tools and software.
Figure4 shows the general block diagram of the STM32F42x family.
Note: Cortex®-M4 with FPU core is binary compatible with the Cortex®-M3 core.
3.2 Adaptive real-time memory accelerator (ART Accelerator™)
The ART Accelerator™ is a memory accelerator, which is optimized for STM32 industry-
standard Arm® Cortex®-M4 with FPU processors. It balances the inherent performance
advantage of the Arm® Cortex®-M4 with FPU over flash memory at higher frequencies.
To release the processor full 225DMIPS performance at this frequency, the accelerator
implements an instruction prefetch queue and branch cache, which increases program
execution speed from the 128-bit flash memory. Based on the CoreMark benchmark, the
performance achieved thanks to the ART Accelerator is equivalent to 0 wait state program
execution from the flash memory at a CPU frequency up to 180 MHz.
3.3 Memory protection unit
The memory protection unit (MPU) is used to manage the CPU accesses to memory to
prevent one task to accidentally corrupt the memory or resources used by any other active
task. This memory area is organized into up to 8 protected areas that can in turn be divided
up into 8 subareas. The protection area sizes are between 32 bytes and the whole 4
gigabytes of addressable memory.
The MPU is especially helpful for applications where some critical or certified code has to be
protected against the misbehavior of other tasks. It is usually managed by an RTOS (real-
time operating system). If a program accesses a memory location that is prohibited by the
MPU, the RTOS can detect it and act. In an RTOS environment, the kernel can dynamically
update the MPU area setting, based on the process to be executed.
The MPU is optional and can be bypassed for applications that do not need it.
DS9405 Rev 13 21/240
44

<!-- Page 22 -->

Functional overview STM32F427xx STM32F429xx
3.4 Embedded flash memory
The devices embed 512 bytes of OTP memory, and a flash memory of up to 2 Mbytes
available for storing programs and data.
3.5 CRC (cyclic redundancy check) calculation unit
The CRC (cyclic redundancy check) calculation unit is used to get a CRC code from a 32-bit
data word and a fixed generator polynomial.
Among other applications, CRC-based techniques are used to verify data transmission or
storage integrity. In the scope of the EN/IEC 60335-1 standard, they offer a means of
verifying the flash memory integrity. The CRC calculation unit helps compute a software
signature during runtime, to be compared with a reference signature generated at link-time
and stored at a given memory location.
3.6 Embedded SRAM
All devices embed:
• Up to 256Kbytes of system SRAM including 64Kbytes of CCM (core coupled memory)
data RAM
RAM memory is accessed (read/write) at CPU clock speed with 0 wait states.
• 4 Kbytes of backup SRAM
This area is accessible only from the CPU. Its content is protected against possible
unwanted write accesses, and is retained in Standby or VBAT mode.
3.7 Multi-AHB bus matrix
The 32-bit multi-AHB bus matrix interconnects all the masters (CPU, DMAs, Ethernet, USB
HS, LCD-TFT, and DMA2D) and the slaves (Flash memory, RAM, FMC, AHB, and APB
peripherals) and ensures a seamless and efficient operation even when several high-speed
peripherals work simultaneously.
22/240 DS9405 Rev 13

<!-- Page 23 -->


|  |  | GP DMA2 |  |  |  |  |  |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1MEM_AMD |  |  | 2P_AMD |  |  |  |  |
|  |  |  |  | E |  |  |  |
|  |  |  |  |  |  |  |  |


| sub-D | sub-S |
| --- | --- |
|  |  |


|  |  |  |  |  |  |  |  |  |  | DCODE |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
|  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |


| SRAM2 16 Kbyte |
| --- |
| SRAM3 64 Kbyte |

STM32F427xx STM32F429xx Functional overview
Figure 5. STM32F427xx and STM32F429xx Multi-AHB matrix
ARM GP GP MAC USB OTG
Cortex-M4 DMA1 DMA2 Ethernet HS
ICODE
DCODE
Bus matrix-S
3.8 DMA controller (DMA)
The devices feature two general-purpose dual-port DMAs (DMA1 and DMA2) with 8
streams each. They are able to manage memory-to-memory, peripheral-to-memory, and
memory-to-peripheral transfers. They feature dedicated FIFOs for APB/AHB peripherals,
support burst transfer and are designed to provide the maximum peripheral bandwidth
(AHB/APB).
The two DMA controllers support circular buffer management, so that no specific code is
needed when the controller reaches the end of the buffer. The two DMA controllers also
have a double buffering feature, which automates the use and switching of two memory
buffers without requiring any special code.
Each stream is connected to dedicated hardware DMA requests, with support for software
trigger on each stream. Configuration is made by software and transfer sizes between
source and destination are independent.
DS9405 Rev 13 23/240
44
LECCA
Flash
memory
SRAM1
112 Kbyte
SRAM2
16 Kbyte
AHB2
peripherals
AHB1
peripherals
FMC external
MemCtl
sub-I sub-D sub-S IP_AMD 1MEM_AMD 2MEM_AMD 2P_AMD M_TENREHTE M_SH_BSU
64-Kbyte Chrom ART Accelerator
LCD-TFT
CCM data RAM (DMA2D)
SRAM3
64 Kbyte
APB1
APB2
MS30421V6
M_TFT-DCL D2AMD

<!-- Page 24 -->

Functional overview STM32F427xx STM32F429xx
The DMA can be used with the main peripherals:
• SPI and I2S
• I2C
• USART
• General-purpose, basic, and advanced-control timers TIMx
• DAC
• SDIO
• Camera interface (DCMI)
• ADC
• SAI1.
3.9 Flexible memory controller (FMC)
All devices embed an FMC. It has four Chip Select outputs supporting the following modes:
PCCard/Compact flash, SDRAM/LPSDR SDRAM, SRAM, PSRAM, NOR flash and NAND
flash.
Functionality overview:
• 8-,16-, 32-bit data bus width
• Read FIFO for SDRAM controller
• Write FIFO
• Maximum FMC_CLK/FMC_SDCLK frequency for synchronous accesses is 90MHz.
LCD parallel interface
The FMC can be configured to interface seamlessly with most graphic LCD controllers. It
supports the Intel 8080 and Motorola 6800 modes, and is flexible enough to adapt to
specific LCD interfaces. This LCD parallel interface capability makes it easy to build cost-
effective graphic applications using LCD modules with embedded controllers or high-
performance solutions using external controllers with dedicated acceleration.
3.10 LCD-TFT controller (available only on STM32F429xx)
The LCD-TFT display controller provides a 24-bit parallel digital RGB (Red, Green, Blue)
and delivers all signals to interface directly to a broad range of LCD and TFT panels up to
XGA (1024x768) resolution with the following features:
• Two display layers with dedicated FIFO (64x32-bit)
• Color look-up table (CLUT) up to 256 colors (256x24-bit) per layer
• Up to eight input color formats selectable per layer
• Flexible blending between two layers using alpha value (per pixel or constant)
• Flexible programmable parameters for each layer
• Color keying (transparency color)
• Up to four programmable interrupt events.
24/240 DS9405 Rev 13

<!-- Page 25 -->

STM32F427xx STM32F429xx Functional overview
3.11 Chrom-ART Accelerator™ (DMA2D)
The Chrom-Art Accelerator™ (DMA2D) is a graphic accelerator, which offers advanced bit
blitting, row data copy and pixel format conversion. It supports the following functions:
• Rectangle filling with a fixed color
• Rectangle copy
• Rectangle copy with pixel format conversion
• Rectangle composition with blending and pixel format conversion.
Various image format coding are supported, from indirect 4 bpp color mode up to 32 bpp
direct color. It embeds dedicated memory to store color lookup tables.
An interrupt can be generated when an operation is complete or at a programmed
watermark.
All the operations are fully automatized and are running independently from the CPU or the
DMAs.
3.12 Nested vectored interrupt controller (NVIC)
The devices embed a nested vectored interrupt controller able to manage 16 priority levels,
and handle up to 91 maskable interrupt channels plus the 16 interrupt lines of the Cortex®-
M4 with FPU core.
• Closely coupled NVIC gives low-latency interrupt processing
• Interrupt entry vector table address passed directly to the core
• Allows early processing of interrupts
• Processing of late arriving, higher-priority interrupts
• Support tail chaining
• Processor state automatically saved
• Interrupt entry restored on interrupt exit with no instruction overhead
This hardware block provides flexible interrupt management features with minimum interrupt
latency.
3.13 External interrupt/event controller (EXTI)
The external interrupt/event controller consists of 23 edge-detector lines used to generate
interrupt/event requests. Each line can be independently configured to select the trigger
event (rising edge, falling edge, both) and can be masked independently. A pending register
maintains the status of the interrupt requests. The EXTI can detect an external line with a
pulse width shorter than the Internal APB2 clock period. Up to 168 GPIOs can be connected
to the 16 external interrupt lines.
3.14 Clocks and startup
On reset the 16 MHz internal RC oscillator is selected as the default CPU clock. The
16MHz internal RC oscillator is factory-trimmed to offer 1% accuracy over the full
temperature range. The application can then select as system clock either the RC oscillator
or an external 4-26 MHz clock source. This clock can be monitored for failure. If a failure is
DS9405 Rev 13 25/240
44

<!-- Page 26 -->

Functional overview STM32F427xx STM32F429xx
detected, the system automatically switches back to the internal RC oscillator and a
software interrupt is generated (if enabled). This clock source is input to a PLL thus allowing
to increase the frequency up to 180MHz. Similarly, full interrupt management of the PLL
clock entry is available when necessary (for example if an indirectly used external oscillator
fails).
Several prescalers allow the configuration of the two AHB buses, the high-speed APB
(APB2) and the low-speed APB (APB1) domains. The maximum frequency of the two AHB
buses is 180MHz while the maximum frequency of the high-speed APB domains is
90MHz. The maximum allowed frequency of the low-speed APB domain is 45MHz.
The devices embed a dedicated PLL (PLLI2S) and PLLSAI, which allows to achieve audio
class performance. In this case, the I2S master clock can generate all standard sampling
frequencies from 8kHz to 192kHz.
3.15 Boot modes
At startup, boot pins are used to select one out of three boot options:
• Boot from user flash
• Boot from system memory
• Boot from embedded SRAM
The bootloader is located in system memory. It is used to reprogram the flash memory
through a serial interface. Refer to application note AN2606 for details.
3.16 Power supply schemes
• V = 1.7 to 3.6 V: external power supply for I/Os and the internal regulator (when
DD
enabled), provided externally through V pins.
DD
• V , V = 1.7 to 3.6 V: external analog power supplies for ADC, DAC, reset blocks,
SSA DDA
RCs, and PLL. V and V must be connected to V and V , respectively.
DDA SSA DD SS
• V = 1.65 to 3.6 V: power supply for RTC, external clock 32 kHz oscillator and
BAT
backup registers (through power switch) when V is not present.
DD
Note: The V
DD
/V
DDA
minimum value of 1.7V is obtained with the use of an external power supply
supervisor (refer to Section3.17.2: Internal reset OFF). Refer to Table3: Voltage regulator
configuration mode versus device operating mode to identify the packages supporting this
option.
3.17 Power supply supervisor
3.17.1 Internal reset ON
On packages embedding the PDR_ON pin, the power supply supervisor is enabled by
holding PDR_ON high. On the other package, the power supply supervisor is always
enabled.
The device has an integrated power-on reset (POR)/ power-down reset (PDR) circuitry
coupled with a brownout reset (BOR) circuitry. At power-on, POR/PDR is always active and
ensures proper operation starting from 1.8V. After the 1.8V POR threshold level is
26/240 DS9405 Rev 13

<!-- Page 27 -->


|  |  |
| --- | --- |
|  |  |


|  |  |
| --- | --- |


|  |  |
| --- | --- |

STM32F427xx STM32F429xx Functional overview
reached, the option byte loading process starts, either to confirm or modify default BOR
thresholds, or to disable BOR permanently. Three BOR thresholds are available through
option bytes. The device remains in reset mode when V is below a specified threshold,
DD
V , or V , without the need for an external reset circuit.
POR/PDR BOR
The device also features an embedded programmable voltage detector (PVD) that monitors
the V /V power supply and compares it to the V threshold. An interrupt can be
DD DDA PVD
generated when V /V drops below the V threshold and/or when V /V is
DD DDA PVD DD DDA
higher than the V threshold. The interrupt service routine can then generate a warning
PVD
message and/or put the MCU into a safe state. The PVD is enabled by software.
3.17.2 Internal reset OFF
This feature is available only on packages featuring the PDR_ON pin. The internal power-on
reset (POR) / power-down reset (PDR) circuitry is disabled through the PDR_ON pin.
An external power supply supervisor should monitor V and should maintain the device in
DD
reset mode as long as V is below a specified threshold. PDR_ON should be connected to
DD
this external power supply supervisor. Refer to Figure6: Power supply supervisor
interconnection with internal reset OFF.
Figure 6. Power supply supervisor interconnection with internal reset OFF
V
DD
External V power supply supervisor
DD
Ext. reset controller active when
V < 1.7 V
DD
PDR_ON
Application reset
NRST signal (optional)
V
DD
MS31383V3
The V specified threshold, below which the device must be maintained under reset, is
DD
1.7V (see Figure7).
A comprehensive set of power-saving mode allows to design low-power applications.
When the internal reset is OFF, the following integrated features are no more supported:
• The integrated power-on reset (POR) / power-down reset (PDR) circuitry is disabled
• The brownout reset (BOR) circuitry must be disabled
• The embedded programmable voltage detector (PVD) is disabled
• V functionality is no more available and V pin should be connected to V .
BAT BAT DD
All packages, except for the LQFP100, allow to disable the internal reset through the
PDR_ON signal.
DS9405 Rev 13 27/240
44

<!-- Page 28 -->

Functional overview STM32F427xx STM32F429xx
Figure 7. PDR_ON control with internal reset OFF
VDD
PDR = 1.7 V
time
Reset by other source than
power supply supervisor
NRST
PDR_ON PDR_ON
time
MS19009V6
3.18 Voltage regulator
The regulator has four operating modes:
• Regulator ON
– Main regulator mode (MR)
– Low-power regulator (LPR)
– Power-down
• Regulator OFF
3.18.1 Regulator ON
On packages embedding the BYPASS_REG pin, the regulator is enabled by holding
BYPASS_REG low. On all other packages, the regulator is always enabled.
There are three power modes configured by software when the regulator is ON:
• MR mode used in Run/sleep modes or in Stop modes
– In Run/Sleep mode
The MR mode is used either in the normal mode (default mode) or the over-drive
mode (enabled by software). Different voltages scaling are provided to reach the
best compromise between maximum frequency and dynamic power consumption.
28/240 DS9405 Rev 13

<!-- Page 29 -->


| Voltage regulator configuration | Run mode | Sleep mode | Stop mode | Standby mode |
| --- | --- | --- | --- | --- |
| Normal mode | MR | MR | MR or LPR | - |
| Over-drive mode(2) | MR | MR | - | - |
| Under-drive mode | - | - | MR or LPR | - |
| Power-down mode | - | - | - | Yes |

STM32F427xx STM32F429xx Functional overview
The overdrive mode allows operating at a higher frequency than the normal mode
for a given voltage scaling.
– In Stop modes
The MR can be configured in two ways during stop mode:
MR operates in normal mode (default mode of MR in stop mode)
MR operates in underdrive mode (reduced leakage mode).
• LPR is used in the Stop modes:
The LP regulator mode is configured by software when entering Stop mode.
Like the MR mode, the LPR can be configured in two ways during stop mode:
– LPR operates in normal mode (default mode when LPR is ON)
– LPR operates in underdrive mode (reduced leakage mode).
• Power-down is used in Standby mode.
The Power-down mode is activated only when entering in Standby mode. The regulator
output is in high impedance and the kernel circuitry is powered down, inducing zero
consumption. The contents of the registers and SRAM are lost.
Refer to Table3 for a summary of voltage regulator modes versus device operating modes.
Two external ceramic capacitors should be connected on V and V pin. Refer to
CAP_1 CAP_2
Figure22: Power supply scheme and Table19: VCAP1/VCAP2 operating conditions.
All packages have the regulator ON feature.
Table 3. Voltage regulator configuration mode versus device operating mode(1)
Voltage regulator
Run mode Sleep mode Stop mode Standby mode
configuration
Normal mode MR MR MR or LPR -
Over-drive
MR MR - -
mode(2)
Under-drive mode - - MR or LPR -
Power-down
- - - Yes
mode
1. ‘-’ means that the corresponding configuration is not available.
2. The over-drive mode is not available when V = 1.7 to 2.1V.
DD
3.18.2 Regulator OFF
This feature is available only on packages featuring the BYPASS_REG pin. The regulator is
disabled by holding BYPASS_REG high. The regulator OFF mode allows to supply
externally a V voltage source through V and V pins.
12 CAP_1 CAP_2
Since the internal voltage scaling is not managed internally, the external voltage value must
be aligned with the targeted maximum frequency. Refer to Table17: General operating
conditions.The two 2.2µF ceramic capacitors should be replaced by two 100nF decoupling
capacitors. Refer to Figure22: Power supply scheme.
When the regulator is OFF, there is no more internal monitoring on V . An external power
12
supply supervisor should be used to monitor the V of the logic power domain. PA0 pin
12
should be used for this purpose, and act as a power-on reset on V power domain.
12
DS9405 Rev 13 29/240
44

<!-- Page 30 -->


|  |
| --- |
|  |


|  |
| --- |
|  |


|  |  |
| --- | --- |


|  |  |
| --- | --- |


|  |  |
| --- | --- |


|  |  |
| --- | --- |


|  |  |
| --- | --- |

Functional overview STM32F427xx STM32F429xx
In regulator OFF mode, the following features are no more supported:
• PA0 cannot be used as a GPIO pin since it allows to reset a part of the V logic power
12
domain, which is not reset by the NRST pin.
• As long as PA0 is kept low, the debug mode cannot be used under power-on reset. As
a consequence, PA0 and NRST pins must be managed separately if the debug
connection under reset or prereset is required.
• The overdrive and underdrive modes are not available.
• The Standby mode is not available.
Figure 8. Regulator OFF
V12
External V power
CAP_1/2
supply supervisor Application reset
Ext. reset controller active signal (optional)
when V < Min V
CAP_1/2 12
V
DD PA0 NRST
V
DD
BYPASS_REG
V12
V
CAP_1
V
CAP_2
ai18498V3
The following conditions must be respected:
• V should always be higher than V and V to avoid current injection
DD CAP_1 CAP_2
between power domains.
• If the time for V and V to reach V minimum value is faster than the time for
CAP_1 CAP_2 12
V to reach 1.7V, then PA0 should be kept low to cover both conditions: until V
DD CAP_1
and V reach V minimum value and until V reaches 1.7V (see Figure9).
CAP_2 12 DD
• Otherwise, if the time for V and V to reach V minimum value is slower
CAP_1 CAP_2 12
than the time for V to reach 1.7V, then PA0 could be asserted low externally (see
DD
Figure10).
• If V and V go below V minimum value and V is higher than 1.7V, then a
CAP_1 CAP_2 12 DD
reset must be asserted on PA0 pin.
Note: The minimum value of V depends on the maximum frequency targeted in the application
12
(see Table17: General operating conditions).
30/240 DS9405 Rev 13

<!-- Page 31 -->


|  |  |  |
| --- | --- | --- |
|  |  |  |
|  |  |  |
|  |  | NRST |


|  |  |  |
| --- | --- | --- |
|  |  |  |
|  |  |  |
|  |  | NRST PA0 asserted externally |

STM32F427xx STM32F429xx Functional overview
Figure 9. Startup in regulator OFF: slow V slope
DD
- power-down reset risen after V /V stabilization
CAP_1 CAP_2
V
DD
PDR = 1.7 V or 1.8 V
V / V
CAP_1 CAP_2
V
12
Min V
12
time
NRST
time
ai18491f
1. This figure is valid whatever the internal reset mode (ON or OFF).
Figure 10. Startup in regulator OFF mode: fast V slope
DD
- power-down reset risen before V /V stabilization
CAP_1 CAP_2
V
DD
PDR = 1.7 V or 1.8 V
V / V
CAP_1 CAP_2
V
12
Min V
12
time
NRST
PA0 asserted externally
time
ai18492e
1. This figure is valid whatever the internal reset mode (ON or OFF).
DS9405 Rev 13 31/240
44

<!-- Page 32 -->


| Package | Regulator ON | Regulator OFF | Internal reset ON | Internal reset OFF |
| --- | --- | --- | --- | --- |
| LQFP100 | Yes | No | Yes | No |
| LQFP144, LQFP208 |  |  | Yes PDR_ON set to V DD | Yes PDR_ON connected to an external power supply supervisor |
| WLCSP143, LQFP176, UFBGA169, UFBGA176, TFBGA216 | Yes BYPASS_REG set to V SS | Yes BYPASS_REG set to V DD |  |  |

Functional overview STM32F427xx STM32F429xx
3.18.3 Regulator ON/OFF and internal reset ON/OFF availability
Table 4. Regulator ON/OFF and internal reset ON/OFF availability
Package Regulator ON Regulator OFF Internal reset ON Internal reset OFF
LQFP100 Yes No
Yes No
LQFP144,
LQFP208 Yes
Yes PDR_ON
WLCSP143,
LQFP176, Yes Yes PDR_ON set to connected to an
V external power
UFBGA169, BYPASS_REG set BYPASS_REG set DD
supply supervisor
UFBGA176, to V to V
SS DD
TFBGA216
3.19 Real-time clock (RTC), backup SRAM, and backup registers
The backup domain includes:
• The real-time clock (RTC)
• 4 Kbytes of backup SRAM
• 20 backup registers
The real-time clock (RTC) is an independent BCD timer/counter. Dedicated registers contain
the second, minute, hour (in 12/24 hour), weekday, date, month, year, in BCD (binary-coded
decimal) format. Correction for 28, 29 (leap year), 30, and 31 day of the month are
performed automatically. The RTC provides a programmable alarm and programmable
periodic interrupts with wake-up from Stop and Standby modes. The subseconds value is
also available in binary format.
It is clocked by a 32.768 kHz external crystal, resonator or oscillator, the internal low-power
RC oscillator or the high-speed external clock divided by 128. The internal low-speed RC
has a typical frequency of 32 kHz. The RTC can be calibrated using an external 512 Hz
output to compensate for any natural quartz deviation.
Two alarm registers are used to generate an alarm at a specific time and calendar fields can
be independently masked for alarm comparison. To generate a periodic interrupt, a 16-bit
programmable binary autoreload downcounter with programmable resolution is available
and allows automatic wake-up and periodic alarms from every 120 µs to every 36 hours.
A 20-bit prescaler is used for the time base clock. It is by default configured to generate a
time base of 1 second from a clock at 32.768 kHz.
The 4-Kbyte backup SRAM is an EEPROM-like memory area. It can be used to store data,
which need to be retained in VBAT and standby mode. This memory area is disabled by
default to minimize power consumption (see Section3.20: Low-power modes). It can be
enabled by software.
The backup registers are 32-bit registers used to store 80 bytes of user application data
when V power is not present. Backup registers are not reset by a system, a power reset,
DD
or when the device wakes up from the Standby mode (see Section3.20: Low-power
modes).
32/240 DS9405 Rev 13

<!-- Page 33 -->


| Voltage regulator configuration | Main regulator (MR) | Low-power regulator (LPR) |
| --- | --- | --- |
| Normal mode | MR ON | LPR ON |
| Under-drive mode | MR in under-drive mode | LPR in under-drive mode |

STM32F427xx STM32F429xx Functional overview
Additional 32-bit registers contain the programmable alarm subseconds, seconds, minutes,
hours, day, and date.
Like backup SRAM, the RTC and backup registers are supplied through a switch that is
powered either from the V supply when present or from the V pin.
DD BAT
3.20 Low-power modes
The devices support three low-power modes to achieve the best compromise between low-
power consumption, short startup time and available wake-up sources:
• Sleep mode
In Sleep mode, only the CPU is stopped. All peripherals continue to operate and can
wake up the CPU when an interrupt/event occurs.
• Stop mode
The Stop mode achieves the lowest power consumption while retaining the contents of
SRAM and registers. All clocks in the 1.2 V domain are stopped, the PLL, the HSI RC
and the HSE crystal oscillators are disabled.
The voltage regulator can be put either in main regulator mode (MR) or in low-power
mode (LPR). Both modes can be configured as follows (see Table5: Voltage regulator
modes in stop mode):
– Normal mode (default mode when MR or LPR is enabled)
– Underdrive mode.
The device can be woken up from the Stop mode by any of the EXTI lines (the EXTI
line source can be one of the 16 external lines, the PVD output, the RTC alarm / wake-
up / tamper / time stamp events, the USB OTG FS/HS wake-up or the Ethernet wake-
up).
Table 5. Voltage regulator modes in stop mode
Voltage regulator
Main regulator (MR) Low-power regulator (LPR)
configuration
Normal mode MR ON LPR ON
Under-drive mode MR in under-drive mode LPR in under-drive mode
• Standby mode
The Standby mode is used to achieve the lowest power consumption. The internal
voltage regulator is switched off so that the entire 1.2 V domain is powered off. The
PLL, the HSI RC and the HSE crystal oscillators are also switched off. After entering
Standby mode, the SRAM and register contents are lost except for registers in the
backup domain and the backup SRAM when selected.
The device exits the Standby mode when an external reset (NRST pin), an IWDG reset,
a rising edge on the WKUP pin, or an RTC alarm / wake-up / tamper /time stamp event
occurs.
The standby mode is not supported when the embedded voltage regulator is bypassed,
and an external power controls the 1.2V domain.
DS9405 Rev 13 33/240
44

<!-- Page 34 -->

Functional overview STM32F427xx STM32F429xx
3.21 V operation
BAT
The V pin allows to power the device V domain from an external battery, an external
BAT BAT
supercapacitor, or from V when no external battery and an external supercapacitor are
DD
present.
V operation is activated when V is not present.
BAT DD
The V pin supplies the RTC, the backup registers, and the backup SRAM.
BAT
Note: When the microcontroller is supplied from V , external interrupts and RTC alarm/events
BAT
do not exit it from V
BAT
operation.
When PDR_ON pin is not connected to V
DD
(Internal Reset OFF), the V
BAT
functionality is
no more available and V
BAT
pin should be connected to VDD.
3.22 Timers and watchdogs
The devices include two advanced-control timers, eight general-purpose timers, two basic
timers and two watchdog timers.
All timer counters can be frozen in debug mode.
Table6 compares the features of the advanced-control, general-purpose and basic timers.
34/240 DS9405 Rev 13

<!-- Page 35 -->


| Timer type | Timer | Counter resolution | Counter type | Prescaler factor | DMA request generation | Capture/ compare channels | Complementary output | Max interface clock (MHz) | Max timer clock (MHz) (1) |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Advanced -control | TIM1, TIM8 | 16-bit | Up, Down, Up/down | Any integer between 1 and 65536 | Yes | 4 | Yes | 90 | 180 |
| General purpose | TIM2, TIM5 | 32-bit | Up, Down, Up/down | Any integer between 1 and 65536 | Yes | 4 | No | 45 | 90/180 |
|  | TIM3, TIM4 | 16-bit | Up, Down, Up/down | Any integer between 1 and 65536 | Yes | 4 | No | 45 | 90/180 |
|  | TIM9 | 16-bit | Up | Any integer between 1 and 65536 | No | 2 | No | 90 | 180 |
|  | TIM10 , TIM11 | 16-bit | Up | Any integer between 1 and 65536 | No | 1 | No | 90 | 180 |
|  | TIM12 | 16-bit | Up | Any integer between 1 and 65536 | No | 2 | No | 45 | 90/180 |
|  | TIM13 , TIM14 | 16-bit | Up | Any integer between 1 and 65536 | No | 1 | No | 45 | 90/180 |
| Basic | TIM6, TIM7 | 16-bit | Up | Any integer between 1 and 65536 | Yes | 0 | No | 45 | 90/180 |

STM32F427xx STM32F429xx Functional overview
Table 6. Timer feature comparison
Max
Max
DMA Capture/ timer
Timer Counter Counter Prescaler Complementary interface
Timer request compare clock
type resolution type factor output clock
generation channels (MHz)
(MHz)
(1)
Any
Up, integer
Advanced TIM1,
16-bit Down, between 1 Yes 4 Yes 90 180
-control TIM8
Up/down and
65536
Any
Up, integer
TIM2,
32-bit Down, between 1 Yes 4 No 45 90/180
TIM5
Up/down and
65536
Any
Up, integer
TIM3,
16-bit Down, between 1 Yes 4 No 45 90/180
TIM4
Up/down and
65536
Any
integer
TIM9 16-bit Up between 1 No 2 No 90 180
and
General 65536
purpose
Any
TIM10 integer
, 16-bit Up between 1 No 1 No 90 180
TIM11 and
65536
Any
integer
TIM12 16-bit Up between 1 No 2 No 45 90/180
and
65536
Any
TIM13 integer
, 16-bit Up between 1 No 1 No 45 90/180
TIM14 and
65536
Any
integer
TIM6,
Basic 16-bit Up between 1 Yes 0 No 45 90/180
TIM7
and
65536
1. The maximum timer clock is either 90 or 180MHz depending on TIMPRE bit configuration in the RCC_DCKCFGR register.
DS9405 Rev 13 35/240
44

<!-- Page 36 -->

Functional overview STM32F427xx STM32F429xx
3.22.1 Advanced-control timers (TIM1, TIM8)
The advanced-control timers (TIM1, TIM8) can be seen as three-phase PWM generators
multiplexed on six channels. They have complementary PWM outputs with programmable
inserted dead times. They can also be considered as complete general-purpose timers.
Their four independent channels can be used for:
• Input capture
• Output compare
• PWM generation (edge- or center-aligned modes)
• One-pulse mode output
If configured as standard 16-bit timers, they have the same features as the general-purpose
TIMx timers. If configured as 16-bit PWM generators, they have full modulation capability (0-
100%).
The advanced-control timer can work together with the TIMx timers via the Timer Link
feature for synchronization or event chaining.
TIM1 and TIM8 support independent DMA request generation.
3.22.2 General-purpose timers (TIMx)
There are ten synchronizable general-purpose timers embedded in the STM32F42x devices
(see Table6 for differences).
• TIM2, TIM3, TIM4, TIM5
The STM32F42x include 4 full-featured general-purpose timers: TIM2, TIM5, TIM3,
and TIM4. The TIM2 and TIM5 timers are based on a 32-bit autoreload
up/downcounter and a 16-bit prescaler. The TIM3 and TIM4 timers are based on a 16-
bit auto-reload up/downcounter and a 16-bit prescaler. They all feature 4 independent
channels for input capture/output compare, PWM, or one-pulse mode output. This
gives up to 16 input capture/output compare/PWMs on the largest packages.
The TIM2, TIM3, TIM4, TIM5 general-purpose timers can work together, or with the
other general-purpose timers and the advanced-control timers TIM1 and TIM8 via the
Timer Link feature for synchronization or event chaining.
Any of these general-purpose timers can be used to generate PWM outputs.
TIM2, TIM3, TIM4, TIM5 all have independent DMA request generation. They can
handle quadrature (incremental) encoder signals and the digital outputs from 1 to 4
hall-effect sensors.
• TIM9, TIM10, TIM11, TIM12, TIM13, and TIM14
These timers are based on a 16-bit auto-reload upcounter and a 16-bit prescaler.
TIM10, TIM11, TIM13, and TIM14 feature one independent channel, whereas TIM9
and TIM12 have two independent channels for input capture/output compare, PWM or
one-pulse mode output. They can be synchronized with the TIM2, TIM3, TIM4, TIM5
full-featured general-purpose timers. They can also be used as simple time bases.
3.22.3 Basic timers TIM6 and TIM7
These timers are mainly used for DAC trigger and waveform generation. They can also be
used as a generic 16-bit time base.
TIM6 and TIM7 support independent DMA request generation.
36/240 DS9405 Rev 13

<!-- Page 37 -->


|  | Analog filter | Digital filter |
| --- | --- | --- |
| Pulse width of suppressed spikes | ≥ 50 ns | Programmable length from 1 to 15 I2C peripheral clocks |

STM32F427xx STM32F429xx Functional overview
3.22.4 Independent watchdog
The independent watchdog is based on a 12-bit downcounter and 8-bit prescaler. It is
clocked from an independent 32 kHz internal RC and as it operates independently from the
main clock, it can operate in Stop and Standby modes. It can be used either as a watchdog
to reset the device when a problem occurs, or as a free-running timer for application timeout
management. It is hardware- or software-configurable through the option bytes.
3.22.5 Window watchdog
The window watchdog is based on a 7-bit downcounter that can be set as free-running. It
can be used as a watchdog to reset the device when a problem occurs. It is clocked from
the main clock. It has an early warning interrupt capability and the counter can be frozen in
debug mode.
3.22.6 SysTick timer
This timer is dedicated to real-time operating systems, but could also be used as a standard
downcounter. It features:
• A 24-bit downcounter
• Autoreload capability
• Maskable system interrupt generation when the counter reaches 0
• Programmable clock source.
2
3.23 Inter-integrated circuit interface ( I C)
Up to three I²C bus interfaces can operate in multimaster and slave modes. They can
support the standard (up to 100kHz), and fast (up to 400kHz) modes. They support the
7/10-bit addressing mode and the 7-bit dual addressing mode (as slave). A hardware CRC
generation/verification is embedded.
They can be served by DMA and they support SMBus 2.0/PMBus.
The devices also include programmable analog and digital noise filters (see Table7).
Table 7. Comparison of I2C analog and digital filters
Analog filter Digital filter
Pulse width of Programmable length from 1 to 15
≥ 50 ns
suppressed spikes I2C peripheral clocks
3.24 Universal synchronous/asynchronous receiver transmitters
(USART)
The devices embed four universal synchronous/asynchronous receiver transmitters
(USART1, USART2, USART3, and USART6) and four universal asynchronous receiver
transmitters (UART4, UART5, UART7, and UART8).
These six interfaces provide asynchronous communication, IrDA SIR ENDEC support,
multiprocessor communication mode, single-wire half-duplex communication mode and
have LIN Master/Slave capability. The USART1 and USART6 interfaces are able to
DS9405 Rev 13 37/240
44

<!-- Page 38 -->


| USART name | Standard features | Modem (RTS/CTS) | LIN | SPI master | irDA | Smartcard (ISO7816) | Max. baud rate in Mbit/s (oversampling by 16) | Max. baud rate in Mbit/s (oversampling by 8) | APB mapping |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| USART1 | X | X | X | X | X | X | 5.62 | 11.25 | APB2 (max. 90MHz) |
| USART2 | X | X | X | X | X | X | 2.81 | 5.62 | APB1 (max. 45MHz) |
| USART3 | X | X | X | X | X | X | 2.81 | 5.62 | APB1 (max. 45MHz) |
| UART4 | X | - | X | - | X | - | 2.81 | 5.62 | APB1 (max. 45MHz) |
| UART5 | X | - | X | - | X | - | 2.81 | 5.62 | APB1 (max. 45MHz) |
| USART6 | X | X | X | X | X | X | 5.62 | 11.25 | APB2 (max. 90MHz) |
| UART7 | X | - | X | - | X | - | 2.81 | 5.62 | APB1 (max. 45MHz) |
| UART8 | X | - | X | - | X | - | 2.81 | 5.62 | APB1 (max. 45MHz) |

Functional overview STM32F427xx STM32F429xx
communicate at speeds of up to 11.25 Mbit/s. The other available interfaces communicate
at up to 5.62bit/s.
USART1, USART2, USART3 and USART6 also provide hardware management of the CTS
and RTS signals, Smart Card mode (ISO 7816 compliant) and SPI-like communication
capability. All interfaces can be served by the DMA controller.
Table 8. USART feature comparison(1)
Max. baud Max. baud
USART Standard Modem SPI Smartcard rate in Mbit/s rate in Mbit/s APB
LIN irDA
name features (RTS/CTS) master (ISO7816) (oversampling (oversampling mapping
by 16) by 8)
APB2
USART1 X X X X X X 5.62 11.25 (max.
90MHz)
APB1
USART2 X X X X X X 2.81 5.62 (max.
45MHz)
APB1
USART3 X X X X X X 2.81 5.62 (max.
45MHz)
APB1
UART4 X - X - X - 2.81 5.62 (max.
45MHz)
APB1
UART5 X - X - X - 2.81 5.62 (max.
45MHz)
APB2
USART6 X X X X X X 5.62 11.25 (max.
90MHz)
APB1
UART7 X - X - X - 2.81 5.62 (max.
45MHz)
APB1
UART8 X - X - X - 2.81 5.62 (max.
45MHz)
1. X = feature supported.
3.25 Serial peripheral interface (SPI)
The devices feature up to six SPIs in slave and master modes in full-duplex and simplex
communication modes. SPI1, SPI4, SPI5, and SPI6 can communicate at up to 45Mbits/s,
SPI2 and SPI3 can communicate at up to 22.5Mbit/s. The 3-bit prescaler gives 8 master
mode frequencies and the frame is configurable to 8 bits or 16 bits. The hardware CRC
generation/verification supports basic SD Card/MMC modes. All SPIs can be served by the
DMA controller.
The SPI interface can be configured to operate in TI mode for communications in master
mode and slave mode.
38/240 DS9405 Rev 13

<!-- Page 39 -->

STM32F427xx STM32F429xx Functional overview
2
3.26 Inter-integrated sound (I S)
Two standard I2S interfaces (multiplexed with SPI2 and SPI3) are available. They can be
operated in master or slave mode, in full duplex and simplex communication modes, and
can be configured to operate with a 16-/32-bit resolution as an input or output channel.
Audio sampling frequencies from 8kHz up to 192kHz are supported. When either or both of
the I2S interfaces is/are configured in master mode, the master clock can be output to the
external DAC/CODEC at 256 times the sampling frequency.
All I2Sx can be served by the DMA controller.
Note: For I2S2 full-duplex mode, I2S2_CK and I2S2_WS signals can be used only on GPIO Port
B and GPIO Port D.
3.27 Serial Audio interface (SAI1)
The serial audio interface (SAI1) is based on two independent audio sub-blocks which can
operate as transmitter or receiver with their FIFO. Many audio protocols are supported by
each block: I2S standards, LSB or MSB-justified, PCM/DSP, TDM, AC’97 and SPDIF
output, supporting audio sampling frequencies from 8kHz up to 192kHz. Both sub-blocks
can be configured in master or in slave mode.
In master mode, the master clock can be output to the external DAC/CODEC at 256 times of
the sampling frequency.
The two sub-blocks can be configured in synchronous mode when full-duplex mode is
required.
SAI1 can be served by the DMA controller.
3.28 Audio PLL (PLLI2S)
The devices feature an additional dedicated PLL for audio I2S and SAI applications. It allows
to achieve error-free I2S sampling clock accuracy without compromising on the CPU
performance, while using USB peripherals.
The PLLI2S configuration can be modified to manage an I2S/SAI sample rate change
without disabling the main PLL (PLL) used for CPU, USB and Ethernet interfaces.
The audio PLL can be programmed with very low error to obtain sampling rates ranging
from 8KHz to 192KHz.
In addition to the audio PLL, a master clock input pin can be used to synchronize the
I2S/SAI flow with an external PLL (or Codec output).
3.29 Audio and LCD PLL(PLLSAI)
An additional PLL dedicated to audio and LCD-TFT is used for SAI1 peripheral in case the
PLLI2S is programmed to achieve another audio sampling frequency (49.152MHz or
11.2896MHz) and the audio application requires both sampling frequencies simultaneously.
The PLLSAI is also used to generate the LCD-TFT clock.
DS9405 Rev 13 39/240
44

<!-- Page 40 -->

Functional overview STM32F427xx STM32F429xx
3.30 Secure digital input/output interface (SDIO)
An SD/SDIO/MMC host interface is available, that supports MultiMediaCard System
Specification Version 4.2 in three different databus modes: 1-bit (default), 4-bit and 8-bit.
The interface allows data transfer at up to 48 MHz, and is compliant with the SD Memory
Card Specification Version 2.0.
The SDIO Card Specification Version 2.0 is also supported with two different databus
modes: 1-bit (default) and 4-bit.
The current version supports only one SD/SDIO/MMC4.2 card at any one time and a stack
of MMC4.1 or previous.
In addition to SD/SDIO/MMC, this interface is fully compliant with the CE-ATA digital
protocol Rev1.1.
3.31 Ethernet MAC interface with dedicated DMA and IEEE 1588
support
The devices provide an IEEE-802.3-2002-compliant media access controller (MAC) for
ethernet LAN communications through an industry-standard medium-independent interface
(MII) or a reduced medium-independent interface (RMII). The microcontroller requires an
external physical interface device (PHY) to connect to the physical LAN bus (twisted-pair,
fiber, etc.). The PHY is connected to the device MII port using 17 signals for MII or 9 signals
for RMII, and can be clocked using the 25MHz (MII) from the microcontroller.
The devices include the following features:
• Supports 10 and 100 Mbit/s rates
• Dedicated DMA controller allowing high-speed transfers between the dedicated SRAM
and the descriptors (see the STM32F4xx reference manual for details)
• Tagged MAC frame support (VLAN support)
• Half-duplex (CSMA/CD) and full-duplex operation
• MAC control sublayer (control frames) support
• 32-bit CRC generation and removal
• Several address filtering modes for physical and multicast address (multicast and
group addresses)
• 32-bit status code for each transmitted or received frame
• Internal FIFOs to buffer transmit and receive frames. The transmit FIFO and the
receive FIFO are both 2 Kbytes.
• Supports hardware PTP (precision time protocol) in accordance with IEEE 1588 2008
(PTP V2) with the time stamp comparator connected to the TIM2 input
• Triggers interrupt when system time becomes greater than target time
3.32 Controller area network (bxCAN)
The two CANs are compliant with the 2.0A and B (active) specifications with a bitrate up to 1
Mbit/s. They can receive and transmit standard frames with 11-bit identifiers as well as
extended frames with 29-bit identifiers. Each CAN has three transmit mailboxes, two receive
40/240 DS9405 Rev 13

<!-- Page 41 -->

STM32F427xx STM32F429xx Functional overview
FIFOS with 3 stages and 28 shared scalable filter banks (all of them can be used even if one
CAN is used). 256 bytes of SRAM are allocated for each CAN.
3.33 Universal serial bus on-the-go full-speed (OTG_FS)
The devices embed an USB OTG full-speed device/host/OTG peripheral with integrated
transceivers. The USB OTG FS peripheral is compliant with the USB 2.0 specification and
with the OTG 1.0 specification. It has software-configurable endpoint setting and supports
suspend/resume. The USB OTG full-speed controller requires a dedicated 48 MHz clock
that is generated by a PLL connected to the HSE oscillator. The major features are:
• Combined Rx and Tx FIFO size of 320 × 35 bits with dynamic FIFO sizing
• Supports the session request protocol (SRP) and host negotiation protocol (HNP)
• 4 bidirectional endpoints
• 8 host channels with periodic OUT support
• HNP/SNP/IP inside (no need for any external resistor)
• For OTG/Host modes, a power switch is needed in case bus-powered devices are
connected
3.34 Universal serial bus on-the-go high-speed (OTG_HS)
The devices embed a USB OTG high-speed (up to 480Mb/s) device/host/OTG peripheral.
The USB OTG HS supports both full-speed and high-speed operations. It integrates the
transceivers for full-speed operation (12MB/s) and features a UTMI low-pin interface (ULPI)
for high-speed operation (480MB/s). When using the USB OTG HS in HS mode, an
external PHY device connected to the ULPI is required.
The USB OTG HS peripheral is compliant with the USB 2.0 specification and with the OTG
1.0 specification. It has software-configurable endpoint setting and supports
suspend/resume. The USB OTG full-speed controller requires a dedicated 48MHz clock
that is generated by a PLL connected to the HSE oscillator.
The major features are:
• Combined Rx and Tx FIFO size of 1 Kbit × 35 with dynamic FIFO sizing
• Supports the session request protocol (SRP) and host negotiation protocol (HNP)
• 6 bidirectional endpoints
• 12 host channels with periodic OUT support
• Internal FS OTG PHY support
• External HS or HS OTG operation supporting ULPI in SDR mode. The OTG PHY is
connected to the microcontroller ULPI port through 12 signals. It can be clocked using
the 60MHz output.
• Internal USB DMA
• HNP/SNP/IP inside (no need for any external resistor)
• for OTG/Host modes, a power switch is needed in case bus-powered devices are
connected
DS9405 Rev 13 41/240
44

<!-- Page 42 -->

Functional overview STM32F427xx STM32F429xx
3.35 Digital camera interface (DCMI)
The devices embed a camera interface that can connect with camera modules and CMOS
sensors through an 8-bit to 14-bit parallel interface, to receive video data. The camera
interface can sustain a data transfer rate up to 54Mbyte/s at 54MHz. It features:
• Programmable polarity for the input pixel clock and synchronization signals
• Parallel data communication can be 8-, 10-, 12- or 14-bit
• Supports 8-bit progressive video monochrome or raw bayer format, YCbCr 4:2:2
progressive video, RGB 565 progressive video or compressed data (like JPEG)
• Supports continuous mode or snapshot (a single frame) mode
• Capability to automatically crop the image
3.36 True random number generator (RNG)
The RNG is a true random number generator that provides full entropy outputs to the
application as 32-bit samples. It is composed of a live entropy source (analog) and an
internal conditioning component.
All devices embed an RNG that delivers 32-bit random numbers generated by an integrated
analog circuit.
3.37 General-purpose input/outputs (GPIOs)
Each of the GPIO pins can be configured by software as output (push-pull or open-drain,
with or without pull-up or pull-down), as input (floating, with or without pull-up or pull-down)
or as peripheral alternate function. Most of the GPIO pins are shared with digital or analog
alternate functions. All GPIOs are high-current-capable and have speed selection to better
manage internal noise, power consumption and electromagnetic emission.
The I/O configuration can be locked if needed by following a specific sequence in order to
avoid spurious writing to the I/Os registers.
Fast I/O handling allowing maximum I/O toggling up to 90MHz.
3.38 Analog-to-digital converters (ADCs)
Three 12-bit analog-to-digital converters are embedded and each ADC shares up to 16
external channels, performing conversions in the single-shot or scan mode. In scan mode,
automatic conversion is performed on a selected group of analog inputs.
Additional logic functions embedded in the ADC interface allow:
• Simultaneous sample and hold
• Interleaved sample and hold
The ADC can be served by the DMA controller. An analog watchdog feature allows very
precise monitoring of the converted voltage of one, some or all selected channels. An
interrupt is generated when the converted voltage is outside the programmed thresholds.
To synchronize A/D conversion and timers, the ADCs could be triggered by any of TIM1,
TIM2, TIM3, TIM4, TIM5, or TIM8 timer.
42/240 DS9405 Rev 13

<!-- Page 43 -->

STM32F427xx STM32F429xx Functional overview
3.39 Temperature sensor
The temperature sensor has to generate a voltage that varies linearly with temperature. The
conversion range is between 1.7 V and 3.6 V. The temperature sensor is internally
connected to the same input channel as V , ADC1_IN18, which is used to convert the
BAT
sensor output voltage into a digital value. When the temperature sensor and V
BAT
conversion are enabled at the same time, only V conversion is performed.
BAT
As the offset of the temperature sensor varies from chip to chip due to process variation, the
internal temperature sensor is mainly suitable for applications that detect temperature
changes instead of absolute temperatures. If an accurate temperature reading is needed,
then an external temperature sensor part should be used.
3.40 Digital-to-analog converter (DAC)
The two 12-bit buffered DAC channels can be used to convert two digital signals into two
analog voltage signal outputs.
This dual digital Interface supports the following features:
• two DAC converters: one for each output channel
• 8-bit or 10-bit monotonic output
• left or right data alignment in 12-bit mode
• synchronized update capability
• noise-wave generation
• triangular-wave generation
• dual DAC channel independent or simultaneous conversions
• DMA capability for each channel
• external triggers for conversion
• input voltage reference V
REF+
Eight DAC trigger inputs are used in the device. The DAC channels are triggered through
the timer update outputs that are also connected to different DMA streams.
3.41 Serial wire JTAG debug port (SWJ-DP)
The Arm SWJ-DP interface is embedded, and is a combined JTAG and serial wire debug
port that enables either a serial wire debug or a JTAG probe to be connected to the target.
Debug is performed using 2 pins only instead of 5 required by the JTAG (JTAG pins could
be re-use as GPIO with alternate function): the JTAG TMS and TCK pins are shared with
SWDIO and SWCLK, respectively, and a specific sequence on the TMS pin is used to
switch between JTAG-DP and SW-DP.
DS9405 Rev 13 43/240
44

<!-- Page 44 -->

Functional overview STM32F427xx STM32F429xx
3.42 Embedded Trace Macrocell™
The Arm Embedded Trace Macrocell provides a greater visibility of the instruction and data
flow inside the CPU core by streaming compressed data at a very high rate from the
STM32F42x through a small number of ETM pins to an external hardware trace port
analyzer (TPA) device. The TPA is connected to a host computer using USB, Ethernet, or
any other high-speed channel. Real-time instruction and data flow activity can be recorded
and then formatted for display on the host computer that runs the debugger software. TPA
hardware is commercially available from common development tool vendors.
The Embedded Trace Macrocell operates with third party debugger software tools.
44/240 DS9405 Rev 13

<!-- Page 45 -->


|  |
| --- |
| 0 |


|  |
| --- |
|  |


|  |
| --- |
|  |


|  |
| --- |
|  |


|  |
| --- |
|  |


|  |
| --- |
|  |


|  |
| --- |
|  |


|  |
| --- |
|  |


|  |
| --- |
|  |


|  |
| --- |
|  |


|  |
| --- |
|  |


|  |
| --- |
|  |


|  |
| --- |
|  |


|  |
| --- |
|  |


|  |
| --- |
|  |


|  |
| --- |
|  |


|  |
| --- |
|  |


|  |
| --- |
|  |


|  | 1 |
| --- | --- |


| 5 |  |
| --- | --- |


|  | 2 |
| --- | --- |


| 4 |  |
| --- | --- |


|  | 3 |
| --- | --- |


| 3 |  |
| --- | --- |


|  | 4 |
| --- | --- |


| 2 |  |
| --- | --- |


|  | 5 |
| --- | --- |


| 1 |  |
| --- | --- |


|  | 6 |
| --- | --- |


| 0 |  |
| --- | --- |


|  | 7 |
| --- | --- |


| 9 |  |
| --- | --- |


|  | 8 |
| --- | --- |


| 8 |  |
| --- | --- |


|  | 9 |
| --- | --- |


| 7 |  |
| --- | --- |


|  | 1 |
| --- | --- |


| 6 |  |
| --- | --- |


|  | 1 |
| --- | --- |


| 5 |  |
| --- | --- |


|  | 1 |
| --- | --- |


| 4 |  |
| --- | --- |


|  | 1 |
| --- | --- |


| 3 |  |
| --- | --- |


|  | 1 |
| --- | --- |


| 2 |  |
| --- | --- |


|  | 1 |
| --- | --- |


| 1 |  |
| --- | --- |


|  | 1 |
| --- | --- |


| 0 |  |
| --- | --- |


|  | 1 |
| --- | --- |


| 9 |  |
| --- | --- |


|  | 1 |
| --- | --- |


| 8 |  |
| --- | --- |


|  | 1 |
| --- | --- |


| 7 |  |
| --- | --- |


|  | 2 |
| --- | --- |


| 6 |  |
| --- | --- |


|  | 2 |
| --- | --- |


| 5 |  |
| --- | --- |


|  | 2 |
| --- | --- |


| 4 |  |
| --- | --- |


|  | 2 |
| --- | --- |


| 3 |  |
| --- | --- |


|  | 2 |
| --- | --- |


| 2 |  |
| --- | --- |


|  | 2 |
| --- | --- |


| 1 |  |
| --- | --- |


| 2 |
| --- |
|  |


| 2 |
| --- |
|  |


| 2 |
| --- |
|  |


| 2 |
| --- |
|  |


| 3 |
| --- |
|  |


| 3 |
| --- |
|  |


| 3 |
| --- |
|  |


| 3 |
| --- |
|  |


| 3 |
| --- |
|  |


| 3 |
| --- |
|  |


| 3 |
| --- |
|  |


| 3 |
| --- |
|  |


| 3 |
| --- |
|  |


| 3 |
| --- |
|  |


| 4 |
| --- |
|  |


| 4 |
| --- |
|  |


| 4 |
| --- |
|  |


| 4 |
| --- |
|  |

STM32F427xx STM32F429xx Pinouts and pin description
4 Pinouts and pin description
Figure 11. STM32F42x LQFP100 pinout
1. The above figure shows the package top view.
DS9405 Rev 13 45/240
85
001
99 89 79 69 59 49 39 29 19 09 98 88 78 68 58 48 38 28 18 08 97 87 77 67
PE2 1 75 VDD
PE3 2 74 VSS
PE4 3 73 VCAP_2
PE5 4 72 PA13
PE6 5 71 PA12
VBAT 6 70 PA 11
7 69 PA10
PC14 8 68 PA9
PC15 9 67 PA8
VSS 10 66 PC9
VDD 11 65 PC8
PH0 12 64 PC7
13 63 PC6
NRST 14 62 PD15
PC0 15 61 PD14
PC1 16 60 PD13
PC2 17 59 PD12
PC3 18 58 PD11
VDD 19 57 PD10
VSSA 20 56 PD9
VREF+ 21 55 PD8
VDDA 22 54 PB15
PA0 23 53 PB14
PA1 24 52 PB13
PA2 25 51 PB12
DDV
62
3AP
SSV
72
SSV
1EP
82
DDV
0EP
92
4AP
9BP
03
5AP
8BP
13
6AP
0TOOB
23
7AP
7BP
33
4CP
6BP
43
5CP
5BP
53
0BP
4BP
63
1BP
3BP
73
2BP
7DP
83
7EP
6DP
93
8EP
5DP
04
9EP
4DP
14
01EP
3DP
24
11EP
2DP
34
21EP
1DP
44
31EP
0DP
54
41EP
21CP
64
51EP
11CP
74
01BP
01CP
84
11BP
51AP
94
1_PACV
41AP
05
DDV
PC13
PH1 LQFP100
ai18495c

<!-- Page 46 -->

Pinouts and pin description STM32F427xx STM32F429xx
Figure 12. STM32F42x WLCSP143 ballout
11 10 9 8 7 6 5 4 3 2 1
A PDR PE1 PB8 PB6 PG15 PG12 PD7 PD5 PD2 PC10 VDD
_ON
B PE4 PE0 PB9 PB7 PB3 PG11 PD4 PD3 PD0 PC11 PA14
BOOT
C VBAT PE3 0 PB5 PB4 PG10 VDD PD1 PC12 PA15 VDD
D PC14 PC13 PE5 PE2 VDD PPGG1133 PA10 PA11 PA13 VSS VCAP
_2
E PC15 VDD PF1 PE6 VSS VDD PG9 PC8 PC9 PA9 PA12
F PF0 PF2 PF4 PF5 PF7 PG14 VSS PD6 PC7 PC6 PA8
G PF3 PF6 PF10 PF9 VDD PG5 PG4 PG6 PG3 PG8 VVDDDD
H PF8 PH1 NRST PC0 VSS PD12 PD13 PD10 VSS VSS PG7
J PH0 PC2 PC3 VDD VDD VDD VDD PE10 PB15 PD14 PG2
K PPCC11 VSSA PA0 PA1 PB1 PF13 PG1 PE11 PB14 PD11 PD15
L VREF VDDA PA2 PA7 PB2 PF14 PE7 PE12 PE15 PD8 VDD
+
M PA3 PA4 PA5 PC4 PF11 PF15 PE8 PE14 PB10 PB12 PD9
N BYPASS_ PA6 PC5 PB0 PF12 PG0 PE9 PE13 PB11 VCAP PB13
REG _1
MS31855V2
1. The above figure shows the package bump view.
46/240 DS9405 Rev 13

<!-- Page 47 -->


|  |
| --- |
| 4 |


|  |
| --- |
| 3 |


|  |
| --- |
| 2 |


|  |
| --- |
| 1 |


|  |
| --- |
| 0 |


|  |
| --- |
| 9 |


|  |
| --- |
| 8 |


|  |
| --- |
| 7 |


|  |
| --- |
| 6 |


|  |
| --- |
| 5 |


|  |
| --- |
| 4 |


|  |
| --- |
| 3 |


|  |
| --- |
| 2 |


|  |
| --- |
| 1 |


|  |
| --- |
| 0 |


|  |
| --- |
| 9 |


|  |
| --- |
| 8 |


|  |
| --- |
| 7 |


|  |
| --- |
| 0 |


|  |
| --- |
| 9 |


|  |
| --- |
| 8 |


|  |
| --- |
| 7 |


|  |
| --- |
| 6 |


|  |
| --- |
| 5 |


|  |
| --- |
| 4 |


|  |
| --- |
| 3 |


|  |
| --- |
| 2 |


|  |
| --- |
| 1 |


|  |
| --- |
| 0 |


|  | 1 |
| --- | --- |


| 8 |  |
| --- | --- |


|  | 2 |
| --- | --- |


| 7 |  |
| --- | --- |


|  | 3 |
| --- | --- |


| 6 |  |
| --- | --- |


|  | 4 |
| --- | --- |


| 5 |  |
| --- | --- |


|  | 5 |
| --- | --- |


| 4 |  |
| --- | --- |


|  | 6 |
| --- | --- |


| 3 |  |
| --- | --- |


|  | 7 |
| --- | --- |


| 2 |  |
| --- | --- |


|  | 8 |
| --- | --- |


| 1 |  |
| --- | --- |


|  | 9 |
| --- | --- |


| 0 |  |
| --- | --- |


|  | 1 |
| --- | --- |


| 9 |  |
| --- | --- |


|  | 1 |
| --- | --- |


| 8 |  |
| --- | --- |


|  | 1 |
| --- | --- |


| 7 |  |
| --- | --- |


|  | 1 |
| --- | --- |


| 6 |  |
| --- | --- |


|  | 1 |
| --- | --- |


| 5 |  |
| --- | --- |


|  | 1 |
| --- | --- |


| 4 |  |
| --- | --- |


|  | 1 |
| --- | --- |


| 3 |  |
| --- | --- |


|  | 1 |
| --- | --- |


| 2 |  |
| --- | --- |


|  | 1 |
| --- | --- |


| 1 |  |
| --- | --- |


|  | 1 |
| --- | --- |


| 0 |  |
| --- | --- |


|  | 2 |
| --- | --- |


| 9 |  |
| --- | --- |


|  | 2 |
| --- | --- |


| 8 |  |
| --- | --- |


|  | 2 |
| --- | --- |


| 7 |  |
| --- | --- |


|  | 2 |
| --- | --- |


| 6 |  |
| --- | --- |


|  | 2 |
| --- | --- |


| 5 |  |
| --- | --- |


|  | 2 |
| --- | --- |
|  | 2 |
|  | 2 |
|  | 2 |
|  | 2 |
|  | 3 |
|  | 3 |
|  | 3 |
|  | 3 |
|  | 3 |
|  | 3 |
|  | 3 |


| 4 |  |
| --- | --- |


| 2 |  |
| --- | --- |
| 1 |  |
| 0 |  |
| 9 |  |
| 8 |  |
| 7 |  |
| 6 |  |
| 5 |  |
| 4 |  |
| 3 |  |


| 3 |
| --- |
|  |


| 3 |
| --- |
|  |


| 3 |
| --- |
|  |


| 4 |
| --- |
|  |


| 4 |
| --- |
|  |


| 4 |
| --- |
|  |


| 4 |
| --- |
|  |


| 4 |
| --- |
|  |


| 4 |
| --- |
|  |


| 4 |
| --- |
|  |


| 4 |
| --- |
|  |


| 4 |
| --- |
|  |


| 4 |
| --- |
|  |


| 5 |
| --- |
|  |


| 5 |
| --- |
|  |


| 5 |
| --- |
|  |


| 5 |
| --- |
|  |


| 5 |
| --- |
|  |


| 6 |
| --- |
|  |


| 6 |
| --- |
|  |


| 6 |
| --- |
|  |


| 6 |
| --- |
|  |


| 6 |
| --- |
|  |


| 6 |
| --- |
|  |


| 6 |
| --- |
|  |


| 6 |
| --- |
|  |


| 6 |
| --- |
|  |


| 7 |
| --- |
|  |


| 7 |
| --- |
|  |

STM32F427xx STM32F429xx Pinouts and pin description
Figure 13. STM32F42x LQFP144 pinout
1. The above figure shows the package top view.
DS9405 Rev 13 47/240
85
V
DD
NO_RDP
1EP 0EP 9BP 8BP
0TOOB
7BP 6BP 5BP 4BP 3BP
51GP
V
DD
V
SS
41GP 31GP 21GP 11GP 01GP
9GP 7DP 6DP
V
DD
V
SS 5DP 4DP 3DP 2DP 1DP 0DP
21CP 11CP 01CP 51AP 41AP
PE2 V
DD
PE3 V
SS
PE4
PE5 PA13
PE6 PA12
VBAT PA11
PC13 PA10
PC14 PA9
PC15 PA8
PF0 PC9
PF1 PC8
PF2 PC7
PF3 PC6
PF4 V
DD
PF5 V
SS
V SS PG8
V DD PG7
PF6 PG6
PF7 PG5
PF8 PG4
PF9 PG3
PF10 PG2
PH0 PD15
PH1 PD14
NRST V
DD
PC0 V
SS
PC1 PD13
PC2 PD12
PC3 PD11
V DD PD10
V SSA PD9
V REF+ PD8
V DDA PB15
PA0 PB14
PA1 PB13
PA2 PB12
441
3AP
341
V
SS
241
V
DD
141
4AP
041
5AP
931
6AP
831
7AP
731
4CP
631
5CP
531
0BP
431
1BP
331
2BP
231
11FP
131
21FP
031 921
V
DD
821
31FP
721
41FP
621
51FP
521
0GP
421
1GP
321
7EP
221
8EP
121
9EP
V
SS
V
DD 01EP 11EP 21EP 31EP 41EP 51EP 01BP 11BP
V
1_PAC
901
V
DD
1 108
2 107
3 106
4 105
5 104
6 103
7 102
8 101
9 100
10 99
11 98
12 97
13 96
14 95
15 94
16 93
17 92
18 91
19 90
20 89
21 88
22 87
23 86
24 85
25 84
73 83 93 04 14 24 34 44 54 64 74 84 94 05 15 25 35 45 55 65 75 85 95 06 27
LQFP144
021
16
911
26
811
36
711
46
611
56
511
66
411
76
311
86
211
96
111
07
011
17
V
CAP_2
26 83
27 82
28 81
29 80
30 79
31 78
32 77
33 76
34 75
35 74
36 73
ai18496b
V
SS

<!-- Page 48 -->


|  |
| --- |
| 2 |


|  |
| --- |
| 1 |


|  |
| --- |
| 0 |


|  |
| --- |
| 9 |


|  |
| --- |
| 8 |


|  |
| --- |
| 7 |


|  |
| --- |
| 6 |


|  |
| --- |
| 5 |


|  |
| --- |
| 4 |


|  |
| --- |
| 3 |


|  |
| --- |
| 2 |


|  |
| --- |
| 1 |


|  |
| --- |
| 0 |


|  |
| --- |
| 9 |


|  |
| --- |
| 8 |


|  |
| --- |
| 7 |


|  |  |  |  |  |
| --- | --- | --- | --- | --- |
| 6 |  | 5 |  | 4 |


|  |  |
| --- | --- |


|  |  |
| --- | --- |


|  |  |
| --- | --- |


|  |  |
| --- | --- |


|  |  |
| --- | --- |


|  |  |
| --- | --- |


|  |  |
| --- | --- |


|  |  |
| --- | --- |


|  |  |
| --- | --- |


|  |  |
| --- | --- |


|  |  |
| --- | --- |


|  |  |
| --- | --- |


|  |  |
| --- | --- |


|  |  |
| --- | --- |


|  |  |
| --- | --- |


|  |  |
| --- | --- |


|  |  |
| --- | --- |


|  |  |
| --- | --- |


|  |  |
| --- | --- |


|  |  |
| --- | --- |


|  |  |
| --- | --- |


|  |  |
| --- | --- |


|  |  |
| --- | --- |


|  |  |
| --- | --- |


|  |  |
| --- | --- |


|  |  |
| --- | --- |


|  |  |
| --- | --- |


|  |  |
| --- | --- |


|  |  |
| --- | --- |


|  |  |
| --- | --- |


|  |  |
| --- | --- |


|  |  |
| --- | --- |


|  |  |
| --- | --- |


|  |  |
| --- | --- |


|  |  |
| --- | --- |


|  |  |
| --- | --- |


|  |  |
| --- | --- |


|  |  |
| --- | --- |


|  |  |
| --- | --- |


|  |  |
| --- | --- |


|  |  |
| --- | --- |


|  |  |
| --- | --- |


|  |  |
| --- | --- |


|  |  |
| --- | --- |


|  |  |
| --- | --- |


|  |  |
| --- | --- |


|  |  |
| --- | --- |


|  |  |
| --- | --- |


|  |  |
| --- | --- |


|  |  |
| --- | --- |


|  |  |
| --- | --- |


| 7 |  |
| --- | --- |


|  |  |
| --- | --- |


| 6 |  |
| --- | --- |


|  |  |
| --- | --- |


| 5 |  |
| --- | --- |


|  |  |
| --- | --- |


| 4 |  |
| --- | --- |


|  |  |
| --- | --- |


| 3 |  |
| --- | --- |


|  |  |
| --- | --- |


| 2 |  |
| --- | --- |


|  |  |
| --- | --- |


| 1 |  |
| --- | --- |


|  |  |
| --- | --- |


| 0 |  |
| --- | --- |


|  |  |
| --- | --- |


| 9 |  |
| --- | --- |


|  |  |
| --- | --- |


| 8 |  |
| --- | --- |


|  |  |
| --- | --- |


| 7 |  |
| --- | --- |


|  |  |
| --- | --- |


| 6 |  |
| --- | --- |


|  |  |
| --- | --- |


| 5 |  |
| --- | --- |


|  |  |
| --- | --- |


| 4 |  |
| --- | --- |


|  |  |
| --- | --- |


| 3 |  |
| --- | --- |


|  |  |
| --- | --- |


| 2 |  |
| --- | --- |


|  |  |
| --- | --- |


| 1 |  |
| --- | --- |


|  |  |
| --- | --- |


| 0 |  |
| --- | --- |


|  |  |
| --- | --- |


| 9 |  |
| --- | --- |


| 4 |
| --- |
|  |


| 5 |  | 5 |  | 5 |  | 5 |  | 5 |  | 5 |  | 5 |  | 5 |  | 5 |  | 5 |  | 6 |  | 6 |  | 6 |  | 6 |  | 6 |  | 6 |  | 6 |  | 6 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |

Pinouts and pin description STM32F427xx STM32F429xx
Figure 14. STM32F42x LQFP176 pinout
MS31878V1
1. The above figure shows the package top view.
48/240 DS9405 Rev 13
7IP 6IP DDV
NO_RDP
1EP 0EP 9BP 8BP
0TOOB
7BP 6BP 5BP 4BP 3BP 51GP DDV SSV 41GP 31GP 21GP 11GP 01GP 9GP 7DP 6DP DDV SSV 5DP 4DP 3DP 2DP 1DP 0DP 21CP 11CP 01CP
PE2
PE3
PE4
PE5
PE6
VBAT VDD
PI8 VSS
PC14 PA13
PC15 PA12
PA11
PA10
PA9
PA8
PC9
PF0 PC8
PF1 PC7
PF2 PC6
PF3 VDD
PF4 VSS
PF5 PG8
PG7
PG6
PF6 PG5
PF7 PG4
PF8 PG3
PF9 PG2
PF10 PD15
PH0 PD14
PH1 VDD
NRST VSS
PC0 PD13
PC1 PD12
PC2 PD11
PC3 PD10
PD9
PD8
VREF+ PB15
PB14
PA0 PB13
PA1 PB12
PA2
671 571 471
3AP
371
GER_SSAPYB
271
DDV
171
4AP
071
5AP
961
6AP
861
7AP
761
4CP
661
5CP
561
0BP
461
1BP
361
2BP
261
11FP
161
21FP
061
SSV
951
DDV
851
31FP
751
41FP
651
51FP
551
0GP
451
1GP
351
7EP 8EP 9EP SSV DDV 01EP 11EP 21EP 31EP 41EP 51EP 01BP
141
11BP 1_PACV DDV
1 132
2 131
3 130
4 129
5 128
6 127
7 126
8 125
9 124
10 123
11 122
12 121
13 120
14 119
15 118
16 117
17 116
18 115
19 114
20 113
21 112
22 111
23 110
24 109
25 108
54 64 74 84 94 05 15 25 35 45 55 65 75 85 95 06 16 26 36 46 56 66 76 86 08
LQFP176
251
96
151
07
051
17
941
27
841
37
741
47
641
57
541
67
441
77
341
87
241
97
VCAP_2
26 107
27 106
28 105
29 104
30 103
31 102
32 101
33 100
34 99
35 98
36
89
4HP 5HP
5IP 4IP
041
18
931
28
51AP
831
38
6HP
41AP
731
48
7HP
DDV
631
58
8HP
SSV
531
68
9HP
3IP
431
78
01HP
2IP
331
88
11HP
PI1
PI0
PH15
PH14
PH13
PC13
PI9
PI10
PI11
VSS
VDD
VSS
VDD
VDD 97
VSSA 37 96
38 95
VDDA 39 94
40 93
41 92
42 91 VDD
PH2 43 90 VSS
PH3 44 PH12

<!-- Page 49 -->

DS9405
Rev
13
49/240
STM32F427xx
STM32F429xx
Pinouts
and
pin
description
Figure 15. STM32F42x LQFP208 pinout
MS30422V2
1. The above figure shows the package top view.
7IP
802
6IP
702
5IP
602
4IP
502
DDV
402
NO_RDP
302
SSV
202
1EP
102
0EP
002
9BP
991
8BP
891
0TOOB
791
7BP
691
6BP
591
5BP
491
4BP
391
3BP
291
51GP
191
7KP
091
6KP
981
5KP
881
4KP
781
3KP
681
DDV
581
SSV
481
41GP
381
31GP
281
21GP
181
11GP
081
01GP
971
9GP
871
51JP
771
41JP
671
31JP
571
21JP
471
7DP
371
6DP
271
DDV
171
SSV
071
5DP
961
4DP
861
3DP
761
2DP
661
1DP
561
0DP
461
21CP
361
11CP
261
01CP
161
51AP
061
41AP
951
DDV
851
3IP
751
PE2 1 156 PI2
PE3 2 155 PI1
PE4 3 154 PI0
PE5 4 153 PH15
PE6 5 152 PH14
VBAT 6 151 PH13
PI8 7 150 VDD
PC13 8 149 VSS
PC14 9 148 VCAP2
PC15 10 147 PA13
PI9 11 146 PA12
PI10 12 145 PA11
PI11 13 144 PA10
VSS 14 143 PA9
VDD 15 142 PA8
PF0 16 141 PC9
PF1 17 140 PC8
PF2 18 139 PC7
PI12 19 138 PC6
PI13 20 137 VDD
PI14 21 136 VSS
PF3 22 135 PG8
PF4 23 134 PG7
PF5 24 133 PG6
VSS 25 LQFP208 132 PG5
VDD 26 131 PG4
PF6 27 130 PG3
PF7 28 129 PG2
PF8 29 128 PK2
PF9 30 127 PK1
PF10 31 126 PK0
PH0 32 125 VSS
PH1 33 124 VDD
NRST 34 123 PJ11
PC0 35 122 PJ10
PC1 36 121 PJ9
PC2 37 120 PJ8
PC3 38 119 PJ7
VDD 39 118 PJ6
VSSA 40 117 PD15
VREF+ 41 116 PD14
VDDA 42 115 VDD
PA0 43 114 VSS
PA1 44 113 PD13
PA2 45 112 PD12
PH2 46 111 PD11
PH3 47 110 PD10
PH4 48 109 PD9
PH5 49 108 PD8
PA3 50 107 PB15
VSS 51 106 PB14
VDD 52 105 PB13
35
4AP
45
5AP
55
6AP
65
7AP
75
4CP
85
5CP
95
DDV
06
SSV
16
0BP
26
1BP
36
2BP
46
51IP
56
0JP
66
1JP
76
2JP
86
3JP
96
4JP
07
11FP
17
21FP
27
SSV
37
DDV
47
31FP
57
41FP
67
51FP
77
0GP
87
1GP
97
7EP
08
8EP
18
9EP
28
SSV
38
DDV
48
01EP
58
11EP
68
21EP
78
31EP
88
41EP
98
51EP
09
01BP
19
11BP
29
1PACV
39
SSV
49
DDV
59
5JP
69
6HP
79
7HP
89
8HP
99
9HP
001
01HP
101
11HP
201
21HP
301
DDV
401
21BP

<!-- Page 50 -->

Pinouts and pin description STM32F427xx STM32F429xx
Figure 16. STM32F42x UFBGA169 ballout
1 2 3 4 5 6 7 8 9 10 11 12 13
A PI6 PI5 PE1 BOOT0 PB4 PG12 PD7 PD3 PC12 PA14 PI3
B PI7 PE2 PI4 PE0 PB7 PB3 PG11 PD6 PD2 PC11 PA15 PI2 PI0
C PE3 PE4 P _O D N R PB9 PB6 PG15 PG10 PD5 PD1 PC10 PI1 PH15 PH14
D PE5 PE6 VDD PB8 PB5 VDD VSS PD4 PD0 VDD VSS VCAP PH13
_2
E PC14 PI9 PI10 PC13 VBAT VDD VSS PA9 PA10 PA11 PA12 PA13 PA8
F PC15 PF0 PF1 VDD VSS VSS VDD VDD PC6 PC7 PC8 PC9 PG8
G PH1 PH0 PF4 PF3 PF2 PC0 VSS VDD VSS VDD PG6 PG7 PG5
H PF10 NRST PF5 VDD PC1 PC2 PC3 VDD PE13 PD11 PD14 PG4 PG2
J VSSA VREF- VREF+ VDDA PA0 VSS VSS PE8 PE14 VSS VDD PD15 PD12
K PA1 PA2 PA3 PA7 PB1 VDD PF14 PE9 PE15 PH9 PD10 PD13 PD9
L PH3 PH2 PH5 PC4 PB2 VDD PF15 PE10 PB10 PH8 PH12 PD8 PB15
M B _ Y R P E A G SS PH4 PA5 PC5 PF11 PF13 PG1 PE11 PB11 PH7 PH11 PB13 PB14
VCAP
N PA4 PA6 PB0 PF12 PG0 PE7 PE12 _1 PH6 PH10 PB12
MS33732V1
1. The above figure shows the package top view.
2. The 4 corners balls, A1, A13, N1 and N13, are not bonded internally and should be left not connected on the PCB.
50/240 DS9405 Rev 13

<!-- Page 51 -->

STM32F427xx STM32F429xx Pinouts and pin description
Figure 17. STM32F42x UFBGA176 ballout
1 2 3 4 5 6 7 8 9 10 11 12 13 14 15
A PE3 PE2 PE1 PE0 PB8 PB5 PG14 PG13 PB4 PB3 PD7 PC12 PA15 PA14 PA13
B PE4 PE5 PE6 PB9 PB7 PB6 PG15 PG12 PG11 PG10 PD6 PD0 PC11 PC10 PA12
C VBAT PI7 PI6 PI5 VDD PDR_ON VDD VDD VDD PG9 PD5 PD1 PI3 PI2 PA11
D PC13 PI8 PI9 PI4 VSS BOOT0 VSS VSS VSS PD4 PD3 PD2 PH15 PI1 PA10
E PC14 PF0 PI10 PI11 PH13 PH14 PI0 PA9
F PC15 VSS VDD PH2 VSS VSS VSS VSS VSS VSS VCAP2 PC9 PA8
G PH0 VSS VDD PH3 VSS VSS VSS VSS VSS VSS VDD PC8 PC7
H PH1 PF2 PF1 PH4 VSS VSS VSS VSS VSS VSS VDD PG8 PC6
J NRST PF3 PF4 PH5 VSS VSS VSS VSS VSS VDD VDD PG7 PG6
K PF7 PF6 PF5 VDD VSS VSS VSS VSS VSS PH12 PG5 PG4 PG3
L PF10 PF9 PF8 BY R P E A G SS_ PH11 PH10 PD15 PG2
M VSSA PC0 PC1 PC2 PC3 PB2 PG1 VSS VSS VCAP_1 PH6 PH8 PH9 PD14 PD13
N VREF- PA1 PA0 PA4 PC4 PF13 PG0 VDD VDD VDD PE13 PH7 PD12 PD11 PD10
P VREF+ PA2 PA6 PA5 PC5 PF12 PF15 PE8 PE9 PE11 PE14 PB12 PB13 PD9 PD8
R VDDA PA3 PA7 PB1 PB0 PF11 PF14 PE7 PE10 PE12 PE15 PB10 PB11 PB14 PB15
ai18497c
1. The above figure shows the package top view.
DS9405 Rev 13 51/240
85

<!-- Page 52 -->

Pinouts and pin description STM32F427xx STM32F429xx
Figure 18. STM32F42x TFBGA216 ballout
1 2 3 4 5 6 7 8 9 10 11 12 13 14 15
A PE4 PE3 PE2 PG14 PE1 PE0 PB8 PB5 PB4 PB3 PD7 PC12 PA15 PA14 PA13
B PE5 PE6 PG13 PB9 PB7 PB6 PG15 PG11 PJ13 PJ12 PD6 PD0 PC11 PC10 PA12
C VBAT PI8 PI4 PK7 PK6 PK5 PG12 PG10 PJ14 PD5 PD3 PD1 PI3 PI2 PA11
D PC13 PF0 PI5 PI7 PI10 PI6 PK4 PK3 PG9 PJ15 PD4 PD2 PH15 PI1 PA10
E PC14 PF1 PI12 PI9 PDR_ BOOT0 VDD VDD VDD VDD VCAP2 PH13 PH14 PI0 PA9
ON
F PC15 VSS PI11 VDD VDD VSS VSS VSS VSS VSS VDD PK1 PK2 PC9 PA8
G PH0 PF2 PI13 PI15 VDD VSS VSS VDD PJ11 PK0 PC8 PC7
H PH1 PF3 PI14 PH4 VDD VSS VSS VDD PJ8 PJ10 PG8 PC6
J NRST PF4 PH5 PH3 VDD VSS VSS VDD PJ7 PJ9 PG7 PG6
K PF7 PF6 PF5 PH2 VDD VSS VSS VSS VSS VSS VDD PJ6 PD15 PB13 PD10
L BYPASS-
PF10 PF9 PF8 PC3 REG VSS VDD VDD VDD VDD VCAP1 PD14 PB12 PD9 PD8
M VSSA PC0 PC1 PC2 PB2 PF12 PG1 PF15 PJ4 PD12 PD13 PG3 PG2 PJ5 PH12
N VREF- PA1 PA0 PA4 PC4 PF13 PG0 PJ3 PE8 PD11 PG5 PG4 PH7 PH9 PH11
P VREF+ PA2 PA6 PA5 PC5 PF14 PJ2 PF11 PE9 PE11 PE14 PB10 PH6 PH8 PH10
R VDDA PA3 PA7 PB1 PB0 PJ0 PJ1 PE7 PE10 PE12 PE15 PE13 PB11 PB14 PB15
MS30423V2
1. The above figure shows the package top view.
52/240 DS9405 Rev 13

<!-- Page 53 -->


| Name | Abbreviation | Definition |
| --- | --- | --- |
| Pin name | Unless otherwise specified in brackets below the pin name, the pin function during and after reset is the same as the actual pin name |  |
| Pin type | S | Supply pin |
|  | I | Input only pin |
|  | I/O | Input / output pin |
| I/O structure | FT | 5V tolerant I/O |
|  | TTa | 3.3V tolerant I/O directly connected to ADC |
|  | B | Dedicated BOOT0 pin |
|  | RST | Bidirectional reset pin with weak pull-up resistor |
| Notes | Unless otherwise specified by a note, all I/Os are set as floating inputs during and after reset |  |
| Alternate functions | Functions selected through GPIOx_AFR registers |  |
| Additional functions | Functions directly selected/enabled through peripheral registers |  |


| Pin number |  |  |  |  |  |  |  | Pin name (function after reset)(1) | epyt niP | erutcurts O / I | setoN | Alternate functions | Additional functions |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 001PFQL | 441PFQL | 961AGBFU | )2(671AGBFU | 671PFQL | 341PSCLW | 802PFQL | 612AGBFT |  |  |  |  |  |  |
| 1 | 1 | B2 | A2 | 1 | D8 | 1 | A3 | PE2 | I/O | FT | - | TRACECLK, SPI4_SCK, SAI1_MCLK_A, ETH_MII_TXD3, FMC_A23, EVENTOUT | - |
| 2 | 2 | C1 | A1 | 2 | C10 | 2 | A2 | PE3 | I/O | FT | - | TRACED0, SAI1_SD_B, FMC_A19, EVENTOUT | - |
| 3 | 3 | C2 | B1 | 3 | B11 | 3 | A1 | PE4 | I/O | FT | - | TRACED1, SPI4_NSS, SAI1_FS_A, FMC_A20, DCMI_D4, LCD_B0, EVENTOUT | - |

STM32F427xx STM32F429xx Pinouts and pin description
Table 9. Legend/abbreviations used in the pinout table
Name Abbreviation Definition
Unless otherwise specified in brackets below the pin name, the pin function during and after
Pin name
reset is the same as the actual pin name
S Supply pin
Pin type I Input only pin
I/O Input / output pin
FT 5V tolerant I/O
TTa 3.3V tolerant I/O directly connected to ADC
I/O structure
B Dedicated BOOT0 pin
RST Bidirectional reset pin with weak pull-up resistor
Notes Unless otherwise specified by a note, all I/Os are set as floating inputs during and after reset
Alternate
Functions selected through GPIOx_AFR registers
functions
Additional
Functions directly selected/enabled through peripheral registers
functions
Table 10. STM32F427xx and STM32F429xx pin and ball definitions
Pin number
Pin name
(function
after reset)(1)
DS9405 Rev 13 53/240
85
epyt
niP
erutcurts
O
/
I
setoN Additional
Alternate functions
functions
001PFQL 441PFQL 961AGBFU )2(671AGBFU 671PFQL 341PSCLW 802PFQL 612AGBFT
TRACECLK,
SPI4_SCK,
1 1 B2 A2 1 D8 1 A3 PE2 I/O FT - SAI1_MCLK_A, -
ETH_MII_TXD3,
FMC_A23, EVENTOUT
TRACED0,
2 2 C1 A1 2 C10 2 A2 PE3 I/O FT - SAI1_SD_B, -
FMC_A19, EVENTOUT
TRACED1, SPI4_NSS,
SAI1_FS_A, FMC_A20,
3 3 C2 B1 3 B11 3 A1 PE4 I/O FT - -
DCMI_D4, LCD_B0,
EVENTOUT

<!-- Page 54 -->


| Pin number |  |  |  |  |  |  |  | Pin name (function after reset)(1) | epyt niP | erutcurts O / I | setoN | Alternate functions | Additional functions |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 001PFQL | 441PFQL | 961AGBFU | )2(671AGBFU | 671PFQL | 341PSCLW | 802PFQL | 612AGBFT |  |  |  |  |  |  |
| 4 | 4 | D1 | B2 | 4 | D9 | 4 | B1 | PE5 | I/O | FT | - | TRACED2, TIM9_CH1, SPI4_MISO, SAI1_SCK_A, FMC_A21, DCMI_D6, LCD_G0, EVENTOUT | - |
| 5 | 5 | D2 | B3 | 5 | E8 | 5 | B2 | PE6 | I/O | FT | - | TRACED3, TIM9_CH2, SPI4_MOSI, SAI1_SD_A, FMC_A22, DCMI_D7, LCD_G1, EVENTOUT | - |
| - | - | - | - | - | - | - | G6 | V SS | S | - | - | - | - |
| - | - | - | - | - | - | - | F5 | V DD | S | - | - | - | - |
| 6 | 6 | E5 | C1 | 6 | C11 | 6 | C1 | V BAT | S | - | - | - | - |
| - | - | NC (3) | D2 | 7 | - | 7 | C2 | PI8 | I/O | FT | (4) (5) | EVENTOUT | TAMP_2 |
| 7 | 7 | E4 | D1 | 8 | D10 | 8 | D1 | PC13 | I/O | FT | (4) (5) | EVENTOUT | TAMP_1 |
| 8 | 8 | E1 | E1 | 9 | D11 | 9 | E1 | PC14- OSC32_IN (PC14) | I/O | FT | (4) (5) | EVENTOUT | OSC32_IN (6) |
| 9 | 9 | F1 | F1 | 10 | E11 | 10 | F1 | PC15- OSC32_OUT (PC15) | I/O | FT | (4) (5) | EVENTOUT | OSC32_ OUT(6) |
| - | - | - | - | - | - | - | G5 | V DD | S | - | - | - | - |
| - | - | E2 | D3 | 11 | - | 11 | E4 | PI9 | I/O | FT | - | CAN1_RX, FMC_D30, LCD_VSYNC, EVENTOUT | - |
| - | - | E3 | E3 | 12 | - | 12 | D5 | PI10 | I/O | FT | - | ETH_MII_RX_ER, FMC_D31, LCD_HSYNC, EVENTOUT | - |
| - | - | NC (3) | E4 | 13 | - | 13 | F3 | PI11 | I/O | FT | - | OTG_HS_ULPI_DIR, EVENTOUT | - |
| - | - | F6 | F2 | 14 | E7 | 14 | F2 | V SS | S | - | - | - | - |

Pinouts and pin description STM32F427xx STM32F429xx
Table 10. STM32F427xx and STM32F429xx pin and ball definitions (continued)
Pin number
Pin name
(function
after reset)(1)
TRACED2, TIM9_CH1,
SPI4_MISO,
4 4 D1 B2 4 D9 4 B1 PE5 I/O FT - SAI1_SCK_A, -
FMC_A21, DCMI_D6,
LCD_G0, EVENTOUT
TRACED3, TIM9_CH2,
SPI4_MOSI,
5 5 D2 B3 5 E8 5 B2 PE6 I/O FT - SAI1_SD_A, -
FMC_A22, DCMI_D7,
LCD_G1, EVENTOUT
- - - - - - - G6 V S - - - -
SS
- - - - - - - F5 V S - - - -
DD
6 6 E5 C1 6 C11 6 C1 V S - - - -
BAT
(4)
NC
- - D2 7 - 7 C2 PI8 I/O FT EVENTOUT TAMP_2
(3) (5)
(4)
7 7 E4 D1 8 D10 8 D1 PC13 I/O FT EVENTOUT TAMP_1
(5)
PC14-
(4)
OSC32_IN
8 8 E1 E1 9 D11 9 E1 OSC32_IN I/O FT EVENTOUT
(5) (6)
(PC14)
PC15-
(4)
OSC32_
9 9 F1 F1 10 E11 10 F1 OSC32_OUT I/O FT EVENTOUT
(5) OUT(6)
(PC15)
- - - - - - - G5 V S - - - -
DD
CAN1_RX, FMC_D30,
- - E2 D3 11 - 11 E4 PI9 I/O FT - LCD_VSYNC, -
EVENTOUT
ETH_MII_RX_ER,
FMC_D31,
- - E3 E3 12 - 12 D5 PI10 I/O FT - -
LCD_HSYNC,
EVENTOUT
NC OTG_HS_ULPI_DIR,
- - E4 13 - 13 F3 PI11 I/O FT - -
(3) EVENTOUT
- - F6 F2 14 E7 14 F2 V S - - - -
SS
54/240 DS9405 Rev 13
epyt
niP
erutcurts
O
/
I
setoN Additional
Alternate functions
functions
001PFQL 441PFQL 961AGBFU )2(671AGBFU 671PFQL 341PSCLW 802PFQL 612AGBFT

<!-- Page 55 -->


| Pin number |  |  |  |  |  |  |  | Pin name (function after reset)(1) | epyt niP | erutcurts O / I | setoN | Alternate functions | Additional functions |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 001PFQL | 441PFQL | 961AGBFU | )2(671AGBFU | 671PFQL | 341PSCLW | 802PFQL | 612AGBFT |  |  |  |  |  |  |
| - | - | F4 | F3 | 15 | E10 | 15 | F4 | V DD | S | - | - | - | - |
| - | 10 | F2 | E2 | 16 | F11 | 16 | D2 | PF0 | I/O | FT | - | I2C2_SDA, FMC_A0, EVENTOUT | - |
| - | 11 | F3 | H3 | 17 | E9 | 17 | E2 | PF1 | I/O | FT | - | I2C2_SCL, FMC_A1, EVENTOUT | - |
| - | 12 | G5 | H2 | 18 | F10 | 18 | G2 | PF2 | I/O | FT | - | I2C2_SMBA, FMC_A2, EVENTOUT | - |
| - | - | - | - | - | - | 19 | E3 | PI12 | I/O | FT | - | LCD_HSYNC, EVENTOUT | - |
| - | - | - | - | - | - | 20 | G3 | PI13 | I/O | FT | - | LCD_VSYNC, EVENTOUT | - |
| - | - | - | - | - | - | 21 | H3 | PI14 | I/O | FT |  | LCD_CLK, EVENTOUT | - |
| - | 13 | G4 | J2 | 19 | G11 | 22 | H2 | PF3 | I/O | FT | (6) | FMC_A3, EVENTOUT | ADC3_IN9 |
| - | 14 | G3 | J3 | 20 | F9 | 23 | J2 | PF4 | I/O | FT | (6) | FMC_A4, EVENTOUT | ADC3_ IN14 |
| - | 15 | H3 | K3 | 21 | F8 | 24 | K3 | PF5 | I/O | FT | (6) | FMC_A5, EVENTOUT | ADC3_ IN15 |
| 10 | 16 | G7 | G2 | 22 | H7 | 25 | H6 | V SS | S | - | - | - | - |
| 11 | 17 | G8 | G3 | 23 | - | 26 | H5 | V DD | S | - | - | - | - |
| - | 18 | NC (3) | K2 | 24 | G10 | 27 | K2 | PF6 | I/O | FT | (6) | TIM10_CH1, SPI5_NSS, SAI1_SD_B, UART7_Rx, FMC_NIORD, EVENTOUT | ADC3_IN4 |
| - | 19 | NC (3) | K1 | 25 | F7 | 28 | K1 | PF7 | I/O | FT | (6) | TIM11_CH1, SPI5_SCK, SAI1_MCLK_B, UART7_Tx, FMC_NREG, EVENTOUT | ADC3_IN5 |

STM32F427xx STM32F429xx Pinouts and pin description
Table 10. STM32F427xx and STM32F429xx pin and ball definitions (continued)
Pin number
Pin name
(function
after reset)(1)
- - F4 F3 15 E10 15 F4 V S - - - -
DD
I2C2_SDA, FMC_A0,
- 10 F2 E2 16 F11 16 D2 PF0 I/O FT - -
EVENTOUT
I2C2_SCL, FMC_A1,
- 11 F3 H3 17 E9 17 E2 PF1 I/O FT - -
EVENTOUT
I2C2_SMBA, FMC_A2,
- 12 G5 H2 18 F10 18 G2 PF2 I/O FT - -
EVENTOUT
LCD_HSYNC,
- - - - - - 19 E3 PI12 I/O FT - -
EVENTOUT
LCD_VSYNC,
- - - - - - 20 G3 PI13 I/O FT - -
EVENTOUT
- - - - - - 21 H3 PI14 I/O FT LCD_CLK, EVENTOUT -
- 13 G4 J2 19 G11 22 H2 PF3 I/O FT (6) FMC_A3, EVENTOUT ADC3_IN9
ADC3_
- 14 G3 J3 20 F9 23 J2 PF4 I/O FT (6) FMC_A4, EVENTOUT
IN14
ADC3_
- 15 H3 K3 21 F8 24 K3 PF5 I/O FT (6) FMC_A5, EVENTOUT
IN15
10 16 G7 G2 22 H7 25 H6 V S - - - -
SS
11 17 G8 G3 23 - 26 H5 V S - - - -
DD
TIM10_CH1,
SPI5_NSS,
NC SAI1_SD_B,
- 18 K2 24 G10 27 K2 PF6 I/O FT (6) ADC3_IN4
(3) UART7_Rx,
FMC_NIORD,
EVENTOUT
TIM11_CH1,
SPI5_SCK,
NC SAI1_MCLK_B,
- 19 K1 25 F7 28 K1 PF7 I/O FT (6) ADC3_IN5
(3) UART7_Tx,
FMC_NREG,
EVENTOUT
DS9405 Rev 13 55/240
85
epyt
niP
erutcurts
O
/
I
setoN Additional
Alternate functions
functions
001PFQL 441PFQL 961AGBFU )2(671AGBFU 671PFQL 341PSCLW 802PFQL 612AGBFT

<!-- Page 56 -->


| Pin number |  |  |  |  |  |  |  | Pin name (function after reset)(1) | epyt niP | erutcurts O / I | setoN | Alternate functions | Additional functions |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 001PFQL | 441PFQL | 961AGBFU | )2(671AGBFU | 671PFQL | 341PSCLW | 802PFQL | 612AGBFT |  |  |  |  |  |  |
| - | 20 | NC (3) | L3 | 26 | H11 | 29 | L3 | PF8 | I/O | FT | (6) | SPI5_MISO, SAI1_SCK_B, TIM13_CH1, FMC_NIOWR, EVENTOUT | ADC3_IN6 |
| - | 21 | NC (3) | L2 | 27 | G8 | 30 | L2 | PF9 | I/O | FT | (6) | SPI5_MOSI, SAI1_FS_B, TIM14_CH1, FMC_CD, EVENTOUT | ADC3_IN7 |
| - | 22 | H1 | L1 | 28 | G9 | 31 | L1 | PF10 | I/O | FT | (6) | FMC_INTR, DCMI_D11, LCD_DE, EVENTOUT | ADC3_IN8 |
| 12 | 23 | G2 | G1 | 29 | J11 | 32 | G1 | PH0-OSC_IN (PH0) | I/O | FT | - | EVENTOUT | OSC_IN(6) |
| 13 | 24 | G1 | H1 | 30 | H10 | 33 | H1 | PH1- OSC_OUT (PH1) | I/O | FT | - | EVENTOUT | OSC_OUT (6) |
| 14 | 25 | H2 | J1 | 31 | H9 | 34 | J1 | NRST | I/O | RS T | - | - | - |
| 15 | 26 | G6 | M2 | 32 | H8 | 35 | M2 | PC0 | I/O | FT | (6) | OTG_HS_ULPI_STP, FMC_SDNWE, EVENTOUT | ADC123_ IN10 |
| 16 | 27 | H5 | M3 | 33 | K11 | 36 | M3 | PC1 | I/O | FT | (6) | ETH_MDC, EVENTOUT | ADC123_ IN11 |
| 17 | 28 | H6 | M4 | 34 | J10 | 37 | M4 | PC2 | I/O | FT | (6) | SPI2_MISO, I2S2ext_SD, OTG_HS_ULPI_DIR, ETH_MII_TXD2, FMC_SDNE0, EVENTOUT | ADC123_ IN12 |
| 18 | 29 | H7 | M5 | 35 | J9 | 38 | L4 | PC3 | I/O | FT | (6) | SPI2_MOSI/I2S2_SD, OTG_HS_ULPI_NXT, ETH_MII_TX_CLK, FMC_SDCKE0, EVENTOUT | ADC123_ IN13 |
| 19 | 30 | - | - | 36 | G7 | 39 | J5 | V DD | S | - | - | - | - |

Pinouts and pin description STM32F427xx STM32F429xx
Table 10. STM32F427xx and STM32F429xx pin and ball definitions (continued)
Pin number
Pin name
(function
after reset)(1)
SPI5_MISO,
SAI1_SCK_B,
NC
- 20 L3 26 H11 29 L3 PF8 I/O FT (6) TIM13_CH1, ADC3_IN6
(3)
FMC_NIOWR,
EVENTOUT
SPI5_MOSI,
NC SAI1_FS_B,
- 21 L2 27 G8 30 L2 PF9 I/O FT (6) ADC3_IN7
(3) TIM14_CH1, FMC_CD,
EVENTOUT
FMC_INTR,
- 22 H1 L1 28 G9 31 L1 PF10 I/O FT (6) DCMI_D11, LCD_DE, ADC3_IN8
EVENTOUT
PH0-OSC_IN
12 23 G2 G1 29 J11 32 G1 I/O FT - EVENTOUT OSC_IN(6)
(PH0)
PH1-
OSC_OUT
13 24 G1 H1 30 H10 33 H1 OSC_OUT I/O FT - EVENTOUT
(6)
(PH1)
RS
14 25 H2 J1 31 H9 34 J1 NRST I/O - - -
T
OTG_HS_ULPI_STP,
ADC123_
15 26 G6 M2 32 H8 35 M2 PC0 I/O FT (6) FMC_SDNWE,
IN10
EVENTOUT
ETH_MDC, ADC123_
16 27 H5 M3 33 K11 36 M3 PC1 I/O FT (6)
EVENTOUT IN11
SPI2_MISO,
I2S2ext_SD,
OTG_HS_ULPI_DIR, ADC123_
17 28 H6 M4 34 J10 37 M4 PC2 I/O FT (6)
ETH_MII_TXD2, IN12
FMC_SDNE0,
EVENTOUT
SPI2_MOSI/I2S2_SD,
OTG_HS_ULPI_NXT,
ADC123_
18 29 H7 M5 35 J9 38 L4 PC3 I/O FT (6) ETH_MII_TX_CLK,
IN13
FMC_SDCKE0,
EVENTOUT
19 30 - - 36 G7 39 J5 V S - - - -
DD
56/240 DS9405 Rev 13
epyt
niP
erutcurts
O
/
I
setoN Additional
Alternate functions
functions
001PFQL 441PFQL 961AGBFU )2(671AGBFU 671PFQL 341PSCLW 802PFQL 612AGBFT

<!-- Page 57 -->


| Pin number |  |  |  |  |  |  |  | Pin name (function after reset)(1) | epyt niP | erutcurts O / I | setoN | Alternate functions | Additional functions |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 001PFQL | 441PFQL | 961AGBFU | )2(671AGBFU | 671PFQL | 341PSCLW | 802PFQL | 612AGBFT |  |  |  |  |  |  |
| - | - | - | - | - | - | - | J6 | V SS | S | - | - | - | - |
| 20 | 31 | J1 | M1 | 37 | K10 | 40 | M1 | V SSA | S | - | - | - | - |
| - | - | J2 | N1 | - | - | - | N1 | V REF– | S | - | - | - | - |
| 21 | 32 | J3 | P1 | 38 | L11 | 41 | P1 | V REF+ | S | - | - | - | - |
| 22 | 33 | J4 | R1 | 39 | L10 | 42 | R1 | V DDA | S | - | - | - | - |
| 23 | 34 | J5 | N3 | 40 | K9 | 43 | N3 | PA0-WKUP (PA0) | I/O | FT | (7) | TIM2_CH1/TIM2_ETR, TIM5_CH1, TIM8_ETR, USART2_CTS, UART4_TX, ETH_MII_CRS, EVENTOUT | ADC123_ IN0/WKUP (6) |
| 24 | 35 | K1 | N2 | 41 | K8 | 44 | N2 | PA1 | I/O | FT | (6) | TIM2_CH2, TIM5_CH2, USART2_RTS, UART4_RX, ETH_MII_RX_CLK/ET H_RMII_REF_CLK, EVENTOUT | ADC123_ IN1 |
| 25 | 36 | K2 | P2 | 42 | L9 | 45 | P2 | PA2 | I/O | FT | (6) | TIM2_CH3, TIM5_CH3, TIM9_CH1, USART2_TX, ETH_MDIO, EVENTOUT | ADC123_ IN2 |
| - | - | L2 | F4 | 43 | - | 46 | K4 | PH2 | I/O | FT | - | ETH_MII_CRS, FMC_SDCKE0, LCD_R0, EVENTOUT | - |
| - | - | L1 | G4 | 44 | - | 47 | J4 | PH3 | I/O | FT | - | ETH_MII_COL, FMC_SDNE0, LCD_R1, EVENTOUT | - |
| - | - | M2 | H4 | 45 | - | 48 | H4 | PH4 | I/O | FT | - | I2C2_SCL, OTG_HS_ULPI_NXT, EVENTOUT | - |
| - | - | L3 | J4 | 46 | - | 49 | J3 | PH5 | I/O | FT | - | I2C2_SDA, SPI5_NSS, FMC_SDNWE, EVENTOUT | - |

STM32F427xx STM32F429xx Pinouts and pin description
Table 10. STM32F427xx and STM32F429xx pin and ball definitions (continued)
Pin number
Pin name
(function
after reset)(1)
- - - - - - - J6 V S - - - -
SS
20 31 J1 M1 37 K10 40 M1 V S - - - -
SSA
- - J2 N1 - - - N1 V S - - - -
REF–
21 32 J3 P1 38 L11 41 P1 V S - - - -
REF+
22 33 J4 R1 39 L10 42 R1 V S - - - -
DDA
TIM2_CH1/TIM2_ETR,
TIM5_CH1, TIM8_ETR,
ADC123_
PA0-WKUP USART2_CTS,
23 34 J5 N3 40 K9 43 N3 I/O FT (7) IN0/WKUP
(PA0) UART4_TX, (6)
ETH_MII_CRS,
EVENTOUT
TIM2_CH2, TIM5_CH2,
USART2_RTS,
UART4_RX, ADC123_
24 35 K1 N2 41 K8 44 N2 PA1 I/O FT (6)
ETH_MII_RX_CLK/ET IN1
H_RMII_REF_CLK,
EVENTOUT
TIM2_CH3, TIM5_CH3,
TIM9_CH1,
ADC123_
25 36 K2 P2 42 L9 45 P2 PA2 I/O FT (6) USART2_TX,
IN2
ETH_MDIO,
EVENTOUT
ETH_MII_CRS,
- - L2 F4 43 - 46 K4 PH2 I/O FT - FMC_SDCKE0, -
LCD_R0, EVENTOUT
ETH_MII_COL,
- - L1 G4 44 - 47 J4 PH3 I/O FT - FMC_SDNE0, -
LCD_R1, EVENTOUT
I2C2_SCL,
- - M2 H4 45 - 48 H4 PH4 I/O FT - OTG_HS_ULPI_NXT, -
EVENTOUT
I2C2_SDA, SPI5_NSS,
- - L3 J4 46 - 49 J3 PH5 I/O FT - FMC_SDNWE, -
EVENTOUT
DS9405 Rev 13 57/240
85
epyt
niP
erutcurts
O
/
I
setoN Additional
Alternate functions
functions
001PFQL 441PFQL 961AGBFU )2(671AGBFU 671PFQL 341PSCLW 802PFQL 612AGBFT

<!-- Page 58 -->


| Pin number |  |  |  |  |  |  |  | Pin name (function after reset)(1) | epyt niP | erutcurts O / I | setoN | Alternate functions | Additional functions |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 001PFQL | 441PFQL | 961AGBFU | )2(671AGBFU | 671PFQL | 341PSCLW | 802PFQL | 612AGBFT |  |  |  |  |  |  |
| 26 | 37 | K3 | R2 | 47 | M11 | 50 | R2 | PA3 | I/O | FT | (6) | TIM2_CH4, TIM5_CH4, TIM9_CH2, USART2_RX, OTG_HS_ULPI_D0, ETH_MII_COL, LCD_B5, EVENTOUT | ADC123_ IN3 |
| 27 | 38 | - | - |  | - | 51 | K6 | V SS | S | - | - | - | - |
| - | - | M1 | L4 | 48 | N11 | - | L5 | BYPASS_ REG | I | FT | - | - | - |
| 28 | 39 | J11 | K4 | 49 | J8 | 52 | K5 | V DD | S | - | - | - | - |
| 29 | 40 | N2 | N4 | 50 | M1 0 | 53 | N4 | PA4 | I/O | TTa | (6) | SPI1_NSS, SPI3_NSS/I2S3_WS, USART2_CK, OTG_HS_SOF, DCMI_HSYNC, LCD_VSYNC, EVENTOUT | ADC12_ IN4 /DAC_ OUT1 |
| 30 | 41 | M3 | P4 | 51 | M9 | 54 | P4 | PA5 | I/O | TTa | (6) | TIM2_CH1/TIM2_ETR, TIM8_CH1N, SPI1_SCK, OTG_HS_ULPI_CK, EVENTOUT | ADC12_ IN5/DAC_ OUT2 |
| 31 | 42 | N3 | P3 | 52 | N10 | 55 | P3 | PA6 | I/O | FT | (6) | TIM1_BKIN, TIM3_CH1, TIM8_BKIN, SPI1_MISO, TIM13_CH1, DCMI_PIXCLK, LCD_G2, EVENTOUT | ADC12_ IN6 |
| 32 | 43 | K4 | R3 | 53 | L8 | 56 | R3 | PA7 | I/O | FT | (6) | TIM1_CH1N, TIM3_CH2, TIM8_CH1N, SPI1_MOSI, TIM14_CH1, ETH_MII_RX_DV/ETH _RMII_CRS_DV, EVENTOUT | ADC12_ IN7 |

Pinouts and pin description STM32F427xx STM32F429xx
Table 10. STM32F427xx and STM32F429xx pin and ball definitions (continued)
Pin number
Pin name
(function
after reset)(1)
TIM2_CH4, TIM5_CH4,
TIM9_CH2,
USART2_RX, ADC123_
26 37 K3 R2 47 M11 50 R2 PA3 I/O FT (6)
OTG_HS_ULPI_D0, IN3
ETH_MII_COL,
LCD_B5, EVENTOUT
27 38 - - - 51 K6 V S - - - -
SS
BYPASS_
- - M1 L4 48 N11 - L5 I FT - - -
REG
28 39 J11 K4 49 J8 52 K5 V S - - - -
DD
SPI1_NSS,
SPI3_NSS/I2S3_WS,
USART2_CK, ADC12_
M1
29 40 N2 N4 50 53 N4 PA4 I/O TTa (6) OTG_HS_SOF, IN4 /DAC_
0
DCMI_HSYNC, OUT1
LCD_VSYNC,
EVENTOUT
TIM2_CH1/TIM2_ETR,
TIM8_CH1N, ADC12_
30 41 M3 P4 51 M9 54 P4 PA5 I/O TTa (6) SPI1_SCK, IN5/DAC_
OTG_HS_ULPI_CK, OUT2
EVENTOUT
TIM1_BKIN,
TIM3_CH1,
TIM8_BKIN,
ADC12_
31 42 N3 P3 52 N10 55 P3 PA6 I/O FT (6) SPI1_MISO,
IN6
TIM13_CH1,
DCMI_PIXCLK,
LCD_G2, EVENTOUT
TIM1_CH1N,
TIM3_CH2,
TIM8_CH1N,
SPI1_MOSI, ADC12_
32 43 K4 R3 53 L8 56 R3 PA7 I/O FT (6)
TIM14_CH1, IN7
ETH_MII_RX_DV/ETH
_RMII_CRS_DV,
EVENTOUT
58/240 DS9405 Rev 13
epyt
niP
erutcurts
O
/
I
setoN Additional
Alternate functions
functions
001PFQL 441PFQL 961AGBFU )2(671AGBFU 671PFQL 341PSCLW 802PFQL 612AGBFT

<!-- Page 59 -->


| Pin number |  |  |  |  |  |  |  | Pin name (function after reset)(1) | epyt niP | erutcurts O / I | setoN | Alternate functions | Additional functions |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 001PFQL | 441PFQL | 961AGBFU | )2(671AGBFU | 671PFQL | 341PSCLW | 802PFQL | 612AGBFT |  |  |  |  |  |  |
| 33 | 44 | L4 | N5 | 54 | M8 | 57 | N5 | PC4 | I/O | FT | (6) | ETH_MII_RXD0/ETH_ RMII_RXD0, EVENTOUT | ADC12_ IN14 |
| 34 | 45 | M4 | P5 | 55 | N9 | 58 | P5 | PC5 | I/O | FT | (6) | ETH_MII_RXD1/ETH_ RMII_RXD1, EVENTOUT | ADC12_ IN15 |
| - | - | - | - | - | J7 | 59 | L7 | V DD | S | - | - | - | - |
| - | - | - | - | - | - | 60 | L6 | VSS | S | - | - | - | - |
| 35 | 46 | N4 | R5 | 56 | N8 | 61 | R5 | PB0 | I/O | FT | (6) | TIM1_CH2N, TIM3_CH3, TIM8_CH2N, LCD_R3, OTG_HS_ULPI_D1, ETH_MII_RXD2, EVENTOUT | ADC12_ IN8 |
| 36 | 47 | K5 | R4 | 57 | K7 | 62 | R4 | PB1 | I/O | FT | (6) | TIM1_CH3N, TIM3_CH4, TIM8_CH3N, LCD_R6, OTG_HS_ULPI_D2, ETH_MII_RXD3, EVENTOUT | ADC12_ IN9 |
| 37 | 48 | L5 | M6 | 58 | L7 | 63 | M5 | PB2-BOOT1 (PB2) | I/O | FT | - | EVENTOUT | - |
| - | - | - | - | - | - | 64 | G4 | PI15 | I/O | FT | - | LCD_R0, EVENTOUT | - |
| - | - | - | - | - | - | 65 | R6 | PJ0 | I/O | FT | - | LCD_R1, EVENTOUT | - |
| - | - | - | - | - | - | 66 | R7 | PJ1 | I/O | FT | - | LCD_R2, EVENTOUT | - |
| - | - | - | - | - | - | 67 | P7 | PJ2 | I/O | FT | - | LCD_R3, EVENTOUT | - |
| - | - | - | - | - | - | 68 | N8 | PJ3 | I/O | FT | - | LCD_R4, EVENTOUT | - |
| - | - | - | - | - | - | 69 | M9 | PJ4 | I/O | FT | - | LCD_R5, EVENTOUT | - |
| - | 49 | M5 | R6 | 59 | M7 | 70 | P8 | PF11 | I/O | FT | - | SPI5_MOSI, FMC_SDNRAS, DCMI_D12, EVENTOUT | - |
| - | 50 | N5 | P6 | 60 | N7 | 71 | M6 | PF12 | I/O | FT | - | FMC_A6, EVENTOUT | - |
| - | 51 | G9 | M8 | 61 | - | 72 | K7 | V SS | S |  | - | - | - |

STM32F427xx STM32F429xx Pinouts and pin description
Table 10. STM32F427xx and STM32F429xx pin and ball definitions (continued)
Pin number
Pin name
(function
after reset)(1)
ETH_MII_RXD0/ETH_
ADC12_
33 44 L4 N5 54 M8 57 N5 PC4 I/O FT (6) RMII_RXD0,
IN14
EVENTOUT
ETH_MII_RXD1/ETH_
ADC12_
34 45 M4 P5 55 N9 58 P5 PC5 I/O FT (6) RMII_RXD1,
IN15
EVENTOUT
- - - - - J7 59 L7 V S - - - -
DD
- - - - - - 60 L6 VSS S - - - -
TIM1_CH2N,
TIM3_CH3,
TIM8_CH2N, LCD_R3, ADC12_
35 46 N4 R5 56 N8 61 R5 PB0 I/O FT (6)
OTG_HS_ULPI_D1, IN8
ETH_MII_RXD2,
EVENTOUT
TIM1_CH3N,
TIM3_CH4,
TIM8_CH3N, LCD_R6, ADC12_
36 47 K5 R4 57 K7 62 R4 PB1 I/O FT (6)
OTG_HS_ULPI_D2, IN9
ETH_MII_RXD3,
EVENTOUT
PB2-BOOT1
37 48 L5 M6 58 L7 63 M5 I/O FT - EVENTOUT -
(PB2)
- - - - - - 64 G4 PI15 I/O FT - LCD_R0, EVENTOUT -
- - - - - - 65 R6 PJ0 I/O FT - LCD_R1, EVENTOUT -
- - - - - - 66 R7 PJ1 I/O FT - LCD_R2, EVENTOUT -
- - - - - - 67 P7 PJ2 I/O FT - LCD_R3, EVENTOUT -
- - - - - - 68 N8 PJ3 I/O FT - LCD_R4, EVENTOUT -
- - - - - - 69 M9 PJ4 I/O FT - LCD_R5, EVENTOUT -
SPI5_MOSI,
FMC_SDNRAS,
- 49 M5 R6 59 M7 70 P8 PF11 I/O FT - -
DCMI_D12,
EVENTOUT
- 50 N5 P6 60 N7 71 M6 PF12 I/O FT - FMC_A6, EVENTOUT -
- 51 G9 M8 61 - 72 K7 V S - - -
SS
DS9405 Rev 13 59/240
85
epyt
niP
erutcurts
O
/
I
setoN Additional
Alternate functions
functions
001PFQL 441PFQL 961AGBFU )2(671AGBFU 671PFQL 341PSCLW 802PFQL 612AGBFT

<!-- Page 60 -->


| Pin number |  |  |  |  |  |  |  | Pin name (function after reset)(1) | epyt niP | erutcurts O / I | setoN | Alternate functions | Additional functions |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 001PFQL | 441PFQL | 961AGBFU | )2(671AGBFU | 671PFQL | 341PSCLW | 802PFQL | 612AGBFT |  |  |  |  |  |  |
| - | 52 | D10 | N8 | 62 | - | 73 | L8 | V DD | S |  | - | - | - |
| - | 53 | M6 | N6 | 63 | K6 | 74 | N6 | PF13 | I/O | FT | - | FMC_A7, EVENTOUT | - |
| - | 54 | K7 | R7 | 64 | L6 | 75 | P6 | PF14 | I/O | FT | - | FMC_A8, EVENTOUT | - |
| - | 55 | L7 | P7 | 65 | M6 | 76 | M8 | PF15 | I/O | FT | - | FMC_A9, EVENTOUT | - |
| - | 56 | N6 | N7 | 66 | N6 | 77 | N7 | PG0 | I/O | FT | - | FMC_A10, EVENTOUT | - |
| - | 57 | M7 | M7 | 67 | K5 | 78 | M7 | PG1 | I/O | FT | - | FMC_A11, EVENTOUT | - |
| 38 | 58 | N7 | R8 | 68 | L5 | 79 | R8 | PE7 | I/O | FT | - | TIM1_ETR, UART7_Rx, FMC_D4, EVENTOUT | - |
| 39 | 59 | J8 | P8 | 69 | M5 | 80 | N9 | PE8 | I/O | FT | - | TIM1_CH1N, UART7_Tx, FMC_D5, EVENTOUT | - |
| 40 | 60 | K8 | P9 | 70 | N5 | 81 | P9 | PE9 | I/O | FT | - | TIM1_CH1, FMC_D6, EVENTOUT | - |
| - | 61 | J6 | M9 | 71 | H3 | 82 | K8 | V SS | S |  | - | - | - |
| - | 62 | G10 | N9 | 72 | J5 | 83 | L9 | V DD | S |  | - | - | - |
| 41 | 63 | L8 | R9 | 73 | J4 | 84 | R9 | PE10 | I/O | FT | - | TIM1_CH2N, FMC_D7, EVENTOUT | - |
| 42 | 64 | M8 | P10 | 74 | K4 | 85 | P10 | PE11 | I/O | FT | - | TIM1_CH2, SPI4_NSS, FMC_D8, LCD_G3, EVENTOUT | - |
| 43 | 65 | N8 | R10 | 75 | L4 | 86 | R10 | PE12 | I/O | FT | - | TIM1_CH3N, SPI4_SCK, FMC_D9, LCD_B4, EVENTOUT | - |
| 44 | 66 | H9 | N11 | 76 | N4 | 87 | R12 | PE13 | I/O | FT | - | TIM1_CH3, SPI4_MISO, FMC_D10, LCD_DE, EVENTOUT | - |
| 45 | 67 | J9 | P11 | 77 | M4 | 88 | P11 | PE14 | I/O | FT | - | TIM1_CH4, SPI4_MOSI, FMC_D11, LCD_CLK, EVENTOUT | - |
| 46 | 68 | K9 | R11 | 78 | L3 | 89 | R11 | PE15 | I/O | FT | - | TIM1_BKIN, FMC_D12, LCD_R7, EVENTOUT | - |

Pinouts and pin description STM32F427xx STM32F429xx
Table 10. STM32F427xx and STM32F429xx pin and ball definitions (continued)
Pin number
Pin name
(function
after reset)(1)
- 52 D10 N8 62 - 73 L8 V S - - -
DD
- 53 M6 N6 63 K6 74 N6 PF13 I/O FT - FMC_A7, EVENTOUT -
- 54 K7 R7 64 L6 75 P6 PF14 I/O FT - FMC_A8, EVENTOUT -
- 55 L7 P7 65 M6 76 M8 PF15 I/O FT - FMC_A9, EVENTOUT -
- 56 N6 N7 66 N6 77 N7 PG0 I/O FT - FMC_A10, EVENTOUT -
- 57 M7 M7 67 K5 78 M7 PG1 I/O FT - FMC_A11, EVENTOUT -
TIM1_ETR,
38 58 N7 R8 68 L5 79 R8 PE7 I/O FT - UART7_Rx, FMC_D4, -
EVENTOUT
TIM1_CH1N,
39 59 J8 P8 69 M5 80 N9 PE8 I/O FT - UART7_Tx, FMC_D5, -
EVENTOUT
TIM1_CH1, FMC_D6,
40 60 K8 P9 70 N5 81 P9 PE9 I/O FT - -
EVENTOUT
- 61 J6 M9 71 H3 82 K8 V S - - -
SS
- 62 G10 N9 72 J5 83 L9 V S - - -
DD
TIM1_CH2N, FMC_D7,
41 63 L8 R9 73 J4 84 R9 PE10 I/O FT - -
EVENTOUT
TIM1_CH2, SPI4_NSS,
42 64 M8 P10 74 K4 85 P10 PE11 I/O FT - FMC_D8, LCD_G3, -
EVENTOUT
TIM1_CH3N,
43 65 N8 R10 75 L4 86 R10 PE12 I/O FT - SPI4_SCK, FMC_D9, -
LCD_B4, EVENTOUT
TIM1_CH3,
SPI4_MISO,
44 66 H9 N11 76 N4 87 R12 PE13 I/O FT - -
FMC_D10, LCD_DE,
EVENTOUT
TIM1_CH4,
45 67 J9 P11 77 M4 88 P11 PE14 I/O FT - SPI4_MOSI, FMC_D11, -
LCD_CLK, EVENTOUT
TIM1_BKIN, FMC_D12,
46 68 K9 R11 78 L3 89 R11 PE15 I/O FT - -
LCD_R7, EVENTOUT
60/240 DS9405 Rev 13
epyt
niP
erutcurts
O
/
I
setoN Additional
Alternate functions
functions
001PFQL 441PFQL 961AGBFU )2(671AGBFU 671PFQL 341PSCLW 802PFQL 612AGBFT

<!-- Page 61 -->


| Pin number |  |  |  |  |  |  |  | Pin name (function after reset)(1) | epyt niP | erutcurts O / I | setoN | Alternate functions | Additional functions |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 001PFQL | 441PFQL | 961AGBFU | )2(671AGBFU | 671PFQL | 341PSCLW | 802PFQL | 612AGBFT |  |  |  |  |  |  |
| 47 | 69 | L9 | R12 | 79 | M3 | 90 | P12 | PB10 | I/O | FT | - | TIM2_CH3, I2C2_SCL, SPI2_SCK/I2S2_CK, USART3_TX, OTG_HS_ULPI_D3, ETH_MII_RX_ER, LCD_G4, EVENTOUT | - |
| 48 | 70 | M9 | R13 | 80 | N3 | 91 | R13 | PB11 | I/O | FT | - | TIM2_CH4, I2C2_SDA, USART3_RX, OTG_HS_ULPI_D4, ETH_MII_TX_EN/ETH_ RMII_TX_EN, LCD_G5, EVENTOUT | - |
| 49 | 71 | N9 | M10 | 81 | N2 | 92 | L11 | V CAP_1 | S | - | - | - | - |
| - | - | - | - | - | H2 | 93 | K9 | V SS | S | - | - | - | - |
| 50 | 72 | F8 | N10 | 82 | J6 | 94 | L10 | V DD | S | - | - | - | - |
| - | - | - | - | - | - | 95 | M14 | PJ5 | I/O | - | - | LCD_R6, EVENTOUT | - |
| - | - | N10 | M11 | 83 | - | 96 | P13 | PH6 | I/O | FT | - | I2C2_SMBA, SPI5_SCK, TIM12_CH1, ETH_MII_RXD2, FMC_SDNE1, DCMI_D8, EVENTOUT | - |
| - | - | M10 | N12 | 84 | - | 97 | N13 | PH7 | I/O | FT | - | I2C3_SCL, SPI5_MISO, ETH_MII_RXD3, FMC_SDCKE1, DCMI_D9, EVENTOUT | - |
| - | - | L10 | M12 | 85 | - | 98 | P14 | PH8 | I/O | FT | - | I2C3_SDA, FMC_D16, DCMI_HSYNC, LCD_R2, EVENTOUT | - |
| - | - | K10 | M13 | 86 | - | 99 | N14 | PH9 | I/O | FT | - | I2C3_SMBA, TIM12_CH2, FMC_D17, DCMI_D0, LCD_R3, EVENTOUT | - |
| - | - | N11 | L13 | 87 | - | 100 | P15 | PH10 | I/O | FT | - | TIM5_CH1, FMC_D18, DCMI_D1, LCD_R4, EVENTOUT | - |

STM32F427xx STM32F429xx Pinouts and pin description
Table 10. STM32F427xx and STM32F429xx pin and ball definitions (continued)
Pin number
Pin name
(function
after reset)(1)
TIM2_CH3, I2C2_SCL,
SPI2_SCK/I2S2_CK,
USART3_TX,
47 69 L9 R12 79 M3 90 P12 PB10 I/O FT - -
OTG_HS_ULPI_D3,
ETH_MII_RX_ER,
LCD_G4, EVENTOUT
TIM2_CH4, I2C2_SDA,
USART3_RX,
OTG_HS_ULPI_D4,
48 70 M9 R13 80 N3 91 R13 PB11 I/O FT - -
ETH_MII_TX_EN/ETH_
RMII_TX_EN, LCD_G5,
EVENTOUT
49 71 N9 M10 81 N2 92 L11 V S - - - -
CAP_1
- - - - - H2 93 K9 V S - - - -
SS
50 72 F8 N10 82 J6 94 L10 V S - - - -
DD
- - - - - - 95 M14 PJ5 I/O - - LCD_R6, EVENTOUT -
I2C2_SMBA,
SPI5_SCK,
TIM12_CH1,
- - N10 M11 83 - 96 P13 PH6 I/O FT - -
ETH_MII_RXD2,
FMC_SDNE1,
DCMI_D8, EVENTOUT
I2C3_SCL,
SPI5_MISO,
- - M10 N12 84 - 97 N13 PH7 I/O FT - ETH_MII_RXD3, -
FMC_SDCKE1,
DCMI_D9, EVENTOUT
I2C3_SDA, FMC_D16,
- - L10 M12 85 - 98 P14 PH8 I/O FT - DCMI_HSYNC, -
LCD_R2, EVENTOUT
I2C3_SMBA,
TIM12_CH2,
- - K10 M13 86 - 99 N14 PH9 I/O FT - -
FMC_D17, DCMI_D0,
LCD_R3, EVENTOUT
TIM5_CH1, FMC_D18,
- - N11 L13 87 - 100 P15 PH10 I/O FT - DCMI_D1, LCD_R4, -
EVENTOUT
DS9405 Rev 13 61/240
85
epyt
niP
erutcurts
O
/
I
setoN Additional
Alternate functions
functions
001PFQL 441PFQL 961AGBFU )2(671AGBFU 671PFQL 341PSCLW 802PFQL 612AGBFT

<!-- Page 62 -->


| Pin number |  |  |  |  |  |  |  | Pin name (function after reset)(1) | epyt niP | erutcurts O / I | setoN | Alternate functions | Additional functions |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 001PFQL | 441PFQL | 961AGBFU | )2(671AGBFU | 671PFQL | 341PSCLW | 802PFQL | 612AGBFT |  |  |  |  |  |  |
| - | - | M11 | L12 | 88 | - | 101 | N15 | PH11 | I/O | FT | - | TIM5_CH2, FMC_D19, DCMI_D2, LCD_R5, EVENTOUT | - |
| - | - | L11 | K12 | 89 | - | 102 | M15 | PH12 | I/O | FT | - | TIM5_CH3, FMC_D20, DCMI_D3, LCD_R6, EVENTOUT | - |
| - | - | E7 | H12 | 90 | - | - | K10 | V SS | S | - | - | - | - |
| - | - | H8 | J12 | 91 | - | 103 | K11 | V DD | S | - | - | - | - |
| 51 | 73 | N12 | P12 | 92 | M2 | 104 | L13 | PB12 | I/O | FT | - | TIM1_BKIN, I2C2_SMBA, SPI2_NSS/I2S2_WS, USART3_CK, CAN2_RX, OTG_HS_ULPI_D5, ETH_MII_TXD0/ETH_ RMII_TXD0, OTG_HS_ID, EVENTOUT | - |
| 52 | 74 | M12 | P13 | 93 | N1 | 105 | K14 | PB13 | I/O | FT | - | TIM1_CH1N, SPI2_SCK/I2S2_CK, USART3_CTS, CAN2_TX, OTG_HS_ULPI_D6, ETH_MII_TXD1/ETH_ RMII_TXD1, EVENTOUT | OTG_HS_ VBUS |
| 53 | 75 | M13 | R14 | 94 | K3 | 106 | R14 | PB14 | I/O | FT | - | TIM1_CH2N, TIM8_CH2N, SPI2_MISO, I2S2ext_SD, USART3_RTS, TIM12_CH1, OTG_HS_DM, EVENTOUT | - |

Pinouts and pin description STM32F427xx STM32F429xx
Table 10. STM32F427xx and STM32F429xx pin and ball definitions (continued)
Pin number
Pin name
(function
after reset)(1)
TIM5_CH2, FMC_D19,
- - M11 L12 88 - 101 N15 PH11 I/O FT - DCMI_D2, LCD_R5, -
EVENTOUT
TIM5_CH3, FMC_D20,
- - L11 K12 89 - 102 M15 PH12 I/O FT - DCMI_D3, LCD_R6, -
EVENTOUT
- - E7 H12 90 - - K10 V S - - - -
SS
- - H8 J12 91 - 103 K11 V S - - - -
DD
TIM1_BKIN,
I2C2_SMBA,
SPI2_NSS/I2S2_WS,
USART3_CK,
CAN2_RX,
51 73 N12 P12 92 M2 104 L13 PB12 I/O FT - -
OTG_HS_ULPI_D5,
ETH_MII_TXD0/ETH_
RMII_TXD0,
OTG_HS_ID,
EVENTOUT
TIM1_CH1N,
SPI2_SCK/I2S2_CK,
USART3_CTS,
CAN2_TX, OTG_HS_
52 74 M12 P13 93 N1 105 K14 PB13 I/O FT -
OTG_HS_ULPI_D6, VBUS
ETH_MII_TXD1/ETH_
RMII_TXD1,
EVENTOUT
TIM1_CH2N,
TIM8_CH2N,
SPI2_MISO,
I2S2ext_SD,
53 75 M13 R14 94 K3 106 R14 PB14 I/O FT - -
USART3_RTS,
TIM12_CH1,
OTG_HS_DM,
EVENTOUT
62/240 DS9405 Rev 13
epyt
niP
erutcurts
O
/
I
setoN Additional
Alternate functions
functions
001PFQL 441PFQL 961AGBFU )2(671AGBFU 671PFQL 341PSCLW 802PFQL 612AGBFT

<!-- Page 63 -->


| Pin number |  |  |  |  |  |  |  | Pin name (function after reset)(1) | epyt niP | erutcurts O / I | setoN | Alternate functions | Additional functions |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 001PFQL | 441PFQL | 961AGBFU | )2(671AGBFU | 671PFQL | 341PSCLW | 802PFQL | 612AGBFT |  |  |  |  |  |  |
| 54 | 76 | L13 | R15 | 95 | J3 | 107 | R15 | PB15 | I/O | FT | - | RTC_REFIN, TIM1_CH3N, TIM8_CH3N, SPI2_MOSI/I2S2_SD, TIM12_CH2, OTG_HS_DP, EVENTOUT | - |
| 55 | 77 | L12 | P15 | 96 | L2 | 108 | L15 | PD8 | I/O | FT | - | USART3_TX, FMC_D13, EVENTOUT | - |
| 56 | 78 | K13 | P14 | 97 | M1 | 109 | L14 | PD9 | I/O | FT | - | USART3_RX, FMC_D14, EVENTOUT | - |
| 57 | 79 | K11 | N15 | 98 | H4 | 110 | K15 | PD10 | I/O | FT | - | USART3_CK, FMC_D15, LCD_B3, EVENTOUT | - |
| 58 | 80 | H10 | N14 | 99 | K2 | 111 | N10 | PD11 | I/O | FT | - | USART3_CTS, FMC_A16, EVENTOUT | - |
| 59 | 81 | J13 | N13 | 100 | H6 | 112 | M10 | PD12 | I/O | FT | - | TIM4_CH1, USART3_RTS, FMC_A17, EVENTOUT | - |
| 60 | 82 | K12 | M15 | 101 | H5 | 113 | M11 | PD13 | I/O | FT | - | TIM4_CH2, FMC_A18, EVENTOUT | - |
| - | 83 | - | - | 102 | - | 114 | J10 | V SS | S |  | - | - | - |
| - | 84 | F7 | J13 | 103 | L1 | 115 | J11 | V DD | S |  | - | - | - |
| 61 | 85 | H11 | M14 | 104 | J2 | 116 | L12 | PD14 | I/O | FT | - | TIM4_CH3, FMC_D0, EVENTOUT | - |
| 62 | 86 | J12 | L14 | 105 | K1 | 117 | K13 | PD15 | I/O | FT | - | TIM4_CH4, FMC_D1, EVENTOUT | - |
| - | - | - | - | - | - | 118 | K12 | PJ6 | I/O | FT | - | LCD_R7, EVENTOUT | - |
| - | - | - | - | - | - | 119 | J12 | PJ7 | I/O | FT | - | LCD_G0, EVENTOUT | - |
| - | - | - | - | - | - | 120 | H12 | PJ8 | I/O | FT | - | LCD_G1, EVENTOUT | - |
| - | - | - | - | - | - | 121 | J13 | PJ9 | I/O | FT | - | LCD_G2, EVENTOUT | - |
| - | - | - | - | - | - | 122 | H13 | PJ10 | I/O | FT | - | LCD_G3, EVENTOUT | - |
| - | - | - | - | - | - | 123 | G12 | PJ11 | I/O | FT | - | LCD_G4, EVENTOUT | - |

STM32F427xx STM32F429xx Pinouts and pin description
Table 10. STM32F427xx and STM32F429xx pin and ball definitions (continued)
Pin number
Pin name
(function
after reset)(1)
RTC_REFIN,
TIM1_CH3N,
TIM8_CH3N,
54 76 L13 R15 95 J3 107 R15 PB15 I/O FT - SPI2_MOSI/I2S2_SD, -
TIM12_CH2,
OTG_HS_DP,
EVENTOUT
USART3_TX,
55 77 L12 P15 96 L2 108 L15 PD8 I/O FT - -
FMC_D13, EVENTOUT
USART3_RX,
56 78 K13 P14 97 M1 109 L14 PD9 I/O FT - -
FMC_D14, EVENTOUT
USART3_CK,
57 79 K11 N15 98 H4 110 K15 PD10 I/O FT - FMC_D15, LCD_B3, -
EVENTOUT
USART3_CTS,
58 80 H10 N14 99 K2 111 N10 PD11 I/O FT - -
FMC_A16, EVENTOUT
TIM4_CH1,
59 81 J13 N13 100 H6 112 M10 PD12 I/O FT - USART3_RTS, -
FMC_A17, EVENTOUT
TIM4_CH2, FMC_A18,
60 82 K12 M15 101 H5 113 M11 PD13 I/O FT - -
EVENTOUT
- 83 - - 102 - 114 J10 V S - - -
SS
- 84 F7 J13 103 L1 115 J11 V S - - -
DD
TIM4_CH3, FMC_D0,
61 85 H11 M14 104 J2 116 L12 PD14 I/O FT - -
EVENTOUT
TIM4_CH4, FMC_D1,
62 86 J12 L14 105 K1 117 K13 PD15 I/O FT - -
EVENTOUT
- - - - - - 118 K12 PJ6 I/O FT - LCD_R7, EVENTOUT -
- - - - - - 119 J12 PJ7 I/O FT - LCD_G0, EVENTOUT -
- - - - - - 120 H12 PJ8 I/O FT - LCD_G1, EVENTOUT -
- - - - - - 121 J13 PJ9 I/O FT - LCD_G2, EVENTOUT -
- - - - - - 122 H13 PJ10 I/O FT - LCD_G3, EVENTOUT -
- - - - - - 123 G12 PJ11 I/O FT - LCD_G4, EVENTOUT -
DS9405 Rev 13 63/240
85
epyt
niP
erutcurts
O
/
I
setoN Additional
Alternate functions
functions
001PFQL 441PFQL 961AGBFU )2(671AGBFU 671PFQL 341PSCLW 802PFQL 612AGBFT

<!-- Page 64 -->


| Pin number |  |  |  |  |  |  |  | Pin name (function after reset)(1) | epyt niP | erutcurts O / I | setoN | Alternate functions | Additional functions |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 001PFQL | 441PFQL | 961AGBFU | )2(671AGBFU | 671PFQL | 341PSCLW | 802PFQL | 612AGBFT |  |  |  |  |  |  |
| - | - | - | - | - | - | 124 | H11 | VDD | I/O | FT | - | - | - |
| - | - | - | - | - | - | 125 | H10 | VSS | I/O | FT | - | - | - |
| - | - | - | - | - | - | 126 | G13 | PK0 | I/O | FT | - | LCD_G5, EVENTOUT | - |
| - | - | - | - | - | - | 127 | F12 | PK1 | I/O | FT | - | LCD_G6, EVENTOUT | - |
| - | - | - | - | - | - | 128 | F13 | PK2 | I/O | FT | - | LCD_G7, EVENTOUT | - |
| - | 87 | H13 | L15 | 106 | J1 | 129 | M13 | PG2 | I/O | FT | - | FMC_A12, EVENTOUT | - |
| - | 88 | NC (3) | K15 | 107 | G3 | 130 | M12 | PG3 | I/O | FT | - | FMC_A13, EVENTOUT | - |
| - | 89 | H12 | K14 | 108 | G5 | 131 | N12 | PG4 | I/O | FT | - | FMC_A14/FMC_BA0, EVENTOUT | - |
| - | 90 | G13 | K13 | 109 | G6 | 132 | N11 | PG5 | I/O | FT | - | FMC_A15/FMC_BA1, EVENTOUT | - |
| - | 91 | G11 | J15 | 110 | G4 | 133 | J15 | PG6 | I/O | FT | - | FMC_INT2, DCMI_D12, LCD_R7, EVENTOUT | - |
| - | 92 | G12 | J14 | 111 | H1 | 134 | J14 | PG7 | I/O | FT | - | USART6_CK, FMC_INT3, DCMI_D13, LCD_CLK, EVENTOUT | - |
| - | 93 | F13 | H14 | 112 | G2 | 135 | H14 | PG8 | I/O | FT | - | SPI6_NSS, USART6_RTS, ETH_PPS_OUT, FMC_SDCLK, EVENTOUT | - |
| - | 94 | J7 | G12 | 113 | D2 | 136 | G10 | V SS | S |  | - | - | - |
| - | 95 | E6 | H13 | 114 | G1 | 137 | G11 | V DD | S |  | - | - | - |
| 63 | 96 | F9 | H15 | 115 | F2 | 138 | H15 | PC6 | I/O | FT | - | TIM3_CH1, TIM8_CH1, I2S2_MCK, USART6_TX, SDIO_D6, DCMI_D0, LCD_HSYNC, EVENTOUT | - |

Pinouts and pin description STM32F427xx STM32F429xx
Table 10. STM32F427xx and STM32F429xx pin and ball definitions (continued)
Pin number
Pin name
(function
after reset)(1)
- - - - - - 124 H11 VDD I/O FT - - -
- - - - - - 125 H10 VSS I/O FT - - -
- - - - - - 126 G13 PK0 I/O FT - LCD_G5, EVENTOUT -
- - - - - - 127 F12 PK1 I/O FT - LCD_G6, EVENTOUT -
- - - - - - 128 F13 PK2 I/O FT - LCD_G7, EVENTOUT -
- 87 H13 L15 106 J1 129 M13 PG2 I/O FT - FMC_A12, EVENTOUT -
NC
- 88 K15 107 G3 130 M12 PG3 I/O FT - FMC_A13, EVENTOUT -
(3)
FMC_A14/FMC_BA0,
- 89 H12 K14 108 G5 131 N12 PG4 I/O FT - -
EVENTOUT
FMC_A15/FMC_BA1,
- 90 G13 K13 109 G6 132 N11 PG5 I/O FT - -
EVENTOUT
FMC_INT2,
- 91 G11 J15 110 G4 133 J15 PG6 I/O FT - DCMI_D12, LCD_R7, -
EVENTOUT
USART6_CK,
FMC_INT3,
- 92 G12 J14 111 H1 134 J14 PG7 I/O FT - -
DCMI_D13, LCD_CLK,
EVENTOUT
SPI6_NSS,
USART6_RTS,
- 93 F13 H14 112 G2 135 H14 PG8 I/O FT - ETH_PPS_OUT, -
FMC_SDCLK,
EVENTOUT
- 94 J7 G12 113 D2 136 G10 V S - - -
SS
- 95 E6 H13 114 G1 137 G11 V S - - -
DD
TIM3_CH1, TIM8_CH1,
I2S2_MCK,
USART6_TX,
63 96 F9 H15 115 F2 138 H15 PC6 I/O FT - -
SDIO_D6, DCMI_D0,
LCD_HSYNC,
EVENTOUT
64/240 DS9405 Rev 13
epyt
niP
erutcurts
O
/
I
setoN Additional
Alternate functions
functions
001PFQL 441PFQL 961AGBFU )2(671AGBFU 671PFQL 341PSCLW 802PFQL 612AGBFT

<!-- Page 65 -->


| Pin number |  |  |  |  |  |  |  | Pin name (function after reset)(1) | epyt niP | erutcurts O / I | setoN | Alternate functions | Additional functions |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 001PFQL | 441PFQL | 961AGBFU | )2(671AGBFU | 671PFQL | 341PSCLW | 802PFQL | 612AGBFT |  |  |  |  |  |  |
| 64 | 97 | F10 | G15 | 116 | F3 | 139 | G15 | PC7 | I/O | FT | - | TIM3_CH2, TIM8_CH2, I2S3_MCK, USART6_RX, SDIO_D7, DCMI_D1, LCD_G6, EVENTOUT | - |
| 65 | 98 | F11 | G14 | 117 | E4 | 140 | G14 | PC8 | I/O | FT | - | TIM3_CH3, TIM8_CH3, USART6_CK, SDIO_D0, DCMI_D2, EVENTOUT | - |
| 66 | 99 | F12 | F14 | 118 | E3 | 141 | F14 | PC9 | I/O | FT | - | MCO2, TIM3_CH4, TIM8_CH4, I2C3_SDA, I2S_CKIN, SDIO_D1, DCMI_D3, EVENTOUT | - |
| 67 | 100 | E13 | F15 | 119 | F1 | 142 | F15 | PA8 | I/O | FT | - | MCO1, TIM1_CH1, I2C3_SCL, USART1_CK, OTG_FS_SOF, LCD_R6, EVENTOUT | - |
| 68 | 101 | E8 | E15 | 120 | E2 | 143 | E15 | PA9 | I/O | FT | - | TIM1_CH2, I2C3_SMBA, USART1_TX, DCMI_D0, EVENTOUT | OTG_FS_ VBUS |
| 69 | 102 | E9 | D15 | 121 | D5 | 144 | D15 | PA10 | I/O | FT | - | TIM1_CH3, USART1_RX, OTG_FS_ID, DCMI_D1, EVENTOUT | - |
| 70 | 103 | E10 | C15 | 122 | D4 | 145 | C15 | PA11 | I/O | FT | - | TIM1_CH4, USART1_CTS, CAN1_RX, LCD_R4, OTG_FS_DM, EVENTOUT | - |
| 71 | 104 | E11 | B15 | 123 | E1 | 146 | B15 | PA12 | I/O | FT | - | TIM1_ETR, USART1_RTS, CAN1_TX, LCD_R5, OTG_FS_DP, EVENTOUT | - |

STM32F427xx STM32F429xx Pinouts and pin description
Table 10. STM32F427xx and STM32F429xx pin and ball definitions (continued)
Pin number
Pin name
(function
after reset)(1)
TIM3_CH2, TIM8_CH2,
I2S3_MCK,
64 97 F10 G15 116 F3 139 G15 PC7 I/O FT - USART6_RX, -
SDIO_D7, DCMI_D1,
LCD_G6, EVENTOUT
TIM3_CH3, TIM8_CH3,
USART6_CK,
65 98 F11 G14 117 E4 140 G14 PC8 I/O FT - -
SDIO_D0, DCMI_D2,
EVENTOUT
MCO2, TIM3_CH4,
TIM8_CH4, I2C3_SDA,
66 99 F12 F14 118 E3 141 F14 PC9 I/O FT - -
I2S_CKIN, SDIO_D1,
DCMI_D3, EVENTOUT
MCO1, TIM1_CH1,
I2C3_SCL,
67 100 E13 F15 119 F1 142 F15 PA8 I/O FT - USART1_CK, -
OTG_FS_SOF,
LCD_R6, EVENTOUT
TIM1_CH2,
I2C3_SMBA, OTG_FS_
68 101 E8 E15 120 E2 143 E15 PA9 I/O FT -
USART1_TX, VBUS
DCMI_D0, EVENTOUT
TIM1_CH3,
USART1_RX,
69 102 E9 D15 121 D5 144 D15 PA10 I/O FT - -
OTG_FS_ID,
DCMI_D1, EVENTOUT
TIM1_CH4,
USART1_CTS,
70 103 E10 C15 122 D4 145 C15 PA11 I/O FT - CAN1_RX, LCD_R4, -
OTG_FS_DM,
EVENTOUT
TIM1_ETR,
USART1_RTS,
71 104 E11 B15 123 E1 146 B15 PA12 I/O FT - CAN1_TX, LCD_R5, -
OTG_FS_DP,
EVENTOUT
DS9405 Rev 13 65/240
85
epyt
niP
erutcurts
O
/
I
setoN Additional
Alternate functions
functions
001PFQL 441PFQL 961AGBFU )2(671AGBFU 671PFQL 341PSCLW 802PFQL 612AGBFT

<!-- Page 66 -->


| Pin number |  |  |  |  |  |  |  | Pin name (function after reset)(1) | epyt niP | erutcurts O / I | setoN | Alternate functions | Additional functions |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 001PFQL | 441PFQL | 961AGBFU | )2(671AGBFU | 671PFQL | 341PSCLW | 802PFQL | 612AGBFT |  |  |  |  |  |  |
| 72 | 105 | E12 | A15 | 124 | D3 | 147 | A15 | PA13 (JTMS- SWDIO) | I/O | FT | - | JTMS-SWDIO, EVENTOUT | - |
| 73 | 106 | D12 | F13 | 125 | D1 | 148 | E11 | V CAP_2 | S |  | - | - | - |
| 74 | 107 | J10 | F12 | 126 | D2 | 149 | F10 | V SS | S |  | - | - | - |
| 75 | 108 | H4 | G13 | 127 | C1 | 150 | F11 | V DD | S |  | - | - | - |
| - | - | D13 | E12 | 128 | - | 151 | E12 | PH13 | I/O | FT | - | TIM8_CH1N, CAN1_TX, FMC_D21, LCD_G2, EVENTOUT | - |
| - | - | C13 | E13 | 129 | - | 152 | E13 | PH14 | I/O | FT | - | TIM8_CH2N, FMC_D22, DCMI_D4, LCD_G3, EVENTOUT | - |
| - | - | C12 | D13 | 130 | - | 153 | D13 | PH15 | I/O | FT | - | TIM8_CH3N, FMC_D23, DCMI_D11, LCD_G4, EVENTOUT | - |
| - | - | B13 | E14 | 131 | - | 154 | E14 | PI0 | I/O | FT | - | TIM5_CH4, SPI2_NSS/I2S2_WS(8), FMC_D24, DCMI_D13, LCD_G5, EVENTOUT | - |
| - | - | C11 | D14 | 132 | - | 155 | D14 | PI1 | I/O | FT | - | SPI2_SCK/I2S2_CK(8), FMC_D25, DCMI_D8, LCD_G6, EVENTOUT | - |
| - | - | B12 | C14 | 133 | - | 156 | C14 | PI2 | I/O | FT | - | TIM8_CH4, SPI2_MISO, I2S2ext_SD, FMC_D26, DCMI_D9, LCD_G7, EVENTOUT | - |
| - | - | A12 | C13 | 134 | - | 157 | C13 | PI3 | I/O | FT | - | TIM8_ETR, SPI2_MOSI/I2S2_SD, FMC_D27, DCMI_D10, EVENTOUT | - |
| - | - | D11 | D9 | 135 | F5 | - | F9 | V SS | S |  | - | - | - |
| - | - | D3 | C9 | 136 | A1 | 158 | E10 | V DD | S |  | - | - | - |

Pinouts and pin description STM32F427xx STM32F429xx
Table 10. STM32F427xx and STM32F429xx pin and ball definitions (continued)
Pin number
Pin name
(function
after reset)(1)
PA13
JTMS-SWDIO,
72 105 E12 A15 124 D3 147 A15 (JTMS- I/O FT - -
EVENTOUT
SWDIO)
73 106 D12 F13 125 D1 148 E11 V S - - -
CAP_2
74 107 J10 F12 126 D2 149 F10 V S - - -
SS
75 108 H4 G13 127 C1 150 F11 V S - - -
DD
TIM8_CH1N,
- - D13 E12 128 - 151 E12 PH13 I/O FT - CAN1_TX, FMC_D21, -
LCD_G2, EVENTOUT
TIM8_CH2N,
- - C13 E13 129 - 152 E13 PH14 I/O FT - FMC_D22, DCMI_D4, -
LCD_G3, EVENTOUT
TIM8_CH3N,
- - C12 D13 130 - 153 D13 PH15 I/O FT - FMC_D23, DCMI_D11, -
LCD_G4, EVENTOUT
TIM5_CH4,
SPI2_NSS/I2S2_WS(8),
- - B13 E14 131 - 154 E14 PI0 I/O FT - -
FMC_D24, DCMI_D13,
LCD_G5, EVENTOUT
SPI2_SCK/I2S2_CK(8),
- - C11 D14 132 - 155 D14 PI1 I/O FT - FMC_D25, DCMI_D8, -
LCD_G6, EVENTOUT
TIM8_CH4,
SPI2_MISO,
- - B12 C14 133 - 156 C14 PI2 I/O FT - I2S2ext_SD, -
FMC_D26, DCMI_D9,
LCD_G7, EVENTOUT
TIM8_ETR,
SPI2_MOSI/I2S2_SD,
- - A12 C13 134 - 157 C13 PI3 I/O FT - -
FMC_D27, DCMI_D10,
EVENTOUT
- - D11 D9 135 F5 - F9 V S - - -
SS
- - D3 C9 136 A1 158 E10 V S - - -
DD
66/240 DS9405 Rev 13
epyt
niP
erutcurts
O
/
I
setoN Additional
Alternate functions
functions
001PFQL 441PFQL 961AGBFU )2(671AGBFU 671PFQL 341PSCLW 802PFQL 612AGBFT

<!-- Page 67 -->


| Pin number |  |  |  |  |  |  |  | Pin name (function after reset)(1) | epyt niP | erutcurts O / I | setoN | Alternate functions | Additional functions |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 001PFQL | 441PFQL | 961AGBFU | )2(671AGBFU | 671PFQL | 341PSCLW | 802PFQL | 612AGBFT |  |  |  |  |  |  |
| 76 | 109 | A11 | A14 | 137 | B1 | 159 | A14 | PA14 (JTCK- SWCLK) | I/O | FT | - | JTCK-SWCLK/ EVENTOUT | - |
| 77 | 110 | B11 | A13 | 138 | C2 | 160 | A13 | PA15 (JTDI) | I/O | FT | - | JTDI, TIM2_CH1/TIM2_ETR, SPI1_NSS, SPI3_NSS/I2S3_WS, EVENTOUT | - |
| 78 | 111 | C10 | B14 | 139 | A2 | 161 | B14 | PC10 | I/O | FT | - | SPI3_SCK/I2S3_CK, USART3_TX, UART4_TX, SDIO_D2, DCMI_D8, LCD_R2, EVENTOUT | - |
| 79 | 112 | B10 | B13 | 140 | B2 | 162 | B13 | PC11 | I/O | FT | - | I2S3ext_SD, SPI3_MISO, USART3_RX, UART4_RX, SDIO_D3, DCMI_D4, EVENTOUT | - |
| 80 | 113 | A10 | A12 | 141 | C3 | 163 | A12 | PC12 | I/O | FT | - | SPI3_MOSI/I2S3_SD, USART3_CK, UART5_TX, SDIO_CK, DCMI_D9, EVENTOUT | - |
| 81 | 114 | D9 | B12 | 142 | B3 | 164 | B12 | PD0 | I/O | FT | - | CAN1_RX, FMC_D2, EVENTOUT | - |
| 82 | 115 | C9 | C12 | 143 | C4 | 165 | C12 | PD1 | I/O | FT | - | CAN1_TX, FMC_D3, EVENTOUT | - |
| 83 | 116 | B9 | D12 | 144 | A3 | 166 | D12 | PD2 | I/O | FT | - | TIM3_ETR, UART5_RX, SDIO_CMD, DCMI_D11, EVENTOUT | - |
| 84 | 117 | A9 | D11 | 145 | B4 | 167 | C11 | PD3 | I/O | FT | - | SPI2_SCK/I2S2_CK, USART2_CTS, FMC_CLK, DCMI_D5, LCD_G7, EVENTOUT | - |

STM32F427xx STM32F429xx Pinouts and pin description
Table 10. STM32F427xx and STM32F429xx pin and ball definitions (continued)
Pin number
Pin name
(function
after reset)(1)
PA14
JTCK-SWCLK/
76 109 A11 A14 137 B1 159 A14 (JTCK- I/O FT - -
EVENTOUT
SWCLK)
JTDI,
TIM2_CH1/TIM2_ETR,
PA15
77 110 B11 A13 138 C2 160 A13 I/O FT - SPI1_NSS, -
(JTDI)
SPI3_NSS/I2S3_WS,
EVENTOUT
SPI3_SCK/I2S3_CK,
USART3_TX,
78 111 C10 B14 139 A2 161 B14 PC10 I/O FT - UART4_TX, SDIO_D2, -
DCMI_D8, LCD_R2,
EVENTOUT
I2S3ext_SD,
SPI3_MISO,
79 112 B10 B13 140 B2 162 B13 PC11 I/O FT - USART3_RX, -
UART4_RX, SDIO_D3,
DCMI_D4, EVENTOUT
SPI3_MOSI/I2S3_SD,
USART3_CK,
80 113 A10 A12 141 C3 163 A12 PC12 I/O FT - -
UART5_TX, SDIO_CK,
DCMI_D9, EVENTOUT
CAN1_RX, FMC_D2,
81 114 D9 B12 142 B3 164 B12 PD0 I/O FT - -
EVENTOUT
CAN1_TX, FMC_D3,
82 115 C9 C12 143 C4 165 C12 PD1 I/O FT - -
EVENTOUT
TIM3_ETR,
UART5_RX,
83 116 B9 D12 144 A3 166 D12 PD2 I/O FT - SDIO_CMD, -
DCMI_D11,
EVENTOUT
SPI2_SCK/I2S2_CK,
USART2_CTS,
84 117 A9 D11 145 B4 167 C11 PD3 I/O FT - -
FMC_CLK, DCMI_D5,
LCD_G7, EVENTOUT
DS9405 Rev 13 67/240
85
epyt
niP
erutcurts
O
/
I
setoN Additional
Alternate functions
functions
001PFQL 441PFQL 961AGBFU )2(671AGBFU 671PFQL 341PSCLW 802PFQL 612AGBFT

<!-- Page 68 -->


| Pin number |  |  |  |  |  |  |  | Pin name (function after reset)(1) | epyt niP | erutcurts O / I | setoN | Alternate functions | Additional functions |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 001PFQL | 441PFQL | 961AGBFU | )2(671AGBFU | 671PFQL | 341PSCLW | 802PFQL | 612AGBFT |  |  |  |  |  |  |
| 85 | 118 | D8 | D10 | 146 | B5 | 168 | D11 | PD4 | I/O | FT | - | USART2_RTS, FMC_NOE, EVENTOUT | - |
| 86 | 119 | C8 | C11 | 147 | A4 | 169 | C10 | PD5 | I/O | FT | - | USART2_TX, FMC_NWE, EVENTOUT | - |
| - | 120 | - | D8 | 148 | - | 170 | F8 | V SS | S |  | - | - | - |
| - | 121 | D6 | C8 | 149 | C5 | 171 | E9 | V DD | S |  | - | - | - |
| 87 | 122 | B8 | B11 | 150 | F4 | 172 | B11 | PD6 | I/O | FT | - | SPI3_MOSI/I2S3_SD, SAI1_SD_A, USART2_RX, FMC_NWAIT, DCMI_D10, LCD_B2, EVENTOUT | - |
| 88 | 123 | A8 | A11 | 151 | A5 | 173 | A11 | PD7 | I/O | FT | - | USART2_CK, FMC_NE1/FMC_NCE2 , EVENTOUT | - |
| - | - | - | - | - | - | 174 | B10 | PJ12 | I/O | FT | - | LCD_B0, EVENTOUT | - |
| - | - | - | - | - | - | 175 | B9 | PJ13 | I/O | FT | - | LCD_B1, EVENTOUT | - |
| - | - | - | - | - | - | 176 | C9 | PJ14 | I/O | FT | - | LCD_B2, EVENTOUT | - |
| - | - | - | - | - | - | 177 | D10 | PJ15 | I/O | FT | - | LCD_B3, EVENTOUT | - |
| - | 124 | NC (3) | C10 | 152 | E5 | 178 | D9 | PG9 | I/O | FT | - | USART6_RX, FMC_NE2/FMC_NCE3 , DCMI_VSYNC(9), EVENTOUT | - |
| - | 125 | C7 | B10 | 153 | C6 | 179 | C8 | PG10 | I/O | FT | - | LCD_G3, FMC_NCE4_1/FMC_N E3, DCMI_D2, LCD_B2, EVENTOUT | - |
| - | 126 | B7 | B9 | 154 | B6 | 180 | B8 | PG11 | I/O | FT | - | ETH_MII_TX_EN/ETH_ RMII_TX_EN, FMC_NCE4_2, DCMI_D3, LCD_B3, EVENTOUT | - |

Pinouts and pin description STM32F427xx STM32F429xx
Table 10. STM32F427xx and STM32F429xx pin and ball definitions (continued)
Pin number
Pin name
(function
after reset)(1)
USART2_RTS,
85 118 D8 D10 146 B5 168 D11 PD4 I/O FT - FMC_NOE, -
EVENTOUT
USART2_TX,
86 119 C8 C11 147 A4 169 C10 PD5 I/O FT - FMC_NWE, -
EVENTOUT
- 120 - D8 148 - 170 F8 V S - - -
SS
- 121 D6 C8 149 C5 171 E9 V S - - -
DD
SPI3_MOSI/I2S3_SD,
SAI1_SD_A,
USART2_RX,
87 122 B8 B11 150 F4 172 B11 PD6 I/O FT - -
FMC_NWAIT,
DCMI_D10, LCD_B2,
EVENTOUT
USART2_CK,
88 123 A8 A11 151 A5 173 A11 PD7 I/O FT - FMC_NE1/FMC_NCE2 -
, EVENTOUT
- - - - - - 174 B10 PJ12 I/O FT - LCD_B0, EVENTOUT -
- - - - - - 175 B9 PJ13 I/O FT - LCD_B1, EVENTOUT -
- - - - - - 176 C9 PJ14 I/O FT - LCD_B2, EVENTOUT -
- - - - - - 177 D10 PJ15 I/O FT - LCD_B3, EVENTOUT -
USART6_RX,
NC FMC_NE2/FMC_NCE3
- 124 C10 152 E5 178 D9 PG9 I/O FT - -
(3) , DCMI_VSYNC(9),
EVENTOUT
LCD_G3,
FMC_NCE4_1/FMC_N
- 125 C7 B10 153 C6 179 C8 PG10 I/O FT - -
E3, DCMI_D2,
LCD_B2, EVENTOUT
ETH_MII_TX_EN/ETH_
RMII_TX_EN,
- 126 B7 B9 154 B6 180 B8 PG11 I/O FT - FMC_NCE4_2, -
DCMI_D3, LCD_B3,
EVENTOUT
68/240 DS9405 Rev 13
epyt
niP
erutcurts
O
/
I
setoN Additional
Alternate functions
functions
001PFQL 441PFQL 961AGBFU )2(671AGBFU 671PFQL 341PSCLW 802PFQL 612AGBFT

<!-- Page 69 -->


| Pin number |  |  |  |  |  |  |  | Pin name (function after reset)(1) | epyt niP | erutcurts O / I | setoN | Alternate functions | Additional functions |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 001PFQL | 441PFQL | 961AGBFU | )2(671AGBFU | 671PFQL | 341PSCLW | 802PFQL | 612AGBFT |  |  |  |  |  |  |
| - | 127 | A7 | B8 | 155 | A6 | 181 | C7 | PG12 | I/O | FT | - | SPI6_MISO, USART6_RTS, LCD_B4, FMC_NE4, LCD_B1, EVENTOUT | - |
| - | 128 | NC (3) | A8 | 156 | D6 | 182 | B3 | PG13 | I/O | FT | - | SPI6_SCK, USART6_CTS, ETH_MII_TXD0/ETH_ RMII_TXD0, FMC_A24, EVENTOUT | - |
| - | 129 | NC (3) | A7 | 157 | F6 | 183 | A4 | PG14 | I/O | FT | - | SPI6_MOSI, USART6_TX, ETH_MII_TXD1/ETH_ RMII_TXD1, FMC_A25, EVENTOUT | - |
| - | 130 | D7 | D7 | 158 | - | 184 | F7 | V SS | S |  | - | - | - |
| - | 131 | L6 | C7 | 159 | E6 | 185 | E8 | V DD | S |  | - | - | - |
| - | - | - | - | - | - | 186 | D8 | PK3 | I/O | FT | - | LCD_B4, EVENTOUT | - |
| - | - | - | - | - | - | 187 | D7 | PK4 | I/O | FT | - | LCD_B5, EVENTOUT | - |
| - | - | - | - | - | - | 188 | C6 | PK5 | I/O | FT | - | LCD_B6, EVENTOUT | - |
| - | - | - | - | - | - | 189 | C5 | PK6 | I/O | FT | - | LCD_B7, EVENTOUT | - |
| - | - | - | - | - | - | 190 | C4 | PK7 | I/O | FT | - | LCD_DE, EVENTOUT | - |
| - | 132 | C6 | B7 | 160 | A7 | 191 | B7 | PG15 | I/O | FT | - | USART6_CTS, FMC_SDNCAS, DCMI_D13, EVENTOUT | - |
| 89 | 133 | B6 | A10 | 161 | B7 | 192 | A10 | PB3 (JTDO/TRACE SWO) | I/O | FT | - | JTDO/TRACESWO, TIM2_CH2, SPI1_SCK, SPI3_SCK/I2S3_CK, EVENTOUT | - |
| 90 | 134 | A6 | A9 | 162 | C7 | 193 | A9 | PB4 (NJTRST) | I/O | FT | - | NJTRST, TIM3_CH1, SPI1_MISO, SPI3_MISO, I2S3ext_SD, EVENTOUT | - |

STM32F427xx STM32F429xx Pinouts and pin description
Table 10. STM32F427xx and STM32F429xx pin and ball definitions (continued)
Pin number
Pin name
(function
after reset)(1)
SPI6_MISO,
USART6_RTS,
- 127 A7 B8 155 A6 181 C7 PG12 I/O FT - -
LCD_B4, FMC_NE4,
LCD_B1, EVENTOUT
SPI6_SCK,
USART6_CTS,
NC
- 128 A8 156 D6 182 B3 PG13 I/O FT - ETH_MII_TXD0/ETH_ -
(3)
RMII_TXD0, FMC_A24,
EVENTOUT
SPI6_MOSI,
USART6_TX,
NC
- 129 A7 157 F6 183 A4 PG14 I/O FT - ETH_MII_TXD1/ETH_ -
(3)
RMII_TXD1, FMC_A25,
EVENTOUT
- 130 D7 D7 158 - 184 F7 V S - - -
SS
- 131 L6 C7 159 E6 185 E8 V S - - -
DD
- - - - - - 186 D8 PK3 I/O FT - LCD_B4, EVENTOUT -
- - - - - - 187 D7 PK4 I/O FT - LCD_B5, EVENTOUT -
- - - - - - 188 C6 PK5 I/O FT - LCD_B6, EVENTOUT -
- - - - - - 189 C5 PK6 I/O FT - LCD_B7, EVENTOUT -
- - - - - - 190 C4 PK7 I/O FT - LCD_DE, EVENTOUT -
USART6_CTS,
FMC_SDNCAS,
- 132 C6 B7 160 A7 191 B7 PG15 I/O FT - -
DCMI_D13,
EVENTOUT
JTDO/TRACESWO,
PB3
TIM2_CH2, SPI1_SCK,
89 133 B6 A10 161 B7 192 A10 (JTDO/TRACE I/O FT - -
SPI3_SCK/I2S3_CK,
SWO)
EVENTOUT
NJTRST, TIM3_CH1,
SPI1_MISO,
PB4
90 134 A6 A9 162 C7 193 A9 I/O FT - SPI3_MISO, -
(NJTRST)
I2S3ext_SD,
EVENTOUT
DS9405 Rev 13 69/240
85
epyt
niP
erutcurts
O
/
I
setoN Additional
Alternate functions
functions
001PFQL 441PFQL 961AGBFU )2(671AGBFU 671PFQL 341PSCLW 802PFQL 612AGBFT

<!-- Page 70 -->


| Pin number |  |  |  |  |  |  |  | Pin name (function after reset)(1) | epyt niP | erutcurts O / I | setoN | Alternate functions | Additional functions |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 001PFQL | 441PFQL | 961AGBFU | )2(671AGBFU | 671PFQL | 341PSCLW | 802PFQL | 612AGBFT |  |  |  |  |  |  |
| 91 | 135 | D5 | A6 | 163 | C8 | 194 | A8 | PB5 | I/O | FT | - | TIM3_CH2, I2C1_SMBA, SPI1_MOSI, SPI3_MOSI/I2S3_SD, CAN2_RX, OTG_HS_ULPI_D7, ETH_PPS_OUT, FMC_SDCKE1, DCMI_D10, EVENTOUT | - |
| 92 | 136 | C5 | B6 | 164 | A8 | 195 | B6 | PB6 | I/O | FT | - | TIM4_CH1, I2C1_SCL, USART1_TX, CAN2_TX, FMC_SDNE1, DCMI_D5, EVENTOUT | - |
| 93 | 137 | B5 | B5 | 165 | B8 | 196 | B5 | PB7 | I/O | FT | - | TIM4_CH2, I2C1_SDA, USART1_RX, FMC_NL, DCMI_VSYNC, EVENTOUT | - |
| 94 | 138 | A5 | D6 | 166 | C9 | 197 | E6 | BOOT0 | I | B | - | - | V PP |
| 95 | 139 | D4 | A5 | 167 | A9 | 198 | A7 | PB8 | I/O | FT | - | TIM4_CH3, TIM10_CH1, I2C1_SCL, CAN1_RX, ETH_MII_TXD3, SDIO_D4, DCMI_D6, LCD_B6, EVENTOUT | - |
| 96 | 140 | C4 | B4 | 168 | B9 | 199 | B4 | PB9 | I/O | FT | - | TIM4_CH4, TIM11_CH1, I2C1_SDA, SPI2_NSS/I2S2_WS, CAN1_TX, SDIO_D5, DCMI_D7, LCD_B7, EVENTOUT | - |
| 97 | 141 | B4 | A4 | 169 | B10 | 200 | A6 | PE0 | I/O | FT | - | TIM4_ETR, UART8_RX, FMC_NBL0, DCMI_D2, EVENTOUT | - |

Pinouts and pin description STM32F427xx STM32F429xx
Table 10. STM32F427xx and STM32F429xx pin and ball definitions (continued)
Pin number
Pin name
(function
after reset)(1)
TIM3_CH2,
I2C1_SMBA,
SPI1_MOSI,
SPI3_MOSI/I2S3_SD,
CAN2_RX,
91 135 D5 A6 163 C8 194 A8 PB5 I/O FT - -
OTG_HS_ULPI_D7,
ETH_PPS_OUT,
FMC_SDCKE1,
DCMI_D10,
EVENTOUT
TIM4_CH1, I2C1_SCL,
USART1_TX,
92 136 C5 B6 164 A8 195 B6 PB6 I/O FT - CAN2_TX, -
FMC_SDNE1,
DCMI_D5, EVENTOUT
TIM4_CH2, I2C1_SDA,
USART1_RX,
93 137 B5 B5 165 B8 196 B5 PB7 I/O FT - FMC_NL, -
DCMI_VSYNC,
EVENTOUT
94 138 A5 D6 166 C9 197 E6 BOOT0 I B - - V
PP
TIM4_CH3,
TIM10_CH1,
I2C1_SCL, CAN1_RX,
95 139 D4 A5 167 A9 198 A7 PB8 I/O FT - -
ETH_MII_TXD3,
SDIO_D4, DCMI_D6,
LCD_B6, EVENTOUT
TIM4_CH4,
TIM11_CH1,
I2C1_SDA,
96 140 C4 B4 168 B9 199 B4 PB9 I/O FT - SPI2_NSS/I2S2_WS, -
CAN1_TX, SDIO_D5,
DCMI_D7, LCD_B7,
EVENTOUT
TIM4_ETR,
UART8_RX,
97 141 B4 A4 169 B10 200 A6 PE0 I/O FT - -
FMC_NBL0, DCMI_D2,
EVENTOUT
70/240 DS9405 Rev 13
epyt
niP
erutcurts
O
/
I
setoN Additional
Alternate functions
functions
001PFQL 441PFQL 961AGBFU )2(671AGBFU 671PFQL 341PSCLW 802PFQL 612AGBFT

<!-- Page 71 -->


| Pin number |  |  |  |  |  |  |  | Pin name (function after reset)(1) | epyt niP | erutcurts O / I | setoN | Alternate functions | Additional functions |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 001PFQL | 441PFQL | 961AGBFU | )2(671AGBFU | 671PFQL | 341PSCLW | 802PFQL | 612AGBFT |  |  |  |  |  |  |
| 98 | 142 | A4 | A3 | 170 | A10 | 201 | A5 | PE1 | I/O | FT | - | UART8_Tx, FMC_NBL1, DCMI_D3, EVENTOUT | - |
| 99 | - | F5 | D5 | - | - | 202 | F6 | V SS | S |  | - | - | - |
| - | 143 | C3 | C6 | 171 | A11 | 203 | E5 | PDR_ON | S |  | - | - | - |
| 100 | 144 | K6 | C5 | 172 | D7 | 204 | E7 | V DD | S |  | - | - | - |
| - | - | B3 | D4 | 173 | - | 205 | C3 | PI4 | I/O | FT | - | TIM8_BKIN, FMC_NBL2, DCMI_D5, LCD_B4, EVENTOUT | - |
| - | - | A3 | C4 | 174 | - | 206 | D3 | PI5 | I/O | FT | - | TIM8_CH1, FMC_NBL3, DCMI_VSYNC, LCD_B5, EVENTOUT | - |
| - | - | A2 | C3 | 175 | - | 207 | D6 | PI6 | I/O | FT | - | TIM8_CH2, FMC_D28, DCMI_D6, LCD_B6, EVENTOUT | - |
| - | - | B1 | C2 | 176 | - | 208 | D4 | PI7 | I/O | FT | - | TIM8_CH3, FMC_D29, DCMI_D7, LCD_B7, EVENTOUT | - |

STM32F427xx STM32F429xx Pinouts and pin description
Table 10. STM32F427xx and STM32F429xx pin and ball definitions (continued)
Pin number
Pin name
(function
after reset)(1)
UART8_Tx,
98 142 A4 A3 170 A10 201 A5 PE1 I/O FT - FMC_NBL1, DCMI_D3, -
EVENTOUT
99 - F5 D5 - - 202 F6 V S - - -
SS
- 143 C3 C6 171 A11 203 E5 PDR_ON S - - -
100 144 K6 C5 172 D7 204 E7 V S - - -
DD
TIM8_BKIN,
- - B3 D4 173 - 205 C3 PI4 I/O FT - FMC_NBL2, DCMI_D5, -
LCD_B4, EVENTOUT
TIM8_CH1,
FMC_NBL3,
- - A3 C4 174 - 206 D3 PI5 I/O FT - -
DCMI_VSYNC,
LCD_B5, EVENTOUT
TIM8_CH2, FMC_D28,
- - A2 C3 175 - 207 D6 PI6 I/O FT - DCMI_D6, LCD_B6, -
EVENTOUT
TIM8_CH3, FMC_D29,
- - B1 C2 176 - 208 D4 PI7 I/O FT - DCMI_D7, LCD_B7, -
EVENTOUT
1. Function availability depends on the chosen device.
2. On the UFBGA176 package, the balls F6, F7, F8, F9, F10, G6, G7, G8, G9, G10, H6, H7, H8, H9, H10, J6, J7, J8, J9, J10,
K6, K7, K8, K9, and K10 are connected to VSS. Their purpose is heat dissipation and package mechanical stability
3. NC (not-connected) pins are not bonded. They must be configured by software to output push-pull and forced to 0 in the
output data register to avoid extra current consumption in low-power modes.
4. PC13, PC14, PC15, and PI8 are supplied through the power switch. Since the switch only sinks a limited amount of current
(3mA), the use of GPIOs PC13 to PC15 and PI8 in output mode is limited:
- The speed should not exceed 2 MHz with a maximum load of 30 pF.
- These I/Os must not be used as a current source (for example, to drive an LED).
5. The main function after the first backup domain power-up. Later on, it depends on the contents of the RTC registers even
after reset (because these registers are not reset by the main reset). For details on how to manage these I/Os, refer to the
RTC register description sections in the STM32F4xx reference manual, available from the STMicroelectronics website:
www.st.com.
6. FT = 5 V tolerant except when in analog mode or oscillator mode (for PC14, PC15, PH0, and PH1).
7. If the device is delivered in a WLCSP143, UFBGA169, UFBGA176, LQFP176 or TFBGA216 package, and the
BYPASS_REG pin is set to V (Regulator OFF/internal reset ON mode), then PA0 is used as an internal Reset (active
DD
low).
8. PI0 and PI1 cannot be used for I2S2 full-duplex mode.
9. The DCMI_VSYNC alternate function on PG9 is only available on silicon revision 3.
DS9405 Rev 13 71/240
85
epyt
niP
erutcurts
O
/
I
setoN Additional
Alternate functions
functions
001PFQL 441PFQL 961AGBFU )2(671AGBFU 671PFQL 341PSCLW 802PFQL 612AGBFT

<!-- Page 72 -->


| Pin name | CF | NOR/PSRAM/ SRAM | NOR/PSRAM Mux | NAND16 | SDRAM |
| --- | --- | --- | --- | --- | --- |
| PF0 | A0 | A0 | - | - | A0 |
| PF1 | A1 | A1 | - | - | A1 |
| PF2 | A2 | A2 | - | - | A2 |
| PF3 | A3 | A3 | - | - | A3 |
| PF4 | A4 | A4 | - | - | A4 |
| PF5 | A5 | A5 | - | - | A5 |
| PF12 | A6 | A6 | - | - | A6 |
| PF13 | A7 | A7 | - | - | A7 |
| PF14 | A8 | A8 | - | - | A8 |
| PF15 | A9 | A9 | - | - | A9 |
| PG0 | A10 | A10 | - | - | A10 |
| PG1 | - | A11 | - | - | A11 |
| PG2 | - | A12 | - | - | A12 |
| PG3 | - | A13 | - | - | - |
| PG4 | - | A14 | - | - | BA0 |
| PG5 | - | A15 | - | - | BA1 |
| PD11 | - | A16 | A16 | CLE | - |
| PD12 | - | A17 | A17 | ALE | - |
| PD13 | - | A18 | A18 | - | - |
| PE3 | - | A19 | A19 | - | - |
| PE4 | - | A20 | A20 | - | - |
| PE5 | - | A21 | A21 | - | - |
| PE6 | - | A22 | A22 | - | - |
| PE2 | - | A23 | A23 | - | - |
| PG13 | - | A24 | A24 | - | - |
| PG14 | - | A25 | A25 | - | - |
| PD14 | D0 | D0 | DA0 | D0 | D0 |
| PD15 | D1 | D1 | DA1 | D1 | D1 |
| PD0 | D2 | D2 | DA2 | D2 | D2 |
| PD1 | D3 | D3 | DA3 | D3 | D3 |
| PE7 | D4 | D4 | DA4 | D4 | D4 |
| PE8 | D5 | D5 | DA5 | D5 | D5 |
| PE9 | D6 | D6 | DA6 | D6 | D6 |
| PE10 | D7 | D7 | DA7 | D7 | D7 |

Pinouts and pin description STM32F427xx STM32F429xx
Table 11. FMC pin definition
NOR/PSRAM/ NOR/PSRAM
Pin name CF NAND16 SDRAM
SRAM Mux
PF0 A0 A0 - - A0
PF1 A1 A1 - - A1
PF2 A2 A2 - - A2
PF3 A3 A3 - - A3
PF4 A4 A4 - - A4
PF5 A5 A5 - - A5
PF12 A6 A6 - - A6
PF13 A7 A7 - - A7
PF14 A8 A8 - - A8
PF15 A9 A9 - - A9
PG0 A10 A10 - - A10
PG1 - A11 - - A11
PG2 - A12 - - A12
PG3 - A13 - - -
PG4 - A14 - - BA0
PG5 - A15 - - BA1
PD11 - A16 A16 CLE -
PD12 - A17 A17 ALE -
PD13 - A18 A18 - -
PE3 - A19 A19 - -
PE4 - A20 A20 - -
PE5 - A21 A21 - -
PE6 - A22 A22 - -
PE2 - A23 A23 - -
PG13 - A24 A24 - -
PG14 - A25 A25 - -
PD14 D0 D0 DA0 D0 D0
PD15 D1 D1 DA1 D1 D1
PD0 D2 D2 DA2 D2 D2
PD1 D3 D3 DA3 D3 D3
PE7 D4 D4 DA4 D4 D4
PE8 D5 D5 DA5 D5 D5
PE9 D6 D6 DA6 D6 D6
PE10 D7 D7 DA7 D7 D7
72/240 DS9405 Rev 13

<!-- Page 73 -->


| Pin name | CF | NOR/PSRAM/ SRAM | NOR/PSRAM Mux | NAND16 | SDRAM |
| --- | --- | --- | --- | --- | --- |
| PE11 | D8 | D8 | DA8 | D8 | D8 |
| PE12 | D9 | D9 | DA9 | D9 | D9 |
| PE13 | D10 | D10 | DA10 | D10 | D10 |
| PE14 | D11 | D11 | DA11 | D11 | D11 |
| PE15 | D12 | D12 | DA12 | D12 | D12 |
| PD8 | D13 | D13 | DA13 | D13 | D13 |
| PD9 | D14 | D14 | DA14 | D14 | D14 |
| PD10 | D15 | D15 | DA15 | D15 | D15 |
| PH8 | - | D16 | - | - | D16 |
| PH9 | - | D17 | - | - | D17 |
| PH10 | - | D18 | - | - | D18 |
| PH11 | - | D19 | - | - | D19 |
| PH12 | - | D20 | - | - | D20 |
| PH13 | - | D21 | - | - | D21 |
| PH14 | - | D22 | - | - | D22 |
| PH15 | - | D23 | - | - | D23 |
| PI0 | - | D24 | - | - | D24 |
| PI1 | - | D25 | - | - | D25 |
| PI2 | - | D26 | - | - | D26 |
| PI3 | - | D27 | - | - | D27 |
| PI6 | - | D28 | - | - | D28 |
| PI7 | - | D29 | - | - | D29 |
| PI9 | - | D30 | - | - | D30 |
| PI10 | - | D31 | - | - | D31 |
| PD7 | - | NE1 | NE1 | NCE2 | - |
| PG9 | - | NE2 | NE2 | NCE3 | - |
| PG10 | NCE4_1 | NE3 | NE3 | - | - |
| PG11 | NCE4_2 | - | - | - | - |
| PG12 | - | NE4 | NE4 | - | - |
| PD3 | - | CLK | CLK | - | - |
| PD4 | NOE | NOE | NOE | NOE | - |
| PD5 | NWE | NWE | NWE | NWE | - |
| PD6 | NWAIT | NWAIT | NWAIT | NWAIT | - |
| PB7 | - | NL(NADV) | NL(NADV) | - | - |

STM32F427xx STM32F429xx Pinouts and pin description
Table 11. FMC pin definition (continued)
NOR/PSRAM/ NOR/PSRAM
Pin name CF NAND16 SDRAM
SRAM Mux
PE11 D8 D8 DA8 D8 D8
PE12 D9 D9 DA9 D9 D9
PE13 D10 D10 DA10 D10 D10
PE14 D11 D11 DA11 D11 D11
PE15 D12 D12 DA12 D12 D12
PD8 D13 D13 DA13 D13 D13
PD9 D14 D14 DA14 D14 D14
PD10 D15 D15 DA15 D15 D15
PH8 - D16 - - D16
PH9 - D17 - - D17
PH10 - D18 - - D18
PH11 - D19 - - D19
PH12 - D20 - - D20
PH13 - D21 - - D21
PH14 - D22 - - D22
PH15 - D23 - - D23
PI0 - D24 - - D24
PI1 - D25 - - D25
PI2 - D26 - - D26
PI3 - D27 - - D27
PI6 - D28 - - D28
PI7 - D29 - - D29
PI9 - D30 - - D30
PI10 - D31 - - D31
PD7 - NE1 NE1 NCE2 -
PG9 - NE2 NE2 NCE3 -
PG10 NCE4_1 NE3 NE3 - -
PG11 NCE4_2 - - - -
PG12 - NE4 NE4 - -
PD3 - CLK CLK - -
PD4 NOE NOE NOE NOE -
PD5 NWE NWE NWE NWE -
PD6 NWAIT NWAIT NWAIT NWAIT -
PB7 - NL(NADV) NL(NADV) - -
DS9405 Rev 13 73/240
85

<!-- Page 74 -->


| Pin name | CF | NOR/PSRAM/ SRAM | NOR/PSRAM Mux | NAND16 | SDRAM |
| --- | --- | --- | --- | --- | --- |
| PF6 | NIORD | - | - | - | - |
| PF7 | NREG | - | - | - | - |
| PF8 | NIOWR | - | - | - | - |
| PF9 | CD | - | - | - | - |
| PF10 | INTR | - | - | - | - |
| PG6 | - | - | - | INT2 | - |
| PG7 | - | - | - | INT3 | - |
| PE0 | - | NBL0 | NBL0 | - | NBL0 |
| PE1 | - | NBL1 | NBL1 | - | NBL1 |
| PI4 | - | NBL2 | - | - | NBL2 |
| PI5 | - | NBL3 | - | - | NBL3 |
| PG8 | - | - | - | - | SDCLK |
| PC0 | - | - | - | - | SDNWE |
| PF11 | - | - | - | - | SDNRAS |
| PG15 | - | - | - | - | SDNCAS |
| PH2 | - | - | - | - | SDCKE0 |
| PH3 | - | - | - | - | SDNE0 |
| PH6 | - | - | - | - | SDNE1 |
| PH7 | - | - | - | - | SDCKE1 |
| PH5 | - | - | - | - | SDNWE |
| PC2 | - | - | - | - | SDNE0 |
| PC3 | - | - | - | - | SDCKE0 |
| PB5 | - | - | - | - | SDCKE1 |
| PB6 | - | - | - | - | SDNE1 |

Pinouts and pin description STM32F427xx STM32F429xx
Table 11. FMC pin definition (continued)
NOR/PSRAM/ NOR/PSRAM
Pin name CF NAND16 SDRAM
SRAM Mux
PF6 NIORD - - - -
PF7 NREG - - - -
PF8 NIOWR - - - -
PF9 CD - - - -
PF10 INTR - - - -
PG6 - - - INT2 -
PG7 - - - INT3 -
PE0 - NBL0 NBL0 - NBL0
PE1 - NBL1 NBL1 - NBL1
PI4 - NBL2 - - NBL2
PI5 - NBL3 - - NBL3
PG8 - - - - SDCLK
PC0 - - - - SDNWE
PF11 - - - - SDNRAS
PG15 - - - - SDNCAS
PH2 - - - - SDCKE0
PH3 - - - - SDNE0
PH6 - - - - SDNE1
PH7 - - - - SDCKE1
PH5 - - - - SDNWE
PC2 - - - - SDNE0
PC3 - - - - SDCKE0
PB5 - - - - SDCKE1
PB6 - - - - SDNE1
74/240 DS9405 Rev 13

<!-- Page 75 -->


| Port |  | AF0 | AF1 | AF2 | AF3 | AF4 | AF5 | AF6 | AF7 | AF8 | AF9 | AF10 | AF11 | AF12 | AF13 | AF14 | AF15 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
|  |  | SYS | TIM1/2 | TIM3/4/5 | TIM8/9/ 10/11 | I2C1/ 2/3 | SPI1/2/ 3/4/5/6 | SPI2/3/ SAI1 | SPI3/ USART1/ 2/3 | USART6/ UART4/5/7 /8 | CAN1/2/ TIM12/13/14 /LCD | OTG2_HS /OTG1_ FS | ETH | FMC/SDIO /OTG2_FS | DCMI | LCD | SYS |
| Port A | PA0 | - | TIM2_ CH1/TIM2 _ETR | TIM5_ CH1 | TIM8_ ETR | - | - | - | USART2_ CTS | UART4_TX | - | - | ETH_MII_ CRS | - | - | - | EVEN TOUT |
|  | PA1 | - | TIM2_ CH2 | TIM5_ CH2 | - | - | - | - | USART2_ RTS | UART4_RX | - | - | ETH_MII_ RX_CLK/E TH_RMII_ REF_CLK | - | - | - | EVEN TOUT |
|  | PA2 | - | TIM2_ CH3 | TIM5_ CH3 | TIM9_ CH1 | - | - | - | USART2_ TX | - | - | - | ETH_ MDIO | - | - | - | EVEN TOUT |
|  | PA3 | - | TIM2_ CH4 | TIM5_ CH4 | TIM9_ CH2 | - | - | - | USART2_ RX | - | - | OTG_HS_ ULPI_D0 | ETH_MII_ COL | - | - | LCD_B5 | EVEN TOUT |
|  | PA4 | - | - | - | - | - | SPI1_ NSS | SPI3_ NSS/ I2S3_WS | USART2_ CK | - | - | - | - | OTG_HS_ SOF | DCMI_ HSYNC | LCD_ VSYNC | EVEN TOUT |
|  | PA5 | - | TIM2_ CH1/TIM2 _ETR | - | TIM8_ CH1N | - | SPI1_ SCK | - | - | - | - | OTG_HS_ ULPI_CK | - | - | - | - | EVEN TOUT |
|  | PA6 | - | TIM1_ BKIN | TIM3_ CH1 | TIM8_ BKIN | - | SPI1_ MISO | - | - | - | TIM13_CH1 | - | - | - | DCMI_ PIXCLK | LCD_G2 | EVEN TOUT |
|  | PA7 | - | TIM1_ CH1N | TIM3_ CH2 | TIM8_ CH1N | - | SPI1_ MOSI | - | - | - | TIM14_CH1 | - | ETH_MII_ RX_DV/ ETH_RMII _CRS_DV | - | - | - | EVEN TOUT |
|  | PA8 | MCO1 | TIM1_ CH1 | - | - | I2C3_ SCL | - | - | USART1_ CK | - | - | OTG_FS_ SOF | - | - | - | LCD_R6 | EVEN TOUT |
|  | PA9 | - | TIM1_ CH2 | - | - | I2C3_ SMBA | - | - | USART1_ TX | - | - | - | - | - | DCMI_ D0 | - | EVEN TOUT |
|  | PA10 | - | TIM1_ CH3 | - | - | - | - | - | USART1_ RX | - | - | OTG_FS_ ID | - | - | DCMI_ D1 | - | EVEN TOUT |
|  | PA11 | - | TIM1_ CH4 | - | - | - | - | - | USART1_ CTS | - | CAN1_RX | OTG_FS_ DM | - | - | - | LCD_R4 | EVEN TOUT |
|  | PA12 | - | TIM1_ ETR | - | - | - | - | - | USART1_ RTS | - | CAN1_TX | OTG_FS_ DP | - | - | - | LCD_R5 | EVEN TOUT |

DS9405
Rev
13
75/240
STM32F427xx
STM32F429xx
Pinouts
and
pin
description
Table 12. STM32F427xx and STM32F429xx alternate function mapping
AF0 AF1 AF2 AF3 AF4 AF5 AF6 AF7 AF8 AF9 AF10 AF11 AF12 AF13 AF14 AF15
Port SPI3/ USART6/ CAN1/2/ OTG2_HS
TIM8/9/ I2C1/ SPI1/2/ SPI2/3/ FMC/SDIO
SYS TIM1/2 TIM3/4/5 USART1/ UART4/5/7 TIM12/13/14 /OTG1_ ETH DCMI LCD SYS
10/11 2/3 3/4/5/6 SAI1 /OTG2_FS
2/3 /8 /LCD FS
TIM2_
TIM5_ TIM8_ USART2_ ETH_MII_ EVEN
PA0 - CH1/TIM2 - - - UART4_TX - - - - -
CH1 ETR CTS CRS TOUT
_ETR
ETH_MII_
TIM2_ TIM5_ USART2_ RX_CLK/E EVEN
PA1 - - - - - UART4_RX - - - - -
CH2 CH2 RTS TH_RMII_ TOUT
REF_CLK
TIM2_ TIM5_ TIM9_ USART2_ ETH_ EVEN
PA2 - - - - - - - - - -
CH3 CH3 CH1 TX MDIO TOUT
TIM2_ TIM5_ TIM9_ USART2_ OTG_HS_ ETH_MII_ EVEN
PA3 - - - - - - - - LCD_B5
CH4 CH4 CH2 RX ULPI_D0 COL TOUT
SPI3_
SPI1_ USART2_ OTG_HS_ DCMI_ LCD_ EVEN
PA4 - - - - - NSS/ - - - -
NSS CK SOF HSYNC VSYNC TOUT
I2S3_WS
TIM2_
TIM8_ SPI1_ OTG_HS_ EVEN
PA5 - CH1/TIM2 - - - - - - - - - -
CH1N SCK ULPI_CK TOUT
_ETR
Port A
TIM1_ TIM3_ TIM8_ SPI1_ DCMI_ EVEN
PA6 - - - - - TIM13_CH1 - - - LCD_G2
BKIN CH1 BKIN MISO PIXCLK TOUT
ETH_MII_
TIM1_ TIM3_ TIM8_ SPI1_ RX_DV/ EVEN
PA7 - - - - - TIM14_CH1 - - - -
CH1N CH2 CH1N MOSI ETH_RMII TOUT
_CRS_DV
TIM1_ I2C3_ USART1_ OTG_FS_ EVEN
PA8 MCO1 - - - - - - - - - LCD_R6
CH1 SCL CK SOF TOUT
TIM1_ I2C3_ USART1_ DCMI_ EVEN
PA9 - - - - - - - - - - -
CH2 SMBA TX D0 TOUT
TIM1_ USART1_ OTG_FS_ DCMI_ EVEN
PA10 - - - - - - - - - - -
CH3 RX ID D1 TOUT
TIM1_ USART1_ OTG_FS_ EVEN
PA11 - - - - - - - CAN1_RX - - - LCD_R4
CH4 CTS DM TOUT
TIM1_ USART1_ OTG_FS_ EVEN
PA12 - - - - - - - CAN1_TX - - - LCD_R5
ETR RTS DP TOUT

<!-- Page 76 -->


| Port |  | AF0 | AF1 | AF2 | AF3 | AF4 | AF5 | AF6 | AF7 | AF8 | AF9 | AF10 | AF11 | AF12 | AF13 | AF14 | AF15 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
|  |  | SYS | TIM1/2 | TIM3/4/5 | TIM8/9/ 10/11 | I2C1/ 2/3 | SPI1/2/ 3/4/5/6 | SPI2/3/ SAI1 | SPI3/ USART1/ 2/3 | USART6/ UART4/5/7 /8 | CAN1/2/ TIM12/13/14 /LCD | OTG2_HS /OTG1_ FS | ETH | FMC/SDIO /OTG2_FS | DCMI | LCD | SYS |
| Port A | PA13 | JTMS- SWDI O | - | - | - | - | - | - | - | - | - | - | - | - | - | - | EVEN TOUT |
|  | PA14 | JTCK- SWCL K | - | - | - | - | - | - | - | - | - | - | - | - | - | - | EVEN TOUT |
|  | PA15 | JTDI | TIM2_ CH1/TIM2 _ETR | - | - | - | SPI1_ NSS | SPI3_ NSS/ I2S3_WS | - | - | - | - | - | - | - | - | EVEN TOUT |
| Port B | PB0 | - | TIM1_ CH2N | TIM3_ CH3 | TIM8_ CH2N | - | - | - | - | - | LCD_R3 | OTG_HS_ ULPI_D1 | ETH_MII_ RXD2 | - | - | - | EVEN TOUT |
|  | PB1 | - | TIM1_ CH3N | TIM3_ CH4 | TIM8_ CH3N | - | - | - | - | - | LCD_R6 | OTG_HS_ ULPI_D2 | ETH_MII_ RXD3 | - | - | - | EVEN TOUT |
|  | PB2 | - | - | - | - | - | - | - | - | - | - | - | - | - | - | - | EVEN TOUT |
|  | PB3 | JTDO/ TRAC ESWO | TIM2_ CH2 | - | - | - | SPI1_ SCK | SPI3_ SCK/ I2S3_CK | - | - | - | - | - | - | - | - | EVEN TOUT |
|  | PB4 | NJTR ST | - | TIM3_ CH1 | - | - | SPI1_ MISO | SPI3_ MISO | I2S3ext_ SD | - | - | - | - | - | - | - | EVEN TOUT |
|  | PB5 | - | - | TIM3_ CH2 | - | I2C1_ SMBA | SPI1_ MOSI | SPI3_ MOSI/ I2S3_SD | - | - | CAN2_RX | OTG_HS_ ULPI_D7 | ETH_PPS _OUT | FMC_ SDCKE1 | DCMI_ D10 | - | EVEN TOUT |
|  | PB6 | - | - | TIM4_ CH1 | - | I2C1_ SCL | - | - | USART1_ TX | - | CAN2_TX | - | - | FMC_ SDNE1 | DCMI_ D5 | - | EVEN TOUT |
|  | PB7 | - | - | TIM4_ CH2 | - | I2C1_ SDA | - | - | USART1_ RX | - | - | - | - | FMC_NL | DCMI_ VSYNC | - | EVEN TOUT |
|  | PB8 | - | - | TIM4_ CH3 | TIM10_ CH1 | I2C1_ SCL | - | - | - | - | CAN1_RX | - | ETH_MII_ TXD3 | SDIO_D4 | DCMI_ D6 | LCD_B6 | EVEN TOUT |
|  | PB9 | - | - | TIM4_ CH4 | TIM11_ CH1 | I2C1_ SDA | SPI2_ NSS/I2 S2_WS | - | - | - | CAN1_TX | - | - | SDIO_D5 | DCMI_ D7 | LCD_B7 | EVEN TOUT |
|  | PB10 | - | TIM2_ CH3 | - | - | I2C2_ SCL | SPI2_ SCK/I2 S2_CK | - | USART3_ TX | - | - | OTG_HS_ ULPI_D3 | ETH_MII_ RX_ER | - | - | LCD_G4 | EVEN TOUT |

76/240
DS9405
Rev
13
Pinouts
and
pin
description
STM32F427xx
STM32F429xx
Table 12. STM32F427xx and STM32F429xx alternate function mapping (continued)
AF0 AF1 AF2 AF3 AF4 AF5 AF6 AF7 AF8 AF9 AF10 AF11 AF12 AF13 AF14 AF15
Port SPI3/ USART6/ CAN1/2/ OTG2_HS
TIM8/9/ I2C1/ SPI1/2/ SPI2/3/ FMC/SDIO
SYS TIM1/2 TIM3/4/5 USART1/ UART4/5/7 TIM12/13/14 /OTG1_ ETH DCMI LCD SYS
10/11 2/3 3/4/5/6 SAI1 /OTG2_FS
2/3 /8 /LCD FS
JTMS-
EVEN
PA13 SWDI - - - - - - - - - - - - - -
TOUT
O
JTCK-
EVEN
Port A PA14 SWCL - - - - - - - - - - - - - -
TOUT
K
TIM2_ SPI3_
SPI1_ EVEN
PA15 JTDI CH1/TIM2 - - - NSS/ - - - - - - - -
NSS TOUT
_ETR I2S3_WS
TIM1_ TIM3_ TIM8_ OTG_HS_ ETH_MII_ EVEN
PB0 - - - - - - LCD_R3 - - -
CH2N CH3 CH2N ULPI_D1 RXD2 TOUT
TIM1_ TIM3_ TIM8_ OTG_HS_ ETH_MII_ EVEN
PB1 - - - - - - LCD_R6 - - -
CH3N CH4 CH3N ULPI_D2 RXD3 TOUT
EVEN
PB2 - - - - - - - - - - - - - - -
TOUT
JTDO/ SPI3_ TIM2_ SPI1_ EVEN
PB3 TRAC - - - SCK/ - - - - - - - -
CH2 SCK TOUT
ESWO I2S3_CK
NJTR TIM3_ SPI1_ SPI3_ I2S3ext_ EVEN
PB4 - - - - - - - - - -
ST CH1 MISO MISO SD TOUT
SPI3_
TIM3_ I2C1_ SPI1_ OTG_HS_ ETH_PPS FMC_ DCMI_ EVEN
PB5 - - - MOSI/ - - CAN2_RX -
Port B CH2 SMBA MOSI ULPI_D7 _OUT SDCKE1 D10 TOUT
I2S3_SD
TIM4_ I2C1_ USART1_ FMC_ DCMI_ EVEN
PB6 - - - - - - CAN2_TX - - -
CH1 SCL TX SDNE1 D5 TOUT
TIM4_ I2C1_ USART1_ DCMI_ EVEN
PB7 - - - - - - - - - FMC_NL -
CH2 SDA RX VSYNC TOUT
TIM4_ TIM10_ I2C1_ ETH_MII_ DCMI_ EVEN
PB8 - - - - - - CAN1_RX - SDIO_D4 LCD_B6
CH3 CH1 SCL TXD3 D6 TOUT
SPI2_
TIM4_ TIM11_ I2C1_ DCMI_ EVEN
PB9 - - NSS/I2 - - - CAN1_TX - - SDIO_D5 LCD_B7 CH4 CH1 SDA D7 TOUT
S2_WS
SPI2_
TIM2_ I2C2_ USART3_ OTG_HS_ ETH_MII_ EVEN
PB10 - - - SCK/I2 - - - - - LCD_G4
CH3 SCL TX ULPI_D3 RX_ER TOUT
S2_CK

<!-- Page 77 -->


| Port |  | AF0 | AF1 | AF2 | AF3 | AF4 | AF5 | AF6 | AF7 | AF8 | AF9 | AF10 | AF11 | AF12 | AF13 | AF14 | AF15 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
|  |  | SYS | TIM1/2 | TIM3/4/5 | TIM8/9/ 10/11 | I2C1/ 2/3 | SPI1/2/ 3/4/5/6 | SPI2/3/ SAI1 | SPI3/ USART1/ 2/3 | USART6/ UART4/5/7 /8 | CAN1/2/ TIM12/13/14 /LCD | OTG2_HS /OTG1_ FS | ETH | FMC/SDIO /OTG2_FS | DCMI | LCD | SYS |
| Port B | PB11 | - | TIM2_ CH4 | - | - | I2C2_ SDA | - | - | USART3_ RX | - | - | OTG_HS_ ULPI_D4 | ETH_MII_ TX_EN/ ETH_RMII _TX_EN | - | - | LCD_G5 | EVEN TOUT |
|  | PB12 | - | TIM1_ BKIN | - | - | I2C2_ SMBA | SPI2_ NSS/I2 S2_WS | - | USART3_ CK | - | CAN2_RX | OTG_HS_ ULPI_D5 | ETH_MII_ TXD0/ETH _RMII_ TXD0 | OTG_HS_ ID | - | - | EVEN TOUT |
|  | PB13 | - | TIM1_ CH1N | - | - | - | SPI2_ SCK/I2 S2_CK | - | USART3_ CTS | - | CAN2_TX | OTG_HS_ ULPI_D6 | ETH_MII_ TXD1/ETH _RMII_TX D1 | - | - | - | EVEN TOUT |
|  | PB14 | - | TIM1_ CH2N | - | TIM8_ CH2N | - | SPI2_ MISO | I2S2ext_ SD | USART3_ RTS | - | TIM12_CH1 | - | - | OTG_HS_ DM | - | - | EVEN TOUT |
|  | PB15 | RTC_ REFIN | TIM1_ CH3N | - | TIM8_ CH3N | - | SPI2_ MOSI/I2 S2_SD | - | - | - | TIM12_CH2 | - | - | OTG_HS_ DP | - | - | EVEN TOUT |
| Port C | PC0 | - | - | - | - | - | - | - | - | - | - | OTG_HS_ ULPI_STP | - | FMC_SDN WE | - | - | EVEN TOUT |
|  | PC1 | - | - | - | - | - | - | - | - | - | - | - | ETH_MDC | - | - | - | EVEN TOUT |
|  | PC2 | - | - | - | - | - | SPI2_ MISO | I2S2ext_ SD | - | - | - | OTG_HS_ ULPI_DIR | ETH_MII_ TXD2 | FMC_ SDNE0 | - | - | EVEN TOUT |
|  | PC3 | - | - | - | - | - | SPI2_ MOSI/I2 S2_SD | - | - | - | - | OTG_HS_ ULPI_NXT | ETH_MII_ TX_CLK | FMC_ SDCKE0 | - | - | EVEN TOUT |
|  | PC4 | - | - | - | - | - | - | - | - | - | - | - | ETH_MII_ RXD0/ETH _RMII_ RXD0 | - | - | - | EVEN TOUT |
|  | PC5 | - | - | - | - | - | - | - | - | - | - | - | ETH_MII_ RXD1/ETH _RMII_ RXD1 | - | - | - | EVEN TOUT |
|  | PC6 | - | - | TIM3_ CH1 | TIM8_ CH1 | - | I2S2_ MCK | - | - | USART6_ TX | - | - | - | SDIO_D6 | DCMI_ D0 | LCD_ HSYNC | EVEN TOUT |
|  | PC7 | - | - | TIM3_ CH2 | TIM8_ CH2 | - | - | I2S3_ MCK | - | USART6_ RX | - | - | - | SDIO_D7 | DCMI_ D1 | LCD_G6 | EVEN TOUT |

DS9405
Rev
13
77/240
STM32F427xx
STM32F429xx
Pinouts
and
pin
description
Table 12. STM32F427xx and STM32F429xx alternate function mapping (continued)
AF0 AF1 AF2 AF3 AF4 AF5 AF6 AF7 AF8 AF9 AF10 AF11 AF12 AF13 AF14 AF15
Port SPI3/ USART6/ CAN1/2/ OTG2_HS
TIM8/9/ I2C1/ SPI1/2/ SPI2/3/ FMC/SDIO
SYS TIM1/2 TIM3/4/5 USART1/ UART4/5/7 TIM12/13/14 /OTG1_ ETH DCMI LCD SYS
10/11 2/3 3/4/5/6 SAI1 /OTG2_FS
2/3 /8 /LCD FS
ETH_MII_
TIM2_ I2C2_ USART3_ OTG_HS_ TX_EN/ EVEN
PB11 - - - - - - - - - LCD_G5
CH4 SDA RX ULPI_D4 ETH_RMII TOUT
_TX_EN
ETH_MII_
SPI2_
TIM1_ I2C2_ USART3_ OTG_HS_ TXD0/ETH OTG_HS_ EVEN
PB12 - - - NSS/I2 - - CAN2_RX - -
BKIN SMBA CK ULPI_D5 _RMII_ ID TOUT
S2_WS
TXD0
Port B ETH_MII_
SPI2_
TIM1_ USART3_ OTG_HS_ TXD1/ETH EVEN
PB13 - - - - SCK/I2 - - CAN2_TX - - -
CH1N CTS ULPI_D6 _RMII_TX TOUT
S2_CK
D1
TIM1_ TIM8_ SPI2_ I2S2ext_ USART3_ OTG_HS_ EVEN
PB14 - - - - TIM12_CH1 - - - -
CH2N CH2N MISO SD RTS DM TOUT
SPI2_
RTC_ TIM1_ TIM8_ OTG_HS_ EVEN
PB15 - - MOSI/I2 - - - TIM12_CH2 - - - -
REFIN CH3N CH3N DP TOUT
S2_SD
OTG_HS_ FMC_SDN EVEN
PC0 - - - - - - - - - - - - -
ULPI_STP WE TOUT
EVEN
PC1 - - - - - - - - - - - ETH_MDC - - -
TOUT
SPI2_ I2S2ext_ OTG_HS_ ETH_MII_ FMC_ EVEN
PC2 - - - - - - - - - -
MISO SD ULPI_DIR TXD2 SDNE0 TOUT
SPI2_
OTG_HS_ ETH_MII_ FMC_ EVEN
PC3 - - - - - MOSI/I2 - - - - - -
ULPI_NXT TX_CLK SDCKE0 TOUT
S2_SD
Port ETH_MII_
C RXD0/ETH EVEN
PC4 - - - - - - - - - - - - - -
_RMII_ TOUT
RXD0
ETH_MII_
RXD1/ETH EVEN PC5 - - - - - - - - - - - - - -
_RMII_ TOUT
RXD1
TIM3_ TIM8_ I2S2_ USART6_ DCMI_ LCD_ EVEN
PC6 - - - - - - - - SDIO_D6
CH1 CH1 MCK TX D0 HSYNC TOUT
TIM3_ TIM8_ I2S3_ USART6_ DCMI_ EVEN
PC7 - - - - - - - - SDIO_D7 LCD_G6
CH2 CH2 MCK RX D1 TOUT

<!-- Page 78 -->


| Port |  | AF0 | AF1 | AF2 | AF3 | AF4 | AF5 | AF6 | AF7 | AF8 | AF9 | AF10 | AF11 | AF12 | AF13 | AF14 | AF15 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
|  |  | SYS | TIM1/2 | TIM3/4/5 | TIM8/9/ 10/11 | I2C1/ 2/3 | SPI1/2/ 3/4/5/6 | SPI2/3/ SAI1 | SPI3/ USART1/ 2/3 | USART6/ UART4/5/7 /8 | CAN1/2/ TIM12/13/14 /LCD | OTG2_HS /OTG1_ FS | ETH | FMC/SDIO /OTG2_FS | DCMI | LCD | SYS |
| Port C | PC8 | - | - | TIM3_ CH3 | TIM8_ CH3 | - | - | - | - | USART6_ CK | - | - | - | SDIO_D0 | DCMI_ D2 | - | EVEN TOUT |
|  | PC9 | MCO2 | - | TIM3_ CH4 | TIM8_ CH4 | I2C3_ SDA | I2S_ CKIN | - | - | - | - | - | - | SDIO_D1 | DCMI_ D3 | - | EVEN TOUT |
|  | PC10 | - | - | - | - | - | - | SPI3_ SCK/I2S 3_CK | USART3_ TX | UART4_TX | - | - | - | SDIO_D2 | DCMI_ D8 | LCD_R2 | EVEN TOUT |
|  | PC11 | - | - | - | - | - | I2S3ext _SD | SPI3_ MISO | USART3_ RX | UART4_RX | - | - | - | SDIO_D3 | DCMI_ D4 | - | EVEN TOUT |
|  | PC12 | - | - | - | - | - | - | SPI3_ MOSI/I2 S3_SD | USART3_ CK | UART5_TX | - | - | - | SDIO_CK | DCMI_ D9 | - | EVEN TOUT |
|  | PC13 | - | - | - | - | - | - | - | - | - | - | - | - | - | - | - | EVEN TOUT |
|  | PC14 | - | - | - | - | - | - | - | - | - | - | - | - | - | - | - | EVEN TOUT |
|  | PC15 | - | - | - | - | - | - | - | - | - | - | - | - | - | - | - | EVEN TOUT |
| Port D | PD0 | - | - | - | - | - | - | - | - | - | CAN1_RX | - | - | FMC_D2 | - | - | EVEN TOUT |
|  | PD1 | - | - | - | - | - | - | - | - | - | CAN1_TX | - | - | FMC_D3 | - | - | EVEN TOUT |
|  | PD2 | - | - | TIM3_ ETR | - | - | - | - | - | UART5_RX | - | - | - | SDIO_ CMD | DCMI_ D11 | - | EVEN TOUT |
|  | PD3 | - | - | - | - | - | SPI2_S CK/I 2S2_CK | - | USART2_ CTS | - | - | - | - | FMC_CLK | DCMI_ D5 | LCD_G7 | EVEN TOUT |
|  | PD4 | - | - | - | - | - | - | - | USART2_ RTS | - | - | - | - | FMC_NOE | - | - | EVEN TOUT |
|  | PD5 | - | - | - | - | - | - | - | USART2_ TX | - | - | - | - | FMC_NWE | - | - | EVEN TOUT |
|  | PD6 | - | - | - | - | - | SPI3_ MOSI/I2 S3_SD | SAI1_ SD_A | USART2_ RX | - | - | - | - | FMC_ NWAIT | DCMI_ D10 | LCD_B2 | EVEN TOUT |

78/240
DS9405
Rev
13
Pinouts
and
pin
description
STM32F427xx
STM32F429xx
Table 12. STM32F427xx and STM32F429xx alternate function mapping (continued)
AF0 AF1 AF2 AF3 AF4 AF5 AF6 AF7 AF8 AF9 AF10 AF11 AF12 AF13 AF14 AF15
Port SPI3/ USART6/ CAN1/2/ OTG2_HS
TIM8/9/ I2C1/ SPI1/2/ SPI2/3/ FMC/SDIO
SYS TIM1/2 TIM3/4/5 USART1/ UART4/5/7 TIM12/13/14 /OTG1_ ETH DCMI LCD SYS
10/11 2/3 3/4/5/6 SAI1 /OTG2_FS
2/3 /8 /LCD FS
TIM3_ TIM8_ USART6_ DCMI_ EVEN
PC8 - - - - - - - - - SDIO_D0 -
CH3 CH3 CK D2 TOUT
TIM3_ TIM8_ I2C3_ I2S_ DCMI_ EVEN
PC9 MCO2 - - - - - - - SDIO_D1 -
CH4 CH4 SDA CKIN D3 TOUT
SPI3_
USART3_ DCMI_ EVEN
PC10 - - - - - - SCK/I2S UART4_TX - - - SDIO_D2 LCD_R2
TX D8 TOUT
3_CK
I2S3ext SPI3_ USART3_ DCMI_ EVEN
PC11 - - - - - UART4_RX - - - SDIO_D3 -
_SD MISO RX D4 TOUT
Port
C
SPI3_
USART3_ DCMI_ EVEN
PC12 - - - - - - MOSI/I2 UART5_TX - - - SDIO_CK -
CK D9 TOUT
S3_SD
EVEN
PC13 - - - - - - - - - - - - - - -
TOUT
EVEN
PC14 - - - - - - - - - - - - - - -
TOUT
EVEN
PC15 - - - - - - - - - - - - - - - TOUT
EVEN
PD0 - - - - - - - - - CAN1_RX - - FMC_D2 - -
TOUT
EVEN
PD1 - - - - - - - - - CAN1_TX - - FMC_D3 - -
TOUT
TIM3_ SDIO_ DCMI_ EVEN
PD2 - - - - - - - UART5_RX - - - -
ETR CMD D11 TOUT
SPI2_S
USART2_ DCMI_ EVEN
Port PD3 - - - - - CK/I - - - - - FMC_CLK LCD_G7
CTS D5 TOUT
D 2S2_CK
USART2_ EVEN
PD4 - - - - - - - - - - - FMC_NOE - -
RTS TOUT
USART2_ EVEN PD5 - - - - - - - - - - - FMC_NWE - -
TX TOUT
SPI3_
SAI1_ USART2_ FMC_ DCMI_ EVEN
PD6 - - - - - MOSI/I2 - - - - LCD_B2
SD_A RX NWAIT D10 TOUT
S3_SD

<!-- Page 79 -->


| Port |  | AF0 | AF1 | AF2 | AF3 | AF4 | AF5 | AF6 | AF7 | AF8 | AF9 | AF10 | AF11 | AF12 | AF13 | AF14 | AF15 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
|  |  | SYS | TIM1/2 | TIM3/4/5 | TIM8/9/ 10/11 | I2C1/ 2/3 | SPI1/2/ 3/4/5/6 | SPI2/3/ SAI1 | SPI3/ USART1/ 2/3 | USART6/ UART4/5/7 /8 | CAN1/2/ TIM12/13/14 /LCD | OTG2_HS /OTG1_ FS | ETH | FMC/SDIO /OTG2_FS | DCMI | LCD | SYS |
| Port D | PD7 | - | - | - | - | - | - | - | USART2_ CK | - | - | - | - | FMC_NE1/ FMC_ NCE2 | - | - | EVEN TOUT |
|  | PD8 | - | - | - | - | - | - | - | USART3_ TX | - | - | - | - | FMC_D13 | - | - | EVEN TOUT |
|  | PD9 | - | - | - | - | - | - | - | USART3_ RX | - | - | - | - | FMC_D14 | - | - | EVEN TOUT |
|  | PD10 | - | - | - | - | - | - | - | USART3_ CK | - | - | - | - | FMC_D15 | - | LCD_B3 | EVEN TOUT |
|  | PD11 | - | - | - | - | - | - | - | USART3_ CTS | - | - | - | - | FMC_A16 | - | - | EVEN TOUT |
|  | PD12 | - | - | TIM4_ CH1 | - | - | - | - | USART3_ RTS | - | - | - | - | FMC_A17 | - | - | EVEN TOUT |
|  | PD13 | - | - | TIM4_ CH2 | - | - | - | - | - | - | - | - | - | FMC_A18 | - | - | EVEN TOUT |
|  | PD14 | - | - | TIM4_ CH3 | - | - | - | - | - | - | - | - | - | FMC_D0 | - | - | EVEN TOUT |
|  | PD15 | - | - | TIM4_ CH4 | - | - | - | - | - | - | - | - | - | FMC_D1 | - | - | EVEN TOUT |
| Port E | PE0 | - | - | TIM4_ ETR | - | - | - | - | - | UART8_Rx | - | - | - | FMC_ NBL0 | DCMI_ D2 | - | EVEN TOUT |
|  | PE1 | - | - | - | - | - | - | - | - | UART8_Tx | - | - | - | FMC_ NBL1 | DCMI_ D3 | - | EVEN TOUT |
|  | PE2 | TRAC ECLK | - | - | - | - | SPI4_ SCK | SAI1_ MCLK_A | - | - | - | - | ETH_MII_ TXD3 | FMC_A23 | - | - | EVEN TOUT |
|  | PE3 | TRAC ED0 | - | - | - | - | - | SAI1_ SD_B | - | - | - | - | - | FMC_A19 | - | - | EVEN TOUT |
|  | PE4 | TRAC ED1 | - | - | - | - | SPI4_ NSS | SAI1_ FS_A | - | - | - | - | - | FMC_A20 | DCMI_ D4 | LCD_B0 | EVEN TOUT |
|  | PE5 | TRAC ED2 | - | - | TIM9_ CH1 | - | SPI4_M ISO | SAI1_ SCK_A | - | - | - | - | - | FMC_A21 | DCMI_ D6 | LCD_G0 | EVEN TOUT |
|  | PE6 | TRAC ED3 | - | - | TIM9_ CH2 | - | SPI4_ MOSI | SAI1_ SD_A | - | - | - | - | - | FMC_A22 | DCMI_ D7 | LCD_G1 | EVEN TOUT |

DS9405
Rev
13
79/240
STM32F427xx
STM32F429xx
Pinouts
and
pin
description
Table 12. STM32F427xx and STM32F429xx alternate function mapping (continued)
AF0 AF1 AF2 AF3 AF4 AF5 AF6 AF7 AF8 AF9 AF10 AF11 AF12 AF13 AF14 AF15
Port SPI3/ USART6/ CAN1/2/ OTG2_HS
TIM8/9/ I2C1/ SPI1/2/ SPI2/3/ FMC/SDIO
SYS TIM1/2 TIM3/4/5 USART1/ UART4/5/7 TIM12/13/14 /OTG1_ ETH DCMI LCD SYS
10/11 2/3 3/4/5/6 SAI1 /OTG2_FS
2/3 /8 /LCD FS
FMC_NE1/
USART2_ EVEN
PD7 - - - - - - - - - - - FMC_ - -
CK TOUT
NCE2
USART3_ EVEN
PD8 - - - - - - - - - - - FMC_D13 - -
TX TOUT
USART3_ EVEN
PD9 - - - - - - - - - - - FMC_D14 - -
RX TOUT
USART3_ EVEN
PD10 - - - - - - - - - - - FMC_D15 - LCD_B3
CK TOUT
Port
USART3_ EVEN
D PD11 - - - - - - - - - - - FMC_A16 - -
CTS TOUT
TIM4_ USART3_ EVEN
PD12 - - - - - - - - - - FMC_A17 - - CH1 RTS TOUT
TIM4_ EVEN
PD13 - - - - - - - - - - - FMC_A18 - -
CH2 TOUT
TIM4_ EVEN
PD14 - - - - - - - - - - - FMC_D0 - -
CH3 TOUT
TIM4_ EVEN
PD15 - - - - - - - - - - - FMC_D1 - -
CH4 TOUT
TIM4_ FMC_ DCMI_ EVEN
PE0 - - - - - - - UART8_Rx - - - -
ETR NBL0 D2 TOUT
FMC_ DCMI_ EVEN
PE1 - - - - - - - - UART8_Tx - - - -
NBL1 D3 TOUT
TRAC SPI4_ SAI1_ ETH_MII_ EVEN
PE2 - - - - - - - - FMC_A23 - -
ECLK SCK MCLK_A TXD3 TOUT
TRAC SAI1_ EVEN
Port E PE3 - - - - - - - - - - FMC_A19 - -
ED0 SD_B TOUT
TRAC SPI4_ SAI1_ DCMI_ EVEN
PE4 - - - - - - - - - FMC_A20 LCD_B0
ED1 NSS FS_A D4 TOUT
TRAC TIM9_ SPI4_M SAI1_ DCMI_ EVEN
PE5 - - - - - - - - FMC_A21 LCD_G0
ED2 CH1 ISO SCK_A D6 TOUT
TRAC TIM9_ SPI4_ SAI1_ DCMI_ EVEN
PE6 - - - - - - - - FMC_A22 LCD_G1
ED3 CH2 MOSI SD_A D7 TOUT

<!-- Page 80 -->


| Port |  | AF0 | AF1 | AF2 | AF3 | AF4 | AF5 | AF6 | AF7 | AF8 | AF9 | AF10 | AF11 | AF12 | AF13 | AF14 | AF15 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
|  |  | SYS | TIM1/2 | TIM3/4/5 | TIM8/9/ 10/11 | I2C1/ 2/3 | SPI1/2/ 3/4/5/6 | SPI2/3/ SAI1 | SPI3/ USART1/ 2/3 | USART6/ UART4/5/7 /8 | CAN1/2/ TIM12/13/14 /LCD | OTG2_HS /OTG1_ FS | ETH | FMC/SDIO /OTG2_FS | DCMI | LCD | SYS |
| Port E | PE7 | - | TIM1_ ETR | - | - | - | - | - | - | UART7_Rx | - | - | - | FMC_D4 | - | - | EVEN TOUT |
|  | PE8 | - | TIM1_ CH1N | - | - | - | - | - | - | UART7_Tx | - | - | - | FMC_D5 | - | - | EVEN TOUT |
|  | PE9 | - | TIM1_ CH1 | - | - | - | - | - | - | - | - | - | - | FMC_D6 | - | - | EVEN TOUT |
|  | PE10 | - | TIM1_ CH2N | - | - | - | - | - | - | - | - | - | - | FMC_D7 | - | - | EVEN TOUT |
|  | PE11 | - | TIM1_ CH2 | - | - | - | SPI4_ NSS | - | - | - | - | - | - | FMC_D8 | - | LCD_G3 | EVEN TOUT |
|  | PE12 | - | TIM1_ CH3N | - | - | - | SPI4_ SCK | - | - | - | - | - | - | FMC_D9 | - | LCD_B4 | EVEN TOUT |
|  | PE13 | - | TIM1_ CH3 | - | - | - | SPI4_ MISO | - | - | - | - | - | - | FMC_D10 | - | LCD_DE | EVEN TOUT |
|  | PE14 | - | TIM1_ CH4 | - | - | - | SPI4_ MOSI | - | - | - | - | - | - | FMC_D11 | - | LCD_ CLK | EVEN TOUT |
|  | PE15 | - | TIM1_ BKIN | - | - | - |  | - | - | - | - | - | - | FMC_D12 | - | LCD_R7 | EVEN TOUT |
| Port F | PF0 | - | - | - | - | I2C2_ SDA | - | - | - | - | - | - | - | FMC_A0 | - | - | EVEN TOUT |
|  | PF1 | - |  |  |  | I2C2_ SCL | - | - | - | - | - | - | - | FMC_A1 | - | - | EVEN TOUT |
|  | PF2 | - | - | - | - | I2C2_ SMBA | - | - | - | - | - | - | - | FMC_A2 | - | - | EVEN TOUT |
|  | PF3 | - | - | - | - |  | - | - | - | - | - | - | - | FMC_A3 | - | - | EVEN TOUT |
|  | PF4 | - | - | - | - |  | - | - | - | - | - | - | - | FMC_A4 | - | - | EVEN TOUT |
|  | PF5 | - | - | - | - |  | - | - | - | - | - | - | - | FMC_A5 | - | - | EVEN TOUT |
|  | PF6 | - | - | - | TIM10_ CH1 | - | SPI5_ NSS | SAI1_ SD_B | - | UART7_Rx | - | - | - | FMC_ NIORD | - | - | EVEN TOUT |
|  | PF7 | - | - | - | TIM11_ CH1 | - | SPI5_ SCK | SAI1_ MCLK_B | - | UART7_Tx | - | - | - | FMC_ NREG | - | - | EVEN TOUT |

80/240
DS9405
Rev
13
Pinouts
and
pin
description
STM32F427xx
STM32F429xx
Table 12. STM32F427xx and STM32F429xx alternate function mapping (continued)
AF0 AF1 AF2 AF3 AF4 AF5 AF6 AF7 AF8 AF9 AF10 AF11 AF12 AF13 AF14 AF15
Port SPI3/ USART6/ CAN1/2/ OTG2_HS
TIM8/9/ I2C1/ SPI1/2/ SPI2/3/ FMC/SDIO
SYS TIM1/2 TIM3/4/5 USART1/ UART4/5/7 TIM12/13/14 /OTG1_ ETH DCMI LCD SYS
10/11 2/3 3/4/5/6 SAI1 /OTG2_FS
2/3 /8 /LCD FS
TIM1_ EVEN
PE7 - - - - - - - UART7_Rx - - - FMC_D4 - -
ETR TOUT
TIM1_ EVEN
PE8 - - - - - - - UART7_Tx - - - FMC_D5 - -
CH1N TOUT
TIM1_ EVEN
PE9 - - - - - - - - - - - FMC_D6 - -
CH1 TOUT
TIM1_ EVEN
PE10 - - - - - - - - - - - FMC_D7 - -
CH2N TOUT
TIM1_ SPI4_ EVEN
Port E PE11 - - - - - - - - - - FMC_D8 - LCD_G3
CH2 NSS TOUT
TIM1_ SPI4_ EVEN
PE12 - - - - - - - - - - FMC_D9 - LCD_B4
CH3N SCK TOUT
TIM1_ SPI4_ EVEN
PE13 - - - - - - - - - - FMC_D10 - LCD_DE
CH3 MISO TOUT
TIM1_ SPI4_ LCD_ EVEN
PE14 - - - - - - - - - - FMC_D11 -
CH4 MOSI CLK TOUT
TIM1_ EVEN
PE15 - - - - - - - - - - FMC_D12 - LCD_R7
BKIN TOUT
I2C2_ EVEN
PF0 - - - - - - - - - - - FMC_A0 - -
SDA TOUT
I2C2_ EVEN
PF1 - - - - - - - - FMC_A1 - -
SCL TOUT
I2C2_ EVEN
PF2 - - - - - - - - - - - FMC_A2 - -
SMBA TOUT
EVEN
PF3 - - - - - - - - - - - FMC_A3 - -
TOUT
Port F
EVEN
PF4 - - - - - - - - - - - FMC_A4 - -
TOUT
EVEN
PF5 - - - - - - - - - - - FMC_A5 - -
TOUT
TIM10_ SPI5_ SAI1_ FMC_ EVEN
PF6 - - - - - UART7_Rx - - - - -
CH1 NSS SD_B NIORD TOUT
TIM11_ SPI5_ SAI1_ FMC_ EVEN
PF7 - - - - - UART7_Tx - - - - -
CH1 SCK MCLK_B NREG TOUT

<!-- Page 81 -->


| Port |  | AF0 | AF1 | AF2 | AF3 | AF4 | AF5 | AF6 | AF7 | AF8 | AF9 | AF10 | AF11 | AF12 | AF13 | AF14 | AF15 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
|  |  | SYS | TIM1/2 | TIM3/4/5 | TIM8/9/ 10/11 | I2C1/ 2/3 | SPI1/2/ 3/4/5/6 | SPI2/3/ SAI1 | SPI3/ USART1/ 2/3 | USART6/ UART4/5/7 /8 | CAN1/2/ TIM12/13/14 /LCD | OTG2_HS /OTG1_ FS | ETH | FMC/SDIO /OTG2_FS | DCMI | LCD | SYS |
| Port F | PF8 | - | - | - | - | - | SPI5_ MISO | SAI1_ SCK_B | - | - | TIM13_CH1 | - | - | FMC_ NIOWR | - | - | EVEN TOUT |
|  | PF9 | - | - | - | - | - | SPI5_ MOSI | SAI1_ FS_B | - | - | TIM14_CH1 | - | - | FMC_CD | - | - | EVEN TOUT |
|  | PF10 | - | - | - | - | - | - | - | - | - | - | - | - | FMC_INTR | DCMI_ D11 | LCD_DE | EVEN TOUT |
|  | PF11 | - | - | - | - | - | SPI5_ MOSI | - | - | - | - | - | - | FMC_ SDNRAS | DCMI_ D12 | - | EVEN TOUT |
|  | PF12 | - | - | - | - | - | - | - | - | - | - | - | - | FMC_A6 | - | - | EVEN TOUT |
|  | PF13 | - | - | - | - | - | - | - | - | - | - | - | - | FMC_A7 | - | - | EVEN TOUT |
|  | PF14 | - | - | - | - | - | - | - | - | - | - | - | - | FMC_A8 | - | - | EVEN TOUT |
|  | PF15 | - | - | - | - | - | - | - | - | - | - | - | - | FMC_A9 | - | - | EVEN TOUT |
| Port G | PG0 | - | - | - | - | - | - | - | - | - | - | - | - | FMC_A10 | - | - | EVEN TOUT |
|  | PG1 | - | - | - | - | - | - | - | - | - | - | - | - | FMC_A11 | - | - | EVEN TOUT |
|  | PG2 | - | - | - | - | - | - | - | - | - | - | - | - | FMC_A12 | - | - | EVEN TOUT |
|  | PG3 | - | - | - | - | - | - | - | - | - | - | - | - | FMC_A13 | - | - | EVEN TOUT |
|  | PG4 | - | - | - | - | - | - | - | - | - | - | - | - | FMC_A14/ FMC_BA0 | - | - | EVEN TOUT |
|  | PG5 | - | - | - | - | - | - | - | - | - | - | - | - | FMC_A15/ FMC_BA1 | - | - | EVEN TOUT |
|  | PG6 | - | - | - | - | - | - | - | - | - | - | - | - | FMC_INT2 | DCMI_ D12 | LCD_R7 | EVEN TOUT |
|  | PG7 | - | - | - | - | - | - | - | - | USART6_ CK | - | - | - | FMC_INT3 | DCMI_ D13 | LCD_ CLK | EVEN TOUT |
|  | PG8 | - | - | - | - | - | SPI6_ NSS | - | - | USART6_ RTS | - | - | ETH_PPS _OUT | FMC_SDC LK | - | - | EVEN TOUT |

DS9405
Rev
13
81/240
STM32F427xx
STM32F429xx
Pinouts
and
pin
description
Table 12. STM32F427xx and STM32F429xx alternate function mapping (continued)
AF0 AF1 AF2 AF3 AF4 AF5 AF6 AF7 AF8 AF9 AF10 AF11 AF12 AF13 AF14 AF15
Port SPI3/ USART6/ CAN1/2/ OTG2_HS
TIM8/9/ I2C1/ SPI1/2/ SPI2/3/ FMC/SDIO
SYS TIM1/2 TIM3/4/5 USART1/ UART4/5/7 TIM12/13/14 /OTG1_ ETH DCMI LCD SYS
10/11 2/3 3/4/5/6 SAI1 /OTG2_FS
2/3 /8 /LCD FS
SPI5_ SAI1_ FMC_ EVEN
PF8 - - - - - - - TIM13_CH1 - - - -
MISO SCK_B NIOWR TOUT
SPI5_ SAI1_ EVEN
PF9 - - - - - - - TIM14_CH1 - - FMC_CD - -
MOSI FS_B TOUT
DCMI_ EVEN
PF10 - - - - - - - - - - - - FMC_INTR LCD_DE
D11 TOUT
SPI5_ FMC_ DCMI_ EVEN
PF11 - - - - - - - - - - - -
MOSI SDNRAS D12 TOUT
Port F
EVEN
PF12 - - - - - - - - - - - - FMC_A6 - -
TOUT
EVEN
PF13 - - - - - - - - - - - - FMC_A7 - -
TOUT
EVEN
PF14 - - - - - - - - - - - - FMC_A8 - -
TOUT
EVEN
PF15 - - - - - - - - - - - - FMC_A9 - -
TOUT
EVEN
PG0 - - - - - - - - - - - - FMC_A10 - -
TOUT
EVEN
PG1 - - - - - - - - - - - - FMC_A11 - -
TOUT
EVEN
PG2 - - - - - - - - - - - - FMC_A12 - -
TOUT
EVEN
PG3 - - - - - - - - - - - - FMC_A13 - - TOUT
Port FMC_A14/ EVEN
PG4 - - - - - - - - - - - - - -
G FMC_BA0 TOUT
FMC_A15/ EVEN
PG5 - - - - - - - - - - - - - -
FMC_BA1 TOUT
DCMI_ EVEN
PG6 - - - - - - - - - - - - FMC_INT2 LCD_R7
D12 TOUT
USART6_ DCMI_ LCD_ EVEN
PG7 - - - - - - - - - - - FMC_INT3
CK D13 CLK TOUT
SPI6_ USART6_ ETH_PPS FMC_SDC EVEN
PG8 - - - - - - - - - - -
NSS RTS _OUT LK TOUT

<!-- Page 82 -->


| Port |  | AF0 | AF1 | AF2 | AF3 | AF4 | AF5 | AF6 | AF7 | AF8 | AF9 | AF10 | AF11 | AF12 | AF13 | AF14 | AF15 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
|  |  | SYS | TIM1/2 | TIM3/4/5 | TIM8/9/ 10/11 | I2C1/ 2/3 | SPI1/2/ 3/4/5/6 | SPI2/3/ SAI1 | SPI3/ USART1/ 2/3 | USART6/ UART4/5/7 /8 | CAN1/2/ TIM12/13/14 /LCD | OTG2_HS /OTG1_ FS | ETH | FMC/SDIO /OTG2_FS | DCMI | LCD | SYS |
| Port G | PG9 | - | - | - | - | - | - | - | - | USART6_ RX | - | - | - | FMC_NE2/ FMC_ NCE3 | DCMI_ VSYNC (1) | - | EVEN TOUT |
|  | PG10 | - | - | - | - | - | - | - | - | - | LCD_G3 | - | - | FMC_ NCE4_1/ FMC_NE3 | DCMI_ D2 | LCD_B2 | EVEN TOUT |
|  | PG11 | - | - | - | - | - | - | - | - | - | - | - | ETH_MII_ TX_EN/ ETH_RMII _TX_EN | FMC_ NCE4_2 | DCMI_ D3 | LCD_B3 | EVEN TOUT |
|  | PG12 | - | - | - | - | - | SPI6_ MISO | - | - | USART6_ RTS | LCD_B4 | - | - | FMC_NE4 | - | LCD_B1 | EVEN TOUT |
|  | PG13 | - | - | - | - | - | SPI6_ SCK | - | - | USART6_ CTS | - | - | ETH_MII_ TXD0/ ETH_RMII _TXD0 | FMC_A24 | - | - | EVEN TOUT |
|  | PG14 | - | - | - | - | - | SPI6_ MOSI | - | - | USART6_ TX | - | - | ETH_MII_ TXD1/ ETH_RMII _TXD1 | FMC_A25 | - | - | EVEN TOUT |
|  | PG15 | - | - | - | - | - | - | - | - | USART6_ CTS | - | - | - | FMC_ SDNCAS | DCMI_ D13 | - | EVEN TOUT |
| Port H | PH0 | - | - | - | - | - | - | - | - | - | - | - | - | - | - | - | EVEN TOUT |
|  | PH1 | - | - | - | - | - | - | - | - | - | - | - | - | - | - | - | EVEN TOUT |
|  | PH2 | - | - | - | - | - | - | - | - | - | - | - | ETH_MII_ CRS | FMC_ SDCKE0 | - | LCD_R0 | EVEN TOUT |
|  | PH3 | - | - | - | - | - | - | - | - | - | - | - | ETH_MII_ COL | FMC_SDN E0 | - | LCD_R1 | EVEN TOUT |
|  | PH4 | - | - | - | - | I2C2_ SCL | - | - | - | - | - | OTG_HS_ ULPI_NXT | - | - | - | - | EVEN TOUT |
|  | PH5 | - | - | - | - | I2C2_ SDA | SPI5_N SS | - | - | - | - | - | - | FMC_SDN WE | - | - | EVEN TOUT |
|  | PH6 | - | - | - | - | I2C2_ SMBA | SPI5_ SCK | - | - | - | TIM12_CH1 | - | ETH_MII_ RXD2 | FMC_ SDNE1 | DCMI_ D8 | - | - |

82/240
DS9405
Rev
13
Pinouts
and
pin
description
STM32F427xx
STM32F429xx
Table 12. STM32F427xx and STM32F429xx alternate function mapping (continued)
AF0 AF1 AF2 AF3 AF4 AF5 AF6 AF7 AF8 AF9 AF10 AF11 AF12 AF13 AF14 AF15
Port SPI3/ USART6/ CAN1/2/ OTG2_HS
TIM8/9/ I2C1/ SPI1/2/ SPI2/3/ FMC/SDIO
SYS TIM1/2 TIM3/4/5 USART1/ UART4/5/7 TIM12/13/14 /OTG1_ ETH DCMI LCD SYS
10/11 2/3 3/4/5/6 SAI1 /OTG2_FS
2/3 /8 /LCD FS
FMC_NE2/ DCMI_
USART6_ EVEN
PG9 - - - - - - - - - - - FMC_ VSYNC -
RX NCE3 (1) TOUT
FMC_
DCMI_ EVEN
PG10 - - - - - - - - - LCD_G3 - - NCE4_1/ LCD_B2
D2 TOUT
FMC_NE3
ETH_MII_
TX_EN/ FMC_ DCMI_ EVEN
PG11 - - - - - - - - - - - LCD_B3
ETH_RMII NCE4_2 D3 TOUT
_TX_EN
Port SPI6_ USART6_ EVEN
PG12 - - - - - - - LCD_B4 - - FMC_NE4 - LCD_B1
G MISO RTS TOUT
ETH_MII_
SPI6_ USART6_ TXD0/ EVEN
PG13 - - - - - - - - - FMC_A24 - -
SCK CTS ETH_RMII TOUT
_TXD0
ETH_MII_
SPI6_ USART6_ TXD1/ EVEN
PG14 - - - - - - - - - FMC_A25 - -
MOSI TX ETH_RMII TOUT
_TXD1
USART6_ FMC_ DCMI_ EVEN
PG15 - - - - - - - - - - - -
CTS SDNCAS D13 TOUT
EVEN
PH0 - - - - - - - - - - - - - - -
TOUT
EVEN
PH1 - - - - - - - - - - - - - - -
TOUT
ETH_MII_ FMC_ EVEN
PH2 - - - - - - - - - - - - LCD_R0
CRS SDCKE0 TOUT
Port ETH_MII_ FMC_SDN EVEN
PH3 - - - - - - - - - - - - LCD_R1
H COL E0 TOUT
I2C2_ OTG_HS_ EVEN
PH4 - - - - - - - - - - - - -
SCL ULPI_NXT TOUT
I2C2_ SPI5_N FMC_SDN EVEN
PH5 - - - - - - - - - - - -
SDA SS WE TOUT
I2C2_ SPI5_ ETH_MII_ FMC_ DCMI_
PH6 - - - - - - - TIM12_CH1 - - -
SMBA SCK RXD2 SDNE1 D8

<!-- Page 83 -->


| Port |  | AF0 | AF1 | AF2 | AF3 | AF4 | AF5 | AF6 | AF7 | AF8 | AF9 | AF10 | AF11 | AF12 | AF13 | AF14 | AF15 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
|  |  | SYS | TIM1/2 | TIM3/4/5 | TIM8/9/ 10/11 | I2C1/ 2/3 | SPI1/2/ 3/4/5/6 | SPI2/3/ SAI1 | SPI3/ USART1/ 2/3 | USART6/ UART4/5/7 /8 | CAN1/2/ TIM12/13/14 /LCD | OTG2_HS /OTG1_ FS | ETH | FMC/SDIO /OTG2_FS | DCMI | LCD | SYS |
| Port H | PH7 | - | - | - | - | I2C3_ SCL | SPI5_ MISO | - | - | - | - | - | ETH_MII_ RXD3 | FMC_ SDCKE1 | DCMI_ D9 | - | - |
|  | PH8 | - | - | - | - | I2C3_ SDA | - | - | - | - | - | - | - | FMC_D16 | DCMI_ HSYNC | LCD_R2 | EVEN TOUT |
|  | PH9 | - | - | - | - | I2C3_ SMBA | - | - | - | - | TIM12_CH2 | - | - | FMC_D17 | DCMI_ D0 | LCD_R3 | EVEN TOUT |
|  | PH10 | - | - | TIM5_ CH1 | - | - | - | - | - | - | - | - | - | FMC_D18 | DCMI_ D1 | LCD_R4 | EVEN TOUT |
|  | PH11 | - | - | TIM5_ CH2 | - | - | - | - | - | - | - | - | - | FMC_D19 | DCMI_ D2 | LCD_R5 | EVEN TOUT |
|  | PH12 | - | - | TIM5_ CH3 | - | - | - | - | - | - | - | - | - | FMC_D20 | DCMI_ D3 | LCD_R6 | EVEN TOUT |
|  | PH13 | - | - | - | TIM8_ CH1N | - | - | - | - | - | CAN1_TX | - | - | FMC_D21 | - | LCD_G2 | EVEN TOUT |
|  | PH14 | - | - | - | TIM8_ CH2N | - | - | - | - | - | - | - | - | FMC_D22 | DCMI_ D4 | LCD_G3 | EVEN TOUT |
|  | PH15 | - | - | - | TIM8_ CH3N | - | - | - | - | - | - | - | - | FMC_D23 | DCMI_ D11 | LCD_G4 | EVEN TOUT |
| Port I | PI0 | - | - | TIM5_ CH4 | - | - | SPI2_ NSS/I2 S2_WS | - | - | - | - | - | - | FMC_D24 | DCMI_ D13 | LCD_G5 | EVEN TOUT |
|  | PI1 | - | - | - | - | - | SPI2_ SCK/I2 S2_CK | - | - | - | - | - | - | FMC_D25 | DCMI_ D8 | LCD_G6 | EVEN TOUT |
|  | PI2 | - | - | - | TIM8_ CH4 | - | SPI2_ MISO | I2S2ext_ SD | - | - | - | - | - | FMC_D26 | DCMI_ D9 | LCD_G7 | EVEN TOUT |
|  | PI3 | - | - | - | TIM8_ ETR | - | SPI2_M OSI/I2S 2_SD |  |  |  |  |  |  | FMC_D27 | DCMI_D 10 |  | EVEN TOUT |
|  | PI4 | - | - | - | TIM8_ BKIN | - | - | - | - | - | - | - | - | FMC_ NBL2 | DCMI_D 5 | LCD_B4 | EVEN TOUT |
|  | PI5 | - | - | - | TIM8_ CH1 | - | - | - | - | - | - | - | - | FMC_ NBL3 | DCMI_ VSYNC | LCD_B5 | EVEN TOUT |
|  | PI6 | - | - | - | TIM8_ CH2 | - | - | - | - | - | - | - | - | FMC_D28 | DCMI_ D6 | LCD_B6 | EVEN TOUT |

DS9405
Rev
13
83/240
STM32F427xx
STM32F429xx
Pinouts
and
pin
description
Table 12. STM32F427xx and STM32F429xx alternate function mapping (continued)
AF0 AF1 AF2 AF3 AF4 AF5 AF6 AF7 AF8 AF9 AF10 AF11 AF12 AF13 AF14 AF15
Port SPI3/ USART6/ CAN1/2/ OTG2_HS
TIM8/9/ I2C1/ SPI1/2/ SPI2/3/ FMC/SDIO
SYS TIM1/2 TIM3/4/5 USART1/ UART4/5/7 TIM12/13/14 /OTG1_ ETH DCMI LCD SYS
10/11 2/3 3/4/5/6 SAI1 /OTG2_FS
2/3 /8 /LCD FS
I2C3_ SPI5_ ETH_MII_ FMC_ DCMI_
PH7 - - - - - - - - - - -
SCL MISO RXD3 SDCKE1 D9
I2C3_ DCMI_ EVEN
PH8 - - - - - - - - - - - FMC_D16 LCD_R2
SDA HSYNC TOUT
I2C3_ DCMI_ EVEN
PH9 - - - - - - - - TIM12_CH2 - - FMC_D17 LCD_R3
SMBA D0 TOUT
TIM5_ DCMI_ EVEN
PH10 - - - - - - - - - - - FMC_D18 LCD_R4
CH1 D1 TOUT
Port TIM5_ DCMI_ EVEN
PH11 - - - - - - - - - - - FMC_D19 LCD_R5
H CH2 D2 TOUT
TIM5_ DCMI_ EVEN
PH12 - - - - - - - - - - - FMC_D20 LCD_R6
CH3 D3 TOUT
TIM8_ EVEN
PH13 - - - - - - - - CAN1_TX - - FMC_D21 - LCD_G2
CH1N TOUT
TIM8_ DCMI_ EVEN PH14 - - - - - - - - - - - FMC_D22 LCD_G3
CH2N D4 TOUT
TIM8_ DCMI_ EVEN
PH15 - - - - - - - - - - - FMC_D23 LCD_G4
CH3N D11 TOUT
SPI2_
TIM5_ DCMI_ EVEN
PI0 - - - - NSS/I2 - - - - - - FMC_D24 LCD_G5
CH4 D13 TOUT
S2_WS
SPI2_
DCMI_ EVEN
PI1 - - - - - SCK/I2 - - - - - - FMC_D25 LCD_G6
D8 TOUT
S2_CK
TIM8_ SPI2_ I2S2ext_ DCMI_ EVEN
PI2 - - - - - - - - - FMC_D26 LCD_G7
CH4 MISO SD D9 TOUT
Port I SPI2_M
TIM8_ DCMI_D EVEN PI3 - - - - OSI/I2S FMC_D27
ETR 10 TOUT
2_SD
TIM8_ FMC_ DCMI_D EVEN
PI4 - - - - - - - - - - - LCD_B4
BKIN NBL2 5 TOUT
TIM8_ FMC_ DCMI_ EVEN
PI5 - - - - - - - - - - - LCD_B5
CH1 NBL3 VSYNC TOUT
TIM8_ DCMI_ EVEN
PI6 - - - - - - - - - - - FMC_D28 LCD_B6
CH2 D6 TOUT

<!-- Page 84 -->


| Port |  | AF0 | AF1 | AF2 | AF3 | AF4 | AF5 | AF6 | AF7 | AF8 | AF9 | AF10 | AF11 | AF12 | AF13 | AF14 | AF15 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
|  |  | SYS | TIM1/2 | TIM3/4/5 | TIM8/9/ 10/11 | I2C1/ 2/3 | SPI1/2/ 3/4/5/6 | SPI2/3/ SAI1 | SPI3/ USART1/ 2/3 | USART6/ UART4/5/7 /8 | CAN1/2/ TIM12/13/14 /LCD | OTG2_HS /OTG1_ FS | ETH | FMC/SDIO /OTG2_FS | DCMI | LCD | SYS |
| Port I | PI7 | - | - | - | TIM8_ CH3 | - | - | - | - | - | - | - | - | FMC_D29 | DCMI_ D7 | LCD_B7 | EVEN TOUT |
|  | PI8 | - | - | - | - | - | - | - | - | - | - | - | - | - | - | - | EVEN TOUT |
|  | PI9 | - | - | - | - | - | - | - | - | - | CAN1_RX | - | - | FMC_D30 | - | LCD_ VSYNC | EVEN TOUT |
|  | PI10 | - | - | - | - | - | - | - | - | - | - | - | ETH_MII_ RX_ER | FMC_D31 | - | LCD_ HSYNC | EVEN TOUT |
|  | PI11 | - | - | - | - | - | - | - | - | - | - | OTG_HS_ ULPI_DIR | - | - | - | - | EVEN TOUT |
|  | PI12 | - | - | - | - | - | - | - | - | - | - | - | - | - | - | LCD_ HSYNC | EVEN TOUT |
|  | PI13 | - | - | - | - | - | - | - | - | - | - | - | - | - | - | LCD_ VSYNC | EVEN TOUT |
|  | PI14 | - | - | - | - | - | - | - | - | - | - | - | - | - | - | LCD_ CLK | EVEN TOUT |
|  | PI15 | - | - | - | - | - | - | - | - | - | - | - | - | - | - | LCD_R0 | EVEN TOUT |
| Port J | PJ0 | - | - | - | - | - | - | - | - | - | - | - | - | - | - | LCD_R1 | EVEN TOUT |
|  | PJ1 | - | - | - | - | - | - | - | - | - | - | - | - | - | - | LCD_R2 | EVEN TOUT |
|  | PJ2 | - | - | - | - | - | - | - | - | - | - | - | - | - | - | LCD_R3 | EVEN TOUT |
|  | PJ3 | - | - | - | - | - | - | - | - | - | - | - | - | - | - | LCD_R4 | EVEN TOUT |
|  | PJ4 | - | - | - | - | - | - | - | - | - | - | - | - | - | - | LCD_R5 | EVEN TOUT |
|  | PJ5 | - | - | - | - | - | - | - | - | - | - | - | - | - | - | LCD_R6 | EVEN TOUT |
|  | PJ6 | - | - | - | - | - | - | - | - | - | - | - | - | - | - | LCD_R7 | EVEN TOUT |
|  | PJ7 | - | - | - | - | - | - | - | - | - | - | - | - | - | - | LCD_G0 | EVEN TOUT |

84/240
DS9405
Rev
13
Pinouts
and
pin
description
STM32F427xx
STM32F429xx
Table 12. STM32F427xx and STM32F429xx alternate function mapping (continued)
AF0 AF1 AF2 AF3 AF4 AF5 AF6 AF7 AF8 AF9 AF10 AF11 AF12 AF13 AF14 AF15
Port SPI3/ USART6/ CAN1/2/ OTG2_HS
TIM8/9/ I2C1/ SPI1/2/ SPI2/3/ FMC/SDIO
SYS TIM1/2 TIM3/4/5 USART1/ UART4/5/7 TIM12/13/14 /OTG1_ ETH DCMI LCD SYS
10/11 2/3 3/4/5/6 SAI1 /OTG2_FS
2/3 /8 /LCD FS
TIM8_ DCMI_ EVEN
PI7 - - - - - - - - - - - FMC_D29 LCD_B7
CH3 D7 TOUT
EVEN
PI8 - - - - - - - - - - - - - - -
TOUT
LCD_ EVEN
PI9 - - - - - - - - - CAN1_RX - - FMC_D30 -
VSYNC TOUT
ETH_MII_ LCD_ EVEN
PI10 - - - - - - - - - - - FMC_D31 -
RX_ER HSYNC TOUT
OTG_HS_ EVEN
Port I PI11 - - - - - - - - - - - - - -
ULPI_DIR TOUT
LCD_ EVEN
PI12 - - - - - - - - - - - - - -
HSYNC TOUT
LCD_ EVEN
PI13 - - - - - - - - - - - - - -
VSYNC TOUT
LCD_ EVEN
PI14 - - - - - - - - - - - - - -
CLK TOUT
EVEN
PI15 - - - - - - - - - - - - - - LCD_R0
TOUT
EVEN
PJ0 - - - - - - - - - - - - - - LCD_R1
TOUT
EVEN
PJ1 - - - - - - - - - - - - - - LCD_R2
TOUT
EVEN
PJ2 - - - - - - - - - - - - - - LCD_R3
TOUT
EVEN
PJ3 - - - - - - - - - - - - - - LCD_R4
TOUT
Port J
EVEN
PJ4 - - - - - - - - - - - - - - LCD_R5
TOUT
EVEN
PJ5 - - - - - - - - - - - - - - LCD_R6
TOUT
EVEN
PJ6 - - - - - - - - - - - - - - LCD_R7
TOUT
EVEN
PJ7 - - - - - - - - - - - - - - LCD_G0
TOUT

<!-- Page 85 -->


| Port |  | AF0 | AF1 | AF2 | AF3 | AF4 | AF5 | AF6 | AF7 | AF8 | AF9 | AF10 | AF11 | AF12 | AF13 | AF14 | AF15 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
|  |  | SYS | TIM1/2 | TIM3/4/5 | TIM8/9/ 10/11 | I2C1/ 2/3 | SPI1/2/ 3/4/5/6 | SPI2/3/ SAI1 | SPI3/ USART1/ 2/3 | USART6/ UART4/5/7 /8 | CAN1/2/ TIM12/13/14 /LCD | OTG2_HS /OTG1_ FS | ETH | FMC/SDIO /OTG2_FS | DCMI | LCD | SYS |
| Port J | PJ8 | - | - | - | - | - | - | - | - | - | - | - | - | - | - | LCD_G1 | EVEN TOUT |
|  | PJ9 | - | - | - | - | - | - | - | - | - | - | - | - | - | - | LCD_G2 | EVEN TOUT |
|  | PJ10 | - | - | - | - | - | - | - | - | - | - | - | - | - | - | LCD_G3 | EVEN TOUT |
|  | PJ11 | - | - | - | - | - | - | - | - | - | - | - | - | - | - | LCD_G4 | EVEN TOUT |
|  | PJ12 | - | - | - | - | - | - | - | - | - | - | - | - | - | - | LCD_B0 | EVEN TOUT |
|  | PJ13 | - | - | - | - | - | - | - | - | - | - | - | - | - | - | LCD_B1 | EVEN TOUT |
|  | PJ14 | - | - | - | - | - | - | - | - | - | - | - | - | - | - | LCD_B2 | EVEN TOUT |
|  | PJ15 | - | - | - | - | - | - | - | - | - | - | - | - | - | - | LCD_B3 | EVEN TOUT |
| Port K | PK0 | - | - | - | - | - | - | - | - | - | - | - | - | - | - | LCD_G5 | EVEN TOUT |
|  | PK1 | - | - | - | - | - | - | - | - | - | - | - | - | - | - | LCD_G6 | EVEN TOUT |
|  | PK2 | - | - | - | - | - | - | - | - | - | - | - | - | - | - | LCD_G7 | EVEN TOUT |
|  | PK3 | - | - | - | - | - | - | - | - | - | - | - | - | - | - | LCD_B4 | EVEN TOUT |
|  | PK4 | - | - | - | - | - | - | - | - | - | - | - | - | - | - | LCD_B5 | EVEN TOUT |
|  | PK5 | - | - | - | - | - | - | - | - | - | - | - | - | - | - | LCD_B6 | EVEN TOUT |
|  | PK6 | - | - | - | - | - | - | - | - | - | - | - | - | - | - | LCD_B7 | EVEN TOUT |
|  | PK7 | - | - | - | - | - | - | - | - | - | - | - | - | - | - | LCD_DE | EVEN TOUT |

DS9405
Rev
13
85/240
STM32F427xx
STM32F429xx
Pinouts
and
pin
description
Table 12. STM32F427xx and STM32F429xx alternate function mapping (continued)
AF0 AF1 AF2 AF3 AF4 AF5 AF6 AF7 AF8 AF9 AF10 AF11 AF12 AF13 AF14 AF15
Port SPI3/ USART6/ CAN1/2/ OTG2_HS
TIM8/9/ I2C1/ SPI1/2/ SPI2/3/ FMC/SDIO
SYS TIM1/2 TIM3/4/5 USART1/ UART4/5/7 TIM12/13/14 /OTG1_ ETH DCMI LCD SYS
10/11 2/3 3/4/5/6 SAI1 /OTG2_FS
2/3 /8 /LCD FS
EVEN
PJ8 - - - - - - - - - - - - - - LCD_G1
TOUT
EVEN
PJ9 - - - - - - - - - - - - - - LCD_G2
TOUT
EVEN
PJ10 - - - - - - - - - - - - - - LCD_G3
TOUT
EVEN
PJ11 - - - - - - - - - - - - - - LCD_G4
TOUT
Port J
EVEN
PJ12 - - - - - - - - - - - - - - LCD_B0
TOUT
EVEN
PJ13 - - - - - - - - - - - - - - LCD_B1
TOUT
EVEN
PJ14 - - - - - - - - - - - - - - LCD_B2
TOUT
EVEN
PJ15 - - - - - - - - - - - - - - LCD_B3
TOUT
EVEN
PK0 - - - - - - - - - - - - - - LCD_G5
TOUT
EVEN
PK1 - - - - - - - - - - - - - - LCD_G6
TOUT
EVEN
PK2 - - - - - - - - - - - - - - LCD_G7
TOUT
EVEN
PK3 - - - - - - - - - - - - - - LCD_B4 TOUT
Port K
EVEN
PK4 - - - - - - - - - - - - - - LCD_B5
TOUT
EVEN
PK5 - - - - - - - - - - - - - - LCD_B6
TOUT
EVEN
PK6 - - - - - - - - - - - - - - LCD_B7
TOUT
EVEN
PK7 - - - - - - - - - - - - - - LCD_DE
TOUT
1. The DCMI_VSYNC alternate function on PG9 is only available on silicon revision 3.

<!-- Page 86 -->


| Reserved 0xE010 0000 - 0xFFFF FFFF Cortex-M4 internal peripherals 0xE000 0000 - 0xE00F FFFF AHB3 0x6000 0000 - 0xDFFF FFFF Reserved 0x5006 0C00 - 0x5FFF FFFF 0x5006 0BFF AHB2 0xFFFF FFFF 512-Mbyte Block 7 Cortex-M4 0x5000 0000 Internal Reserved 0x4008 0000 - 0x4FFF FFFF peripherals Reserved 0xE010 0000 - 0xFFFF FFFF 0x4007 FFFF 0xE000 0000 Cortex-M4 internal 0xDFFF FFFF peripherals 0xE000 0000 - 0xE00F FFFF 512-Mbyte Block 6 AHB3 0x6000 0000 - 0xDFFF FFFF FMC 0xD000 0000 0x5006 0C00 - 0x5FFF FFFF 0xCFFF FFFF AHB1 512-Mbyte Block 5 FMC 0xA000 0000 0x9FFF FFFF 512-Mbyte 0x4002 0000 Block 4 Reserved 0x4001 6C00 - 0x4001 FFFF FMC bank 3 to 0x8000 0000 bank 4 0x4001 6BFF 0x7FFF FFFF 512-Mbyte Block 3 FMC bank 1 to bank 2 0x6000 0000 0x5FFF FFFF APB2 512-Mbyte Block 2 Peripherals 0x4000 0000 0x3FFF FFFF 512-Mbyte Block 1 SRAM Reserved 0x2003 0000 - 0x3FFF FFFF 0x4001 0000 0 0 x x 2 1 0 F 0 F 0 F 0 F 0 F 0 F 0 F 512-Mbyte S S R R A A B M M y b ( ( 1 6 it 6 4 -b K K a B B nd a a in l l i i g a a s s e e d d 0 0 x x 2 2 0 0 0 0 1 2 C 0 0 0 0 0 0 0 - - 0 0 x x 2 2 0 0 0 0 1 2 F F F F F F F F Reserved 0 0 x x 4 4 0 0 0 0 0 0 7 8 F 0 F 0 F 0 - 0x4000 FFFF Block 0 By bit-banding SRAM SRAM (112 KB aliased 0x2000 0000 - 0x2001 BFFF By bit-banding 0x0000 0000 Reserved 0x1FFF C010 - 0x1FFF FFFF Option Bytes 0x1FFF C000 - 0x1FFF C00F Reserved 0x1FFF 7A10 - 0x1FFF 7FFF System memory 0x1FFF 0000 - 0x1FFF 7A0F APB1 Reserved 0x1FFE C010 - 0x1FFE FFFF Option bytes 0x1FFE C000 - 0x1FFF C00F Reserved 0x1001 0000 - 0x1FFE BFFF CCM data RAM 0x1000 0000 - 0x1000 FFFF (64 KB data SRAM) Reserved 0x0820 0000 - 0x0FFF FFFF Flash memory 0x0800 0000 - 0x081F FFFF Reserved 0x0020 0000 - 0x07FF FFFF Aliased to Flash, system 0x4000 0000 memory or SRAM depending 0x0000 0000 - 0x001F FFFF on the BOOT pins MS30424V5 |
| --- |
|  |


| 512-Mbyte Block 7 Cortex-M4 Internal peripherals |
| --- |
| 512-Mbyte Block 6 FMC |
| 512-Mbyte Block 5 FMC |
| 512-Mbyte Block 4 FMC bank 3 to bank 4 |
| 512-Mbyte Block 3 FMC bank 1 to bank 2 |
| 512-Mbyte Block 2 Peripherals |
| 512-Mbyte Block 1 SRAM |
| 512-Mbyte Block 0 SRAM |


| Reserved |
| --- |
| SRAM (64 KB aliased By bit-banding |
| SRAM (16 KB aliased By bit-banding |
| SRAM (112 KB aliased By bit-banding |
| Reserved |
| Option Bytes |
| Reserved |
| System memory |
| Reserved |
| Option bytes |
| Reserved |
| CCM data RAM (64 KB data SRAM) |
| Reserved |
| Flash memory |
| Reserved |
| Aliased to Flash, system memory or SRAM depending on the BOOT pins |

Memory mapping STM32F427xx STM32F429xx
5 Memory mapping
The memory map is shown in Figure19.
Figure 19. Memory map
Reserved 0xE010 0000 - 0xFFFF FFFF
Cortex-M4 internal
peripherals 0xE000 0000 - 0xE00F FFFF
AHB3 0x6000 0000 - 0xDFFF FFFF
Reserved 0x5006 0C00 - 0x5FFF FFFF
0x5006 0BFF
AHB2
0xFFFF FFFF 512-Mbyte
Block 7
Cortex-M4 0x5000 0000
Internal Reserved 0x4008 0000 - 0x4FFF FFFF
peripherals Reserved 0xE010 0000 - 0xFFFF FFFF 0x4007 FFFF
0xE000 0000 Cortex-M4 internal
0xDFFF FFFF peripherals 0xE000 0000 - 0xE00F FFFF
512-Mbyte
Block 6 AHB3 0x6000 0000 - 0xDFFF FFFF
FMC
0xD000 0000 0x5006 0C00 - 0x5FFF FFFF
0xCFFF FFFF AHB1
512-Mbyte
Block 5
FMC
0xA000 0000
0x9FFF FFFF
512-Mbyte 0x4002 0000
Block 4 Reserved 0x4001 6C00 - 0x4001 FFFF
FMC bank 3 to
0x8000 0000 bank 4 0x4001 6BFF
0x7FFF FFFF
512-Mbyte
Block 3
FMC bank 1 to
bank 2
0x6000 0000
0x5FFF FFFF
APB2
512-Mbyte
Block 2
Peripherals
0x4000 0000
0x3FFF FFFF
512-Mbyte
Block 1
SRAM Reserved 0x2003 0000 - 0x3FFF FFFF
0x4001 0000
0 0 x x 2 1 0 F 0 F 0 F 0 F 0 F 0 F 0 F 512-Mbyte S S R R A A B M M y b ( ( 1 6 it 6 4 -b K K a B B nd a a in l l i i g a a s s e e d d 0 0 x x 2 2 0 0 0 0 1 2 C 0 0 0 0 0 0 0 - - 0 0 x x 2 2 0 0 0 0 1 2 F F F F F F F F Reserved 0 0 x x 4 4 0 0 0 0 0 0 7 8 F 0 F 0 F 0 - 0x4000 FFFF
Block 0 By bit-banding
SRAM SRAM (112 KB aliased 0x2000 0000 - 0x2001 BFFF
By bit-banding
0x0000 0000 Reserved 0x1FFF C010 - 0x1FFF FFFF
Option Bytes 0x1FFF C000 - 0x1FFF C00F
Reserved 0x1FFF 7A10 - 0x1FFF 7FFF
System memory 0x1FFF 0000 - 0x1FFF 7A0F APB1
Reserved 0x1FFE C010 - 0x1FFE FFFF
Option bytes 0x1FFE C000 - 0x1FFF C00F
Reserved 0x1001 0000 - 0x1FFE BFFF
CCM data RAM 0x1000 0000 - 0x1000 FFFF
(64 KB data SRAM)
Reserved 0x0820 0000 - 0x0FFF FFFF
Flash memory 0x0800 0000 - 0x081F FFFF
Reserved 0x0020 0000 - 0x07FF FFFF
Aliased to Flash, system 0x4000 0000
memory or SRAM depending 0x0000 0000 - 0x001F FFFF
on the BOOT pins
MS30424V5
86/240 DS9405 Rev 13

<!-- Page 87 -->


| Bus | Boundary address | Peripheral |
| --- | --- | --- |
|  | 0xE00F FFFF - 0xFFFF FFFF | Reserved |
| Cortex-M4 | 0xE000 0000 - 0xE00F FFFF | Cortex-M4 internal peripherals |
| AHB3 | 0xD000 0000 - 0xDFFF FFFF | FMC bank 6 |
|  | 0xC000 0000 - 0xCFFF FFFF | FMC bank 5 |
|  | 0xA000 1000 - 0xBFFF FFFF | Reserved |
|  | 0xA000 0000- 0xA000 0FFF | FMC control register |
|  | 0x9000 0000 - 0x9FFF FFFF | FMC bank 4 |
|  | 0x8000 0000 - 0x8FFF FFFF | FMC bank 3 |
|  | 0x7000 0000 - 0x7FFF FFFF | FMC bank 2 |
|  | 0x6000 0000 - 0x6FFF FFFF | FMC bank 1 |
|  | 0x5006 0C00- 0x5FFF FFFF | Reserved |
| AHB2 | 0x5006 0800 - 0X5006 0BFF | RNG |
|  | 0x5005 0400 - X5006 07FF | Reserved |
|  | 0x5005 0000 - 0X5005 03FF | DCMI |
|  | 0x5004 0000- 0x5004 FFFF | Reserved |
|  | 0x5000 0000 - 0X5003 FFFF | USB OTG FS |

STM32F427xx STM32F429xx Memory mapping
Table 13. STM32F427xx and STM32F429xx register boundary addresses
Bus Boundary address Peripheral
0xE00F FFFF - 0xFFFF FFFF Reserved
Cortex-M4 0xE000 0000 - 0xE00F FFFF Cortex-M4 internal peripherals
0xD000 0000 - 0xDFFF FFFF FMC bank 6
0xC000 0000 - 0xCFFF FFFF FMC bank 5
0xA000 1000 - 0xBFFF FFFF Reserved
0xA000 0000- 0xA000 0FFF FMC control register
AHB3
0x9000 0000 - 0x9FFF FFFF FMC bank 4
0x8000 0000 - 0x8FFF FFFF FMC bank 3
0x7000 0000 - 0x7FFF FFFF FMC bank 2
0x6000 0000 - 0x6FFF FFFF FMC bank 1
0x5006 0C00- 0x5FFF FFFF Reserved
0x5006 0800 - 0X5006 0BFF RNG
0x5005 0400 - X5006 07FF Reserved
AHB2 0x5005 0000 - 0X5005 03FF DCMI
0x5004 0000- 0x5004 FFFF Reserved
0x5000 0000 - 0X5003 FFFF USB OTG FS
DS9405 Rev 13 87/240
90

<!-- Page 88 -->


| Bus | Boundary address | Peripheral |
| --- | --- | --- |
|  | 0x4008 0000- 0x4FFF FFFF | Reserved |
| AHB1 | 0x4004 0000 - 0x4007 FFFF | USB OTG HS |
|  | 0x4002 BC00- 0x4003 FFFF | Reserved |
|  | 0x4002 B000 - 0x4002 BBFF | DMA2D |
|  | 0x4002 9400 - 0x4002 AFFF | Reserved |
|  | 0x4002 9000 - 0x4002 93FF | ETHERNET MAC |
|  | 0x4002 8C00 - 0x4002 8FFF |  |
|  | 0x4002 8800 - 0x4002 8BFF |  |
|  | 0x4002 8400 - 0x4002 87FF |  |
|  | 0x4002 8000 - 0x4002 83FF |  |
|  | 0x4002 6800 - 0x4002 7FFF | Reserved |
|  | 0x4002 6400 - 0x4002 67FF | DMA2 |
|  | 0x4002 6000 - 0x4002 63FF | DMA1 |
|  | 0X4002 5000 - 0X4002 5FFF | Reserved |
|  | 0x4002 4000 - 0x4002 4FFF | BKPSRAM |
|  | 0x4002 3C00 - 0x4002 3FFF | Flash interface register |
|  | 0x4002 3800 - 0x4002 3BFF | RCC |
|  | 0X4002 3400 - 0X4002 37FF | Reserved |
|  | 0x4002 3000 - 0x4002 33FF | CRC |
|  | 0x4002 2C00 - 0x4002 2FFF | Reserved |
|  | 0x4002 2800 - 0x4002 2BFF | GPIOK |
|  | 0x4002 2400 - 0x4002 27FF | GPIOJ |
|  | 0x4002 2000 - 0x4002 23FF | GPIOI |
|  | 0x4002 1C00 - 0x4002 1FFF | GPIOH |
|  | 0x4002 1800 - 0x4002 1BFF | GPIOG |
|  | 0x4002 1400 - 0x4002 17FF | GPIOF |
|  | 0x4002 1000 - 0x4002 13FF | GPIOE |
|  | 0X4002 0C00 - 0x4002 0FFF | GPIOD |
|  | 0x4002 0800 - 0x4002 0BFF | GPIOC |
|  | 0x4002 0400 - 0x4002 07FF | GPIOB |
|  | 0x4002 0000 - 0x4002 03FF | GPIOA |

Memory mapping STM32F427xx STM32F429xx
Table 13. STM32F427xx and STM32F429xx register boundary addresses (continued)
Bus Boundary address Peripheral
0x4008 0000- 0x4FFF FFFF Reserved
0x4004 0000 - 0x4007 FFFF USB OTG HS
0x4002 BC00- 0x4003 FFFF Reserved
0x4002 B000 - 0x4002 BBFF DMA2D
0x4002 9400 - 0x4002 AFFF Reserved
0x4002 9000 - 0x4002 93FF
0x4002 8C00 - 0x4002 8FFF
0x4002 8800 - 0x4002 8BFF ETHERNET MAC
0x4002 8400 - 0x4002 87FF
0x4002 8000 - 0x4002 83FF
0x4002 6800 - 0x4002 7FFF Reserved
0x4002 6400 - 0x4002 67FF DMA2
0x4002 6000 - 0x4002 63FF DMA1
0X4002 5000 - 0X4002 5FFF Reserved
0x4002 4000 - 0x4002 4FFF BKPSRAM
0x4002 3C00 - 0x4002 3FFF Flash interface register
AHB1
0x4002 3800 - 0x4002 3BFF RCC
0X4002 3400 - 0X4002 37FF Reserved
0x4002 3000 - 0x4002 33FF CRC
0x4002 2C00 - 0x4002 2FFF Reserved
0x4002 2800 - 0x4002 2BFF GPIOK
0x4002 2400 - 0x4002 27FF GPIOJ
0x4002 2000 - 0x4002 23FF GPIOI
0x4002 1C00 - 0x4002 1FFF GPIOH
0x4002 1800 - 0x4002 1BFF GPIOG
0x4002 1400 - 0x4002 17FF GPIOF
0x4002 1000 - 0x4002 13FF GPIOE
0X4002 0C00 - 0x4002 0FFF GPIOD
0x4002 0800 - 0x4002 0BFF GPIOC
0x4002 0400 - 0x4002 07FF GPIOB
0x4002 0000 - 0x4002 03FF GPIOA
88/240 DS9405 Rev 13

<!-- Page 89 -->


| Bus | Boundary address | Peripheral |
| --- | --- | --- |
|  | 0x4001 6C00- 0x4001 FFFF | Reserved |
| APB2 | 0x4001 6800 - 0x4001 6BFF | LCD-TFT |
|  | 0x4001 5C00 - 0x4001 67FF | Reserved |
|  | 0x4001 5800 - 0x4001 5BFF | SAI1 |
|  | 0x4001 5400 - 0x4001 57FF | SPI6 |
|  | 0x4001 5000 - 0x4001 53FF | SPI5 |
|  | 0x4001 5400 - 0x4001 57FF | SPI6 |
|  | 0x4001 5000 - 0x4001 53FF | SPI5 |
|  | 0x4001 4C00 - 0x4001 4FFF | Reserved |
|  | 0x4001 4800 - 0x4001 4BFF | TIM11 |
|  | 0x4001 4400 - 0x4001 47FF | TIM10 |
|  | 0x4001 4000 - 0x4001 43FF | TIM9 |
|  | 0x4001 3C00 - 0x4001 3FFF | EXTI |
|  | 0x4001 3800 - 0x4001 3BFF | SYSCFG |
|  | 0x4001 3400 - 0x4001 37FF | SPI4 |
|  | 0x4001 3000 - 0x4001 33FF | SPI1 |
|  | 0x4001 2C00 - 0x4001 2FFF | SDIO |
|  | 0x4001 2400 - 0x4001 2BFF | Reserved |
|  | 0x4001 2000 - 0x4001 23FF | ADC1 - ADC2 - ADC3 |
|  | 0x4001 1800 - 0x4001 1FFF | Reserved |
|  | 0x4001 1400 - 0x4001 17FF | USART6 |
|  | 0x4001 1000 - 0x4001 13FF | USART1 |
|  | 0x4001 0800 - 0x4001 0FFF | Reserved |
|  | 0x4001 0400 - 0x4001 07FF | TIM8 |
|  | 0x4001 0000 - 0x4001 03FF | TIM1 |

STM32F427xx STM32F429xx Memory mapping
Table 13. STM32F427xx and STM32F429xx register boundary addresses (continued)
Bus Boundary address Peripheral
0x4001 6C00- 0x4001 FFFF Reserved
0x4001 6800 - 0x4001 6BFF LCD-TFT
0x4001 5C00 - 0x4001 67FF Reserved
0x4001 5800 - 0x4001 5BFF SAI1
0x4001 5400 - 0x4001 57FF SPI6
0x4001 5000 - 0x4001 53FF SPI5
0x4001 5400 - 0x4001 57FF SPI6
0x4001 5000 - 0x4001 53FF SPI5
0x4001 4C00 - 0x4001 4FFF Reserved
0x4001 4800 - 0x4001 4BFF TIM11
0x4001 4400 - 0x4001 47FF TIM10
0x4001 4000 - 0x4001 43FF TIM9
0x4001 3C00 - 0x4001 3FFF EXTI
APB2
0x4001 3800 - 0x4001 3BFF SYSCFG
0x4001 3400 - 0x4001 37FF SPI4
0x4001 3000 - 0x4001 33FF SPI1
0x4001 2C00 - 0x4001 2FFF SDIO
0x4001 2400 - 0x4001 2BFF Reserved
0x4001 2000 - 0x4001 23FF ADC1 - ADC2 - ADC3
0x4001 1800 - 0x4001 1FFF Reserved
0x4001 1400 - 0x4001 17FF USART6
0x4001 1000 - 0x4001 13FF USART1
0x4001 0800 - 0x4001 0FFF Reserved
0x4001 0400 - 0x4001 07FF TIM8
0x4001 0000 - 0x4001 03FF TIM1
DS9405 Rev 13 89/240
90

<!-- Page 90 -->


| Bus | Boundary address | Peripheral |
| --- | --- | --- |
|  | 0x4000 8000- 0x4000 FFFF | Reserved |
| APB1 | 0x4000 7C00 - 0x4000 7FFF | UART8 |
|  | 0x4000 7800 - 0x4000 7BFF | UART7 |
|  | 0x4000 7400 - 0x4000 77FF | DAC |
|  | 0x4000 7000 - 0x4000 73FF | PWR |
|  | 0x4000 6C00 - 0x4000 6FFF | Reserved |
|  | 0x4000 6800 - 0x4000 6BFF | CAN2 |
|  | 0x4000 6400 - 0x4000 67FF | CAN1 |
|  | 0x4000 6000 - 0x4000 63FF | Reserved |
|  | 0x4000 5C00 - 0x4000 5FFF | I2C3 |
|  | 0x4000 5800 - 0x4000 5BFF | I2C2 |
|  | 0x4000 5400 - 0x4000 57FF | I2C1 |
|  | 0x4000 5000 - 0x4000 53FF | UART5 |
|  | 0x4000 4C00 - 0x4000 4FFF | UART4 |
|  | 0x4000 4800 - 0x4000 4BFF | USART3 |
|  | 0x4000 4400 - 0x4000 47FF | USART2 |
|  | 0x4000 4000 - 0x4000 43FF | I2S3ext |
|  | 0x4000 3C00 - 0x4000 3FFF | SPI3 / I2S3 |
|  | 0x4000 3800 - 0x4000 3BFF | SPI2 / I2S2 |
|  | 0x4000 3400 - 0x4000 37FF | I2S2ext |
|  | 0x4000 3000 - 0x4000 33FF | IWDG |
|  | 0x4000 2C00 - 0x4000 2FFF | WWDG |
|  | 0x4000 2800 - 0x4000 2BFF | RTC & BKP Registers |
|  | 0x4000 2400 - 0x4000 27FF | Reserved |
|  | 0x4000 2000 - 0x4000 23FF | TIM14 |
|  | 0x4000 1C00 - 0x4000 1FFF | TIM13 |
|  | 0x4000 1800 - 0x4000 1BFF | TIM12 |
|  | 0x4000 1400 - 0x4000 17FF | TIM7 |
|  | 0x4000 1000 - 0x4000 13FF | TIM6 |
|  | 0x4000 0C00 - 0x4000 0FFF | TIM5 |
|  | 0x4000 0800 - 0x4000 0BFF | TIM4 |
|  | 0x4000 0400 - 0x4000 07FF | TIM3 |
|  | 0x4000 0000 - 0x4000 03FF | TIM2 |

Memory mapping STM32F427xx STM32F429xx
Table 13. STM32F427xx and STM32F429xx register boundary addresses (continued)
Bus Boundary address Peripheral
0x4000 8000- 0x4000 FFFF Reserved
0x4000 7C00 - 0x4000 7FFF UART8
0x4000 7800 - 0x4000 7BFF UART7
0x4000 7400 - 0x4000 77FF DAC
0x4000 7000 - 0x4000 73FF PWR
0x4000 6C00 - 0x4000 6FFF Reserved
0x4000 6800 - 0x4000 6BFF CAN2
0x4000 6400 - 0x4000 67FF CAN1
0x4000 6000 - 0x4000 63FF Reserved
0x4000 5C00 - 0x4000 5FFF I2C3
0x4000 5800 - 0x4000 5BFF I2C2
0x4000 5400 - 0x4000 57FF I2C1
0x4000 5000 - 0x4000 53FF UART5
0x4000 4C00 - 0x4000 4FFF UART4
0x4000 4800 - 0x4000 4BFF USART3
0x4000 4400 - 0x4000 47FF USART2
0x4000 4000 - 0x4000 43FF I2S3ext
APB1
0x4000 3C00 - 0x4000 3FFF SPI3 / I2S3
0x4000 3800 - 0x4000 3BFF SPI2 / I2S2
0x4000 3400 - 0x4000 37FF I2S2ext
0x4000 3000 - 0x4000 33FF IWDG
0x4000 2C00 - 0x4000 2FFF WWDG
0x4000 2800 - 0x4000 2BFF RTC & BKP Registers
0x4000 2400 - 0x4000 27FF Reserved
0x4000 2000 - 0x4000 23FF TIM14
0x4000 1C00 - 0x4000 1FFF TIM13
0x4000 1800 - 0x4000 1BFF TIM12
0x4000 1400 - 0x4000 17FF TIM7
0x4000 1000 - 0x4000 13FF TIM6
0x4000 0C00 - 0x4000 0FFF TIM5
0x4000 0800 - 0x4000 0BFF TIM4
0x4000 0400 - 0x4000 07FF TIM3
0x4000 0000 - 0x4000 03FF TIM2
90/240 DS9405 Rev 13

<!-- Page 91 -->


| Figure 20. Pin loading conditions MCU pin C = 50 pF MS19011V2 | Figure 21. Pin input voltage MCU pin VIN MS19010V2 |
| --- | --- |


|  |  |
| --- | --- |
|  |  |

STM32F427xx STM32F429xx Electrical characteristics
6 Electrical characteristics
6.1 Parameter conditions
Unless otherwise specified, all voltages are referenced to V .
SS
6.1.1 Minimum and maximum values
Unless otherwise specified the minimum and maximum values are guaranteed in the worst
conditions of ambient temperature, supply voltage and frequencies by tests in production on
100% of the devices with an ambient temperature at T = 25 °C and T = T max (given by
A A A
the selected temperature range).
Data based on characterization results, design simulation and/or technology characteristics
are indicated in the table footnotes and are not tested in production. Based on
characterization, the minimum and maximum values refer to sample tests and represent the
mean value plus or minus three times the standard deviation (mean±3σ).
6.1.2 Typical values
Unless otherwise specified, typical data are based on T = 25 °C, V = 3.3 V (for the
A DD
1.7V≤V ≤ 3.6V voltage range). They are given only as design guidelines and are not
DD
tested.
Typical ADC accuracy values are determined by characterization of a batch of samples from
a standard diffusion lot over the full temperature range, where 95% of the devices have an
error less than or equal to the value indicated (mean±2σ).
6.1.3 Typical curves
Unless otherwise specified, all typical curves are given only as design guidelines and are
not tested.
6.1.4 Loading capacitor
The loading conditions used for pin parameter measurement are shown in Figure20.
6.1.5 Pin input voltage
The input voltage measurement on a pin of the device is described in Figure21.
Figure 20. Pin loading conditions Figure 21. Pin input voltage
MCU pin MCU pin
C = 50 pF VIN
MS19011V2 MS19010V2
DS9405 Rev 13 91/240
197

<!-- Page 92 -->


| retfihs leveL | IO Logic |
| --- | --- |

Electrical characteristics STM32F427xx STM32F429xx
6.1.6 Power supply scheme
Figure 22. Power supply scheme
VBAT
Backup circuitry
VBAT = Power (OSC32K,RTC,
1.65 to 3.6V switch Wakeup logic
Backup registers,
backup RAM)
OUT
GPIOs
IN
Kernel logic
(CPU, digital
& RAM)
Voltage
15 × 100 nF regulator
+ 1 × 4.7 μF
VDDA
Analog:
ADC RCs,
PLL,..
MS19911V3
1. To connect BYPASS_REG and PDR_ON pins, refer to Section3.17: Power supply supervisor and Section3.18: Voltage
regulator
2. The two 2.2µF ceramic capacitors should be replaced by two 100nF decoupling capacitors when the voltage regulator is
OFF.
3. The 4.7µF ceramic capacitor must be connected to one of the VDD pins.
4. V =V and V =V .
DDA DD SSA SS
Caution: Each power supply pair (V /V , V /V ...) must be decoupled with filtering ceramic
DD SS DDA SSA
capacitors as shown above. These capacitors must be placed as close as possible to, or
below, the appropriate pins on the underside of the PCB to ensure good operation of the
device. It is not recommended to remove filtering capacitors to reduce PCB size or cost.
This might cause incorrect operation of the device.
92/240 DS9405 Rev 13
retfihs
leveL
IO
Logic
VCAP_1
2 × 2.2 μF VCAP_2
VDD VDD
1/2/...14/15
VSS
1/2/...14/15
Flash memory
BYPASS_REG
Reset
PDR_ON controller
VDD
VREF
VREF+
100 nF 100 nF VREF-
+ 1 μF + 1 μF
VSSA

<!-- Page 93 -->


|  |  |
| --- | --- |


|  |  |
| --- | --- |


|  |  |
| --- | --- |


| Symbol | Ratings | Min | Max | Unit |
| --- | --- | --- | --- | --- |
| V –V DD SS | External main supply voltage (including V ,V DDA DD andVBAT)(1) | −0.3 | 4.0 | V |
| V IN | Input voltage on FT pins(2) | V −0.3 SS | V + 4.0 DD |  |
|  | Input voltage on TTa pins | V −0.3 SS | 4.0 |  |
|  | Input voltage on any other pin | V −0.3 SS | 4.0 |  |
|  | Input voltage on BOOT0 pin | V SS | 9.0 |  |
| |∆V | DDx | Variations between different V power pins DD | - | 50 | mV |
| |V -V | SSX SS | Variations between all the different ground pins including V REF- | - | 50 |  |
| V ESD(HBM) | Electrostatic discharge voltage (human body model) | see Section6.3.15: Absolute maximum ratings (electrical sensitivity) |  |  |

STM32F427xx STM32F429xx Electrical characteristics
6.1.7 Current consumption measurement
Figure 23. Current consumption measurement scheme
IDD_VBAT
VBAT
IDD
VDD
VDDA
ai14126
6.2 Absolute maximum ratings
Stresses above the absolute maximum ratings listed in Table14: Voltage characteristics,
Table15: Current characteristics, and Table16: Thermal characteristics may cause
permanent damage to the device. These are stress ratings only and functional operation of
the device at these conditions is not implied. Exposure to maximum rating conditions for
extended periods may affect device reliability.
Device mission profile (application conditions) is compliant with JEDEC JESD47
Qualification Standard, extended mission profiles are available on demand.
Table 14. Voltage characteristics
Symbol Ratings Min Max Unit
External main supply voltage (including V ,V
V
DD
–V
SS andVBAT)(1)
DDA DD −0.3 4.0
Input voltage on FT pins(2) V −0.3 V + 4.0
SS DD
V
Input voltage on TTa pins V −0.3 4.0
SS
V
IN
Input voltage on any other pin V −0.3 4.0
SS
Input voltage on BOOT0 pin V 9.0
SS
|∆V | Variations between different V power pins - 50
DDx DD
mV
Variations between all the different ground pins
|V -V | - 50
SSX SS including V
REF-
see Section6.3.15:
Absolute maximum
V Electrostatic discharge voltage (human body model)
ESD(HBM) ratings (electrical
sensitivity)
1. All main power (V , V ) and ground (V , V ) pins must always be connected to the external
DD DDA SS SSA
power supply, in the permitted range.
2. V maximum value must always be respected. Refer to Table15 for the values of the maximum allowed
IN
injected current.
DS9405 Rev 13 93/240
197

<!-- Page 94 -->


| Symbol | Ratings | Max. | Unit |
| --- | --- | --- | --- |
| ∑I VDD | Total current into sum of all V power lines (source)(1) DD_x | 270 | mA |
| ∑I VSS | Total current out of sum of all V ground lines (sink)(1) SS_x | −270 |  |
| I VDD | Maximum current into each V power line (source)(1) DD_x | 100 |  |
| I VSS | Maximum current out of each V ground line (sink)(1) SS_x | −100 |  |
| I IO | Output current sunk by any I/O and control pin | 25 |  |
|  | Output current sourced by any I/Os and control pin | −25 |  |
| ∑I IO | Total output current sunk by sum of all I/O and control pins (2) | 120 |  |
|  | Total output current sourced by sum of all I/Os and control pins(2) | −120 |  |
| I (3) INJ(PIN) | Injected current on FT pins (4) | −5/+0 |  |
|  | Injected current on NRST and BOOT0 pins (4) |  |  |
|  | Injected current on TTa pins(5) | ±5 |  |
| ∑I (5) INJ(PIN) | Total injected current (sum of all I/O and control pins)(6) | ±25 |  |


| Symbol | Ratings | Value | Unit |
| --- | --- | --- | --- |
| T STG | Storage temperature range | −65 to +150 | °C |
| T J | Maximum junction temperature | 125 | °C |

Electrical characteristics STM32F427xx STM32F429xx
Table 15. Current characteristics
Symbol Ratings Max. Unit
∑I Total current into sum of all V power lines (source)(1) 270
VDD DD_x
∑I Total current out of sum of all V ground lines (sink)(1) −270
VSS SS_x
I Maximum current into each V power line (source)(1) 100
VDD DD_x
I Maximum current out of each V ground line (sink)(1) −100
VSS SS_x
Output current sunk by any I/O and control pin 25
I
IO
Output current sourced by any I/Os and control pin −25
mA
Total output current sunk by sum of all I/O and control pins (2) 120
∑I
IO Total output current sourced by sum of all I/Os and control pins(2) −120
Injected current on FT pins (4)
−5/+0
I (3) Injected current on NRST and BOOT0 pins (4)
INJ(PIN)
Injected current on TTa pins(5) ±5
∑I (5) Total injected current (sum of all I/O and control pins)(6) ±25
INJ(PIN)
1. All main power (V , V ) and ground (V , V ) pins must always be connected to the external power supply, in the
DD DDA SS SSA
permitted range.
2. This current consumption must be correctly distributed over all I/Os and control pins. The total output current must not be
sunk/sourced between two consecutive power supply pins referring to high pin count LQFP packages.
3. Negative injection disturbs the analog performance of the device. See note in Section6.3.21: 12-bit ADC characteristics.
4. Positive injection is not possible on these I/Os and does not occur for input voltages lower than the specified maximum
value.
5. A positive injection is induced by V >V while a negative injection is induced by V <V . I must never be
IN DDA IN SS INJ(PIN)
exceeded. Refer to Table14 for the values of the maximum allowed input voltage.
6. When several inputs are submitted to a current injection, the maximum ΣI is the absolute sum of the positive and
INJ(PIN)
negative injected currents (instantaneous values).
Table 16. Thermal characteristics
Symbol Ratings Value Unit
T Storage temperature range −65 to +150 °C
STG
T Maximum junction temperature 125 °C
J
94/240 DS9405 Rev 13

<!-- Page 95 -->


| Symbol | Parameter | Conditions(1) |  | Min | Typ | Max | Unit |
| --- | --- | --- | --- | --- | --- | --- | --- |
| f HCLK | Internal AHB clock frequency | Power Scale 3 (VOS[1:0] bits in PWR_CR register = 0x01), Regulator ON, over-drive OFF |  | 0 | - | 120 | MHz |
|  |  | Power Scale 2 (VOS[1:0] bits in PWR_CR register = 0x10), Regulator ON | Over- drive OFF | 0 | - | 144 |  |
|  |  |  | Over- drive ON |  | - | 168 |  |
|  |  | Power Scale 1 (VOS[1:0] bits in PWR_CR register= 0x11), Regulator ON | Over- drive OFF | 0 | - | 168 |  |
|  |  |  | Over- drive ON |  | - | 180 |  |
| f PCLK1 | Internal APB1 clock frequency | Over-drive OFF |  | 0 | - | 42 |  |
|  |  | Over-drive ON |  | 0 | - | 45 |  |
| f PCLK2 | Internal APB2 clock frequency | Over-drive OFF |  | 0 | - | 84 |  |
|  |  | Over-drive ON |  | 0 | - | 90 |  |
| V DD | Standard operating voltage |  |  | 1.7(2) | - | 3.6 | V |
| V DDA (3)(4) | Analog operating voltage (ADC limited to 1.2 M samples) | Must be the same potential as V (5) DD |  | 1.7(2) | - | 2.4 |  |
|  | Analog operating voltage (ADC limited to 2.4 M samples) |  |  | 2.4 | - | 3.6 |  |
| V BAT | Backup operating voltage |  |  | 1.65 | - | 3.6 |  |
| V 12 | Regulator ON: 1.2V internal voltage on V /V pins CAP_1 CAP_2 | Power Scale 3 ((VOS[1:0] bits in PWR_CR register = 0x01), 120MHz HCLK max frequency |  | 1.08 | 1.14 | 1.20 | V |
|  |  | Power Scale 2 ((VOS[1:0] bits in PWR_CR register = 0x10), 144MHz HCLK max frequency with over-drive OFF or 168MHz with over-drive ON |  | 1.20 | 1.26 | 1.32 |  |
|  |  | Power Scale 1 ((VOS[1:0] bits in PWR_CR register = 0x11), 168MHz HCLK max frequency with over-drive OFF or 180MHz with over-drive ON |  | 1.26 | 1.32 | 1.40 |  |
|  | Regulator OFF: 1.2V external voltage must be supplied from external regulator on V /V pins(6) CAP_1 CAP_2 | Max frequency 120MHz |  | 1.10 | 1.14 | 1.20 |  |
|  |  | Max frequency 144MHz |  | 1.20 | 1.26 | 1.32 |  |
|  |  | Max frequency 168MHz |  | 1.26 | 1.32 | 1.38 |  |

STM32F427xx STM32F429xx Electrical characteristics
6.3 Operating conditions
6.3.1 General operating conditions
Table 17. General operating conditions
Symbol Parameter Conditions(1) Min Typ Max Unit
Power Scale 3 (VOS[1:0] bits in
PWR_CR register = 0x01), Regulator 0 - 120
ON, over-drive OFF
Over-
Power Scale 2 (VOS[1:0] bits drive OFF - 144
in PWR_CR register = 0x10), 0
f HCLK Internal AHB clock frequency Regulator ON Over- - 168
drive ON
Over-
Power Scale 1 (VOS[1:0] bits drive OFF - 168 MHz
in PWR_CR register= 0x11), 0
Regulator ON Over-
- 180
drive ON
Over-drive OFF 0 - 42
f Internal APB1 clock frequency
PCLK1
Over-drive ON 0 - 45
Over-drive OFF 0 - 84
f Internal APB2 clock frequency
PCLK2
Over-drive ON 0 - 90
V Standard operating voltage 1.7(2) - 3.6
DD
Analog operating voltage
1.7(2) - 2.4
V (ADC limited to 1.2 M samples)
DDA Must be the same potential as V (5) V
(3)(4) DD
Analog operating voltage
2.4 - 3.6
(ADC limited to 2.4 M samples)
V Backup operating voltage 1.65 - 3.6
BAT
Power Scale 3 ((VOS[1:0] bits in
PWR_CR register = 0x01), 120MHz 1.08 1.14 1.20
HCLK max frequency
Power Scale 2 ((VOS[1:0] bits in
PWR_CR register = 0x10), 144MHz
Regulator ON: 1.2V internal 1.20 1.26 1.32
HCLK max frequency with over-drive
voltage on V /V pins
CAP_1 CAP_2 OFF or 168MHz with over-drive ON
V 12 Power Scale 1 ((VOS[1:0] bits in V
PWR_CR register = 0x11), 168MHz
1.26 1.32 1.40
HCLK max frequency with over-drive
OFF or 180MHz with over-drive ON
Regulator OFF: 1.2V external Max frequency 120MHz 1.10 1.14 1.20
voltage must be supplied from
Max frequency 144MHz 1.20 1.26 1.32
external regulator on
V /V pins(6) Max frequency 168MHz 1.26 1.32 1.38
CAP_1 CAP_2
DS9405 Rev 13 95/240
197

<!-- Page 96 -->


| Symbol | Parameter | Conditions(1) | Min | Typ | Max | Unit |
| --- | --- | --- | --- | --- | --- | --- |
| V IN | Input voltage on RST and FT pins(7) | 2V ≤ V ≤ 3.6V DD | −0.3 | - | 5.5 | V |
|  |  | V ≤ 2V DD | −0.3 | - | 5.2 |  |
|  | Input voltage on TTa pins |  | −0.3 | - | V + DDA 0.3 |  |
|  | Input voltage on BOOT0 pin |  | 0 | - | 9 |  |
| P D | Power dissipation at T = 85°C A for suffix 6 or T = 105°C for A suffix 7(8) | LQFP100 | - | - | 465 | mW |
|  |  | WLCSP143 | - | - | 641 |  |
|  |  | LQFP144 | - | - | 500 |  |
|  |  | UFBGA169 | - | - | 385 |  |
|  |  | LQFP176 | - | - | 526 |  |
|  |  | UFBGA176 | - | - | 513 |  |
|  |  | LQFP208 | - | - | 1053 |  |
|  |  | TFBGA216 | - | - | 690 |  |
| TA | Ambient temperature for 6 suffix version | Maximum power dissipation | −40 | - | 85 | °C |
|  |  | Low power dissipation(9) | −40 | - | 105 |  |
|  | Ambient temperature for 7 suffix version | Maximum power dissipation | −40 | - | 105 | °C |
|  |  | Low power dissipation(9) | −40 | - | 125 |  |
| TJ | Junction temperature range | 6 suffix version | −40 | - | 105 | °C |
|  |  | 7 suffix version | −40 | - | 125 |  |

Electrical characteristics STM32F427xx STM32F429xx
Table 17. General operating conditions (continued)
Symbol Parameter Conditions(1) Min Typ Max Unit
Input voltage on RST and FT 2V ≤ V DD ≤ 3.6V −0.3 - 5.5
pins(7)
V ≤ 2V −0.3 - 5.2
DD
V IN Input voltage on TTa pins −0.3 - V DDA + V
0.3
Input voltage on BOOT0 pin 0 - 9
LQFP100 - - 465
WLCSP143 - - 641
LQFP144 - - 500
Power dissipation at T A = 85°C UFBGA169 - - 385
P for suffix 6 or T = 105°C for mW
D A
suffix 7(8) LQFP176 - - 526
UFBGA176 - - 513
LQFP208 - - 1053
TFBGA216 - - 690
Ambient temperature for 6 suffix Maximum power dissipation −40 - 85
°C
version Low power dissipation(9) −40 - 105
TA
Ambient temperature for 7 suffix Maximum power dissipation −40 - 105
°C
version Low power dissipation(9) −40 - 125
6 suffix version −40 - 105
TJ Junction temperature range °C
7 suffix version −40 - 125
1. The overdrive mode is not supported at the voltage ranges from 1.7 to 2.1V.
2. V /V minimum value of 1.7V is obtained with the use of an external power supply supervisor (refer to Section3.17.2:
DD DDA
Internal reset OFF).
3. When the ADC is used, refer to Table75: ADC characteristics.
4. If a V pin is present, it must respect the following condition: V -V < 1.2V.
REF+ DDA REF+
5. It is recommended to power V and V from the same source. A maximum difference of 300mV between V and V
DD DDA DD DDA
can be tolerated during power-up and power-down operation.
6. The overdrive mode is not supported when the internal regulator is OFF.
7. To sustain a voltage higher than VDD+0.3, the internal pull-up and pull-down resistors must be disabled
8. If T is lower, higher P values are allowed as long as T does not exceed T .
A D J Jmax
9. In low-power dissipation state, T can be extended to this range as long as T does not exceed T .
A J Jmax
96/240 DS9405 Rev 13

<!-- Page 97 -->


| Operating power supply range | ADC operation | Maximum Flash memory access frequency with no wait states (f ) Flashmax | Maximum HCLK frequency vs Flash memory wait states (1)(2) | I/O operation | Possible Flash memory operations |
| --- | --- | --- | --- | --- | --- |
| V =1.7 to DD 2.1V(3) | Conversion time up to 1.2Msps | 20MHz(4) | 168MHz with 8 wait states and over-drive OFF | No I/O compensation | 8-bit erase and program operations only |
| V = 2.1 to DD 2.4V | Conversion time up to 1.2Msps | 22MHz | 180MHz with 8 wait states and over-drive ON | No I/O compensation | 16-bit erase and program operations |
| V = 2.4 to DD 2.7V | Conversion time up to 2.4Msps | 24MHz | 180MHz with 7 wait states and over-drive ON | I/O compensation works | 16-bit erase and program operations |
| V = 2.7 to DD 3.6V(5) | Conversion time up to 2.4Msps | 30MHz | 180MHz with 5 wait states and over-drive ON | I/O compensation works | 32-bit erase and program operations |


|  |
| --- |
|  |


|  |
| --- |
|  |


|  |
| --- |
|  |


| Symbol | Parameter | Conditions |
| --- | --- | --- |
| CEXT | Capacitance of external capacitor | 2.2 µF |
| ESR | ESR of external capacitor | < 2 Ω |

STM32F427xx STM32F429xx Electrical characteristics
Table 18. Limitations depending on the operating power supply range
Maximum Flash
Maximum HCLK
Operating memory access Possible Flash
frequency vs Flash
power supply ADC operation frequency with I/O operation memory
memory wait states
range no wait states operations
(1)(2)
(f )
Flashmax
168MHz with 8 wait 8-bit erase and
V =1.7 to Conversion time No I/O
DD 20MHz(4) states and over-drive program
2.1V(3) up to 1.2Msps compensation
OFF operations only
180MHz with 8 wait 16-bit erase and
V = 2.1 to Conversion time No I/O
DD 22MHz states and over-drive program
2.4V up to 1.2Msps compensation
ON operations
180MHz with 7 wait 16-bit erase and
V = 2.4 to Conversion time I/O compensation
DD 24MHz states and over-drive program
2.7V up to 2.4Msps works
ON operations
180MHz with 5 wait 32-bit erase and
V = 2.7 to Conversion time I/O compensation
DD 30MHz states and over-drive program
3.6V(5) up to 2.4Msps works
ON operations
1. Applicable only when the code is executed from flash memory. When the code is executed from RAM, no wait state is
required.
2. Thanks to the ART accelerator and the 128-bit flash memory, the number of wait states given here does not impact the
execution speed from flash memory since the ART accelerator allows to achieve a performance equivalent to 0 wait state
program execution.
3. The V V minimum value of 1.7V is obtained with the use of an external power supply supervisor (refer to
DD/ DDA
Section3.17.2: Internal reset OFF).
4. Prefetch is not available.
5. The voltage range for USB full speed PHYs can drop down to 2.7V. However, the electrical characteristics of D- and D+
pins are degraded between 2.7 and 3V.
6.3.2 VCAP1/VCAP2 external capacitor
Stabilization for the main regulator is achieved by connecting an external capacitor C to
EXT
the VCAP1/VCAP2 pins. C is specified in Table19.
EXT
Figure 24. External capacitor C
EXT
C
ESR
R Leak
MS19044V2
1. Legend: ESR is the equivalent series resistance.
Table 19. VCAP1/VCAP2 operating conditions(1)
Symbol Parameter Conditions
CEXT Capacitance of external capacitor 2.2 µF
ESR ESR of external capacitor < 2 Ω
1. When bypassing the voltage regulator, the two 2.2µF V capacitors are not required and should be
CAP
replaced by two 100nF decoupling capacitors.
DS9405 Rev 13 97/240
197

<!-- Page 98 -->


| Symbol | Parameter | Min | Max | Unit |
| --- | --- | --- | --- | --- |
| t VDD | V rise time rate DD | 20 | ∞ | µs/V |
|  | V fall time rate DD | 20 | ∞ |  |


| Symbol | Parameter | Conditions | Min | Max | Unit |
| --- | --- | --- | --- | --- | --- |
| t VDD | V rise time rate DD | Power-up | 20 | ∞ | µs/V |
|  | V fall time rate DD | Power-down | 20 | ∞ |  |
| t VCAP | V and V rise time rate CAP_1 CAP_2 | Power-up | 20 | ∞ |  |
|  | V and V fall time rate CAP_1 CAP_2 | Power-down | 20 | ∞ |  |

Electrical characteristics STM32F427xx STM32F429xx
6.3.3 Operating conditions at power-up / power-down (regulator ON)
Subject to general operating conditions for T .
A
Table 20. Operating conditions at power-up / power-down (regulator ON)
Symbol Parameter Min Max Unit
V rise time rate 20 ∞
DD
t µs/V
VDD
V fall time rate 20 ∞
DD
6.3.4 Operating conditions at power-up / power-down (regulator OFF)
Subject to general operating conditions for T .
A
Table 21. Operating conditions at power-up / power-down (regulator OFF)(1)
Symbol Parameter Conditions Min Max Unit
V rise time rate Power-up 20 ∞
DD
t
VDD
V fall time rate Power-down 20 ∞
DD
µs/V
V and V rise time rate Power-up 20 ∞
CAP_1 CAP_2
t
VCAP
V and V fall time rate Power-down 20 ∞
CAP_1 CAP_2
1. To reset the internal logic at power-down, a reset must be applied on pin PA0 when V reaches below
DD
1.08V.
98/240 DS9405 Rev 13

<!-- Page 99 -->


| Symbol | Parameter | Conditions | Min | Typ | Max | Unit |
| --- | --- | --- | --- | --- | --- | --- |
| V PVD | Programmable voltage detector level selection | PLS[2:0]=000 (rising edge) | 2.09 | 2.14 | 2.19 | V |
|  |  | PLS[2:0]=000 (falling edge) | 1.98 | 2.04 | 2.08 | V |
|  |  | PLS[2:0]=001 (rising edge) | 2.23 | 2.30 | 2.37 | V |
|  |  | PLS[2:0]=001 (falling edge) | 2.13 | 2.19 | 2.25 | V |
|  |  | PLS[2:0]=010 (rising edge) | 2.39 | 2.45 | 2.51 | V |
|  |  | PLS[2:0]=010 (falling edge) | 2.29 | 2.35 | 2.39 | V |
|  |  | PLS[2:0]=011 (rising edge) | 2.54 | 2.60 | 2.65 | V |
|  |  | PLS[2:0]=011 (falling edge) | 2.44 | 2.51 | 2.56 | V |
|  |  | PLS[2:0]=100 (rising edge) | 2.70 | 2.76 | 2.82 | V |
|  |  | PLS[2:0]=100 (falling edge) | 2.59 | 2.66 | 2.71 | V |
|  |  | PLS[2:0]=101 (rising edge) | 2.86 | 2.93 | 2.99 | V |
|  |  | PLS[2:0]=101 (falling edge) | 2.75 | 2.84 | 2.92 | V |
|  |  | PLS[2:0]=110 (rising edge) | 2.96 | 3.03 | 3.10 | V |
|  |  | PLS[2:0]=110 (falling edge) | 2.85 | 2.93 | 2.99 | V |
|  |  | PLS[2:0]=111 (rising edge) | 3.07 | 3.14 | 3.21 | V |
|  |  | PLS[2:0]=111 (falling edge) | 2.95 | 3.03 | 3.09 | V |
| V (1) PVDhyst | PVD hysteresis | - | - | 100 | - | mV |
| V POR/PDR | Power-on/power-down reset threshold | Falling edge | 1.60 | 1.68 | 1.76 | V |
|  |  | Rising edge | 1.64 | 1.72 | 1.80 | V |
| V (1) PDRhyst | PDR hysteresis | - | - | 40 | - | mV |
| V BOR1 | Brownout level 1 threshold | Falling edge | 2.13 | 2.19 | 2.24 | V |
|  |  | Rising edge | 2.23 | 2.29 | 2.33 | V |
| V BOR2 | Brownout level 2 threshold | Falling edge | 2.44 | 2.50 | 2.56 | V |
|  |  | Rising edge | 2.53 | 2.59 | 2.63 | V |
| V BOR3 | Brownout level 3 threshold | Falling edge | 2.75 | 2.83 | 2.88 | V |
|  |  | Rising edge | 2.85 | 2.92 | 2.97 | V |
| V (1) BORhyst | BOR hysteresis | - | - | 100 | - | mV |
| T RSTTEMPO (1)(2) | POR reset temporization | - | 0.5 | 1.5 | 3.0 | ms |

STM32F427xx STM32F429xx Electrical characteristics
6.3.5 Reset and power control block characteristics
The parameters given in Table22 are derived from tests performed under ambient
temperature and V supply voltage conditions summarized in Table17.
DD
Table 22. Reset and power control block characteristics
Symbol Parameter Conditions Min Typ Max Unit
PLS[2:0]=000 (rising edge) 2.09 2.14 2.19 V
PLS[2:0]=000 (falling edge) 1.98 2.04 2.08 V
PLS[2:0]=001 (rising edge) 2.23 2.30 2.37 V
PLS[2:0]=001 (falling edge) 2.13 2.19 2.25 V
PLS[2:0]=010 (rising edge) 2.39 2.45 2.51 V
PLS[2:0]=010 (falling edge) 2.29 2.35 2.39 V
PLS[2:0]=011 (rising edge) 2.54 2.60 2.65 V
Programmable voltage PLS[2:0]=011 (falling edge) 2.44 2.51 2.56 V
V
PVD detector level selection
PLS[2:0]=100 (rising edge) 2.70 2.76 2.82 V
PLS[2:0]=100 (falling edge) 2.59 2.66 2.71 V
PLS[2:0]=101 (rising edge) 2.86 2.93 2.99 V
PLS[2:0]=101 (falling edge) 2.75 2.84 2.92 V
PLS[2:0]=110 (rising edge) 2.96 3.03 3.10 V
PLS[2:0]=110 (falling edge) 2.85 2.93 2.99 V
PLS[2:0]=111 (rising edge) 3.07 3.14 3.21 V
PLS[2:0]=111 (falling edge) 2.95 3.03 3.09 V
V (1) PVD hysteresis - - 100 - mV
PVDhyst
Power-on/power-down Falling edge 1.60 1.68 1.76 V
V
POR/PDR reset threshold Rising edge 1.64 1.72 1.80 V
V (1) PDR hysteresis - - 40 - mV
PDRhyst
Brownout level 1 Falling edge 2.13 2.19 2.24 V
V
BOR1 threshold Rising edge 2.23 2.29 2.33 V
Brownout level 2 Falling edge 2.44 2.50 2.56 V
V
BOR2 threshold
Rising edge 2.53 2.59 2.63 V
Brownout level 3 Falling edge 2.75 2.83 2.88 V
V
BOR3 threshold
Rising edge 2.85 2.92 2.97 V
V (1) BOR hysteresis - - 100 - mV
BORhyst
T
RSTTEMPO POR reset temporization - 0.5 1.5 3.0 ms
(1)(2)
DS9405 Rev 13 99/240
197

<!-- Page 100 -->


| Symbol | Parameter | Conditions | Min | Typ | Max | Unit |
| --- | --- | --- | --- | --- | --- | --- |
| I (1) RUSH | InRush current on voltage regulator power- on (POR or wakeup from Standby) | - | - | 160 | 200 | mA |
| E (1) RUSH | InRush energy on voltage regulator power- on (POR or wakeup from Standby) | V = 1.7V, T = 105°C, DD A I = 171mA for 31µs RUSH | - | - | 5.4 | µC |


| Symbol | Parameter | Conditions | Min | Typ | Max | Unit |
| --- | --- | --- | --- | --- | --- | --- |
| Tod_swen | Over_drive switch enable time | HSI | - | 45 | - | µs |
|  |  | HSE max for 4MHz and min for 26MHz | 45 | - | 100 |  |
|  |  | External HSE 50MHz | - | 40 | - |  |
| Tod_swdis | Over_drive switch disable time | HSI | - | 20 | - |  |
|  |  | HSE max for 4MHz and min for 26MHz. | 20 | - | 80 |  |
|  |  | External HSE 50MHz | - | 15 | - |  |

Electrical characteristics STM32F427xx STM32F429xx
Table 22. Reset and power control block characteristics (continued)
Symbol Parameter Conditions Min Typ Max Unit
InRush current on
voltage regulator power-
I (1) - - 160 200 mA
RUSH on (POR or wakeup
from Standby)
InRush energy on
E (1) voltage regulator power- V DD = 1.7V, T A = 105°C, - - 5.4 µC
RUSH on (POR or wakeup I = 171mA for 31µs
RUSH
from Standby)
1. Specified by design.
2. The reset temporization is measured from the power-on (POR reset or wake-up from V ) to the instant
BAT
when the first instruction is read by the user application code.
6.3.6 Overdrive switching characteristics
When the overdrive mode switches from enabled to disabled or disabled to enabled, the
system clock is stalled during the internal voltage set-up.
The overdrive switching characteristics are given in Table23. They are subject to general
operating conditions for T .
A
Table 23. Over-drive switching characteristics(1)
Symbol Parameter Conditions Min Typ Max Unit
HSI - 45 -
HSE max for 4MHz
Over_drive switch 45 - 100
Tod_swen and min for 26MHz
enable time
External HSE
- 40 -
50MHz
µs
HSI - 20 -
HSE max for 4MHz
Over_drive switch 20 - 80
Tod_swdis and min for 26MHz.
disable time
External HSE
- 15 -
50MHz
1. Specified by design.
100/240 DS9405 Rev 13

<!-- Page 101 -->

STM32F427xx STM32F429xx Electrical characteristics
6.3.7 Supply current characteristics
The current consumption is a function of several parameters and factors such as the
operating voltage, ambient temperature, I/O pin loading, device software configuration,
operating frequencies, I/O pin switching rate, program location in memory and executed
binary code.
The current consumption is measured as described in Figure23: Current consumption
measurement scheme.
All the run-mode current consumption measurements given in this section are performed
with a reduced code that gives a consumption equivalent to CoreMark code.
Typical and maximum current consumption
The MCU is placed under the following conditions:
• All I/O pins are in input mode with a static value at V or V (no load).
DD SS
• All peripherals are disabled except if it is explicitly mentioned.
• The flash memory access time is adjusted both to f frequency and V range (see
HCLK DD
Table18: Limitations depending on the operating power supply range).
• Regulator ON
• The voltage scaling and overdrive mode are adjusted to f frequency as follows:
HCLK
– Scale 3 for f ≤ 120MHz
HCLK
– Scale 2 for 120MHz < f ≤ 144MHz
HCLK
– Scale 1 for 144MHz < f ≤ 180MHz. The overdrive is only ON at 180MHz.
HCLK
• The system clock is HCLK, f = f /4, and f = f /2.
PCLK1 HCLK PCLK2 HCLK
• The external clock frequency is 4MHz and PLL is ON when f is higher than
HCLK
25MHz.
• The maximum values are obtained for V = 3.6V and a maximum ambient
DD
temperature (T ), and the typical values for T = 25°C and V = 3.3V unless
A A DD
otherwise specified.
DS9405 Rev 13 101/240
197

<!-- Page 102 -->


| Symbol | Parameter | Conditions | f (MHz) HCLK | Typ | Max(2) |  |  | Unit |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
|  |  |  |  |  | T = A 25°C | T = A 85°C | T = A 105°C |  |
| I DD | Supply current in RUN mode | All Peripherals enabled(3)(4) | 180 | 98 | 104(5) | 123 | 141(5) | mA |
|  |  |  | 168 | 89 | 98(5) | 116 | 133(5) |  |
|  |  |  | 150 | 75 | 84 | 100 | 115 |  |
|  |  |  | 144 | 72 | 81 | 96 | 112 |  |
|  |  |  | 120 | 54 | 58 | 72 | 85 |  |
|  |  |  | 90 | 43 | 45 | 56 | 66 |  |
|  |  |  | 60 | 29 | 30 | 52 | 62 |  |
|  |  |  | 30 | 16 | 20 | 34 | 46 |  |
|  |  |  | 25 | 13 | 16 | 30 | 43 |  |
|  |  |  | 16 | 11 | 13 | 27 | 39 |  |
|  |  |  | 8 | 5 | 9 | 23 | 36 |  |
|  |  |  | 4 | 4 | 8 | 21 | 34 |  |
|  |  |  | 2 | 2 | 7 | 20 | 33 |  |
|  |  | All Peripherals disabled(3) | 180 | 44 | 47(5) | 69 | 87(5) |  |
|  |  |  | 168 | 41 | 45(5) | 66 | 83(5) |  |
|  |  |  | 150 | 36 | 39 | 57 | 73 |  |
|  |  |  | 144 | 33 | 37 | 56 | 72 |  |
|  |  |  | 120 | 25 | 29 | 43 | 56 |  |
|  |  |  | 90 | 20 | 23 | 41 | 53 |  |
|  |  |  | 60 | 14 | 16 | 34 | 45 |  |
|  |  |  | 30 | 8 | 12 | 26 | 39 |  |
|  |  |  | 25 | 7 | 10 | 24 | 37 |  |
|  |  |  | 16 | 7 | 9 | 22 | 35 |  |
|  |  |  | 8 | 3 | 7 | 21 | 34 |  |
|  |  |  | 4 | 3 | 6 | 20 | 33 |  |
|  |  |  | 2 | 2 | 6 | 20 | 33 |  |

Electrical characteristics STM32F427xx STM32F429xx
Table 24. Typical and maximum current consumption in Run mode, code with data processing
running from Flash memory (ART accelerator enabled except prefetch) or RAM(1)
Max(2)
Symbol Parameter Conditions f (MHz) Typ Unit
HCLK T = T = T =
A A A
25°C 85°C 105°C
180 98 104(5) 123 141(5)
168 89 98(5) 116 133(5)
150 75 84 100 115
144 72 81 96 112
120 54 58 72 85
90 43 45 56 66
All
Peripherals 60 29 30 52 62
enabled(3)(4)
30 16 20 34 46
25 13 16 30 43
16 11 13 27 39
8 5 9 23 36
4 4 8 21 34
Supply 2 2 7 20 33
I current in mA
DD RUN mode 180 44 47(5) 69 87(5)
168 41 45(5) 66 83(5)
150 36 39 57 73
144 33 37 56 72
120 25 29 43 56
90 20 23 41 53
All
Peripherals 60 14 16 34 45
disabled(3)
30 8 12 26 39
25 7 10 24 37
16 7 9 22 35
8 3 7 21 34
4 3 6 20 33
2 2 6 20 33
1. Code and data processing running from SRAM1 using boot pins.
2. Evaluated by characterization.
3. When analog peripheral blocks such as ADCs, DACs, HSE, LSE, HSI, or LSI are ON, an additional power consumption
should be considered.
4. When the ADC is ON (ADON bit set in the ADC_CR2 register), add an additional power consumption of 1.6 mA per ADC
for the analog part.
5. Evaluated by test in production.
102/240 DS9405 Rev 13

<!-- Page 103 -->


| Symbol | Parameter | Conditions | f (MHz) HCLK | Typ | Max(1) |  |  | Unit |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
|  |  |  |  |  | TA= 25°C | TA=85°C | TA=105°C |  |
| I DD | Supply current in RUN mode | All Peripherals enabled(2)(3) | 180 | 103 | 112 | 140 | 151 | mA |
|  |  |  | 168 | 98 | 107 | 126 | 144 |  |
|  |  |  | 150 | 87 | 95 | 112 | 128 |  |
|  |  |  | 144 | 85 | 92 | 108 | 124 |  |
|  |  |  | 120 | 66 | 71 | 85 | 99 |  |
|  |  |  | 90 | 54 | 58 | 69 | 80 |  |
|  |  |  | 60 | 37 | 39 | 47 | 55 |  |
|  |  |  | 30 | 20 | 24 | 39 | 51 |  |
|  |  |  | 25 | 17 | 21 | 35 | 48 |  |
|  |  |  | 16 | 12 | 16 | 30 | 42 |  |
|  |  |  | 8 | 7 | 11 | 24 | 37 |  |
|  |  |  | 4 | 5 | 8 | 22 | 35 |  |
|  |  |  | 2 | 3 | 7 | 21 | 34 |  |
|  |  | All Peripherals disabled(3) | 180 | 57 | 62 | 87 | 106 |  |
|  |  |  | 168 | 50 | 54 | 76 | 93 |  |
|  |  |  | 150 | 46 | 50 | 70 | 86 |  |
|  |  |  | 144 | 45 | 49 | 68 | 84 |  |
|  |  |  | 120 | 36 | 41 | 56 | 69 |  |
|  |  |  | 90 | 29 | 34 | 46 | 57 |  |
|  |  |  | 60 | 21 | 24 | 33 | 41 |  |
|  |  |  | 30 | 13 | 17 | 31 | 44 |  |
|  |  |  | 25 | 11 | 15 | 28 | 41 |  |
|  |  |  | 16 | 8 | 12 | 25 | 38 |  |
|  |  |  | 8 | 5 | 9 | 23 | 35 |  |
|  |  |  | 4 | 4 | 7 | 21 | 34 |  |
|  |  |  | 2 | 3 | 6.5 | 20 | 33 |  |

STM32F427xx STM32F429xx Electrical characteristics
Table 25. T y pical and maximum current consumption in Run mode, code with data processing
running from Flash memory (ART accelerator disabled)
Max(1)
Symbol Parameter Conditions f (MHz) Typ Unit
HCLK TA=
TA=85°C TA=105°C
25°C
180 103 112 140 151
168 98 107 126 144
150 87 95 112 128
144 85 92 108 124
120 66 71 85 99
90 54 58 69 80
All Peripherals
60 37 39 47 55
enabled(2)(3)
30 20 24 39 51
25 17 21 35 48
16 12 16 30 42
8 7 11 24 37
4 5 8 22 35
Supply 2 3 7 21 34
I current in mA
DD
RUN mode 180 57 62 87 106
168 50 54 76 93
150 46 50 70 86
144 45 49 68 84
120 36 41 56 69
90 29 34 46 57
All Peripherals
60 21 24 33 41
disabled(3)
30 13 17 31 44
25 11 15 28 41
16 8 12 25 38
8 5 9 23 35
4 4 7 21 34
2 3 6.5 20 33
1. Evaluated by characterization unless otherwise specified.
2. When analog peripheral blocks such as ADCs, DACs, HSE, LSE, HSI, or LSI are ON, an additional power consumption
should be considered.
3. When the ADC is ON (ADON bit set in the ADC_CR2 register), add an additional power consumption of 1.6 mA per ADC for
the analog part.
DS9405 Rev 13 103/240
197

<!-- Page 104 -->


| Symbol | Parameter | Conditions | f (MHz) HCLK | Typ | Max(1) |  |  | Unit |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
|  |  |  |  |  | T = A 25°C | T = A 85°C | T = A 105°C |  |
| I DD | Supply current in Sleep mode | All Peripherals enabled(2) | 180 | 78 | 89(3) | 110 | 130(3) | mA |
|  |  |  | 168 | 66 | 75(3) | 93 | 110(3) |  |
|  |  |  | 150 | 56 | 61 | 80 | 96 |  |
|  |  |  | 144 | 54 | 58 | 78 | 94 |  |
|  |  |  | 120 | 40 | 44 | 59 | 72 |  |
|  |  |  | 90 | 32 | 34 | 46 | 56 |  |
|  |  |  | 60 | 22 | 23 | 31 | 45 |  |
|  |  |  | 30 | 10 | 16 | 30 | 43 |  |
|  |  |  | 25 | 9 | 14 | 28 | 40 |  |
|  |  |  | 16 | 5 | 12 | 25 | 40 |  |
|  |  |  | 8 | 3 | 8 | 22 | 35 |  |
|  |  |  | 4 | 3 | 7 | 21 | 34 |  |
|  |  |  | 2 | 2 | 6.5 | 20 | 33 |  |
|  |  | All Peripherals disabled | 180 | 21 | 26(3) | 54 | 76(3) |  |
|  |  |  | 168 | 16 | 20(3) | 41 | 58(3) |  |
|  |  |  | 150 | 14 | 17 | 36 | 52 |  |
|  |  |  | 144 | 13 | 16.5 | 35 | 51 |  |
|  |  |  | 120 | 10 | 14 | 28 | 41 |  |
|  |  |  | 90 | 8 | 13 | 26 | 37 |  |
|  |  |  | 60 | 6 | 9 | 24 | 37 |  |
|  |  |  | 30 | 5 | 8 | 22 | 35 |  |
|  |  |  | 25 | 3 | 7 | 21 | 34 |  |
|  |  |  | 16 | 3 | 7 | 21 | 34 |  |
|  |  |  | 8 | 2 | 6 | 20 | 33 |  |
|  |  |  | 4 | 2 | 6 | 20 | 33 |  |
|  |  |  | 2 | 2 | 6 | 20 | 33 |  |

Electrical characteristics STM32F427xx STM32F429xx
Table 26. Typical and maximum current consumption in Sleep mode
Max(1)
Symbol Parameter Conditions f (MHz) Typ Unit
HCLK T = T = T =
A A A
25°C 85°C 105°C
180 78 89(3) 110 130(3)
168 66 75(3) 93 110(3)
150 56 61 80 96
144 54 58 78 94
120 40 44 59 72
90 32 34 46 56
All
Peripherals 60 22 23 31 45
enabled(2)
30 10 16 30 43
25 9 14 28 40
16 5 12 25 40
8 3 8 22 35
4 3 7 21 34
Supply 2 2 6.5 20 33
I current in mA
DD Sleep mode 180 21 26(3) 54 76(3)
168 16 20(3) 41 58(3)
150 14 17 36 52
144 13 16.5 35 51
120 10 14 28 41
90 8 13 26 37
All
Peripherals 60 6 9 24 37
disabled
30 5 8 22 35
25 3 7 21 34
16 3 7 21 34
8 2 6 20 33
4 2 6 20 33
2 2 6 20 33
1. Evaluated by characterization unless otherwise specified.
2. When analog peripheral blocks such as ADCs, DACs, HSE, LSE, HSI, or LSI are ON, an additional power consumption
should be considered.
3. Based on characterization, tested in production.
104/240 DS9405 Rev 13

<!-- Page 105 -->


| Symbol | Parameter | Conditions | Typ | Max(1) |  |  | Unit |
| --- | --- | --- | --- | --- | --- | --- | --- |
|  |  |  |  | V = 3.6V DD |  |  |  |
|  |  |  | T = A 25°C | T = A 25°C | T = A 85°C | T = A 105°C |  |
| I DD_STOP_NM (normal mode) | Supply current in Stop mode with voltage regulator in main regulator mode | Flash memory in Stop mode, all oscillators OFF, no independent watchdog | 0.40 | 1.50 | 14.00 | 25.00 | mA |
|  |  | Flash memory in Deep power down mode, all oscillators OFF, no independent watchdog | 0.35 | 1.50 | 14.00 | 25.00 |  |
|  | Supply current in Stop mode with voltage regulator in Low Power regulator mode | Flash memory in Stop mode, all oscillators OFF, no independent watchdog | 0.29 | 1.10 | 10.00 | 18.00 |  |
|  |  | Flash memory in Deep power down mode, all oscillators OFF, no independent watchdog | 0.23 | 1.10 | 10.00 | 18.00 |  |
| I DD_STOP_UDM (under-drive mode) | Supply current in Stop mode with voltage regulator in main regulator and under- drive mode | Flash memory in Deep power down mode, main regulator in under-drive mode, all oscillators OFF, no independent watchdog | 0.19 | 0.50 | 6.00 | 9.00 |  |
|  | Supply current in Stop mode with voltage regulator in Low Power regulator and under- drive mode | Flash memory in Deep power down mode, Low Power regulator in under-drive mode, all oscillators OFF, no independent watchdog | 0.10 | 0.40 | 4.00 | 7.00 |  |

STM32F427xx STM32F429xx Electrical characteristics
Table 27. Typical and maximum current consumptions in Stop mode
Max(1)
Typ
V = 3.6V
Symbol Parameter Conditions DD Unit
T = T = T = T =
A A A A
25°C 25°C 85°C 105°C
Flash memory in Stop mode, all
Supply current in Stop oscillators OFF, no independent 0.40 1.50 14.00 25.00
mode with voltage watchdog
regulator in main Flash memory in Deep power
regulator mode down mode, all oscillators OFF, no 0.35 1.50 14.00 25.00
I independent watchdog
DD_STOP_NM
(normal mode) Flash memory in Stop mode, all
Supply current in Stop oscillators OFF, no independent 0.29 1.10 10.00 18.00
mode with voltage watchdog
regulator in Low Power Flash memory in Deep power
regulator mode down mode, all oscillators OFF, no 0.23 1.10 10.00 18.00 mA
independent watchdog
Supply current in Stop
Flash memory in Deep power
mode with voltage
down mode, main regulator in
regulator in main 0.19 0.50 6.00 9.00
under-drive mode, all oscillators
regulator and under-
I OFF, no independent watchdog
DD_STOP_UDM drive mode
(under-drive
Supply current in Stop
mode) Flash memory in Deep power
mode with voltage
down mode, Low Power regulator
regulator in Low Power 0.10 0.40 4.00 7.00
in under-drive mode, all oscillators
regulator and under-
OFF, no independent watchdog
drive mode
1. Data based on characterization, tested in production.
DS9405 Rev 13 105/240
197

<!-- Page 106 -->


| Symbol | Parameter | Conditions | Typ(1) |  |  | Max(2) |  |  | Unit |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
|  |  |  | T = 25°C A |  |  | T = A 25°C | T = A 85°C | T = A 105°C |  |
|  |  |  | V = DD 1.7V | V = DD 2.4V | V = DD 3.3V | V = 3.6V DD |  |  |  |
| I DD_STBY | Supply current in Standby mode | Backup SRAM ON, low-speed oscillator (LSE) and RTC ON | 2.80 | 3.00 | 3.60 | 7.00 | 19.00 | 36.00 | µA |
|  |  | Backup SRAM OFF, low- speed oscillator (LSE) and RTC ON | 2.30 | 2.60 | 3.10 | 6.00 | 16.00 | 31.00 |  |
|  |  | Backup SRAM ON, RTC and LSE OFF | 2.30 | 2.50 | 2.90 | 6.00(3) | 18.00(3) | 35.00(3) |  |
|  |  | Backup SRAM OFF, RTC and LSE OFF | 1.70 | 1.90 | 2.20 | 5.00(3) | 15.00(3) | 30.00(3) |  |


| Symbol | Parameter | Conditions(1) | Typ |  |  | Max(2) |  | Unit |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
|  |  |  | T = 25°C A |  |  | T = 85°C A | T = A 105°C |  |
|  |  |  | V = BAT 1.7V | V = BAT 2.4V | V = BAT 3.3V | V = 3.6V BAT |  |  |
| I DD_VBAT | Backup domain supply current | Backup SRAM ON, low-speed oscillator (LSE) and RTC ON | 1.28 | 1.40 | 1.62 | 6 | 11 | µA |
|  |  | Backup SRAM OFF, low-speed oscillator (LSE) and RTC ON | 0.66 | 0.76 | 0.97 | 3 | 5 |  |
|  |  | Backup SRAM ON, RTC and LSE OFF | 0.70 | 0.72 | 0.74 | 5 | 10 |  |
|  |  | Backup SRAM OFF, RTC and LSE OFF | 0.10 | 0.10 | 0.10 | 2 | 4 |  |

Electrical characteristics STM32F427xx STM32F429xx
Table 28. Typical and maximum current consumptions in Standby mode
Typ(1) Max(2)
T = T = T =
T = 25°C A A A
Symbol Parameter Conditions A 25°C 85°C 105°C Unit
V = V = V =
DD DD DD V = 3.6V
1.7V 2.4V 3.3V DD
Backup SRAM ON, low-speed
2.80 3.00 3.60 7.00 19.00 36.00
oscillator (LSE) and RTC ON
Backup SRAM OFF, low-
Supply current speed oscillator (LSE) and 2.30 2.60 3.10 6.00 16.00 31.00
I DD_STBY in Standby RTC ON µA
mode Backup SRAM ON, RTC and
2.30 2.50 2.90 6.00(3) 18.00(3) 35.00(3)
LSE OFF
Backup SRAM OFF, RTC and
1.70 1.90 2.20 5.00(3) 15.00(3) 30.00(3)
LSE OFF
1. The typical current consumption values are given with PDR OFF (internal reset OFF). When the PDR is OFF (internal reset
OFF), the typical current consumption is reduced by an additional 1.2µA.
2. Evaluated by characterization, not tested in production unless otherwise specified.
3. Based on characterization, tested in production.
Table 29. Typical and maximum current consumptions in V mode
BAT
Typ Max(2)
T =
T = 25°C T = 85°C A
Symbol Parameter Conditions(1) A A 105°C Unit
V = V = V =
BAT BAT BAT V = 3.6V
1.7V 2.4V 3.3V BAT
Backup SRAM ON, low-speed
1.28 1.40 1.62 6 11
oscillator (LSE) and RTC ON
Backup SRAM OFF, low-speed
Backup 0.66 0.76 0.97 3 5
oscillator (LSE) and RTC ON
I domain supply µA
DD_VBAT
current Backup SRAM ON, RTC and 0.70 0.72 0.74 5 10
LSE OFF
Backup SRAM OFF, RTC and
0.10 0.10 0.10 2 4
LSE OFF
1. Crystal used: Abracon ABS07-120-32.768 kHz-T with a C of 6 pF for typical values.
L
2. Evaluated by characterization.
106/240 DS9405 Rev 13

<!-- Page 107 -->


|  |  |  |  |  |  |
| --- | --- | --- | --- | --- | --- |
|  |  |  |  |  |  |
|  |  |  |  | 1.65V |  |
|  |  |  |  |  | 1.65V |
|  |  |  |  |  | 1.7V 1.8V 2V 2.4V |
|  |  |  |  |  | 2.7V 3V |
|  |  |  |  | 3.3V 3.6V | 3.3V 3.6V |


|  |  |  |  |  |
| --- | --- | --- | --- | --- |
|  |  |  |  |  |
|  |  |  |  | 1.65V 1.7V |
|  |  |  |  | 1.8V 2V 2.4V |
|  |  |  |  | 2.7V 3V |
|  |  |  |  | 3.3V 3.6V |

STM32F427xx STM32F429xx Electrical characteristics
Figure 25. Typical V current consumption (LSE and RTC ON/backup RAM OFF)
BAT
3
2.5
2
1.5
1
0.5
0
0°C 25°C 55°C 85°C 105°C
MS30490V1
Figure 26. Typical V current consumption (LSE and RTC ON/backup RAM ON)
BAT
DS9405 Rev 13 107/240
197
)Aμ(
TABV_DDI
1.65V
1.7V
1.8V
2V
2.4V
2.7V
3V
3.3V
3.6V
Temperature
MS30491V1
)Aμ(
TABV_DDI
6
5
4
1.65V
1.7V
3 1.8V
2V
2 2.4V
2.7V
3V
1
3.3V
3.6V
0
0°C 25°C 55°C 85°C 105°C
Temperature

<!-- Page 108 -->


| Symbol | Parameter | Conditions | f (MHz) HCLK | Typ | Unit |
| --- | --- | --- | --- | --- | --- |
| I DD | Supply current in RUN mode from V supply DD | All Peripheral enabled | 168 | 88.2 | mA |
|  |  |  | 150 | 74.3 |  |
|  |  |  | 144 | 71.3 |  |
|  |  |  | 120 | 52.9 |  |
|  |  |  | 90 | 42.6 |  |
|  |  |  | 60 | 28.6 |  |
|  |  |  | 30 | 15.7 |  |
|  |  |  | 25 | 12.3 |  |
|  |  | All Peripheral disabled | 168 | 40.6 |  |
|  |  |  | 150 | 30.6 |  |
|  |  |  | 144 | 32.6 |  |
|  |  |  | 120 | 24.7 |  |
|  |  |  | 90 | 19.7 |  |
|  |  |  | 60 | 13.6 |  |
|  |  |  | 30 | 7.7 |  |
|  |  |  | 25 | 6.7 |  |

Electrical characteristics STM32F427xx STM32F429xx
Additional current consumption
The MCU is placed under the following conditions:
• All I/O pins are configured in analog mode.
• The flash memory access time is adjusted to fHCLK frequency.
• The voltage scaling is adjusted to fHCLK frequency as follows:
– Scale 3 for f ≤ 120MHz,
HCLK
– Scale 2 for 120MHz < f ≤ 144MHz
HCLK
– Scale 1 for 144MHz < f ≤ 180MHz. The overdrive is only ON at 180MHz.
HCLK
• The system clock is HCLK, f = f /4, and f = f /2.
PCLK1 HCLK PCLK2 HCLK
• HSE crystal clock frequency is 25MHz.
• When the regulator is OFF, V12 is provided externally as described in Table17:
General operating conditions
• T = 25°C.
A
Table 30. T y pical current consumption in Run mode, code with data processing running from
Flash memory or RAM, regulator ON (ART accelerator enabled except prefetch),
V =1.7V(1)
DD
Symbol Parameter Conditions f (MHz) Typ Unit
HCLK
168 88.2
150 74.3
144 71.3
All Peripheral 120 52.9
enabled
90 42.6
60 28.6
30 15.7
Supply current in 25 12.3
I RUN mode from mA
DD
V supply 168 40.6
DD
150 30.6
144 32.6
All Peripheral 120 24.7
disabled
90 19.7
60 13.6
30 7.7
25 6.7
1. When peripherals are enabled, the power consumption corresponding to the analog part of the peripherals (such as ADC,
or DAC) is not included.
108/240 DS9405 Rev 13

<!-- Page 109 -->


| Symbol | Parameter | Conditions | f HCLK (MHz) | VDD=3.3V |  | VDD=1.7V |  | Unit |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
|  |  |  |  | I DD12 | I DD | I DD12 | I DD |  |
| I / I DD12 DD | Supply current in RUN mode from V and V 12 DD supply | All Peripherals enabled | 168 | 77.8 | 1.3 | 76.8 | 1.0 | mA |
|  |  |  | 150 | 70.8 | 1.3 | 69.8 | 1.0 |  |
|  |  |  | 144 | 64.5 | 1.3 | 63.6 | 1.0 |  |
|  |  |  | 120 | 49.9 | 1.2 | 49.3 | 0.9 |  |
|  |  |  | 90 | 39.2 | 1.3 | 38.7 | 1.0 |  |
|  |  |  | 60 | 27.2 | 1.2 | 26.8 | 0.9 |  |
|  |  |  | 30 | 15.6 | 1.2 | 15.4 | 0.9 |  |
|  |  |  | 25 | 13.6 | 1.2 | 13.5 | 0.9 |  |
|  |  | All Peripherals disabled | 168 | 38.2 | 1.3 | 37.0 | 1.0 |  |
|  |  |  | 150 | 34.6 | 1.3 | 33.4 | 1.0 |  |
|  |  |  | 144 | 31.3 | 1.3 | 30.3 | 1.0 |  |
|  |  |  | 120 | 24.0 | 1.2 | 23.2 | 0.9 |  |
|  |  |  | 90 | 18.1 | 1.4 | 18.0 | 1.0 |  |
|  |  |  | 60 | 12.9 | 1.2 | 12.5 | 0.9 |  |
|  |  |  | 30 | 7.2 | 1.2 | 6.9 | 0.9 |  |
|  |  |  | 25 | 6.3 | 1.2 | 6.1 | 0.9 |  |

STM32F427xx STM32F429xx Electrical characteristics
Table 31. T ypical current consumption in Run mode, code with data processing running
from Flash memory, regulator OFF (ART accelerator enabled except prefetch)(1)
VDD=3.3V VDD=1.7V
f
Symbol Parameter Conditions HCLK Unit
(MHz)
I I I I
DD12 DD DD12 DD
168 77.8 1.3 76.8 1.0
150 70.8 1.3 69.8 1.0
144 64.5 1.3 63.6 1.0
All Peripherals 120 49.9 1.2 49.3 0.9
enabled
90 39.2 1.3 38.7 1.0
60 27.2 1.2 26.8 0.9
30 15.6 1.2 15.4 0.9
Supply current in
RUN mode from 25 13.6 1.2 13.5 0.9
I / I mA
DD12 DD V and V
12 DD 168 38.2 1.3 37.0 1.0
supply
150 34.6 1.3 33.4 1.0
144 31.3 1.3 30.3 1.0
120 24.0 1.2 23.2 0.9
All Peripherals
disabled 90 18.1 1.4 18.0 1.0
60 12.9 1.2 12.5 0.9
30 7.2 1.2 6.9 0.9
25 6.3 1.2 6.1 0.9
1. When peripherals are enabled, the power consumption corresponding to the analog part of the peripherals (such as ADC,
or DAC) is not included.
DS9405 Rev 13 109/240
197

<!-- Page 110 -->


| Symbol | Parameter | Conditions | f (MHz) HCLK | Typ | Unit |
| --- | --- | --- | --- | --- | --- |
| I DD | Supply current in Sleep mode from V supply DD | All Peripherals enabled | 168 | 65.5 | mA |
|  |  |  | 150 | 55.5 |  |
|  |  |  | 144 | 53.5 |  |
|  |  |  | 120 | 39.0 |  |
|  |  |  | 90 | 31.6 |  |
|  |  |  | 60 | 21.7 |  |
|  |  |  | 30 | 9.8 |  |
|  |  |  | 25 | 8.8 |  |
|  |  | All Peripherals disabled | 168 | 15.7 |  |
|  |  |  | 150 | 13.7 |  |
|  |  |  | 144 | 12.7 |  |
|  |  |  | 120 | 9.7 |  |
|  |  |  | 90 | 7.7 |  |
|  |  |  | 60 | 5.7 |  |
|  |  |  | 30 | 4.7 |  |
|  |  |  | 25 | 2.8 |  |

Electrical characteristics STM32F427xx STM32F429xx
Tabl e 32. Typical current consumption in Sleep mode, regulator ON, V =1.7V(1)
DD
Symbol Parameter Conditions f (MHz) Typ Unit
HCLK
168 65.5
150 55.5
144 53.5
120 39.0
All Peripherals enabled
90 31.6
60 21.7
30 9.8
Supply current in Sleep 25 8.8
I mA
DD mode from V supply
DD 168 15.7
150 13.7
144 12.7
120 9.7
All Peripherals disabled
90 7.7
60 5.7
30 4.7
25 2.8
1. When peripherals are enabled, the power consumption corresponding to the analog part of the peripherals (such as ADC,
or DAC) is not included.
110/240 DS9405 Rev 13

<!-- Page 111 -->


| Symbol | Parameter | Conditions | f (MHz) HCLK | VDD=3.3V |  | VDD=1.7V |  | Unit |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
|  |  |  |  | I DD12 | I DD | I DD12 | I DD |  |
| I /I DD12 DD | Supply current in Sleep mode from V and 12 V supply DD | All Peripherals enabled | 180 | 61.5 | 1.4 | - | - | mA |
|  |  |  | 168 | 59.4 | 1.3 | 59.4 | 1.0 |  |
|  |  |  | 150 | 53.9 | 1.3 | 53.9 | 1.0 |  |
|  |  |  | 144 | 49.0 | 1.3 | 49.0 | 1.0 |  |
|  |  |  | 120 | 38.0 | 1.2 | 38.0 | 0.9 |  |
|  |  |  | 90 | 29.3 | 1.4 | 29.3 | 1.1 |  |
|  |  |  | 60 | 20.2 | 1.2 | 20.2 | 0.9 |  |
|  |  |  | 30 | 11.9 | 1.2 | 11.9 | 0.9 |  |
|  |  |  | 25 | 10.4 | 1.2 | 10.4 | 0.9 |  |
|  |  | All Peripherals disabled | 180 | 14.9 | 1.4 | - | - |  |
|  |  |  | 168 | 14.0 | 1.3 | 14.0 | 1.0 |  |
|  |  |  | 150 | 12.6 | 1.3 | 12.6 | 1.0 |  |
|  |  |  | 144 | 11.5 | 1.3 | 11.5 | 1.0 |  |
|  |  |  | 120 | 8.7 | 1.2 | 8.7 | 0.9 |  |
|  |  |  | 90 | 7.1 | 1.4 | 7.1 | 1.1 |  |
|  |  |  | 60 | 5.0 | 1.2 | 5.0 | 0.9 |  |
|  |  |  | 30 | 3.1 | 1.2 | 3.1 | 0.9 |  |
|  |  |  | 25 | 2.8 | 1.2 | 2.8 | 0.9 |  |

STM32F427xx STM32F429xx Electrical characteristics
Table 33. Tyical current consumption in Sleep mode, regulator OFF(1)
VDD=3.3V VDD=1.7V Unit
Symbol Parameter Conditions f (MHz)
HCLK
I I I I
DD12 DD DD12 DD
180 61.5 1.4 - -
168 59.4 1.3 59.4 1.0
150 53.9 1.3 53.9 1.0
144 49.0 1.3 49.0 1.0
All Peripherals
120 38.0 1.2 38.0 0.9
enabled
90 29.3 1.4 29.3 1.1
60 20.2 1.2 20.2 0.9
30 11.9 1.2 11.9 0.9
Supply current
25 10.4 1.2 10.4 0.9
in Sleep mode
I /I mA
DD12 DD from V and
12 180 14.9 1.4 - -
V supply
DD
168 14.0 1.3 14.0 1.0
150 12.6 1.3 12.6 1.0
144 11.5 1.3 11.5 1.0
All Peripherals
120 8.7 1.2 8.7 0.9
disabled
90 7.1 1.4 7.1 1.1
60 5.0 1.2 5.0 0.9
30 3.1 1.2 3.1 0.9
25 2.8 1.2 2.8 0.9
1. When peripherals are enabled, the power consumption corresponding to the analog part of the peripherals (such as ADC,
or DAC) is not included.
DS9405 Rev 13 111/240
197

<!-- Page 112 -->

Electrical characteristics STM32F427xx STM32F429xx
I/O system current consumption
The current consumption of the I/O system has two components: static and dynamic.
I/O static current consumption
All the I/Os used as inputs with pull resistors generate current consumption when the pin is
externally held to the opposite level. The value of this current consumption can be simply
computed by using the pull-up/pull-down resistor values given in Table57: I/O static
characteristics.
For the output pins, any internal or external pull-up or pull-down and external load must also
be considered to estimate the current consumption.
Additional I/O current consumption is due to I/Os configured as inputs if an intermediate
voltage level is externally applied. This current consumption is caused by the input Schmitt
trigger circuits used to discriminate the input value. Unless this specific configuration is
required by the application, this supply current consumption can be avoided by configuring
these I/Os in analog mode. This is notably the case of ADC input pins, which should be
configured as analog inputs.
Caution: Any floating input pin can also settle to an intermediate voltage level or switch inadvertently,
as a result of external electromagnetic noise. To avoid current consumption related to
floating pins, they must either be configured in analog mode, or forced internally to a definite
digital value. This can be done either by using pull-up/down resistors or by configuring the
pins in output mode.
I/O dynamic current consumption
In addition to the internal peripheral current consumption (see Table35: Peripheral current
consumption), the I/Os used by an application also contribute to the current consumption.
When an I/O pin switches, it uses the current from the MCU supply voltage to supply the I/O
pin circuitry and to charge/discharge the capacitive load internal or external connected to
the pin:
I = V × f × C
SW DD SW
where
I is the current sunk by a switching I/O to charge/discharge the capacitive load
SW
V is the MCU supply voltage
DD
f is the I/O switching frequency
SW
C is the total capacitance seen by the I/O pin: C = C + C
INT EXT
The test pin is configured in push-pull output mode and is toggled by software at a fixed
frequency.
112/240 DS9405 Rev 13

<!-- Page 113 -->


| Symbol | Parameter | Conditions | I/O toggling frequency (fsw) | Typ | Unit |
| --- | --- | --- | --- | --- | --- |
| I DDIO | I/O switching Current | V = 3.3V DD C= C (2) INT | 2 MHz | 0.0 | mA |
|  |  |  | 8 MHz | 0.2 |  |
|  |  |  | 25 MHz | 0.6 |  |
|  |  |  | 50 MHz | 1.1 |  |
|  |  |  | 60 MHz | 1.3 |  |
|  |  |  | 84 MHz | 1.8 |  |
|  |  |  | 90 MHz | 1.9 |  |
|  |  | V = 3.3V DD C = 0pF EXT C = C + C INT EXT + C S | 2 MHz | 0.1 |  |
|  |  |  | 8 MHz | 0.4 |  |
|  |  |  | 25 MHz | 1.23 |  |
|  |  |  | 50 MHz | 2.43 |  |
|  |  |  | 60 MHz | 2.93 |  |
|  |  |  | 84 MHz | 3.86 |  |
|  |  |  | 90 MHz | 4.07 |  |
| I DDIO | I/O switching Current | V = 3.3V DD C = 10pF EXT C = C + C INT EXT + C S | 2 MHz | 0.18 | mA |
|  |  |  | 8 MHz | 0.67 |  |
|  |  |  | 25 MHz | 2.09 |  |
|  |  |  | 50 MHz | 3.6 |  |
|  |  |  | 60 MHz | 4.5 |  |
|  |  |  | 84 MHz | 7.8 |  |
|  |  |  | 90 MHz | 9.8 |  |
|  |  | V = 3.3V DD C = 22pF EXT C = C + C INT EXT + C S | 2 MHz | 0.26 |  |
|  |  |  | 8 MHz | 1.01 |  |
|  |  |  | 25 MHz | 3.14 |  |
|  |  |  | 50 MHz | 6.39 |  |
|  |  |  | 60 MHz | 10.68 |  |
|  |  | V = 3.3V DD C = 33pF EXT C = C + Cext INT + C S | 2 MHz | 0.33 |  |
|  |  |  | 8 MHz | 1.29 |  |
|  |  |  | 25 MHz | 4.23 |  |
|  |  |  | 50 MHz | 11.02 |  |

STM32F427xx STM32F429xx Electrical characteristics
Table 34. Switching output I/O current consumption(1)
I/O toggling
Symbol Parameter Conditions frequency Typ Unit
(fsw)
2 MHz 0.0
8 MHz 0.2
25 MHz 0.6
V = 3.3V
DD 50 MHz 1.1
C= C (2)
INT
60 MHz 1.3
84 MHz 1.8
I/O switching 90 MHz 1.9
I mA
DDIO Current 2 MHz 0.1
8 MHz 0.4
V DD = 3.3V 25 MHz 1.23
C = 0pF
EXT 50 MHz 2.43
C = C + C
INT EXT
+ C 60 MHz 2.93
S
84 MHz 3.86
90 MHz 4.07
2 MHz 0.18
8 MHz 0.67
V DD = 3.3V 25 MHz 2.09
C = 10pF
EXT 50 MHz 3.6
C = C + C
INT EXT
+ C 60 MHz 4.5
S
84 MHz 7.8
90 MHz 9.8
I/O switching 2 MHz 0.26
I mA
DDIO Current V DD = 3.3V 8 MHz 1.01
C = 22pF
EXT 25 MHz 3.14
C = C + C
INT EXT
+ C 50 MHz 6.39
S
60 MHz 10.68
2 MHz 0.33
V = 3.3V
DD
8 MHz 1.29
C = 33pF
EXT
C = C INT + Cext 25 MHz 4.23
+ C
S
50 MHz 11.02
1. C is the PCB board capacitance including the pad pin. C = 7pF (estimated value).
S S
2. This test is performed by cutting the LQFP176 package pin (pad removal).
DS9405 Rev 13 113/240
197

<!-- Page 114 -->


| Peripheral |  | I ( Typ)(1) DD |  |  | Unit |
| --- | --- | --- | --- | --- | --- |
|  |  | Scale 1 | Scale 2 | Scale 3 |  |
| AHB1 (up to 180MHz) | GPIOA | 2.50 | 2.36 | 2.08 | µA/MHz |
|  | GPIOB | 2.56 | 2.36 | 2.08 |  |
|  | GPIOC | 2.44 | 2.29 | 2.00 |  |
|  | GPIOD | 2.50 | 2.36 | 2.08 |  |
|  | GPIOE | 2.44 | 2.29 | 2.00 |  |
|  | GPIOF | 2.44 | 2.29 | 2.00 |  |
|  | GPIOG | 2.39 | 2.22 | 2.00 |  |
|  | GPIOH | 2.33 | 2.15 | 1.92 |  |
|  | GPIOI | 2.39 | 2.22 | 2.00 |  |
|  | GPIOJ | 2.33 | 2.15 | 1.92 |  |
|  | GPIOK | 2.33 | 2.15 | 1.92 |  |
|  | OTG_HS+ULPI | 27.00 | 24.86 | 21.92 |  |
|  | CRC | 0.44 | 0.42 | 0.33 |  |
|  | BKPSRAM | 0.78 | 0.69 | 0.58 |  |
|  | DMA1 | 25.33 | 23.26 | 20.50 |  |
|  | DMA2 | 24.72 | 22.71 | 20.00 |  |
|  | DMA2D | 28.50 | 26.32 | 23.33 |  |
|  | ETH_MAC ETH_MAC_TX ETH_MAC_RX ETH_MAC_PTP | 21.56 | 20.07 | 17.75 |  |

Electrical characteristics STM32F427xx STM32F429xx
On-chip peripheral current consumption
The MCU is placed under the following conditions:
• At startup, all I/O pins are in analog input configuration.
• All peripherals are disabled unless otherwise mentioned.
• I/O compensation cell enabled.
• The ART accelerator is ON.
• Scale 1 mode selected, internal digital voltage V12 = 1.32V.
• HCLK is the system clock. f = f /4, and f = f /2.
PCLK1 HCLK PCLK2 HCLK
The given value is calculated by measuring the difference of current consumption
– with all the peripherals clocked off
– with only one peripheral clocked on
– f =180MHz (scale1 + overdrive ON), f =144MHz (scale 2),
HCLK HCLK
f =120MHz (scale 3)"
HCLK
• Ambient operating temperature is 25°C and V =3.3V.
DD
Table 35. Peripheral current consumption
I ( Typ)(1)
DD
Peripheral Unit
Scale 1 Scale 2 Scale 3
GPIOA 2.50 2.36 2.08
GPIOB 2.56 2.36 2.08
GPIOC 2.44 2.29 2.00
GPIOD 2.50 2.36 2.08
GPIOE 2.44 2.29 2.00
GPIOF 2.44 2.29 2.00
GPIOG 2.39 2.22 2.00
GPIOH 2.33 2.15 1.92
GPIOI 2.39 2.22 2.00
AHB1 GPIOJ 2.33 2.15 1.92
(up to µA/MHz
GPIOK 2.33 2.15 1.92
180MHz)
OTG_HS+ULPI 27.00 24.86 21.92
CRC 0.44 0.42 0.33
BKPSRAM 0.78 0.69 0.58
DMA1 25.33 23.26 20.50
DMA2 24.72 22.71 20.00
DMA2D 28.50 26.32 23.33
ETH_MAC
ETH_MAC_TX
21.56 20.07 17.75
ETH_MAC_RX
ETH_MAC_PTP
114/240 DS9405 Rev 13

<!-- Page 115 -->


| Peripheral |  | I ( Typ)(1) DD |  |  | Unit |
| --- | --- | --- | --- | --- | --- |
|  |  | Scale 1 | Scale 2 | Scale 3 |  |
| AHB2 (up to 180MHz) | OTG_FS | 25.67 | 26.67 | 23.58 | µA/MHz |
|  | DCMI | 3.72 | 3.40 | 3.00 |  |
|  | RNG | 2.28 | 2.36 | 2.17 |  |
| AHB3 (up to 180 MHz) | FMC | 21.39 | 19.79 | 17.50 | µA/MHz |
| Bus matrix(2) |  | 14.06 | 13.19 | 11.75 | µA/MHz |
| APB1 (up to 45MHz) | TIM2 | 17.56 | 16.42 | 14.47 | µA/MHz |
|  | TIM3 | 14.22 | 13.36 | 11.80 |  |
|  | TIM4 | 14.89 | 13.64 | 12.13 |  |
|  | TIM5 | 17.33 | 16.42 | 14.47 |  |
|  | TIM6 | 2.89 | 2.53 | 2.47 |  |
|  | TIM7 | 3.11 | 2.81 | 2.47 |  |
|  | TIM12 | 7.33 | 6.97 | 6.13 |  |
|  | TIM13 | 4.89 | 4.47 | 4.13 |  |
|  | TIM14 | 5.56 | 5.31 | 4.80 |  |
|  | PWR | 11.11 | 10.31 | 9.13 |  |
|  | USART2 | 4.22 | 3.92 | 3.47 |  |
|  | USART3 | 4.44 | 4.19 | 3.80 |  |
|  | UART4 | 4.00 | 3.92 | 3.47 |  |
|  | UART5 | 4.00 | 3.92 | 3.47 |  |
|  | UART7 | 4.00 | 3.92 | 3.47 |  |
|  | UART8 | 3.78 | 3.92 | 3.47 |  |
|  | I2C1 | 4.00 | 3.92 | 3.47 |  |
|  | I2C2 | 4.00 | 3.92 | 3.47 |  |
|  | I2C3 | 4.00 | 3.92 | 3.47 |  |
|  | SPI2(3) | 3.11 | 3.08 | 2.80 |  |
|  | SPI3(3) | 3.56 | 3.36 | 3.13 |  |
|  | I2S2 | 2.89 | 2.81 | 2.47 |  |
|  | I2S3 | 3.33 | 3.08 | 2.80 |  |
|  | CAN1 | 6.89 | 6.42 | 5.80 |  |
|  | CAN2 | 6.67 | 6.14 | 5.47 |  |
|  | DAC(4) | 2.89 | 2.25 | 2.13 |  |
|  | WWDG | 0.89 | 0.86 | 0.80 |  |

STM32F427xx STM32F429xx Electrical characteristics
Table 35. Peripheral current consumption (continued)
I ( Typ)(1)
DD
Peripheral Unit
Scale 1 Scale 2 Scale 3
OTG_FS 25.67 26.67 23.58
AHB2
(up to DCMI 3.72 3.40 3.00 µA/MHz
180MHz)
RNG 2.28 2.36 2.17
AHB3
(up to FMC 21.39 19.79 17.50 µA/MHz
180 MHz)
Bus matrix(2) 14.06 13.19 11.75 µA/MHz
TIM2 17.56 16.42 14.47
TIM3 14.22 13.36 11.80
TIM4 14.89 13.64 12.13
TIM5 17.33 16.42 14.47
TIM6 2.89 2.53 2.47
TIM7 3.11 2.81 2.47
TIM12 7.33 6.97 6.13
TIM13 4.89 4.47 4.13
TIM14 5.56 5.31 4.80
PWR 11.11 10.31 9.13
USART2 4.22 3.92 3.47
USART3 4.44 4.19 3.80
UART4 4.00 3.92 3.47
APB1
(up to UART5 4.00 3.92 3.47 µA/MHz
45MHz)
UART7 4.00 3.92 3.47
UART8 3.78 3.92 3.47
I2C1 4.00 3.92 3.47
I2C2 4.00 3.92 3.47
I2C3 4.00 3.92 3.47
SPI2(3) 3.11 3.08 2.80
SPI3(3) 3.56 3.36 3.13
I2S2 2.89 2.81 2.47
I2S3 3.33 3.08 2.80
CAN1 6.89 6.42 5.80
CAN2 6.67 6.14 5.47
DAC(4) 2.89 2.25 2.13
WWDG 0.89 0.86 0.80
DS9405 Rev 13 115/240
197

<!-- Page 116 -->


| Peripheral |  | I ( Typ)(1) DD |  |  | Unit |
| --- | --- | --- | --- | --- | --- |
|  |  | Scale 1 | Scale 2 | Scale 3 |  |
| APB2 (up to 90MHz) | SDIO | 8.11 | 8.75 | 7.83 | µA/MHz |
|  | TIM1 | 17.11 | 15.97 | 14.17 |  |
|  | TIM8 | 17.33 | 16.11 | 14.33 |  |
|  | TIM9 | 7.22 | 6.67 | 6.00 |  |
|  | TIM10 | 4.56 | 4.31 | 3.83 |  |
|  | TIM11 | 4.78 | 4.44 | 4.00 |  |
|  | ADC1(5) | 4.67 | 4.31 | 3.83 |  |
|  | ADC2(5) | 4.78 | 4.44 | 4.00 |  |
|  | ADC3(5) | 4.56 | 4.17 | 3.67 |  |
|  | SPI1 | 1.44 | 1.39 | 1.17 |  |
|  | USART1 | 4.00 | 3.75 | 3.33 |  |
|  | USART6 | 4.00 | 3.75 | 3.33 |  |
|  | SPI4 | 1.44 | 1.39 | 1.17 |  |
|  | SPI5 | 1.44 | 1.39 | 1.17 |  |
|  | SPI6 | 1.44 | 1.39 | 1.17 |  |
|  | SYSCFG | 0.78 | 0.69 | 0.67 |  |
|  | LCD_TFT | 39.89 | 37.22 | 33.17 |  |
|  | SAI1 | 3.78 | 3.47 | 3.17 |  |

Electrical characteristics STM32F427xx STM32F429xx
Table 35. Peripheral current consumption (continued)
I ( Typ)(1)
DD
Peripheral Unit
Scale 1 Scale 2 Scale 3
SDIO 8.11 8.75 7.83
TIM1 17.11 15.97 14.17
TIM8 17.33 16.11 14.33
TIM9 7.22 6.67 6.00
TIM10 4.56 4.31 3.83
TIM11 4.78 4.44 4.00
ADC1(5) 4.67 4.31 3.83
ADC2(5) 4.78 4.44 4.00
APB2 ADC3(5) 4.56 4.17 3.67
(up to µA/MHz
SPI1 1.44 1.39 1.17
90MHz)
USART1 4.00 3.75 3.33
USART6 4.00 3.75 3.33
SPI4 1.44 1.39 1.17
SPI5 1.44 1.39 1.17
SPI6 1.44 1.39 1.17
SYSCFG 0.78 0.69 0.67
LCD_TFT 39.89 37.22 33.17
SAI1 3.78 3.47 3.17
1. When the I/O compensation cell is ON, I typical value increases by 0.22mA.
DD
2. The BusMatrix is automatically active when at least one master is ON.
3. To enable an I2S peripheral, first set the I2SMOD bit and then the I2SE bit in the SPI_I2SCFGR register.
4. When the DAC is ON and EN1/2 bits are set in the DAC_CR register, add an additional power
consumption of 0.8 mA per DAC channel for the analog part.
5. When the ADC is ON (ADON bit set in the ADC_CR2 register), add an additional power consumption of
1.6 mA per ADC for the analog part.
116/240 DS9405 Rev 13

<!-- Page 117 -->


| Symbol | Parameter | Conditions | Typ(1) | Max(1) | Unit |
| --- | --- | --- | --- | --- | --- |
| t (2) WUSLEEP | Wakeup from Sleep | - | 6 | - | CPU clock cycle |
| t (2) WUSTOP | Wakeup from Stop mode with MR/LP regulator in normal mode | Main regulator is ON | 13.6 | - | µs |
|  |  | Main regulator is ON and Flash memory in Deep power down mode | 93 | 111 |  |
|  |  | Low power regulator is ON | 22 | 32 |  |
|  |  | Low power regulator is ON and Flash memory in Deep power down mode | 103 | 126 |  |
| t (2) WUSTOP | Wakeup from Stop mode with MR/LP regulator in Under-drive mode | Main regulator in under-drive mode (Flash memory in Deep power-down mode) | 105 | 128 |  |
|  |  | Low power regulator in under-drive mode (Flash memory in Deep power-down mode ) | 125 | 155 |  |
| tWUSTDBY (2)(3) | Wakeup from Standby mode | - | 318 | 412 |  |

STM32F427xx STM32F429xx Electrical characteristics
6.3.8 Wake-up time from low-power modes
The wake-up times given in Table36 are measured starting from the wake-up event trigger
up to the first instruction executed by the CPU:
• For Stop or Sleep modes: the wake-up event is WFE.
• WKUP (PA0) pin is used to wake up from Standby, Stop, and Sleep modes.
All timings are derived from tests performed under ambient temperature and V =3.3V.
DD
Table 36. Low-power mode wakeup timings
Symbol Parameter Conditions Typ(1) Max(1) Unit
CPU
t (2) Wakeup from Sleep - 6 - clock
WUSLEEP
cycle
Main regulator is ON 13.6 -
Main regulator is ON and Flash
93 111
memory in Deep power down mode
Wakeup from Stop mode
t (2) with MR/LP regulator in
WUSTOP
normal mode Low power regulator is ON 22 32
Low power regulator is ON and Flash
103 126
memory in Deep power down mode
µs
Main regulator in under-drive mode
(Flash memory in Deep power-down 105 128
mode)
Wakeup from Stop mode
t WUSTOP (2) with MR/LP regulator in Low power regulator in under-drive
Under-drive mode mode
125 155
(Flash memory in Deep power-down
mode )
tWUSTDBY Wakeup from Standby
- 318 412
(2)(3) mode
1. Evaluated by characterization.
2. The wake-up times are measured from the wake-up event to the point in which the application code reads the first
3. t maximum value is given at –40°C.
WUSTDBY
DS9405 Rev 13 117/240
197

<!-- Page 118 -->


| Symbol | Parameter | Conditions | Min | Typ | Max | Unit |
| --- | --- | --- | --- | --- | --- | --- |
| f HSE_ext | External user clock source frequency(1) | - | 1 | - | 50 | MHz |
| V HSEH | OSC_IN input pin high level voltage |  | 0.7V DD | - | V DD | V |
| V HSEL | OSC_IN input pin low level voltage |  | V SS | - | 0.3V DD |  |
| t w(HSE) t w(HSE) | OSC_IN high or low time(1) |  | 5 | - | - | ns |
| t r(HSE) t f(HSE) | OSC_IN rise or fall time(1) |  | - | - | 10 |  |
| C in(HSE) | OSC_IN input capacitance(1) | - | - | 5 | - | pF |
| DuCy (HSE) | Duty cycle | - | 45 | - | 55 | % |
| I L | OSC_IN Input leakage current | V ≤ V ≤ V SS IN DD | - | - | ±1 | µA |

Electrical characteristics STM32F427xx STM32F429xx
6.3.9 External clock source characteristics
High-speed external user clock generated from an external source
In bypass mode the HSE oscillator is switched off and the input pin is a standard I/O. The
external clock signal has to respect the Table57: I/O static characteristics. However, the
recommended clock input waveform is shown in Figure27.
The characteristics given in Table37 result from tests performed using a high-speed
external clock source, and under ambient temperature and supply voltage conditions
summarized in Table17.
Table 37. High-speed external user clock characteristics
Symbol Parameter Conditions Min Typ Max Unit
External user clock source
f 1 - 50 MHz
HSE_ext frequency(1)
V OSC_IN input pin high level voltage 0.7V - V
HSEH DD DD
V
V OSC_IN input pin low level voltage V - 0.3V
HSEL - SS DD
t
w(HSE) OSC_IN high or low time(1) 5 - -
t
w(HSE)
ns
t
r(HSE) OSC_IN rise or fall time(1) - - 10
t
f(HSE)
C OSC_IN input capacitance(1) - - 5 - pF
in(HSE)
DuCy Duty cycle - 45 - 55 %
(HSE)
I OSC_IN Input leakage current V ≤ V ≤ V - - ±1 µA
L SS IN DD
1. Specified by design.
118/240 DS9405 Rev 13

<!-- Page 119 -->


| Symbol | Parameter | Conditions | Min | Typ | Max | Unit |
| --- | --- | --- | --- | --- | --- | --- |
| f LSE_ext | User External clock source frequency(1) | - | - | 32.768 | 1000 | kHz |
| V LSEH | OSC32_IN input pin high level voltage |  | 0.7V DD | - | V DD | V |
| V LSEL | OSC32_IN input pin low level voltage |  | V SS | - | 0.3V DD |  |
| t w(LSE) t f(LSE) | OSC32_IN high or low time(1) |  | 450 | - | - | ns |
| t r(LSE) t f(LSE) | OSC32_IN rise or fall time(1) |  | - | - | 50 |  |
| C in(LSE) | OSC32_IN input capacitance(1) | - | - | 5 | - | pF |
| DuCy (LSE) | Duty cycle | - | 30 | - | 70 | % |
| I L | OSC32_IN Input leakage current | V ≤ V ≤ V SS IN DD | - | - | ±1 | µA |


|  |  |  |  |  |  |  |  |
| --- | --- | --- | --- | --- | --- | --- | --- |
|  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |

STM32F427xx STM32F429xx Electrical characteristics
Low-speed external user clock generated from an external source
In bypass mode the LSE oscillator is switched off and the input pin is a standard I/O. The
external clock signal has to respect the Table57: I/O static characteristics. However, the
recommended clock input waveform is shown in Figure28.
The characteristics given in Table38 result from tests performed using a low-speed external
clock source, and under ambient temperature and supply voltage conditions summarized in
Table17.
Table 38. Low-speed external user clock characteristics
Symbol Parameter Conditions Min Typ Max Unit
User External clock source
f - 32.768 1000 kHz
LSE_ext frequency(1)
OSC32_IN input pin high level
V 0.7V - V
LSEH voltage DD DD V
V OSC32_IN input pin low level voltage - V - 0.3V
LSEL SS DD
t
w(LSE) OSC32_IN high or low time(1) 450 - -
t
f(LSE)
ns
t
r(LSE) OSC32_IN rise or fall time(1) - - 50
t
f(LSE)
C OSC32_IN input capacitance(1) - - 5 - pF
in(LSE)
DuCy Duty cycle - 30 - 70 %
(LSE)
I OSC32_IN Input leakage current V ≤ V ≤ V - - ±1 µA
L SS IN DD
1. Specified by design.
Figure 27. High-speed external clock source AC timing diagram
VHSEH
90%
10%
VHSEL
tr(HSE) tf(HSE) tW(HSE) tW(HSE) t
THSE
External
fHSE_ext
IL
clock source OSC_IN
STM32F
ai17528
DS9405 Rev 13 119/240
197

<!-- Page 120 -->


|  |  |  |  |  |  |  |  |
| --- | --- | --- | --- | --- | --- | --- | --- |
|  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |


|  |  |
| --- | --- |


| Symbol | Parameter | Conditions | Min | Typ | Max | Unit |
| --- | --- | --- | --- | --- | --- | --- |
| f OSC_IN | Oscillator frequency | - | 4 | - | 26 | MHz |
| R F | Feedback resistor | - | - | 200 | - | kΩ |
| I DD | HSE current consumption | V =3.3V, DD ESR= 30Ω, C =5pF@25MHz L | - | 450 | - | µA |
|  |  | V =3.3V, DD ESR= 30Ω, C =10pF@25MHz L | - | 530 | - |  |
| ACC (2) HSE | HSE accuracy | - | −500 | - | 500 | ppm |
| G _crit_max m | Maximum critical crystal g m | Startup | - | - | 1 | mA/V |
| t (3) SU(HSE | Startup time | V is stabilized DD | - | 2 | - | ms |

Electrical characteristics STM32F427xx STM32F429xx
Figure 28. Low-speed external clock source AC timing diagram
VLSEH
90%
10%
VLSEL
tr(LSE) tf(LSE) tW(LSE) tW(LSE) t
TLSE
External
fLSE_ext
OSC32_IN
IL
clock source
STM32F
ai17529
High-speed external clock generated from a crystal/ceramic resonator
The high-speed external (HSE) clock can be supplied with a 4 to 26 MHz crystal/ceramic
resonator oscillator. All the information in this paragraph is based on the characterization
results obtained with typical external components specified in Table39. In the application,
the resonator and the load capacitors have to be placed as close as possible to the
oscillator pins to minimize output distortion and startup stabilization time. Refer to the crystal
resonator manufacturer for more details on the resonator characteristics (frequency,
package, accuracy).
Table 39. HSE 4-26 MHz oscillator characteristics (1)
Symbol Parameter Conditions Min Typ Max Unit
f Oscillator frequency - 4 - 26 MHz
OSC_IN
R Feedback resistor - - 200 - kΩ
F
V =3.3V,
DD
ESR= 30Ω, - 450 -
C =5pF@25MHz
L
I HSE current consumption µA
DD
V =3.3V,
DD
ESR= 30Ω, - 530 -
C =10pF@25MHz
L
ACC (2) HSE accuracy - −500 - 500 ppm
HSE
G _crit_max Maximum critical crystal g Startup - - 1 mA/V
m m
t (3) Startup time V is stabilized - 2 - ms
SU(HSE DD
1. Specified by design.
2. This parameter depends on the crystal used in the application. The minimum and maximum values must
be respected to comply with USB standard specifications.
3. t is the startup time measured from the moment that it is enabled (by software) until a stabilized 8
SU(HSE)
MHz oscillation is reached. This value is based on characterization and not tested in production. It is
measured for a standard crystal resonator and it can vary significantly with the crystal manufacturer.
120/240 DS9405 Rev 13

<!-- Page 121 -->


|  |  |  |
| --- | --- | --- |
|  | RF | Bias controlled gain |
|  |  |  |


| 8 MHz resonator |  |  |
| --- | --- | --- |
|  |  |  |


|  |  |
| --- | --- |


| Symbol | Parameter | Conditions | Min | Typ | Max | Unit |
| --- | --- | --- | --- | --- | --- | --- |
| R F | Feedback resistor | - | - | 18.4 | - | MΩ |
| I DD | LSE current consumption | - | - | - | 1 | µA |
| ACC (2) LSE | LSE accuracy | - | −500 | - | 500 | ppm |
| G _crit_max m | Maximum critical crystal g m | Startup | - | - | 0.56 | µA/V |
| t (3) SU(LSE) | startup time | V is stabilized DD | - | 2 | - | s |

STM32F427xx STM32F429xx Electrical characteristics
For C and C , it is recommended to use high-quality external ceramic capacitors in the
L1 L2
5pF to 25pF range (typ.), designed for high-frequency applications, and selected to match
the requirements of the crystal or resonator (see Figure29). C and C are usually the
L1 L2
same size. The crystal manufacturer typically specifies a load capacitance, which is the
series combination of C and C . PCB and MCU pin capacitance must be included (10pF
L1 L2
can be used as a rough estimate of the combined pin and board capacitance) when sizing
C and C .
L1 L2
Note: For information on selecting the crystal, refer to the application note AN2867 “Oscillator
design guide for ST microcontrollers” available from the ST website www.st.com.
Figure 29. Typical application with an 8 MHz crystal
Resonator with
integrated capacitors
CL1
OSC_IN fHSE
Bias
8 MHz
RF controlled
resonator
gain
CL2 REXT (1) OSC_OUT STM32F
ai17530
1. R value depends on the crystal characteristics.
EXT
Low-speed external clock generated from a crystal/ceramic resonator
The low-speed external (LSE) clock can be supplied with a 32.768 kHz crystal/ceramic
resonator oscillator. All the information given in this paragraph are based on
characterization results obtained with typical external components specified in Table40. In
the application, the resonator and the load capacitors have to be placed as close as
possible to the oscillator pins to minimize output distortion and startup stabilization time.
Refer to the crystal resonator manufacturer for more details on the resonator characteristics
(frequency, package, accuracy).
Table 40. LSE oscillator characteristics (f = 32.768 kHz) (1)
LSE
Symbol Parameter Conditions Min Typ Max Unit
R Feedback resistor - - 18.4 - MΩ
F
I LSE current consumption - - - 1 µA
DD
ACC (2) LSE accuracy - −500 - 500 ppm
LSE
G _crit_max Maximum critical crystal g Startup - - 0.56 µA/V
m m
t (3) startup time V is stabilized - 2 - s
SU(LSE) DD
1. Specified by design.
2. This parameter depends on the crystal used in the application. Refer to application note AN2867.
3. t is the startup time measured from the moment that it is enabled (by software) to a stabilized
SU(LSE)
32.768kHz oscillation is reached. This value is based on characterization and not tested in production. It is
measured for a standard crystal resonator and it can vary significantly with the crystal manufacturer.
Note: For information on selecting the crystal, refer to the application note AN2867 “Oscillator
design guide for ST microcontrollers” available from the ST website www.st.com.
DS9405 Rev 13 121/240
197

<!-- Page 122 -->


|  |  |  |
| --- | --- | --- |
| 32.768 kHz resonator |  |  |
|  |  |  |


|  |  |  |
| --- | --- | --- |
|  | RF | Bias controlled gain |
|  |  |  |


| Symbol | Parameter | Conditions | Min | Typ | Max | Unit |
| --- | --- | --- | --- | --- | --- | --- |
| f HSI | Frequency | - | - | 16 | - | MHz |
| ACC HSI | HSI user-trimming step (2) | - | - | - | 1 | % |
|  | Accuracy of the HSI oscillator | T = –40 to 105°C(3) A | −8 | - | 4.5 | % |
|  |  | T = –10 to 85°C(3) A | −4 | - | 4 | % |
|  |  | T = 25°C(4) A | −1 | - | 1 | % |
| t (2) su(HSI) | HSI oscillator startup time | - | - | 2.2 | 4 | µs |
| I (2) DD(HSI) | HSI oscillator power consumption | - | - | 60 | 80 | µA |

Electrical characteristics STM32F427xx STM32F429xx
Figure 30. Typical application with a 32.768 kHz crystal
Resonator with
integrated capacitors
CL1
OSC32_IN fLSE
Bias
32.768 kHz RF controlled
resonator
gain
OSC32_OUT STM32F
CL2
ai17531
6.3.10 Internal clock source characteristics
The parameters given in Table41 and Table42 are derived from tests performed under
ambient temperature and V supply voltage conditions summarized in Table17.
DD
High-speed internal (HSI) RC oscillator
Table 41. HSI oscillator characteristics (1)
Symbol Parameter Conditions Min Typ Max Unit
f Frequency - - 16 - MHz
HSI
HSI user-trimming step (2) - - - 1 %
T = –40 to 105°C(3) −8 - 4.5 %
A
ACC
HSI Accuracy of the HSI oscillator T = –10 to 85°C(3) −4 - 4 %
A
T = 25°C(4) −1 - 1 %
A
t (2) HSI oscillator startup time - - 2.2 4 µs
su(HSI)
HSI oscillator power
I (2) - - 60 80 µA
DD(HSI) consumption
1. VDD = 3.3 V, PLL OFF, T = –40 to 125 °C unless otherwise specified.
A
2. Specified by design.
3. Evaluated by characterization results.
4. Factory calibrated, parts not soldered.
122/240 DS9405 Rev 13

<!-- Page 123 -->


|  |
| --- |
| 6 4 2 )%( 0 -40 0 25 55 85 105 125 ISHCCA TA (°C) -2 -4 Min Max Typical -6 -8 MSv41925V1 |


| Symbol | Parameter | Min | Typ | Max | Unit |
| --- | --- | --- | --- | --- | --- |
| f (2) LSI | Frequency | 17 | 32 | 47 | kHz |
| t (3) su(LSI) | LSI oscillator startup time | - | 15 | 40 | µs |
| I (3) DD(LSI) | LSI oscillator power consumption | - | 0.4 | 0.6 | µA |

STM32F427xx STM32F429xx Electrical characteristics
Figure 31. ACCHSI accuracy versus temperature
6
4
2
0
-40 0 25 55 85 105 125
-2
-4
-6
-8
MSv41925V1
1. Evaluated by characterization results.
Low-speed internal (LSI) RC oscillator
Table 42. LSI oscillator characteristics (1)
Symbol Parameter Min Typ Max Unit
f (2) Frequency 17 32 47 kHz
LSI
t (3) LSI oscillator startup time - 15 40 µs
su(LSI)
I (3) LSI oscillator power consumption - 0.4 0.6 µA
DD(LSI)
1. V = 3 V, T = –40 to 105 °C unless otherwise specified.
DD A
2. Evaluated by characterization results.
3. Specified by design.
DS9405 Rev 13 123/240
197
)%(
ISHCCA TA (°C)
Min Max Typical

<!-- Page 124 -->


|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


| Symbol | Parameter | Conditions | Min | Typ | Max | Unit |
| --- | --- | --- | --- | --- | --- | --- |
| f PLL_IN | PLL input clock(1) | - | 0.95(2) | 1 | 2.10 | MHz |
| f PLL_OUT | PLL multiplier output clock | - | 24 | - | 180 | MHz |
| f PLL48_OUT | 48 MHz PLL multiplier output clock | - | - | 48 | 75 | MHz |
| f VCO_OUT | PLL VCO output | - | 100 | - | 432 | MHz |
| t LOCK | PLL lock time | VCO freq = 100MHz | 75 | - | 200 | µs |
|  |  | VCO freq = 432MHz | 100 | - | 300 |  |

Electrical characteristics STM32F427xx STM32F429xx
Figure 32. ACC versus temperature
LSI
50
40
30
20
10
0
-10
-20
-30
-40
-45 -35 -25 -15 -5 5 15 25 35 45 55 65 75 85 95 105
MS19013V1
6.3.11 PLL characteristics
The parameters given in Table43 and Table44 are derived from tests performed under
temperature and V supply voltage conditions summarized in Table17.
DD
124/240 DS9405 Rev 13
)%(
noitaived
dezilamroN
max
avg
min
Temperature (°C)
Table 43. Main PLL characteristics
Symbol Parameter Conditions Min Typ Max Unit
f PLL input clock(1) - 0.95(2) 1 2.10 MHz
PLL_IN
f PLL multiplier output clock - 24 - 180 MHz
PLL_OUT
48 MHz PLL multiplier output
f - - 48 75 MHz
PLL48_OUT clock
f PLL VCO output - 100 - 432 MHz
VCO_OUT
VCO freq = 100MHz 75 - 200
t PLL lock time µs
LOCK
VCO freq = 432MHz 100 - 300

<!-- Page 125 -->


| Symbol | Parameter | Conditions |  | Min | Typ | Max | Unit |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Jitter(3) | Cycle-to-cycle jitter | System clock 120MHz | RMS | - | 25 | - | ps |
|  |  |  | peak to peak | - | ±150 | - |  |
|  | Period Jitter |  | RMS | - | 15 | - |  |
|  |  |  | peak to peak | - | ±200 | - |  |
|  | Main clock output (MCO) for RMII Ethernet | Cycle to cycle at 50 MHz on 1000 samples |  | - | 32 | - |  |
|  | Main clock output (MCO) for MII Ethernet | Cycle to cycle at 25 MHz on 1000 samples |  | - | 40 | - |  |
|  | Bit Time CAN jitter | Cycle to cycle at 1 MHz on 1000 samples |  | - | 330 | - |  |
| I (4) DD(PLL) | PLL power consumption on VDD | VCO freq = 100MHz VCO freq = 432MHz |  | 0.15 0.45 | - | 0.40 0.75 | mA |
| I (4) DDA(PLL) | PLL power consumption on VDDA | VCO freq = 100MHz VCO freq = 432MHz |  | 0.30 0.55 | - | 0.40 0.85 | mA |


| Symbol | Parameter | Conditions |  | Min | Typ | Max | Unit |
| --- | --- | --- | --- | --- | --- | --- | --- |
| f PLLI2S_IN | PLLI2S input clock(1) | - |  | 0.95(2) | 1 | 2.10 | MHz |
| f PLLI2S_OUT | PLLI2S multiplier output clock | - |  | - | - | 216 | MHz |
| f VCO_OUT | PLLI2S VCO output | - |  | 100 | - | 432 | MHz |
| t LOCK | PLLI2S lock time | VCO freq = 100MHz |  | 75 | - | 200 | µs |
|  |  | VCO freq = 432MHz |  | 100 | - | 300 |  |
| Jitter(3) | Master I2S clock jitter | Cycle to cycle at 12.288MHz on 48KHz period, N=432, R=5 | RMS | - | 90 | - | - |
|  |  |  | peak to peak | - | ±280 | - | ps |
|  |  | Average frequency of 12.288MHz N = 432, R = 5 on 1000 samples |  | - | 90 | - | ps |
|  | WS I2S clock jitter | Cycle to cycle at 48KHz on 1000 samples |  | - | 400 | - | ps |

STM32F427xx STM32F429xx Electrical characteristics
Table 43. Main PLL characteristics (continued)
Symbol Parameter Conditions Min Typ Max Unit
RMS - 25 -
Cycle-to-cycle jitter peak
to - ±150 -
System clock peak
120MHz
RMS - 15 -
Period Jitter peak
to - ±200 -
Jitter(3) ps
peak
Main clock output (MCO) for Cycle to cycle at 50 MHz
- 32 -
RMII Ethernet on 1000 samples
Main clock output (MCO) for MII Cycle to cycle at 25 MHz
- 40 -
Ethernet on 1000 samples
Cycle to cycle at 1 MHz
Bit Time CAN jitter - 330 -
on 1000 samples
VCO freq = 100MHz 0.15 0.40
I (4) PLL power consumption on VDD - mA
DD(PLL) VCO freq = 432MHz 0.45 0.75
PLL power consumption on VCO freq = 100MHz 0.30 0.40
I (4) - mA
DDA(PLL) VDDA VCO freq = 432MHz 0.55 0.85
1. Use the appropriate division factor M to obtain the specified PLL input clock values. The M factor is shared between PLL
and PLLI2S.
2. Specified by design.
3. The use of 2 PLLs in parallel could degrade the Jitter up to +30%.
4. Evaluated by characterization.
Table 44. PLLI2S (audio PLL) characteristics
Symbol Parameter Conditions Min Typ Max Unit
f PLLI2S input clock(1) - 0.95(2) 1 2.10 MHz
PLLI2S_IN
f PLLI2S multiplier output clock - - - 216 MHz
PLLI2S_OUT
f PLLI2S VCO output - 100 - 432 MHz
VCO_OUT
VCO freq = 100MHz 75 - 200
t PLLI2S lock time µs
LOCK
VCO freq = 432MHz 100 - 300
Cycle to cycle at RMS - 90 - -
12.288MHz on
peak
48KHz period,
to - ±280 - ps
N=432, R=5
peak
Master I2S clock jitter
Average frequency of
Jitter(3)
12.288MHz
- 90 - ps
N = 432, R = 5
on 1000 samples
Cycle to cycle at 48KHz
WS I2S clock jitter - 400 - ps
on 1000 samples
DS9405 Rev 13 125/240
197

<!-- Page 126 -->


| Symbol | Parameter | Conditions | Min | Typ | Max | Unit |
| --- | --- | --- | --- | --- | --- | --- |
| I (4) DD(PLLI2S) | PLLI2S power consumption on V DD | VCO freq = 100MHz VCO freq = 432MHz | 0.15 0.45 | - | 0.40 0.75 | mA |
| I (4) DDA(PLLI2S) | PLLI2S power consumption on V DDA | VCO freq = 100MHz VCO freq = 432MHz | 0.30 0.55 | - | 0.40 0.85 | mA |


| Symbol | Parameter | Conditions |  | Min | Typ | Max | Unit |
| --- | --- | --- | --- | --- | --- | --- | --- |
| f PLLSAI_IN | PLLSAI input clock(1) | - |  | 0.95(2) | 1 | 2.10 | MHz |
| f PLLSAI_OUT | PLLSAI multiplier output clock | - |  | - | - | 216 | MHz |
| f VCO_OUT | PLLSAI VCO output | - |  | 100 | - | 432 | MHz |
| t LOCK | PLLSAI lock time | VCO freq = 100MHz |  | 75 | - | 200 | µs |
|  |  | VCO freq = 432MHz |  | 100 | - | 300 |  |
| Jitter(3) | Main SAI clock jitter | Cycle to cycle at 12.288MHz on 48KHz period, N=432, R=5 | RMS | - | 90 | - | - |
|  |  |  | peak to peak | - | ±280 | - | ps |
|  |  | Average frequency of 12.288MHz N = 432, R = 5 on 1000 samples |  | - | 90 | - | ps |
|  | FS clock jitter | Cycle to cycle at 48KHz on 1000 samples |  | - | 400 | - | ps |
| I (4) DD(PLLSAI) | PLLSAI power consumption on V DD | VCO freq = 100MHz VCO freq = 432MHz |  | 0.15 0.45 | - | 0.40 0.75 | mA |
| I (4) DDA(PLLSAI) | PLLSAI power consumption on V DDA | VCO freq = 100MHz VCO freq = 432MHz |  | 0.30 0.55 | - | 0.40 0.85 | mA |

Electrical characteristics STM32F427xx STM32F429xx
Table 44. PLLI2S (audio PLL) characteristics (continued)
Symbol Parameter Conditions Min Typ Max Unit
PLLI2S power consumption on VCO freq = 100MHz 0.15 0.40
I (4) - mA
DD(PLLI2S) V DD VCO freq = 432MHz 0.45 0.75
PLLI2S power consumption on VCO freq = 100MHz 0.30 0.40
I (4) - mA
DDA(PLLI2S) V DDA VCO freq = 432MHz 0.55 0.85
1. Use the appropriate division factor M to have the specified PLL input clock values.
2. Specified by design.
3. Value given with the main PLL running.
4. Evaluated by characterization.
Table 45. PLLISAI (audio and LCD-TFT PLL) characteristics
Symbol Parameter Conditions Min Typ Max Unit
f PLLSAI input clock(1) - 0.95(2) 1 2.10 MHz
PLLSAI_IN
f PLLSAI multiplier output clock - - - 216 MHz
PLLSAI_OUT
f PLLSAI VCO output - 100 - 432 MHz
VCO_OUT
VCO freq = 100MHz 75 - 200
t PLLSAI lock time µs
LOCK
VCO freq = 432MHz 100 - 300
Cycle to cycle at RMS - 90 - -
12.288MHz on
peak
48KHz period,
to - ±280 - ps
N=432, R=5
peak
Main SAI clock jitter
Average frequency of
Jitter(3)
12.288MHz
- 90 - ps
N = 432, R = 5
on 1000 samples
Cycle to cycle at 48KHz
FS clock jitter - 400 - ps
on 1000 samples
PLLSAI power consumption on VCO freq = 100MHz 0.15 0.40
I (4) - mA
DD(PLLSAI) V DD VCO freq = 432MHz 0.45 0.75
PLLSAI power consumption on VCO freq = 100MHz 0.30 0.40
I (4) - mA
DDA(PLLSAI) V DDA VCO freq = 432MHz 0.55 0.85
1. Use the appropriate division factor M to have the specified PLL input clock values.
2. Specified by design.
3. Value given with the main PLL running.
4. Evaluated by characterization.
126/240 DS9405 Rev 13

<!-- Page 127 -->


| Symbol | Parameter | Min | Typ | Max(1) | Unit |
| --- | --- | --- | --- | --- | --- |
| f Mod | Modulation frequency | - | - | 10 | KHz |
| md | Peak modulation depth | 0.25 | - | 2 | % |
| MODEPER * INCSTEP | - | - | - | 215−1 | - |

STM32F427xx STM32F429xx Electrical characteristics
6.3.12 PLL spread spectrum clock generation (SSCG) characteristics
The spread spectrum clock generation (SSCG) feature allows the decrease of
electromagnetic interferences (see Table52: EMI characteristics for fHSE= 25 MHz and
fCPU= 168 MHz). It is available only on the main PLL.
Table 46. SSCG parameters constraint
Symbol Parameter Min Typ Max(1) Unit
f Modulation frequency - - 10 KHz
Mod
md Peak modulation depth 0.25 - 2 %
MODEPER * INCSTEP - - - 215−1 -
1. Specified by design.
Equation 1
The frequency modulation period (MODEPER) is given by the equation below:
MODEPER = round[f ⁄ (4× f )]
PLL_IN Mod
f and f must be expressed in Hz.
PLL_IN Mod
As an example:
If f = 1MHz, and f = 1kHz, the modulation depth (MODEPER) is given by
PLL_IN MOD
equation 1:
MODEPER = round[10 6⁄ (4× 10 3)] = 250
Equation 2
Equation 2 allows to calculate the increment step (INCSTEP):
INCSTEP = round[((2 15 –1)× md× PLLN)⁄ (100× 5× MODEPER)]
f must be expressed in MHz.
VCO_OUT
With a modulation depth (md)=±2% (4% peak to peak), and PLLN=240 (in MHz):
INCSTEP = round[((2 15 –1)× 2× 240)⁄ (100× 5× 250)] = 126md(quantitazed)%
An amplitude quantization error may be generated because the linear modulation profile is
obtained by taking the quantized values (rounded to the nearest integer) of MODPER and
INCSTEP. As a result, the achieved modulation depth is quantized. The percentage
quantized modulation depth is given by the following formula:
md % = (MODEPER× INCSTEP× 100× 5)⁄ ((2 15 –1)× PLLN)
quantized
As a result:
md % = (250× 126× 100× 5)⁄ ((2 15 –1)× 240) = 2.002%(peak)
quantized
DS9405 Rev 13 127/240
197

<!-- Page 128 -->


|  | md |  |
| --- | --- | --- |
|  |  | md |


|  | 2xmd |
| --- | --- |

Electrical characteristics STM32F427xx STM32F429xx
Figure33 and Figure34 show the main PLL output clock waveforms in center spread and
down spread modes, where:
F0 is f nominal.
PLL_OUT
T is the modulation period.
mode
md is the modulation depth.
Figure 33. PLL output clock waveforms in center spread mode
Frequency (PLL_OUT)
md
F0
md
Time
tmode 2xtmode
ai17291
Figure 34. PLL output clock waveforms in down spread mode
Frequency (PLL_OUT)
F0
2xmd
Time
tmode 2xtmode
ai17292b
128/240 DS9405 Rev 13

<!-- Page 129 -->


| Symbol | Parameter | Conditions | Min | Typ | Max | Unit |
| --- | --- | --- | --- | --- | --- | --- |
| I DD | Supply current | Write / Erase 8-bit mode, V = 1.7V DD | - | 5 | - | mA |
|  |  | Write / Erase 16-bit mode, V = 2.1V DD | - | 8 | - |  |
|  |  | Write / Erase 32-bit mode, V = 3.3V DD | - | 12 | - |  |


| Symbol | Parameter | Conditions | Min(1) | Typ | Max(1) | Unit |
| --- | --- | --- | --- | --- | --- | --- |
| t prog | Word programming time | Program/erase parallelism (PSIZE) = x 8/16/32 | - | 16 | 100(2) | µs |
| t ERASE16KB | Sector (16 KB) erase time | Program/erase parallelism (PSIZE) = x 8 | - | 400 | 800 | ms |
|  |  | Program/erase parallelism (PSIZE) = x 16 | - | 300 | 600 |  |
|  |  | Program/erase parallelism (PSIZE) = x 32 | - | 250 | 500 |  |
| t ERASE64KB | Sector (64 KB) erase time | Program/erase parallelism (PSIZE) = x 8 | - | 1200 | 2400 | ms |
|  |  | Program/erase parallelism (PSIZE) = x 16 | - | 700 | 1400 |  |
|  |  | Program/erase parallelism (PSIZE) = x 32 | - | 550 | 1100 |  |
| t ERASE128KB | Sector (128 KB) erase time | Program/erase parallelism (PSIZE) = x 8 | - | 2 | 4 | s |
|  |  | Program/erase parallelism (PSIZE) = x 16 | - | 1.3 | 2.6 |  |
|  |  | Program/erase parallelism (PSIZE) = x 32 | - | 1 | 2 |  |
| t ME | Mass erase time | Program/erase parallelism (PSIZE) = x 8 | - | 16 | 32 | s |
|  |  | Program/erase parallelism (PSIZE) = x 16 | - | 11 | 22 |  |
|  |  | Program/erase parallelism (PSIZE) = x 32 | - | 8 | 16 |  |

STM32F427xx STM32F429xx Electrical characteristics
6.3.13 Memory characteristics
Flash memory
The characteristics are given at TA = –40 to 105 °C unless otherwise specified.
The devices are shipped to customers with the flash memory erased.
Table 47. Flash memory characteristics
Symbol Parameter Conditions Min Typ Max Unit
Write / Erase 8-bit mode, V = 1.7V - 5 -
DD
I Supply current Write / Erase 16-bit mode, V = 2.1V - 8 - mA
DD DD
Write / Erase 32-bit mode, V = 3.3V - 12 -
DD
Table 48. Flash memory programming
Symbol Parameter Conditions Min(1) Typ Max(1) Unit
Program/erase parallelism
t Word programming time - 16 100(2) µs
prog (PSIZE) = x 8/16/32
Program/erase parallelism
- 400 800
(PSIZE) = x 8
Program/erase parallelism
t Sector (16 KB) erase time - 300 600 ms
ERASE16KB (PSIZE) = x 16
Program/erase parallelism
- 250 500
(PSIZE) = x 32
Program/erase parallelism
- 1200 2400
(PSIZE) = x 8
Program/erase parallelism
t Sector (64 KB) erase time - 700 1400 ms
ERASE64KB (PSIZE) = x 16
Program/erase parallelism
- 550 1100
(PSIZE) = x 32
Program/erase parallelism
- 2 4
(PSIZE) = x 8
Program/erase parallelism
t Sector (128 KB) erase time - 1.3 2.6 s
ERASE128KB (PSIZE) = x 16
Program/erase parallelism
- 1 2
(PSIZE) = x 32
Program/erase parallelism
- 16 32
(PSIZE) = x 8
Program/erase parallelism
t Mass erase time - 11 22 s
ME (PSIZE) = x 16
Program/erase parallelism
- 8 16
(PSIZE) = x 32
DS9405 Rev 13 129/240
197

<!-- Page 130 -->


| Symbol | Parameter | Conditions | Min(1) | Typ | Max(1) | Unit |
| --- | --- | --- | --- | --- | --- | --- |
| t BE | Bank erase time | Program/erase parallelism (PSIZE) = x 8 | - | 16 | 32 | s |
|  |  | Program/erase parallelism (PSIZE) = x 16 | - | 11 | 22 |  |
|  |  | Program/erase parallelism (PSIZE) = x 32 | - | 8 | 16 |  |
| V prog | Programming voltage | 32-bit program operation | 2.7 | - | 3.6 | V |
|  |  | 16-bit program operation | 2.1 | - | 3.6 | V |
|  |  | 8-bit program operation | 1.7 | - | 3.6 | V |


| Symbol | Parameter | Conditions | Min(1) | Typ | Max(1) | Unit |
| --- | --- | --- | --- | --- | --- | --- |
| t prog | Double word programming | T = 0 to +40°C A V = 3.3V DD V = 8.5V PP | - | 16 | 100(2) | µs |
| t ERASE16KB | Sector (16 KB) erase time |  | - | 230 | - | ms |
| t ERASE64KB | Sector (64 KB) erase time |  | - | 490 | - |  |
| t ERASE128KB | Sector (128 KB) erase time |  | - | 875 | - |  |
| t ME | Mass erase time |  | - | 6.9 | - | s |
| t BE | Bank erase time | - | - | 6.9 | - | s |
| V prog | Programming voltage | - | 2.7 | - | 3.6 | V |
| V PP | V voltage range PP | - | 7 | - | 9 | V |
| I PP | Minimum current sunk on the V pin PP | - | 10 | - | - | mA |
| t (3) VPP | Cumulative time during which V is applied PP | - | - | - | 1 | hour |

Electrical characteristics STM32F427xx STM32F429xx
Table 48. Flash memory programming (continued)
Symbol Parameter Conditions Min(1) Typ Max(1) Unit
Program/erase parallelism
- 16 32
(PSIZE) = x 8
Program/erase parallelism
t Bank erase time - 11 22 s
BE (PSIZE) = x 16
Program/erase parallelism
- 8 16
(PSIZE) = x 32
32-bit program operation 2.7 - 3.6 V
V Programming voltage 16-bit program operation 2.1 - 3.6 V
prog
8-bit program operation 1.7 - 3.6 V
1. Evaluated by characterization.
2. The maximum programming time is measured after 100 K erase operations.
Table 49. Flash memory programming with V
PP
Symbol Parameter Conditions Min(1) Typ Max(1) Unit
t Double word programming - 16 100(2) µs
prog
t Sector (16 KB) erase time - 230 -
ERASE16KB T = 0 to +40°C
A
t Sector (64 KB) erase time V = 3.3V - 490 - ms
ERASE64KB DD
V = 8.5V
t Sector (128 KB) erase time PP - 875 -
ERASE128KB
t Mass erase time - 6.9 - s
ME
t Bank erase time - - 6.9 - s
BE
V Programming voltage - 2.7 - 3.6 V
prog
V V voltage range - 7 - 9 V
PP PP
Minimum current sunk on
I - 10 - - mA
PP the V pin
PP
Cumulative time during
t (3) - - - 1 hour
VPP which V is applied
PP
1. Specified by design.
2. The maximum programming time is measured after 100 K erase operations.
3. V should only be connected during programming/erasing.
PP
130/240 DS9405 Rev 13

<!-- Page 131 -->


| Symbol | Parameter | Conditions | Value | Unit |
| --- | --- | --- | --- | --- |
|  |  |  | Min(1) |  |
| N END | Endurance | T = –40 to +85 °C (6 suffix versions) A T = –40 to +105 °C (7 suffix versions) A | 10 | kcycles |
| t RET | Data retention | 1 kcycle(2) at T = 85 °C A | 30 | Years |
|  |  | 1 kcycle(2) at T = 105 °C A | 10 |  |
|  |  | 10 kcycles(2) at T = 55 °C A | 20 |  |


| Symbol | Parameter | Conditions | Level/ Class |
| --- | --- | --- | --- |
| V FESD | Voltage limits to be applied on any I/O pin to induce a functional disturbance | V = 3.3 V, LQFP176, T = DD A +25°C, f = 168 MHz, conforms HCLK to IEC61000-4-2 | 2B |
| V EFTB | Fast transient voltage burst limits to be applied through 100 pF on V and V DD SS pins to induce a functional disturbance | V = 3.3 V, LQFP176, T =+25°C, DD A f = 168MHz, conforms to HCLK IEC61000-4-2 | 4A |

STM32F427xx STM32F429xx Electrical characteristics
Table 50. Flash memory endurance and data retention
Value
Symbol Parameter Conditions Unit
Min(1)
T = –40 to +85 °C (6 suffix versions)
N Endurance A 10 kcycles
END T = –40 to +105 °C (7 suffix versions)
A
1 kcycle(2) at T = 85 °C 30
A
t Data retention 1 kcycle(2) at T = 105 °C 10 Years
RET A
10 kcycles(2) at T = 55 °C 20
A
1. Evaluated by characterization results.
2. Cycling performed over the whole temperature range.
6.3.14 EMC characteristics
Susceptibility tests are performed on a sample basis during device characterization.
Functional EMS (electromagnetic susceptibility)
While a simple application is executed on the device (toggling 2 LEDs through I/O ports).
Two electromagnetic events stress the device until a failure occurs. The LEDs indicate the
failure:
• Electrostatic discharge (ESD) (positive and negative) is applied to all device pins until
a functional disturbance occurs. This test is compliant with the IEC 61000-4-2 standard.
• FTB: A burst of fast transient voltage (positive and negative) is applied to V and V
DD SS
through a 100 pF capacitor, until a functional disturbance occurs. This test is compliant
with the IEC 61000-4-4 standard.
A device reset allows normal operations to be resumed.
The test results are given in Table51. They are based on the EMS levels and classes
defined in application note AN1709.
Table 51. EMS characteristics
Level/
Symbol Parameter Conditions
Class
V = 3.3 V, LQFP176, T =
Voltage limits to be applied on any I/O pin to DD A
V +25°C, f = 168 MHz, conforms 2B
FESD induce a functional disturbance HCLK
to IEC61000-4-2
Fast transient voltage burst limits to be V = 3.3 V, LQFP176, T =+25°C,
DD A
V applied through 100 pF on V and V f = 168MHz, conforms to 4A
EFTB DD SS HCLK
pins to induce a functional disturbance IEC61000-4-2
When the application is exposed to a noisy environment, it is recommended to avoid pin
exposition to disturbances. The pins showing a middle range robustness are: PA0, PA1,
PA2, PH2, PH3, PH4, PH5, PA3, PA4, PA5, PA6, PA7, PC4, and PC5.
As a consequence, it is recommended to add a serial resistor (1kΏ) located as close as
possible to the MCU to the pins exposed to noise (connected to tracks longer than 50mm
on PCB).
DS9405 Rev 13 131/240
197

<!-- Page 132 -->


| Symbol | Parameter | Conditions | Monitored frequency band | Max vs. [f /f ] HSE CPU | Unit |
| --- | --- | --- | --- | --- | --- |
|  |  |  |  | 25/168MHz |  |
| S EMI | Peak(1) | V = 3.3V, T = 25°C, LQFP176 DD A package, conforming to SAE J1752/3 EEMBC, ART ON, all peripheral clocks enabled, clock dithering disabled. | 0.1 to 30 MHz | 16 | dBµV |
|  |  |  | 30 to 130 MHz | 23 |  |
|  |  |  | 130 MHz to 1GHz | 25 |  |
|  | Level(2) |  | 0.1 MHz to 1GHz | 4 | - |
|  | Peak(1) | VDD = 3.3V, TA = 25°C, LQFP176 package, conforming to SAE J1752/3 EEMBC, ART ON, all peripheral clocks enabled, clock dithering enabled | 0.1 to 30 MHz | 17 | dBµV |
|  |  |  | 30 to 130 MHz | 8 |  |
|  |  |  | 130 MHz to 1GHz | 11 |  |
|  | Level(2) |  | 0.1 MHz to 1GHz | 3.5 | - |

Electrical characteristics STM32F427xx STM32F429xx
Designing hardened software to avoid noise problems
EMC characterization and optimization are performed at component level with a typical
application environment and simplified MCU software. It should be noted that good EMC
performance is highly dependent on the user application and the software in particular.
Therefore, it is recommended that the user applies EMC software optimization and
prequalification tests in relation with the EMC level requested for his application.
Software recommendations
The software flowchart must include the management of runaway conditions such as:
• Corrupted program counter
• Unexpected reset
• Critical data corruption (control registers...)
Prequalification trials
Most of the common failures (unexpected reset and program counter corruption) can be
reproduced by manually forcing a low state on the NRST pin or the oscillator pins for 1
second.
To complete these trials, ESD stress can be applied directly on the device, over the range of
specification values. When unexpected behavior is detected, the software can be hardened
to prevent unrecoverable errors occurring (see application note AN1015).
Electromagnetic Interference (EMI)
The electromagnetic field emitted by the device is monitored while a simple application,
executing EEMBC? code, is running. This emission test is compliant with SAE IEC61967-2
standard, which specifies the test board and the pin loading.
Table 52. EMI characteristics for f = 25 MHz and f = 168 MHz
HSE CPU
Max vs.
Monitored [f /f ]
Symbol Parameter Conditions HSE CPU Unit
frequency band
25/168MHz
0.1 to 30 MHz 16
V = 3.3V, T = 25°C, LQFP176
DD A
Peak(1) package, conforming to SAE J1752/3 30 to 130 MHz 23 dBµV
EEMBC, ART ON, all peripheral
130 MHz to
clocks enabled, clock dithering 25
1GHz
disabled.
0.1 MHz to
Level(2) 4 -
1GHz
S
EMI
0.1 to 30 MHz 17
VDD = 3.3V, TA = 25°C, LQFP176
Peak(1) package, conforming to SAE J1752/3 30 to 130 MHz 8 dBµV
EEMBC, ART ON, all peripheral
130 MHz to
clocks enabled, clock dithering 11
1GHz
enabled
0.1 MHz to
Level(2) 3.5 -
1GHz
1. Refer to chapter “EMI radiated test” in AN1709.
2. Refer to chapter “EMI level classification” in AN1709.
132/240 DS9405 Rev 13

<!-- Page 133 -->


| Symbol | Parameter | Conditions | Monitored frequency band | Max vs. [f /f ] HSE CPU | Unit |
| --- | --- | --- | --- | --- | --- |
|  |  |  |  | 25/180MHz |  |
| S EMI | Peak(1) | V = 3.3V, T = 25°C, LQFP176 DD A package, conforming to SAE J1752/3 EEMBC, ART ON, all peripheral clocks enabled, clock dithering disabled. | 0.1 to 30 MHz | 19 | dBµV |
|  |  |  | 30 to 130 MHz | 23 |  |
|  |  |  | 130 MHz to 1GHz | 22 |  |
|  | Level(2) |  | 0.1 MHz to 1GHz | 4 | - |
|  | Peak(1) | VDD = 3.3V, TA = 25°C, LQFP176 package, conforming to SAE J1752/3 EEMBC, ART ON, all peripheral clocks enabled, clock dithering enabled | 0.1 to 30 MHz | 16 | dBµV |
|  |  |  | 30 to 130 MHz | 10 |  |
|  |  |  | 130 MHz to 1GHz | 16 |  |
|  | Level(2) |  | 0.1 MHz to 1GHz | 3.5 | - |


| Symbol | Ratings | Conditions | Class | Maximum value(1) | Unit |
| --- | --- | --- | --- | --- | --- |
| V ESD(HBM) | Electrostatic discharge voltage (human body model) | T = +25 °C conforming to A ANSI/ESDA/JEDEC JS-001 | 2 | 2000 | V |
| V ESD(CDM) | Electrostatic discharge voltage (charge device model) | T = +25 °C conforming to ANSI/ESD S5.3.1, A LQFP100/144/176, UFBGA169/176, TFBGA176 and WLCSP143 packages | C3 | 250 |  |
|  |  | T = +25 °C conforming to ANSI/ESD S5.3.1, A LQFP208 package | C3 | 250 |  |

STM32F427xx STM32F429xx Electrical characteristics
Table 53. EMI characteristics for = 25 MHz and f = 180 MHz
HSE CPU
Max vs.
Monitored [f /f ]
Symbol Parameter Conditions HSE CPU Unit
frequency band
25/180MHz
0.1 to 30 MHz 19
V = 3.3V, T = 25°C, LQFP176
DD A
Peak(1) package, conforming to SAE J1752/3 30 to 130 MHz 23 dBµV
EEMBC, ART ON, all peripheral
130 MHz to
clocks enabled, clock dithering 22
1GHz
disabled.
0.1 MHz to
Level(2) 4 -
1GHz
S
EMI
0.1 to 30 MHz 16
VDD = 3.3V, TA = 25°C, LQFP176
Peak(1) package, conforming to SAE J1752/3 30 to 130 MHz 10 dBµV
EEMBC, ART ON, all peripheral
130 MHz to
clocks enabled, clock dithering 16
1GHz
enabled
0.1 MHz to
Level(2) 3.5 -
1GHz
1. Refer to chapter “EMI radiated test” in AN1709.
2. Refer to chapter “EMI level classification” in AN1709.
6.3.15 Absolute maximum ratings (electrical sensitivity)
Based on three different tests (ESD, LU) using specific measurement methods, the device is
stressed to determine its performance in terms of electrical sensitivity.
Electrostatic discharge (ESD)
Electrostatic discharges (a positive then a negative pulse separated by 1 second) are
applied to the pins of each sample according to each pin combination. The sample size
depends on the number of supply pins in the device (3 parts × (n+1) supply pins). This test
conforms to the ANSI/ESDA/JEDEC JS-001 and ANSI/ESD S5.3.1 standards.
Table 54. ESD absolute maximum ratings
Maximum
Symbol Ratings Conditions Class Unit
value(1)
Electrostatic discharge
T = +25 °C conforming to
V voltage (human body A 2 2000
ESD(HBM) ANSI/ESDA/JEDEC JS-001
model)
T = +25 °C conforming to ANSI/ESD S5.3.1,
A
V
Electrostatic discharge LQFP100/144/176, UFBGA169/176, C3 250
V voltage (charge device TFBGA176 and WLCSP143 packages
ESD(CDM)
model) T = +25 °C conforming to ANSI/ESD S5.3.1,
A C3 250
LQFP208 package
1. Evaluated by characterization.
DS9405 Rev 13 133/240
197

<!-- Page 134 -->


| Symbol | Parameter | Conditions | Class |
| --- | --- | --- | --- |
| LU | Static latch-up class | T = +105 °C conforming to JESD78A A | II level A |


| Symbol | Description | Functional susceptibility |  | Unit |
| --- | --- | --- | --- | --- |
|  |  | Negative injection | Positive injection |  |
| I INJ | Injected current on BOOT0 pin | −0 | NA | mA |
|  | Injected current on NRST pin | −0 | NA |  |
|  | Injected current on PA0, PA1, PA2, PA3, PA6, PA7, PB0, PC0, PC1, PC2, PC3, PC4, PC5, PH1, PH2, PH3, PH4, PH5 | −0 | NA |  |
|  | Injected current on TTa pins: PA4 and PA5 | −0 | +5 |  |
|  | Injected current on any other FT pin | −5 | NA |  |

Electrical characteristics STM32F427xx STM32F429xx
Static latchup
Two complementary static tests are required on six parts to assess the latchup
performance:
• A supply overvoltage is applied to each power supply pin
• A current injection is applied to each input, output, and configurable I/O pin
These tests are compliant with the EIA/JESD 78A IC latchup standard.
Table 55. Electrical sensitivities
Symbol Parameter Conditions Class
LU Static latch-up class T = +105 °C conforming to JESD78A II level A
A
6.3.16 I/O current injection characteristics
As a general rule, current injection to the I/O pins, due to external voltage below V or
SS
above V (for standard, 3 V-capable I/O pins) should be avoided during normal product
DD
operation. However, to give an indication of the robustness of the microcontroller in cases
when abnormal injection accidentally happens, susceptibility tests are performed on a
sample basis during device characterization.
Functional susceptibility to I/O current injection
While a simple application is executed on the device, the device is stressed by injecting
current into the I/O pins programmed in floating input mode. While current is injected into
the I/O pin, one at a time, the device is checked for functional failures.
An out of range parameter indicates the failure: ADC error above a certain limit (>5 LSB
TUE), out of conventional limits of induced leakage current on adjacent pins (out of –
5µA/+0µA range), or other functional failure (for example reset, oscillator frequency
deviation).
Negative induced leakage current is caused by negative injection and positive induced
leakage current by positive injection.
The test results are given in Table56.
Table 56. I/O current injection susceptibility(1)
Functional susceptibility
Symbol Description Unit
Negative Positive
injection injection
Injected current on BOOT0 pin −0 NA
Injected current on NRST pin −0 NA
Injected current on PA0, PA1, PA2, PA3, PA6, PA7, PB0,
I −0 NA mA
INJ PC0, PC1, PC2, PC3, PC4, PC5, PH1, PH2, PH3, PH4, PH5
Injected current on TTa pins: PA4 and PA5 −0 +5
Injected current on any other FT pin −5 NA
1. NA = not applicable.
134/240 DS9405 Rev 13

<!-- Page 135 -->


| Symbol | Parameter | Conditions | Min | Typ | Max | Unit |
| --- | --- | --- | --- | --- | --- | --- |
| V IL | FT, TTa and NRST I/O input low level voltage | 1.7V≤V DD≤3.6V | - | - | 0.35V −0.04 DD (1) | V |
|  |  |  |  |  | 0.3V (2) DD |  |
|  | BOOT0 I/O input low level voltage | 1.75V ≤3.6V, DD –40°C≤T A≤105°C | - | - | 0.1V +0.1(1) DD |  |
|  |  | 1.7V≤V DD≤3.6V, 0°C≤T A≤105°C | - | - |  |  |
| V IH | FT, TTa and NRST I/O input high level voltage(5) | 1.7V≤V DD≤3.6V | 0.45V +0.3(1) DD | - | - | V |
|  |  |  | 0.7V (2) DD |  |  |  |
|  | BOOT0 I/O input high level voltage | 1.75V≤V ≤3.6V, DD –40°C≤T A≤105°C | 0.17V +0.7(1) DD | - | - |  |
|  |  | 1.7V≤V DD≤3.6V, 0°C≤T A≤105°C |  |  |  |  |
| V HYS | FT, TTa and NRST I/O input hysteresis | 1.7V≤V DD≤3.6V | 10%V (3) DD | - | - | V |
|  | BOOT0 I/O input hysteresis | 1.75V≤V DD≤3.6V, –40°C≤T A≤105°C | 0.1 | - | - |  |
|  |  | 1.7V≤V DD≤3.6V, 0°C≤T A≤105°C |  |  |  |  |
| I lkg | I/O input leakage current (4) | V SS≤V IN≤V DD | - | - | ±1 | µA |
|  | I/O FT input leakage current (5) | V IN = 5V | - | - | 3 |  |

STM32F427xx STM32F429xx Electrical characteristics
Note: It is recommended to add a Schottky diode (pin to ground) to analog pins, which may
potentially inject negative currents.
6.3.17 I/O port characteristics
General input/output characteristics
Unless otherwise specified, the parameters given in Table57: I/O static characteristics are
derived from tests performed under the conditions summarized in Table17. All I/Os are
CMOS and TTL compliant.
Note: For information on GPIO configuration, refer to the application note AN4899 “STM32 GPIO
configuration for hardware settings and low-power consumption” available from
www.st.com.
Table 57. I/O static characteristics
Symbol Parameter Conditions Min Typ Max Unit
0.35V −0.04
DD
FT, TTa and NRST I/O input (1)
low level voltage
1.7V≤V DD≤3.6V - -
0.3V (2)
DD
V IL 1.75V DD ≤3.6V, - - V
BOOT0 I/O input low level –40°C≤T A≤105°C
0.1V +0.1(1)
voltage 1.7V≤V DD≤3.6V,
- -
DD
0°C≤T A≤105°C
FT, TTa and NRST I/O input 0.45V DD +0.3(1)
high level voltage(5)
1.7V≤V DD≤3.6V
0.7V (2)
- -
DD
V IH 1.75V≤V DD ≤3.6V, V
BOOT0 I/O input high level –40°C≤T A≤105°C
0.17V +0.7(1) - -
voltage 1.7V≤V DD≤3.6V, DD
0°C≤T A≤105°C
FT, TTa and NRST I/O input
hysteresis
1.7V≤V DD≤3.6V 10%V
DD
(3) - -
V
1.75V≤V DD≤3.6V,
V
HYS –40°C≤T A≤105°C
BOOT0 I/O input hysteresis 0.1 - -
1.7V≤V DD≤3.6V,
0°C≤T A≤105°C
I/O input leakage current (4) V SS≤V IN≤V
DD
- - ±1
I µA
lkg I/O FT input leakage current (5) V IN = 5V - - 3
DS9405 Rev 13 135/240
197

<!-- Page 136 -->


| Symbol | Parameter |  | Conditions | Min | Typ | Max | Unit |
| --- | --- | --- | --- | --- | --- | --- | --- |
| R PU | Weak pull-up equivalent resistor(6) | All pins except for PA10/PB12 (OTG_FS_ID, OTG_HS_ID) | V IN = V SS | 30 | 40 | 50 | kΩ |
|  |  | PA10/PB12 (OTG_FS_ID, OTG_HS_ID) |  | 7 | 10 | 14 |  |
| R PD | Weak pull- down equivalent resistor(7) | All pins except for PA10/PB12 (OTG_FS_ID, OTG_HS_ID) | V IN = V DD | 30 | 40 | 50 |  |
|  |  | PA10/PB12 (OTG_FS_ID, OTG_HS_ID) |  | 7 | 10 | 14 |  |
| C (8) IO | I/O pin capacitance |  | - | - | 5 | - | pF |

Electrical characteristics STM32F427xx STM32F429xx
Table 57. I/O static characteristics (continued)
Symbol Parameter Conditions Min Typ Max Unit
All pins
except for
PA10/PB12 30 40 50
Weak pull-up (OTG_FS_ID,
R PU equivalent OTG_HS_ID) V IN = V SS
resistor(6)
PA10/PB12
(OTG_FS_ID, 7 10 14
OTG_HS_ID)
kΩ
All pins
except for
Weak pull- PA10/PB12 30 40 50
down (OTG_FS_ID,
R PD equivalent OTG_HS_ID) V IN = V DD
resistor(7)
PA10/PB12
(OTG_FS_ID, 7 10 14
OTG_HS_ID)
C (8) I/O pin capacitance - - 5 - pF
IO
1. Specified by design.
2. Tested in production.
3. With a minimum of 200mV.
4. Leakage could be higher than the maximum value, if negative current is injected on adjacent pins, refer to Table56: I/O
current injection susceptibility
5. To sustain a voltage higher than VDD +0.3 V, the internal pull-up/pull-down resistors must be disabled. Leakage could be
higher than the maximum value, if a negative current is injected on adjacent pins. Refer to Table56: I/O current injection
susceptibility
6. Pull-up resistors are designed with a true resistance in series with a switchable PMOS. This PMOS contribution to the series
resistance is minimum (~10% order).
7. Pull-down resistors are designed with a true resistance in series with a switchable NMOS. This NMOS contribution to the
series resistance is minimum (~10% order).
8. Hysteresis voltage between Schmitt trigger switching levels. Evaluated by characterization.
All I/Os are CMOS and TTL compliant (no software configuration required). Their
characteristics cover more than the strict CMOS-technology or TTL parameters. The
coverage of these requirements for FT I/Os is shown in Figure35.
136/240 DS9405 Rev 13

<!-- Page 137 -->


| M C |  |
| --- | --- |
| - ro d u c ti o n Tested n in D p e si g n s i m u A l a t r io e n a s, V n | 0.45VDD IHmin= ot |
| Based o determi sim n |  |
| Desig Based on Tested in production - C | TTL requirement VILmax = 0.8V MOS requirement VILma |

STM32F427xx STM32F429xx Electrical characteristics
Figure 35. FT I/O input characteristics
VIL/VIH (V)
2.52
min
=
0.7VDD
ment
VIH
1 1 . 2 . 1 1 0 0 9 1 . . . 6 . 0 2 . 1 2 8 7 5 9 2 Ba Te s s e t d e d o n i B n D a p e s ro s e i d g d u n c o s t n i i o m d D n u e e A l - a s t C t r i e i g o M e n n r O a s m s S , i V m n r i e n I o H u q T e l u m t a T i d r t in i e L o = n r e 0 s . , T q 4 V u T 5 V i I V L r L I e D H m r m e D m a q + e x i u 0 n n = . i t 3 r = 0 e V . m 2 3 IL V 5 e m V n D a t x D -0.04
0.55 = 0.8V
0.51
Tested in production - CMOS requirement VILmax = 0.3VDD
VDD (V)
1.7 2.0 2.4 2.7 3.3 3.6
MS33746V1
Output driving current
The GPIOs (general-purpose input/outputs) can sink or source up to ±8 mA, and sink or
source up to ±20 mA (with a relaxed V /V ) except PC13, PC14, PC15, and PI8, which
OL OH
can sink or source up to ±3mA. When using the PC13 to PC15 and PI8 GPIOs in output
mode, the speed should not exceed 2 MHz with a maximum load of 30 pF.
In the user application, the number of I/O pins, which can drive current must be limited to
respect the absolute maximum rating specified in Section6.2. In particular:
• The sum of the currents sourced by all the I/Os on V plus the maximum Run
DD,
consumption of the MCU sourced on V cannot exceed the absolute maximum rating
DD,
∑I (see Table15).
VDD
• The sum of the currents sunk by all the I/Os on V plus the maximum Run
SS
consumption of the MCU sunk on V cannot exceed the absolute maximum rating
SS
∑I (see Table15).
VSS
DS9405 Rev 13 137/240
197

<!-- Page 138 -->


| Symbol | Parameter | Conditions | Min | Max | Unit |
| --- | --- | --- | --- | --- | --- |
| V (1) OL | Output low level voltage for an I/O pin | CMOS port(2) I = +8 mA IO 2.7 V ≤V DD ≤ 3.6V | - | 0.4 | V |
| V (3) OH | Output high level voltage for an I/O pin |  | V −0.4 DD | - |  |
| V (1) OL | Output low level voltage for an I/O pin | TTL port(2) I =+ 8mA IO 2.7 V ≤V DD ≤ 3.6V | - | 0.4 | V |
| V (3) OH | Output high level voltage for an I/O pin |  | 2.4 | - |  |
| V (1) OL | Output low level voltage for an I/O pin | I = +20 mA IO 2.7 V ≤ V DD ≤ 3.6V | - | 1.3(4) | V |
| V (3) OH | Output high level voltage for an I/O pin |  | V −1.3(4) DD | - |  |
| V (1) OL | Output low level voltage for an I/O pin | I = +6 mA IO 1.8 V ≤ V DD ≤ 3.6 V | - | 0.4(4) | V |
| V (3) OH | Output high level voltage for an I/O pin |  | V −0.4(4) DD | - |  |
| V (1) OL | Output low level voltage for an I/O pin | I = +4 mA IO 1.7 V ≤ V DD ≤ 3.6V | - | 0.4(5) | V |
| V (3) OH | Output high level voltage for an I/O pin |  | V −0.4(5) DD | - |  |

Electrical characteristics STM32F427xx STM32F429xx
Output voltage levels
Unless otherwise specified, the parameters given in Table58 are derived from tests
performed under ambient temperature and V supply voltage conditions summarized in
DD
Table17. All I/Os are CMOS and TTL compliant.
Table 58. Output voltage characteristics
Symbol Parameter Conditions Min Max Unit
V (1) Output low level voltage for an I/O pin CMOS port(2) - 0.4
OL
I = +8 mA V
V (3) Output high level voltage for an I/O pin IO V −0.4 -
OH 2.7 V ≤V DD ≤ 3.6V DD
V (1) Output low level voltage for an I/O pin TTL port(2) - 0.4
OL
I =+ 8mA V
V (3) Output high level voltage for an I/O pin IO 2.4 -
OH 2.7 V ≤V DD ≤ 3.6V
V OL (1) Output low level voltage for an I/O pin I IO = +20 mA - 1.3(4) V
V OH (3) Output high level voltage for an I/O pin 2.7 V ≤ V DD ≤ 3.6V V DD −1.3(4) -
V OL (1) Output low level voltage for an I/O pin I IO = +6 mA - 0.4(4) V
V OH (3) Output high level voltage for an I/O pin 1.8 V ≤ V DD ≤ 3.6 V V DD −0.4(4) -
V OL (1) Output low level voltage for an I/O pin I IO = +4 mA - 0.4(5) V
V OH (3) Output high level voltage for an I/O pin 1.7 V ≤ V DD ≤ 3.6V V DD −0.4(5) -
1. The I current sunk by the device must always respect the absolute maximum rating specified in Table15.
IO
and the sum of I (I/O ports and control pins) must not exceed I .
IO VSS
2. TTL and CMOS outputs are compatible with JEDEC standards JESD36 and JESD52.
3. The I current sourced by the device must always respect the absolute maximum rating specified in
IO
Table15 and the sum of I (I/O ports and control pins) must not exceed I .
IO VDD
4. Based on characterization data.
5. Specified by design.
138/240 DS9405 Rev 13

<!-- Page 139 -->


| OSPEEDRy [1:0] bit value(1) | Symbol | Parameter | Conditions | Min | Typ | Max | Unit |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 00 | f max(IO)out | Maximum frequency(3) | C = 50pF, V ≥ 2.7 V L DD | - | - | 4 | MHz |
|  |  |  | C = 50pF, V ≥ 1.7 V L DD | - | - | 2 |  |
|  |  |  | C = 10pF, V ≥ 2.7 V L DD | - | - | 8 |  |
|  |  |  | C = 10pF, V ≥ 1.8 V L DD | - | - | 4 |  |
|  |  |  | C = 10pF, V ≥ 1.7 V L DD | - | - | 3 |  |
|  | t / f(IO)out t r(IO)out | Output high to low level fall time and output low to high level rise time | C = 50pF, V = 1.7 V to L DD 3.6V | - | - | 100 | ns |
| 01 | f max(IO)out | Maximum frequency(3) | C = 50pF, V ≥ 2.7 V L DD | - | - | 25 | MHz |
|  |  |  | C = 50pF, V ≥ 1.8 V L DD | - | - | 12.5 |  |
|  |  |  | C = 50pF, V ≥ 1.7 V L DD | - | - | 10 |  |
|  |  |  | C = 10pF, V ≥ 2.7 V L DD | - | - | 50 |  |
|  |  |  | C = 10pF, V ≥ 1.8 V L DD | - | - | 20 |  |
|  |  |  | C = 10pF, V ≥ 1.7 V L DD | - | - | 12.5 |  |
|  | t / f(IO)out t r(IO)out | Output high to low level fall time and output low to high level rise time | C = 50pF, V ≥ 2.7 V L DD | - | - | 10 | ns |
|  |  |  | C = 10pF, V ≥ 2.7 V L DD | - | - | 6 |  |
|  |  |  | C = 50pF, V ≥ 1.7 V L DD | - | - | 20 |  |
|  |  |  | C = 10pF, V ≥ 1.7 V L DD | - | - | 10 |  |
| 10 | f max(IO)out | Maximum frequency(3) | C = 40pF, V ≥ 2.7 V L DD | - | - | 50(4) | MHz |
|  |  |  | C = 10pF, V ≥ 2.7 V L DD | - | - | 100(4) |  |
|  |  |  | C = 40pF, V ≥ 1.7 V L DD | - | - | 25 |  |
|  |  |  | C = 10pF, V ≥ 1.8 V L DD | - | - | 50 |  |
|  |  |  | C = 10pF, V ≥ 1.7 V L DD | - | - | 42.5 |  |
|  | t / f(IO)out t r(IO)out | Output high to low level fall time and output low to high level rise time | C = 40pF, V ≥2.7 V L DD | - | - | 6 | ns |
|  |  |  | C = 10pF, V ≥ 2.7 V L DD | - | - | 4 |  |
|  |  |  | C = 40pF, V ≥ 1.7 V L DD | - | - | 10 |  |
|  |  |  | C = 10pF, V ≥ 1.7 V L DD | - | - | 6 |  |

STM32F427xx STM32F429xx Electrical characteristics
Input/output AC characteristics
The definition and values of input/output AC characteristics are given in Figure36 and
Table59, respectively.
Unless otherwise specified, the parameters given in Table59 are derived from tests
performed under the ambient temperature and V supply voltage conditions summarized
DD
in Table17.
Table 59. I/O AC characteristics(1)(2)
OSPEEDRy
[1:0] bit Symbol Parameter Conditions Min Typ Max Unit
value(1)
C = 50pF, V ≥ 2.7 V - - 4
L DD
C = 50pF, V ≥ 1.7 V - - 2
L DD
f Maximum frequency(3) C = 10pF, V ≥ 2.7 V - - 8 MHz
max(IO)out L DD
00 C L = 10pF, V DD ≥ 1.8 V - - 4
C = 10pF, V ≥ 1.7 V - - 3
L DD
Output high to low level fall
t / C = 50pF, V = 1.7 V to
f(IO)out time and output low to high L DD - - 100 ns
t 3.6V
r(IO)out level rise time
C = 50pF, V ≥ 2.7 V - - 25
L DD
C = 50pF, V ≥ 1.8 V - - 12.5
L DD
C = 50pF, V ≥ 1.7 V - - 10
f Maximum frequency(3) L DD MHz
max(IO)out
C = 10pF, V ≥ 2.7 V - - 50
L DD
C = 10pF, V ≥ 1.8 V - - 20
L DD
01
C = 10pF, V ≥ 1.7 V - - 12.5
L DD
C = 50pF, V ≥ 2.7 V - - 10
L DD
t f(IO)out / O tim ut e p u an t h d i g o h u t t p o u l t o l w ow le t v o e h l i f g a h ll C L = 10pF, V DD ≥ 2.7 V - - 6 ns
t r(IO)out level rise time C L = 50pF, V DD ≥ 1.7 V - - 20
C = 10pF, V ≥ 1.7 V - - 10
L DD
C = 40pF, V ≥ 2.7 V - - 50(4)
L DD
C = 10pF, V ≥ 2.7 V - - 100(4)
L DD
f Maximum frequency(3) C = 40pF, V ≥ 1.7 V - - 25 MHz
max(IO)out L DD
C = 10pF, V ≥ 1.8 V - - 50
L DD
10 C = 10pF, V ≥ 1.7 V - - 42.5
L DD
C = 40pF, V ≥2.7 V - - 6
L DD
t f(IO)out / O tim ut e p u an t h d i g o h u t t p o u l t o l w ow le t v o e h l i f g a h ll C L = 10pF, V DD ≥ 2.7 V - - 4 ns
t r(IO)out level rise time C L = 40pF, V DD ≥ 1.7 V - - 10
C = 10pF, V ≥ 1.7 V - - 6
L DD
DS9405 Rev 13 139/240
197

<!-- Page 140 -->


| OSPEEDRy [1:0] bit value(1) | Symbol | Parameter | Conditions | Min | Typ | Max | Unit |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 11 | f max(IO)out | Maximum frequency(3) | C = 30pF, V ≥ 2.7 V L DD | - | - | 100(4) | MHz |
|  |  |  | C = 30pF, V ≥ 1.8 V L DD | - | - | 50 |  |
|  |  |  | C = 30pF, V ≥ 1.7 V L DD | - | - | 42.5 |  |
|  |  |  | C = 10pF, V ≥ 2.7 V L DD | - | - | 180(4) |  |
|  |  |  | C = 10pF, V ≥ 1.8 V L DD | - | - | 100 |  |
|  |  |  | C = 10pF, V ≥ 1.7 V L DD | - | - | 72.5 |  |
|  | t / f(IO)out t r(IO)out | Output high to low level fall time and output low to high level rise time | C = 30pF, V ≥ 2.7 V L DD | - | - | 4 | ns |
|  |  |  | C = 30pF, V ≥1.8 V L DD | - | - | 6 |  |
|  |  |  | C = 30pF, V ≥1.7 V L DD | - | - | 7 |  |
|  |  |  | C = 10pF, V ≥ 2.7 V L DD | - | - | 2.5 |  |
|  |  |  | C = 10pF, V ≥1.8 V L DD | - | - | 3.5 |  |
|  |  |  | C = 10pF, V ≥1.7 V L DD | - | - | 4 |  |
| - | tEXTIpw | Pulse width of external signals detected by the EXTI controller | - | 10 | - | - | ns |

Electrical characteristics STM32F427xx STM32F429xx
Table 59. I/O AC characteristics(1)(2) (continued)
OSPEEDRy
[1:0] bit Symbol Parameter Conditions Min Typ Max Unit
value(1)
C = 30pF, V ≥ 2.7 V - - 100(4)
L DD
C = 30pF, V ≥ 1.8 V - - 50
L DD
C = 30pF, V ≥ 1.7 V - - 42.5
f Maximum frequency(3) L DD MHz
max(IO)out C = 10pF, V ≥ 2.7 V - - 180(4)
L DD
C = 10pF, V ≥ 1.8 V - - 100
L DD
C = 10pF, V ≥ 1.7 V - - 72.5
L DD
11
C = 30pF, V ≥ 2.7 V - - 4
L DD
C = 30pF, V ≥1.8 V - - 6
L DD
t f(IO)out / O tim ut e p u an t h d i g o h u t t p o u l t o l w ow le t v o e h l i f g a h ll C L = 30pF, V DD ≥1.7 V - - 7 ns
t r(IO)out level rise time C L = 10pF, V DD ≥ 2.7 V - - 2.5
C = 10pF, V ≥1.8 V - - 3.5
L DD
C = 10pF, V ≥1.7 V - - 4
L DD
Pulse width of external signals
- tEXTIpw detected by the EXTI - 10 - - ns
controller
1. Specified by design.
2. The I/O speed is configured using the OSPEEDRy[1:0] bits. Refer to the STM32F4xx reference manual for a description of
the GPIOx_SPEEDR GPIO port output speed register.
3. The maximum frequency is defined in Figure36.
4. For maximum frequencies above 50MHz and V DD > 2.4V, the compensation cell should be used.
Figure 36. I/O AC characteristics definition
90% 10%
50% 50%
10% 90%
tr(IO)out t f(IO)out
T
Maximum frequency is achieved with a duty cycle at (45 - 55%) when loaded by the
specified capacitance.
MS32132V4
140/240 DS9405 Rev 13

<!-- Page 141 -->


| Symbol | Parameter | Conditions | Min | Typ | Max | Unit |
| --- | --- | --- | --- | --- | --- | --- |
| R PU | Weak pull-up equivalent resistor(1) | V IN = V SS | 30 | 40 | 50 | kΩ |
| T (2) F(NRST) | NRST Input filtered pulse | - | - | - | 100 | ns |
| T (2) NF(NRST) | NRST Input not filtered pulse | V > 2.7 V DD | 300 | - | - | ns |
| T NRST_OUT | Generated reset pulse duration | Internal Reset source | 20 | - | - | µs |


|  |  |
| --- | --- |


|  |
| --- |
| Filter |


|  |  |
| --- | --- |
|  |  |

STM32F427xx STM32F429xx Electrical characteristics
6.3.18 NRST pin characteristics
The NRST pin input driver uses CMOS technology. It is connected to a permanent pull-up
resistor, R (see Table57: I/O static characteristics).
PU
Unless otherwise specified, the parameters given in Table60 are derived from tests
performed under the ambient temperature and V supply voltage conditions summarized
DD
in Table17.
Table 60. NRST pin characteristics
Symbol Parameter Conditions Min Typ Max Unit
R PU Weak pull-up equivalent resistor(1) V IN = V SS 30 40 50 kΩ
T (2) NRST Input filtered pulse - - - 100 ns
F(NRST)
T (2) NRST Input not filtered pulse V > 2.7 V 300 - - ns
NF(NRST) DD
T Generated reset pulse duration Internal Reset source 20 - - µs
NRST_OUT
1. The pull-up is designed with a true resistance in series with a switchable PMOS. This PMOS contribution to the series
resistance must be minimum (~10% order).
2. Specified by design.
Figure 37. Recommended NRST pin protection
VDD
External
reset circuit(1)
NRST(2) RPU Internal Reset
Filter
0.1 μF
STM32F
ai14132c
1. The reset network protects the device against parasitic resets.
2. The external capacitor must be placed as close as possible to the device.
3. The user must ensure that the level on the NRST pin can go below the V max level specified in
IL(NRST)
Table60. Otherwise, the reset is not considered by the device.
DS9405 Rev 13 141/240
197

<!-- Page 142 -->


| Symbol | Parameter | Conditions(3) | Min | Max | Unit |
| --- | --- | --- | --- | --- | --- |
| t res(TIM) | Timer resolution time | AHB/APBx prescaler=1 or 2 or 4, f = TIMxCLK 180MHz | 1 | - | t TIMxCLK |
|  |  | AHB/APBx prescaler>4, f = 90MHz TIMxCLK | 1 | - | t TIMxCLK |
| f EXT | Timer external clock frequency on CH1 to CH4 | f = 180MHz TIMxCLK | 0 | f /2 TIMxCLK | MHz |
| Res TIM | Timer resolution |  | - | 16/32 | bit |
| t MAX_COUNT | Maximum possible count with 32-bit counter | - | - | 65536 × 65536 | t TIMxCLK |

Electrical characteristics STM32F427xx STM32F429xx
6.3.19 TIM timer characteristics
The parameters given in Table61 are specified by design.
Refer to Section6.3.17: I/O port characteristics for details on the input/output alternate
function characteristics (output compare, input capture, external clock, PWM output).
Table 61. TIMx characteristics(1)(2)
Symbol Parameter Conditions(3) Min Max Unit
AHB/APBx prescaler=1
or 2 or 4, f TIMxCLK = 1 - t TIMxCLK
t res(TIM) Timer resolution time 180MHz
AHB/APBx prescaler>4,
f
TIMxCLK
= 90MHz 1 - t TIMxCLK
Timer external clock
f EXT frequency on CH1 to CH4 f = 180MHz 0 f TIMxCLK /2 MHz
TIMxCLK
Res TIM Timer resolution - 16/32 bit
Maximum possible count 65536 ×
t MAX_COUNT with 32-bit counter - - 65536 t TIMxCLK
1. TIMx is used as a general term to refer to the TIM1 to TIM12 timers.
2. Specified by design.
3. The maximum timer frequency on APB1 or APB2 is up to 180MHz, by setting the TIMPRE bit in the
RCC_DCKCFGR register, if APBx prescaler is 1 or 2 or 4, then TIMxCLK = HCKL, otherwise TIMxCLK =
4x PCLKx.
6.3.20 Communications interfaces
I
2
C interface characteristics
2 2
The I C interface meets the timing requirements of the I C-bus specification and user
manual rev. 03 for:
• Standard mode (Sm): with a bit rate up to 100kbit/s
• Fast mode (Fm): with a bit rate up to 400 kbit/s.
2
The I C timings requirements are specified by design when the I2C peripheral is properly
configured (refer to RM0090 reference manual).
The SDA and SCL I/O requirements are met with the following restrictions: the SDA and
SCL I/O pins are not “true” open-drain. When configured as open-drain, the PMOS
connected between the I/O pin and V is disabled, but is still present. Refer to
DD
Section6.3.17: I/O port characteristics for more details on the I2C I/O characteristics.
2
All I C SDA and SCL I/Os embed an analog filter. Refer to the table below for the analog
filter characteristics:
142/240 DS9405 Rev 13

<!-- Page 143 -->


| Symbol | Parameter | Min | Max | Unit |
| --- | --- | --- | --- | --- |
| t AF | Maximum pulse width of spikes that are suppressed by the analog filter | 50(2) | 260(3) | ns |


| Symbol | Parameter | Conditions |  | Min | Typ | Max | Unit |
| --- | --- | --- | --- | --- | --- | --- | --- |
| f SCK 1/t c(SCK) | SPI clock frequency | Master mode, SPI1/4/5/6, 2.7V≤V ≤3.6V DD |  | - | - | 45 | MHz |
|  |  | Slave mode, SPI1/4/5/6, 2.7V≤V ≤3.6V DD | Receiver |  |  | 45 |  |
|  |  |  | Transmitter/ full-duplex |  |  | 38(2) |  |
|  |  | Master mode, SPI1/2/3/4/5/6, 1.7V≤V ≤3.6V DD |  | - | - | 22.5 |  |
|  |  | Slave mode, SPI1/2/3/4/5/6, 1.7V≤V ≤3.6V DD |  |  |  | 22.5 |  |
| Duty(SCK) | Duty cycle of SPI clock frequency | Slave mode |  | 30 | 50 | 70 | % |

STM32F427xx STM32F429xx Electrical characteristics
Table 62. I2C analog filter characteristics(1)
Symbol Parameter Min Max Unit
Maximum pulse width of spikes that
t 50(2) 260(3) ns
AF are suppressed by the analog filter
1. Specified by design.
2. Spikes with widths below t are filtered.
AF(min)
3. Spikes with widths above t are not filtered
AF(max)
SPI interface characteristics
Unless otherwise specified, the parameters given in Table63 for the SPI interface are
derived from tests performed under the ambient temperature, f frequency, and V
PCLKx DD
supply voltage conditions summarized in Table17, with the following configuration:
• Output speed is set to OSPEEDRy[1:0] = 10
• Capacitive load C = 30pF
• Measurement points are done at CMOS levels: 0.5V
DD
Refer to Section6.3.17: I/O port characteristics for more details on the input/output alternate
function characteristics (NSS, SCK, MOSI, MISO for SPI).
Table 63. SPI dynamic characteristics(1)
Symbol Parameter Conditions Min Typ Max Unit
Master mode, SPI1/4/5/6,
45
2.7V≤V ≤3.6V
DD
Slave mode, Receiver - - 45
SPI1/4/5/6, Transmitter/
f
SCK SPI clock frequency 2.7V≤V DD ≤3.6V full-duplex
38(2)
MHz
1/t
c(SCK)
Master mode, SPI1/2/3/4/5/6,
22.5
1.7V≤V ≤3.6V
DD
- -
Slave mode, SPI1/2/3/4/5/6,
22.5
1.7V≤V ≤3.6V
DD
Duty cycle of SPI clock
Duty(SCK) Slave mode 30 50 70 %
frequency
DS9405 Rev 13 143/240
197

<!-- Page 144 -->


| Symbol | Parameter | Conditions | Min | Typ | Max | Unit |
| --- | --- | --- | --- | --- | --- | --- |
| t w(SCKH) | SCK high and low time | Master mode, SPI presc = 2, 2.7V≤V ≤3.6V DD | T −0.5 PCLK | T PCLK | T +0.5 PCLK | ns |
| t w(SCKL) |  | Master mode, SPI presc = 2, 1.7V≤V ≤3.6V DD | T −2 PCLK | T PCLK | T +2 PCLK |  |
| t su(NSS) | NSS setup time | Slave mode, SPI presc = 2 | 4T PCLK | - | - |  |
| t h(NSS) | NSS hold time | Slave mode, SPI presc = 2 | 2T PCLK |  |  |  |
| t su(MI) | Data input setup time | Master mode | 3 | - | - |  |
| t su(SI) |  | Slave mode | 0 | - | - |  |
| t h(MI) | Data input hold time | Master mode | 0.5 | - | - |  |
| t h(SI) |  | Slave mode | 2 | - | - |  |
| t ) a(SO | Data output access time | Slave mode, SPI presc = 2 | 0 | - | 4T PCLK |  |
| t dis(SO) | Data output disable time | Slave mode, SPI1/4/5/6, 2.7V≤V ≤3.6V DD | 0 | - | 8.5 |  |
|  |  | Slave mode, SPI1/2/3/4/5/6 and 1.7V≤V ≤3.6V DD | 0 | - | 16.5 |  |
| t v(SO) t h(SO) | Data output valid/hold time | Slave mode (after enable edge), SPI1/4/5/6 and 2.7V ≤ V ≤ 3.6V DD | - | 11 | 13 | ns |
|  |  | Slave mode (after enable edge), SPI2/3, 2.7V≤V ≤3.6V DD | - | 14 | 15 |  |
|  |  | Slave mode (after enable edge), SPI1/4/5/6, 1.7V≤V ≤3.6V DD | - | 15.5 | 19 |  |
|  |  | Slave mode (after enable edge), SPI2/3, 1.7V≤V ≤3.6V DD | - | 15.5 | 17.5 |  |
| t v(MO) | Data output valid time | Master mode (after enable edge), SPI1/4/5/6, 2.7V≤V ≤3.6V DD | - | - | 2.5 |  |
|  |  | Master mode (after enable edge), SPI1/2/3/4/5/6, 1.7V≤V ≤3.6V DD | - | - | 4.5 |  |
| t h(MO) | Data output hold time | Master mode (after enable edge) | 0 | - | - |  |

Electrical characteristics STM32F427xx STM32F429xx
Table 63. SPI dynamic characteristics(1) (continued)
Symbol Parameter Conditions Min Typ Max Unit
Master mode, SPI presc = 2,
t w(SCKH) 2.7V≤V ≤3.6V T PCLK −0.5 T PCLK T PCLK +0.5
DD
SCK high and low time
Master mode, SPI presc = 2,
t w(SCKL) 1.7V≤V ≤3.6V T PCLK −2 T PCLK T PCLK +2
DD
t
su(NSS)
NSS setup time Slave mode, SPI presc = 2 4T
PCLK
- -
t
h(NSS)
NSS hold time Slave mode, SPI presc = 2 2T
PCLK
t Master mode 3 - -
su(MI)
Data input setup time
t Slave mode 0 - - ns
su(SI)
t Master mode 0.5 - -
h(MI)
Data input hold time
t Slave mode 2 - -
h(SI)
t
a(SO
) Data output access time Slave mode, SPI presc = 2 0 - 4T
PCLK
Slave mode, SPI1/4/5/6,
0 - 8.5
2.7V≤V ≤3.6V
DD
t Data output disable time
dis(SO)
Slave mode, SPI1/2/3/4/5/6 and
0 - 16.5
1.7V≤V ≤3.6V
DD
Slave mode (after enable edge),
- 11 13
SPI1/4/5/6 and 2.7V ≤ V ≤ 3.6V
DD
Slave mode (after enable edge),
- 14 15
t v(SO) Data output valid/hold SPI2/3, 2.7V≤V DD ≤3.6V
t h(SO) time Slave mode (after enable edge),
- 15.5 19
SPI1/4/5/6, 1.7V≤V ≤3.6V
DD
Slave mode (after enable edge), ns
- 15.5 17.5
SPI2/3, 1.7V≤V ≤3.6V
DD
Master mode (after enable edge),
- - 2.5
SPI1/4/5/6, 2.7V≤V ≤3.6V
DD
t Data output valid time
v(MO)
Master mode (after enable edge),
- - 4.5
SPI1/2/3/4/5/6, 1.7V≤V ≤3.6V
DD
t h(MO) Data output hold time Master mode (after enable edge) 0 - -
1. Evaluated by characterization.
2. The maximum frequency in Slave transmitter mode is determined by the sum of t and t which has to fit into SCK
v(SO) su(MI),
low or high phase preceding the SCK sampling edge. This value can be achieved when the SPI communicates with a
master having t = 0 while Duty(SCK) = 50%.
su(MI)
144/240 DS9405 Rev 13

<!-- Page 145 -->


|  |
| --- |
| t c(SCK) |


|  |
| --- |
| t su(NSS) |


|  |
| --- |
| t a(SO) |


| t |
| --- |
| v(SO |


|  |
| --- |
| th(SO) |


|  |
| --- |
| t dis(SO) |


|  |
| --- |
| t su(SI) |


|  |
| --- |
| h(SI |


|  |
| --- |
| t h(NSS |


|  |
| --- |
| t a(SO) |


|  |
| --- |
| t su(SI) |


|  |
| --- |
| t h(SI) |

STM32F427xx STM32F429xx Electrical characteristics
Figure 38. SPI timing diagram - slave mode and CPHA = 0
NSS input
CPHA=0
CPOL=0
MSv41658V2
Figure 39. SPI timing diagram - slave mode and CPHA = 1
DS9405 Rev 13 145/240
197
tupni
KCS
t t
c(SCK) h(NSS)
t t
su(NSS) w(SCKH)
CPHA=0
CPOL=1
t a(SO) t w(SCKL) t v(SO) th(SO) t dis(SO)
MISO output First bit OUT Next bits OUT Last bit OUT
t t
su(SI) h(SI)
MOSI input First bit IN Next bits IN Last bit IN
NSS input
t t
c(SCK) h(NSS)
t t
su(NSS) w(SCKH)
CPHA=1
CPOL=0
CPHA=1
CPOL=1
t t t t
a(SO) w(SCKL) v(SO) dis(SO)
MISO output
t t
su(SI) h(SI)
MOSI input
MSv41659V2
tupni
KCS
t
h(SO)
First bit OUT Next bits OUT Last bit OUT
First bit IN Next bits IN Last bit IN

<!-- Page 146 -->


|  |
| --- |
| t su(MI) |


|  |
| --- |
| h(MI) |


|  |  |
| --- | --- |
|  |  |
| t h(MO) |  |


|  |
| --- |
| v(MO) |


| Symbol | Parameter | Conditions | Min | Max | Unit |
| --- | --- | --- | --- | --- | --- |
| f MCK | I2S Main clock output | - | 256x8K | 256xFs(2) | MHz |
| f CK | I2S clock frequency | Master data: 32 bits | - | 64xFs | MHz |
|  |  | Slave data: 32 bits | - | 64xFs |  |
| D CK | I2S clock frequency duty cycle | Slave receiver | 30 | 70 | % |

Electrical characteristics STM32F427xx STM32F429xx
Figure 40. SPI timing diagram - master mode
NSS input
t
c(SCK)
t
w(SCKH)
CPHA=0
CPOL=0
CPHA=0
CPOL=1
t
w(SCKL)
t t
su(MI) h(MI)
MISO input
MOSI output
MSv72626V1
I2S interface characteristics
Unless otherwise specified, the parameters given in Table64 for the I2S interface are
derived from tests performed under the ambient temperature, f frequency, and V
PCLKx DD
supply voltage conditions summarized in Table17, with the following configuration:
• Output speed is set to OSPEEDRy[1:0] = 10
• Capacitive load C = 30pF
• Measurement points are done at CMOS levels: 0.5V
DD
Refer to Section6.3.17: I/O port characteristics for more details on the input/output alternate
function characteristics (CK, SD, WS).
146/240 DS9405 Rev 13
tuptuo
KCS
CPHA=1
CPOL=0
CPHA=1
CPOL=1
First bit IN Next bits IN Last bit IN
First bit OUT Next bits OUT Last bit OUT
tuptuo
KCS
High
t t
v(MO) h(MO)
Table 64. I2S dynamic characteristics(1)
Symbol Parameter Conditions Min Max Unit
f I2S Main clock output - 256x8K 256xFs(2) MHz
MCK
Master data: 32 bits - 64xFs
f I2S clock frequency MHz
CK
Slave data: 32 bits - 64xFs
D I2S clock frequency duty cycle Slave receiver 30 70 %
CK

<!-- Page 147 -->


| Symbol | Parameter | Conditions | Min | Max | Unit |
| --- | --- | --- | --- | --- | --- |
| t v(WS) | WS valid time | Master mode | 0 | 6 | ns |
| t h(WS) | WS hold time | Master mode | 0 | - |  |
| t su(WS) | WS setup time | Slave mode | 1 | - |  |
| t h(WS) | WS hold time | Slave mode | 0 | - |  |
| t su(SD_MR) | Data input setup time | Master receiver | 7.5 | - |  |
| t su(SD_SR) |  | Slave receiver | 2 | - |  |
| t h(SD_MR) | Data input hold time | Master receiver | 0 | - |  |
| t h(SD_SR) |  | Slave receiver | 0 | - |  |
| t v(SD_ST) t h(SD_ST) | Data output valid time | Slave transmitter (after enable edge) | - | 27 |  |
| t v(SD_MT) |  | Master transmitter (after enable edge) | - | 20 |  |
| t h(SD_MT) | Data output hold time | Master transmitter (after enable edge) | 2.5 | - |  |

STM32F427xx STM32F429xx Electrical characteristics
Table 64. I2S dynamic characteristics(1) (continued)
Symbol Parameter Conditions Min Max Unit
t WS valid time Master mode 0 6
v(WS)
t WS hold time Master mode 0 -
h(WS)
t WS setup time Slave mode 1 -
su(WS)
t WS hold time Slave mode 0 -
h(WS)
t Master receiver 7.5 -
su(SD_MR)
Data input setup time
t Slave receiver 2 -
su(SD_SR)
t Master receiver 0 - ns
h(SD_MR)
Data input hold time
t Slave receiver 0 -
h(SD_SR)
t
v(SD_ST) Slave transmitter (after enable edge) - 27
t h(SD_ST) Data output valid time
t Master transmitter (after enable edge) - 20
v(SD_MT)
-
t Data output hold time Master transmitter (after enable edge) 2.5
h(SD_MT)
1. Evaluated by characterization.
2. The maximum value of 256xFs is 45MHz (APB1 maximum frequency).
Note: Refer to the I2S section of the RM0090 reference manual for more details on the sampling
frequency (F ).
S
f , f , and D values reflect only the digital peripheral behavior. The values of these
MCK CK CK
parameters might be slightly impacted by the source clock precision. D depends mainly
CK
on the value of ODD bit. The digital contribution leads to a minimum value of
(I2SDIV/(2*I2SDIV+ODD) and a maximum value of (I2SDIV+ODD)/(2*I2SDIV+ODD). F
S
maximum value is supported for each mode/condition.
DS9405 Rev 13 147/240
197

<!-- Page 148 -->


|  |  |
| --- | --- |
|  |  |
|  |  |


|  |
| --- |
|  |

Electrical characteristics STM32F427xx STM32F429xx
Figure 41. I2S slave timing diagram (Philips protocol)(1)
1. .LSB transmit/receive of the previously transmitted byte. No LSB transmit/receive is sent before the first
byte.
Figure 42. I2S master timing diagram (Philips protocol)(1)
1. LSB transmit/receive of the previously transmitted byte. No LSB transmit/receive is sent before the first
byte.
148/240 DS9405 Rev 13

<!-- Page 149 -->


| Symbol | Parameter | Conditions | Min | Max | Unit |
| --- | --- | --- | --- | --- | --- |
| f MCKL | SAI Main clock output | - | 256 x 8K | 256xFs(2) | MHz |
| F SCK | SAI clock frequency | Master data: 32 bits | - | 64xFs | MHz |
|  |  | Slave data: 32 bits | - | 64xFs |  |
| D SCK | SAI clock frequency duty cycle | Slave receiver | 30 | 70 | % |
| t v(FS) | FS valid time | Master mode | 8 | 22 | ns |
| t su(FS) | FS setup time | Slave mode | 2 | - |  |
| t h(FS) | FS hold time | Master mode | 8 | - |  |
|  |  | Slave mode | 0 | - |  |
| t su(SD_MR) | Data input setup time | Master receiver | 5 | - |  |
| t su(SD_SR) |  | Slave receiver | 3 | - |  |
| t h(SD_MR) | Data input hold time | Master receiver | 0 | - |  |
| t h(SD_SR) |  | Slave receiver | 0 | - |  |
| t v(SD_ST) t h(SD_ST) | Data output valid time | Slave transmitter (after enable edge) | - | 22 |  |
| t v(SD_MT) |  | Master transmitter (after enable edge) | - | 20 |  |
| t h(SD_MT) | Data output hold time | Master transmitter (after enable edge) | 8 | - |  |

STM32F427xx STM32F429xx Electrical characteristics
SAI characteristics
Unless otherwise specified, the parameters given in Table65 for SAI are derived from tests
performed under the ambient temperature, f frequency, and VDD supply voltage
PCLKx
conditions summarized in Table17, with the following configuration:
• Output speed is set to OSPEEDRy[1:0] = 10
• Capacitive load C=30pF
• Measurement points are performed at CMOS levels: 0.5V
DD
Refer to Section6.3.17: I/O port characteristics for more details on the input/output alternate
function characteristics (SCK,SD,WS).
Table 65. SAI characteristics(1)
Symbol Parameter Conditions Min Max Unit
f SAI Main clock output - 256 x 8K 256xFs(2) MHz
MCKL
Master data: 32 bits - 64xFs
F SAI clock frequency MHz
SCK
Slave data: 32 bits - 64xFs
SAI clock frequency duty
D Slave receiver 30 70 %
SCK cycle
t FS valid time Master mode 8 22
v(FS)
t FS setup time Slave mode 2 -
su(FS)
Master mode 8 -
t FS hold time
h(FS)
Slave mode 0 -
t Master receiver 5 -
su(SD_MR)
Data input setup time
t Slave receiver 3 -
su(SD_SR)
t Master receiver 0 - ns
h(SD_MR)
Data input hold time
t Slave receiver 0 -
h(SD_SR)
t v(SD_ST) Slave transmitter (after enable - 22
t edge)
h(SD_ST)
Data output valid time
Master transmitter (after enable
t - 20
v(SD_MT) edge)
Master transmitter (after enable
t Data output hold time 8 -
h(SD_MT) edge)
1. Evaluated by characterization.
2. 256xFs maximum corresponds to 45MHz (APB2 maximum frequency)
DS9405 Rev 13 149/240
197

<!-- Page 150 -->


| S | lot n |
| --- | --- |


| Sl | ot n |
| --- | --- |

Electrical characteristics STM32F427xx STM32F429xx
Figure 43. SAI master timing waveforms
1/fSCK
SAI_SCK_X
th(FS)
SAI_FS_X
(output) tv(FS) tv(SD_MT) th(SD_MT)
SAI_SD_X
Slot n Slot n+2
(transmit)
tsu(SD_MR) th(SD_MR)
SAI_SD_X Slot n
(receive)
MS32771V1
Figure 44. SAI slave timing waveforms
1/fSCK
SAI_SCK_X
tw(CKH_X) tw(CKL_X) th(FS)
SAI_FS_X
(input) tsu(FS) tv(SD_ST) th(SD_ST)
SAI_SD_X
Slot n Slot n+2
(transmit)
tsu(SD_SR) th(SD_SR)
SAI_SD_X Slot n
(receive)
MS32772V1
150/240 DS9405 Rev 13

<!-- Page 151 -->


| Symbol | Parameter | Max | Unit |
| --- | --- | --- | --- |
| t (1) STARTUP | USB OTG full speed transceiver startup time | 1 | µs |


| Symbol |  | Parameter | Conditions | Min.(1) | Typ. | Max.(1) | Unit |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Input levels | V DD | USB OTG full speed transceiver operating voltage |  | 3.0(2) | - | 3.6 | V |
|  | V (3) DI | Differential input sensitivity | I(USB_FS_DP/DM, USB_HS_DP/DM) | 0.2 | - | - | V |
|  | V (3) CM | Differential common mode range | Includes V range DI | 0.8 | - | 2.5 |  |
|  | V (3) SE | Single ended receiver threshold |  | 1.3 | - | 2.0 |  |
| Output levels | V OL | Static output level low | R of 1.5kΩ to 3.6V(4) L | - | - | 0.3 | V |
|  | V OH | Static output level high | R of 15kΩ to V (4) L SS | 2.8 | - | 3.6 |  |
| R PD |  | PA11, PA12, PB14, PB15 (USB_FS_DP/DM, USB_HS_DP/DM) | V = V IN DD | 17 | 21 | 24 | kΩ |
|  |  | PA9, PB13 (OTG_FS_VBUS, OTG_HS_VBUS) |  | 0.65 | 1.1 | 2.0 |  |
| R PU |  | PA12, PB15 (USB_FS_DP, USB_HS_DP) | V = V IN SS | 1.5 | 1.8 | 2.1 |  |
|  |  | PA9, PB13 (OTG_FS_VBUS, OTG_HS_VBUS) | V = V IN SS | 0.25 | 0.37 | 0.55 |  |

STM32F427xx STM32F429xx Electrical characteristics
USB OTG full speed (FS) characteristics
This interface is present in both the USB OTG HS and USB OTG FS controllers.
Table 66. USB OTG full speed startup time
Symbol Parameter Max Unit
t (1) USB OTG full speed transceiver startup time 1 µs
STARTUP
1. Specified by design.
Table 67. USB OTG full speed DC electrical characteristics
Symbol Parameter Conditions Min.(1) Typ. Max.(1) Unit
USB OTG full speed
V transceiver operating 3.0(2) - 3.6 V
DD
voltage
I(USB_FS_DP/DM,
V (3) Differential input sensitivity 0.2 - -
Input DI USB_HS_DP/DM)
levels
Differential common mode
V (3) Includes V range 0.8 - 2.5 V
CM range DI
Single ended receiver
V (3) 1.3 - 2.0
SE threshold
Output V OL Static output level low R L of 1.5kΩ to 3.6V(4) - - 0.3
V
levels V Static output level high R of 15kΩ to V (4) 2.8 - 3.6
OH L SS
PA11, PA12, PB14, PB15
(USB_FS_DP/DM, 17 21 24
USB_HS_DP/DM)
R V = V
PD IN DD
PA9, PB13
(OTG_FS_VBUS, 0.65 1.1 2.0
OTG_HS_VBUS) kΩ
PA12, PB15 (USB_FS_DP,
V = V 1.5 1.8 2.1
USB_HS_DP) IN SS
R PU PA9, PB13
(OTG_FS_VBUS, V = V 0.25 0.37 0.55
IN SS
OTG_HS_VBUS)
1. All the voltages are measured from the local ground potential.
2. The USB OTG full speed transceiver functionality is ensured down to 2.7 V but not the full USB full speed
electrical characteristics, which are degraded in the 2.7-to-3.0 V V voltage range.
DD
3. Specified by design.
4. R L is the load connected on the USB OTG full speed drivers.
Note: When the VBUS sensing feature is enabled, PA9 and PB13 should be left at their default
state (floating input), not as alternate function. A typical 200µA current consumption of the
sensing block (current-to-voltage conversion to determine the different sessions) can be
observed on PA9 and PB13 when the feature is enabled.
DS9405 Rev 13 151/240
197

<!-- Page 152 -->


|  |  |  |
| --- | --- | --- |
|  |  |  |


| Driver characteristics |  |  |  |  |  |
| --- | --- | --- | --- | --- | --- |
| Symbol | Parameter | Conditions | Min | Max | Unit |
| t r | Rise time(2) | C = 50 pF L | 4 | 20 | ns |
| t f | Fall time(2) | C = 50 pF L | 4 | 20 | ns |
| t rfm | Rise/ fall time matching | t/t r f | 90 | 110 | % |
| V CRS | Output signal crossover voltage | - | 1.3 | 2.0 | V |
| Z DRV | Output driver impedance(3) | Driving high or low | 28 | 44 | Ω |


| Symbol |  | Parameter | Min.(1) | Max.(1) | Unit |
| --- | --- | --- | --- | --- | --- |
| Input level | V DD | USB OTG HS operating voltage | 1.7 | 3.6 | V |

Electrical characteristics STM32F427xx STM32F429xx
Figure 45. USB OTG full speed timings: definition of data signal rise and fall time
Cross over
points
Differential
data lines
VCRS
VSS
tf tr
ai14137b
Table 68. USB OTG full speed electrical characteristics(1)
Driver characteristics
Symbol Parameter Conditions Min Max Unit
t Rise time(2) C = 50 pF 4 20 ns
r L
t Fall time(2) C = 50 pF 4 20 ns
f L
t Rise/ fall time matching t/t 90 110 %
rfm r f
V Output signal crossover voltage - 1.3 2.0 V
CRS
Driving high or
Z Output driver impedance(3) 28 44 Ω
DRV low
1. Specified by design.
2. Measured from 10% to 90% of the data signal. For more detailed information, refer to USB Specification -
Chapter 7 (version 2.0).
3. No external termination series resistors are required on DP (D+) and DM (D-) pins since the matching
impedance is included in the embedded driver.
USB high speed (HS) characteristics
Unless otherwise specified, the parameters given in Table71 for ULPI are derived from
tests performed under the ambient temperature, f frequency summarized in Table70
HCLK
and V supply voltage conditions summarized in Table69, with the following configuration:
DD
• Output speed is set to OSPEEDRy[1:0] = 10, unless otherwise specified
• Capacitive load C = 30pF, unless otherwise specified
• Measurement points are done at CMOS levels: 0.5V .
DD
Refer to Section6.3.17: I/O port characteristics for more details on the input/output
characteristics.
Table 69. USB HS DC electrical characteristics
Symbol Parameter Min.(1) Max.(1) Unit
Input level V USB OTG HS operating voltage 1.7 3.6 V
DD
1. All the voltages are measured from the local ground potential.
152/240 DS9405 Rev 13

<!-- Page 153 -->


| Symbol | Parameter |  | Min | Typ | Max | Unit |
| --- | --- | --- | --- | --- | --- | --- |
|  | f value to guarantee proper operation of HCLK USB HS interface |  | 30 | - | - | MHz |
| F START_8BIT | Frequency (first transition) | 8-bit ±10% | 54 | 60 | 66 | MHz |
| F STEADY | Frequency (steady state) ±500ppm |  | 59.97 | 60 | 60.03 | MHz |
| D START_8BIT | Duty cycle (first transition) | 8-bit ±10% | 40 | 50 | 60 | % |
| D STEADY | Duty cycle (steady state) ±500ppm |  | 49.975 | 50 | 50.025 | % |
| t STEADY | Time to reach the steady state frequency and duty cycle after the first transition |  | - | - | 1.4 | ms |
| t START_DEV | Clock startup time after the de-assertion of SuspendM | Peripheral | - | - | 5.6 | ms |
| t START_HOST |  | Host | - | - | - |  |
| t PREP | PHY preparation time after the first transition of the input clock |  | - | - | - | µs |


|  |  |  |
| --- | --- | --- |
|  |  |  |


|  |  |
| --- | --- |
|  |  |

STM32F427xx STM32F429xx Electrical characteristics
Table 70. USB HS clock timing parameters(1)
Symbol Parameter Min Typ Max Unit
f value to guarantee proper operation of
HCLK 30 - - MHz
USB HS interface
F Frequency (first transition) 8-bit ±10% 54 60 66 MHz
START_8BIT
F Frequency (steady state) ±500ppm 59.97 60 60.03 MHz
STEADY
D Duty cycle (first transition) 8-bit ±10% 40 50 60 %
START_8BIT
D Duty cycle (steady state) ±500ppm 49.975 50 50.025 %
STEADY
Time to reach the steady state frequency and
t - - 1.4 ms
STEADY duty cycle after the first transition
t START_DEV Clock startup time after the Peripheral - - 5.6
ms
t de-assertion of SuspendM Host - - -
START_HOST
PHY preparation time after the first transition
t - - - µs
PREP of the input clock
1. Specified by design.
Figure 46. ULPI timing diagram
Clock
t
SC
tHC
Control In
(ULPI_DIR,
ULPI_NXT) tSD tHD
data In
(8-bit)
t DC t DC
Control out
(ULPI_STP)
tDD
data out
(8-bit)
ai17361c
DS9405 Rev 13 153/240
197

<!-- Page 154 -->


| Symbol | Parameter | Conditions | Min. | Typ. | Max. | Unit |
| --- | --- | --- | --- | --- | --- | --- |
| t SC | Control in (ULPI_DIR, ULPI_NXT) setup time | - | 2 | - | - | ns |
| t HC | Control in (ULPI_DIR, ULPI_NXT) hold time | - | 0.5 | - | - |  |
| t SD | Data in setup time | - | 1.5 | - | - |  |
| t HD | Data in hold time | - | 2 | - | - |  |
| t /t DC DD | Data/control output delay | 2.7V < V < 3.6V, DD C = 15pF and L OSPEEDRy[1:0] = 11 | - | 9 | 9.5 |  |
|  |  | 2.7V < V < 3.6V, DD C = 20pF and L OSPEEDRy[1:0] = 10 | - | 12 | 15 |  |
|  |  | 1.7V < V < 3.6V, DD C = 15pF and L OSPEEDRy[1:0] = 11 | - |  |  |  |

Electrical characteristics STM32F427xx STM32F429xx
Table 71. Dynamic characteristics: USB ULPI(1)
Symbol Parameter Conditions Min. Typ. Max. Unit
t Control in (ULPI_DIR, ULPI_NXT) setup time - 2 - -
SC
t Control in (ULPI_DIR, ULPI_NXT) hold time - 0.5 - -
HC
t Data in setup time - 1.5 - -
SD
t Data in hold time - 2 - -
HD
2.7V < V < 3.6V,
DD
C = 15pF and - 9 9.5
L
ns
OSPEEDRy[1:0] = 11
2.7V < V < 3.6V,
DD
t /t Data/control output delay C = 20pF and -
DC DD L
OSPEEDRy[1:0] = 10
12 15
1.7V < V < 3.6V,
DD
C = 15pF and -
L
OSPEEDRy[1:0] = 11
1. Specified by characterization.
154/240 DS9405 Rev 13

<!-- Page 155 -->


| Symbol | Parameter | Min | Typ | Max | Unit |
| --- | --- | --- | --- | --- | --- |
| t MDC | MDC cycle time(2.38 MHz) | 411 | 420 | 425 | ns |
| T d(MDIO) | Write data valid time | 6 | 10 | 13 |  |
| t su(MDIO) | Read data setup time | 12 | - | - |  |
| t h(MDIO) | Read data hold time | 0 | - | - |  |

STM32F427xx STM32F429xx Electrical characteristics
Ethernet characteristics
Unless otherwise specified, the parameters given in Table72, Table73 and Table74 for
SMI, RMII and MII are derived from tests performed under the ambient temperature, f
HCLK
frequency summarized in Table17 with the following configuration:
• Output speed is set to OSPEEDRy[1:0] = 10
• Capacitive load C = 30pF for 2.7V < V < 3.6V
DD
• Capacitive load C = 20pF for 1.71V < V < 3.6V
DD
• Measurement points are done at CMOS levels: 0.5V .
DD
Refer to Section6.3.17: I/O port characteristics for more details on the input/output
characteristics.
Table72 gives the list of Ethernet MAC signals for the SMI (station management interface)
and Figure47 shows the corresponding timing diagram.
Figure 47. Ethernet SMI timing diagram
tMDC
ETH_MDC
td(MDIO)
ETH_MDIO(O)
tsu(MDIO) th(MDIO)
ETH_MDIO(I)
MS31384V1
Table 72. Dynamics characteristics: Ethernet MAC signals for SMI(1)
Symbol Parameter Min Typ Max Unit
t MDC cycle time(2.38 MHz) 411 420 425
MDC
T Write data valid time 6 10 13
d(MDIO)
ns
t Read data setup time 12 - -
su(MDIO)
t Read data hold time 0 - -
h(MDIO)
1. Evaluated by characterization.
DS9405 Rev 13 155/240
197

<!-- Page 156 -->


| Symbol | Parameter | Condition | Min | Typ | Max | Unit |
| --- | --- | --- | --- | --- | --- | --- |
| t su(RXD) | Receive data setup time | 1.71V < V < 3.6V DD | 1.5 | - | - | ns |
| t ih(RXD) | Receive data hold time |  | 0 | - | - |  |
| t su(CRS) | Carrier sense setup time |  | 1 | - | - |  |
| t ih(CRS) | Carrier sense hold time |  | 1 | - | - |  |
| t d(TXEN) | Transmit enable valid delay time | 2.7V < V < 3.6V DD | 8 | 10.5 | 12 |  |
|  |  | 1.71V < V < 3.6V DD | 8 | 10.5 | 14 |  |
| t d(TXD) | Transmit data valid delay time | 2.7V < V < 3.6V DD | 8 | 11 | 12.5 |  |
|  |  | 1.71V < V < 3.6V DD | 8 | 11 | 14.5 |  |

Electrical characteristics STM32F427xx STM32F429xx
Table73 gives the list of Ethernet MAC signals for the RMII and Figure48 shows the
corresponding timing diagram.
Figure 48. Ethernet RMII timing diagram
RMII_REF_CLK
td(TXEN)
td(TXD)
RMII_TX_EN
RMII_TXD[1:0]
tsu(RXD) tih(RXD)
tsu(CRS) tih(CRS)
RMII_RXD[1:0]
RMII_CRS_DV
ai15667b
Table 73. Dynamics characteristics: Ethernet MAC signals for RMII(1)
Symbol Parameter Condition Min Typ Max Unit
t Receive data setup time 1.5 - -
su(RXD)
t Receive data hold time 0 - -
ih(RXD)
1.71V < V < 3.6V
DD
t Carrier sense setup time 1 - -
su(CRS)
t Carrier sense hold time 1 - -
ih(CRS)
ns
Transmit enable valid delay 2.7V < V DD < 3.6V 8 10.5 12
t
d(TXEN) time 1.71V < V < 3.6V 8 10.5 14
DD
2.7V < V < 3.6V 8 11 12.5
DD
t Transmit data valid delay time
d(TXD)
1.71V < V < 3.6V 8 11 14.5
DD
1. Evaluated by characterization results.
Table74 gives the list of Ethernet MAC signals for MII and Figure48 shows the
corresponding timing diagram.
156/240 DS9405 Rev 13

<!-- Page 157 -->


| Symbol | Parameter | Condition | Min | Typ | Max | Unit |
| --- | --- | --- | --- | --- | --- | --- |
| t su(RXD) | Receive data setup time | 1.71V < V < 3.6V DD | 9 | - | - | ns |
| t ih(RXD) | Receive data hold time |  | 10 | - | - |  |
| t su(DV) | Data valid setup time |  | 9 | - | - |  |
| t ih(DV) | Data valid hold time |  | 8 | - | - |  |
| t su(ER) | Error setup time |  | 6 | - | - |  |
| t ih(ER) | Error hold time |  | 8 | - | - |  |
| t d(TXEN) | Transmit enable valid delay time | 2.7V < V < 3.6V DD | 8 | 10 | 14 |  |
|  |  | 1.71V < V < 3.6V DD | 8 | 10 | 16 |  |
| t d(TXD) | Transmit data valid delay time | 2.7V < V < 3.6V DD | 7.5 | 10 | 15 |  |
|  |  | 1.71V < V < 3.6V DD | 7.5 | 10 | 17 |  |

STM32F427xx STM32F429xx Electrical characteristics
Figure 49. Ethernet MII timing diagram
MII_RX_CLK
tsu(RXD) tih(RXD)
tsu(ER) tih(ER)
tsu(DV) tih(DV)
MII_RXD[3:0]
MII_RX_DV
MII_RX_ER
MII_TX_CLK
td(TXEN)
td(TXD)
MII_TX_EN
MII_TXD[3:0]
ai15668b
Table 74. Dynamics characteristics: Ethernet MAC signals for MII(1)
Symbol Parameter Condition Min Typ Max Unit
t Receive data setup time 9 - -
su(RXD)
t Receive data hold time 10 - -
ih(RXD)
t Data valid setup time 9 - -
su(DV)
1.71V < V < 3.6V
DD
t Data valid hold time 8 - -
ih(DV)
t Error setup time 6 - -
su(ER)
ns
t Error hold time 8 - -
ih(ER)
2.7V < V < 3.6V 8 10 14
DD
t Transmit enable valid delay time
d(TXEN)
1.71V < V < 3.6V 8 10 16
DD
2.7V < V < 3.6V 7.5 10 15
DD
t Transmit data valid delay time
d(TXD)
1.71V < V < 3.6V 7.5 10 17
DD
1. Evaluated by characterization.
CAN (controller area network) interface
Refer to Section6.3.17: I/O port characteristics for more details on the input/output alternate
function characteristics (CANx_TX and CANx_RX).
DS9405 Rev 13 157/240
197

<!-- Page 158 -->


| Symbol | Parameter | Conditions | Min | Typ | Max | Unit |
| --- | --- | --- | --- | --- | --- | --- |
| V DDA | Power supply | V − V < 1.2V DDA REF+ | 1.7(1) | - | 3.6 | V |
| V REF+ | Positive reference voltage |  | 1.7(1) | - | V DDA |  |
| V REF- | Negative reference voltage | - | - | 0 | - |  |
| f ADC | ADC clock frequency | V = 1.7(1) to 2.4V DDA | 0.6 | 15 | 18 | MHz |
|  |  | V = 2.4 to 3.6V DDA | 0.6 | 30 | 36 | MHz |
| f (2) TRIG | External trigger frequency | f = 30MHz, ADC 12-bit resolution | - | - | 1764 | kHz |
|  |  | - | - | - | 17 | 1/f ADC |
| V AIN | Conversion voltage range(3) | - | 0 (V or V SSA REF- tied to ground) | - | V REF+ | V |
| R (2) AIN | External input impedance | See Equation 1 for details | - | - | 50 | kΩ |
| R (2)(4) ADC | Sampling switch resistance | - | 1.5 | - | 6 | kΩ |
| C (2) ADC | Internal sample and hold capacitor | - | - | 4 | 7 | pF |
| t (2) lat | Injection trigger conversion latency | f = 30MHz ADC | - | - | 0.100 | µs |
|  |  | - | - | - | 3(5) | 1/f ADC |
| t (2) latr | Regular trigger conversion latency | f = 30MHz ADC | - | - | 0.067 | µs |
|  |  | - | - | - | 2(5) | 1/f ADC |
| t (2) S | Sampling time | f = 30MHz ADC | 0.100 | - | 16 | µs |
|  |  | - | 3 | - | 480 | 1/f ADC |
| t (2) STAB | Power-up time | - | - | 2 | 3 | µs |
| t (2) CONV | Total conversion time (including sampling time) | f = 30MHz ADC 12-bit resolution | 0.50 | - | 16.40 | µs |
|  |  | f = 30MHz ADC 10-bit resolution | 0.43 | - | 16.34 | µs |
|  |  | f = 30MHz ADC 8-bit resolution | 0.37 | - | 16.27 | µs |
|  |  | f = 30MHz ADC 6-bit resolution | 0.30 | - | 16.20 | µs |
|  |  | 9 to 492 (t for sampling +n-bit resolution for successive S approximation) |  |  |  | 1/f ADC |

Electrical characteristics STM32F427xx STM32F429xx
6.3.21 12-bit ADC characteristics
Unless otherwise specified, the parameters given in Table75 are derived from tests
performed under the ambient temperature, f frequency and V supply voltage
PCLK2 DDA
conditions summarized in Table17.
Table 75. ADC characteristics
Symbol Parameter Conditions Min Typ Max Unit
V Power supply 1.7(1) - 3.6
DDA
V − V < 1.2V
V Positive reference voltage DDA REF+ 1.7(1) - V V
REF+ DDA
V Negative reference voltage - - 0 -
REF-
V = 1.7(1) to 2.4V 0.6 15 18 MHz
DDA
f ADC clock frequency
ADC
V = 2.4 to 3.6V 0.6 30 36 MHz
DDA
f = 30MHz,
ADC - - 1764 kHz
f (2) External trigger frequency 12-bit resolution
TRIG
- - - 17 1/f
ADC
0
V AIN Conversion voltage range(3) - (V SSA or V REF- - V REF+ V
tied to ground)
See Equation 1 for
R (2) External input impedance - - 50 kΩ
AIN details
R (2)(4) Sampling switch resistance - 1.5 - 6 kΩ
ADC
Internal sample and hold
C (2) - - 4 7 pF
ADC capacitor
t (2) Injection trigger conversion f ADC = 30MHz - - 0.100 µs
lat latency - - - 3(5) 1/f
ADC
t (2) Regular trigger conversion f ADC = 30MHz - - 0.067 µs
latr latency - - - 2(5) 1/f
ADC
f = 30MHz 0.100 - 16 µs
t (2) Sampling time ADC
S
- 3 - 480 1/f
ADC
t (2) Power-up time - - 2 3 µs
STAB
f = 30MHz
ADC 0.50 - 16.40 µs
12-bit resolution
f = 30MHz
ADC 0.43 - 16.34 µs
10-bit resolution
Total conversion time (including f = 30MHz
t CONV (2) sampling time) 8 A - D b C it resolution 0.37 - 16.27 µs
f = 30MHz
ADC 0.30 - 16.20 µs
6-bit resolution
9 to 492 (t for sampling +n-bit resolution for successive
S 1/f
approximation) ADC
158/240 DS9405 Rev 13

<!-- Page 159 -->


| Symbol | Parameter | Conditions | Min | Typ | Max | Unit |
| --- | --- | --- | --- | --- | --- | --- |
| f (2) S | Sampling rate (f = 30 MHz, and ADC t = 3 ADC cycles) S | 12-bit resolution Single ADC | - | - | 2 | Msps |
|  |  | 12-bit resolution Interleave Dual ADC mode | - | - | 3.75 | Msps |
|  |  | 12-bit resolution Interleave Triple ADC mode | - | - | 6 | Msps |
| I (2) VREF+ | ADC V DC current REF consumption in conversion mode | - | - | 300 | 500 | µA |
| I (2) VDDA | ADC V DC current DDA consumption in conversion mode | - | - | 1.6 | 1.8 | mA |


| Symbol | Parameter | Test conditions | Typ | Max(1) | Unit |
| --- | --- | --- | --- | --- | --- |
| ET | Total unadjusted error | f =18MHz ADC V = 1.7 to 3.6V DDA V = 1.7 to 3.6V REF V − V < 1.2V DDA REF | ±3 | ±4 | LSB |
| EO | Offset error |  | ±2 | ±3 |  |
| EG | Gain error |  | ±1 | ±3 |  |
| ED | Differential linearity error |  | ±1 | ±2 |  |
| EL | Integral linearity error |  | ±2 | ±3 |  |

STM32F427xx STM32F429xx Electrical characteristics
Table 75. ADC characteristics (continued)
Symbol Parameter Conditions Min Typ Max Unit
12-bit resolution
- - 2 Msps
Single ADC
Sampling rate 12-bit resolution
f (2) (f = 30 MHz, and Interleave Dual ADC - - 3.75 Msps
S ADC mode
t = 3 ADC cycles)
S
12-bit resolution
Interleave Triple ADC - - 6 Msps
mode
ADC V DC current
REF
I (2) consumption in conversion - - 300 500 µA
VREF+
mode
ADC V DC current
DDA
I (2) consumption in conversion - - 1.6 1.8 mA
VDDA
mode
1. V minimum value of 1.7V is obtained with the use of an external power supply supervisor (refer to Section3.17.2:
DDA
Internal reset OFF).
2. Evaluated by characterization results.
3. V is internally connected to V and V is internally connected to V .
REF+ DDA REF- SSA
4. R maximum value is given for V =1.7V, and a minimum value for V =3.3V.
ADC DD DD
5. For external triggers, a delay of 1/f must be added to the latency specified in Table75.
PCLK2
Equation 1: R max formula
AIN
(k–0.5)
R = ----------------------------------------------------------------–R
AIN
f × C × ln(2
N+2
)
ADC
ADC ADC
The formula above (Equation 1) is used to determine the maximum external impedance
allowed for an error below 1/4 of LSB. N = 12 (from 12-bit resolution) and k is the number of
sampling periods defined in the ADC_SMPR1 register.
Table 76. ADC static accuracy at f = 18MHz
ADC
Symbol Parameter Test conditions Typ Max(1) Unit
ET Total unadjusted error ±3 ±4
f =18MHz
ADC
EO Offset error ±2 ±3
V = 1.7 to 3.6V
DDA LSB
EG Gain error V = 1.7 to 3.6V ±1 ±3
REF
ED Differential linearity error V DDA − V REF < 1.2V ±1 ±2
EL Integral linearity error ±2 ±3
1. Evaluated by characterization - not tested in production results.
DS9405 Rev 13 159/240
197

<!-- Page 160 -->


| Symbol | Parameter | Test conditions | Typ | Max(1) | Unit |
| --- | --- | --- | --- | --- | --- |
| ET | Total unadjusted error | f = 30 MHz, ADC R < 10 kΩ AIN V = 2.4 to 3.6 V, DDA V = 1.7 to 3.6 V, REF V − V < 1.2V DDA REF | ±2 | ±5 | LSB |
| EO | Offset error |  | ±1.5 | ±2.5 |  |
| EG | Gain error |  | ±1.5 | ±3 |  |
| ED | Differential linearity error |  | ±1 | ±2 |  |
| EL | Integral linearity error |  | ±1.5 | ±3 |  |


| Symbol | Parameter | Test conditions | Typ | Max(1) | Unit |
| --- | --- | --- | --- | --- | --- |
| ET | Total unadjusted error | f =36MHz, ADC V = 2.4 to 3.6V, DDA V = 1.7 to 3.6V REF V − V < 1.2V DDA REF | ±4 | ±7 | LSB |
| EO | Offset error |  | ±2 | ±3 |  |
| EG | Gain error |  | ±3 | ±6 |  |
| ED | Differential linearity error |  | ±2 | ±3 |  |
| EL | Integral linearity error |  | ±3 | ±6 |  |


| Symbol | Parameter | Test conditions | Min | Typ | Max | Unit |
| --- | --- | --- | --- | --- | --- | --- |
| ENOB | Effective number of bits | f =18MHz ADC V = V = 1.7V DDA REF+ Input Frequency = 20KHz Temperature = 25°C | 10.3 | 10.4 | - | bits |
| SINAD | Signal-to-noise and distortion ratio |  | 64 | 64.2 | - | dB |
| SNR | Signal-to-noise ratio |  | 64 | 65 | - |  |
| THD | Total harmonic distortion |  | −67 | −72 | - |  |


| Symbol | Parameter | Test conditions | Min | Typ | Max | Unit |
| --- | --- | --- | --- | --- | --- | --- |
| ENOB | Effective number of bits | f =36MHz ADC V = V = 3.3V DDA REF+ Input Frequency = 20KHz Temperature = 25°C | 10.6 | 10.8 | - | bits |
| SINAD | Signal-to noise and distortion ratio |  | 66 | 67 | - | dB |
| SNR | Signal-to noise ratio |  | 64 | 68 | - |  |
| THD | Total harmonic distortion |  | −70 | −72 | - |  |

Electrical characteristics STM32F427xx STM32F429xx
a Table 77. ADC static accuracy at f = 30MHz
ADC
Symbol Parameter Test conditions Typ Max(1) Unit
ET Total unadjusted error ±2 ±5
f = 30 MHz,
EO Offset error ADC ±1.5 ±2.5
R < 10 kΩ
AIN
EG Gain error V = 2.4 to 3.6 V, ±1.5 ±3 LSB
DDA
V = 1.7 to 3.6 V,
ED Differential linearity error REF ±1 ±2
V − V < 1.2V
DDA REF
EL Integral linearity error ±1.5 ±3
1. Evaluated by characterization.
Table 78. ADC static accuracy at f = 36MHz
ADC
Symbol Parameter Test conditions Typ Max(1) Unit
ET Total unadjusted error ±4 ±7
f =36MHz,
EO Offset error ADC ±2 ±3
V = 2.4 to 3.6V,
EG Gain error DDA ±3 ±6 LSB
V = 1.7 to 3.6V
REF
ED Differential linearity error V − V < 1.2V ±2 ±3
DDA REF
EL Integral linearity error ±3 ±6
1. Evaluated by characterization.
Ta b le 79. ADC dynamic accuracy at f = 18MHz - limited test conditions(1)
ADC
Symbol Parameter Test conditions Min Typ Max Unit
ENOB Effective number of bits 10.3 10.4 - bits
f =18MHz
ADC
SINAD Signal-to-noise and distortion ratio V = V = 1.7V 64 64.2 -
DDA REF+
SNR Signal-to-noise ratio Input Frequency = 20KHz 64 65 - dB
Temperature = 25°C
THD Total harmonic distortion −67 −72 -
1. Evaluated by characterization.
Ta b le 80. ADC dynamic accuracy at f = 36MHz - limited test conditions(1)
ADC
Symbol Parameter Test conditions Min Typ Max Unit
ENOB Effective number of bits 10.6 10.8 - bits
f =36MHz
ADC
SINAD Signal-to noise and distortion ratio V = V = 3.3V 66 67 -
DDA REF+
SNR Signal-to noise ratio Input Frequency = 20KHz 64 68 - dB
Temperature = 25°C
THD Total harmonic distortion −70 −72 -
1. Evaluated by characterization.
160/240 DS9405 Rev 13

<!-- Page 161 -->

STM32F427xx STM32F429xx Electrical characteristics
Note: ADC accuracy vs. negative injection current: injecting a negative current on any analog
input pins should be avoided as this significantly reduces the accuracy of the conversion
being performed on another analog input. It is recommended to add a Schottky diode (pin to
ground) to analog pins, which may potentially inject negative currents.
Any positive injection current within the limits specified for I and ∑I in
INJ(PIN) INJ(PIN)
Section6.3.17 does not affect the ADC accuracy.
Figure 50. ADC accuracy characteristics
VREF+ VDDA
[1LSB IDEAL = (or depending on package)]
4096 4096
EG
4095
4094
4093
(2)
ET
(3)
7
(1)
6
5
EO EL
4
3
ED
2
1
1LSBIDEAL
0
1 2 3 456 7 4093 4094 4095 4096
VSSA VDDA
ai14395c
1. See also Table77.
2. Example of an actual transfer curve.
3. Ideal transfer curve.
4. End-point correlation line.
5. E = Total Unadjusted Error: maximum deviation between the actual and the ideal transfer curves.
T
EO = Offset Error: deviation between the first actual transition and the first ideal one.
EG = Gain Error: deviation between the last ideal transition and the last actual one.
ED = Differential Linearity Error: maximum deviation between actual steps and the ideal one.
EL = Integral Linearity Error: maximum deviation between any actual transition and the end-point
correlation line.
DS9405 Rev 13 161/240
197

<!-- Page 162 -->


|  |  |
| --- | --- |


|  |  |
| --- | --- |

Electrical characteristics STM32F427xx STM32F429xx
Figure 51. Typical connection diagram when using the ADC with FT/TT pins
featuring the analog switch function
VDDA (4) VREF+ (4)
I/O Sample-and-hold ADC converter
analog
RAIN (1) switch RADC
Converter
VAIN
Cparasitic (2) Ilkg (3)
Sampling
CADC
switch with
multiplexing
VSS VSS VSSA
MSv67871V3
1. Refer to Table75: ADC characteristics for the values of R , RADC, and C .
AIN ADC
2. C represents the capacitance of the PCB (dependent on soldering and PCB layout quality) plus the
parasitic
pad capacitance (refer to Table57: I/O static characteristics). A high C value downgrades
parasitic
conversion accuracy. To remedy this, f should be reduced.
ADC
3. Refer to Table57: I/O static characteristics for the value of I
Ikg.
4. Refer to Figure22: Power supply scheme.
162/240 DS9405 Rev 13

<!-- Page 163 -->


|  |  |
| --- | --- |


|  |  |
| --- | --- |


|  |  |
| --- | --- |

STM32F427xx STM32F429xx Electrical characteristics
General PCB design guidelines
Power supply decoupling should be performed as shown in Figure52 or Figure53,
depending on whether V is connected to V or not. The 10 nF capacitors should be
REF+ DDA
ceramic (good quality). They should be placed them as close as possible to the chip.
Figure 52. Power supply and reference decoupling (V not connected to V )
REF+ DDA
STM32F
VREF+ (1)
1 μF // 10 nF
VDDA
1 μF // 10 nF
(1)
VSSA/VREF-
ai17535b
1. V and V inputs are both available on UFBGA176. V is also available on LQFP100, LQFP144,
REF+ REF– REF+
and LQFP176. When V and V are not available, they are internally connected to V and V .
REF+ REF– DDA SSA
DS9405 Rev 13 163/240
197

<!-- Page 164 -->


|  |  |
| --- | --- |


|  |  |
| --- | --- |


| Symbol | Parameter | Min | Typ | Max | Unit |
| --- | --- | --- | --- | --- | --- |
| T (1) L | V linearity with temperature SENSE | - | ±1 | ±2 | °C |
| Avg_Slope(1) | Average slope | - | 2.5 | - | mV/°C |
| V (1) 25 | Voltage at 25 °C | - | 0.76 | - | V |
| t (2) START | Startup time | - | 6 | 10 | µs |
| T (2) S_temp | ADC sampling time when reading the temperature (1°C accuracy) | 10 | - | - | µs |


| Symbol | Parameter | Memory address |
| --- | --- | --- |
| TS_CAL1 | TS ADC raw data acquired at temperature of 30°C, V = 3.3V DDA | 0x1FFF7A2C - 0x1FFF7A2D |
| TS_CAL2 | TS ADC raw data acquired at temperature of 110°C, V = 3.3V DDA | 0x1FFF7A2E - 0x1FFF7A2F |

Electrical characteristics STM32F427xx STM32F429xx
Figure 53. Power supply and reference decoupling (V connected to V )
REF+ DDA
STM32F
VREF+/VDDA (1)
1 μF // 10 nF
VREF-/VSSA (1)
ai17536c
1. V and V inputs are both available on UFBGA176. V is also available on LQFP100, LQFP144,
REF+ REF– REF+
and LQFP176. When V and V are not available, they are internally connected to V and V .
REF+ REF– DDA SSA
6.3.22 Temperature sensor characteristics
Table 81. Temperature sensor characteristics
Symbol Parameter Min Typ Max Unit
T (1) V linearity with temperature - ±1 ±2 °C
L SENSE
Avg_Slope(1) Average slope - 2.5 - mV/°C
V (1) Voltage at 25 °C - 0.76 - V
25
t (2) Startup time - 6 10 µs
START
T (2) ADC sampling time when reading the temperature (1°C accuracy) 10 - - µs
S_temp
1. Evaluated by characterization.
2. Specified by design.
Table 82. Temperature sensor calibration values
Symbol Parameter Memory address
TS_CAL1 TS ADC raw data acquired at temperature of 30°C, V = 3.3V 0x1FFF7A2C - 0x1FFF7A2D
DDA
TS_CAL2 TS ADC raw data acquired at temperature of 110°C, V = 3.3V 0x1FFF7A2E - 0x1FFF7A2F
DDA
164/240 DS9405 Rev 13

<!-- Page 165 -->


| Symbol | Parameter | Min | Typ | Max | Unit |
| --- | --- | --- | --- | --- | --- |
| R | Resistor bridge for V BAT | - | 50 | - | KΩ |
| Q | Ratio on V measurement BAT | - | 4 | - | - |
| Er(1) | Error on Q | –1 | - | +1 | % |
| T (2)(2) S_vbat | ADC sampling time when reading the V BAT 1mV accuracy | 5 | - | - | µs |


| Symbol | Parameter | Conditions | Min | Typ | Max | Unit |
| --- | --- | --- | --- | --- | --- | --- |
| V REFINT | Internal reference voltage | –40 °C < T < +105°C A | 1.18 | 1.21 | 1.24 | V |
| T (1) S_vrefint | ADC sampling time when reading the internal reference voltage | - | 10 | - | - | µs |
| V (2) RERINT_s | Internal reference voltage spread over the temperature range | V = 3V ± 10mV DD | - | 3 | 5 | mV |
| T (2) Coeff | Temperature coefficient | - | - | 30 | 50 | ppm/°C |
| t (2) START | Startup time | - | - | 6 | 10 | µs |


| Symbol | Parameter | Memory address |
| --- | --- | --- |
| V REFIN_CAL | Raw data acquired at temperature of 30°C = 3.3V VDDA | 0x1FFF7A2A - 0x1FFF7A2B |

STM32F427xx STM32F429xx Electrical characteristics
6.3.23 V monitoring characteristics
BAT
Table 83. V monitoring characteristics
BAT
Symbol Parameter Min Typ Max Unit
R Resistor bridge for V - 50 - KΩ
BAT
Q Ratio on V measurement - 4 - -
BAT
Er(1) Error on Q –1 - +1 %
ADC sampling time when reading the V
T (2)(2) BAT 5 - - µs
S_vbat 1mV accuracy
1. Specified by design.
2. The shortest sampling time can be determined in the application by multiple iterations.
6.3.24 Reference voltage
The parameters given in Table84 are derived from tests performed under ambient
temperature and V supply voltage conditions summarized in Table17.
DD
Table 84. internal reference voltage
Symbol Parameter Conditions Min Typ Max Unit
V Internal reference voltage –40 °C < T < +105°C 1.18 1.21 1.24 V
REFINT A
ADC sampling time when reading the
T (1) - 10 - - µs
S_vrefint internal reference voltage
Internal reference voltage spread over the
V (2) V = 3V ± 10mV - 3 5 mV
RERINT_s temperature range DD
T (2) Temperature coefficient - - 30 50 ppm/°C
Coeff
t (2) Startup time - - 6 10 µs
START
1. The shortest sampling time can be determined in the application by multiple iterations.
2. Specified by design, not tested in production
Table 85. Internal reference voltage calibration values
Symbol Parameter Memory address
V Raw data acquired at temperature of 30°C = 3.3V 0x1FFF7A2A - 0x1FFF7A2B
REFIN_CAL VDDA
DS9405 Rev 13 165/240
197

<!-- Page 166 -->


| Symbol | Parameter | Conditions |  | Min | Typ | Max | Unit | Comments |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| V DDA | Analog supply voltage | - |  | 1.7(1) | - | 3.6 | V | - |
| V REF+ | Reference supply voltage | - |  | 1.7(1) | - | 3.6 | V | V ≤ V REF+ DDA |
| V SSA | Ground | - |  | 0 | - | 0 | V | - |
| R (2) LOAD | Resistive load | DAC output buffer ON | R LOAD connected to V SSA | 5 | - | - | kΩ | - |
|  |  |  | R LOAD connected to V DDA | 25 |  |  |  | - |
| R (2) O | Impedance output with buffer OFF | - |  | - | - | 15 | kΩ | When the buffer is OFF, the Minimum resistive load between DAC_OUT and V SS to have a 1% accuracy is 1.5MΩ |
| C (2) LOAD | Capacitive load | - |  | - | - | 50 | pF | Maximum capacitive load at DAC_OUT pin (when the buffer is ON). |
| DAC_O UT min(2) | Lower DAC_OUT voltage with buffer ON | - |  | 0.2 | - | - | V | It gives the maximum output excursion of the DAC. It corresponds to 12-bit input code (0x0E0) to (0xF1C) at V = 3.6V and (0x1C7) to REF+ (0xE38) at V = 1.7V REF+ |
| DAC_O UT max(2) | Higher DAC_OUT voltage with buffer ON | - |  | - | - | V DDA − 0.2 | V |  |
| DAC_O UT min(2) | Lower DAC_OUT voltage with buffer OFF | - |  | - | 0.5 | - | mV | It gives the maximum output excursion of the DAC. |
| DAC_O UT max(2) | Higher DAC_OUT voltage with buffer OFF | - |  | - | - | V REF+ − 1LSB | V |  |
| I (4) VREF+ | DAC DC V current REF consumption in quiescent mode (Standby mode) | - |  | - | 170 | 240 | µA | With no load, worst code (0x800) at V = 3.6V in REF+ terms of DC consumption on the inputs |
|  |  | - |  | - | 50 | 75 |  | With no load, worst code (0xF1C) at V = 3.6V in REF+ terms of DC consumption on the inputs |

Electrical characteristics STM32F427xx STM32F429xx
6.3.25 DAC electrical characteristics
Table 86. DAC characteristics
Symbol Parameter Conditions Min Typ Max Unit Comments
V Analog supply voltage - 1.7(1) - 3.6 V -
DDA
Reference supply
V - 1.7(1) - 3.6 V V ≤ V
REF+ voltage REF+ DDA
V Ground - 0 - 0 V -
SSA
R
LOAD
connected 5 - - -
R (2) Resistive load DAC output to V SSA kΩ
LOAD buffer ON R
LOAD
connected 25 -
to V
DDA
When the buffer is OFF, the
Minimum resistive load
Impedance output with
R (2) - - - 15 kΩ between DAC_OUT and V
O buffer OFF SS
to have a 1% accuracy is
1.5MΩ
Maximum capacitive load at
C (2) Capacitive load - - - 50 pF DAC_OUT pin (when the
LOAD
buffer is ON).
DAC_O It gives the maximum output
Lower DAC_OUT
UT - 0.2 - - V excursion of the DAC.
voltage with buffer ON
min(2)
It corresponds to 12-bit input
code (0x0E0) to (0xF1C) at
DAC_O
UT Higher DAC_OUT - - - V DDA V V REF+ = 3.6V and (0x1C7) to
max(2) voltage with buffer ON − 0.2 (0xE38) at V REF+ = 1.7V
DAC_O Lower DAC_OUT
UT voltage with buffer - - 0.5 - mV
min(2) OFF It gives the maximum output
DAC_O Higher DAC_OUT V excursion of the DAC.
REF+
UT voltage with buffer - - - − V
max(2) OFF 1LSB
With no load, worst code
(0x800) at V = 3.6V in
- - 170 240 REF+
DAC DC V current terms of DC consumption on
REF
consumption in the inputs
I (4) µA
VREF+ quiescent mode
With no load, worst code
(Standby mode) (0xF1C) at V = 3.6V in
- - 50 75 REF+
terms of DC consumption on
the inputs
166/240 DS9405 Rev 13

<!-- Page 167 -->


| Symbol | Parameter | Conditions | Min | Typ | Max | Unit | Comments |
| --- | --- | --- | --- | --- | --- | --- | --- |
| I (4) DDA | DAC DC VDDA current consumption in quiescent mode(3) | - | - | 280 | 380 | µA | With no load, middle code (0x800) on the inputs |
|  |  | - | - | 475 | 625 | µA | With no load, worst code (0xF1C) at V = 3.6V in REF+ terms of DC consumption on the inputs |
| DNL(4) | Differential non linearity Difference between two consecutive code- 1LSB) | - | - | - | ±0.5 | LSB | Given for the DAC in 10-bit configuration. |
|  |  | - | - | - | ±2 | LSB | Given for the DAC in 12-bit configuration. |
| INL(4) | Integral non linearity (difference between measured value at Code i and the value at Code i on a line drawn between Code 0 and last Code 1023) | - | - | - | ±1 | LSB | Given for the DAC in 10-bit configuration. |
|  |  | - | - | - | ±4 | LSB | Given for the DAC in 12-bit configuration. |
| Offset(4) | Offset error (difference between measured value at Code (0x800) and the ideal value = V /2) REF+ | - | - | - | ±10 | mV | Given for the DAC in 12-bit configuration |
|  |  | - | - | - | ±3 | LSB | Given for the DAC in 10-bit at V = 3.6V REF+ |
|  |  | - | - | - | ±12 | LSB | Given for the DAC in 12-bit at V = 3.6V REF+ |
| Gain error(4) | Gain error | - | - | - | ±0.5 | % | Given for the DAC in 12-bit configuration |
| t SETTLIN (4) G | Settling time (full scale: for a 10-bit input code transition between the lowest and the highest input codes when DAC_OUT reaches final value ±4LSB | - | - | 3 | 6 | µs | C ≤ 50 pF, LOAD R ≥ 5 kΩ LOAD |
| THD(4) | Total Harmonic Distortion Buffer ON | - | - | - | - | dB | C ≤ 50 pF, LOAD R ≥ 5 kΩ LOAD |
| Update rate(2) | Max frequency for a correct DAC_OUT change when small variation in the input code (from code i to i+1LSB) | - | - | - | 1 | MS/ s | C ≤ 50 pF, LOAD R ≥ 5 kΩ LOAD |

STM32F427xx STM32F429xx Electrical characteristics
Table 86. DAC characteristics (continued)
Symbol Parameter Conditions Min Typ Max Unit Comments
With no load, middle code
- - 280 380 µA
(0x800) on the inputs
DAC DC VDDA
I (4) current consumption in With no load, worst code
DDA
quiescent mode(3) - - 475 625 µA (0xF1C) at V REF+ = 3.6V in
terms of DC consumption on
the inputs
Differential non Given for the DAC in 10-bit
linearity Difference - - - ±0.5 LSB configuration.
DNL(4) between two
consecutive code- Given for the DAC in 12-bit
- - - ±2 LSB
1LSB) configuration.
Integral non linearity Given for the DAC in 10-bit
- - - ±1 LSB
(difference between configuration.
measured value at
INL(4) Code i and the value
at Code i on a line Given for the DAC in 12-bit
- - - ±4 LSB
drawn between Code configuration.
0 and last Code 1023)
Given for the DAC in 12-bit
- - - ±10 mV
Offset error configuration
(difference between
Given for the DAC in 10-bit at
Offset(4) measured value at - - - ±3 LSB
V = 3.6V
Code (0x800) and the REF+
ideal value = V /2) Given for the DAC in 12-bit at
REF+ - - - ±12 LSB
V = 3.6V
REF+
Gain Given for the DAC in 12-bit
Gain error - - - ±0.5 %
error(4) configuration
Settling time (full
scale: for a 10-bit input
code transition
t between the lowest C ≤ 50 pF,
SETTLIN - - 3 6 µs LOAD
(4) and the highest input R ≥ 5 kΩ
G LOAD
codes when
DAC_OUT reaches
final value ±4LSB
Total Harmonic
C ≤ 50 pF,
THD(4) Distortion - - - - dB LOAD
R ≥ 5 kΩ
Buffer ON LOAD
Max frequency for a
correct DAC_OUT
Update change when small MS/ C ≤ 50 pF,
- - - 1 LOAD
rate(2) variation in the input s R ≥ 5 kΩ
LOAD
code (from code i to
i+1LSB)
DS9405 Rev 13 167/240
197

<!-- Page 168 -->


| Symbol | Parameter | Conditions | Min | Typ | Max | Unit | Comments |
| --- | --- | --- | --- | --- | --- | --- | --- |
| t ( WAKEUP 4) | Wakeup time from off state (Setting the ENx bit in the DAC Control register) | - | - | 6.5 | 10 | µs | C ≤ 50 pF, R ≥ 5 kΩ LOAD LOAD input code between lowest and highest possible ones. |
| PSRR+ (2) | Power supply rejection ratio (to V ) (static DDA DC measurement) | - | - | –67 | –40 | dB | No R , C = 50 pF LOAD LOAD |


|  |
| --- |
|  |

Electrical characteristics STM32F427xx STM32F429xx
Table 86. DAC characteristics (continued)
Symbol Parameter Conditions Min Typ Max Unit Comments
Wakeup time from off
C ≤ 50 pF, R ≥ 5 kΩ
t ( state (Setting the ENx LOAD LOAD
WAKEUP - - 6.5 10 µs input code between lowest and
4) bit in the DAC Control
highest possible ones.
register)
Power supply rejection
PSRR+
ratio (to V ) (static - - –67 –40 dB No R , C = 50 pF
(2) DDA LOAD LOAD
DC measurement)
1. V minimum value of 1.7V is obtained with the use of an external power supply supervisor (refer to Section3.17.2:
DDA
Internal reset OFF).
2. Specified by design.
3. The quiescent mode corresponds to a state where the DAC maintains a stable output level to ensure that no dynamic
consumption occurs.
4. Evaluated by characterization - not tested in production.
Figure 54. 12-bit buffered /non-buffered DAC
Buffered/non-buffered DAC
Buffer(1)
RLOAD
12-bit
DACx_OUT
digital to
analog
converter
CLOAD
ai17157a
1. The DAC integrates an output buffer that can be used to reduce the output impedance and to drive external loads directly
without the use of an external operational amplifier. The buffer can be bypassed by configuring the BOFFx bit in the
DAC_CR register.
168/240 DS9405 Rev 13

<!-- Page 169 -->

STM32F427xx STM32F429xx Electrical characteristics
6.3.26 FMC characteristics
Unless otherwise specified, the parameters given in Table87 to Table102 for the FMC
interface are derived from tests performed under the ambient temperature, f frequency,
HCLK
and V supply voltage conditions summarized in Table17, with the following configuration:
DD
• Output speed is set to OSPEEDRy[1:0] = 10 except at V range 1.7 to 2.1 V where
DD
OSPEEDRy[1:0] = 11
• Measurement points are done at CMOS levels: 0.5V
DD
Refer to Section6.3.17: I/O port characteristics for more details on the input/output
characteristics.
Asynchronous waveforms and timings
Figure55 through Figure58 represent asynchronous waveforms and Table87 through
Table94 provide the corresponding timings. The results shown in these tables are obtained
with the following FMC configuration:
• AddressSetupTime = 0x1
• AddressHoldTime = 0x1
• DataSetupTime = 0x1 (except for asynchronous NWAIT mode, DataSetupTime = 0x5)
• BusTurnAroundDuration = 0x0
• For SDRAM memories, V ranges from 2.7 to 3.6V and maximum frequency
DD
FMC_SDCLK = 90MHz
• For Mobile LPSDR SDRAM memories, V ranges from 1.7 to 1.95V and maximum
DD
frequency FMC_SDCLK = 84MHz
DS9405 Rev 13 169/240
197

<!-- Page 170 -->


| tw(NOE) |  |
| --- | --- |
|  |  |
|  |  |
|  |  |
|  |  |
|  |  |


| Symbol | Parameter | Min | Max | Unit |
| --- | --- | --- | --- | --- |
| t w(NE) | FMC_NE low time | 2T − 0.5 HCLK | 2 T +0.5 HCLK | ns |
| t v(NOE_NE) | FMC_NEx low to FMC_NOE low | 0 | 1 | ns |
| t w(NOE) | FMC_NOE low time | 2T HCLK | 2T + 0.5 HCLK | ns |
| t h(NE_NOE) | FMC_NOE high to FMC_NE high hold time | 0 | - | ns |
| t v(A_NE) | FMC_NEx low to FMC_A valid | - | 2 | ns |
| t h(A_NOE) | Address hold time after FMC_NOE high | 0 | - | ns |
| t v(BL_NE) | FMC_NEx low to FMC_BL valid | - | 2 | ns |
| t h(BL_NOE) | FMC_BL hold time after FMC_NOE high | 0 | - | ns |
| t su(Data_NE) | Data to FMC_NEx high setup time | T + 2.5 HCLK | - | ns |
| t su(Data_NOE) | Data to FMC_NOEx high setup time | T +2 HCLK | - | ns |

Electrical characteristics STM32F427xx STM32F429xx
Figure 55. Asynchronous non-multiplexed SRAM/PSRAM/NOR read waveforms
tw(NE)
FMC_NE
tv(NOE_NE) tw(NOE) th(NE_NOE)
FMC_NOE
FMC_NWE
t v(A_NE) th(A_NOE)
FMC_A[25:0]
Address
t
v(BL_NE) th(BL_NOE)
FMC_NBL[1:0]
th(Data_NE)
tsu(Data_NOE) th(Data_NOE)
tsu(Data_NE)
FMC_D[15:0] Data
tv(NADV_NE)
tw(NADV)
FMC_NADV (1)
FMC_NWAIT
th(NE_NWAIT)
tsu(NWAIT_NE)
MS32753V1
1. Mode 2/B, C, and D only. In Mode 1, FMC_NADV is not used.
Table 87. Asynchronous non-multiplexed SRAM/PSRAM/NOR -
read timings(1)(2)
Symbol Parameter Min Max Unit
t FMC_NE low time 2T − 0.5 2 T +0.5 ns
w(NE) HCLK HCLK
t FMC_NEx low to FMC_NOE low 0 1 ns
v(NOE_NE)
t FMC_NOE low time 2T 2T + 0.5 ns
w(NOE) HCLK HCLK
t FMC_NOE high to FMC_NE high hold time 0 - ns
h(NE_NOE)
t FMC_NEx low to FMC_A valid - 2 ns
v(A_NE)
t Address hold time after FMC_NOE high 0 - ns
h(A_NOE)
t FMC_NEx low to FMC_BL valid - 2 ns
v(BL_NE)
t FMC_BL hold time after FMC_NOE high 0 - ns
h(BL_NOE)
t Data to FMC_NEx high setup time T + 2.5 - ns
su(Data_NE) HCLK
t Data to FMC_NOEx high setup time T +2 - ns
su(Data_NOE) HCLK
170/240 DS9405 Rev 13

<!-- Page 171 -->


| Symbol | Parameter | Min | Max | Unit |
| --- | --- | --- | --- | --- |
| t h(Data_NOE) | Data hold time after FMC_NOE high | 0 | - | ns |
| t h(Data_NE) | Data hold time after FMC_NEx high | 0 | - | ns |
| t v(NADV_NE) | FMC_NEx low to FMC_NADV low | - | 0 | ns |
| t w(NADV) | FMC_NADV low time | - | T +1 HCLK | ns |


| Symbol | Parameter | Min | Max | Unit |
| --- | --- | --- | --- | --- |
| t w(NE) | FMC_NE low time | 7T +0.5 HCLK | 7T +1 HCLK | ns |
| t w(NOE) | FMC_NWE low time | 5T −1.5 HCLK | 5T +2 HCLK |  |
| t su(NWAIT_NE) | FMC_NWAIT valid before FMC_NEx high | 5T +1.5 HCLK | - |  |
| t h(NE_NWAIT) | FMC_NEx hold time after FMC_NWAIT invalid | 4T +1 HCLK | - |  |

STM32F427xx STM32F429xx Electrical characteristics
Table 87. Asynchronous non-multiplexed SRAM/PSRAM/NOR -
read timings(1)(2) (continued)
Symbol Parameter Min Max Unit
t Data hold time after FMC_NOE high 0 - ns
h(Data_NOE)
t Data hold time after FMC_NEx high 0 - ns
h(Data_NE)
t FMC_NEx low to FMC_NADV low - 0 ns
v(NADV_NE)
t FMC_NADV low time - T +1 ns
w(NADV) HCLK
1. C = 30pF.
L
2. Evaluated by characterization.
Table 88. Asynchronous non-multiplexed SRAM/PSRAM/NOR read -
NWAIT timings(1)(2)
Symbol Parameter Min Max Unit
t FMC_NE low time 7T +0.5 7T +1
w(NE) HCLK HCLK
t FMC_NWE low time 5T −1.5 5T +2
w(NOE) HCLK HCLK ns
t FMC_NWAIT valid before FMC_NEx high 5T +1.5 -
su(NWAIT_NE) HCLK
FMC_NEx hold time after FMC_NWAIT
t 4T +1 -
h(NE_NWAIT) invalid HCLK
1. C = 30pF.
L
2. Evaluated by characterization.
DS9405 Rev 13 171/240
197

<!-- Page 172 -->


|  |
| --- |
|  |
|  |
|  |


| Symbol | Parameter | Min | Max | Unit |
| --- | --- | --- | --- | --- |
| t w(NE) | FMC_NE low time | 3T HCLK | 3T +1 HCLK | ns |
| t v(NWE_NE) | FMC_NEx low to FMC_NWE low | T −0.5 HCLK | T + 0.5 HCLK | ns |
| t w(NWE) | FMC_NWE low time | T HCLK | T + 0.5 HCLK | ns |
| t h(NE_NWE) | FMC_NWE high to FMC_NE high hold time | T +1.5 HCLK | - | ns |
| t v(A_NE) | FMC_NEx low to FMC_A valid | - | 0 | ns |
| t h(A_NWE) | Address hold time after FMC_NWE high | T +0.5 HCLK | - | ns |
| t v(BL_NE) | FMC_NEx low to FMC_BL valid | - | 1.5 | ns |
| t h(BL_NWE) | FMC_BL hold time after FMC_NWE high | T +0.5 HCLK | - | ns |
| t v(Data_NE) | Data to FMC_NEx low to Data valid | - | T + 2 HCLK | ns |
| t h(Data_NWE) | Data hold time after FMC_NWE high | T +0.5 HCLK | - | ns |
| t v(NADV_NE) | FMC_NEx low to FMC_NADV low | - | 0.5 | ns |
| t w(NADV) | FMC_NADV low time | - | T + 0.5 HCLK | ns |

Electrical characteristics STM32F427xx STM32F429xx
Figure 56. Asynchronous non-multiplexed SRAM/PSRAM/NOR write waveforms
tw(NE)
FMC_NEx
FMC_NOE
tv(NWE_NE) tw(NWE) th(NE_NWE)
FMC_NWE
tv(A_NE) th(A_NWE)
FMC_A[25:0] Address
tv(BL_NE) th(BL_NWE)
FMC_NBL[1:0] NBL
tv(Data_NE) th(Data_NWE)
FMC_D[15:0] Data
tv(NADV_NE)
tw(NADV)
FMC_NADV (1)
FMC_NWAIT
th(NE_NWAIT)
tsu(NWAIT_NE)
MS32754V1
1. Mode 2/B, C, and D only. In Mode 1, FMC_NADV is not used.
Table 89. Asynchronous non-multiplexed SRAM/PSRAM/NOR write timings(1)(2)
Symbol Parameter Min Max Unit
t FMC_NE low time 3T 3T +1 ns
w(NE) HCLK HCLK
t FMC_NEx low to FMC_NWE low T −0.5 T + 0.5 ns
v(NWE_NE) HCLK HCLK
t FMC_NWE low time T T + 0.5 ns
w(NWE) HCLK HCLK
t FMC_NWE high to FMC_NE high hold time T +1.5 - ns
h(NE_NWE) HCLK
t FMC_NEx low to FMC_A valid - 0 ns
v(A_NE)
t Address hold time after FMC_NWE high T +0.5 - ns
h(A_NWE) HCLK
t FMC_NEx low to FMC_BL valid - 1.5 ns
v(BL_NE)
t FMC_BL hold time after FMC_NWE high T +0.5 - ns
h(BL_NWE) HCLK
t Data to FMC_NEx low to Data valid - T + 2 ns
v(Data_NE) HCLK
t Data hold time after FMC_NWE high T +0.5 - ns
h(Data_NWE) HCLK
t FMC_NEx low to FMC_NADV low - 0.5 ns
v(NADV_NE)
t FMC_NADV low time - T + 0.5 ns
w(NADV) HCLK
1. C = 30pF.
L
2. Evaluated by characterization.
172/240 DS9405 Rev 13

<!-- Page 173 -->


| Symbol | Parameter | Min | Max | Unit |
| --- | --- | --- | --- | --- |
| t w(NE) | FMC_NE low time | 8T +1 HCLK | 8T +2 HCLK | ns |
| t w(NWE) | FMC_NWE low time | 6T −1 HCLK | 6T +2 HCLK | ns |
| t su(NWAIT_NE) | FMC_NWAIT valid before FMC_NEx high | 6T +1.5 HCLK | - | ns |
| t h(NE_NWAIT) | FMC_NEx hold time after FMC_NWAIT invalid | 4T +1 HCLK | - | ns |


|  |
| --- |
|  |
|  |
|  |
|  |


|  |
| --- |
|  |

STM32F427xx STM32F429xx Electrical characteristics
Table 90. Asynchronous non-multiplexed SRAM/PSRAM/NOR write -
NWAIT timings(1)(2)
Symbol Parameter Min Max Unit
t FMC_NE low time 8T +1 8T +2 ns
w(NE) HCLK HCLK
t FMC_NWE low time 6T −1 6T +2 ns
w(NWE) HCLK HCLK
t FMC_NWAIT valid before FMC_NEx high 6T +1.5 - ns
su(NWAIT_NE) HCLK
FMC_NEx hold time after FMC_NWAIT
t 4T +1 - ns
h(NE_NWAIT) invalid HCLK
1. C = 30pF.
L
2. Evaluated by characterization.
Figure 57. Asynchronous multiplexed PSRAM/NOR read waveforms
tw(NE)
FMC_ NE
tv(NOE_NE) th(NE_NOE)
FMC_NOE
tw(NOE)
FMC_NWE
tv(A_NE) th(A_NOE)
FMC_ A[25:16]
Address
tv(BL_NE) th(BL_NOE)
FMC_ NBL[1:0] NBL
th(Data_NE)
tsu(Data_NE)
tv(A_NE) tsu(Data_NOE) th(Data_NOE)
FMC_ AD[15:0] Address Data
tv(NADV_NE) th(AD_NADV)
tw(NADV)
FMC_NADV
FMC_NWAIT
th(NE_NWAIT)
tsu(NWAIT_NE)
MS32755V1
DS9405 Rev 13 173/240
197

<!-- Page 174 -->


| Symbol | Parameter | Min | Max | Unit |
| --- | --- | --- | --- | --- |
| t w(NE) | FMC_NE low time | 3T −1 HCLK | 3T +0.5 HCLK | ns |
| t v(NOE_NE) | FMC_NEx low to FMC_NOE low | 2T −0.5 HCLK | 2T HCLK | ns |
| t tw(NOE) | FMC_NOE low time | T −1 HCLK | T +1 HCLK | ns |
| t h(NE_NOE) | FMC_NOE high to FMC_NE high hold time | 1 | - | ns |
| t v(A_NE) | FMC_NEx low to FMC_A valid | - | 2 | ns |
| t v(NADV_NE) | FMC_NEx low to FMC_NADV low | 0 | 2 | ns |
| t w(NADV) | FMC_NADV low time | T −0.5 HCLK | T +0.5 HCLK | ns |
| t h(AD_NADV) | FMC_AD(address) valid hold time after FMC_NADV high) | 0 | - | ns |
| t h(A_NOE) | Address hold time after FMC_NOE high | T −0.5 HCLK | - | ns |
| t h(BL_NOE) | FMC_BL time after FMC_NOE high | 0 | - | ns |
| t v(BL_NE) | FMC_NEx low to FMC_BL valid | - | 2 | ns |
| t su(Data_NE) | Data to FMC_NEx high setup time | T +1.5 HCLK | - | ns |
| t su(Data_NOE) | Data to FMC_NOE high setup time | T +1 HCLK | - | ns |
| t h(Data_NE) | Data hold time after FMC_NEx high | 0 | - | ns |
| t h(Data_NOE) | Data hold time after FMC_NOE high | 0 | - | ns |


| Symbol | Parameter | Min | Max | Unit |
| --- | --- | --- | --- | --- |
| t w(NE) | FMC_NE low time | 8T +0.5 HCLK | 8T +2 HCLK | ns |
| t w(NOE) | FMC_NWE low time | 5T −1 HCLK | 5T +1.5 HCLK | ns |
| t su(NWAIT_NE) | FMC_NWAIT valid before FMC_NEx high | 5T +1.5 HCLK | - | ns |
| t h(NE_NWAIT) | FMC_NEx hold time after FMC_NWAIT invalid | 4T +1 HCLK | - | ns |

Electrical characteristics STM32F427xx STM32F429xx
Table 91. Asynchronous multiplexed PSRAM/NOR read timings(1)(2)
Symbol Parameter Min Max Unit
t FMC_NE low time 3T −1 3T +0.5 ns
w(NE) HCLK HCLK
t FMC_NEx low to FMC_NOE low 2T −0.5 2T ns
v(NOE_NE) HCLK HCLK
t FMC_NOE low time T −1 T +1 ns
tw(NOE) HCLK HCLK
t FMC_NOE high to FMC_NE high hold time 1 - ns
h(NE_NOE)
t FMC_NEx low to FMC_A valid - 2 ns
v(A_NE)
t FMC_NEx low to FMC_NADV low 0 2 ns
v(NADV_NE)
t FMC_NADV low time T −0.5 T +0.5 ns
w(NADV) HCLK HCLK
FMC_AD(address) valid hold time after
t 0 - ns
h(AD_NADV) FMC_NADV high)
t Address hold time after FMC_NOE high T −0.5 - ns
h(A_NOE) HCLK
t FMC_BL time after FMC_NOE high 0 - ns
h(BL_NOE)
t FMC_NEx low to FMC_BL valid - 2 ns
v(BL_NE)
t Data to FMC_NEx high setup time T +1.5 - ns
su(Data_NE) HCLK
t Data to FMC_NOE high setup time T +1 - ns
su(Data_NOE) HCLK
t Data hold time after FMC_NEx high 0 - ns
h(Data_NE)
t Data hold time after FMC_NOE high 0 - ns
h(Data_NOE)
1. C = 30pF.
L
2. Evaluated by characterization.
Table 92. Asynchronous multiplexed PSRAM/NOR read-NWAIT timings(1)(2)
Symbol Parameter Min Max Unit
t FMC_NE low time 8T +0.5 8T +2 ns
w(NE) HCLK HCLK
t FMC_NWE low time 5T −1 5T +1.5 ns
w(NOE) HCLK HCLK
t FMC_NWAIT valid before FMC_NEx high 5T +1.5 - ns
su(NWAIT_NE) HCLK
FMC_NEx hold time after FMC_NWAIT
t 4T +1 - ns
h(NE_NWAIT) invalid HCLK
1. C = 30pF.
L
2. Evaluated by characterization.
174/240 DS9405 Rev 13

<!-- Page 175 -->


|  |
| --- |
|  |
|  |
|  |
|  |


|  |
| --- |
|  |
|  |


|  |
| --- |
|  |
|  |


| Symbol | Parameter | Min | Max | Unit |
| --- | --- | --- | --- | --- |
| t w(NE) | FMC_NE low time | 4T HCLK | 4T +0.5 HCLK | ns |
| t v(NWE_NE) | FMC_NEx low to FMC_NWE low | T −1 HCLK | T +0.5 HCLK | ns |
| t w(NWE) | FMC_NWE low time | 2T HCLK | 2T +0.5 HCLK | ns |
| t h(NE_NWE) | FMC_NWE high to FMC_NE high hold time | T HCLK | - | ns |
| t v(A_NE) | FMC_NEx low to FMC_A valid | - | 0 | ns |
| t v(NADV_NE) | FMC_NEx low to FMC_NADV low | 0.5 | 1 | ns |
| t w(NADV) | FMC_NADV low time | T −0.5 HCLK | T + 0.5 HCLK | ns |
| t h(AD_NADV) | FMC_AD(adress) valid hold time after FMC_NADV high) | T −2 HCLK | - | ns |
| t h(A_NWE) | Address hold time after FMC_NWE high | T HCLK | - | ns |
| t h(BL_NWE) | FMC_BL hold time after FMC_NWE high | T −2 HCLK | - | ns |
| t v(BL_NE) | FMC_NEx low to FMC_BL valid | - | 2 | ns |
| t v(Data_NADV) | FMC_NADV high to Data valid | - | T +1.5 HCLK | ns |
| t h(Data_NWE) | Data hold time after FMC_NWE high | T +0.5 HCLK | - | ns |

STM32F427xx STM32F429xx Electrical characteristics
Figure 58. Asynchronous multiplexed PSRAM/NOR write waveforms
tw(NE)
FMC_ NEx
FMC_NOE
tv(NWE_NE) tw(NWE) th(NE_NWE)
FMC_NWE
tv(A_NE) th(A_NWE)
FMC_ A[25:16]
Address
tv(BL_NE) th(BL_NWE)
FMC_ NBL[1:0] NBL
tv(A_NE) tv(Data_NADV) th(Data_NWE)
FMC_ AD[15:0] Address Data
tv(NADV_NE) th(AD_NADV)
tw(NADV)
FMC_NADV
FMC_NWAIT
th(NE_NWAIT)
tsu(NWAIT_NE)
MS32756V1
Table 93. Asynchronous multiplexed PSRAM/NOR write timings(1)(2)
Symbol Parameter Min Max Unit
t FMC_NE low time 4T 4T +0.5 ns
w(NE) HCLK HCLK
t FMC_NEx low to FMC_NWE low T −1 T +0.5 ns
v(NWE_NE) HCLK HCLK
t FMC_NWE low time 2T 2T +0.5 ns
w(NWE) HCLK HCLK
t FMC_NWE high to FMC_NE high hold time T - ns
h(NE_NWE) HCLK
t FMC_NEx low to FMC_A valid - 0 ns
v(A_NE)
t FMC_NEx low to FMC_NADV low 0.5 1 ns
v(NADV_NE)
t FMC_NADV low time T −0.5 T + 0.5 ns
w(NADV) HCLK HCLK
t FMC_AD(adress) valid hold time after FMC_NADV high) T −2 - ns
h(AD_NADV) HCLK
t Address hold time after FMC_NWE high T - ns
h(A_NWE) HCLK
t FMC_BL hold time after FMC_NWE high T −2 - ns
h(BL_NWE) HCLK
t FMC_NEx low to FMC_BL valid - 2 ns
v(BL_NE)
t FMC_NADV high to Data valid - T +1.5 ns
v(Data_NADV) HCLK
t Data hold time after FMC_NWE high T +0.5 - ns
h(Data_NWE) HCLK
1. C = 30pF.
L
2. Evaluated by characterization.
DS9405 Rev 13 175/240
197

<!-- Page 176 -->


| Symbol | Parameter | Min | Max | Unit |
| --- | --- | --- | --- | --- |
| t w(NE) | FMC_NE low time | 9T HCLK | 9T +0.5 HCLK | ns |
| t w(NWE) | FMC_NWE low time | 7T HCLK | 7T +2 HCLK | ns |
| t su(NWAIT_NE) | FMC_NWAIT valid before FMC_NEx high | 6T +1.5 HCLK | - | ns |
| t h(NE_NWAIT) | FMC_NEx hold time after FMC_NWAIT invalid | 4T –1 HCLK | - | ns |

Electrical characteristics STM32F427xx STM32F429xx
Table 94. Asynchronous multiplexed PSRAM/NOR write-NWAIT timings(1)(2)
Symbol Parameter Min Max Unit
t FMC_NE low time 9T 9T +0.5 ns
w(NE) HCLK HCLK
t FMC_NWE low time 7T 7T +2 ns
w(NWE) HCLK HCLK
t FMC_NWAIT valid before FMC_NEx high 6T +1.5 - ns
su(NWAIT_NE) HCLK
FMC_NEx hold time after FMC_NWAIT
t 4T –1 - ns
h(NE_NWAIT) invalid HCLK
1. C = 30pF.
L
2. Evaluated by characterization.
Synchronous waveforms and timings
Figure59 through Figure62 represent synchronous waveforms and Table95 through
Table98 provide the corresponding timings. The results shown in these tables are obtained
with the following FMC configuration:
• BurstAccessMode = FMC_BurstAccessMode_Enable;
• MemoryType = FMC_MemoryType_CRAM;
• WriteBurst = FMC_WriteBurst_Enable;
• CLKDivision = 1; (0 is not supported. See the STM32F4xx reference manual: RM0090)
• DataLatency = 1 for NOR flash; DataLatency = 0 for PSRAM
In all timing tables, the T is the HCLK clock period (with maximum
HCLK
FMC_CLK=90MHz).
176/240 DS9405 Rev 13

<!-- Page 177 -->


| td(CL td(CL | Da KL-NExL) |  |  |  |  |  |
| --- | --- | --- | --- | --- | --- | --- |
|  |  | ta latency = 0 |  |  |  |  |
|  | td(CL KL-AV) | KL-NADVH) |  |  |  |  |
|  |  |  |  | td(CL | KH-AIV) |  |
|  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |
|  |  |  | OEL) | td(CLKH | -NOEH) |  |
| td(CLK AD | L-ADIV) tsu(A |  |  |  |  |  |
|  |  |  |  |  | th(CL 2 ITV) |  |
|  |  |  |  |  |  |  |
|  | [15:0] |  |  |  |  |  |
|  | tsu(NWA |  |  |  |  |  |
|  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |


|  |
| --- |
| D |


|  |
| --- |
| 2 |


|  |  |
| --- | --- |


|  | LKH) |
| --- | --- |
| TV-C |  |
|  |  |


| Symbol | Parameter | Min | Max | Unit |
| --- | --- | --- | --- | --- |
| t w(CLK) | FMC_CLK period | 2T −1 HCLK | - | ns |
| t d(CLKL-NExL) | FMC_CLK low to FMC_NEx low (x=0..2) | - | 0 | ns |
| t d(CLKH_NExH) | FMC_CLK high to FMC_NEx high (x= 0…2) | T HCLK | - | ns |
| t d(CLKL-NADVL) | FMC_CLK low to FMC_NADV low | - | 0 | ns |
| t d(CLKL-NADVH) | FMC_CLK low to FMC_NADV high | 0 | - | ns |
| t d(CLKL-AV) | FMC_CLK low to FMC_Ax valid (x=16…25) | - | 0 | ns |
| t d(CLKH-AIV) | FMC_CLK high to FMC_Ax invalid (x=16…25) | 0 | - | ns |
| t d(CLKL-NOEL) | FMC_CLK low to FMC_NOE low | - | T +0.5 HCLK | ns |
| t d(CLKH-NOEH) | FMC_CLK high to FMC_NOE high | T −0.5 HCLK | - | ns |
| t d(CLKL-ADV) | FMC_CLK low to FMC_AD[15:0] valid | - | 0.5 | ns |
| t d(CLKL-ADIV) | FMC_CLK low to FMC_AD[15:0] invalid | 0 | - | ns |

STM32F427xx STM32F429xx Electrical characteristics
Figure 59. Synchronous multiplexed NOR/PSRAM read timings
tw(CLK) tw(CLK) BUSTURN = 0
FMC_CLK
Data latency = 0
td(CLKL-NExL) td(CLKH-NExH)
FMC_NEx
td(CLKL-NADVL) td(CLKL-NADVH)
FMC_NADV
td(CLKL-AV) td(CLKH-AIV)
FMC_A[25:16]
td(CLKL-NOEL) td(CLKH-NOEH)
FMC_NOE
td(CLKL-ADIV) th(CLKH-ADV)
td(CLKL-ADV) tsu(ADV-CLKH) tsu(ADV-CLKH) th(CLKH-ADV)
FMC_AD[15:0] AD[15:0] D1 D2
tsu(NWAITV-CLKH) th(CLKH-NWAITV)
FMC_NWAIT
(WAITCFG = 1b,
WAITPOL + 0b)
tsu(NWAITV-CLKH) th(CLKH-NWAITV)
FMC_NWAIT
(WAITCFG = 0b,
WAITPOL + 0b)
tsu(NWAITV-CLKH) th(CLKH-NWAITV)
MS32757V1
Table 95. Synchronous multiplexed NOR/PSRAM read timings(1)(2)
Symbol Parameter Min Max Unit
t FMC_CLK period 2T −1 - ns
w(CLK) HCLK
t FMC_CLK low to FMC_NEx low (x=0..2) - 0 ns
d(CLKL-NExL)
t FMC_CLK high to FMC_NEx high (x= 0…2) T - ns
d(CLKH_NExH) HCLK
t FMC_CLK low to FMC_NADV low - 0 ns
d(CLKL-NADVL)
t FMC_CLK low to FMC_NADV high 0 - ns
d(CLKL-NADVH)
t FMC_CLK low to FMC_Ax valid (x=16…25) - 0 ns
d(CLKL-AV)
t FMC_CLK high to FMC_Ax invalid (x=16…25) 0 - ns
d(CLKH-AIV)
t FMC_CLK low to FMC_NOE low - T +0.5 ns
d(CLKL-NOEL) HCLK
t FMC_CLK high to FMC_NOE high T −0.5 - ns
d(CLKH-NOEH) HCLK
t FMC_CLK low to FMC_AD[15:0] valid - 0.5 ns
d(CLKL-ADV)
t FMC_CLK low to FMC_AD[15:0] invalid 0 - ns
d(CLKL-ADIV)
DS9405 Rev 13 177/240
197

<!-- Page 178 -->


| Symbol | Parameter | Min | Max | Unit |
| --- | --- | --- | --- | --- |
| t su(ADV-CLKH) | FMC_A/D[15:0] valid data before FMC_CLK high | 5 | - | ns |
| t h(CLKH-ADV) | FMC_A/D[15:0] valid data after FMC_CLK high | 0 | - | ns |
| t su(NWAIT-CLKH) | FMC_NWAIT valid before FMC_CLK high | 4 | - | ns |
| t h(CLKH-NWAIT) | FMC_NWAIT valid after FMC_CLK high | 0 | - | ns |


| td(CL td(CL td(CL td(CLK AD | Da KL-NExL) |  |  |  |  |  |  |  |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
|  |  |  |  |  |  | CLK |  |  |
|  |  | ta latency | = 0 |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |
|  | td(CL KL-AV) | KL-NADVH | ) |  |  |  |  |  |
|  |  |  |  |  | t | d(CL | KH-AIV) |  |
|  |  |  |  |  |  |  |  |  |
|  | KL-NWEL) |  |  |  | td(C | LKH | -NWEH) |  |
|  |  |  |  |  |  |  |  |  |
|  |  |  |  |  | d(CLKL-Data) | -NW LKH | D2 AITV) -NBLH) |  |
|  |  |  |  |  | D1 |  |  |  |
|  |  |  |  |  | th(CLKH t d(C |  |  |  |
|  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |

Electrical characteristics STM32F427xx STM32F429xx
Table 95. Synchronous multiplexed NOR/PSRAM read timings(1)(2) (continued)
Symbol Parameter Min Max Unit
FMC_A/D[15:0] valid data before FMC_CLK
t 5 - ns
su(ADV-CLKH) high
t FMC_A/D[15:0] valid data after FMC_CLK high 0 - ns
h(CLKH-ADV)
t FMC_NWAIT valid before FMC_CLK high 4 - ns
su(NWAIT-CLKH)
t FMC_NWAIT valid after FMC_CLK high 0 - ns
h(CLKH-NWAIT)
1. C = 30pF.
L
2. Evaluated by characterization.
Figure 60. Synchronous multiplexed PSRAM write timings
tw(CLK) tw(CLK) BUSTURN = 0
FMC_CLK
Data latency = 0
td(CLKL-NExL) td(CLKH-NExH)
FMC_NEx
td(CLKL-NADVL) td(CLKL-NADVH)
FMC_NADV
td(CLKL-AV) td(CLKH-AIV)
FMC_A[25:16]
td(CLKL-NWEL) td(CLKH-NWEH)
FMC_NWE
td(CLKL-ADIV) td(CLKL-Data)
td(CLKL-ADV) td(CLKL-Data)
FMC_AD[15:0] AD[15:0] D1 D2
FMC_NWAIT
(WAITCFG = 0b,
WAITPOL + 0b) tsu(NWAITV-CLKH) th(CLKH-NWAITV)
t
d(CLKH-NBLH)
FMC_NBL
MS32758V1
178/240 DS9405 Rev 13

<!-- Page 179 -->


| Symbol | Parameter | Min | Max | Unit |
| --- | --- | --- | --- | --- |
| t w(CLK) | FMC_CLK period, VDD range= 2.7 to 3.6 V | 2T −1 HCLK | - | ns |
| t d(CLKL-NExL) | FMC_CLK low to FMC_NEx low (x=0..2) | - | 1.5 | ns |
| t d(CLKH-NExH) | FMC_CLK high to FMC_NEx high (x= 0…2) | T HCLK | - | ns |
| t d(CLKL-NADVL) | FMC_CLK low to FMC_NADV low | - | 0 | ns |
| t d(CLKL-NADVH) | FMC_CLK low to FMC_NADV high | 0 | - | ns |
| t d(CLKL-AV) | FMC_CLK low to FMC_Ax valid (x=16…25) | - | 0 | ns |
| t d(CLKH-AIV) | FMC_CLK high to FMC_Ax invalid (x=16…25) | T HCLK | - | ns |
| t d(CLKL-NWEL) | FMC_CLK low to FMC_NWE low | - | 0 | ns |
| t (CLKH-NWEH) | FMC_CLK high to FMC_NWE high | T −0.5 HCLK | - | ns |
| t d(CLKL-ADV) | FMC_CLK low to FMC_AD[15:0] valid | - | 3 | ns |
| t d(CLKL-ADIV) | FMC_CLK low to FMC_AD[15:0] invalid | 0 | - | ns |
| t d(CLKL-DATA) | FMC_A/D[15:0] valid data after FMC_CLK low | - | 3 | ns |
| t d(CLKL-NBLL) | FMC_CLK low to FMC_NBL low | 0 | - | ns |
| t d(CLKH-NBLH) | FMC_CLK high to FMC_NBL high | T −0.5 HCLK | - | ns |
| t su(NWAIT-CLKH) | FMC_NWAIT valid before FMC_CLK high | 4 | - | ns |
| t h(CLKH-NWAIT) | FMC_NWAIT valid after FMC_CLK high | 0 | - | ns |

STM32F427xx STM32F429xx Electrical characteristics
Table 96. Synchronous multiplexed PSRAM write timings(1)(2)
Symbol Parameter Min Max Unit
t FMC_CLK period, VDD range= 2.7 to 3.6 V 2T −1 - ns
w(CLK) HCLK
t FMC_CLK low to FMC_NEx low (x=0..2) - 1.5 ns
d(CLKL-NExL)
t FMC_CLK high to FMC_NEx high (x= 0…2) T - ns
d(CLKH-NExH) HCLK
t FMC_CLK low to FMC_NADV low - 0 ns
d(CLKL-NADVL)
t FMC_CLK low to FMC_NADV high 0 - ns
d(CLKL-NADVH)
t FMC_CLK low to FMC_Ax valid (x=16…25) - 0 ns
d(CLKL-AV)
t FMC_CLK high to FMC_Ax invalid (x=16…25) T - ns
d(CLKH-AIV) HCLK
t FMC_CLK low to FMC_NWE low - 0 ns
d(CLKL-NWEL)
t FMC_CLK high to FMC_NWE high T −0.5 - ns
(CLKH-NWEH) HCLK
t FMC_CLK low to FMC_AD[15:0] valid - 3 ns
d(CLKL-ADV)
t FMC_CLK low to FMC_AD[15:0] invalid 0 - ns
d(CLKL-ADIV)
t FMC_A/D[15:0] valid data after FMC_CLK low - 3 ns
d(CLKL-DATA)
t FMC_CLK low to FMC_NBL low 0 - ns
d(CLKL-NBLL)
t FMC_CLK high to FMC_NBL high T −0.5 - ns
d(CLKH-NBLH) HCLK
t FMC_NWAIT valid before FMC_CLK high 4 - ns
su(NWAIT-CLKH)
t FMC_NWAIT valid after FMC_CLK high 0 - ns
h(CLKH-NWAIT)
1. C = 30pF.
L
2. Evaluated by characterization.
DS9405 Rev 13 179/240
197

<!-- Page 180 -->


| td(CL | Da |  |  |  |  |
| --- | --- | --- | --- | --- | --- |
|  |  | ta latency = 0 |  |  |  |
|  | td(CL KL-AV) | KL-NADVH) |  |  |  |
|  |  |  |  | td(CL | KH-AIV) |
|  |  |  |  |  |  |
|  |  |  | OEL) | td(CLK | H-NOEH) |
|  | tsu( |  |  |  |  |
|  |  |  |  |  | th(CL 2 ITV) |
|  | tsu(NW |  |  |  |  |
|  |  |  |  |  |  |
|  |  |  |  |  |  |
|  |  |  |  |  |  |


|  |
| --- |
|  |


|  |
| --- |
| D |


|  |
| --- |
| 2 |


|  |  |
| --- | --- |


|  | CLKH) |
| --- | --- |
| ITV- |  |
|  |  |


| Symbol | Parameter | Min | Max | Unit |
| --- | --- | --- | --- | --- |
| t w(CLK) | FMC_CLK period | 2T −1 HCLK | - | ns |
| t (CLKL-NExL) | FMC_CLK low to FMC_NEx low (x=0..2) | - | 0.5 | ns |
| t d(CLKH- NExH) | FMC_CLK high to FMC_NEx high (x= 0…2) | T HCLK | - | ns |
| t d(CLKL- NADVL) | FMC_CLK low to FMC_NADV low | - | 0 | ns |
| t d(CLKL- NADVH) | FMC_CLK low to FMC_NADV high | 0 | - | ns |
| t d(CLKL-AV) | FMC_CLK low to FMC_Ax valid (x=16…25) | - | 0 | ns |
| t d(CLKH-AIV) | FMC_CLK high to FMC_Ax invalid (x=16…25) | T −0.5 HCLK | - | ns |
| t d(CLKL-NOEL) | FMC_CLK low to FMC_NOE low | - | T +2 HCLK | ns |
| t d(CLKH- NOEH) | FMC_CLK high to FMC_NOE high | T −0.5 HCLK | - | ns |
| t su(DV-CLKH) | FMC_D[15:0] valid data before FMC_CLK high | 5 | - | ns |

Electrical characteristics STM32F427xx STM32F429xx
Figure 61. Synchronous non-multiplexed NOR/PSRAM read timings
tw(CLK) tw(CLK)
FMC_CLK
td(CLKL-NExL) td(CLKH-NExH)
Data latency = 0
FMC_NEx
td(CLKL-NADVL) td(CLKL-NADVH)
FMC_NADV
td(CLKL-AV) td(CLKH-AIV)
FMC_A[25:0]
td(CLKL-NOEL) td(CLKH-NOEH)
FMC_NOE
tsu(DV-CLKH) th(CLKH-DV)
tsu(DV-CLKH) th(CLKH-DV)
FMC_D[15:0] D1 D2
tsu(NWAITV-CLKH) th(CLKH-NWAITV)
FMC_NWAIT
(WAITCFG = 1b,
WAITPOL + 0b)
tsu(NWAITV-CLKH) th(CLKH-NWAITV)
FMC_NWAIT
(WAITCFG = 0b,
WAITPOL + 0b)
tsu(NWAITV-CLKH) th(CLKH-NWAITV)
MS32759V1
Table 97. Synchronous non-multiplexed NOR/PSRAM read timings(1)(2)
Symbol Parameter Min Max Unit
t FMC_CLK period 2T −1 - ns
w(CLK) HCLK
t FMC_CLK low to FMC_NEx low (x=0..2) - 0.5 ns
(CLKL-NExL)
t
d(CLKH- FMC_CLK high to FMC_NEx high (x= 0…2) T - ns
HCLK
NExH)
t
d(CLKL- FMC_CLK low to FMC_NADV low - 0 ns
NADVL)
t
d(CLKL- FMC_CLK low to FMC_NADV high 0 - ns
NADVH)
t FMC_CLK low to FMC_Ax valid (x=16…25) - 0 ns
d(CLKL-AV)
t FMC_CLK high to FMC_Ax invalid (x=16…25) T −0.5 - ns
d(CLKH-AIV) HCLK
t FMC_CLK low to FMC_NOE low - T +2 ns
d(CLKL-NOEL) HCLK
t
d(CLKH- FMC_CLK high to FMC_NOE high T −0.5 - ns
HCLK
NOEH)
t FMC_D[15:0] valid data before FMC_CLK high 5 - ns
su(DV-CLKH)
180/240 DS9405 Rev 13

<!-- Page 181 -->


| Symbol | Parameter | Min | Max | Unit |
| --- | --- | --- | --- | --- |
| t h(CLKH-DV) | FMC_D[15:0] valid data after FMC_CLK high | 0 | - | ns |
| t (NWAIT-CLKH) | FMC_NWAIT valid before FMC_CLK high | 4 | - | - |
| t h(CLKH- NWAIT) | FMC_NWAIT valid after FMC_CLK high | 0 | - | - |


| td(CL td(CL | Da |  |  |  |  |  |  |
| --- | --- | --- | --- | --- | --- | --- | --- |
|  |  | ta latency = | 0 |  |  |  |  |
|  | td(CL KL-AV) | KL-NADVH | ) |  |  |  |  |
|  |  |  |  | t | d(CLKH-A | IV) |  |
|  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |
|  | KL-NWEL) |  |  | td(C | LKH-NWE | H) |  |
|  |  |  |  |  |  |  |  |
|  | td(CL | KL-Data) |  |  | td(CL D2 CLKH-NB -NWAITV) | KL-Da LH) |  |
|  |  |  |  | D1 |  |  |  |
|  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |
|  | tsu(NWA |  |  |  |  |  |  |


| Symbol | Parameter | Min | Max | Unit |
| --- | --- | --- | --- | --- |
| t (CLK) | FMC_CLK period | 2T −1 HCLK | - | ns |
| t d(CLKL-NExL) | FMC_CLK low to FMC_NEx low (x=0..2) | - | 0.5 | ns |
| t (CLKH-NExH) | FMC_CLK high to FMC_NEx high (x= 0…2) | T HCLK | - | ns |
| t d(CLKL-NADVL) | FMC_CLK low to FMC_NADV low | - | 0 | ns |
| t d(CLKL-NADVH) | FMC_CLK low to FMC_NADV high | 0 | - | ns |
| t d(CLKL-AV) | FMC_CLK low to FMC_Ax valid (x=16…25) | - | 0 | ns |

STM32F427xx STM32F429xx Electrical characteristics
Table 97. Synchronous non-multiplexed NOR/PSRAM read timings(1)(2) (continued)
Symbol Parameter Min Max Unit
t FMC_D[15:0] valid data after FMC_CLK high 0 - ns
h(CLKH-DV)
t FMC_NWAIT valid before FMC_CLK high 4 - -
(NWAIT-CLKH)
t
h(CLKH- FMC_NWAIT valid after FMC_CLK high 0 - -
NWAIT)
1. C = 30 pF.
L
2. Guaranteed by characterization results.
Figure 62. Synchronous non-multiplexed PSRAM write timings
tw(CLK) tw(CLK)
FMC_CLK
td(CLKL-NExL) td(CLKH-NExH)
Data latency = 0
FMC_NEx
td(CLKL-NADVL) td(CLKL-NADVH)
FMC_NADV
td(CLKL-AV)
td(CLKH-AIV)
FMC_A[25:0]
td(CLKL-NWEL) td(CLKH-NWEH)
FMC_NWE
td(CLKL-Data) td(CLKL-Data)
FMC_D[15:0] D1 D2
FMC_NWAIT
(WAITCFG = 0b, WAITPOL + 0b) tsu(NWAITV-CLKH) td(CLKH-NBLH)
th(CLKH-NWAITV)
FMC_NBL
MS32760V1
Table 98. Synchronous non-multiplexed PSRAM write timings(1)(2)
Symbol Parameter Min Max Unit
t FMC_CLK period 2T −1 - ns
(CLK) HCLK
t FMC_CLK low to FMC_NEx low (x=0..2) - 0.5 ns
d(CLKL-NExL)
t FMC_CLK high to FMC_NEx high (x= 0…2) T - ns
(CLKH-NExH) HCLK
t FMC_CLK low to FMC_NADV low - 0 ns
d(CLKL-NADVL)
t FMC_CLK low to FMC_NADV high 0 - ns
d(CLKL-NADVH)
t FMC_CLK low to FMC_Ax valid (x=16…25) - 0 ns
d(CLKL-AV)
DS9405 Rev 13 181/240
197

<!-- Page 182 -->


| Symbol | Parameter | Min | Max | Unit |
| --- | --- | --- | --- | --- |
| t d(CLKH-AIV) | FMC_CLK high to FMC_Ax invalid (x=16…25) | 0 | - | ns |
| t d(CLKL-NWEL) | FMC_CLK low to FMC_NWE low | - | 0 | ns |
| t d(CLKH-NWEH) | FMC_CLK high to FMC_NWE high | T −0.5 HCLK | - | ns |
| t d(CLKL-Data) | FMC_D[15:0] valid data after FMC_CLK low | - | 2.5 | ns |
| t d(CLKL-NBLL) | FMC_CLK low to FMC_NBL low | 0 | - | ns |
| t d(CLKH-NBLH) | FMC_CLK high to FMC_NBL high | T −0.5 HCLK | - | ns |
| t su(NWAIT-CLKH) | FMC_NWAIT valid before FMC_CLK high | 4 | - | - |
| t h(CLKH-NWAIT) | FMC_NWAIT valid after FMC_CLK high | 0 | - | - |

Electrical characteristics STM32F427xx STM32F429xx
Table 98. Synchronous non-multiplexed PSRAM write timings(1)(2) (continued)
Symbol Parameter Min Max Unit
t FMC_CLK high to FMC_Ax invalid (x=16…25) 0 - ns
d(CLKH-AIV)
t FMC_CLK low to FMC_NWE low - 0 ns
d(CLKL-NWEL)
t FMC_CLK high to FMC_NWE high T −0.5 - ns
d(CLKH-NWEH) HCLK
t FMC_D[15:0] valid data after FMC_CLK low - 2.5 ns
d(CLKL-Data)
t FMC_CLK low to FMC_NBL low 0 - ns
d(CLKL-NBLL)
t FMC_CLK high to FMC_NBL high T −0.5 - ns
d(CLKH-NBLH) HCLK
t FMC_NWAIT valid before FMC_CLK high 4 - -
su(NWAIT-CLKH)
t FMC_NWAIT valid after FMC_CLK high 0 - -
h(CLKH-NWAIT)
1. C = 30pF.
L
2. Evaluated by characterization.
PC Card/CompactFlash controller waveforms and timings
Figure63 through Figure68 represent synchronous waveforms, and Table99 and
Table100 provide the corresponding timings. The results shown in this table are obtained
with the following FMC configuration:
• COM.FMC_SetupTime = 0x04;
• COM.FMC_WaitSetupTime = 0x07;
• COM.FMC_HoldSetupTime = 0x04;
• COM.FMC_HiZSetupTime = 0x00;
• ATT.FMC_SetupTime = 0x04;
• ATT.FMC_WaitSetupTime = 0x07;
• ATT.FMC_HoldSetupTime = 0x04;
• ATT.FMC_HiZSetupTime = 0x00;
• IO.FMC_SetupTime = 0x04;
• IO.FMC_WaitSetupTime = 0x07;
• IO.FMC_HoldSetupTime = 0x04;
• IO.FMC_HiZSetupTime = 0x00;
• TCLRSetupTime = 0;
• TARSetupTime = 0.
In all timing tables, the T is the HCLK clock period.
HCLK
182/240 DS9405 Rev 13

<!-- Page 183 -->


|  | tv(NCE4_1-A) th(NCE4_1-AI) t t d d ( ( N N R IO E R G D -N -N C C E E 4 4 _ _ 1 1 ) ) t t t h h h ( ( ( N N N C C C E E E 4 4 4 _ _ _ 1 1 1 - - - N N N R I I O O E R W G D R ) ) ) |  |
| --- | --- | --- |
|  |  |  |
|  | tw(NWE) td(NWE-NCE4_1) |  |
|  |  |  |
|  |  |  |

STM32F427xx STM32F429xx Electrical characteristics
Figure 63. PC Card/CompactFlash controller waveforms for common memory read
access
FMC_NCE4_2(1)
FMC_NCE4_1
tv(NCEx-A) th(NCEx-AI)
FMC_A[10:0]
t t d d ( ( N N R IO E R G D -N -N C C E E x x ) ) t t th h h ( ( ( N N N C C C E E E x x x - - - N N N I R I O O E W R G D R ) ) )
FMC_NREG
FMC_NIOWR
FMC_NIORD
FMC_NWE
td(NCE4_1-NOE) tw(NOE)
FMC_NOE
tsu(D-NOE) th(NOE-D)
FMC_D[15:0]
MS32761V1
1. FMC_NCE4_2 remains high (inactive during 8-bit access.
Figure 64. PC Card/CompactFlash controller waveforms for common memory write
access
FMC_NCE4_1
FMC_NCE4_2 High
tv(NCE4_1-A) th(NCE4_1-AI)
FMC_A[10:0]
t t d d ( ( N N R IO E R G D -N -N C C E E 4 4 _ _ 1 1 ) ) t t t h h h ( ( ( N N N C C C E E E 4 4 4 _ _ _ 1 1 1 - - - N N N R I I O O E R W G D R ) ) )
FMC_NREG
FMC_NIOWR
FMC_NIORD
td(NCE4_1-NWE) tw(NWE) td(NWE-NCE4_1)
FMC_NWE
FMC_NOE
MEMxHIZ =1
td(D-NWE)
tv(NWE-D) th(NWE-D)
FMC_D[15:0]
MS32762V1
DS9405 Rev 13 183/240
197

<!-- Page 184 -->


|  |
| --- |
|  |
|  |


|  | td(NREG-NCE4_1) th(NCE4_1-NREG) |
| --- | --- |
|  |  |

Electrical characteristics STM32F427xx STM32F429xx
Figure 65. PC Card/CompactFlash controller waveforms for attribute memory
read access
FMC_NCE4_1
tv(NCE4_1-A) th(NCE4_1-AI)
FMC_NCE4_2
High
FMC_A[10:0]
FMC_NIOWR
FMC_NIORD
td(NREG-NCE4_1) th(NCE4_1-NREG)
FMC_NREG
FMC_NWE
td(NCE4_1-NOE) tw(NOE) td(NOE-NCE4_1)
FMC_NOE
tsu(D-NOE) th(NOE-D)
FMC_D[15:0](1)
MS32763V1
1. Only data bits 0...7 are read (bits 8...15 are disregarded).
184/240 DS9405 Rev 13

<!-- Page 185 -->


|  |  |  |
| --- | --- | --- |
|  | tv(NCE4_1-A) th(NCE4_1-AI) |  |
|  | td(NREG-NCE4_1) th(NCE4_1-NREG) |  |
|  |  |  |

STM32F427xx STM32F429xx Electrical characteristics
Figure 66. PC Card/CompactFlash controller waveforms for attribute memory
write access
FMC_NCE4_1
FMC_NCE4_2 High
tv(NCE4_1-A) th(NCE4_1-AI)
FMC_A[10:0]
FMC_NIOWR
FMC_NIORD
td(NREG-NCE4_1) th(NCE4_1-NREG)
FMC_NREG
td(NCE4_1-NWE) tw(NWE)
FMC_NWE
td(NWE-NCE4_1)
FMC_NOE
tv(NWE-D)
FMC_D[7:0](1)
MS32764V1
1. Only data bits 0...7 are driven (bits 8...15 remains Hi-Z).
Figure 67. PC Card/CompactFlash controller waveforms for I/O space read access
FMC_NCE4_1
FMC_NCE4_2
tv(NCEx-A) th(NCE4_1-AI)
FMC_A[10:0]
FMC_NREG
FMC_NWE
FMC_NOE
FMC_NIOWR
td(NIORD-NCE4_1) tw(NIORD)
FMC_NIORD
tsu(D-NIORD) td(NIORD-D)
FMC_D[15:0]
MS32765V1
DS9405 Rev 13 185/240
197

<!-- Page 186 -->


|  |
| --- |
|  |


| Symbol | Parameter | Min | Max | Unit |
| --- | --- | --- | --- | --- |
| t v(NCEx-A) | FMC_Ncex low to FMC_Ay valid | - | 0 | ns |
| t h(NCEx_AI) | FMC_NCEx high to FMC_Ax invalid | 0 | - | ns |
| t d(NREG-NCEx) | FMC_NCEx low to FMC_NREG valid | - | 1 | ns |
| t h(NCEx-NREG) | FMC_NCEx high to FMC_NREG invalid | T −2 HCLK | - | ns |
| t d(NCEx-NWE) | FMC_NCEx low to FMC_NWE low | - | 5T HCLK | ns |
| t w(NWE) | FMC_NWE low width | 8T −0.5 HCLK | 8T +0.5 HCLK | ns |
| t d(NWE_NCEx) | FMC_NWE high to FMC_NCEx high | 5T +1 HCLK | - | ns |
| t V(NWE-D) | FMC_NWE low to FMC_D[15:0] valid | - | 0 | ns |
| t h(NWE-D) | FMC_NWE high to FMC_D[15:0] invalid | 9T −0.5 HCLK | - | ns |
| t d(D-NWE) | FMC_D[15:0] valid before FMC_NWE high | 13T −3 HCLK | - | ns |
| t d(NCEx-NOE) | FMC_NCEx low to FMC_NOE low | - | 5T HCLK | ns |
| t w(NOE) | FMC_NOE low width | 8 T −0.5 HCLK | 8 T +0.5 HCLK | ns |
| t d(NOE_NCEx) | FMC_NOE high to FMC_NCEx high | 5T −1 HCLK | - | ns |
| t su (D-NOE) | FMC_D[15:0] valid data before FMC_NOE high | T HCLK | - | ns |
| t h(NOE-D) | FMC_NOE high to FMC_D[15:0] invalid | 0 | - | ns |

Electrical characteristics STM32F427xx STM32F429xx
Figure 68. PC Card/CompactFlash controller waveforms for I/O space write access
FMC_NCE4_1
FMC_NCE4_2
tv(NCEx-A) th(NCE4_1-AI)
FMC_A[10:0]
FMC_NREG
FMC_NWE
FMC_NOE
FMC_NIORD
td(NCE4_1-NIOWR) tw(NIOWR)
FMC_NIOWR
ATTxHIZ =1
th(NIOWR-D)
tv(NIOWR-D)
FMC_D[15:0]
MS32766V1
T able 99. Switching characteristics for PC Card/CF read and write cycles
in attribute/common space(1)(2)
Symbol Parameter Min Max Unit
t FMC_Ncex low to FMC_Ay valid - 0 ns
v(NCEx-A)
t FMC_NCEx high to FMC_Ax invalid 0 - ns
h(NCEx_AI)
t FMC_NCEx low to FMC_NREG valid - 1 ns
d(NREG-NCEx)
t FMC_NCEx high to FMC_NREG invalid T −2 - ns
h(NCEx-NREG) HCLK
t FMC_NCEx low to FMC_NWE low - 5T ns
d(NCEx-NWE) HCLK
t FMC_NWE low width 8T −0.5 8T +0.5 ns
w(NWE) HCLK HCLK
t FMC_NWE high to FMC_NCEx high 5T +1 - ns
d(NWE_NCEx) HCLK
t FMC_NWE low to FMC_D[15:0] valid - 0 ns
V(NWE-D)
t FMC_NWE high to FMC_D[15:0] invalid 9T −0.5 - ns
h(NWE-D) HCLK
t FMC_D[15:0] valid before FMC_NWE high 13T −3 - ns
d(D-NWE) HCLK
t FMC_NCEx low to FMC_NOE low - 5T ns
d(NCEx-NOE) HCLK
t FMC_NOE low width 8 T −0.5 8 T +0.5 ns
w(NOE) HCLK HCLK
t FMC_NOE high to FMC_NCEx high 5T −1 - ns
d(NOE_NCEx) HCLK
t FMC_D[15:0] valid data before FMC_NOE high T - ns
su (D-NOE) HCLK
t FMC_NOE high to FMC_D[15:0] invalid 0 - ns
h(NOE-D)
1. C = 30pF.
L
2. Guaranteed by characterization results.
186/240 DS9405 Rev 13

<!-- Page 187 -->


| Symbol | Parameter | Min | Max | Unit |
| --- | --- | --- | --- | --- |
| tw(NIOWR) | FMC_NIOWR low width | 8T −0.5 HCLK | - | ns |
| tv(NIOWR-D) | FMC_NIOWR low to FMC_D[15:0] valid | - | 0 | ns |
| th(NIOWR-D) | FMC_NIOWR high to FMC_D[15:0] invalid | 9T −2 HCLK | - | ns |
| td(NCE4_1-NIOWR) | FMC_NCE4_1 low to FMC_NIOWR valid | - | 5T HCLK | ns |
| th(NCEx-NIOWR) | FMC_NCEx high to FMC_NIOWR invalid | 5T HCLK | - | ns |
| td(NIORD-NCEx) | FMC_NCEx low to FMC_NIORD valid | - | 5T HCLK | ns |
| th(NCEx-NIORD) | FMC_NCEx high to FMC_NIORD) valid | 6T +2 HCLK | - | ns |
| tw(NIORD) | FMC_NIORD low width | 8T −0.5 HCLK | 8T +0.5 HCLK | ns |
| tsu(D-NIORD) | FMC_D[15:0] valid before FMC_NIORD high | T HCLK | - | ns |
| td(NIORD-D) | FMC_D[15:0] valid after FMC_NIORD high | 0 | - | ns |

STM32F427xx STM32F429xx Electrical characteristics
T a ble 100. Switching characteristics for PC Card/CF read and write cycles
in I/O space(1)(2)
Symbol Parameter Min Max Unit
tw(NIOWR) FMC_NIOWR low width 8T −0.5 - ns
HCLK
tv(NIOWR-D) FMC_NIOWR low to FMC_D[15:0] valid - 0 ns
th(NIOWR-D) FMC_NIOWR high to FMC_D[15:0] invalid 9T −2 - ns
HCLK
td(NCE4_1-NIOWR) FMC_NCE4_1 low to FMC_NIOWR valid - 5T ns
HCLK
th(NCEx-NIOWR) FMC_NCEx high to FMC_NIOWR invalid 5T - ns
HCLK
td(NIORD-NCEx) FMC_NCEx low to FMC_NIORD valid - 5T ns
HCLK
th(NCEx-NIORD) FMC_NCEx high to FMC_NIORD) valid 6T +2 - ns
HCLK
tw(NIORD) FMC_NIORD low width 8T −0.5 8T +0.5 ns
HCLK HCLK
tsu(D-NIORD) FMC_D[15:0] valid before FMC_NIORD high T - ns
HCLK
td(NIORD-D) FMC_D[15:0] valid after FMC_NIORD high 0 - ns
1. C = 30pF.
L
2. Evaluated by characterization.
NAND controller waveforms and timings
Figure69 and Figure70 represent synchronous waveforms, and Table101 and Table102
provide the corresponding timings. The results shown in this table are obtained with the
following FMC configuration:
• COM.FMC_SetupTime = 0x01;
• COM.FMC_WaitSetupTime = 0x03;
• COM.FMC_HoldSetupTime = 0x02;
• COM.FMC_HiZSetupTime = 0x01;
• ATT.FMC_SetupTime = 0x01;
• ATT.FMC_WaitSetupTime = 0x03;
• ATT.FMC_HoldSetupTime = 0x02;
• ATT.FMC_HiZSetupTime = 0x01;
• Bank = FMC_Bank_NAND;
• MemoryDataWidth = FMC_MemoryDataWidth_16b;
• ECC = FMC_ECC_Enable;
• ECCPageSize = FMC_ECCPageSize_512Bytes;
• TCLRSetupTime = 0;
• TARSetupTime = 0.
In all timing tables, the T is the HCLK clock period.
HCLK
DS9405 Rev 13 187/240
197

<!-- Page 188 -->


| Symbol | Parameter | Min | Max | Unit |
| --- | --- | --- | --- | --- |
| t w(N0E) | FMC_NOE low width | 4T −0.5 HCLK | 4T +0.5 HCLK | ns |
| t su(D-NOE) | FMC_D[15-0] valid data before FMC_NOE high | 9 | - | ns |
| t h(NOE-D) | FMC_D[15-0] valid data after FMC_NOE high | 0 | - | ns |
| t d(ALE-NOE) | FMC_ALE valid before FMC_NOE low | - | 3T −0.5 HCLK | ns |
| t h(NOE-ALE) | FMC_NWE high to FMC_ALE invalid | 3T −2 HCLK | - | ns |

Electrical characteristics STM32F427xx STM32F429xx
Figure 69. NAND controller waveforms for read access
FMC_NCEx
ALE (FMC_A17)
CLE (FMC_A16)
t d(ALE-NOE) t h(NOE-ALE)
FMC_NWE
t
w(NOE)
FMC_NOE (NRE)
t t
su(D-NOE) h(NOE-D)
FMC_D[y:0]
MSv73150V1
1. y= 7 or 15 depending on the NAND flash memory interface.
Figure 70. NAND controller waveforms for write access
FMC_NCEx
ALE (FMC_A17)
CLE (FMC_A16)
t d(ALE-NWE) t w(NWE) t h(NWE-ALE)
FMC_NWE
FMC_NOE (NRE)
t
d(D-NWE)
t v(NWE-D) t h(NWE-D)
FMC_D[y:0]
MSv73151V1
2. y= 7 or 15 depending on the NAND flash memory interface.
N Table 101. Switching characteristics for NAND Flash read cycles(1)
Symbol Parameter Min Max Unit
t FMC_NOE low width 4T −0.5 4T +0.5 ns
w(N0E) HCLK HCLK
t FMC_D[15-0] valid data before FMC_NOE high 9 - ns
su(D-NOE)
t FMC_D[15-0] valid data after FMC_NOE high 0 - ns
h(NOE-D)
t FMC_ALE valid before FMC_NOE low - 3T −0.5 ns
d(ALE-NOE) HCLK
t FMC_NWE high to FMC_ALE invalid 3T −2 - ns
h(NOE-ALE) HCLK
1. C = 30pF.
L
188/240 DS9405 Rev 13

<!-- Page 189 -->


| Symbol | Parameter | Min | Max | Unit |
| --- | --- | --- | --- | --- |
| t w(NWE) | FMC_NWE low width | 4T HCLK | 4T +1 HCLK | ns |
| t v(NWE-D) | FMC_NWE low to FMC_D[15-0] valid | 0 | - | ns |
| t h(NWE-D) | FMC_NWE high to FMC_D[15-0] invalid | 3T −1 HCLK | - | ns |
| t d(D-NWE) | FMC_D[15-0] valid before FMC_NWE high | 5T −3 HCLK | - | ns |
| t d(ALE-NWE) | FMC_ALE valid before FMC_NWE low | - | 3T −0.5 HCLK | ns |
| t h(NWE-ALE) | FMC_NWE high to FMC_ALE invalid | 3T −1 HCLK | - | ns |


|  |  |  |  |  |
| --- | --- | --- | --- | --- |
|  |  | Coln |  |  |
|  |  |  |  |  |
|  |  |  |  |  |
| L_NRAS) |  |  |  |  |
| L_NCAS) |  |  |  |  |
|  |  |  |  |  |
|  |  |  |  |  |

STM32F427xx STM32F429xx Electrical characteristics
Table 102. Switching characteristics for NAND Flash write cycles(1)
Symbol Parameter Min Max Unit
t FMC_NWE low width 4T 4T +1 ns
w(NWE) HCLK HCLK
t FMC_NWE low to FMC_D[15-0] valid 0 - ns
v(NWE-D)
t FMC_NWE high to FMC_D[15-0] invalid 3T −1 - ns
h(NWE-D) HCLK
t FMC_D[15-0] valid before FMC_NWE high 5T −3 - ns
d(D-NWE) HCLK
t FMC_ALE valid before FMC_NWE low - 3T −0.5 ns
d(ALE-NWE) HCLK
t FMC_NWE high to FMC_ALE invalid 3T −1 - ns
h(NWE-ALE) HCLK
1. C = 30pF.
L
SDRAM waveforms and timings
Figure 71. SDRAM read access waveforms (CL = 1)
FMC_SDCLK
td(SDCLKL_AddC)
td(SDCLKL_AddR)
th(SDCLKL_AddR)
FMC_A[12:0] Row n Col1 Col2 Coli Coln
th(SDCLKL_AddC)
td(SDCLKL_SNDE) th(SDCLKL_SNDE)
FMC_SDNE[1:0]
td(SDCLKL_NRAS) th(SDCLKL_NRAS)
FMC_SDNRAS
td(SDCLKL_NCAS) th(SDCLKL_NCAS)
FMC_SDNCAS
FMC_SDNWE
tsu(SDCLKH_Data) th(SDCLKH_Data)
FMC_D[31:0] Data1 Data2 Datai Datan
MS32751V2
DS9405 Rev 13 189/240
197

<!-- Page 190 -->


| Symbol | Parameter | Min | Max | Unit |
| --- | --- | --- | --- | --- |
| t w(SDCLK) | FMC_SDCLK period | 2T −0.5 HCLK | 2T +0.5 HCLK | ns |
| t su(SDCLKH _Data) | Data input setup time | 2 | - |  |
| t h(SDCLKH_Data) | Data input hold time | 0 | - |  |
| t d(SDCLKL_Add) | Address valid time | - | 1.5 |  |
| t d(SDCLKL- SDNE) | Chip select valid time | - | 0.5 |  |
| t h(SDCLKL_SDNE) | Chip select hold time | 0 | - |  |
| t d(SDCLKL_SDNRAS) | SDNRAS valid time | - | 0.5 |  |
| t h(SDCLKL_SDNRAS) | SDNRAS hold time | 0 | - |  |
| t d(SDCLKL_SDNCAS) | SDNCAS valid time | - | 0.5 |  |
| t h(SDCLKL_SDNCAS) | SDNCAS hold time | 0 | - |  |


| Symbol | Parameter | Min | Max | Unit |
| --- | --- | --- | --- | --- |
| t W(SDCLK) | FMC_SDCLK period | 2T −0.5 HCLK | 2T +0.5 HCLK | ns |
| t su(SDCLKH_Data) | Data input setup time | 2.5 | - |  |
| t h(SDCLKH_Data) | Data input hold time | 0 | - |  |
| t d(SDCLKL_Add) | Address valid time | - | 1 |  |
| t d(SDCLKL_SDNE) | Chip select valid time | - | 1 |  |
| t h(SDCLKL_SDNE) | Chip select hold time | 1 | - |  |
| t d(SDCLKL_SDNRAS | SDNRAS valid time | - | 1 |  |
| t h(SDCLKL_SDNRAS) | SDNRAS hold time | 1 | - |  |
| t d(SDCLKL_SDNCAS) | SDNCAS valid time | - | 1 |  |
| t h(SDCLKL_SDNCAS) | SDNCAS hold time | 1 | - |  |

Electrical characteristics STM32F427xx STM32F429xx
Table 103. SDRAM read timings(1)(2)
Symbol Parameter Min Max Unit
t FMC_SDCLK period 2T −0.5 2T +0.5
w(SDCLK) HCLK HCLK
t Data input setup time 2 -
su(SDCLKH _Data)
t Data input hold time 0 -
h(SDCLKH_Data)
t Address valid time - 1.5
d(SDCLKL_Add)
t Chip select valid time - 0.5
d(SDCLKL- SDNE)
ns
t Chip select hold time 0 -
h(SDCLKL_SDNE)
t SDNRAS valid time - 0.5
d(SDCLKL_SDNRAS)
t SDNRAS hold time 0 -
h(SDCLKL_SDNRAS)
t SDNCAS valid time - 0.5
d(SDCLKL_SDNCAS)
t SDNCAS hold time 0 -
h(SDCLKL_SDNCAS)
1. CL = 30 pF on data and address lines. CL=15pF on FMC_SDCLK.
2. Evaluated by characterization.
Table 104. LPSDR SDRAM read timings(1)(2)
Symbol Parameter Min Max Unit
t FMC_SDCLK period 2T −0.5 2T +0.5
W(SDCLK) HCLK HCLK
t Data input setup time 2.5 -
su(SDCLKH_Data)
t Data input hold time 0 -
h(SDCLKH_Data)
t Address valid time - 1
d(SDCLKL_Add)
t Chip select valid time - 1
d(SDCLKL_SDNE)
ns
t Chip select hold time 1 -
h(SDCLKL_SDNE)
t SDNRAS valid time - 1
d(SDCLKL_SDNRAS
t SDNRAS hold time 1 -
h(SDCLKL_SDNRAS)
t SDNCAS valid time - 1
d(SDCLKL_SDNCAS)
t SDNCAS hold time 1 -
h(SDCLKL_SDNCAS)
1. CL = 10 pF.
2. Evaluated by characterization.
190/240 DS9405 Rev 13

<!-- Page 191 -->


|  | L_A L_A |  |  |  |  |  |
| --- | --- | --- | --- | --- | --- | --- |
|  | Col | 2 |  | Coln |  |  |
|  | th(S |  |  |  |  |  |
| DE) |  |  |  |  |  |  |
|  |  |  |  |  |  |  |
| DCLK | L_N | RAS) |  |  |  |  |
| DCLK | L_N | CAS) |  |  |  |  |
|  |  |  |  |  |  |  |
| DCLK | L_N | WE) |  |  |  |  |
|  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |
|  | Data2 |  |  | Datan |  |  |


| 1 |
| --- |
|  |

STM32F427xx STM32F429xx Electrical characteristics
Figure 72. SDRAM write access waveforms
FMC_SDCLK
td(SDCLKL_AddC)
td(SDCLKL_AddR)
th(SDCLKL_AddR)
FMC_A[12:0] Row n Col1 Col2 Coli Coln
th(SDCLKL_AddC)
td(SDCLKL_SNDE) th(SDCLKL_SNDE)
FMC_SDNE[1:0]
td(SDCLKL_NRAS) th(SDCLKL_NRAS)
FMC_SDNRAS
td(SDCLKL_NCAS) th(SDCLKL_NCAS)
FMC_SDNCAS
td(SDCLKL_NWE) th(SDCLKL_NWE)
FMC_SDNWE
td(SDCLKL_Data)
FMC_D[31:0] Data1 Data2 Datai Datan
td(SDCLKL_NBL) th(SDCLKL_Data)
FMC_NBL[3:0]
MS32752V2
DS9405 Rev 13 191/240
197

<!-- Page 192 -->


| Symbol | Parameter | Min | Max | Unit |
| --- | --- | --- | --- | --- |
| t w(SDCLK) | FMC_SDCLK period | 2T −0.5 HCLK | 2T +0.5 HCLK | ns |
| t ) d(SDCLKL _Data | Data output valid time | - | 3.5 |  |
| t h(SDCLKL _Data) | Data output hold time | 0 | - |  |
| t d(SDCLKL_Add) | Address valid time | - | 1.5 |  |
| t d(SDCLKL_SDNWE) | SDNWE valid time | - | 1 |  |
| t h(SDCLKL_SDNWE) | SDNWE hold time | 0 | - |  |
| t d(SDCLKL_ SDNE) | Chip select valid time | - | 0.5 |  |
| t h(SDCLKL-_SDNE) | Chip select hold time | 0 | - |  |
| t d(SDCLKL_SDNRAS) | SDNRAS valid time | - | 2 |  |
| t h(SDCLKL_SDNRAS) | SDNRAS hold time | 0 | - |  |
| t d(SDCLKL_SDNCAS) | SDNCAS valid time | - | 0.5 |  |
| t d(SDCLKL_SDNCAS) | SDNCAS hold time | 0 | - |  |
| t d(SDCLKL_NBL) | NBL valid time | - | 0.5 |  |
| t h(SDCLKL_NBL) | NBLoutput time | 0 | - |  |


| Symbol | Parameter | Min | Max | Unit |
| --- | --- | --- | --- | --- |
| t w(SDCLK) | FMC_SDCLK period | 2T −0.5 HCLK | 2T +0.5 HCLK | ns |
| t ) d(SDCLKL _Data | Data output valid time | - | 5 |  |
| t h(SDCLKL _Data) | Data output hold time | 2 | - |  |
| t d(SDCLKL_Add) | Address valid time | - | 2.8 |  |
| t d(SDCLKL-SDNWE) | SDNWE valid time | - | 2 |  |
| t h(SDCLKL-SDNWE) | SDNWE hold time | 1 | - |  |
| t d(SDCLKL- SDNE) | Chip select valid time | - | 1.5 |  |
| t h(SDCLKL- SDNE) | Chip select hold time | 1 | - |  |
| t d(SDCLKL-SDNRAS) | SDNRAS valid time | - | 1.5 |  |
| t h(SDCLKL-SDNRAS) | SDNRAS hold time | 1.5 | - |  |
| t d(SDCLKL-SDNCAS) | SDNCAS valid time | - | 1.5 |  |
| t d(SDCLKL-SDNCAS) | SDNCAS hold time | 1.5 | - |  |
| t d(SDCLKL_NBL) | NBL valid time | - | 1.5 |  |
| t h(SDCLKL-NBL) | NBL output time | 1.5 | - |  |

Electrical characteristics STM32F427xx STM32F429xx
Table 105. SDRAM write timings(1)(2)
Symbol Parameter Min Max Unit
t FMC_SDCLK period 2T −0.5 2T +0.5
w(SDCLK) HCLK HCLK
t ) Data output valid time - 3.5
d(SDCLKL _Data
t Data output hold time 0 -
h(SDCLKL _Data)
t Address valid time - 1.5
d(SDCLKL_Add)
t SDNWE valid time - 1
d(SDCLKL_SDNWE)
t SDNWE hold time 0 -
h(SDCLKL_SDNWE)
t Chip select valid time - 0.5
d(SDCLKL_ SDNE)
ns
t Chip select hold time 0 -
h(SDCLKL-_SDNE)
t SDNRAS valid time - 2
d(SDCLKL_SDNRAS)
t SDNRAS hold time 0 -
h(SDCLKL_SDNRAS)
t SDNCAS valid time - 0.5
d(SDCLKL_SDNCAS)
t SDNCAS hold time 0 -
d(SDCLKL_SDNCAS)
t NBL valid time - 0.5
d(SDCLKL_NBL)
t NBLoutput time 0 -
h(SDCLKL_NBL)
1. CL = 30 pF on data and address lines. CL=15 pF on FMC_SDCLK.
2. Evaluated by characterization.
Table 106. LPSDR SDRAM write timings(1)(2)
Symbol Parameter Min Max Unit
t FMC_SDCLK period 2T −0.5 2T +0.5
w(SDCLK) HCLK HCLK
t ) Data output valid time - 5
d(SDCLKL _Data
t Data output hold time 2 -
h(SDCLKL _Data)
t Address valid time - 2.8
d(SDCLKL_Add)
t SDNWE valid time - 2
d(SDCLKL-SDNWE)
t SDNWE hold time 1 -
h(SDCLKL-SDNWE)
t Chip select valid time - 1.5
d(SDCLKL- SDNE)
ns
t Chip select hold time 1 -
h(SDCLKL- SDNE)
t SDNRAS valid time - 1.5
d(SDCLKL-SDNRAS)
t SDNRAS hold time 1.5 -
h(SDCLKL-SDNRAS)
t SDNCAS valid time - 1.5
d(SDCLKL-SDNCAS)
t SDNCAS hold time 1.5 -
d(SDCLKL-SDNCAS)
t NBL valid time - 1.5
d(SDCLKL_NBL)
t NBL output time 1.5 -
h(SDCLKL-NBL)
1. CL = 10 pF.
2. Evaluated by characterization.
192/240 DS9405 Rev 13

<!-- Page 193 -->


| Symbol | Parameter | Min | Max | Unit |
| --- | --- | --- | --- | --- |
|  | Frequency ratio DCMI_PIXCLK/f HCLK | - | 0.4 |  |
| DCMI_PIXCLK | Pixel clock input | - | 54 | MHz |
| D Pixel | Pixel clock input duty cycle | 30 | 70 | % |
| t su(DATA) | Data input setup time | 2 | - | ns |
| t h(DATA) | Data input hold time | 2.5 | - |  |
| t su(HSYNC) t su(VSYNC) | DCMI_HSYNC/DCMI_VSYNC input setup time | 0.5 | - |  |
| t h(HSYNC) t h(VSYNC) | DCMI_HSYNC/DCMI_VSYNC input hold time | 1 | - |  |


|  |  |
| --- | --- |
|  |  |


|  |  |
| --- | --- |
|  |  |

STM32F427xx STM32F429xx Electrical characteristics
6.3.27 Camera interface (DCMI) timing specifications
Unless otherwise specified, the parameters given in Table107 for DCMI are derived
from tests performed under the ambient temperature, f frequency, and V supply
HCLK DD
voltage summarized in Table17, with the following configuration:
• DCMI_PIXCLK polarity: falling
• DCMI_VSYNC and DCMI_HSYNC polarity: high
• Data formats: 14 bits
Table 107. DCMI characteristics
Symbol Parameter Min Max Unit
Frequency ratio DCMI_PIXCLK/f - 0.4
HCLK
DCMI_PIXCLK Pixel clock input - 54 MHz
D Pixel clock input duty cycle 30 70 %
Pixel
t Data input setup time 2 -
su(DATA)
t Data input hold time 2.5 -
h(DATA)
t
su(HSYNC) DCMI_HSYNC/DCMI_VSYNC input setup time 0.5 - ns
t
su(VSYNC)
t
h(HSYNC) DCMI_HSYNC/DCMI_VSYNC input hold time 1 -
t
h(VSYNC)
Figure 73. DCMI timing diagram
1/DCMI_PIXCLK
DCMI_PIXCLK
t su(HSYNC) t h(HSYNC)
DCMI_HSYNC
t su(VSYNC) t h(HSYNC)
DCMI_VSYNC
t t
su(DATA) h(DATA)
DATA[0:13]
MS32414V2
DS9405 Rev 13 193/240
197

<!-- Page 194 -->


| Symbol | Parameter | Min | Max | Unit |
| --- | --- | --- | --- | --- |
| f CLK | LTDC clock output frequency | - | 83 | MHz |
| D CLK | LTDC clock output duty cycle | 45 | 55 | % |
| t w(CLKH) t w(CLKL) | Clock High time, low time | tw(CLK)/2−0.5 | tw(CLK)/2+0.5 | ns |
| t v(DATA) | Data output valid time | - | 3.5 |  |
| t h(DATA) | Data output hold time | 1.5 | - |  |
| t v(HSYNC) | HSYNC/VSYNC/DE output valid time | - | 2.5 |  |
| t v(VSYNC) |  |  |  |  |
| t v(DE) |  |  |  |  |
| t h(HSYNC) | HSYNC/VSYNC/DE output hold time | 2 | - |  |
| t h(VSYNC) |  |  |  |  |
| th(DE) |  |  |  |  |

Electrical characteristics STM32F427xx STM32F429xx
6.3.28 LCD-TFT controller (LTDC) characteristics
Unless otherwise specified, the parameters given in Table108 for LCD-TFT are derived
from tests performed under the ambient temperature, f HCLK frequency, and VDD supply
voltage summarized in Table17, with the following configuration:
• LCD_CLK polarity: high
• LCD_DE polarity: low
• LCD_VSYNC and LCD_HSYNC polarity: high
• Pixel formats: 24 bits
Table 108. LTDC characteristics
Symbol Parameter Min Max Unit
f LTDC clock output frequency - 83 MHz
CLK
D LTDC clock output duty cycle 45 55 %
CLK
t
w(CLKH) Clock High time, low time tw(CLK)/2−0.5 tw(CLK)/2+0.5
t
w(CLKL)
t Data output valid time - 3.5
v(DATA)
t Data output hold time 1.5 -
h(DATA)
t
v(HSYNC)
HSYNC/VSYNC/DE output valid ns
t - 2.5
v(VSYNC) time
t
v(DE)
t
h(HSYNC)
HSYNC/VSYNC/DE output hold
t 2 -
h(VSYNC) time
th(DE)
194/240 DS9405 Rev 13

<!-- Page 195 -->


|  | tv(DE) |  |  |  |  | tv(HSYNC) |
| --- | --- | --- | --- | --- | --- | --- |
|  |  |  |  |  | th(DE) |  |
|  |  |  |  |  |  |  |
|  | tv(D |  |  |  |  |  |
|  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |


| M lines data |  |
| --- | --- |
|  |  |


|  |  |
| --- | --- |

STM32F427xx STM32F429xx Electrical characteristics
Figure 74. LCD-TFT horizontal timing diagram
tCLK
LCD_CLK
LCD_VSYNC
tv(HSYNC) tv(HSYNC)
LCD_HSYNC
tv(DE) th(DE)
LCD_DE
tv(DATA)
LCD_R[0:7]
LCD_G[0:7] Pixel Pixel Pixel
1 2 N
LCD_B[0:7]
th(DATA)
HSYNCHorizontal Active width Horizontal
width back porch back porch
One line
MS32749V1
Figure 75. LCD-TFT vertical timing diagram
tCLK
LCD_CLK
tv(VSYNC) tv(VSYNC)
LCD_VSYNC
LCD_R[0:7]
LCD_G[0:7] M lines data
LCD_B[0:7]
VSYNC Vertical Active width Vertical
width back porch back porch
One frame
MS32750V1
DS9405 Rev 13 195/240
197

<!-- Page 196 -->

Electrical characteristics STM32F427xx STM32F429xx
6.3.29 SD/SDIO MMC card host interface (SDIO) characteristics
Unless otherwise specified, the parameters given in Table109 for the SDIO/MMC interface
are derived from tests performed under the ambient temperature, f frequency and V
PCLK2 DD
supply voltage conditions summarized in Table17, with the following configuration:
• Output speed is set to OSPEEDRy[1:0] = 10
• Capacitive load C = 30pF
• Measurement points are done at CMOS levels: 0.5V
DD
Refer to Section6.3.17: I/O port characteristics for more details on the input/output
characteristics.
Figure 76. SDIO high-speed mode
Figure 77. SD default mode
CK
tOVD tOHD
D, CMD
(output)
ai14888
196/240 DS9405 Rev 13

<!-- Page 197 -->


| Symbol | Parameter | Conditions | Min | Typ | Max | Unit |
| --- | --- | --- | --- | --- | --- | --- |
| f PP | Clock frequency in data transfer mode | - | 0 | - | 48 | MHz |
| - | SDIO_CK/fPCLK2 frequency ratio | - | - | - | 8/3 | - |
| t W(CKL) | Clock low time | fpp =48MHz | 8.5 | 9 | - | ns |
| t W(CKH) | Clock high time | fpp =48MHz | 8.3 | 10 | - |  |
| CMD, D inputs (referenced to CK) in MMC and SD HS mode |  |  |  |  |  |  |
| t ISU | Input setup time HS | fpp =48MHz | 3.5 | - | - | ns |
| t IH | Input hold time HS | fpp =48MHz | 0 | - | - |  |
| CMD, D outputs (referenced to CK) in MMC and SD HS mode |  |  |  |  |  |  |
| t OV | Output valid time HS | fpp =48MHz | - | 4.5 | 7 | ns |
| t OH | Output hold time HS | fpp =48MHz | 3 | - | - |  |
| CMD, D inputs (referenced to CK) in SD default mode |  |  |  |  |  |  |
| tISUD | Input setup time SD | fpp =24MHz | 1.5 | - | - | ns |
| tIHD | Input hold time SD | fpp =24MHz | 0.5 | - | - |  |
| CMD, D outputs (referenced to CK) in SD default mode |  |  |  |  |  |  |
| tOVD | Output valid default time SD | fpp =24MHz | - | 4.5 | 6.5 | ns |
| tOHD | Output hold default time SD | fpp =24MHz | 3.5 | - | - |  |


| Symbol | Parameter | Conditions | Min | Max |
| --- | --- | --- | --- | --- |
| - | f /RTCCLKfrequency ratio PCLK1 | Any read/write operation from/to an RTC register | 4 | - |

STM32F427xx STM32F429xx Electrical characteristics
Table 109. Dynamic characteristics: SD / MMC characteristics(1)(2)
Symbol Parameter Conditions Min Typ Max Unit
f Clock frequency in data transfer mode - 0 - 48 MHz
PP
- SDIO_CK/fPCLK2 frequency ratio - - - 8/3 -
t Clock low time fpp =48MHz 8.5 9 -
W(CKL)
ns
t Clock high time fpp =48MHz 8.3 10 -
W(CKH)
CMD, D inputs (referenced to CK) in MMC and SD HS mode
t Input setup time HS fpp =48MHz 3.5 - -
ISU
ns
t Input hold time HS fpp =48MHz 0 - -
IH
CMD, D outputs (referenced to CK) in MMC and SD HS mode
t Output valid time HS fpp =48MHz - 4.5 7
OV
ns
t Output hold time HS fpp =48MHz 3 - -
OH
CMD, D inputs (referenced to CK) in SD default mode
tISUD Input setup time SD fpp =24MHz 1.5 - -
ns
tIHD Input hold time SD fpp =24MHz 0.5 - -
CMD, D outputs (referenced to CK) in SD default mode
tOVD Output valid default time SD fpp =24MHz - 4.5 6.5
ns
tOHD Output hold default time SD fpp =24MHz 3.5 - -
1. Evaluated by characterization.
2. V = 2.7 to 3.6V.
DD
6.3.30 RTC characteristics
Table 110. RTC characteristics
Symbol Parameter Conditions Min Max
Any read/write operation
- f /RTCCLKfrequency ratio 4 -
PCLK1 from/to an RTC register
DS9405 Rev 13 197/240
197

<!-- Page 198 -->

Package information STM32F427xx STM32F429xx
7 Package information
In order to meet environmental requirements, ST offers these devices in different grades of
ECOPACK® packages, depending on their level of environmental compliance. ECOPACK®
specifications, grade definitions and product status are available at: www.st.com.
ECOPACK® is an ST trademark.
7.1 Device marking
Refer to technical note “Reference device marking schematics for STM32 microcontrollers
and microprocessors” (TN1433 ) available on www.st.com, for the location of pin 1 / ball A1
as well as the location and orientation of the marking areas versus pin 1 / ball A1.
Parts marked as “ES”, “E” or accompanied by an engineering sample notification letter, are
not yet qualified and therefore not approved for use in production. ST is not responsible for
any consequences resulting from such use. In no event will ST be liable for the customer
using any of these engineering samples in production. ST’s Quality department must be
contacted prior to any decision to use these engineering samples to run a qualification
activity.
A WLCSP simplified marking example (if any) is provided in the corresponding package
information subsection.
198/240 DS9405 Rev 13

<!-- Page 199 -->


|  |  |
| --- | --- |
|  |  |


|  |  |
| --- | --- |
| E1/4 D1/4 (6) |  |
|  |  |

STM32F427xx STM32F429xx Package information
7.2 LQFP100 package information (1L)
This LQFP is 100 lead, 14 x 14 mm low-profile quad flat package.
Note: See list of notes in the notes section.
Figure 78. LQFP100 - Outline(15)
(cid:537)2 (cid:537)(cid:20)
(2)
R1
R2
H
N
B-B
O
(6) CTI
D1/4
BSE
GAUGE PLANE
S
B (cid:537)
E1/4
(cid:537)(cid:22) L
4x N/4 TIPS
4x (L1)
aaaCA-BD
bbb HA-BD (1)(11)
BOTTOM VIEW SECTION A-A
(N-4) x e (13)
C
A
(9)(11)
0.05 A2 A1 b aaa CA-BD cccC b WITH PLATING
(12)
SIDE VIEW
D (4)
(2) (5) D1 (11) c c1 (11)
D (3)
(10) (4)
N
b1
BASE METAL
(11)
1
2 3 E1/4 SECTION B-B
D1/4
(6) (2)
A B (5)
E1 E
SECTION A-A
A A
TOP VIEW 1L_LQFP100_ME_V3
DS9405 Rev 13 199/240
231

<!-- Page 200 -->


| Symbol | millimeters |  |  | inches(14) |  |  |
| --- | --- | --- | --- | --- | --- | --- |
|  | Min | Typ | Max | Min | Typ | Max |
| A | - | 1.50 | 1.60 | - | 0.0590 | 0.0630 |
| A1(12) | 0.05 | - | 0.15 | 0.0019 | - | 0.0059 |
| A2 | 1.35 | 1.40 | 1.45 | 0.0531 | 0.0551 | 0.0570 |
| b(9)(11) | 0.17 | 0.22 | 0.27 | 0.0067 | 0.0087 | 0.0106 |
| b1(11) | 0.17 | 0.20 | 0.23 | 0.0067 | 0.0079 | 0.0090 |
| c(11) | 0.09 | - | 0.20 | 0.0035 | - | 0.0079 |
| c1(11) | 0.09 | - | 0.16 | 0.0035 | - | 0.0063 |
| D(4) | 16.00 BSC |  |  | 0.6299 BSC |  |  |
| D1(2)(5) | 14.00 BSC |  |  | 0.5512 BSC |  |  |
| E(4) | 16.00 BSC |  |  | 0.6299 BSC |  |  |
| E1(2)(5) | 14.00 BSC |  |  | 0.5512 BSC |  |  |
| e | 0.50 BSC |  |  | 0.0197 BSC |  |  |
| L | 0.45 | 0.60 | 0.75 | 0.177 | 0.0236 | 0.0295 |
| L1(1)(11) | 1.00 |  |  | - | 0.0394 | - |
| N(13) | 100 |  |  |  |  |  |
| θ | 0° | 3.5° | 7° | 0° | 3.5° | 7° |
| θ1 | 0° | - | - | 0° | - | - |
| θ2 | 10° | 12° | 14° | 10° | 12° | 14° |
| θ3 | 10° | 12° | 14° | 10° | 12° | 14° |
| R1 | 0.08 | - | - | 0.0031 | - | - |
| R2 | 0.08 | - | 0.20 | 0.0031 | - | 0.0079 |
| S | 0.20 | - | - | 0.0079 | - | - |
| aaa(1) | 0.20 |  |  | 0.0079 |  |  |
| bbb(1) | 0.20 |  |  | 0.0079 |  |  |
| ccc(1) | 0.08 |  |  | 0.0031 |  |  |
| ddd(1) | 0.08 |  |  | 0.0031 |  |  |

Package information STM32F427xx STM32F429xx
Table 111. LQFP100 - Mechanical data
millimeters inches(14)
Symbol
Min Typ Max Min Typ Max
A - 1.50 1.60 - 0.0590 0.0630
A1(12) 0.05 - 0.15 0.0019 - 0.0059
A2 1.35 1.40 1.45 0.0531 0.0551 0.0570
b(9)(11) 0.17 0.22 0.27 0.0067 0.0087 0.0106
b1(11) 0.17 0.20 0.23 0.0067 0.0079 0.0090
c(11) 0.09 - 0.20 0.0035 - 0.0079
c1(11) 0.09 - 0.16 0.0035 - 0.0063
D(4) 16.00 BSC 0.6299 BSC
D1(2)(5) 14.00 BSC 0.5512 BSC
E(4) 16.00 BSC 0.6299 BSC
E1(2)(5) 14.00 BSC 0.5512 BSC
e 0.50 BSC 0.0197 BSC
L 0.45 0.60 0.75 0.177 0.0236 0.0295
L1(1)(11) 1.00 - 0.0394 -
N(13) 100
θ 0° 3.5° 7° 0° 3.5° 7°
θ1 0° - - 0° - -
θ2 10° 12° 14° 10° 12° 14°
θ3 10° 12° 14° 10° 12° 14°
R1 0.08 - - 0.0031 - -
R2 0.08 - 0.20 0.0031 - 0.0079
S 0.20 - - 0.0079 - -
aaa(1) 0.20 0.0079
bbb(1) 0.20 0.0079
ccc(1) 0.08 0.0031
ddd(1) 0.08 0.0031
200/240 DS9405 Rev 13

<!-- Page 201 -->


| 100 |
| --- |
|  |

STM32F427xx STM32F429xx Package information
Notes:
1. Dimensioning and tolerancing schemes conform to ASME Y14.5M-1994.
2. The Top package body size may be smaller than the bottom package size by as much
as 0.15 mm.
3. Datums A-B and D to be determined at datum plane H.
4. To be determined at seating datum plane C.
5. Dimensions D1 and E1 do not include mold flash or protrusions. Allowable mold flash
or protrusions is “0.25 mm” per side. D1 and E1 are Maximum plastic body size
dimensions including mold mismatch.
6. Details of pin 1 identifier are optional but must be located within the zone indicated.
7. All Dimensions are in millimeters.
8. No intrusion allowed inwards the leads.
9. Dimension “b” does not include dambar protrusion. Allowable dambar protrusion shall
not cause the lead width to exceed the maximum “b” dimension by more than 0.08 mm.
Dambar cannot be located on the lower radius or the foot. Minimum space between
protrusion and an adjacent lead is 0.07 mm for 0.4 mm and 0.5 mm pitch packages.
10. Exact shape of each corner is optional.
11. These dimensions apply to the flat section of the lead between 0.10 mm and 0.25 mm
from the lead tip.
12. A1 is defined as the distance from the seating plane to the lowest point on the package
body.
13. “N” is the number of terminal positions for the specified body size.
14. Values in inches are converted from mm and rounded to 4 decimal digits.
15. Drawing is not to scale.
Figure 79. LQFP100 - Footprint example
75 51
76 50
0.5
0.3
16.7 14.3
100 26
1.2
1 25
12.3
16.7
1L_LQFP100_FP_V1
1. Dimensions are expressed in millimeters.
DS9405 Rev 13 201/240
231

<!-- Page 202 -->


|  | bbb |  |
| --- | --- | --- |


|  |  |
| --- | --- |
|  |  |


|  |  |  |
| --- | --- | --- |
|  |  |  |


|  |
| --- |
|  |
|  |
|  |


|  |  |
| --- | --- |


|  |  |
| --- | --- |
|  | A1 orientation reference |


|  |  |  |  |  |  |
| --- | --- | --- | --- | --- | --- |
|  | ccc | Z | X | Y |  |
|  | ddd | Z |  |  |  |


|  | aaa |
| --- | --- |


| Symbol | millimeters |  |  | inches(1) |  |  |
| --- | --- | --- | --- | --- | --- | --- |
|  | Min | Typ | Max | Min | Typ | Max |
| A | 0.525 | 0.555 | 0.585 | 0.0207 | 0.0219 | 0.0230 |
| A1 | - | 0.175 | - | - | 0.0069 | - |
| A2 | - | 0.380 | - | - | 0.0150 | - |

Package information STM32F427xx STM32F429xx
7.3 WLCSP143 package information
Figure 80. WLCSP143 - 143-ball, 4.521x 5.547mm, 0.4mm pitch wafer level chip scale
package outline
A1 ball location
e1 bbb
F
G
Detail A
e2
e
A3
e
Bottom view A2
Bump side A
Side view
D
Bump A3
eee A1
E
b
A1 orientation ccc ZXY Seating
reference ddd Z plane
Detail A
Rotated 90°
aaa
Top view
Wafer back side
A0WE_ME_V2
1. Drawing is not to scale.
T able 112. WLCSP143 - 143-ball, 4.521x 5.547mm, 0.4mm pitch wafer level chip scale
package mechanical data
millimeters inches(1)
Symbol
Min Typ Max Min Typ Max
A 0.525 0.555 0.585 0.0207 0.0219 0.0230
A1 - 0.175 - - 0.0069 -
A2 - 0.380 - - 0.0150 -
202/240 DS9405 Rev 13

<!-- Page 203 -->


| Symbol | millimeters |  |  | inches(1) |  |  |
| --- | --- | --- | --- | --- | --- | --- |
|  | Min | Typ | Max | Min | Typ | Max |
| A3(2) | - | 0.025 | - | - | 0.0010 | - |
| b(3) | 0.220 | 0.250 | 0.280 | 0.0087 | 0.0098 | 0.0110 |
| D | 4.486 | 4.521 | 4.556 | 0.1766 | 0.1780 | 0.1794 |
| E | 5.512 | 5.547 | 5.582 | 0.2170 | 0.2184 | 0.2198 |
| e | - | 0.400 | - | - | 0.0157 | - |
| e1 | - | 4.000 | - | - | 0.1575 | - |
| e2 | - | 4.800 | - | - | 0.1890 | - |
| F | - | 0.2605 | - | - | 0.0103 | - |
| G | - | 0.3735 | - | - | 0.0147 | - |
| aaa | - | - | 0.100 | - | - | 0.0039 |
| bbb | - | - | 0.100 | - | - | 0.0039 |
| ccc | - | - | 0.100 | - | - | 0.0039 |
| ddd | - | - | 0.050 | - | - | 0.0020 |
| eee | - | - | 0.050 | - | - | 0.0020 |

STM32F427xx STM32F429xx Package information
Table 112. WLCSP143 - 143-ball, 4.521x 5.547mm, 0.4mm pitch wafer level chip scale
package mechanical data (continued)
millimeters inches(1)
Symbol
Min Typ Max Min Typ Max
A3(2) - 0.025 - - 0.0010 -
b(3) 0.220 0.250 0.280 0.0087 0.0098 0.0110
D 4.486 4.521 4.556 0.1766 0.1780 0.1794
E 5.512 5.547 5.582 0.2170 0.2184 0.2198
e - 0.400 - - 0.0157 -
e1 - 4.000 - - 0.1575 -
e2 - 4.800 - - 0.1890 -
F - 0.2605 - - 0.0103 -
G - 0.3735 - - 0.0147 -
aaa - - 0.100 - - 0.0039
bbb - - 0.100 - - 0.0039
ccc - - 0.100 - - 0.0039
ddd - - 0.050 - - 0.0020
eee - - 0.050 - - 0.0020
1. Values in inches are converted from mm and rounded to 4 decimal digits.
2. Back side coating.
3. Dimension is measured at the maximum bump diameter parallel to primary datum Z.
Figure 81. WLCSP143 - 143-ball, 4.521x 5.547mm, 0.4mm pitch wafer level chip scale
package recommended footprint
Dpad
Dsm
A0WE_FP_V1
DS9405 Rev 13 203/240
231

<!-- Page 204 -->


| Dimension | Recommended values |
| --- | --- |
| Pitch | 0.4 |
| Dpad | 0.225mm |
| Dsm | 0.290mm typ. (depends on the soldermask registration tolerance) |
| Stencil opening | 0.250mm |
| Stencil thickness | 0.100mm |


|  |  |
| --- | --- |


|  |  |  |  |
| --- | --- | --- | --- |
| Y | WW |  |  |
|  |  |  |  |

Package information STM32F427xx STM32F429xx
Table 113. WLCSP143 recommended PCB design rules
Dimension Recommended values
Pitch 0.4
Dpad 0.225mm
0.290mm typ. (depends on the soldermask
Dsm
registration tolerance)
Stencil opening 0.250mm
Stencil thickness 0.100mm
7.3.1 Device marking for WLCSP143
The following figure gives an example of topside marking orientation versus ball A 1
identifier location.
Other optional marking or inset/upset marks, which depend on assembly location, are not
indicated below.
Figure 82. WLCSP143 marking example (package top view)
ball A1
Product
identification(1)
Date code = Year+Week
Y WW Revision code
MSv37234V3
204/240 DS9405 Rev 13

<!-- Page 205 -->


|  |
| --- |
| (6) D 1/4 E 1/4 |


|  | aaa | C | A-B | D |
| --- | --- | --- | --- | --- |
|  |  |  |  |  |


|  | bbb | HA-B | D |
| --- | --- | --- | --- |
|  |  |  |  |


| A |  |  |  |
| --- | --- | --- | --- |
|  |  |  |  |


|  |  |  |
| --- | --- | --- |
|  |  |  |
| E 1/4 D 1/4 (6) |  |  |
|  | A (Section A-A) |  |

STM32F427xx STM32F429xx Package information
7.4 LQFP144 package information (1A)
This LQFP is a 144-pin, 20 x 20 mm low-profile quad flat package.
Note: See list of notes in the notes section.
Figure 83. LQFP144 - Outline(15)
BOTTOM VIEW
2 1
(2)
R1
R2
H
N
B-B
O
CTI
(6)
BSE
GAUGE PLANE
D 1/4
S
B
L
3
E 1/4 (L1)
aaaCA-BD SECTION A-A
bbbHA-BD4x
(N-4)x e
C
A
0.05 A2 A1 (12) b ddd CA-BD cccC
D (4)
D1 (2)(5)
(10) (3) D N (4) b
WITH PLATING
1
2
3 E 1/4
c c1
D 1/4 (6)
(3) A B (3) ( ( 2 5 ) )
E1 E b1
BASE METAL
SECTION B-B
A A
(Section A-A)
TOP VIEW
1A_LQFP144_ME_V2
DS9405 Rev 13 205/240
231
52.0
(1)(11)
4x N/4 TIPS
(9)(11)
(11) (11)
(11)

<!-- Page 206 -->


| Symbol | millimeters |  |  | inches(14) |  |  |
| --- | --- | --- | --- | --- | --- | --- |
|  | Min | Typ | Max | Min | Typ | Max |
| A | - | - | 1.60 | - | - | 0.0630 |
| A1(12) | 0.05 | - | 0.15 | 0.0020 | - | 0.0059 |
| A2 | 1.35 | 1.40 | 1.45 | 0.0531 | 0.0551 | 0.0571 |
| b(9)(11) | 0.17 | 0.22 | 0.27 | 0.0067 | 0.0087 | 0.0106 |
| b1(11) | 0.17 | 0.20 | 0.23 | 0.0067 | 0.0079 | 0.0090 |
| c(11) | 0.09 | - | 0.20 | 0.0035 | - | 0.0079 |
| c1(11) | 0.09 | - | 0.16 | 0.0035 | - | 0.0063 |
| D(4) | 22.00 BSC |  |  | 0.8661 BSC |  |  |
| D1(2)(5) | 20.00 BSC |  |  | 0.7874 BSC |  |  |
| E(4) | 22.00 BSC |  |  | 0.8661 BSC |  |  |
| E1(2)(5) | 20.00 BSC |  |  | 0.7874 BSC |  |  |
| e | 0.50 BSC |  |  | 0.0197 BSC |  |  |
| L | 0.45 | 0.60 | 0.75 | 0.0177 | 0.0236 | 0.0295 |
| L1 | 1.00 REF |  |  | 0.0394 REF |  |  |
| N(13) | 144 |  |  |  |  |  |
| θ | 0° | 3.5° | 7° | 0° | 3.5° | 7° |
| θ1 | 0° | - | - | 0° | - | - |
| θ2 | 10° | 12° | 14° | 10° | 12° | 14° |
| θ3 | 10° | 12° | 14° | 10° | 12° | 14° |
| R1 | 0.08 | - | - | 0.0031 | - | - |
| R2 | 0.08 | - | 0.20 | 0.0031 | - | 0.0079 |
| S | 0.20 | - | - | 0.0079 | - | - |
| aaa | 0.20 |  |  | 0.0079 |  |  |
| bbb | 0.20 |  |  | 0.0079 |  |  |
| ccc | 0.08 |  |  | 0.0031 |  |  |
| ddd | 0.08 |  |  | 0.0031 |  |  |

Package information STM32F427xx STM32F429xx
Table 114. LQFP144 - Mechanical data
millimeters inches(14)
Symbol
Min Typ Max Min Typ Max
A - - 1.60 - - 0.0630
A1(12) 0.05 - 0.15 0.0020 - 0.0059
A2 1.35 1.40 1.45 0.0531 0.0551 0.0571
b(9)(11) 0.17 0.22 0.27 0.0067 0.0087 0.0106
b1(11) 0.17 0.20 0.23 0.0067 0.0079 0.0090
c(11) 0.09 - 0.20 0.0035 - 0.0079
c1(11) 0.09 - 0.16 0.0035 - 0.0063
D(4) 22.00 BSC 0.8661 BSC
D1(2)(5) 20.00 BSC 0.7874 BSC
E(4) 22.00 BSC 0.8661 BSC
E1(2)(5) 20.00 BSC 0.7874 BSC
e 0.50 BSC 0.0197 BSC
L 0.45 0.60 0.75 0.0177 0.0236 0.0295
L1 1.00 REF 0.0394 REF
N(13) 144
θ 0° 3.5° 7° 0° 3.5° 7°
θ1 0° - - 0° - -
θ2 10° 12° 14° 10° 12° 14°
θ3 10° 12° 14° 10° 12° 14°
R1 0.08 - - 0.0031 - -
R2 0.08 - 0.20 0.0031 - 0.0079
S 0.20 - - 0.0079 - -
aaa 0.20 0.0079
bbb 0.20 0.0079
ccc 0.08 0.0031
ddd 0.08 0.0031
206/240 DS9405 Rev 13

<!-- Page 207 -->

STM32F427xx STM32F429xx Package information
Notes:
1. Dimensioning and tolerancing schemes conform to ASME Y14.5M-1994.
2. The Top package body size may be smaller than the bottom package size by as much
as 0.15 mm.
3. Datums A-B and D to be determined at datum plane H.
4. To be determined at seating datum plane C.
5. Dimensions D1 and E1 do not include mold flash or protrusions. Allowable mold flash
or protrusions is “0.25 mm” per side. D1 and E1 are Maximum plastic body size
dimensions including mold mismatch.
6. Details of pin 1 identifier are optional but must be located within the zone indicated.
7. All Dimensions are in millimeters.
8. No intrusion allowed inwards the leads.
9. Dimension “b” does not include dambar protrusion. Allowable dambar protrusion shall
not cause the lead width to exceed the maximum “b” dimension by more than 0.08 mm.
Dambar cannot be located on the lower radius or the foot. Minimum space between
protrusion and an adjacent lead is 0.07 mm for 0.4 mm and 0.5 mm pitch packages.
10. Exact shape of each corner is optional.
11. These dimensions apply to the flat section of the lead between 0.10 mm and 0.25 mm
from the lead tip.
12. A1 is defined as the distance from the seating plane to the lowest point on the package
body.
13. “N” is the number of terminal positions for the specified body size.
14. Values in inches are converted from mm and rounded to 4 decimal digits.
15. Drawing is not to scale.
DS9405 Rev 13 207/240
231

<!-- Page 208 -->


|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


|  |
| --- |
|  |
|  |
|  |
|  |
|  |
|  |
|  |
|  |
|  |
|  |
|  |
|  |
|  |
|  |
|  |
|  |
|  |
|  |
|  |
|  |
|  |
|  |
|  |
|  |
|  |
|  |
|  |
|  |
|  |
|  |
|  |
|  |


|  |
| --- |
|  |
|  |
|  |
|  |
|  |
|  |
|  |
|  |
|  |
|  |
|  |
|  |
|  |
|  |
|  |
|  |
|  |
|  |
|  |
|  |
|  |
|  |
|  |
|  |
|  |
|  |
|  |
|  |
|  |
|  |
|  |
|  |


|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |

Package information STM32F427xx STM32F429xx
Figure 84. LQFP144 - Footprint example
1.35
108 73
109 0.35 72
0.50
19.90 17.85
22.60
144 37
1 36
19.90
22.60
1A_LQFP144_FP
1. Dimensions are expressed in millimeters.
208/240 DS9405 Rev 13

<!-- Page 209 -->

STM32F427xx STM32F429xx Package information
7.5 LQFP176 package information (1T)
This LQFP is a 176-pin, 24x24mm, 0.5mm pitch, low profile quad flat package.
Note: See list of notes in the notes section.
Figure 85. LQFP176 - Outline(15)
(cid:537)2 (cid:537)1
(2) R1
H R2
B(See SECTION B-B)
(6) GAUGE PLANE
0.25
D1/4
S B (cid:537)
L
E1/4 (cid:537)(cid:22)
4x N/4 TIPS 4x (L1)
(1)(11)
bbb HA-BD
aaaCA-BD
BOTTOM VIEW SECTION A-A
A2 0.05 (N-4) x e (cid:11)(cid:20)(cid:22)(cid:12)
C
A
A1(12) b ddd CA-BD cccC
SIDE VIEW
D (4)
(2)(5) D1
(10) N D (cid:11)(cid:22)(cid:12) (4) b (9)(11) WITH PLATING
E1/4
(11)c c1(11)
D1/4 (6) (cid:11)(cid:22)(cid:12) (5)
A B (2)
E1 E b1 BASE METAL
(11)
SECTION A-A
A A SECTION B-B
TOP VIEW 1T_LQFP176_ME_V2
DS9405 Rev 13 209/240
231

<!-- Page 210 -->


| Symbol | millimeters |  |  | inches(14) |  |  |
| --- | --- | --- | --- | --- | --- | --- |
|  | Min | Typ | Max | Min | Typ | Max |
| A | - | - | 1.600 | - | - | 0.0630 |
| A1(12) | 0.050 | - | 0.150 | 0.0020 | - | 0.0059 |
| A2 | 1.350 | 1.400 | 1.450 | 0.0531 | 0.0551 | 0.0571 |
| b(9)(11) | 0.170 | 0.220 | 0.270 | 0.0067 | 0.0087 | 0.0106 |
| b1(11) | 0.170 | 0.200 | 0.230 | 0.0067 | 0.0079 | 0.0091 |
| c(11) | 0.090 | - | 0.200 | 0.0035 | - | 0.0079 |
| c1(11) | 0.090 | - | 0.160 | 0.0035 | - | 0.063 |
| D(4) | 26.000 |  |  | 1.0236 |  |  |
| D1(2)(5) | 24.000 |  |  | 0.9449 |  |  |
| E(4) | 26.000 |  |  | 0.0197 |  |  |
| E1(2)(5) | 24.000 |  |  | 0.9449 |  |  |
| e | 0.500 |  |  | 0.1970 |  |  |
| L | 0.450 | 0.600 | 0.750 | 0.0177 | 0.0236 | 0.0295 |
| L1(1)(11) | 1 |  |  | 0.0394 REF |  |  |
| N(13) | 176 |  |  |  |  |  |
| θ | 0° | 3.5° | 7° | 0° | 3.5° | 7° |
| θ1 | 0° | - | - | 0° | - | - |
| θ2 | 10° | 12° | 14° | 10° | 12° | 14° |
| θ3 | 10° | 12° | 14° | 10° | 12° | 14° |
| R1 | 0.080 | - | - | 0.0031 | - | - |
| R2 | 0.080 | - | 0.200 | 0.0031 | - | 0.0079 |
| S | 0.200 | - | - | 0.0079 | - | - |
| aaa(1) | 0.200 |  |  | 0.0079 |  |  |
| bbb(1) | 0.200 |  |  | 0.0079 |  |  |
| ccc(1) | 0.080 |  |  | 0.0031 |  |  |
| ddd(1) | 0.080 |  |  | 0.0031 |  |  |

Package information STM32F427xx STM32F429xx
Table 115. LQFP176 - Mechanical data
millimeters inches(14)
Symbol
Min Typ Max Min Typ Max
A - - 1.600 - - 0.0630
A1(12) 0.050 - 0.150 0.0020 - 0.0059
A2 1.350 1.400 1.450 0.0531 0.0551 0.0571
b(9)(11) 0.170 0.220 0.270 0.0067 0.0087 0.0106
b1(11) 0.170 0.200 0.230 0.0067 0.0079 0.0091
c(11) 0.090 - 0.200 0.0035 - 0.0079
c1(11) 0.090 - 0.160 0.0035 - 0.063
D(4) 26.000 1.0236
D1(2)(5) 24.000 0.9449
E(4) 26.000 0.0197
E1(2)(5) 24.000 0.9449
e 0.500 0.1970
L 0.450 0.600 0.750 0.0177 0.0236 0.0295
L1(1)(11) 1 0.0394 REF
N(13) 176
θ 0° 3.5° 7° 0° 3.5° 7°
θ1 0° - - 0° - -
θ2 10° 12° 14° 10° 12° 14°
θ3 10° 12° 14° 10° 12° 14°
R1 0.080 - - 0.0031 - -
R2 0.080 - 0.200 0.0031 - 0.0079
S 0.200 - - 0.0079 - -
aaa(1) 0.200 0.0079
bbb(1) 0.200 0.0079
ccc(1) 0.080 0.0031
ddd(1) 0.080 0.0031
210/240 DS9405 Rev 13

<!-- Page 211 -->

STM32F427xx STM32F429xx Package information
Notes:
1. Dimensioning and tolerancing schemes conform to ASME Y14.5M-1994.
2. The Top package body size may be smaller than the bottom package size by as much
as 0.15 mm.
3. Datums A-B and D to be determined at datum plane H.
4. To be determined at seating datum plane C.
5. Dimensions D1 and E1 do not include mold flash or protrusions. Allowable mold flash
or protrusions is “0.25 mm” per side. D1 and E1 are Maximum plastic body size
dimensions including mold mismatch.
6. Details of pin 1 identifier are optional but must be located within the zone indicated.
7. All Dimensions are in millimeters.
8. No intrusion allowed inwards the leads.
9. Dimension “b” does not include dambar protrusion. Allowable dambar protrusion shall
not cause the lead width to exceed the maximum “b” dimension by more than 0.08 mm.
Dambar cannot be located on the lower radius or the foot. Minimum space between
protrusion and an adjacent lead is 0.07 mm for 0.4 mm and 0.5 mm pitch packages.
10. Exact shape of each corner is optional.
11. These dimensions apply to the flat section of the lead between 0.10 mm and 0.25 mm
from the lead tip.
12. A1 is defined as the distance from the seating plane to the lowest point on the package
body.
13. “N” is the number of terminal positions for the specified body size.
14. Values in inches are converted from mm and rounded to 4 decimal digits.
15. Drawing is not to scale.
DS9405 Rev 13 211/240
231

<!-- Page 212 -->


|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
|  | 176 1 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 133 0.5 132 0.3 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  | 44 45 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 89 88 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |

Package information STM32F427xx STM32F429xx
Figure 86. LQFP176 - Footprint example
1.2
176 133
1 0.5 132
0.3
44 89
45 88
1.2
21.8
26.7
1T_FP_V1
1. Dimensions are expressed in millimeters.
212/240 DS9405 Rev 13
7.62 8.12

<!-- Page 213 -->


|  | aaa | C | A-B |
| --- | --- | --- | --- |


|  | bbbH | A-B | D |
| --- | --- | --- | --- |


|  |  |
| --- | --- |
|  |  |

STM32F427xx STM32F429xx Package information
7.6 LQFP208 package information
This LQFP is a 208-pin, 28 x 28 mm low-profile quad flat package.
Note: See list of notes in the notes section.
Figure 87. LQFP208 - Outline(15)
2 1
(2) R1
H
R2
N
B-B
O
CTI
SE
B GAUGE PLANE
S
B
3 L
(L1)
SECTION A-A
b WITH
PLATING
c c1
b1
BASE METAL
SECTION B-B
UH_LQFP208_ME_V2
DS9405 Rev 13 213/240
231
52.0
BOTTOM VIEW
D 1/4 (6)
(1)(11)
E 1/4
4x N/4 TIPS
aaaCA-BD bbbHA-BD 4x
(N – 4)xe (13)
C
A
A2
0.05 A1(12) b ddd CA-BD cccC
D (4)
(2)(5) D1
D (3)
(10) N
(4) (9)(11)
1
2
3
E 1/4
(11) (11)
D 1/4 (6)
(3)A B (3) (11)
E1 E
(2)
(5)
A A
(Section A-A)
TOP VIEW

<!-- Page 214 -->


| Symbol | millimeters |  |  | inches(15) |  |  |
| --- | --- | --- | --- | --- | --- | --- |
|  | Min | Typ | Max | Min | Typ | Max |
| A | - | - | 1.60 | - | - | 0.0630 |
| A1(12) | 0.05 | - | 0.15 | 0.0020 | - | 0.0059 |
| A2 | 1.35 | 1.40 | 1.45 | 0.0531 | 0.0551 | 0.0571 |
| b(9)(11) | 0.17 | 0.22 | 0.27 | 0.0067 | 0.0087 | 0.0106 |
| b1(11) | 0.17 | 0.20 | 0.23 | 0.0067 | 0.0079 | 0.0091 |
| c(11) | 0.09 | - | 0.20 | 0.0035 | - | 0.0079 |
| c1(11) | 0.09 | - | 0.16 | 0.0035 | - | 0.0063 |
| D(4) | 30.00 BSC |  |  | 1.1732 BSC |  |  |
| D1(2)(5) | 28.00 BSC |  |  | 1.0945 BSC |  |  |
| E(4) | 30.00 BSC |  |  | 1.1732 BSC |  |  |
| E1(2)(5) | 28.00 BSC |  |  | 1.0945 BSC |  |  |
| e | 0.50 BSC |  |  | 0.0197 BSC |  |  |
| L | 0.45 | 0.60 | 0.75 | 0.0177 | 0.0236 | 0.0295 |
| L1 | 1.00 REF |  |  | 0.0394 REF |  |  |
| N(13) | 208 |  |  |  |  |  |
| θ | 0° | 3.5° | 7° | 0° | 3.5° | 7° |
| θ1 | 0° | - | - | 0° | - | - |
| θ2 | 10° | 12° | 14° | 10° | 12° | 14° |
| θ3 | 10° | 12° | 14° | 10° | 12° | 14° |
| R1 | 0.08 | - | - | 0.0031 | - | - |
| R2 | 0.08 | - | 0.20 | 0.0031 | - | 0.0079 |
| S | 0.20 | - | - | 0.0079 | - | - |
| aaa(1)(7) | 0.20 |  |  | 0.0079 |  |  |
| bbb(1)(7) | 0.20 |  |  | 0.0079 |  |  |
| ccc(1)(7) | 0.08 |  |  | 0.0031 |  |  |
| ddd(1)(7) | 0.08 |  |  | 0.0031 |  |  |

Package information STM32F427xx STM32F429xx
Table 116. LQFP208 - Mechanical data
millimeters inches(15)
Symbol
Min Typ Max Min Typ Max
A - - 1.60 - - 0.0630
A1(12) 0.05 - 0.15 0.0020 - 0.0059
A2 1.35 1.40 1.45 0.0531 0.0551 0.0571
b(9)(11) 0.17 0.22 0.27 0.0067 0.0087 0.0106
b1(11) 0.17 0.20 0.23 0.0067 0.0079 0.0091
c(11) 0.09 - 0.20 0.0035 - 0.0079
c1(11) 0.09 - 0.16 0.0035 - 0.0063
D(4) 30.00 BSC 1.1732 BSC
D1(2)(5) 28.00 BSC 1.0945 BSC
E(4) 30.00 BSC 1.1732 BSC
E1(2)(5) 28.00 BSC 1.0945 BSC
e 0.50 BSC 0.0197 BSC
L 0.45 0.60 0.75 0.0177 0.0236 0.0295
L1 1.00 REF 0.0394 REF
N(13) 208
θ 0° 3.5° 7° 0° 3.5° 7°
θ1 0° - - 0° - -
θ2 10° 12° 14° 10° 12° 14°
θ3 10° 12° 14° 10° 12° 14°
R1 0.08 - - 0.0031 - -
R2 0.08 - 0.20 0.0031 - 0.0079
S 0.20 - - 0.0079 - -
aaa(1)(7) 0.20 0.0079
bbb(1)(7) 0.20 0.0079
ccc(1)(7) 0.08 0.0031
ddd(1)(7) 0.08 0.0031
214/240 DS9405 Rev 13

<!-- Page 215 -->


| 3. |
| --- |
| 82 |
|  |


|  |  |
| --- | --- |

STM32F427xx STM32F429xx Package information
Notes:
1. Dimensioning and tolerancing schemes conform to ASME Y14.5M-1994.
2. The Top package body size may be smaller than the bottom package size by as much
as 0.15 mm.
3. Datums A-B and D to be determined at datum plane H.
4. To be determined at seating datum plane C.
5. Dimensions D1 and E1 do not include mold flash or protrusions. Allowable mold flash
or protrusions is “0.25 mm” per side. D1 and E1 are Maximum plastic body size
dimensions including mold mismatch.
6. Details of pin 1 identifier are optional but must be located within the zone indicated.
7. All Dimensions are in millimeters.
8. No intrusion allowed inwards the leads.
9. Dimension “b” does not include dambar protrusion. Allowable dambar protrusion shall
not cause the lead width to exceed the maximum “b” dimension by more than 0.08 mm.
Dambar cannot be located on the lower radius or the foot. Minimum space between
protrusion and an adjacent lead is 0.07 mm for 0.4 mm and 0.5 mm pitch packages.
10. Exact shape of each corner is optional.
11. These dimensions apply to the flat section of the lead between 0.10 mm and 0.25 mm
from the lead tip.
12. A1 is defined as the distance from the seating plane to the lowest point on the package
body.
13. “N” is the number of terminal positions for the specified body size.
14. Values in inches are converted from mm and rounded to 4 decimal digits.
15. Drawing is not to scale.
Figure 88. LQFP208 - footprint example
52 105
53 104 1.2
25.8
30.7
UH_LQFP208_FP_V3
1. Dimensions are expressed in millimeters.
DS9405 Rev 13 215/240
231
7.03 3.82
208 157
1 156
0.50 1.25
0.30

<!-- Page 216 -->


| E1 e SE N M L K e J H G D1 SD F E D |
| --- |
| C B A 1 2 3 4 5 67 8 910111213 A1 ball pad Øb (169 balls) corner Ø Ø e ff e f e M M C C A B BOTTOM VIEW Detail A A A3 ccc C Mold resin A2 Seating plane SIDE VIEW C 2 C ddd C Substrate Solder balls A1 A5 DETAIL A B E A A1 ball pad 3 corner (DATUM A) D (DATUM B) aaaC (4x) TOP VIEW A0YV_UFBGA169_ME_V2 |


| SE |  |  |  |
| --- | --- | --- | --- |
|  |  |  |  |
|  |  |  | e |
|  |  |  |  |
|  |  |  |  |


|  | ØeeeM | C | A B |
| --- | --- | --- | --- |
|  | Øfff M | C |  |


|  | ccc | C |
| --- | --- | --- |


|  | (DATUM A) |
| --- | --- |
|  | (DATUM B) |

Package information STM32F427xx STM32F429xx
7.7 UFBGA169 package information (A0YV)
This UFBGA is a 169-ball, 7 x 7 mm, 0.50 mm pitch, ultra fine pitch ball grid array package.
Figure 89. UFBGA169 - Outline
E1
e SE
N
M
L
K e
J
H
G D1
SD F
E
D
C
B
A
1 2 3 4 5 67 8 910111213
A1 ball pad Øb (169 balls)
corner Ø Ø e ff e f e M M C C A B
BOTTOM VIEW
Detail A
A A3 ccc C Mold resin A2
Seating plane
SIDE VIEW C 2 C ddd C Substrate Solder balls A1 A5
DETAIL A
B E
A
A1 ball pad
3 corner (DATUM A)
D
(DATUM B)
aaaC
(4x)
TOP VIEW A0YV_UFBGA169_ME_V2
1. Drawing is not to scale.
2. Primary datum C is defined by the plane established by the contact points of three or more solder balls that
support the device when it is placed on top of a planar surface.
3. The terminal (ball) A1 corner must be identified on the top surface of the package by using a corner
chamfer, ink or metallized markings, or other feature of package body or integral heat slug. A distinguish
feature is allowable on the bottom surface of the package to identify the terminal A1 corner. Exact shape of
each corner is optional.
216/240 DS9405 Rev 13

<!-- Page 217 -->


| Symbol | millimeters |  |  | inches(1) |  |  |
| --- | --- | --- | --- | --- | --- | --- |
|  | Min. | Typ. | Max. | Min. | Typ. | Max. |
| A(2) | - | - | 0.60 | - | - | 0.0236 |
| A1(3) | 0.05 | - | - | 0.0020 | - | - |
| A2 | - | 0.43 | - | - | 0.0169 | - |
| b(4) | 0.23 | 0.28 | 0.33 | 0.0091 | 0.0110 | 0.0130 |
| D(5) | 7.00 BSC |  |  | 0.2756 BSC |  |  |
| D1(5) | 6.00 BSC |  |  | 0.2362 BSC |  |  |
| E(5) | 7.00 BSC |  |  | 0.2756 BSC |  |  |
| E1(5) | 6.00 BSC |  |  | 0.2362 BSC |  |  |
| e(5)(6) | 0.50 BSC |  |  | 0.0197 BSC |  |  |
| N(7) | 169 |  |  |  |  |  |
| SD(5)(8) | 0.50 BSC |  |  | 0.0197 BSC |  |  |
| SE(5)(8) | 0.50 BSC |  |  | 0.0197 BSC |  |  |
| aaa(9) | 0.15 |  |  | 0.0059 |  |  |
| ccc(9) | 0.20 |  |  | 0.0079 |  |  |
| ddd(9) | 0.08 |  |  | 0.0031 |  |  |
| eee(9) | 0.15 |  |  | 0.0059 |  |  |
| fff(9) | 0.05 |  |  | 0.0020 |  |  |

STM32F427xx STM32F429xx Package information
Table 117. UFBGA169 - Mechanical data
millimeters inches(1)
Symbol
Min. Typ. Max. Min. Typ. Max.
A(2) - - 0.60 - - 0.0236
A1(3) 0.05 - - 0.0020 - -
A2 - 0.43 - - 0.0169 -
b(4) 0.23 0.28 0.33 0.0091 0.0110 0.0130
D(5) 7.00 BSC 0.2756 BSC
D1(5) 6.00 BSC 0.2362 BSC
E(5) 7.00 BSC 0.2756 BSC
E1(5) 6.00 BSC 0.2362 BSC
e(5)(6) 0.50 BSC 0.0197 BSC
N(7) 169
SD(5)(8) 0.50 BSC 0.0197 BSC
SE(5)(8) 0.50 BSC 0.0197 BSC
aaa(9) 0.15 0.0059
ccc(9) 0.20 0.0079
ddd(9) 0.08 0.0031
eee(9) 0.15 0.0059
fff(9) 0.05 0.0020
1. Values in inches are converted from mm and rounded to 4 decimal digits.
2. The profile height, A, is the distance from the seating plane to the highest point on the package. It is
measured perpendicular to the seating plane.
3. A1 is defined as the distance from the seating plane to the lowest point on the package body.
4. Dimension b is measured at the maximum diameter of the terminal (ball) in a plane parallel to primary
datum C.
5. BSC stands for BASIC dimensions. It corresponds to the nominal value and has no tolerance. For
tolerances refer to form and position table.
6. e represents the solder ball grid pitch.
7. N represents the total number of balls on the BGA.
8. Basic dimensions SD and SE are defined with respect to datums A and B. It defines the position of the
centre ball(s) in the outer row or column of a fully populated matrix.
9. Tolerance of form and position drawing
DS9405 Rev 13 217/240
231

<!-- Page 218 -->


| Dimension | Values |
| --- | --- |
| Pitch | 0.5mm |
| Dpad | 0.27mm |
| Dsm | 0.35mm typ. (depends on the soldermask registration tolerance) |
| Solder paste | 0.27mm aperture diameter. |

Package information STM32F427xx STM32F429xx
Figure 90. UFBGA169 - Footprint example
Dpad
Dsm
BGA_WLCSP_FT_V1
Table 118. UFBGA169 - Example of PCB design rules (0.5mm pitch BGA)
Dimension Values
Pitch 0.5mm
Dpad 0.27mm
0.35mm typ. (depends on the soldermask
Dsm
registration tolerance)
Solder paste 0.27mm aperture diameter.
Note: Non-solder mask defined (NSMD) pads are recommended.
Note: 4 to 6 mils solder paste screen printing process.
218/240 DS9405 Rev 13

<!-- Page 219 -->


|  |  |  |  |  |  |  | ddd | C |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
|  |  |  |  |  |  |  |  |  |


|  |
| --- |
|  |


|  |  |  |
| --- | --- | --- |
|  |  |  |


|  |  |
| --- | --- |


|  | ØeeeM | C | A | B |
| --- | --- | --- | --- | --- |
|  | Øfff M | C |  |  |


| Symbol | millimeters |  |  | inches(1) |  |  |
| --- | --- | --- | --- | --- | --- | --- |
|  | Min. | Typ. | Max. | Min. | Typ. | Max. |
| A | - | - | 0.600 | - | - | 0.0236 |
| A1 | 0.050 | 0.080 | 0.110 | 0.0020 | 0.0031 | 0.0043 |
| A2 | - | 0.450 | - | - | 0.0177 | - |
| A3 | - | 0.130 | - | - | 0.0051 | - |
| A4 | - | 0.320 | - | - | 0.0126 | - |
| b | 0.240 | 0.290 | 0.340 | 0.0094 | 0.0114 | 0.0134 |
| D | 9.850 | 10.000 | 10.150 | 0.3878 | 0.3937 | 0.3996 |
| D1 | - | 9.100 | - | - | 0.3583 | - |
| E | 9.850 | 10.000 | 10.150 | 0.3878 | 0.3937 | 0.3996 |
| E1 | - | 9.100 | - | - | 0.3583 | - |
| e | - | 0.650 | - | - | 0.0256 | - |
| F | - | 0.450 | - | - | 0.0177 | - |
| ddd | - | - | 0.080 | - | - | 0.0031 |

STM32F427xx STM32F429xx Package information
7.8 UFBGA(176+25) package information (A0E7)
This UFBGA is a 176+25-ball, 10 x 10mm, 0.65mm pitch, ultra fine pitch ball grid array
package.
Figure 91. UFBGA(176+25) - Outline
Seating plane
C A4
dddC
A
A2 A3 b A1
A1 ball A
A1 ball index
E
identifier area
E1
e F
A
F
D
D1
e
B
R
15 1
Øb (176 + 25 balls)
BOTTOM VIEW TOP VIEW
ØeeeM C A B
Øfff M C
A0E7_ME_V10
1. Drawing is not to scale.
Table 119. UFBGA(176+25) - Mechanical data
millimeters inches(1)
Symbol
Min. Typ. Max. Min. Typ. Max.
A - - 0.600 - - 0.0236
A1 0.050 0.080 0.110 0.0020 0.0031 0.0043
A2 - 0.450 - - 0.0177 -
A3 - 0.130 - - 0.0051 -
A4 - 0.320 - - 0.0126 -
b 0.240 0.290 0.340 0.0094 0.0114 0.0134
D 9.850 10.000 10.150 0.3878 0.3937 0.3996
D1 - 9.100 - - 0.3583 -
E 9.850 10.000 10.150 0.3878 0.3937 0.3996
E1 - 9.100 - - 0.3583 -
e - 0.650 - - 0.0256 -
F - 0.450 - - 0.0177 -
ddd - - 0.080 - - 0.0031
DS9405 Rev 13 219/240
231

<!-- Page 220 -->


| Symbol | millimeters |  |  | inches(1) |  |  |
| --- | --- | --- | --- | --- | --- | --- |
|  | Min. | Typ. | Max. | Min. | Typ. | Max. |
| eee | - | - | 0.150 | - | - | 0.0059 |
| fff | - | - | 0.050 | - | - | 0.0020 |


| Dimension | Values |
| --- | --- |
| Pitch | 0.65mm |
| Dpad | 0.300mm |
| Dsm | 0.400mm typ. (depends on the soldermask registration tolerance) |
| Stencil opening | 0.300mm |
| Stencil thickness | Between 0.100mm and 0.125mm |
| Pad trace width | 0.100mm |

Package information STM32F427xx STM32F429xx
Table 119. UFBGA(176+25) - Mechanical data (continued)
millimeters inches(1)
Symbol
Min. Typ. Max. Min. Typ. Max.
eee - - 0.150 - - 0.0059
fff - - 0.050 - - 0.0020
1. Values in inches are converted from mm and rounded to 4 decimal digits.
Figure 92. UFBGA(176+25) - Footprint example
Dpad
Dsm
BGA_WLCSP_FT_V1
Table 120. UFBGA(176+25) - Example of PCB design rules (0.65 mm pitch BGA)
Dimension Values
Pitch 0.65mm
Dpad 0.300mm
0.400mm typ. (depends on the soldermask
Dsm
registration tolerance)
Stencil opening 0.300mm
Stencil thickness Between 0.100mm and 0.125mm
Pad trace width 0.100mm
220/240 DS9405 Rev 13

<!-- Page 221 -->


|  | ddd | Z |
| --- | --- | --- |


|  |  |  |
| --- | --- | --- |


|  |
| --- |
|  |


|  |  |  |
| --- | --- | --- |
|  |  |  |
|  |  |  |
|  |  |  |


|  | ØeeeM | Z | Y | X |
| --- | --- | --- | --- | --- |
|  | Øfff M | Z |  |  |

STM32F427xx STM32F429xx Package information
7.9 TFBGA216 package information (A0L2)
This TFBGA is a 216-ball, 13x13mm, 0.8mm pitch, fine pitch ball grid array package.
Figure 93. TFBGA216 - Outline
Z Seating plane
ddd Z
A2
A1 A
D1 A1 ball A1 ball X
identifier index area D
e F
A
G
E1 E
e
Y
R
15 1
BOTTOM VIEW Øb (216 balls) TOP VIEW
ØeeeM Z Y X
Øfff M Z
A0L2_ME_V3
1. Drawing is not to scale.
2. • The terminal A1 corner must be identified on the top surface by using a corner chamfer, ink or metalized
markings, or other feature of package body or integral heat slug.
• A distinguishing feature is allowable on the bottom surface of the package to identify the terminal A1
corner. Exact shape of each corner is optional
DS9405 Rev 13 221/240
231

<!-- Page 222 -->


| Symbol | millimeters |  |  | inches(1) |  |  |
| --- | --- | --- | --- | --- | --- | --- |
|  | Min | Typ | Max | Min | Typ | Max |
| A | - | - | 1.200 | - | - | 0.0472 |
| A1(2) | 0.150 | - | - | 0.0059 | - | - |
| A2 | - | 0.760 | - | - | 0.0299 | - |
| b(3) | 0.350 | 0.400 | 0.450 | 0.0138 | 0.0157 | 0.0177 |
| D | 12.850 | 13.000 | 13.150 | 0.5059 | 0.5118 | 0.5177 |
| D1 | - | 11.200 | - | - | 0.4409 | - |
| E | 12.850 | 13.000 | 13.150 | 0.5059 | 0.5118 | 0.5177 |
| E1 | - | 11.200 | - | - | 0.4409 | - |
| e | - | 0.800 | - | - | 0.0315 | - |
| F | - | 0.900 | - | - | 0.0354 | - |
| G | - | 0.900 | - | - | 0.0354 | - |
| ddd | - | - | 0.100 | - | - | 0.0039 |
| eee(4) | - | - | 0.150 | - | - | 0.0059 |
| fff(5) | - | - | 0.080 | - | - | 0.0031 |

Package information STM32F427xx STM32F429xx
Table 121. TFBGA216 - Mechanical data
millimeters inches(1)
Symbol
Min Typ Max Min Typ Max
A - - 1.200 - - 0.0472
A1(2) 0.150 - - 0.0059 - -
A2 - 0.760 - - 0.0299 -
b(3) 0.350 0.400 0.450 0.0138 0.0157 0.0177
D 12.850 13.000 13.150 0.5059 0.5118 0.5177
D1 - 11.200 - - 0.4409 -
E 12.850 13.000 13.150 0.5059 0.5118 0.5177
E1 - 11.200 - - 0.4409 -
e - 0.800 - - 0.0315 -
F - 0.900 - - 0.0354 -
G - 0.900 - - 0.0354 -
ddd - - 0.100 - - 0.0039
eee(4) - - 0.150 - - 0.0059
fff(5) - - 0.080 - - 0.0031
1. Values in inches are converted from mm and rounded to four decimal digits.
2. • The terminal A1 corner must be identified on the top surface by using a corner chamfer, ink or metallized
markings, or other feature of package body or integral heat slug.
• A distinguishing feature is allowable on the bottom surface of the package to identify the terminal A1
corner. Exact shape of each corner is optional.
3. Initial ball equal 0.350 mm.
4. The tolerance of position that controls the location of the pattern of balls with respect to datums A and B.
For each ball there is a cylindrical tolerance zone eee perpendicular to datum C and located on true
position with respect to datums A and B as defined by e. The axis perpendicular to datum C of each ball
must lie within this tolerance zone.
5. The tolerance of position that controls the location of the balls within the matrix with respect to each other.
For each ball there is a cylindrical tolerance zone fff perpendicular to datum C and located on true position
as defined by e. The axis perpendicular to datum C of each ball must lie within this tolerance zone. Each
tolerance zone fff in the array is contained entirely in the respective zone eee above The axis of each ball
must lie simultaneously in both tolerance zones.
222/240 DS9405 Rev 13

<!-- Page 223 -->


| Dimension | Values |
| --- | --- |
| Pitch | 0.8mm |
| Dpad | 0.400mm |
| Dsm | 0.470mm typ. (depends on the soldermask registration tolerance) |
| Stencil opening | 0.400mm |
| Stencil thickness | Between 0.100mm and 0.125mm |
| Pad trace width | 0.120mm |

STM32F427xx STM32F429xx Package information
Figure 94. TFBGA216 - Footprint example
Dpad
Dsm
BGA_WLCSP_FT_V1
Table 122. TFBGA216 - Example of PCB design rules (0.8 mm pitch)
Dimension Values
Pitch 0.8mm
Dpad 0.400mm
0.470mm typ. (depends on the soldermask
Dsm
registration tolerance)
Stencil opening 0.400mm
Stencil thickness Between 0.100mm and 0.125mm
Pad trace width 0.120mm
DS9405 Rev 13 223/240
231

<!-- Page 224 -->


| Symbol | Parameter | Value | Unit |
| --- | --- | --- | --- |
| Θ JA | Thermal resistance junction-ambient LQFP100 - 14 × 14 mm / 0.5 mm pitch | 43 | °C/W |
|  | Thermal resistance junction-ambient WLCSP143 | 31.2 |  |
|  | Thermal resistance junction-ambient LQFP144 - 20 × 20 mm / 0.5 mm pitch | 40 |  |
|  | Thermal resistance junction-ambient LQFP176 - 24 × 24 mm / 0.5 mm pitch | 38 |  |
|  | Thermal resistance junction-ambient LQFP208 - 28 × 28 mm / 0.5 mm pitch | 19 |  |
|  | Thermal resistance junction-ambient UFBGA169 - 7 × 7mm / 0.5 mm pitch | 52 |  |
|  | Thermal resistance junction-ambient UFBGA176 - 10× 10 mm / 0.5 mm pitch | 39 |  |
|  | Thermal resistance junction-ambient TFBGA216 - 13 × 13 mm / 0.8 mm pitch | 29 |  |

Package information STM32F427xx STM32F429xx
7.10 Thermal characteristics
The maximum chip-junction temperature, T max, in degrees Celsius, may be calculated
J
using the following equation:
T max = T max + (P max x θ )
J A D JA
Where:
• T max is the maximum ambient temperature in °C,
A
• θ is the package junction-to-ambient thermal resistance, in °C/W,
JA
• P max is the sum of P max and P max (P max = P max + P max),
D INT I/O D INT I/O
• P max is the product of I andV , expressed in Watts. This is the maximum chip
INT DD DD
internal power.
P max represents the maximum power dissipation on output pins where:
I/O
P
I/O
max = ∑ (V
OL
× I
OL
) + ∑((V
DD
– V
OH
) × I
OH
),
taking into account the actual V
OL
/ I
OL
and V
OH
/ I
OH
of the I/Os at low and high level in the
application.
Table 123. Package thermal characteristics
Symbol Parameter Value Unit
Thermal resistance junction-ambient
43
LQFP100 - 14 × 14 mm / 0.5 mm pitch
Thermal resistance junction-ambient
31.2
WLCSP143
Thermal resistance junction-ambient
40
LQFP144 - 20 × 20 mm / 0.5 mm pitch
Thermal resistance junction-ambient
38
LQFP176 - 24 × 24 mm / 0.5 mm pitch
Θ °C/W
JA
Thermal resistance junction-ambient
19
LQFP208 - 28 × 28 mm / 0.5 mm pitch
Thermal resistance junction-ambient
52
UFBGA169 - 7 × 7mm / 0.5 mm pitch
Thermal resistance junction-ambient
39
UFBGA176 - 10× 10 mm / 0.5 mm pitch
Thermal resistance junction-ambient
29
TFBGA216 - 13 × 13 mm / 0.8 mm pitch
Reference document
JESD51-2 Integrated Circuits Thermal Test Method Environment Conditions - Natural
Convection (Still Air). Available from www.jedec.org.
224/240 DS9405 Rev 13

<!-- Page 225 -->

STM32F427xx STM32F429xx Ordering information
8 Ordering information
Example: STM32 F 429 V I T 6 xxx
Device family
STM32 = Arm-based 32-bit microcontroller
Product type
F = general-purpose
Device subfamily
427= STM32F427xx, USB OTG FS/HS, camera interface,
Ethernet
429= STM32F429xx, USB OTG FS/HS, camera interface,
Ethernet, LCD-TFT
Pin count
V = 100 pins
Z = 143 and 144 pins
A = 169 pins
I = 176 pins
B = 208 pins
N = 216 pins
Flash memory size
E = 512 Kbytes of Flash memory
G = 1024 Kbytes of Flash memory
I = 2048 Kbytes of Flash memory
Package
T = LQFP
H = BGA
Y = WLCSP
Temperature range
6 = Industrial temperature range, –40 to 85 °C.
7 = Industrial temperature range, –40 to 105 °C.
Options
xxx = programmed parts
TR = tape and reel
For a list of available options (speed, package, etc.) or for further information on any aspect
of this device, please contact your nearest ST sales office.
DS9405 Rev 13 225/240
231

<!-- Page 226 -->


| Operating power supply range | ADC operation | Maximum Flash memory access frequency with no wait states (f ) Flashmax | Maximum Flash memory access frequency with wait states (1)(2) | I/O operation | Possible Flash memory operations |
| --- | --- | --- | --- | --- | --- |
| V =1.7 to DD 2.1V(3) | Conversion time up to 1.2Msps | 20MHz(4) | 168MHz with 8 wait states and over-drive OFF | – No I/O compensation | 8-bit erase and program operations only |

Recommendations when using internal reset OFF STM32F427xx STM32F429xx
Appendix A Recommendations when using internal reset
OFF
When the internal reset is OFF, the following integrated features are no longer supported:
• The integrated power-on reset (POR) / power-down reset (PDR) circuitry is disabled.
• The brownout reset (BOR) circuitry must be disabled.
• The embedded programmable voltage detector (PVD) is disabled.
• V functionality is no more available and VBAT pin should be connected to V .
BAT DD
• The over-drive mode is not supported.
A.1 Operating conditions
Table 124. Limitations depending on the operating power supply range
Maximum
Flash
Operating memory Maximum Flash
Possible Flash
power ADC access memory access
I/O operation memory
supply operation frequency frequency with
operations
range with no wait wait states (1)(2)
states
(f )
Flashmax
Conversion 168MHz with 8 8-bit erase and
V =1.7 to – No I/O
DD time up to 20MHz(4) wait states and program
2.1V(3) compensation
1.2Msps over-drive OFF operations only
1. Applicable only when the code is executed from Flash memory. When the code is executed from RAM, no
wait state is required.
2. Thanks to the ART accelerator and the 128-bit Flash memory, the number of wait states given here does
not impact the execution speed from Flash memory since the ART accelerator allows to achieve a
performance equivalent to 0 wait state program execution.
3. V /V minimum value of 1.7V, with the use of an external power supply supervisor (refer to
DD DDA
Section3.17.1: Internal reset ON).
4. Prefetch is not available. Refer to AN3430 application note for details on how to adjust performance and
power.
226/240 DS9405 Rev 13

<!-- Page 227 -->

STM32F427xx STM32F429xx Application block diagrams
Appendix B Application block diagrams
B.1 USB OTG full speed (FS) interface solutions
Figure 95. USB controller configured as peripheral-only and used
in Full speed mode
VDD
5V to VDD
Volatge regulator(1)
STM32F4xx
VBUS
PA11//PB14
DP
PA12/PB15
VSS
1. External voltage regulator only needed when building a V powered device.
BUS
2. The same application can be developed using the OTG HS in FS mode to achieve enhanced performance
thanks to the large Rx/Tx FIFO and to a dedicated DMA controller.
Figure 96. USB controller configured as host-only and used in full speed mode
1. The current limiter is required only if the application has to support a V powered device. A basic power
BUS
switch can be used if 5 V are available on the application board.
2. The same application can be developed using the OTG HS in FS mode to achieve enhanced performance
thanks to the large Rx/Tx FIFO and to a dedicated DMA controller.
DS9405 Rev 13 227/240
231
rotcennoc
B-dtS
BSU
DM
OSC_IN
OSC_OUT
MS19000V5
VDD
STM32F4xx
VBUS
DP
VSS
rotcennoc
A-dtS
BSU
EN
GPIO
Current limiter
power switch(1) 5 V Pwr
Overcurrent
GPIO+IRQ
DM
PA11//PB14
OSC_IN
PA12/PB15
OSC_OUT
MS19001V4

<!-- Page 228 -->


| DM |
| --- |
| DP |

Application block diagrams STM32F427xx STM32F429xx
Figure 97. USB controller configured in dual mode and used in full speed mode
VDD
STM32F4xx
VBUS
PA9/PB13
PA11/PB14
DP PA12/PB15
VSS
1. External voltage regulator only needed when building a V powered device.
BUS
2. The current limiter is required only if the application has to support a V powered device. A basic power
BUS
switch can be used if 5V are available on the application board.
3. The ID pin is required in dual role only.
4. The same application can be developed using the OTG HS in FS mode to achieve enhanced performance
thanks to the large Rx/Tx FIFO and to a dedicated DMA controller.
228/240 DS9405 Rev 13
rotcennoc
BA-orcim
BSU
VDD
5 V to VDD
voltage regulator(1)
EN
GPIO
Current limiter 5 V Pwr
Overcurrent power switch(2)
GPIO+IRQ
DM
OSC_IN
PA10/PB12
ID(3)
OSC_OUT
MS19002V3

<!-- Page 229 -->


| High speed OTG PHY XI | DM |
| --- | --- |
|  | ID(2) |
|  | VBUS |
|  | VSS |


|  |  |  | ULPI_D[7:0] |
| --- | --- | --- | --- |
|  |  |  | ULPI_DIR |
|  |  |  | ULPI_STP |
|  |  |  | ULPI_NXT |
|  |  |  | XT1 24 or 26 MHz XT(1) MCO1 or MCO2 |
|  |  |  |  |
|  |  |  |  |
|  |  |  |  |
|  |  |  |  |

STM32F427xx STM32F429xx Application block diagrams
B.2 USB OTG high speed (HS) interface solutions
Figure 98. USB controller configured as peripheral, host, or dual-mode
and used in high speed mode
STM32F4xx
DP
FS PHY not connected
USB HS DM
OTG Ctrl
DP
ULPI_CLK
DM
ULPI_D[7:0]
ID(2)
USB
ULPI_DIR
ULPI VBUS connector
ULPI_STP
VSS
ULPI_NXT
High speed
OTG PHY
XT1
PLL
24 or 26 MHz XT(1)
MCO1 or MCO2
XI
MS19005V2
1. It is possible to use MCO1 or MCO2 to save a crystal. It is however not mandatory to clock the STM32F42x
with a 24 or 26MHz crystal when using USB HS. The above figure only shows an example of a possible
connection.
2. The ID pin is required in dual role only.
DS9405 Rev 13 229/240
231

<!-- Page 230 -->


|  | MII_TX_EN |
| --- | --- |
|  | MII_TXD[3:0] |
|  | MII_CRS |
|  | MII_COL |
|  | MII_RX_CLK |
|  | MII_RXD[3:0] |
|  | MII_RX_DV |
|  | MII_RX_ER |
|  | MDIO |
|  | MDC |
|  | PPS_OUT(2) PHY_CLK 25 MHz |


|  |  | RMII_TXD[1:0] |  |
| --- | --- | --- | --- |
|  |  | RMII_RXD[1:0] |  |
|  |  | RMII_CRX_DV |  |
|  |  | RMII_REF_CLK MDIO |  |
|  |  | MDIO |  |
|  |  | MDC |  |
|  |  |  | 50 MHz |


| /2 or /20 |
| --- |
| synchronous |

Application block diagrams STM32F427xx STM32F429xx
B.3 Ethernet interface solutions
Figure 99. MII mode using a 25MHz crystal
STM32
MII_TX_CLK
MCU
Ethernet MII_TX_EN Ethernet
MII_TXD[3:0]
MAC 10/100 PHY 10/100
MII_CRS
MII
MII_COL
= 15 pins
HCLK(1)
MII_RX_CLK MII + MDC
MII_RXD[3:0] = 17 pins
IEEE1588 PTP MII_RX_DV
Timer MII_RX_ER
input
TIM2 triggerTimestamp MDIO
comparator
MDC
PPS_OUT(2)
XTAL PLL HCLK
25 MHz OSC MCO1/MCO2 PHY_CLK 25 MHz
XT1
MS19968V1
1. f must be greater than 25MHz.
HCLK
2. Pulse per second when using IEEE1588 PTP optional signal.
Figure 100. RMII with a 50MHz oscillator
STM32
Ethernet
PHY 10/100
MCU RMII_TX_EN
Ethernet
RMII_TXD[1:0]
MAC 10/100
RMII_RXD[1:0] RMII
HCLK(1) = 7 pins
RMII_CRX_DV
RMII + MDC
RMII_REF_CLK = 9 pins
IEEE1588 PTP
Timer MDIO
input
triggerTimestamp MDC
TIM2
comparator
/2 or /20
2.5 or 25 MHz synchronous 50 MHz
OSC PLL HCLK
50 MHz
PHY_CLK 50 MHz XT1 50 MHz
MS19969V1
1. f must be greater than 25MHz.
HCLK
230/240 DS9405 Rev 13

<!-- Page 231 -->


|  |  | RMII_TXD[1:0] |  |
| --- | --- | --- | --- |
|  |  | RMII_RXD[1:0] |  |
|  |  | RMII_CRX_DV |  |
|  |  | RMII_REF_CLK |  |
|  |  | MDIO | P XT1 |
|  |  | MDC |  |
|  |  | PHY_CLK 25 MHz |  |


| /2 or /20 |
| --- |
| synchronous |


| P | LL |
| --- | --- |

STM32F427xx STM32F429xx Application block diagrams
Figure 101. RMII with a 25MHz crystal and PHY with PLL
STM32F Ethernet
PHY 10/100
MCU RMII_TX_EN
Ethernet
RMII_TXD[1:0]
MAC 10/100
RMII_RXD[1:0]
HCLK(1) RMII
RMII_CRX_DV = 7 pins
RMII_REF_CLK REF_CLK RMII + MDC
IEEE1588 PTP = 9 pins
Timer MDIO
input
triggerTimestamp MDC
TIM2
comparator
/2 or /20
2.5 or 25 MHz synchronous 50 MHz
XTAL PLL HCLK PLL
25 MHz OSC MCO1/MCO2 PHY_CLK 25 MHz XT1
MS19970V1
1. f must be greater than 25MHz.
HCLK
2. The 25MHz (PHY_CLK) must be derived directly from the HSE oscillator, before the PLL block.
DS9405 Rev 13 231/240
231

<!-- Page 232 -->

Important security notice STM32F427xx STM32F429xx
9 Important security notice
The STMicroelectronics group of companies (ST) places a high value on product security,
which is why the ST product(s) identified in this documentation may be certified by various
security certification bodies and/or may implement our own security measures as set forth
herein. However, no level of security certification and/or built-in security measures can
guarantee that ST products are resistant to all forms of attacks. As such, it is the
responsibility of each of ST's customers to determine if the level of security provided in an
ST product meets the customer needs both in relation to the ST product alone, as well as
when combined with other components and/or software for the customer end product or
application. In particular, take note that:
• ST products may have been certified by one or more security certification bodies, such
as Platform Security Architecture (www.psacertified.org) and/or Security Evaluation
standard for IoT Platforms (www.trustcb.com). For details concerning whether the ST
product(s) referenced herein have received security certification along with the level
and current status of such certification, either visit the relevant certification standards
website or go to the relevant product page on www.st.com for the most up to date
information. As the status and/or level of security certification for an ST product can
change from time to time, customers should re-check security certification status/level
as needed. If an ST product is not shown to be certified under a particular security
standard, customers should not assume it is certified.
• Certification bodies have the right to evaluate, grant and revoke security certification in
relation to ST products. These certification bodies are therefore independently
responsible for granting or revoking security certification for an ST product, and ST
does not take any responsibility for mistakes, evaluations, assessments, testing, or
other activity carried out by the certification body with respect to any ST product.
• Industry-based cryptographic algorithms (such as AES, DES, or MD5) and other open
standard technologies which may be used in conjunction with an ST product are based
on standards which were not developed by ST. ST does not take responsibility for any
flaws in such cryptographic algorithms or open technologies or for any methods which
have been or may be developed to bypass, decrypt or crack such algorithms or
technologies.
• While robust security testing may be done, no level of certification can absolutely
guarantee protections against all attacks, including, for example, against advanced
attacks which have not been tested for, against new or unidentified forms of attack, or
against any form of attack when using an ST product outside of its specification or
intended use, or in conjunction with other components or software which are used by
customer to create their end product or application. ST is not responsible for resistance
against such attacks. As such, regardless of the incorporated security features and/or
any information or support that may be provided by ST, each customer is solely
responsible for determining if the level of attacks tested for meets their needs, both in
relation to the ST product alone and when incorporated into a customer end product or
application.
• All security features of ST products (inclusive of any hardware, software,
documentation, and the like), including but not limited to any enhanced security
features added by ST, are provided on an "AS IS" BASIS. AS SUCH, TO THE EXTENT
PERMITTED BY APPLICABLE LAW, ST DISCLAIMS ALL WARRANTIES, EXPRESS
OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE IMPLIED WARRANTIES OF
MERCHANTABILITY OR FITNESS FOR A PARTICULAR PURPOSE, unless the
applicable written and signed contract terms specifically provide otherwise.
232/240 DS9405 Rev 13

<!-- Page 233 -->


| Date | Revision | Changes |
| --- | --- | --- |
| 19-Mar-2013 | 1 | Initial release. |
| 10-Sep-2013 | 2 | Added STM32F429xx part numbers and related informations. STM32F427xx part numbers: Replaced FSMC by FMC added Chrom-ART Accelerator and SAI interface. Increased core, timer, GPIOs, SPI maximum frequencies Updated Figure 8.Updated Figure 9. Removed note in Section ·: Standby mode. Updated Figure 18. Updated Table 10: STM32F437xx and STM32F439xx pin and ball definitions and Table 12: STM32F437xx and STM32F439xx alternate function mapping.. Modified Figure 19: Memory map. Updated Table 17: General operating conditions, Table 18: Limitations depending on the operating power supply range. Removed note 1 in Table 22: reset and power control block characteristics. Added Table 23: Over-drive switching characteristics. Updated Section : Typical and maximum current consumption, Table 34: Switching output I/O current consumption, Table 35: Peripheral current consumption and Section : On-chip peripheral current consumption. Updated Table 36: Low-power mode wakeup timings. Modified Section : High-speed external user clock generated from an external source, Section : Low-speed external user clock generated from an external source, and Section 6.3.10: Internal clock source characteristics. Updated Table 43: Main PLL characteristics and Table 45: PLLISAI (audio and LCD-TFT PLL) characteristics. Updated Table 52: EMI characteristics. Updated Table 57: Output voltage characteristics and Table 58: I/O AC characteristics. Updated Table 60: TIMx characteristics, Table 61: I2C characteristics, Table 62: SPI dynamic characteristics, Section : SAI characteristics. Updated Table 102: SDRAM read timings and Table 104: SDRAM write timings. |

STM32F427xx STM32F429xx Revision history
10 Revision history
Table 125. Document revision history
Date Revision Changes
19-Mar-2013 1 Initial release.
Added STM32F429xx part numbers and related informations.
STM32F427xx part numbers:
Replaced FSMC by FMC added Chrom-ART Accelerator and SAI
interface.
Increased core, timer, GPIOs, SPI maximum frequencies
Updated Figure 8.Updated Figure 9.
Removed note in Section ·: Standby mode.
Updated Figure 18.
Updated Table 10: STM32F437xx and STM32F439xx pin and ball
definitions and Table 12: STM32F437xx and STM32F439xx alternate
function mapping..
Modified Figure 19: Memory map.
Updated Table 17: General operating conditions, Table 18: Limitations
depending on the operating power supply range. Removed note 1 in
Table 22: reset and power control block characteristics. Added Table
23: Over-drive switching characteristics.
10-Sep-2013 2 Updated Section : Typical and maximum current consumption, Table
34: Switching output I/O current consumption, Table 35: Peripheral
current consumption and Section : On-chip peripheral current
consumption.
Updated Table 36: Low-power mode wakeup timings.
Modified Section : High-speed external user clock generated from an
external source, Section : Low-speed external user clock generated
from an external source, and Section 6.3.10: Internal clock source
characteristics.
Updated Table 43: Main PLL characteristics and Table 45: PLLISAI
(audio and LCD-TFT PLL) characteristics.
Updated Table 52: EMI characteristics.
Updated Table 57: Output voltage characteristics and Table 58: I/O AC
characteristics.
Updated Table 60: TIMx characteristics, Table 61: I2C characteristics,
Table 62: SPI dynamic characteristics, Section : SAI characteristics.
Updated Table 102: SDRAM read timings and Table 104: SDRAM write
timings.
DS9405 Rev 13 233/240
239

<!-- Page 234 -->


| Date | Revision | Changes |
| --- | --- | --- |
| 24-Jan-2014 | 3 | .Added STM32F429xE part numbers featuring 512 Mbytes of Flash memory and UFBGA169 package. Added LPSDR SDRAM. Changed INTN into INTR in Figure 4: STM32F437xx and STM32F439xx block diagram. Added note 4 in Table 2: STM32F427xx and STM32F429xx features and peripheral counts. Updated Section 3.15: Boot modes. Updated for PA4 and PA5 in Table 10: STM32F437xx and STM32F439xx pin and ball definitions. Added VIN for BOOT0 pins in Table 14: Voltage characteristics. Updated Note 6., added Note 1.,and updated maximum VIN for B pins in Table 17: General operating conditions. Updated maximum Flash memory access frequency with wait states for VDD =1.8 to 2.1 V in Table 18: Limitations depending on the operating power supply range. Updated Table 24: Typical and maximum current consumption in Run mode, code with data processing running from Flash memory (ART accelerator enabled except prefetch) or RAM and Table 25: Typical and maximum current consumption in Run mode, code with data processing running from Flash memory (ART accelerator disabled). Updated Table 30: Typical current consumption in Run mode, code with data processing running from Flash memory or RAM, regulator ON (ART accelerator enabled except prefetch), VDD=1.7 V, Table 31: Typical current consumption in Run mode, code with data processing running from Flash memory, regulator OFF (ART accelerator enabled except prefetch), and Table 32: Typical current consumption in Sleep mode, regulator ON, VDD=1.7 V. Updated Table 57: Output voltage characteristics. Updated Table 58: I/O AC characteristics. Added Figure 35. Updated th(SDA), tr(SDA) and tr(SCL) and added tSP in Table 61: I2C characteristics. Updated fSCK in Table 62: SPI dynamic characteristics. Updated Table 70: Dynamic characteristics: USB ULPI. Updated Section 6.3.26: FMC characteristics conditions. Updated Figure 73: SDRAM read access waveforms (CL = 1) and Figure 74: SDRAM write access waveforms. Added Table 103: LPSDR SDRAM read timings and Table 105: LPSDR SDRAM write timings. Updated Table 102: SDRAM read timings and Table 104: SDRAM write timings and added note 2.Table 108: Dynamic characteristics: SD / MMC characteristics |

Revision history STM32F427xx STM32F429xx
Table 125. Document revision history
Date Revision Changes
.Added STM32F429xE part numbers featuring 512 Mbytes of Flash
memory and UFBGA169 package.
Added LPSDR SDRAM.
Changed INTN into INTR in Figure 4: STM32F437xx and
STM32F439xx block diagram.
Added note 4 in Table 2: STM32F427xx and STM32F429xx features
and peripheral counts.
Updated Section 3.15: Boot modes.
Updated for PA4 and PA5 in Table 10: STM32F437xx and
STM32F439xx pin and ball definitions.
Added VIN for BOOT0 pins in Table 14: Voltage characteristics.
Updated Note 6., added Note 1.,and updated maximum VIN for B pins
in Table 17: General operating conditions.
Updated maximum Flash memory access frequency with wait states
for VDD =1.8 to 2.1 V in Table 18: Limitations depending on the
operating power supply range.
Updated Table 24: Typical and maximum current consumption in Run
mode, code with data processing running from Flash memory (ART
accelerator enabled except prefetch) or RAM and Table 25: Typical and
maximum current consumption in Run mode, code with data
24-Jan-2014 3 processing running from Flash memory (ART accelerator disabled).
Updated Table 30: Typical current consumption in Run mode, code
with data processing running from Flash memory or RAM, regulator
ON (ART accelerator enabled except prefetch), VDD=1.7 V, Table 31:
Typical current consumption in Run mode, code with data processing
running from Flash memory, regulator OFF (ART accelerator enabled
except prefetch), and Table 32: Typical current consumption in Sleep
mode, regulator ON, VDD=1.7 V.
Updated Table 57: Output voltage characteristics.
Updated Table 58: I/O AC characteristics. Added Figure 35.
Updated th(SDA), tr(SDA) and tr(SCL) and added tSP in Table 61: I2C
characteristics.
Updated fSCK in Table 62: SPI dynamic characteristics.
Updated Table 70: Dynamic characteristics: USB ULPI.
Updated Section 6.3.26: FMC characteristics conditions. Updated
Figure 73: SDRAM read access waveforms (CL = 1) and Figure 74:
SDRAM write access waveforms. Added Table 103: LPSDR SDRAM
read timings and Table 105: LPSDR SDRAM write timings. Updated
Table 102: SDRAM read timings and Table 104: SDRAM write timings
and added note 2.Table 108: Dynamic characteristics: SD / MMC
characteristics
234/240 DS9405 Rev 13

<!-- Page 235 -->


| Date | Revision | Changes |
| --- | --- | --- |
| 24-Apr-2014 | 4 | In the whole document, minimum supply voltage changed to 1.7 V when external power supply supervisor is used. Added DCMI_VSYNC alternate function on PG9 and updated note 6. in Table 10: STM32F437xx and STM32F439xx pin and ball definitions and Table 12: STM32F437xx and STM32F439xx alternate function mapping. Added note 2.belowFigure 16: STM32F43x UFBGA169 ballout. Changed SVGA (800x600) into XGA1024x768) on cover page and in Section 3.10: LCD-TFT controller (available only on STM32F439xx). Updated Section 3.18.2: Regulator OFF. Updated signal corresponding to pin L5 in Figure 12: STM32F43x WLCSP143 ballout. Added ACCHSE in Table 39: HSE 4-26 MHz oscillator characteristics and ACCLSE in Table 40: LSE oscillator characteristics (fLSE = 32.768 kHz). Updated Table 53: ESD absolute maximum ratings. Updated VIH in Table 56: I/O static characteristics. Added condition VDD>1.7 V in Table 58: I/O AC characteristics. Updated conditions in Table 62: SPI dynamic characteristics. Added ZDRV in Table 67: USB OTG full speed electrical characteristics Removed note 3 in Table 80: Temperature sensor characteristics. Added Figure 82: LQFP100 marking example (package top view), Figure 85: WLCSP143 marking example (package top view), Figure 88: LQFP144 marking example (package top view), Figure 91: LQFP176 marking (package top view), Figure 94: LQFP208 marking example (package top view), Figure 97: UFBGA169 marking example (package top view) and Figure 100: UFBGA176+25 marking example (package top view). Added Appendix A: Recommendations when using internal reset OFF. Removed Internal reset OFF hardware connection appendix. |

STM32F427xx STM32F429xx Revision history
Table 125. Document revision history
Date Revision Changes
In the whole document, minimum supply voltage changed to 1.7 V
when external power supply supervisor is used.
Added DCMI_VSYNC alternate function on PG9 and updated note 6.
in Table 10: STM32F437xx and STM32F439xx pin and ball definitions
and Table 12: STM32F437xx and STM32F439xx alternate function
mapping. Added note 2.belowFigure 16: STM32F43x UFBGA169
ballout.
Changed SVGA (800x600) into XGA1024x768) on cover page and in
Section 3.10: LCD-TFT controller (available only on STM32F439xx).
Updated Section 3.18.2: Regulator OFF.
Updated signal corresponding to pin L5 in Figure 12: STM32F43x
WLCSP143 ballout.
Added ACCHSE in Table 39: HSE 4-26 MHz oscillator characteristics
and ACCLSE in Table 40: LSE oscillator characteristics (fLSE = 32.768
kHz).
24-Apr-2014 4 Updated Table 53: ESD absolute maximum ratings.
Updated VIH in Table 56: I/O static characteristics. Added condition
VDD>1.7 V in Table 58: I/O AC characteristics.
Updated conditions in Table 62: SPI dynamic characteristics.
Added ZDRV in Table 67: USB OTG full speed electrical characteristics
Removed note 3 in Table 80: Temperature sensor characteristics.
Added Figure 82: LQFP100 marking example (package top view),
Figure 85: WLCSP143 marking example (package top view), Figure
88: LQFP144 marking example (package top view), Figure 91:
LQFP176 marking (package top view), Figure 94: LQFP208 marking
example (package top view), Figure 97: UFBGA169 marking example
(package top view) and Figure 100: UFBGA176+25 marking example
(package top view).
Added Appendix A: Recommendations when using internal reset OFF.
Removed Internal reset OFF hardware connection appendix.
DS9405 Rev 13 235/240
239

<!-- Page 236 -->


| Date | Revision | Changes |
| --- | --- | --- |
| 19-Feb-2015 | 5 | Update SPI/IS2 in Table 2: STM32F427xx and STM32F429xx features and peripheral counts. Updated LQFP208 in Table 4: Regulator ON/OFF and internal reset ON/OFF availability. Updated Figure 19: Memory map. Changed PLS[2:0]=101 (falling edge) maximum value in Table 22: reset and power control block characteristics. Updated current consumption with all peripherals disabled in Table 24: Typical and maximum current consumption in Run mode, code with data processing running from Flash memory (ART accelerator enabled except prefetch) or RAM. Updated note 1. in Table 28: Typical and maximum current consumptions in Standby mode. Updated tWUSTOP in Table 36: Low-power mode wakeup timings. Updated ESD standards and Table 53: ESD absolute maximum ratings. Updated Table 56: I/O static characteristics. Section : I2C interface characteristics: updated section introduction, removed Table I2C characteristics, Figure I2C bus AC waveforms and measurement circuit and Table SCL frequency; added Table 61: I2C analog filter characteristics. Updated measurement conditions in Table 62: SPI dynamic characteristics. Updated Figure 51: Typical connection diagram using the ADC. Updated Section : Device marking for LQFP100. Updated Figure 83: WLCSP143 - 143-ball, 4.521x 5.547 mm, 0.4 mm pitch wafer level chip scale package outline and Table 111: WLCSP143 - 143-ball, 4.521x 5.547 mm, 0.4 mm pitch wafer level chip scale package mechanical data; added Figure 84: WLCSP143 - 143-ball, 4.521x 5.547 mm, 0.4 mm pitch wafer level chip scale recommended footprint and Table 112: WLCSP143 recommended PCB design rules (0.4 mm pitch). Updated Figure 85: WLCSP143 marking example (package top view) and related note. Updated Section : Device marking for WLCSP143. Updated Section : Device marking for LQFP144. Updated Section : Device marking for LQFP176. Updated Figure 92: LQFP208 - 208-pin, 28 x 28 mm low-profile quad flat package outline; Updated Section : Device marking for LQFP208. Modified UFBGA169 pitch, updated Figure 95: UFBGA169 - 169-ball 7 x 7 mm 0.50 mm pitch, ultra fine pitch ball grid array package outline and Table 116: UFBGA169 - 169-ball 7 x 7 mm 0.50 mm pitch, ultra fine pitch ball grid array package mechanical data; updated Section : Device marking for LQFP208. updated Section : Device marking for UFBGA169, Section : Device marking for UFBGA176+25 and Section : Device marking for TFBGA176. Updated Z pin count in Table : . ?ǣ |

Revision history STM32F427xx STM32F429xx
Table 125. Document revision history
Date Revision Changes
Update SPI/IS2 in Table 2: STM32F427xx and STM32F429xx features
and peripheral counts.
Updated LQFP208 in Table 4: Regulator ON/OFF and internal reset
ON/OFF availability.
Updated Figure 19: Memory map.
Changed PLS[2:0]=101 (falling edge) maximum value in Table 22:
reset and power control block characteristics.
Updated current consumption with all peripherals disabled in Table 24:
Typical and maximum current consumption in Run mode, code with
data processing running from Flash memory (ART accelerator
enabled except prefetch) or RAM. Updated note 1. in Table 28: Typical
and maximum current consumptions in Standby mode.
Updated tWUSTOP in Table 36: Low-power mode wakeup timings.
Updated ESD standards and Table 53: ESD absolute maximum
ratings.
Updated Table 56: I/O static characteristics.
Section : I2C interface characteristics: updated section introduction,
removed Table I2C characteristics, Figure I2C bus AC waveforms and
measurement circuit and Table SCL frequency; added Table 61: I2C
analog filter characteristics.
Updated measurement conditions in Table 62: SPI dynamic
characteristics.
Updated Figure 51: Typical connection diagram using the ADC.
19-Feb-2015 5
Updated Section : Device marking for LQFP100.
Updated Figure 83: WLCSP143 - 143-ball, 4.521x 5.547 mm, 0.4 mm
pitch wafer level chip scale package outline and Table 111: WLCSP143
- 143-ball, 4.521x 5.547 mm, 0.4 mm pitch wafer level chip scale
package mechanical data; added Figure 84: WLCSP143 - 143-ball,
4.521x 5.547 mm, 0.4 mm pitch wafer level chip scale recommended
footprint and Table 112: WLCSP143 recommended PCB design rules
(0.4 mm pitch). Updated Figure 85: WLCSP143 marking example
(package top view) and related note. Updated Section : Device
marking for WLCSP143.
Updated Section : Device marking for LQFP144.
Updated Section : Device marking for LQFP176.
Updated Figure 92: LQFP208 - 208-pin, 28 x 28 mm low-profile quad
flat package outline; Updated Section : Device marking for LQFP208.
Modified UFBGA169 pitch, updated Figure 95: UFBGA169 - 169-ball 7
x 7 mm 0.50 mm pitch, ultra fine pitch ball grid array package outline
and Table 116: UFBGA169 - 169-ball 7 x 7 mm 0.50 mm pitch, ultra
fine pitch ball grid array package mechanical data; updated Section :
Device marking for LQFP208.
updated Section : Device marking for UFBGA169, Section : Device
marking for UFBGA176+25 and Section : Device marking for
TFBGA176.
Updated Z pin count in Table : .
?ǣ
236/240 DS9405 Rev 13

<!-- Page 237 -->


| Date | Revision | Changes |
| --- | --- | --- |
| 17-Sep-2015 | 6 | Updated notes related to the minimum and maximum values guaranteed by design, characterization or test in production. Updated IDD_STOP_UDM in Table 27: Typical and maximum current consumptions in Stop mode. Removed note related to tests in production in Table 24: Typical and maximum current consumption in Run mode, code with data processing running from Flash memory (ART accelerator enabled except prefetch) or RAM and Table 26: Typical and maximum current consumption in Sleep mode. Updated Table 41: HSI oscillator characteristics. Figure 31 renamed ACCHSI accuracy versus temperature and updated. Updated Figure 38: SPI timing diagram - slave mode and CPHA = 0. Updated Section : Ethernet characteristics. Updated Table 43: Main PLL characteristics, Table 44: PLLI2S (audio PLL) characteristics and Table 45: PLLISAI (audio and LCD-TFT PLL) characteristics. Removed note 1 in Table 75: ADC static accuracy at fADC = 18 MHz, Table 76: ADC static accuracy at fADC = 30 MHz and Table 77: ADC static accuracy at fADC = 36 MHz. Updated td(SDCLKL _Data) and th(SDCLKL _Data) in Table 104: SDRAM write timings. Added Figure 96: UFBGA169 - 169-ball, 7 x 7 mm, 0.50 mm pitch, ultra fine pitch ball grid array recommended footprint and Table 117: UFBGA169 recommended PCB design rules (0.5 mm pitch BGA). Added Figure 99: UFBGA176+25-ball, 10 x 10 mm, 0.65 mm pitch, ultra fine pitch ball grid array package recommended footprint and Table 119: UFBGA176+25 recommended PCB design rules (0.65 mm pitch BGA). |
| 30-Nov-2015 | 7 | Updated |VSSX -VSS| in Table 14: Voltage characteristics to add VREF-. Updated td(TXEN) and td(TXD) minimum value in Table 72: Dynamics characteristics: Ethernet MAC signals for RMII and Table 73: Dynamics characteristics: Ethernet MAC signals for MII. Added VREF- in Table 74: ADC characteristics. Added A1 minimum and maximum values in Table 111: WLCSP143 - 143-ball, 4.521x 5.547 mm, 0.4 mm pitch wafer level chip scale package mechanical data. Updated Figure 86: LQFP144-144-pin, 20 x 20 mm low-profile quad flat package outline. Updated Figure 98: UFBGA176+25 - ball 10 x 10 mm, 0.65 mm pitch ultra thin fine pitch ball grid array package outline and Table 118: UFBGA176+25 - ball, 10 x 10 mm, 0.65 mm pitch, ultra fine pitch ball grid array package mechanical data. Updated Figure 101: TFBGA216 - 216 ball 13 × 13 mm 0.8 mm pitch thin fine pitch ball grid array package outline and Table 120: TFBGA216 - 216 ball 13 × 13 mm 0.8 mm pitch thin fine pitch ball grid array package mechanical data. |
| 21-Jan-2016 | 8 | Updated Figure 22: Power supply scheme. Added td(TXD) values corresponding to 1.71 V < VDD < 3.6 V in Table 72: Dynamics characteristics: Ethernet MAC signals for RMII. |

STM32F427xx STM32F429xx Revision history
Table 125. Document revision history
Date Revision Changes
Updated notes related to the minimum and maximum values
guaranteed by design, characterization or test in production.
Updated IDD_STOP_UDM in Table 27: Typical and maximum current
consumptions in Stop mode.
Removed note related to tests in production in Table 24: Typical and
maximum current consumption in Run mode, code with data
processing running from Flash memory (ART accelerator enabled
except prefetch) or RAM and Table 26: Typical and maximum current
consumption in Sleep mode.
Updated Table 41: HSI oscillator characteristics. Figure 31 renamed
ACCHSI accuracy versus temperature and updated.
Updated Figure 38: SPI timing diagram - slave mode and CPHA = 0.
Updated Section : Ethernet characteristics.
Updated Table 43: Main PLL characteristics, Table 44: PLLI2S (audio
17-Sep-2015 6
PLL) characteristics and Table 45: PLLISAI (audio and LCD-TFT PLL)
characteristics.
Removed note 1 in Table 75: ADC static accuracy at fADC = 18 MHz,
Table 76: ADC static accuracy at fADC = 30 MHz and Table 77: ADC
static accuracy at fADC = 36 MHz.
Updated td(SDCLKL _Data) and th(SDCLKL _Data) in Table 104:
SDRAM write timings.
Added Figure 96: UFBGA169 - 169-ball, 7 x 7 mm, 0.50 mm pitch, ultra
fine pitch ball grid array recommended footprint and Table 117:
UFBGA169 recommended PCB design rules (0.5 mm pitch BGA).
Added Figure 99: UFBGA176+25-ball, 10 x 10 mm, 0.65 mm pitch,
ultra fine pitch ball grid array package recommended footprint and
Table 119: UFBGA176+25 recommended PCB design rules (0.65 mm
pitch BGA).
Updated |VSSX -VSS| in Table 14: Voltage characteristics to add
VREF-.
Updated td(TXEN) and td(TXD) minimum value in Table 72: Dynamics
characteristics: Ethernet MAC signals for RMII and Table 73: Dynamics
characteristics: Ethernet MAC signals for MII.
Added VREF- in Table 74: ADC characteristics.
Added A1 minimum and maximum values in Table 111: WLCSP143 -
143-ball, 4.521x 5.547 mm, 0.4 mm pitch wafer level chip scale
30-Nov-2015 7 package mechanical data. Updated Figure 86: LQFP144-144-pin, 20 x
20 mm low-profile quad flat package outline.
Updated Figure 98: UFBGA176+25 - ball 10 x 10 mm, 0.65 mm pitch
ultra thin fine pitch ball grid array package outline and Table 118:
UFBGA176+25 - ball, 10 x 10 mm, 0.65 mm pitch, ultra fine pitch ball
grid array package mechanical data. Updated Figure 101: TFBGA216 -
216 ball 13 × 13 mm 0.8 mm pitch thin fine pitch ball grid array package
outline and Table 120: TFBGA216 - 216 ball 13 × 13 mm 0.8 mm pitch
thin fine pitch ball grid array package mechanical data.
Updated Figure 22: Power supply scheme.
Added td(TXD) values corresponding to 1.71 V < VDD < 3.6 V in Table
21-Jan-2016 8
72: Dynamics characteristics: Ethernet MAC signals for RMII.
DS9405 Rev 13 237/240
239

<!-- Page 238 -->


| Date | Revision | Changes |
| --- | --- | --- |
| 18-Jul-2016 | 9 | Updated Figure 1: Compatible board design STM32F10xx/STM32F2xx/STM32F4xx for LQFP100 package. Added mission profile compliance with JEDEC JESD47 in Section 6.2: Absolute maximum ratings. Changed Figure 31 HSI deviation versus temperature to ACCHSI versus temperature. Updated RLOAD in Table 85: DAC characteristics. Added note 2. related to the position of the 0.1 µF capacitor below Figure 37: Recommended NRST pin protection. Updated Figure 40: SPI timing diagram - master mode. Added reference to optional marking or inset/upset marks in all package device marking sections. Updated Figure 85: WLCSP143 marking example (package top view), Figure 88: LQFP144 marking example (package top view), Figure 91: LQFP176 marking (package top view), Figure 94: LQFP208 marking example (package top view). Updated Figure 98: UFBGA176+25 - ball 10 x 10 mm, 0.65 mm pitch ultra thin fine pitch ball grid array package outline and Table 118: UFBGA176+25 - ball, 10 x 10 mm, 0.65 mm pitch, ultra fine pitch ball grid array package mechanical data. |
| 19-Jan-2018 | 10 | Updated Arm wordmark and added Arm logo in Section 2: Description. Updated LDC-TFT feature on cover page. Updated Table 24: Typical and maximum current consumption in Run mode, code with data processing running from Flash memory (ART accelerator enabled except prefetch) or RAM and Table 26: Typical and maximum current consumption in Sleep mode. RADC minimum value added in Table 74: ADC characteristics. LTDC clock output frequency changed to 83 MHz in Table 107: LTDC characteristics. |

Revision history STM32F427xx STM32F429xx
Table 125. Document revision history
Date Revision Changes
Updated Figure 1: Compatible board design
STM32F10xx/STM32F2xx/STM32F4xx for LQFP100 package.
Added mission profile compliance with JEDEC JESD47 in Section 6.2:
Absolute maximum ratings.
Changed Figure 31 HSI deviation versus temperature to ACCHSI
versus temperature.
Updated RLOAD in Table 85: DAC characteristics.
Added note 2. related to the position of the 0.1 µF capacitor below
Figure 37: Recommended NRST pin protection.
18-Jul-2016 9
Updated Figure 40: SPI timing diagram - master mode.
Added reference to optional marking or inset/upset marks in all
package device marking sections. Updated Figure 85: WLCSP143
marking example (package top view), Figure 88: LQFP144 marking
example (package top view), Figure 91: LQFP176 marking (package
top view), Figure 94: LQFP208 marking example (package top view).
Updated Figure 98: UFBGA176+25 - ball 10 x 10 mm, 0.65 mm pitch
ultra thin fine pitch ball grid array package outline and Table 118:
UFBGA176+25 - ball, 10 x 10 mm, 0.65 mm pitch, ultra fine pitch ball
grid array package mechanical data.
Updated Arm wordmark and added Arm logo in Section 2: Description.
Updated LDC-TFT feature on cover page.
Updated Table 24: Typical and maximum current consumption in Run
mode, code with data processing running from Flash memory (ART
19-Jan-2018 10 accelerator enabled except prefetch) or RAM and Table 26: Typical and
maximum current consumption in Sleep mode.
RADC minimum value added in Table 74: ADC characteristics.
LTDC clock output frequency changed to 83 MHz in Table 107: LTDC
characteristics.
238/240 DS9405 Rev 13

<!-- Page 239 -->


| Date | Revision | Changes |
| --- | --- | --- |
| 24-Oct-2024 | 11 | General datasheet update to include minor terminology updates. Updated the datasheet cover page. General update of the Section 7: Package information. Updated the figure Figure 19: Memory map. Edited the footnote in Table 41: HSI oscillator characteristics. Updated the Table 22: Reset and power control block characteristics. Updated the Section 6.2: Absolute maximum ratings. Updated the Section : I/O system current consumption. Updated the Section 3.36: True random number generator (RNG). Updated the Figure 36: I/O AC characteristics definition. Updated the Figure 51: Typical connection diagram when using the ADC with FT/TT pins featuring the analog switch function. Updated the Section : Electromagnetic Interference (EMI). Updated the Figure 38: SPI timing diagram - slave mode and CPHA = 0, the Figure 39: SPI timing diagram - slave mode and CPHA = 1, and the Figure 40: SPI timing diagram - master mode. Updated the Figure 69: NAND controller waveforms for read access and Figure 70: NAND controller waveforms for write access. Updated the Table 14: Voltage characteristics. |
| 05-May-2025 | 12 | Corrected Table57: I/O static characteristics. |
| 23-Feb-2026 | 13 | Updated Arm legal notice in Section1: Introduction. Updated V max. in Table14: Voltage characteristics. IN Updated terminology in Table60: NRST pin characteristics. |

STM32F427xx STM32F429xx Revision history
Table 125. Document revision history
Date Revision Changes
General datasheet update to include minor terminology updates.
Updated the datasheet cover page.
General update of the Section 7: Package information.
Updated the figure Figure 19: Memory map.
Edited the footnote in Table 41: HSI oscillator characteristics.
Updated the Table 22: Reset and power control block characteristics.
Updated the Section 6.2: Absolute maximum ratings.
Updated the Section : I/O system current consumption.
Updated the Section 3.36: True random number generator (RNG).
24-Oct-2024 11
Updated the Figure 36: I/O AC characteristics definition.
Updated the Figure 51: Typical connection diagram when using the
ADC with FT/TT pins featuring the analog switch function.
Updated the Section : Electromagnetic Interference (EMI).
Updated the Figure 38: SPI timing diagram - slave mode and CPHA =
0, the Figure 39: SPI timing diagram - slave mode and CPHA = 1, and
the Figure 40: SPI timing diagram - master mode.
Updated the Figure 69: NAND controller waveforms for read access
and Figure 70: NAND controller waveforms for write access.
Updated the Table 14: Voltage characteristics.
05-May-2025 12 Corrected Table57: I/O static characteristics.
Updated Arm legal notice in Section1: Introduction.
23-Feb-2026 13 Updated V max. in Table14: Voltage characteristics.
IN
Updated terminology in Table60: NRST pin characteristics.
DS9405 Rev 13 239/240
239

<!-- Page 240 -->

STM32F427xx STM32F429xx
IMPORTANT NOTICE – READ CAREFULLY
STMicroelectronics NV and its subsidiaries (“ST”) reserve the right to make changes, corrections, enhancements, modifications, and
improvements to ST products and/or to this document at any time without notice.
In the event of any conflict between the provisions of this document and the provisions of any contractual arrangement in force between the
purchasers and ST, the provisions of such contractual arrangement shall prevail.
The purchasers should obtain the latest relevant information on ST products before placing orders. ST products are sold pursuant to ST’s
terms and conditions of sale in place at the time of order acknowledgment.
The purchasers are solely responsible for the choice, selection, and use of ST products and ST assumes no liability for application
assistance or the design of the purchasers’ products.
No license, express or implied, to any intellectual property right is granted by ST herein.
Resale of ST products with provisions different from the information set forth herein shall void any warranty granted by ST for such product.
If the purchasers identify an ST product that meets their functional and performance requirements but that is not designated for the
purchasers' market segment, the purchasers shall contact ST for more information.
ST and the ST logo are trademarks of ST. For additional information about ST trademarks, refer to www.st.com/trademarks. All other product
or service names are the property of their respective owners.
Information in this document supersedes and replaces information previously supplied in any prior versions of this document.
© 2026 STMicroelectronics – All rights reserved
240/240 DS9405 Rev 13