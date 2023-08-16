# OpenMotion System

OpenMotion is an open motor driver software and hardware ecosystem that controls BDLC, Stepper, and DC motors

## Hardware Design

The motor controller circuit is based on the STM32G4 from ST Microelectronics. The micro controller
has the hardware to drive 4 MOSFET or IGBT half bridges for small or large motors. Being able to independantly
control the 4 phase outputs this hardware enables support for most motors.

Features:
- Tuned system for CNC and Robotics applications
- Arm Cortex-M4 (DSP + FPU) 170MHz processor
- 4x 20A 600V Phase Ouputs
    * 2 DC (brushed) Motors
    * 1 3-phase brushless direct current motor (BDLC)
    * 1 3-phase permanent magnet synchronous motor (PMSM)
    * Control 1 Stepper Motor
- Flexible external interface
    * USB-C interface with power delivery for programming, debugging, and computer control
    * CAN bus for communication with other motor controllers
    * I2C for communication with sensors
    * SPI for communication with sensors
    * USART for communication with hosts


Tachometer input
Hall effect sensor / Encoder input

1KW Motor: 600 V IGBT STGP10H60DF in a TO-220 (Heatsink)
150W Motor: 600V 4.5A STGD3HF60HD with L6389E gate driver
2.2KW Spindle: 1200V 25A IGBT

## Developer Getting Started

1. download and install the [STM32CubeIDE](https://www.st.com/en/development-tools/stm32cubeide.html) from ST Microelectronics
2. 

Setup the development environment  

```sh
make tools
```


## Reccomended Motors

### AC Motors High Power


## Alternatives

* [The VESC Project](https://vesc-project.com/)
