# AN4031 - Using the STM32F2, STM32F4 and STM32F7 Series DMA Controller

> Converted from PDF. Source: `STM32F4_DMA.pdf`

---


<!-- Page 1 -->

AN4031
Application note
Using the STM32F2, STM32F4 and STM32F7 Series
DMA controller
Introduction
This application note describes how to use direct memory access (DMA) controller available
in STM32F2, STM32F4 and STM32F7 Series. The DMA controller features, the system
architecture, the multi-layer bus matrix and the memory system contribute to provide a high
data bandwidth and to develop very low latency response-time software.
This application note also describes some tips and tricks to allow developers to take full
advantage of these features and ensure correct response times for different peripherals and
subsystems.
STM32F2, STM32F4 and STM32F7 are referred to as “STM32F2/F4/F7 devices” and the
DMA controller as “DMA” throughout the document.
In this document STM32F4 Series is selected as illustrative example. DMA behavior is the
same over STM32F2, STM32F4 and STM32F7 Series unless otherwise specified.
Reference documents
This application note should be read in conjunction with the STM32F2/F4/F7 reference
manuals:
• STM32F205/215 and STM32F207/217 reference manual (RM0033)
• STM32F405/415, STM32F407/417, STM32F427/437 and STM32F429/439 reference
manual (RM0090)
• STM32F401xB/C and STM32F401xD/E reference manual (RM0368)
• STM32F410 reference manual (RM0401)
• STM32F411xC/E reference manual (RM0383)
• STM32F412 reference manual (RM0402)
• STM32F446xx reference manual (RM0390)
• STM32F469xx and STM32F479xx reference manual (RM0386)
• STM32F75xxx and STM32F74xxx reference manual (RM0385)
• STM32F76xxx and STM32F77xxx reference manual (RM0410)
June 2016 DocID022648 Rev 3 1/38
www.st.com

<!-- Page 2 -->

Contents AN4031
Contents
1 DMA controller description . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 6
1.1 DMA transfer properties . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 6
1.1.1 DMA streams/channels . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 7
1.1.2 Stream priority . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 9
1.1.3 Source and destination addresses . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 9
1.1.4 Transfer mode . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 10
1.1.5 Transfer size . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 10
1.1.6 Incrementing source/destination address . . . . . . . . . . . . . . . . . . . . . . . 10
1.1.7 Source and destination data width . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 10
1.1.8 Transfer types . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 10
1.1.9 DMA FIFO mode . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 11
1.1.10 Source and destination burst size . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 12
1.1.11 Double-buffer mode . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 13
1.1.12 Flow control . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 14
1.2 Setting up a DMA transfer . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 14
2 System performance considerations . . . . . . . . . . . . . . . . . . . . . . . . . . 16
2.1 Multi-layer bus matrix . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 17
2.1.1 Definitions . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 17
2.1.2 Round-robin priority scheme . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 18
2.1.3 BusMatrix arbitration and DMA transfer delays worst case . . . . . . . . . . 19
2.2 DMA transfer paths . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 20
2.2.1 Dual DMA port . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 20
2.2.2 DMA transfer states . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 22
2.2.3 DMA request arbitration . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 23
2.3 AHB-to-APB bridge . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 24
2.3.1 Dual AHB-to-APB port . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 24
2.3.2 AHB-to-APB bridge arbitration . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 24
3 How to predict DMA latencies . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 26
3.1 DMA transfer time . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 26
3.1.1 Default DMA transfer timing . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 26
3.1.2 DMA transfer time versus concurrent access . . . . . . . . . . . . . . . . . . . . 27
3.2 Examples . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 28
2/38 DocID022648 Rev 3

<!-- Page 3 -->

AN4031 Contents
3.2.1 ADC-to-SRAM DMA transfer . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 28
3.2.2 SPI full duplex DMA transfer . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 29
4 Tips and warnings while programming the DMA controller . . . . . . . . 31
4.1 Software sequence to disable DMA . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 31
4.2 DMA flag management before enabling a new transfer . . . . . . . . . . . . . . 31
4.3 Software sequence to enable DMA . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 31
4.4 Memory-to-memory transfer while NDTR=0 . . . . . . . . . . . . . . . . . . . . . . 31
4.5 DMA peripheral burst with PINC/MINC=0 . . . . . . . . . . . . . . . . . . . . . . . . 31
4.6 Twice-mapped DMA requests . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 32
4.7 Best DMA throughput configuration . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 32
4.8 DMA transfer suspension . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 32
4.9 Take benefits of DMA2 controller and system architecture flexibility . . . . 33
4.9.1 Inverting transfers over DMA2 AHB ports consideration . . . . . . . . . . . . 33
4.9.2 Example for inverting Quad-SPI transfers over DMA2 AHB ports
consideration . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 34
4.10 STM32F7 DMA transfer and cache maintenance to avoid data incoherency
35
5 Conclusion . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 36
6 Revision history . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 37
DocID022648 Rev 3 3/38
3

<!-- Page 4 -->

List of tables AN4031
List of tables
Table 1. STM32F427/437 and STM32F429/439 DMA1 request mapping. . . . . . . . . . . . . . . . . . . . . 8
Table 2. STM32F427/437 and STM32F429/439 DMA2 request mapping. . . . . . . . . . . . . . . . . . . . . 9
Table 3. Possible burst configurations . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 13
Table 4. Peripheral port access/transfer time versus DMA path used . . . . . . . . . . . . . . . . . . . . . . . 27
Table 5. Memory port access/transfer time . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 27
Table 6. DMA peripheral (ADC) port transfer latency. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 28
Table 7. DMA memory (SRAM) port transfer latency. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 28
Table 8. DMA AHB port direction vs. transfer mode configuration. . . . . . . . . . . . . . . . . . . . . . . . . . 33
Table 9. Code Snippet . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 35
Table 10. Document revision history . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 37
4/38 DocID022648 Rev 3

<!-- Page 5 -->

AN4031 List of figures
List of figures
Figure 1. DMA block diagram . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 7
Figure 2. Channel selection. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 8
Figure 3. DMA source address and destination address incrementing . . . . . . . . . . . . . . . . . . . . . . . 10
Figure 4. FIFO structure. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 11
Figure 5. DMA burst transfer . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 12
Figure 6. Double-buffer mode . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 13
Figure 7. STM32F405/415 and STM32F407/417 system architecture . . . . . . . . . . . . . . . . . . . . . . . 17
Figure 8. CPU and DMA1 request an access to SRAM1. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 18
Figure 9. Five masters request SRAM access. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 19
Figure 10. DMA transfer delay due to CPU transfer issued by interrupt . . . . . . . . . . . . . . . . . . . . . . . 20
Figure 11. DMA dual port. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 21
Figure 12. Peripheral-to-memory transfer states. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 22
Figure 13. Memory-to-peripheral transfer states . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 23
Figure 14. DMA request arbitration . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 23
Figure 15. AHB-to-APB1 bridge concurrent CPU and DMA1 access request. . . . . . . . . . . . . . . . . . . 25
Figure 16. SPI full duplex DMA transfer time. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 29
Figure 17. DMA in Memory-to-Pheripheral transfer mode. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 34
DocID022648 Rev 3 5/38
5

<!-- Page 6 -->

DMA controller description AN4031
1 DMA controller description
The DMA is an AMBA advanced high-performance bus (AHB) module that features three
AHB ports: a slave port for DMA programming and two master ports (peripheral and
memory ports) that allow the DMA to initiate data transfers between different slave modules.
The DMA allows data transfers to take place in the background, without the intervention of
the Cortex-Mx processor. During this operation, the main processor can execute other tasks
and it is only interrupted when a whole data block is available for processing.
Large amounts of data can be transferred with no major impact on the system performance.
The DMA is mainly used to implement central data buffer storage (usually in the system
SRAM) for different peripheral modules. This solution is less expensive in terms of silicon
and power consumption compared to a distributed solution where each peripheral needs to
implement it own local data storage.
The STM32F2/F4/F7 DMA controller takes full advantage of the multi-layer bus system in
order to ensure very low latency both for DMA transfers and for CPU execution/interrupt
event detection/service.
1.1 DMA transfer properties
A DMA transfer is characterized by the following properties:
• DMA stream/channel
• Stream priority
• Source and destination addresses
• Transfer mode
• Transfer size (only when DMA is the flow controller)
• Source/destination address incrementing or non-incrementing
• Source and destination data width
• Transfer type
• FIFO mode
• Source/destination burst size
• Double-buffer mode
• Flow control
STM32F2/F4/F7 devices embed two DMA controllers, and each DMA has two port, one
peripheral port and one memory port, which can work simultaneously.
Figure 1 shows the DMA block diagram.
6/38 DocID022648 Rev 3

<!-- Page 7 -->


|  | DMA controller retsam REQ_STR0_CH0 Memory port REQ_STR0_CH1 BHA REQ_STR0_CH7 0 1 2 7 3 4 5 6 MAERTS MAERTS MAERTS MAERTS MAERTS MAERTS MAERTS MAERTS REQ_STR1_CH0 REQ_STR1_CH1 REQ_STREAM0 REQ_STREAM1 REQ_STR1_CH7 REQ_STREAM2 REQ_STREAM3 OFIF OFIF OFIF OFIF OFIF OFIF OFIF OFIF REQ_STREAM4 Arbiter REQ_STREAM5 REQ_STREAM6 0 1 2 7 3 4 5 6 REQ_STREAM7 MAERTS MAERTS MAERTS MAERTS MAERTS MAERTS MAERTS MAERTS REQ_STR7_CH0 REQ_STR7_CH1 retsam Peripheral port REQ_STR7_CH7 BHA Channel selection AHB slave programming Programming port interface ai15945 |  |
| --- | --- | --- |


| OFIF OFIF OFIF OFIF OFIF OFIF OFIF OFIF |  |
| --- | --- |
|  |  |

AN4031 DMA controller description
Figure 1. DMA block diagram
The following subsections provide a detailed description of each DMA transfer property.
1.1.1 DMA streams/channels
STM32F2/F4/F7 devices embed two DMA controllers, offering up to 16 streams in total
(eight per controller), each dedicated to managing memory access requests from one or
more peripherals.
Each stream has up to eight selectable channels (requests) in total. This selection is
software-configurable and allows several peripherals to initiate DMA requests.
Figure 2 describes the channel selection for a dedicated stream.
DocID022648 Rev 3 7/38
37
retsam
BHA
Memory port
OFIF
retsam
BHA
Peripheral port
0
MAERTS
0
MAERTS
1
MAERTS
1
MAERTS
OFIF
2
MAERTS
2
MAERTS
OFIF
7
MAERTS
7
MAERTS
OFIF
DMA controller
REQ_STR0_CH0
REQ_STR0_CH1
REQ_STREAM0
3
MAERTS
3
MAERTS
OFIF
4
MAERTS
4
MAERTS
OFIF
5
MAERTS
5
MAERTS
OFIF
6
MAERTS
6
MAERTS
OFIF
REQ_STR0_CH7
REQ_STR1_CH0
REQ_STR1_CH1
REQ_STREAM1
REQ_STR1_CH7 REQ_STREAM2
REQ_STREAM3
REQ_STREAM4 Arbiter
REQ_STREAM5
REQ_STREAM6
REQ_STREAM7
REQ_STR7_CH0
REQ_STR7_CH1
REQ_STR7_CH7
Channel
selection
AHB slave
programming Programming port
interface
ai15945

<!-- Page 8 -->


|  | REQ_STRx_CH7 REQ_STRx_CH6 REQ_STRx_CH5 REQ_STRx_CH4 REQ_STREAMx REQ_STRx_CH3 REQ_STRx_CH2 REQ_STRx_CH1 REQ_STRx_CH0 31 29 27 0 DMA_SxCR CHSEL[2:0] ai15947 |  |
| --- | --- | --- |


| Peripheral requests | Stream 0 | Stream 1 | Stream 2 | Stream 3 | Stream 4 | Stream 5 | Stream 6 | Stream 7 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Channel 0 | SPI3_RX | - | SPI3_RX | SPI2_RX | SPI2_TX | SPI3_TX | - | SPI3_TX |
| Channel 1 | I2C1_RX | - | TIM7_UP |  | TIM7_UP | I2C1_RX | I2C1_TX | I2C1_TX |
| Channel 2 | TIM4_CH1 | - | I2S3_EXT _RX | TIM4_CH2 | I2S2_EXT_ TX | I2S3_EXT _TX | TIM4_UP | TIM4_CH3 |
| Channel 3 | I2S3_EXT _RX | TIM2_UP TIM2_CH3 | I2C3_RX | I2S2_EXT_ RX | I2C3_TX | TIM2_CH1 | TIM2_CH2 TIM2_CH4 | TIM2_UP TIM2_CH4 |
| Channel 4 | UART5 _RX | USART3 _RX | UART4 _RX | USART3 _TX | UART4 _TX | USART2 _RX | USART2 _TX | UART5 _TX |
| Channel 5 | UART8 _TX | UART7 _TX | TIM3_CH4 TIM3_UP | UART7 _RX | TIM3_CH1 TIM3_TRIG | TIM3_CH2 | UART8_ RX | TIM3_CH3 |
| Channel 6 | TIM5_CH3 TIM5_UP | TIM5_CH4 TIM5_TRIG | TIM5_CH1 | TIM5_CH4 TIM5_TRIG | TIM5_CH2 | - | TIM5_UP | - |
| Channel 7 | - | TIM6_UP | I2C2_RX | I2C2_RX | USART3_ TX | DAC1 | DAC2 | I2C2_TX |

DMA controller description AN4031
Figure 2. Channel selection
REQ_STRx_CH7
REQ_STRx_CH6
REQ_STRx_CH5
REQ_STRx_CH4
REQ_STREAMx
REQ_STRx_CH3
REQ_STRx_CH2
REQ_STRx_CH1
REQ_STRx_CH0
31 29 27 0
DMA_SxCR CHSEL[2:0]
ai15947
Note: Only one channel/request can be active at the same time in a stream.
More than one enabled DMA stream must not serve the same peripheral request.
Table 1 and Table 2 show STM32F427/437 and STM32F429/439 DMA1/DMA2 requests
mapping. The tables give the available configuration of DMA streams/channels versus
peripheral requests.
Table 1. STM32F427/437 and STM32F429/439 DMA1 request mapping
Peripheral
Stream 0 Stream 1 Stream 2 Stream 3 Stream 4 Stream 5 Stream 6 Stream 7
requests
Channel 0 SPI3_RX - SPI3_RX SPI2_RX SPI2_TX SPI3_TX - SPI3_TX
Channel 1 I2C1_RX - TIM7_UP TIM7_UP I2C1_RX I2C1_TX I2C1_TX
I2S3_EXT I2S2_EXT_ I2S3_EXT
Channel 2 TIM4_CH1 - TIM4_CH2 TIM4_UP TIM4_CH3
_RX TX _TX
I2S3_EXT TIM2_UP I2S2_EXT_ TIM2_CH2 TIM2_UP
Channel 3 I2C3_RX I2C3_TX TIM2_CH1
_RX TIM2_CH3 RX TIM2_CH4 TIM2_CH4
UART5 USART3 UART4 USART3 UART4 USART2 USART2 UART5
Channel 4
_RX _RX _RX _TX _TX _RX _TX _TX
UART8 UART7 TIM3_CH4 UART7 TIM3_CH1 UART8_
Channel 5 TIM3_CH2 TIM3_CH3
_TX _TX TIM3_UP _RX TIM3_TRIG RX
TIM5_CH3 TIM5_CH4 TIM5_CH4
Channel 6 TIM5_CH1 TIM5_CH2 - TIM5_UP -
TIM5_UP TIM5_TRIG TIM5_TRIG
USART3_
Channel 7 - TIM6_UP I2C2_RX I2C2_RX DAC1 DAC2 I2C2_TX
TX
8/38 DocID022648 Rev 3

<!-- Page 9 -->


| Peripheral requests | Stream 0 | Stream 1 | Stream 2 | Stream 3 | Stream 4 | Stream 5 | Stream 6 | Stream 7 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Channel 0 | ADC1 | SAI1_A | TIM8_CH1 TIM8_CH2 TIM8_CH3 | SAI1_A | ADC1 | SAI1_B | TIM1_CH1 TIM1_CH2 TIM1_CH3 | - |
| Channel 1 | - | DCMI | ADC2 | ADC2 | SAI1_B | SPI6_TX | SPI6_RX | DCMI |
| Channel 2 | ADC3 | ADC3 | - | SPI5_RX | SPI5_TX | CRYP_ OUT | CRYP_IN | HASH_IN |
| Channel 3 | SPI1_RX | - | SPI1_RX | SPI1_TX | - | SPI1_TX | - | - |
| Channel 4 | SPI4_RX | SPI4_TX | USART1_ RX | SDIO | - | USART1_ RX | SDIO | USART1_ TX |
| Channel 5 | - | USART6_ RX | USART6_ RX | SPI4_RX | SPI4_TX | - | USART6_ TX | USART6_ TX |
| Channel 6 | TIM1_TRI G | TIM1_CH1 | TIM1_CH2 | TIM1_CH1 | TIM1_CH4 TIM1_TRIG TIM1_COM | TIM1_UP | TIM1_CH3 | - |
| Channel 7 | - | TIM8_UP | TIM8_CH1 | TIM8_CH2 | TIM8_CH3 | SPI5_RX | SPI5_TX | TIM8_CH4 TIM8_TRIG TIM8_COM |

AN4031 DMA controller description
Table 2. STM32F427/437 and STM32F429/439 DMA2 request mapping
Peripheral
Stream 0 Stream 1 Stream 2 Stream 3 Stream 4 Stream 5 Stream 6 Stream 7
requests
TIM8_CH1 TIM1_CH1
Channel 0 ADC1 SAI1_A TIM8_CH2 SAI1_A ADC1 SAI1_B TIM1_CH2 -
TIM8_CH3 TIM1_CH3
Channel 1 - DCMI ADC2 ADC2 SAI1_B SPI6_TX SPI6_RX DCMI
CRYP_
Channel 2 ADC3 ADC3 - SPI5_RX SPI5_TX CRYP_IN HASH_IN
OUT
Channel 3 SPI1_RX - SPI1_RX SPI1_TX - SPI1_TX - -
USART1_ USART1_ USART1_
Channel 4 SPI4_RX SPI4_TX SDIO - SDIO
RX RX TX
USART6_ USART6_ USART6_ USART6_
Channel 5 - SPI4_RX SPI4_TX -
RX RX TX TX
TIM1_CH4
TIM1_TRI
Channel 6 TIM1_CH1 TIM1_CH2 TIM1_CH1 TIM1_TRIG TIM1_UP TIM1_CH3 -
G
TIM1_COM
TIM8_CH4
Channel 7 - TIM8_UP TIM8_CH1 TIM8_CH2 TIM8_CH3 SPI5_RX SPI5_TX TIM8_TRIG
TIM8_COM
STM32F2/F4/F7 DMA request mapping is designed in such a way that the software
application has more flexibility to map each DMA request for the associated peripheral
request, and that most of the use case applications are covered by multiplexing the
corresponding DMA streams and channels. Refer to DMA1/DMA2 request mapping tables
in the reference manual corresponding to the microcontroller you are using (see Section :
Reference documents).
1.1.2 Stream priority
Each DMA port has an arbiter for handling the priority between other DMA streams. Stream
priority is software-configurable (there are four software levels). If two or more DMA streams
have the same software priority level, the hardware priority is used (stream 0 has priority
over stream 1, etc.).
1.1.3 Source and destination addresses
A DMA transfer is defined by a source address and a destination address. Both the source
and destination should be in the AHB or APB memory ranges and should be aligned to
transfer size.
DocID022648 Rev 3 9/38
37

<!-- Page 10 -->


| (cid:68) |
| --- |
| (cid:69) |
| (cid:70) |
| (cid:71) |
| (cid:72) |
| (cid:17)(cid:17)(cid:17) |


| (cid:68) |
| --- |
| (cid:69) |
|  |
|  |
|  |
|  |

DMA controller description AN4031
1.1.4 Transfer mode
DMA is capable of performing three different transfer modes:
• Peripheral to memory,
• Memory to peripheral,
• Memory to memory (only DMA2 is able to do such transfer, in this mode, the circular
and direct modes are not allowed.)
1.1.5 Transfer size
The transfer size value has to be defined only when the DMA is the flow controller. In fact,
this value defines the volume of data to be transferred from source to destination.
The transfer size is defined by the DMA_SxNDTR register value and by the peripheral side
data width. Depending on the received request (burst or single), the transfer size value is
decreased by the amount of the transferred data.
1.1.6 Incrementing source/destination address
It is possible to configure the DMA to automatically increment the source and/or destination
address after each data transfer.
Figure 3. DMA source address and destination address incrementing
(cid:39)(cid:48)(cid:36)(cid:3)(cid:71)(cid:68)(cid:87)(cid:68)(cid:3)(cid:87)(cid:85)(cid:68)(cid:81)(cid:86)(cid:73)(cid:72)(cid:85)
(cid:68) (cid:68)
(cid:54)(cid:82)(cid:88)(cid:85)(cid:70)(cid:72)(cid:3)(cid:68)(cid:71)(cid:71)(cid:85)(cid:72)(cid:86)(cid:86) (cid:69) (cid:39)(cid:72)(cid:86)(cid:87)(cid:76)(cid:81)(cid:68)(cid:87)(cid:76)(cid:82)(cid:81)(cid:3)(cid:68)(cid:71)(cid:71)(cid:85)(cid:72)(cid:86)(cid:86) (cid:69)
(cid:70)
(cid:71)
(cid:44)(cid:81)(cid:70)(cid:85)(cid:72)(cid:80)(cid:72)(cid:81)(cid:87)(cid:76)(cid:81)(cid:74)(cid:3)(cid:86)(cid:82)(cid:88)(cid:85)(cid:70)(cid:72) (cid:44)(cid:81)(cid:70)(cid:85)(cid:72)(cid:80)(cid:72)(cid:81)(cid:87)(cid:76)(cid:81)(cid:74)(cid:3)(cid:71)(cid:72)(cid:86)(cid:87)(cid:76)(cid:81)(cid:68)(cid:87)(cid:76)(cid:82)(cid:81)
(cid:72)
(cid:17)(cid:17)(cid:17)
(cid:48)(cid:54)(cid:89)(cid:22)(cid:21)(cid:20)(cid:28)(cid:23)(cid:57)(cid:20)
1.1.7 Source and destination data width
Data width for source and destination can be defined as:
● Byte (8 bits)
● Half-word (16 bits)
● Word (32 bits)
1.1.8 Transfer types
• Circular mode: the Circular mode is available to handle circular buffers and continuous
data flows (the DMA_SxNDTR register is then reloaded automatically with the
previously programmed value).
• Normal mode: once the DMA_SxNDTR register reaches zero, the stream is disabled
(the EN bit in the DMA_SxCR register is then equal to 0).
10/38 DocID022648 Rev 3

<!-- Page 11 -->


|  |
| --- |
| 4 words Empty 1/4 1/2 3/4 Full byte lane 3 B15 B 11 B7 B3 Source: byte Destination: word byte lane 2 B14 B10 B6 B2 B15 B14 B13 B12 B11 B10 B9 B8 B7 B6 B5 B4 B3 B2 B1 B0 W3, W2, W1, W0 byte lane 1 B13 B9 B5 B1 byte lane 0W3 B12 W2 B8 W1B4 W0 B0 4 words Empty 1/4 1/2 3/4 Full Source: byte byte lane 3 B15 B 11 B7 B3 Destination: half-word byte lane 2H7 B14 H5 B10 H3B6 H1 B2 B15 B14 B13 B12 B11 B10 B9 B8 B7 B6 B5 B4 B3 B2 B1 B0 H7, H6, H5, H4, H3, H2, H1, H0 byte lane 1 B13 B9 B5 B1 byte lane 0H6 B12 H4 B8 H2B4 H0 B0 4 words Empty 1/4 1/2 3/4 Full byte lane 3 Source: half-word H7 H5 H3 H1 Destination: word byte lane 2 H7 H6 H5 H4 H3 H2 H1 H0 W3, W2, W1, W0 byte lane 1 H6 H4 H2 H0 byte lane 0 W3 W2 W1 W0 4-words Empty 1/4 1/2 3/4 Full byte lane 3 B15 B 11 B7 B3 Source: half-word Destination: byte byte lane 2 H7 B14 H5 B10 H3B6 H1 B2 H7 H6 H5 H4 H3 H2 H1 H0 B15 B14 B13 B12 B11 B10 B9 B8 byte lane 1 B13 B9 B5 B1 B7 B6 B5 B4 B3 B2 B1 B0 byte lane 0 H6 B12 H4 B8 H2B4 H0 B0 ai15951 |


| B15 | B 11 | B7 | B3 |
| --- | --- | --- | --- |
| B14 | B10 | B6 | B2 |
| B13 | B9 | B5 | B1 |
| W3 B12 | W2 B8 | W1B4 | W0 B0 |


| B15 | B 11 | B7 | B3 |
| --- | --- | --- | --- |
| H7 B14 | H5 B10 | H3B6 | H1 B2 |
| B13 | B9 | B5 | B1 |
| H6 B12 | H4 B8 | H2B4 | H0 B0 |


| H7 | H5 | H3 | H1 |
| --- | --- | --- | --- |
| H6 W3 | H4 W2 | H2 W1 | H0 W0 |


| B15 | B 11 | B7 | B3 |
| --- | --- | --- | --- |
| H7 B14 | H5 B10 | H3B6 | H1 B2 |
| B13 | B9 | B5 | B1 |
| H6 B12 | H4 B8 | H2B4 | H0 B0 |

AN4031 DMA controller description
1.1.9 DMA FIFO mode
Each stream has an independent 4-word (4 * 32 bits) FIFO and the threshold level is
software-configurable between 1/4, 1/2, 3/4 or full. The FIFO is used to temporarily store
data coming from the source before transmitting them to the destination.
DMA FIFO can be enabled or disabled by software; when disabled, the Direct mode is used.
If DMA FIFO is enabled, data packing/unpacking and/or Burst mode can be used. The
configured DMA FIFO threshold defines the DMA memory port request time.
The DMA FIFOs implemented on STM32F2/F4/F7 devices help to:
• reduce SRAM access and so give more time for the other masters to access the bus
matrix without additional concurrency,
• allow software to do burst transactions which optimize the transfer bandwidth,
• allow packing/unpacking data to adapt source and destination data width with no extra
DMA access.
Figure 4. FIFO structure
4 words
Empty 1/4 1/2 3/4 Full
byte lane 3 B15 B 11 B7 B3
Source: byte
Destination: word
byte lane 2 B14 B10 B6 B2
B15 B14 B13 B12 B11 B10 B9 B8 B7 B6 B5 B4 B3 B2 B1 B0 W3, W2, W1, W0
byte lane 1 B13 B9 B5 B1
byte lane 0W3 B12 W2 B8 W1B4 W0 B0
4 words
Empty 1/4 1/2 3/4 Full
Source: byte byte lane 3 B15 B 11 B7 B3 Destination: half-word
byte lane 2H7 B14 H5 B10 H3B6 H1 B2
B15 B14 B13 B12 B11 B10 B9 B8 B7 B6 B5 B4 B3 B2 B1 B0 H7, H6, H5, H4, H3, H2, H1, H0
byte lane 1 B13 B9 B5 B1
byte lane 0H6 B12 H4 B8 H2B4 H0 B0
4 words
Empty 1/4 1/2 3/4 Full
byte lane 3
Source: half-word H7 H5 H3 H1 Destination: word
byte lane 2
H7 H6 H5 H4 H3 H2 H1 H0 W3, W2, W1, W0
byte lane 1
H6 H4 H2 H0
byte lane 0 W3 W2 W1 W0
4-words
Empty 1/4 1/2 3/4 Full
byte lane 3 B15 B 11 B7 B3
Source: half-word Destination: byte
byte lane 2 H7 B14 H5 B10 H3B6 H1 B2
H7 H6 H5 H4 H3 H2 H1 H0 B15 B14 B13 B12 B11 B10 B9 B8
byte lane 1 B13 B9 B5 B1 B7 B6 B5 B4 B3 B2 B1 B0
byte lane 0 H6 B12 H4 B8 H2B4 H0 B0
ai15951
DocID022648 Rev 3 11/38
37

<!-- Page 12 -->


|  |
| --- |
|  |
| (cid:68) |
|  |
|  |
|  |


| (cid:58)(cid:50)(cid:53)(cid:39)(cid:3)(cid:20) |
| --- |
| (cid:58)(cid:50)(cid:53)(cid:39)(cid:21) |
| (cid:58)(cid:50)(cid:53)(cid:39)(cid:3)(cid:22) |
| (cid:58)(cid:50)(cid:53)(cid:39)(cid:3)(cid:23) |

DMA controller description AN4031
1.1.10 Source and destination burst size
Burst transfers are guaranteed by the implemented DMA FIFOs.
Figure 5. DMA burst transfer
(cid:51)(cid:72)(cid:85)(cid:76)(cid:83)(cid:75)(cid:72)(cid:85)(cid:68)(cid:79)(cid:3)(cid:85)(cid:72)(cid:84)(cid:88)(cid:72)(cid:86)(cid:87)
(cid:54)(cid:53)(cid:36)(cid:48)
(cid:48)(cid:54)(cid:89)(cid:22)(cid:23)(cid:24)(cid:21)(cid:22)(cid:57)(cid:20)
In response to a burst request from peripheral DMA reads/writes the number of data units
(data unit can be a word, a half-word, or a byte) programmed by the burst size (4x, 8x or 16x
data unit). The burst size on the DMA peripheral port must be set according to the peripheral
needs/capabilities.
The DMA burst size on the memory port and the FIFO threshold configuration must match.
This allows the DMA stream to have enough data in the FIFO when burst transfer on the
memory port is started. Table 3 shows the possible combinations of memory burst size,
FIFO threshold configuration and data size.
To ensure data coherence, each group of transfers that form a burst is indivisible: AHB
transfers are locked and the arbiter of the AHB bus matrix does not remove the DMA
master’s access rights during the burst transfer sequence.
12/38 DocID022648 Rev 3
(cid:3)(cid:72)(cid:69)(cid:3)(cid:82)(cid:87)(cid:3)(cid:86)(cid:85)(cid:72)(cid:87)(cid:86)(cid:76)(cid:74)(cid:72)(cid:85)(cid:3)(cid:73)(cid:82)(cid:3)(cid:85)(cid:72)(cid:69)(cid:80)(cid:88)(cid:49)
(cid:87)(cid:86)(cid:85)(cid:88)(cid:69)(cid:3)(cid:81)(cid:76)(cid:3)(cid:71)(cid:72)(cid:85)(cid:85)(cid:72)(cid:73)(cid:86)(cid:81)(cid:68)(cid:85)(cid:87)
(cid:51)(cid:72)(cid:85)(cid:76)(cid:83)(cid:75)(cid:72)(cid:85)(cid:68)(cid:79)
(cid:39)(cid:48)(cid:36)(cid:3)(cid:54)(cid:87)(cid:85)(cid:72)(cid:68)(cid:80)
(cid:72)(cid:85)
(cid:54)(cid:87)(cid:85)(cid:72)(cid:68)(cid:80)(cid:3)(cid:41)(cid:44)(cid:41)(cid:50) (cid:81)
(cid:86)(cid:73)
(cid:68)
(cid:58)(cid:50)(cid:53)(cid:39)(cid:3)(cid:20) (cid:36)(cid:3)(cid:87)(cid:85)
(cid:68)
(cid:48)
(cid:58)(cid:50)(cid:53)(cid:39)(cid:21) (cid:39)
(cid:86)(cid:87)(cid:3)
(cid:58)(cid:50)(cid:53)(cid:39)(cid:3)(cid:22) (cid:88)(cid:85)
(cid:37)
(cid:58)(cid:50)(cid:53)(cid:39)(cid:3)(cid:23)

<!-- Page 13 -->


| MSIZE | FIFO level | MBURST = INCR4 | MBURST = INCR8 | MBURST = INCR16 |
| --- | --- | --- | --- | --- |
| Byte | 1/4 | 1 burst of 4 bytes | forbidden | forbidden |
|  | 1/2 | 2 bursts of 4 bytes | 1 burst of 8 bytes |  |
|  | 3/4 | 3 bursts of 4 bytes | forbidden |  |
|  | Full | 4 bursts of 4 bytes | 2 bursts of 8 bytes | 1 burst of 16 bytes |
| Half-word | 1/4 | forbidden | forbidden | forbidden |
|  | 1/2 | 1 burst of 4 half-words |  |  |
|  | 3/4 | forbidden |  |  |
|  | Full | 2 bursts of 4 half-words | 1 burst of 8 Half-word |  |
| Word | 1/4 | forbidden | forbidden |  |
|  | 1/2 |  |  |  |
|  | 3/4 |  |  |  |
|  | Full | 1 burst of 4 words |  |  |

AN4031 DMA controller description
Table 3. Possible burst configurations
MSIZE FIFO level MBURST = INCR4 MBURST = INCR8 MBURST = INCR16
1/4 1 burst of 4 bytes forbidden
1/2 2 bursts of 4 bytes 1 burst of 8 bytes forbidden
Byte
3/4 3 bursts of 4 bytes forbidden
Full 4 bursts of 4 bytes 2 bursts of 8 bytes 1 burst of 16 bytes
1/4 forbidden
1/2 1 burst of 4 half-words forbidden
Half-word
3/4 forbidden
Full 2 bursts of 4 half-words 1 burst of 8 Half-word
forbidden
1/4
1/2 forbidden
Word forbidden
3/4
Full 1 burst of 4 words
1.1.11 Double-buffer mode
A double-buffer stream works as a regular (single-buffer) stream, with the difference that it
has two memory pointers. When the Double-buffer mode is enabled, the Circular mode is
automatically enabled and at each end of transaction (DMA_SxNDTR register reach 0), the
memory pointers are swapped.
This allows the software to process one memory area while the second memory area is
being filled/used by the DMA transfer.
Figure 6. Double-buffer mode
(cid:39)(cid:48)(cid:36)(cid:66)(cid:54)(cid:91)(cid:48)(cid:19)(cid:36)(cid:53) (cid:48)(cid:72)(cid:80)(cid:82)(cid:85)(cid:92)(cid:3)(cid:79)(cid:82)(cid:70)(cid:68)(cid:87)(cid:76)(cid:82)(cid:81)(cid:3)(cid:20) (cid:48)(cid:72)(cid:80)(cid:82)(cid:85)(cid:92)(cid:3)(cid:79)(cid:82)(cid:70)(cid:68)(cid:87)(cid:76)(cid:82)(cid:81)(cid:3)(cid:19)
(cid:39)(cid:48)(cid:36)(cid:66)(cid:54)(cid:91)(cid:48)(cid:20)(cid:36)(cid:53)
(cid:39)(cid:48)(cid:36)(cid:66)(cid:54)(cid:91)(cid:51)(cid:36)(cid:53) (cid:38)(cid:55)(cid:3)(cid:32)(cid:3)(cid:20)
(cid:38)(cid:55) (cid:55)(cid:38) (cid:43)(cid:55)
(cid:38)(cid:55)(cid:3)(cid:32)(cid:3)(cid:19)
(cid:51)(cid:72)(cid:85)(cid:76)(cid:83)(cid:75)(cid:72)(cid:85)(cid:68)(cid:79)(cid:3)(cid:71)(cid:68)(cid:87)(cid:68)(cid:3)
(cid:85)(cid:72)(cid:74)(cid:76)(cid:86)(cid:87)(cid:72)(cid:85)
(cid:48)(cid:54)(cid:89)(cid:22)(cid:21)(cid:20)(cid:28)(cid:24)(cid:57)(cid:20)
DocID022648 Rev 3 13/38
37

<!-- Page 14 -->

DMA controller description AN4031
In Double-buffer mode, it is possible to update the base address for the AHB memory port
on-the-fly (DMA_SxM0AR or DMA_SxM1AR) when the stream is enabled:
• When the CT (Current Target) bit in the DMA_SxCR register is equal to 0, the current
DMA memory target is memory location 0 and so the base address memory location 1
(DMA_SxM1AR) can be updated.
• When the CT bit in the DMA_SxCR register is equal to 1, the current DMA memory
target is memory location 1 and so the base address memory location 0
(DMA_SxM0AR) can be updated.
1.1.12 Flow control
The flow controller is the unit that controls the data transfer length and which is responsible
for stopping the DMA transfer.
The flow controller can be either the DMA or the peripheral.
• With DMA as flow controller:
In this case, it is necessary to define the transfer size value in the DMA_SxNDTR register
before enabling the associated DMA stream. When a DMA request is served, the transfer
size value decreases by the amount of transferred data (depending of the type of request:
burst or single).
When the transfer size value reaches 0, the DMA transfer is finished and the DMA stream is
disabled.
• With the peripheral as flow controller:
This is the case when the number of data items to be transferred is unknown. The peripheral
indicates by hardware to the DMA controller when the last data are being transferred. Only
the SD/MMC and JPEG peripherals support this mode.
1.2 Setting up a DMA transfer
To configure DMA stream x (where x is the stream number), the following procedure should
be applied:
1. If the stream is enabled, disable it by resetting the EN bit in the DMA_SxCR register,
then read this bit in order to confirm that there is no ongoing stream operation. Writing
this bit to 0 is not immediately effective since it is actually written to 0 once all the
current transfers have finished. When the EN bit is read as 0, this means that the
stream is ready to be configured. It is therefore necessary to wait for the EN bit to be
cleared before starting any stream configuration. All the stream-dedicated bits set in
14/38 DocID022648 Rev 3

<!-- Page 15 -->

AN4031 DMA controller description
the status register (DMA_LISR and DMA_HISR) from the previous data block DMA
transfer should be cleared before the stream can be re-enabled.
2. Set the peripheral port register address in the DMA_SxPAR register. The data will be
moved from/to this address to/from the peripheral port after the peripheral event.
3. Set the memory address in the DMA_SxMA0R register (and in the DMA_SxMA1R
register in the case of a Double-buffer mode). The data will be written to or read from
this memory after the peripheral event.
4. Configure the total number of data items to be transferred in the DMA_SxNDTR
register. After each peripheral event or each beat of the burst, this value is
decremented.
5. Select the DMA channel (request) using CHSEL[2:0] in the DMA_SxCR register.
6. If the peripheral is intended to be the flow controller and if it supports this feature, set
the PFCTRL bit in the DMA_SxCR register.
7. Configure the stream priority using the PL[1:0] bits in the DMA_SxCR register.
8. Configure the FIFO usage (enable or disable, threshold in transmission and reception).
9. Configure the data transfer direction, peripheral and memory incremented/fixed mode,
single or burst transactions, peripheral and memory data widths, Circular mode,
Double-buffer mode and interrupts after half and/or full transfer, and/or errors in the
DMA_SxCR register.
10. Activate the stream by setting the EN bit in the DMA_SxCR register.
As soon as the stream is enabled, it can serve any DMA request from the peripheral
connected to the stream.
DocID022648 Rev 3 15/38
37

<!-- Page 16 -->

System performance considerations AN4031
2 System performance considerations
STM32F2/F4/F7 devices embed a multi-masters/multi-slaves architecture:
• Multiple masters:
– Cortex®-Mx core AHB buses
– DMA1 memory bus
– DMA2 memory bus
– DMA2 peripheral bus
– Ethernet DMA bus
– USB high-speed DMA bus
– Chrom-ART Accelerator bus
– LCD-TFT bus
• Multiple slaves:
– Internal Flash interfaces connected to the multi-layer bus matrix
– Main internal SRAM1 and Auxiliary internal SRAMs (SRAM2, SRAM3 when
available on device)
– AHB1 peripherals including AHB-to-APB bridges and APB peripherals
– AHB2 peripherals
– AHB3 peripheral (such as FMC, Quad-SPI peripherals when available on product
line)
Masters and slaves are connected via a multi-layer bus matrix ensuring concurrent access
and efficient operation, even when several high-speed peripherals work simultaneously.
This architecture is shown in the next figure for the case of STM32F405/415 and
STM32F407/417 lines.
16/38 DocID022648 Rev 3

<!-- Page 17 -->


|  |  |  |  |
| --- | --- | --- | --- |
|  |  |  |  |
|  |  |  |  |

AN4031 System performance considerations
Figure 7. STM32F405/415 and STM32F407/417 system architecture
(cid:41)(cid:44)(cid:41)(cid:50)(cid:18)(cid:27)(cid:3)(cid:54)(cid:87)(cid:85)(cid:72)(cid:68)(cid:80)(cid:86)
(cid:36)(cid:43)(cid:37)(cid:21)
(cid:54)(cid:53)(cid:36)(cid:48)(cid:20)
(cid:20)(cid:20)(cid:21)(cid:46)(cid:37)
(cid:54)(cid:53)(cid:36)(cid:48)(cid:21)
(cid:20)(cid:25)(cid:46)(cid:37)
(cid:41)(cid:54)(cid:48)(cid:38)
(cid:48)(cid:88)(cid:79)(cid:87)(cid:76)(cid:16)(cid:36)(cid:43)(cid:37)(cid:3)(cid:37)(cid:88)(cid:86)(cid:3)(cid:48)(cid:68)(cid:87)(cid:85)(cid:76)(cid:91)
(cid:48)(cid:54)(cid:22)(cid:22)(cid:20)(cid:28)(cid:24)(cid:57)(cid:20)
2.1 Multi-layer bus matrix
The multi-layer bus matrix allows masters to perform data transfers concurrently as long as
they are addressing different slave modules. On top of the Cortex-Mx architecture and dual
AHB port DMAs, this structure enhances data transfer parallelism, thus contributing to
reduce the execution time, and optimizing the DMA efficiency and power consumption.
2.1.1 Definitions
• AHB master: a bus master is able to initiate read and write operations. Only one master
can win bus ownership at a defined time period.
• AHB slave: a bus slave response to master read or write operations. The bus slave
signals back to master success, failure or waiting states.
• AHB arbiter: a bus arbiter insures that only one master can initiate a read or write
operation at one time.
• AHB bus matrix: a multi-layer AHB bus matrix that interconnects AHB masters to AHB
slaves with dedicated AHB arbiter for each layer. The arbitration uses a round-robin
algorithm.
DocID022648 Rev 3 17/38
37
(cid:36)(cid:53)(cid:55)
(cid:36)(cid:70)(cid:70)(cid:72)(cid:79)(cid:72)(cid:85)(cid:68)(cid:87)(cid:82)(cid:85)
(cid:38)(cid:50)(cid:53)(cid:55)(cid:40)(cid:59)(cid:48)(cid:23)
(cid:20)(cid:25)(cid:27)(cid:48)(cid:43)(cid:93)
(cid:90)(cid:18)(cid:3)(cid:41)(cid:51)(cid:56)(cid:3)(cid:9)(cid:3)(cid:48)(cid:51)(cid:56)
(cid:48)(cid:88)(cid:79)(cid:87)(cid:76)(cid:16)(cid:36)(cid:43)(cid:37)(cid:37)(cid:88)(cid:86)(cid:48)(cid:68)(cid:87)(cid:85)(cid:76)(cid:91)
(cid:39)(cid:16)(cid:37)(cid:88)(cid:86)
(cid:40)(cid:87)(cid:75)(cid:72)(cid:85)(cid:81)(cid:72)(cid:87)(cid:3) (cid:43)(cid:76)(cid:74)(cid:75)(cid:3)(cid:54)(cid:83)(cid:72)(cid:72)(cid:71) (cid:39)(cid:88)(cid:68)(cid:79)(cid:3)(cid:51)(cid:82)(cid:85)(cid:87) (cid:39)(cid:88)(cid:68)(cid:79)(cid:3)(cid:51)(cid:82)(cid:85)(cid:87)
(cid:38)(cid:38)(cid:48)(cid:3) (cid:20)(cid:19)(cid:18)(cid:20)(cid:19)(cid:19) (cid:56)(cid:54)(cid:37)(cid:21)(cid:17)(cid:19) (cid:39)(cid:48)(cid:36)(cid:20) (cid:39)(cid:48)(cid:36)(cid:21)
(cid:71)(cid:68)(cid:87)(cid:68)(cid:3)(cid:53)(cid:36)(cid:48)
(cid:25)(cid:23)(cid:46)(cid:37) (cid:48)(cid:68)(cid:86)(cid:87)(cid:72)(cid:85) (cid:48)(cid:68)(cid:86)(cid:87)(cid:72)(cid:3)(cid:85) (cid:48)(cid:68)(cid:86)(cid:87)(cid:72)(cid:85) (cid:48)(cid:68)(cid:86)(cid:87)(cid:72)(cid:85)
(cid:48)(cid:68)(cid:86)(cid:87)(cid:72)(cid:85)
(cid:41)(cid:44)(cid:41)(cid:50)(cid:18)(cid:39)(cid:48)(cid:36)
(cid:39)(cid:88)(cid:68)(cid:79)(cid:3)(cid:51)(cid:82)(cid:85)(cid:87)
(cid:36)(cid:43)(cid:37)(cid:20)(cid:16)(cid:36)(cid:51)(cid:37)(cid:21)
(cid:36)(cid:43)(cid:37)(cid:20)
(cid:16)(cid:36)(cid:51)(cid:37)(cid:20)
(cid:44)(cid:16)(cid:38)(cid:82)(cid:71)(cid:72)
(cid:41)(cid:47)(cid:36)(cid:54)(cid:43)
(cid:20)(cid:48)(cid:69)(cid:92)(cid:87)(cid:72)(cid:86)
(cid:39)(cid:16)(cid:38)(cid:82)(cid:71)(cid:72)
(cid:40)(cid:87)(cid:75)(cid:72)(cid:85)(cid:81)(cid:72)(cid:87) (cid:56)(cid:54)(cid:37)(cid:66)(cid:43)(cid:54) (cid:39)(cid:48)(cid:36)(cid:20) (cid:48)(cid:40)(cid:48) (cid:39)(cid:48)(cid:36)(cid:20) (cid:51) (cid:39)(cid:48)(cid:36)(cid:21) (cid:48)(cid:40)(cid:48) (cid:39)(cid:48)(cid:36)(cid:21) (cid:51)
(cid:41)(cid:44)(cid:41)(cid:50)(cid:18)(cid:39)(cid:48)(cid:36) (cid:41)(cid:44)(cid:41)(cid:50)(cid:18)(cid:27)(cid:3)(cid:54)(cid:87)(cid:85)(cid:72)(cid:68)(cid:80)(cid:86)
(cid:39)(cid:88)(cid:68)(cid:79)(cid:3)(cid:51)(cid:82)(cid:85)(cid:87)
(cid:36)(cid:43)(cid:37)(cid:20)(cid:16)(cid:36)(cid:51)(cid:37)(cid:20)
(cid:44)(cid:16)(cid:37)(cid:88)(cid:86) (cid:54)(cid:16)(cid:37)(cid:88)(cid:86)

<!-- Page 18 -->


| (cid:41)(cid:44)(cid:41) | (cid:50)(cid:18)(cid:27)(cid:3)(cid:54)(cid:87)(cid:85)(cid:72)(cid:68)(cid:80)(cid:86) |
| --- | --- |

System performance considerations AN4031
2.1.2 Round-robin priority scheme
A round-robin priority scheme is implemented at bus matrix level in order to ensure that
each master can access any slave with very low latency:
• Round-robin arbitration policy allows a fair distribution of bus bandwidth.
• Maximum latency is bounded.
• Round-robin quantum is 1x transfer.
Bus matrix arbiters intervene to solve access conflicts when several AHB masters try to
access the same AHB slave simultaneously.
In the following example (Figure 8), both the CPU and DMA1 try to access SRAM1 to read
data.
Figure 8. CPU and DMA1 request an access to SRAM1
(cid:48)(cid:54)(cid:22)(cid:22)(cid:20)(cid:28)(cid:25)(cid:57)(cid:20)
In case of bus access concurrency as in the above example, a bus matrix arbitration is
required. The round-robin policy is then applied in order to solve the issue: if the last master
which wan the bus was the CPU, during the next access DMA1 wins the bus and accesses
SRAM1 first. The CPU has then the rights to access SRAM1.
This proves that the transfer latency associated to one master depends on the number of
other pending master requests to access the same AHB slave. In the following example
(Figure 9), five masters try to access simultaneously SRAM1.
18/38 DocID022648 Rev 3
(cid:16) (cid:36)(cid:43)(cid:37)(cid:20)(cid:16)(cid:36)(cid:51)(cid:37)(cid:21)
(cid:36)(cid:43)(cid:37)(cid:20)(cid:16)(cid:36)(cid:51)(cid:37)(cid:20)
(cid:39)(cid:88)(cid:68)(cid:79)(cid:3)(cid:51)(cid:82)(cid:85)(cid:87)
(cid:39)(cid:88)(cid:68)(cid:79)(cid:3)(cid:51)(cid:82)(cid:85)(cid:87)
(cid:38)(cid:50)(cid:53)(cid:55)(cid:40)(cid:59)(cid:16)(cid:48)(cid:23) (cid:39)(cid:88)(cid:68)(cid:79)(cid:3)(cid:51)(cid:82)(cid:85)(cid:87)
(cid:20)(cid:25)(cid:27)(cid:48)(cid:43)(cid:93) (cid:39)(cid:48)(cid:36)(cid:20)
(cid:41)(cid:51)(cid:56)(cid:3)(cid:9)(cid:3)(cid:48)(cid:51)(cid:56) (cid:48)(cid:68)(cid:86)(cid:87)(cid:72)(cid:85)(cid:3)
(cid:48)(cid:68)(cid:86)(cid:87)(cid:72)(cid:85)(cid:3)
(cid:41)(cid:44)(cid:41)(cid:50)(cid:18)(cid:27)(cid:3)(cid:54)(cid:87)(cid:85)(cid:72)(cid:68)(cid:80)(cid:86)
(cid:36)(cid:43)(cid:37)(cid:20)
(cid:36)(cid:43)(cid:37)(cid:21)
(cid:54)(cid:53)(cid:36)(cid:48)(cid:20)
(cid:20)(cid:20)(cid:21)(cid:46)(cid:37)
(cid:88)(cid:86)(cid:3)(cid:48)(cid:68)(cid:87)(cid:85)(cid:76)(cid:91)
(cid:39)(cid:16)(cid:37)(cid:88)(cid:86) (cid:44)(cid:16)(cid:37)(cid:88)(cid:86) (cid:54)(cid:16)(cid:37)(cid:88)(cid:86) (cid:48)(cid:40)(cid:48) (cid:39)(cid:48)(cid:36)(cid:20) (cid:51) (cid:39)(cid:48)(cid:36)(cid:20)
(cid:88)(cid:86)(cid:48)(cid:68)(cid:87)(cid:85)(cid:76)(cid:91)

<!-- Page 19 -->


|  |  |  |  | (cid:28)(cid:100)(cid:44) |  |  |
| --- | --- | --- | --- | --- | --- | --- |
|  |  |  |  |  |  |  |
|  |  |  | (cid:104)(cid:94)(cid:17)(cid:890)(cid:44)(cid:94) |  |  |  |
|  |  |  |  |  |  |  |
|  |  | (cid:24)(cid:68)(cid:4)(cid:1006) |  |  |  |  |
|  |  |  |  |  |  |  |
|  | (cid:24)(cid:68)(cid:4)(cid:1005) |  |  |  |  | (cid:24)(cid:68)(cid:4)(cid:1005) |
|  |  |  |  |  |  |  |
| (cid:18)(cid:87)(cid:104) |  |  |  |  | (cid:18)(cid:87)(cid:104) |  |

AN4031 System performance considerations
Figure 9. Five masters request SRAM access
(cid:52)(cid:88)(cid:68)(cid:81)(cid:87)(cid:88)(cid:80)(cid:3)(cid:32)(cid:3)
(cid:20)(cid:3)(cid:36)(cid:43)(cid:37)(cid:3)(cid:70)(cid:92)(cid:70)(cid:79)(cid:72)
(cid:28)(cid:100)(cid:44)
(cid:104)(cid:94)(cid:17)(cid:890)(cid:44)(cid:94)
(cid:24)(cid:68)(cid:4)(cid:1006)
(cid:24)(cid:68)(cid:4)(cid:1005) (cid:24)(cid:68)(cid:4)(cid:1005)
(cid:18)(cid:87)(cid:104) (cid:18)(cid:87)(cid:104)
(cid:48)(cid:54)(cid:89)(cid:22)(cid:21)(cid:20)(cid:28)(cid:27)(cid:57)(cid:20)
The latency associated to DMA1 to win the bus matrix again and access SRAM1 (for
example) is equal to the execution time of all pending requests coming from the other
masters.
2.1.3 BusMatrix arbitration and DMA transfer delays worst case
The latency seen by the DMA master port on one transaction depends on the other masters’
transfer types and lengths.
For instance, if we consider previous DMA1 & CPU example (Figure 8) with concurrency to
access SRAM, latency on the DMA transfer varies depending on the CPU transaction
length.
If bus access is first granted to the CPU and the CPU is not performing a single data
load/store, the DMA wait time to gain access to SRAM can expand from one AHB cycle for
a single data load/store to N AHB cycles, where N is the number of data words in the CPU
transaction.
The CPU locks the AHB bus to keep ownership and reduces latency during multiple
load/store operations and interrupts entry. This enhances firmware responsiveness but it
can result in delays on the DMA transaction.
Delay on DMA1 SRAM access when in concurrency with CPU depends on the type of
transfer:
• CPU transfer issued by interrupt (context save): 8 AHB cycles
• CPU transfer issued by cache controller (256-bit cache line fill/eviction): 8 AHB
cycles(a)
• CPU transfer issued by LDM/STM instructions: 14 AHB cycles(b)
– Transfers of up to 14 registers from/to memory
DocID022648 Rev 3 19/38
37
(cid:48)(cid:36)(cid:53)(cid:54)(cid:3)(cid:74)(cid:81)(cid:76)(cid:86)(cid:86)(cid:72)(cid:70)(cid:70)(cid:68)(cid:3)(cid:85)(cid:72)(cid:87)(cid:86)(cid:68)(cid:48)
(cid:39)(cid:48)(cid:36)(cid:20)(cid:3)(cid:79)(cid:68)(cid:87)(cid:72)(cid:81)(cid:70)(cid:92) (cid:55)(cid:76)(cid:80)(cid:72)
a. Only for STM32F7xx devices
b. Latency due to transfer issued by LDM/STM instructions can be reduced by configuring compiler to split
load/store multiple instructions into single load/store instructions.

<!-- Page 20 -->

System performance considerations AN4031
Figure 10. DMA transfer delay due to CPU transfer issued by interrupt
(cid:39)(cid:48)(cid:36)(cid:20) (cid:36)(cid:53)(cid:37) (cid:39)(cid:72)(cid:79)(cid:68)(cid:92) (cid:39)(cid:48)(cid:36)(cid:20)(cid:3)
(cid:38)(cid:51)(cid:56) (cid:38)(cid:48)(cid:16)(cid:91)
(cid:39)(cid:48)(cid:36)(cid:20)(cid:3)
(cid:48)(cid:72)(cid:48)(cid:85)(cid:72)(cid:84)
(cid:39)(cid:72)(cid:79)(cid:68)(cid:92)(cid:3)(cid:87)(cid:82)(cid:3)(cid:86)(cid:72)(cid:85)(cid:89)(cid:72)(cid:3)(cid:87)(cid:75)(cid:72)(cid:3)(cid:85)(cid:72)(cid:84)(cid:88)(cid:72)(cid:86)(cid:87)
(cid:48)(cid:54)(cid:22)(cid:23)(cid:21)(cid:20)(cid:23)(cid:57)(cid:20)
The above figure details the case of a DMA transfer delayed by a CPU multi-cycle transfer
due to an interrupt entry. DMA memory port is triggered to perform a memory access. After
arbitration, AHB bus is not granted to DMA1 memory port but to CPU. An additional delay is
observed to serve the DMA request. It is 8 AHB cycles for a CPU transfer issued by
interrupt.
The same behavior can be observed with other masters (like DMA2, USB_HS, Ethernet…)
when addressing simultaneously the same slave with a transaction length different from one
data unit.
In order to improve DMA access performance over BusMatrix, it is recommended to avoid
bus contention.
2.2 DMA transfer paths
2.2.1 Dual DMA port
STM32F2/F4/F7 devices embed two DMAs. Each DMA has two ports, a memory port and a
peripheral port, which can operate simultaneously not only at DMA level but also with other
system masters, using the external bus matrix and dedicated DMA paths.
The simultaneous operation allows to optimize DMA efficiency and to reduce response time
(wait time between request and data transfer).
20/38 DocID022648 Rev 3

<!-- Page 21 -->


|  |
| --- |
| (cid:43)(cid:37)(cid:3)(cid:37) |


|  |  |
| --- | --- |
|  |  |

AN4031 System performance considerations
Figure 11. DMA dual port
(cid:41)(cid:44)(cid:41)(cid:50)(cid:18)(cid:27)(cid:3)(cid:54)(cid:87)(cid:85)(cid:72)(cid:68)(cid:80)(cid:86)
(cid:36)(cid:43)(cid:37)(cid:21)
(cid:54)(cid:53)(cid:36)(cid:48)(cid:20)
(cid:20)(cid:20)(cid:21)(cid:46)(cid:37)
(cid:54)(cid:53)(cid:36)(cid:48)(cid:21)
(cid:20)(cid:25)(cid:46)(cid:37)
(cid:41)(cid:54)(cid:48)(cid:38)
(cid:48)(cid:88)(cid:79)(cid:87)(cid:76)(cid:16)(cid:36)(cid:43)(cid:37)(cid:3)(cid:37)(cid:88)(cid:86)(cid:3)(cid:48)(cid:68)(cid:87)(cid:85)(cid:76)(cid:91)
(cid:48)(cid:54)(cid:22)(cid:22)(cid:20)(cid:28)(cid:26)(cid:57)(cid:20)
For DMA2:
• The MEM (memory port) can access AHB1, AHB2, AHB3 (External memory controller,
FSMC), SRAMs, and Flash memory through the bus matrix.
• The Periph (peripheral port) can access:
– AHB1, AHB2, AHB3 (External memory controller, FSMC), SRAMs, and Flash
memory through the bus matrix,
– the AHB-to-APB2 bridge through a direct path (not crossing the bus matrix).
For DMA1:
• The MEM (memory port) can access AHB3 (External memory controller, FSMC),
SRAMs, and Flash memory through the bus matrix.
• The Periph (peripheral port) can only access the AHB-to-APB1 bridge through a direct
path (not crossing the bus matrix).
DocID022648 Rev 3 21/38
37
(cid:36)(cid:53)(cid:55)
(cid:36)(cid:70)(cid:70)(cid:72)(cid:79)(cid:72)(cid:85)(cid:68)(cid:87)(cid:82)(cid:85)
(cid:38)(cid:50)(cid:53)(cid:55)(cid:40)(cid:59)(cid:48)(cid:23)
(cid:20)(cid:25)(cid:27)(cid:48)(cid:43)(cid:93)
(cid:90)(cid:18)(cid:3)(cid:41)(cid:51)(cid:56)(cid:3)(cid:9)(cid:3)(cid:48)(cid:51)(cid:56)
(cid:39)(cid:16)(cid:37)(cid:88)(cid:86)
(cid:39)(cid:88)(cid:68)(cid:79)(cid:3)(cid:51)(cid:82)(cid:85)(cid:87) (cid:39)(cid:88)(cid:68)(cid:79)(cid:3)(cid:51)(cid:82)(cid:85)(cid:87)
(cid:38)(cid:38)(cid:48)(cid:3) (cid:39)(cid:48)(cid:36)(cid:20) (cid:39)(cid:48)(cid:36)(cid:21)
(cid:71)(cid:68)(cid:87)(cid:68)(cid:3)(cid:53)(cid:36)(cid:48)
(cid:25)(cid:23)(cid:46)(cid:37) (cid:3) (cid:48)(cid:68)(cid:86)(cid:87)(cid:72)(cid:85) (cid:48)(cid:68)(cid:86)(cid:87)(cid:72)(cid:85)
(cid:48)(cid:68)(cid:86)(cid:87)(cid:72)(cid:85)
(cid:39)(cid:88)(cid:68)(cid:79)(cid:3)(cid:51)(cid:82)(cid:85)(cid:87)
(cid:36)(cid:43)(cid:37)(cid:20)(cid:16)(cid:36)(cid:51)(cid:37)(cid:21)
(cid:36)(cid:43)(cid:37)(cid:20)
(cid:16)(cid:36)(cid:51)(cid:37)(cid:20)
(cid:44)(cid:16)(cid:38)(cid:82)(cid:71)(cid:72)
(cid:41)(cid:47)(cid:36)(cid:54)(cid:43)
(cid:20)(cid:48)(cid:69)(cid:92)(cid:87)(cid:72)(cid:86)
(cid:39)(cid:16)(cid:38)(cid:82)(cid:71)(cid:72)
(cid:39)(cid:48)(cid:36)(cid:20) (cid:48)(cid:40)(cid:48) (cid:39)(cid:48)(cid:36)(cid:20) (cid:51) (cid:39)(cid:48)(cid:36)(cid:21) (cid:48)(cid:40)(cid:48) (cid:39)(cid:48)(cid:36)(cid:21) (cid:51)
(cid:41)(cid:44)(cid:41)(cid:50)(cid:18)(cid:27)(cid:3)(cid:54)(cid:87)(cid:85)(cid:72)(cid:68)(cid:80)(cid:86)
(cid:39)(cid:88)(cid:68)(cid:79)(cid:3)(cid:51)(cid:82)(cid:85)(cid:87)
(cid:36)(cid:43)(cid:37)(cid:20)(cid:16)(cid:36)(cid:51)(cid:37)(cid:20)
(cid:44)(cid:16)(cid:37)(cid:88)(cid:86) (cid:54)(cid:16)(cid:37)(cid:88)(cid:86)

<!-- Page 22 -->

System performance considerations AN4031
2.2.2 DMA transfer states
This section explains the DMA transfer steps at the peripheral port level and also at the
memory port level:
• For a peripheral-to-memory transfer:
In this transfer mode, DMA requires two bus accesses to perform the transfer:
– One access over the peripheral port triggered by the peripheral’s request,
– One access over the memory port which can be triggered either by the FIFO
threshold (when FIFO mode is used) or immediately after peripheral read (when
Direct mode is used).
Figure 12. Peripheral-to-memory transfer states
(cid:49)(cid:72)(cid:90)(cid:3) (cid:53)(cid:72)(cid:84)(cid:88)(cid:72)(cid:86)(cid:87)(cid:3) (cid:55)(cid:85)(cid:68)(cid:81)(cid:86)(cid:73)(cid:72)(cid:85)(cid:3)
(cid:85)(cid:72)(cid:84)(cid:88)(cid:72)(cid:86)(cid:87) (cid:68)(cid:85)(cid:69)(cid:76)(cid:87)(cid:85)(cid:68)(cid:87)(cid:76)(cid:82)(cid:81) (cid:71)(cid:68)(cid:87)(cid:68)
(cid:54)(cid:87)(cid:82)(cid:85)(cid:72)(cid:3)(cid:71)(cid:68)(cid:87)(cid:68)(cid:3)
(cid:76)(cid:81)(cid:3)(cid:41)(cid:44)(cid:41)(cid:50)
(cid:51)(cid:72)(cid:85)(cid:76)(cid:83)(cid:75)(cid:72)(cid:85)(cid:68)(cid:79)(cid:3)(cid:86)(cid:76)(cid:71)(cid:72)
(cid:49)(cid:72)(cid:90)(cid:3)(cid:41)(cid:44)(cid:41)(cid:50)(cid:3)
(cid:85)(cid:72)(cid:84)(cid:88)(cid:72)(cid:86)(cid:87)
(cid:53)(cid:72)(cid:84)(cid:88)(cid:72)(cid:86)(cid:87)(cid:3) (cid:55)(cid:85)(cid:68)(cid:81)(cid:86)(cid:73)(cid:72)(cid:85)(cid:3) (cid:40)(cid:81)(cid:71)(cid:3)(cid:82)(cid:73)(cid:3)
(cid:68)(cid:85)(cid:69)(cid:76)(cid:87)(cid:85)(cid:68)(cid:87)(cid:76)(cid:82)(cid:81) (cid:71)(cid:68)(cid:87)(cid:68) (cid:87)(cid:85)(cid:68)(cid:81)(cid:86)(cid:73)(cid:72)(cid:85)
(cid:48)(cid:72)(cid:80)(cid:82)(cid:85)(cid:92)(cid:3)(cid:86)(cid:76)(cid:71)(cid:72)
(cid:48)(cid:54)(cid:89)(cid:22)(cid:21)(cid:20)(cid:28)(cid:25)(cid:57)(cid:21)
• For a memory-to-peripheral transfer:
In this transfer mode, DMA requires two bus accesses to perform the transfer:
– DMA anticipates the peripheral’s access and reads data from the memory and
stores it in FIFO to ensure an immediate data transfer as soon as a DMA
peripheral request is triggered.
– When a peripheral request is triggered, a transfer is generated on the DMA
peripheral port.
22/38 DocID022648 Rev 3

<!-- Page 23 -->


| (cid:53)(cid:72)(cid:84)(cid:88)(cid:72)(cid:86)(cid:87)(cid:3)(cid:20) (cid:86)(cid:87)(cid:68)(cid:87)(cid:88)(cid:86) |
| --- |
| (cid:53)(cid:72)(cid:84)(cid:88)(cid:72)(cid:86)(cid:87)(cid:3)(cid:21) (cid:86)(cid:87)(cid:68)(cid:87)(cid:88)(cid:86) |
| (cid:39)(cid:48)(cid:36) (cid:68)(cid:85)(cid:69)(cid:76)(cid:87)(cid:85)(cid:68)(cid:87)(cid:76)(cid:82)(cid:81) |
| (cid:39)(cid:48)(cid:36) (cid:87)(cid:85)(cid:68)(cid:81)(cid:86)(cid:73)(cid:72)(cid:85) |


| (cid:53)(cid:21)(cid:3)(cid:68) | (cid:70)(cid:87)(cid:76)(cid:89)(cid:72) |
| --- | --- |

AN4031 System performance considerations
Figure 13. Memory-to-peripheral transfer states
(cid:49)(cid:72)(cid:90)(cid:3) (cid:53)(cid:72)(cid:84)(cid:88)(cid:72)(cid:86)(cid:87)(cid:3) (cid:55)(cid:85)(cid:68)(cid:81)(cid:86)(cid:73)(cid:72)(cid:85)(cid:3)
(cid:85)(cid:72)(cid:84)(cid:88)(cid:72)(cid:86)(cid:87) (cid:68)(cid:85)(cid:69)(cid:76)(cid:87)(cid:85)(cid:68)(cid:87)(cid:76)(cid:82)(cid:81) (cid:71)(cid:68)(cid:87)(cid:68)
(cid:54)(cid:87)(cid:82)(cid:85)(cid:72)(cid:3)(cid:71)(cid:68)(cid:87)(cid:68)(cid:3)
(cid:76)(cid:81)(cid:3)(cid:41)(cid:44)(cid:41)(cid:50)
(cid:48)(cid:72)(cid:80)(cid:82)(cid:85)(cid:92)(cid:3)(cid:86)(cid:76)(cid:71)(cid:72)
(cid:49)(cid:72)(cid:90)(cid:3)(cid:41)(cid:44)(cid:41)(cid:50)(cid:3)
(cid:85)(cid:72)(cid:84)(cid:88)(cid:72)(cid:86)(cid:87)
(cid:53)(cid:72)(cid:84)(cid:88)(cid:72)(cid:86)(cid:87)(cid:3) (cid:55)(cid:85)(cid:68)(cid:81)(cid:86)(cid:73)(cid:72)(cid:85)(cid:3) (cid:40)(cid:81)(cid:71)(cid:3)(cid:82)(cid:73)(cid:3)
(cid:68)(cid:85)(cid:69)(cid:76)(cid:87)(cid:85)(cid:68)(cid:87)(cid:76)(cid:82)(cid:81) (cid:71)(cid:68)(cid:87)(cid:68) (cid:87)(cid:85)(cid:68)(cid:81)(cid:86)(cid:73)(cid:72)(cid:85)
(cid:51)(cid:72)(cid:85)(cid:76)(cid:83)(cid:75)(cid:72)(cid:85)(cid:68)(cid:79)(cid:3)(cid:86)(cid:76)(cid:71)(cid:72)
(cid:48)(cid:54)(cid:89)(cid:22)(cid:21)(cid:20)(cid:28)(cid:26)(cid:57)(cid:21)
2.2.3 DMA request arbitration
As described in Section 1.1.2: Stream priority, the STM32F2/F4/F7 DMA embeds an arbiter
that manages the eight DMA stream requests based on their priorities for each of the two
AHB master ports (memory and peripheral ports) and launches the peripheral/memory
access sequences.
When more than one DMA request is active, DMA needs to arbiter internally between the
active requests and decide which request is to be served first.
The following figure shows two circular DMA requests triggered at the same time by DMA
stream “request 1” and by DMA stream “request 2” (requests 1 and 2 could be any DMA
peripheral request). At the next AHB clock cycle, the DMA arbiter checks on the active
pending requests and grants access to the “request 1” stream which has the highest priority.
The next arbitration cycle occurs during the last data cycle of the “request 1” stream. At that
time, “request 1” is masked and the arbiter sees only “request 2” as active, so access is
reserved to “request 2” this time, and so on.
Figure 14. DMA request arbitration
(cid:53)(cid:72)(cid:84)(cid:88)(cid:72)(cid:86)(cid:87)(cid:3)(cid:20) (cid:49)(cid:82)(cid:87)(cid:3)(cid:68)(cid:70)(cid:87)(cid:76)(cid:89)(cid:72) (cid:87)(cid:85)(cid:76)(cid:74) (cid:53) (cid:74)(cid:72) (cid:20) (cid:85)(cid:72)(cid:71) (cid:53)(cid:20)(cid:3)(cid:68)(cid:70)(cid:87)(cid:76)(cid:89)(cid:72) (cid:80)(cid:68) (cid:53) (cid:86) (cid:20) (cid:78)(cid:72) (cid:3) (cid:71) (cid:53)(cid:20)(cid:3)(cid:68)(cid:70)(cid:87)(cid:76)(cid:89)(cid:72) (cid:80)(cid:68) (cid:53) (cid:86) (cid:20) (cid:78)(cid:72) (cid:3) (cid:71)
(cid:86)(cid:87)(cid:68)(cid:87)(cid:88)(cid:86)
(cid:53)(cid:72)(cid:84)(cid:88)(cid:72)(cid:86)(cid:87)(cid:3)(cid:21)
(cid:86)(cid:87)(cid:68)(cid:87)(cid:88)(cid:86) (cid:49)(cid:82)(cid:87)(cid:3)(cid:68)(cid:70)(cid:87)(cid:76)(cid:89)(cid:72) (cid:87)(cid:85)(cid:76)(cid:74) (cid:53) (cid:74)(cid:72) (cid:21) (cid:85)(cid:72)(cid:71) (cid:53)(cid:21)(cid:3)(cid:68)(cid:70)(cid:87)(cid:76)(cid:89)(cid:72) (cid:80)(cid:68) (cid:53) (cid:86) (cid:21) (cid:78)(cid:72) (cid:3) (cid:71) (cid:53)(cid:21)(cid:3)(cid:68)(cid:70)(cid:87)(cid:76)(cid:89)(cid:72)
(cid:39)(cid:48)(cid:36)
(cid:39)(cid:48)(cid:36)(cid:3) (cid:39)(cid:48)(cid:36)(cid:3) (cid:39)(cid:48)(cid:36)(cid:3) (cid:39)(cid:48)(cid:36)(cid:3)
(cid:68)(cid:85)(cid:69)(cid:76)(cid:87)(cid:85)(cid:68)(cid:87)(cid:76)(cid:82)(cid:81) (cid:68)(cid:85)(cid:69)(cid:76)(cid:87)(cid:85)(cid:68)(cid:87)(cid:76)(cid:82)(cid:81) (cid:68)(cid:85)(cid:69)(cid:76)(cid:87)(cid:85)(cid:68)(cid:87)(cid:76)(cid:82)(cid:81) (cid:68)(cid:85)(cid:69)(cid:76)(cid:87)(cid:85)(cid:68)(cid:87)(cid:76)(cid:82)(cid:81) (cid:68)(cid:85)(cid:69)(cid:76)(cid:87)(cid:85)(cid:68)(cid:87)(cid:76)(cid:82)(cid:81)
(cid:39)(cid:48)(cid:36)
(cid:87)(cid:85)(cid:68)(cid:81)(cid:86)(cid:73)(cid:72)(cid:85) (cid:39)(cid:48)(cid:36)(cid:3)(cid:86)(cid:72)(cid:85)(cid:89)(cid:76)(cid:70)(cid:76)(cid:81)(cid:74)(cid:3)(cid:53)(cid:20) (cid:39)(cid:48)(cid:36)(cid:3)(cid:86)(cid:72)(cid:85)(cid:89)(cid:76)(cid:70)(cid:76)(cid:81)(cid:74)(cid:3)(cid:53)(cid:21) (cid:39)(cid:48)(cid:36)(cid:3)(cid:86)(cid:72)(cid:85)(cid:89)(cid:76)(cid:70)(cid:76)(cid:81)(cid:74)(cid:3)(cid:53)(cid:20)
(cid:48)(cid:54)(cid:89)(cid:22)(cid:21)(cid:20)(cid:28)(cid:28)(cid:57)(cid:21)
DocID022648 Rev 3 23/38
37

<!-- Page 24 -->

System performance considerations AN4031
General recommendations:
• The high-speed/high-bandwidth peripherals must have the highest DMA priorities. This
ensures that the maximum data latency is respected for these peripherals and over-
/under-run conditions are avoided.
• In case of equal bandwidth requirements, it is recommended to assign a higher priority
to the peripherals working in Slave mode (which have no control on the data transfer
speed) compared with the ones working in Master mode (which may control the data
flow).
• As the two DMAs can work in parallel based on the bus matrix multi-layer structure,
high-speed peripherals’ requests can be balanced between the two DMAs when
possible.
2.3 AHB-to-APB bridge
STM32F2/F4/F7 devices embed two AHB-to-APB bridges, APB1 and APB2, to which the
peripherals are connected.
2.3.1 Dual AHB-to-APB port
The AHB-to-APB bridge is a dual-port architecture that allows access through two different
paths:
• A direct path (not crossing the bus matrix) that can be generated from DMA1 to APB1
or from DMA2 to APB2; in this case, access is not penalized by the bus matrix arbiter.
• A common path (through the bus matrix) that can be generated either from the CPU or
from DMA2, which needs the bus matrix arbitration to win the bus.
2.3.2 AHB-to-APB bridge arbitration
Due to DMA’s direct paths implementation on these products, an arbiter is implemented at
the AHB-to-APB bridge level to solve concurrent access requests.
The following figure illustrates a concurrent access request at an AHB-APB1 bridge
generated by the CPU (accessed through the bus matrix) and DMA1 (accessed through
direct path).
24/38 DocID022648 Rev 3

<!-- Page 25 -->

AN4031 System performance considerations
Figure 15. AHB-to-APB1 bridge concurrent CPU and DMA1 access request
(cid:36)(cid:43)(cid:37)(cid:21)
(cid:54)(cid:53)(cid:36)(cid:48)(cid:20)
(cid:20)(cid:20)(cid:21)(cid:46)(cid:37)
(cid:48)(cid:88)(cid:79)(cid:87)(cid:76)(cid:16)(cid:36)(cid:43)(cid:37)(cid:3)(cid:37)(cid:88)(cid:86)(cid:3)(cid:48)(cid:68)(cid:87)(cid:85)(cid:76)(cid:91)
(cid:48)(cid:54)(cid:22)(cid:22)(cid:20)(cid:28)(cid:27)(cid:57)(cid:20)
To grant bus access, the AHB-APB bridge applies the round-robin policy:
• Round-robin quantum is 1x APB transfer.
• Max latency on DMA peripheral port is bounded (1 APB transfer).
Only the CPU and DMAs can generate a concurrent access to the APB1 and APB2 buses:
• For APB1, a concurrent access can be generated if the CPU, DMA1 and/or DMA2
request simultaneous access.
• For APB2, a concurrent access can be generated if the CPU and DMA2 request
simultaneous access.
DocID022648 Rev 3 25/38
37
(cid:39)(cid:16)(cid:37)(cid:88)(cid:86)
(cid:39)(cid:88)(cid:68)(cid:79)(cid:3)(cid:51)(cid:82)(cid:85)(cid:87)
(cid:39)(cid:48)(cid:36)(cid:20)
(cid:48)(cid:68)(cid:86)(cid:87)(cid:72)(cid:85)
(cid:39)(cid:48)(cid:36)(cid:20) (cid:48)(cid:40)(cid:48) (cid:39)(cid:48)(cid:36)(cid:20) (cid:51)
(cid:41)(cid:44)(cid:41)(cid:50)(cid:18)(cid:27)(cid:3)(cid:54)(cid:87)(cid:85)(cid:72)(cid:68)(cid:80)(cid:86)
(cid:44)(cid:16)(cid:37)(cid:88)(cid:86) (cid:54)(cid:16)(cid:37)(cid:88)(cid:86)
(cid:38)(cid:50)(cid:53)(cid:55)(cid:40)(cid:59)(cid:48)(cid:23)
(cid:20)(cid:25)(cid:27)(cid:48)(cid:43)(cid:93)
(cid:90)(cid:18)(cid:3)(cid:41)(cid:51)(cid:56)(cid:3)(cid:9)(cid:3)(cid:48)(cid:51)(cid:56)
(cid:3)
(cid:48)(cid:68)(cid:86)(cid:87)(cid:72)(cid:85)
(cid:36)(cid:43)(cid:37)(cid:20) (cid:36)(cid:43)(cid:37)(cid:20)(cid:16)(cid:36)(cid:51)(cid:37)(cid:20)
(cid:39)(cid:88)(cid:68)(cid:79)(cid:3)(cid:51)(cid:82)(cid:85)(cid:87)

<!-- Page 26 -->

How to predict DMA latencies AN4031
3 How to predict DMA latencies
When designing a firmware application based on a microcontroller, the user must ensure
that no underrun/overrun can occur, and that’s why knowing the exact DMA latency for each
transfer is mandatory to check if the internal system can sustain the total data bandwidth
required for the application.
3.1 DMA transfer time
3.1.1 Default DMA transfer timing
As described in Section 2.2.2, to perform a DMA transfer from peripheral to memory, two
bus accesses are required:
• One access over peripheral port triggered by peripheral request, which needs:
– DMA peripheral port request arbitration
– Peripheral address computation
– Reading data from the peripheral to DMA FIFO (DMA source)
• One access over memory port which can be triggered by the FIFO threshold (when
FIFO mode is used) or immediately after peripheral read (when Direct mode is used),
which needs:
– DMA memory port request arbitration
– Memory address computation
– Writing loaded data in SRAM (DMA destination)
When transferring data from memory to peripheral, two accesses are also required as
described in Section 2.2.2:
• First access: DMA anticipates peripheral access and reads data from memory and
stores it in FIFO to ensure an immediate data transfer as soon as DMA peripheral
request is triggered. This operation needs:
– DMA memory port request arbitration
– Memory address computation
– Reading data from memory to DMA FIFO (DMA source)
• Second access: when peripheral request is triggered, a transfer is generated on DMA
peripheral port. This operation needs:
– DMA peripheral port request arbitration
– Peripheral address computation
– Writing loaded data at peripheral address (DMA destination)
As a general rule, the total transfer time by DMA stream T is equal to:
S
T = T (peripheral access/transfer time) + T (memory access/transfer time)
S SP SM
With:
T is the total timing for DMA peripheral port access and transfer which is equal to:
SP
T = t + t + t + t + t
SP PA PAC BMA EDT BS
26/38 DocID022648 Rev 3

<!-- Page 27 -->


| Description | Through bus matrix |  | DMA’s direct paths |
| --- | --- | --- | --- |
|  | To AHB peripherals | To APB peripherals |  |
| t : DMA peripheral port arbitration PA | 1 AHB cycle | 1 AHB cycle | 1 AHB cycle |
| t : peripheral address computation PAC | 1 AHB cycle | 1 AHB cycle | 1 AHB cycle |
| t : bus matrix arbitration (when no BMA concurrency)(1) | 1 AHB cycle | 1 AHB cycle | N/A |
| t : effective data transfer EDT | 1 AHB cycle(2) (3) | 2 APB cycles | 2 APB cycle |
| t : bus synchronization BS | N/A | 1 AHB cycle | 1 AHB cycle |


| Description | Latency |
| --- | --- |
| t : DMA memory port arbitration MA | 1 AHB cycle |
| t : memory address computation MAC | 1 AHB cycle |
| t : bus matrix arbitration (when no concurrency)(1) BMA | 1 AHB cycle(2) |
| t : SRAM read or write access SRAM | 1 AHB cycle |

AN4031 How to predict DMA latencies
Where:
Table 4. Peripheral port access/transfer time versus DMA path used
Through bus matrix
DMA’s
Description
To AHB To APB direct paths
peripherals peripherals
t : DMA peripheral port arbitration 1 AHB cycle 1 AHB cycle 1 AHB cycle
PA
t : peripheral address computation 1 AHB cycle 1 AHB cycle 1 AHB cycle
PAC
t : bus matrix arbitration (when no
BMA 1 AHB cycle 1 AHB cycle N/A
concurrency)(1)
t : effective data transfer 1 AHB cycle(2) (3) 2 APB cycles 2 APB cycle
EDT
t : bus synchronization N/A 1 AHB cycle 1 AHB cycle
BS
1. In the case of STM32F401/STM32F410/STM32F411/STM32F412 line, t is equal to zero.
BMA
2. For FMC, an additional cycle can be added depending on the external memory used. Additional AHB
cycles are added depending on external memory timings.
3. In case of burst, the effective data transfer time depends on the burst length (INC4 t = 4 AHB cycles).
EDT
• T is the total timing for DMA memory port access and transfer which is equal to:
SM
T
SM
= t
MA
+ t
MAC
+ t
BMA
+ t
SRAM
Where:
Table 5. Memory port access/transfer time
Description Latency
t : DMA memory port arbitration 1 AHB cycle
MA
t : memory address computation 1 AHB cycle
MAC
t : bus matrix arbitration (when no concurrency)(1) 1 AHB cycle(2)
BMA
t : SRAM read or write access 1 AHB cycle
SRAM
1. In the case of STM32F401/STM32F410/STM32F411/STM32F412 line, t is equal to zero.
BMA
2. For consecutive SRAM accesses (while no other master accesses the same SRAM in-between),
t = 0 cycle.
BMA
3.1.2 DMA transfer time versus concurrent access
Additional latency can be added to the DMA service timing described in Section 3.1.1 when
several masters try to access simultaneously to the same slave.
For peripheral and memory worst-case access/transfer time, the following factors impact the
total delay time for DMA stream service:
• When several masters are accessing the same AHB destination simultaneously, the
DMA latency is impacted; the DMA transfer cannot start until the bus matrix arbiter
grants access to the DMA as described in Section 2.1.2.
• When several masters (DMA and CPU) are accessing the same AHB-to-APB bridge,
the DMA transfer time is delayed due to the AHB-to-APB bridge arbitration as
described in Section 2.3.2.
DocID022648 Rev 3 27/38
37

<!-- Page 28 -->


| AHB/APB2 frequency | F = 72 MHz/ AHB F = 72 MHz APB2 AHB/APB ratio = 1 | FAHB = 144 MHz/ FAPB2 = 72 MHz AHB/APB ratio = 2 |
| --- | --- | --- |
| Transfer time |  |  |
| t : DMA peripheral port arbitration PA | 1 AHB cycle | 1 AHB cycle |
| t : peripheral address computation PAC | 1 AHB cycle | 1 AHB cycle |
| t : bus matrix arbitration BMA | N/A(1) | N/A(1) |
| t : effective data transfer EDT | 2 AHB cycles | 4 AHB cycles |
| t : bus synchronization BS | 1 AHB cycle | 1 AHB cycle |
| T : total DMA transfer time for peripheral port SP | 5 AHB cycles | 7 AHB cycles |


| CPU/APB2 frequency | FAHB = 72MHz/ FAPB2=72MHz AHB/APB ratio = 1 | FAHB = 144MHz/ FAPB2=72MHz AHB/APB ratio = 2 |
| --- | --- | --- |
| Transfer time |  |  |
| t : DMA memory port arbitration MA | 1 AHB cycle | 1 AHB cycle |
| t : memory address computation MAC | 1 AHB cycle | 1 AHB cycle |
| t : bus matrix arbitration BMA | 1 AHB cycle(1) | 1 AHB cycle(1) |
| t : SRAM write access SRAM | 1 AHB cycle | 1 AHB cycle |
| T : total DMA transfer time for memory port SM | 4 AHB cycles | 4 AHB cycles |

How to predict DMA latencies AN4031
3.2 Examples
3.2.1 ADC-to-SRAM DMA transfer
This example is applicable to products STM32F2, STM32F405/415, STM32F407/417,
STM32F427/437 and STM32F429/439 lines.
The ADC is configured in continuous triple Interleaved mode. In this mode, it converts
continuously one analog input channel at the maximum ADC speed (36 MHz). The ADC
prescaler is set to 2, the sampling time is set to 1.5 cycles, and the delay between two
consecutive ADC samples of the Interleaved mode is set to 5 cycles.
The DMA2 stream0 transfers the ADC converted value to an SRAM buffer. DMA2 access to
ADC is done through direct path; however, DMA access to SRAM is done through the bus
matrix.
Table 6. DMA peripheral (ADC) port transfer latency
AHB/APB2 frequency F = 72 MHz/ FAHB = 144 MHz/
AHB
F = 72 MHz FAPB2 = 72 MHz
APB2
Transfer time AHB/APB ratio = 1 AHB/APB ratio = 2
t : DMA peripheral port arbitration 1 AHB cycle 1 AHB cycle
PA
t : peripheral address computation 1 AHB cycle 1 AHB cycle
PAC
t : bus matrix arbitration N/A(1) N/A(1)
BMA
t : effective data transfer 2 AHB cycles 4 AHB cycles
EDT
t : bus synchronization 1 AHB cycle 1 AHB cycle
BS
T : total DMA transfer time for peripheral port 5 AHB cycles 7 AHB cycles
SP
1. DMA2 accesses ADC through direct path: no bus matrix arbitration.
Table 7. DMA memory (SRAM) port transfer latency
CPU/APB2 frequency FAHB = 72MHz/ FAHB = 144MHz/
FAPB2=72MHz FAPB2=72MHz
Transfer time AHB/APB ratio = 1 AHB/APB ratio = 2
t : DMA memory port arbitration 1 AHB cycle 1 AHB cycle
MA
t : memory address computation 1 AHB cycle 1 AHB cycle
MAC
t : bus matrix arbitration 1 AHB cycle(1) 1 AHB cycle(1)
BMA
t : SRAM write access 1 AHB cycle 1 AHB cycle
SRAM
T : total DMA transfer time for memory port 4 AHB cycles 4 AHB cycles
SM
1. In case of DMA multiple access to SRAM, the bus matrix arbitration is equal to 0 cycle if no other master
accessed to the SRAM in-between.
In this example, the total DMA latency from the ADC DMA trigger (ADC EOC) to write the ADC value
on SRAM is equal to 9 AHB cycles for AHB/APB prescaler equals 1 and 11 AHB cycles for AHB/APB
prescaler equals 2.
Note: When using FIFO, the DMA memory port access is launched when reaching the FIFO level
configured by the user.
28/38 DocID022648 Rev 3

<!-- Page 29 -->


|  |  |  |  |  |  |  |  |  |  |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
|  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |
| (cid:87) (cid:51)(cid:36) | (cid:87) (cid:51)(cid:36)(cid:38) | (cid:87) (cid:40)(cid:39)(cid:55) | (cid:87) (cid:37)(cid:54) | (cid:87) (cid:48)(cid:36) | (cid:87) (cid:48)(cid:36)(cid:38) | (cid:87) (cid:54)(cid:53)(cid:36)(cid:48) |  |  |  |
|  |  |  |  |  |  |  |  |  |  |
|  |  |  |  | (cid:87) (cid:51)(cid:36)(cid:38) | (cid:87) (cid:40)(cid:39)(cid:55) |  | (cid:87) (cid:37)(cid:54)(cid:54) (cid:87) (cid:48)(cid:36) | (cid:87) (cid:48)(cid:36)(cid:38) | (cid:87) (cid:54)(cid:53)(cid:36)(cid:48) |

AN4031 How to predict DMA latencies
3.2.2 SPI full duplex DMA transfer
This example is applicable to products STM32F2, STM32F405/415, STM32F407/417,
STM32F427/437 and STM32F429/439 lines, and is based on the SPI1 peripheral.
Two DMA requests are configured:
• DMA2_Stream2 for SPI1_RX: this stream is configured to be the highest priority in
order to serve in time the SPI1 received data, and transfer it from the SPI1_DR register
to the SRAM buffer.
• DMA2_Stream3 for SPI1_TX: this stream transfers data from the SRAM buffer to the
SPI1_DR register.
The AHB frequency is equal to the APB2 frequency (84 MHz) and SPI1 is configured to
operate at the maximum speed (42 MHz). DMA2_Stream2 (SPI1_RX) is triggered before
DMA2_Stream3 (SPI1_TX), which is triggered two AHB cycles later.
With this configuration, the CPU is polling infinitely on the I2C1_DR register. Knowing that
the I2C1 peripheral is mapped on APB1 and that the SPI1 peripheral is mapped on APB2,
the system paths are the following:
• Direct path for DMA2 to access APB2 (not through bus matrix),
• CPU accesses APB1 through bus matrix.
The aim is to demonstrate that the DMA timings are not impacted by the CPU polling on
APB1. The following figure summarizes the DMA timing for Transmit and Receive modes,
as well as the time scheduling for each operation:
Figure 16. SPI full duplex DMA transfer time
(cid:20) (cid:21) (cid:22) (cid:23) (cid:24) (cid:25) (cid:26) (cid:27) (cid:28) (cid:20)(cid:19) (cid:20)(cid:20)
(cid:36)(cid:43)(cid:37)(cid:66)(cid:38)(cid:47)(cid:46)
(cid:39)(cid:48)(cid:36)(cid:21)(cid:66)(cid:54)(cid:87)(cid:85)(cid:72)(cid:68)(cid:80)(cid:21)(cid:3)(cid:85)(cid:72)(cid:84)(cid:88)(cid:72)(cid:86)(cid:87)
(cid:39)(cid:48)(cid:36)(cid:21)(cid:66)(cid:54)(cid:87)(cid:85)(cid:72)(cid:68)(cid:80)(cid:22)(cid:3)(cid:85)(cid:72)(cid:84)(cid:88)(cid:72)(cid:86)(cid:87)
(cid:87) (cid:87) (cid:87) (cid:87) (cid:87) (cid:87) (cid:87)
(cid:39)(cid:48)(cid:36)(cid:21)(cid:66)(cid:54)(cid:87)(cid:85)(cid:72)(cid:68)(cid:80)(cid:21)(cid:3)(cid:87)(cid:85)(cid:68)(cid:81)(cid:86)(cid:68)(cid:70)(cid:87)(cid:76)(cid:82)(cid:81) (cid:51)(cid:36) (cid:51)(cid:36)(cid:38) (cid:40)(cid:39)(cid:55) (cid:37)(cid:54) (cid:48)(cid:36) (cid:48)(cid:36)(cid:38) (cid:54)(cid:53)(cid:36)(cid:48)
(cid:39)(cid:48)(cid:36)(cid:21)(cid:66)(cid:54)(cid:87)(cid:85)(cid:72)(cid:68)(cid:80)(cid:22)(cid:3)(cid:87)(cid:85)(cid:68)(cid:81)(cid:86)(cid:68)(cid:70)(cid:87)(cid:76)(cid:82)(cid:81) (cid:87) (cid:51)(cid:36)(cid:38) (cid:87) (cid:40)(cid:39)(cid:55) (cid:87) (cid:37)(cid:54)(cid:54) (cid:87) (cid:48)(cid:36) (cid:87) (cid:48)(cid:36)(cid:38) (cid:87) (cid:54)(cid:53)(cid:36)(cid:48)
(cid:48)(cid:54)(cid:89)(cid:22)(cid:23)(cid:21)(cid:19)(cid:19)(cid:57)(cid:20)
This figure illustrates the following conclusions:
• CPU polling on APB1 is not impacting the DMA transfer latency on APB2.
• For the DMA2_Stream2 (SPI1_RX) transaction, at the eighth AHB clock cycle, there is
no bus matrix arbitration since it is supposed that the last master that accessed the
SRAM is DMA2 (so no re-arbitration is needed).
• For the DMA2_Stream3 (SPI1_TX) transaction, this stream anticipates the read from
SRAM and writes it on the FIFO and then, once triggered, the DMA peripheral port
(destination is SPI1) starts operation.
• For DMA2_Stream3, the DMA peripheral arbitration phase (1 AHB cycle) is executed
during the DMA2_Stream2 bus synchronization cycle.
DocID022648 Rev 3 29/38
37

<!-- Page 30 -->

How to predict DMA latencies AN4031
This optimization is always executed like this when the DMA request is triggered before
the end of a current DMA request transaction.
30/38 DocID022648 Rev 3

<!-- Page 31 -->

AN4031 Tips and warnings while programming the DMA controller
4 Tips and warnings while programming the DMA
controller
4.1 Software sequence to disable DMA
To switch off a peripheral connected to a DMA stream request, it is mandatory to:
1. switch off the DMA stream to which the peripheral is connected,
2. wait until the EN bit in DMA_SxCR register is reset (“0”).
Only then can the peripheral be safely disabled. DMA request enable bit in the peripheral
control register should be reset ("0") to guarantee that any pending request from peripheral
side is cleared.
Note: In both cases, a Transfer Complete Interrupt Flag (TCIF in DMA_LISR or DMA_HISR) is set
to indicate the end of transfer due to the stream disable.
4.2 DMA flag management before enabling a new transfer
Before enabling a new transfer, the user must ensure that the Transfer Complete Interrupt
Flag (TCIF) in DMA_LISR or DMA_HISR is cleared.
As a general recommendation, it is advised to clear all flags in the DMA_LIFCR and
DMA_HIFCR registers before starting a new transfer.
4.3 Software sequence to enable DMA
The following software sequence applies when enabling DMA:
1. Configure the suitable DMA stream.
2. Enable the DMA stream used (set the EN bit in the DMA_SxCR register).
3. Enable the peripheral used. DMA request enable bit in the peripheral control register
should be set ("1").
Note: If the user enables the used peripheral before the corresponding DMA stream, a “FEIF”
(FIFO Error Interrupt Flag) may be set due to the fact the DMA is not ready to provide the
first required data to the peripheral (in case of memory-to-peripheral transfer).
4.4 Memory-to-memory transfer while NDTR=0
When configuring a DMA stream to perform a memory-to-memory transfer in normal mode,
once NDTR reaches 0, the Transfer Complete is set. At that time, if the user sets the enable
bit (EN bit in DMA_SxCR) of this stream, the memory-to-memory transfer is automatically
re-triggered again with the last NDTR value.
4.5 DMA peripheral burst with PINC/MINC=0
DMA Burst feature with peripheral address increment (PINC) or memory address increment
(MINC) disable allows to address internal or external (FSMC) peripherals supporting Burst
DocID022648 Rev 3 31/38
37

<!-- Page 32 -->

Tips and warnings while programming the DMA controller AN4031
(embedding FIFOs). This mode ensures that this DMA stream cannot be interrupted by
other DMA streams during its transactions.
4.6 Twice-mapped DMA requests
When the user configures two (or more) DMA streams to serve the same peripheral request,
software should ensure that the current DMA stream is completely disabled (by polling the
EN bit in the DMA_SxCR register) before enabling a new DMA stream.
4.7 Best DMA throughput configuration
When using STM32F4xx with reduced AHB frequency while DMA is servicing a high-speed
peripheral, it is recommended to put the stack and heap in the CCM (which can be
addressed directly by the CPU through D-bus) instead of putting them on the SRAM, which
would create an additional concurrency between CPU and DMA accessing the SRAM
memory.
4.8 DMA transfer suspension
At any time, a DMA transfer can be suspended to be restarted later on or to be definitively
disabled before the end of the DMA transfer.
There are two cases:
• The stream disables the transfer with no later-on restart from the point where it was
stopped: there is no particular action to do, except to clear the EN bit in the DMA_SxCR
register to disable the stream and to wait until the EN bit is reset. As a consequence:
– The DMA_SxNDTR register contains the number of remaining data items at the
moment when the stream was stopped so that the software can determine how
many data items have been transferred before the stream was interrupted.
• The stream suspends the transfer in order to resume it later by re-enabling the stream:
to restart from the point where the transfer was stopped, the software has to read the
DMA_SxNDTR register after disabling the stream (EN bit at “0”) to know the number of
data items already collected. Then:
– The peripheral and/or memory addresses have to be updated in order to adjust
the address pointers.
– The SxNDTR register has to be updated with the remaining number of data items
to be transferred (the value read when the stream was disabled).
– The stream may then be re-enabled to restart the transfer from the point where it
was stopped.
Note: In both cases, a Transfer Complete Interrupt Flag (TCIF in DMA_LISR or DMA_HISR) is set
to indicate the end of transfer due to the stream interruption.
32/38 DocID022648 Rev 3

<!-- Page 33 -->


| Transfer mode | DMA2 AHB memory port | DMA2 AHB peripheral port |
| --- | --- | --- |
| Memory to Peripheral | Read direction | Write direction |
| Peripheral to Memory | Write direction | Read direction |
| Memory to Memory | Write direction | Read direction |

AN4031 Tips and warnings while programming the DMA controller
4.9 Take benefits of DMA2 controller and system architecture
flexibility
The idea behind this section is to show how take benefits from flexibility offered by STM32
architecture and DMA controller. As an illustration of this flexibility we will see how to invert
DMA2 AHB peripheral and memory ports and preserve correct managing of peripherals
data transfers. In order to achieve this and take over control of regular DMA2 behavior we
need to review working model of DMA2.
As both DMA2 ports are connected to AHB BusMatrix, and having symmetric connection
with AHB slaves, this architecture lets traffic flow in one or another direction over the
Peripheral and Memory ports depending on software configuration.
4.9.1 Inverting transfers over DMA2 AHB ports consideration
Software has flexibility to configure DMA2 stream transfer mode according to its needs.
Depending on this configuration one DMA2 AHB port would be programmed in the Read
direction, and the other in the Write direction.
Table 8 shows DMA AHB port direction vs. transfer mode configuration.
Table 8. DMA AHB port direction vs. transfer mode configuration
Transfer mode DMA2 AHB memory port DMA2 AHB peripheral port
Memory to Peripheral Read direction Write direction
Peripheral to Memory Write direction Read direction
Memory to Memory Write direction Read direction
Now focusing on traffic flow, as shown in Section 2.2.2: DMA transfer states, Transfers on
Peripheral port are triggered by peripheral requests, Transfers on Memory port are triggered
either by the FIFO threshold (when FIFO mode is used) or immediately after peripheral read
(when Direct mode is used).
When managing peripherals with DMA2 memory port we need to take care of pre-triggered
transfers, buffered transfers, and last data management:
Pre-triggered transfer:
As described in Section 2.2.2: DMA transfer states when memory to peripheral transfer
mode is configured (reading data over memory port) DMA anticipates the peripheral’s
access and reads data as soon as DMA stream is enabled. One data is buffered in direct
mode and up to 4 x 32-bit words when DMA FIFO is enabled.
When managing peripheral reads over DMA2 memory port, software must ensure that
peripheral is enabled before enabling DMA in order to guarantee validity of first DMA
access.
Figure 17 illustrates DMA accesses over memory and peripheral ports vs. peripheral
triggers.
DocID022648 Rev 3 33/38
37

<!-- Page 34 -->


|  |
| --- |
| (cid:48)(cid:72)(cid:80)(cid:82)(cid:85)(cid:92)(cid:3)(cid:83)(cid:82)(cid:85)(cid:87) (cid:20) (cid:21) (cid:22) (cid:85)(cid:72)(cid:68)(cid:71) (cid:51)(cid:72)(cid:85)(cid:76)(cid:83)(cid:75)(cid:72)(cid:85)(cid:68)(cid:79) (cid:85)(cid:72)(cid:84)(cid:88)(cid:72)(cid:86)(cid:87) (cid:51)(cid:72)(cid:85)(cid:76)(cid:83)(cid:75)(cid:72)(cid:85)(cid:68)(cid:79)(cid:3)(cid:83)(cid:82)(cid:85)(cid:87)(cid:3) (cid:20) (cid:21) (cid:90)(cid:85)(cid:76)(cid:87)(cid:72) (cid:48)(cid:54)(cid:89)(cid:23)(cid:22)(cid:19)(cid:25)(cid:28)(cid:57)(cid:20) |

Tips and warnings while programming the DMA controller AN4031
Figure 17. DMA in Memory-to-Pheripheral transfer mode
(cid:48)(cid:72)(cid:80)(cid:82)(cid:85)(cid:92)(cid:3)(cid:83)(cid:82)(cid:85)(cid:87) (cid:20) (cid:21) (cid:22)
(cid:85)(cid:72)(cid:68)(cid:71)
(cid:51)(cid:72)(cid:85)(cid:76)(cid:83)(cid:75)(cid:72)(cid:85)(cid:68)(cid:79)
(cid:85)(cid:72)(cid:84)(cid:88)(cid:72)(cid:86)(cid:87)
(cid:51)(cid:72)(cid:85)(cid:76)(cid:83)(cid:75)(cid:72)(cid:85)(cid:68)(cid:79)(cid:3)(cid:83)(cid:82)(cid:85)(cid:87)(cid:3) (cid:20) (cid:21)
(cid:90)(cid:85)(cid:76)(cid:87)(cid:72)
(cid:48)(cid:54)(cid:89)(cid:23)(cid:22)(cid:19)(cid:25)(cid:28)(cid:57)(cid:20)
Managing last data read
DMA controller features a 4 x 32-bit words FIFO per stream is used to buffer the data
between AHB ports. When managing peripheral reads over DMA memory port, software
must ensure that 4x extra words are read from the peripheral. This is to guarantee that last
valid data are transferred-out from DMA FIFO
Buffered transfers when DMA direct mode is disabled:
When writing data over memory port to a peripheral in indirect mode (FIFO mode enabled),
the software needs to take care that access over this port is triggered by programmed FIFO
threshold. When threshold is reached data are transferred-out from FIFO to destination over
memory port.
When writing to a register (for example GPIOs not having a FIFO), the data from the DMA
FIFO will be written successively to destination.
Last but not least, when swapping peripheral management from peripheral port to memory
port, the software needs to re-consider transfer size and address increment configuration.
As described in Section 1.1.5: Transfer size, transfer size is defined by peripheral side
transfer width (byte, half-word, word) and by the number of data items to be transferred
(value programmed in DMA_SxNTDR register). According to new DMA configuration when
inverting ports the programmed value in DMA_SxNTDR register may need to be adjusted.
4.9.2 Example for inverting Quad-SPI transfers over DMA2 AHB ports
consideration
In this example DMA_S7M0AR is programmed with Quad-SPI Data register address,
DMA_S7PAR programmed with data buffer address (e.g Buffer in SRAM). In the
DMA_S7CR register the DMA2 stream direction must be configured in Peripheral to
Memory transfer mode when writing to Quad-SPI. DMA2 stream direction must be
configured in Memory to Peripheral transfer mode when reading from Quad-SPI.
4x Extra words (32-bits) are needed for read operation, in order to guarantee that the last
data is transferred-out from DMA FIFO to RAM memory.
34/38 DocID022648 Rev 3

<!-- Page 35 -->


| Write operation | Read operation |
| --- | --- |
| /* Program M0AR with QUADSPI data register address */ DMA2_Stream7->M0AR = (uint32_t)&QUADSPI->DR; /* Program PAR with Buffer address */ DMA2_Stream7->PAR = (uint32_t)&u32Buffer[0]; /* Write number of data items to transfer */ DMA2_Stream7->NDTR = 0x100; /* Configure DMA : MSIZE=PSIZE=0x02 (Word), CHSEL=0x03 (QUADSPI), PINC=1, DIR=0x00 */ DMA2_Stream7->CR = DMA_SxCR_PSIZE_1 | DMA_SxCR_MSIZE_1 | 3ul<<25 | DMA_SxCR_PINC; /* Enable DMA request generation */ QUADSPI->CR |= QUADSPI_CR_DMAEN; /* Write the DLR Register */ QUADSPI->DLR = (0x100* 4)-1; /* Write to QUADSPI DCR */ QUADSPI->CCR = QUADSPI_CCR_IMODE_0| QUADSPI_CCR_ADMODE_0| QUADSPI_CCR_DMODE| QUADSPI_CCR_ADSIZE| QUAD_IN_FAST_PROG_CMD; /* Write the AR Register */ QUADSPI->AR = 0x00ul; /* Enable the selected DMA2_Stream7 by setting EN bit */ DMA2_Stream7->CR |= (uint32_t)DMA_SxCR_EN; /* Wait for the end of Transfer */ while((QUADSPI->SR & QUADSPI_SR_TCF) != QUADSPI_SR_TCF); | /* Program M0AR with QUADSPI data register address */ DMA2_Stream7->M0AR = (uint32_t)&QUADSPI->DR; /* Program PAR with Buffer address */ DMA2_Stream7->PAR = (uint32_t)&u32Buffer[0]; /* Write number of data items to transfer */ DMA2_Stream7->NDTR = 0x100; /* Configure DMA : MSIZE=PSIZE=0x02 (Word), CHSEL=0x03 (QUADSPI), PINC=1, DIR=0x01 */ DMA2_Stream7->CR = DMA_SxCR_PSIZE_1 | DMA_SxCR_MSIZE_1 | 3ul<<25 | DMA_SxCR_PINC| DMA_SxCR_DIR_0; /* Enable DMA request generation */ QUADSPI->CR |= QUADSPI_CR_DMAEN; /* Write the DLR Register */ QUADSPI->DLR = ((0x100+4)* 4)-1; /* Write to QUADSPI DCR */ QUADSPI->CCR = QUADSPI_CCR_IMODE_0| QUADSPI_CCR_ADMODE_0| QUADSPI_CCR_DMODE| QUADSPI_CCR_ADSIZE| QUADSPI_CCR_FMODE_0|QUAD_OUT_FAST_READ_CMD; /* Write the AR Register */ QUADSPI->AR = 0x00ul; /* Enable the selected DMA2_Stream7 by setting EN bit */ DMA2_Stream7->CR |= (uint32_t)DMA_SxCR_EN; /* Wait for the end of Transfer */ while((DMA2_Stream7->CR & DMA_SxCR_EN) == DMA_SxCR_EN); |

AN4031 Tips and warnings while programming the DMA controller
Table 9. Code Snippet
Write operation Read operation
/* Program M0AR with QUADSPI data register address */ /* Program M0AR with QUADSPI data register address */
DMA2_Stream7->M0AR = (uint32_t)&QUADSPI->DR; DMA2_Stream7->M0AR = (uint32_t)&QUADSPI->DR;
/* Program PAR with Buffer address */ /* Program PAR with Buffer address */
DMA2_Stream7->PAR = (uint32_t)&u32Buffer[0]; DMA2_Stream7->PAR = (uint32_t)&u32Buffer[0];
/* Write number of data items to transfer */ /* Write number of data items to transfer */
DMA2_Stream7->NDTR = 0x100; DMA2_Stream7->NDTR = 0x100;
/* Configure DMA : MSIZE=PSIZE=0x02 (Word), CHSEL=0x03 /* Configure DMA : MSIZE=PSIZE=0x02 (Word), CHSEL=0x03
(QUADSPI), PINC=1, DIR=0x00 */ (QUADSPI), PINC=1, DIR=0x01 */
DMA2_Stream7->CR = DMA_SxCR_PSIZE_1 | DMA2_Stream7->CR = DMA_SxCR_PSIZE_1 |
DMA_SxCR_MSIZE_1 | 3ul<<25 | DMA_SxCR_PINC; DMA_SxCR_MSIZE_1 | 3ul<<25 | DMA_SxCR_PINC|
DMA_SxCR_DIR_0;
/* Enable DMA request generation */ /* Enable DMA request generation */
QUADSPI->CR |= QUADSPI_CR_DMAEN; QUADSPI->CR |= QUADSPI_CR_DMAEN;
/* Write the DLR Register */ /* Write the DLR Register */
QUADSPI->DLR = (0x100* 4)-1; QUADSPI->DLR = ((0x100+4)* 4)-1;
/* Write to QUADSPI DCR */ /* Write to QUADSPI DCR */
QUADSPI->CCR = QUADSPI_CCR_IMODE_0| QUADSPI->CCR = QUADSPI_CCR_IMODE_0|
QUADSPI_CCR_ADMODE_0| QUADSPI_CCR_DMODE| QUADSPI_CCR_ADMODE_0| QUADSPI_CCR_DMODE|
QUADSPI_CCR_ADSIZE| QUAD_IN_FAST_PROG_CMD; QUADSPI_CCR_ADSIZE|
QUADSPI_CCR_FMODE_0|QUAD_OUT_FAST_READ_CMD;
/* Write the AR Register */ /* Write the AR Register */
QUADSPI->AR = 0x00ul; QUADSPI->AR = 0x00ul;
/* Enable the selected DMA2_Stream7 by setting EN bit */ /* Enable the selected DMA2_Stream7 by setting EN bit */
DMA2_Stream7->CR |= (uint32_t)DMA_SxCR_EN; DMA2_Stream7->CR |= (uint32_t)DMA_SxCR_EN;
/* Wait for the end of Transfer */ /* Wait for the end of Transfer */
while((QUADSPI->SR & QUADSPI_SR_TCF) != while((DMA2_Stream7->CR & DMA_SxCR_EN) ==
QUADSPI_SR_TCF); DMA_SxCR_EN);
Note: The limitation of the data corruption when DMA2 is managing in parallel AHB and APB2
transfers (refer to product errata sheets to identify impacted STM32F2/F4 MCUs) can be
overcome by swapping DMA2 peripheral and memory ports, as described in this section.
4.10 STM32F7 DMA transfer and cache maintenance to avoid data
incoherency
When the software is using cacheable memory regions for the DMA source/or destination
buffers, it must trigger a cache clean before starting a DMA operation to ensure that all the
data are committed to the subsystem memory. When reading the data from the peripheral
after the completion of DMA transfer, the software must perform a cache invalidate before
reading the updated memory region.
It is preferable to use non-cacheable regions for DMA buffers. The software can use the
MPU to set up a non-cacheable memory block to use as a shared memory between the
CPU and DMA.
DocID022648 Rev 3 35/38
37

<!-- Page 36 -->

Conclusion AN4031
5 Conclusion
The DMA controller is designed to cover most of the embedded use case applications by:
• Giving flexibility to firmware to choose the suitable combination between 16 streams X
16 channels (eight for each DMA),
• Reducing the total latency time for a DMA transfer, thanks to dual AHB port
architecture, and direct path to APB bridges avoiding CPU stall on AHB1 access when
DMA is servicing low-speed APB peripherals,
• FIFOs implementation on DMA allows more flexibility to firmware to configure different
data sizes between source and destination, and speeds-up transfers when using
incremental burst transfer mode.
36/38 DocID022648 Rev 3

<!-- Page 37 -->


| Date | Revision | Changes |
| --- | --- | --- |
| 04-Feb-2014 | 1 | Initial release. |
| 06-Aug-2015 | 2 | Added Section : Reference documents. Document scope extended to STM32F2 and STM32F4 Series and list of reference manuals updated. Removed note 1 in Table 1: STM32F427/437 and STM32F429/439 DMA1 request mapping and Table 2: STM32F427/437 and STM32F429/439 DMA2 request mapping. Removed Table DMA1 request mapping for STM32F401 line and Table DMA2 request mapping for STM32F401 line. Updated Section 4.1: Software sequence to disable DMA and Section 4.3: Software sequence to enable DMA. |
| 23-Jun-2016 | 3 | Added: – Section 4.9: Take benefits of DMA2 controller and system architecture flexibility – Section 4.10: STM32F7 DMA transfer and cache maintenance to avoid data incoherency – Updated: – Introduction – Section 2: System performance considerations – Section 2.1.3: BusMatrix arbitration and DMA transfer delays worst case – Figure 14: DMA request arbitration Removed: – Table 1: Applicable products |

AN4031 Revision history
6 Revision history
Table 10. Document revision history
Date Revision Changes
04-Feb-2014 1 Initial release.
Added Section : Reference documents.
Document scope extended to STM32F2 and STM32F4
Series and list of reference manuals updated.
Removed note 1 in Table 1: STM32F427/437 and
STM32F429/439 DMA1 request mapping and Table 2:
STM32F427/437 and STM32F429/439 DMA2 request
06-Aug-2015 2
mapping. Removed Table DMA1 request mapping for
STM32F401 line and Table DMA2 request mapping for
STM32F401 line.
Updated Section 4.1: Software sequence to disable
DMA and Section 4.3: Software sequence to enable
DMA.
Added:
– Section 4.9: Take benefits of DMA2 controller and
system architecture flexibility
– Section 4.10: STM32F7 DMA transfer and cache
maintenance to avoid data incoherency
– Updated:
23-Jun-2016 3 – Introduction
– Section 2: System performance considerations
– Section 2.1.3: BusMatrix arbitration and DMA transfer
delays worst case
– Figure 14: DMA request arbitration
Removed:
– Table 1: Applicable products
DocID022648 Rev 3 37/38
37

<!-- Page 38 -->

AN4031
IMPORTANT NOTICE – PLEASE READ CAREFULLY
STMicroelectronics NV and its subsidiaries (“ST”) reserve the right to make changes, corrections, enhancements, modifications, and
improvements to ST products and/or to this document at any time without notice. Purchasers should obtain the latest relevant information on
ST products before placing orders. ST products are sold pursuant to ST’s terms and conditions of sale in place at the time of order
acknowledgement.
Purchasers are solely responsible for the choice, selection, and use of ST products and ST assumes no liability for application assistance or
the design of Purchasers’ products.
No license, express or implied, to any intellectual property right is granted by ST herein.
Resale of ST products with provisions different from the information set forth herein shall void any warranty granted by ST for such product.
ST and the ST logo are trademarks of ST. All other product or service names are the property of their respective owners.
Information in this document supersedes and replaces information previously supplied in any prior versions of this document.
© 2016 STMicroelectronics – All rights reserved
38/38 DocID022648 Rev 3