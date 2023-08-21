// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.
#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]
extern crate cortex_m_rt;
extern crate panic_halt;

use cortex_m_rt::entry;

// use core::num::Wrapping;
// use core::usize;
// use heapless::Vec;
use rtt_target::rprintln;

// https://docs.rs/stm32f4xx-hal/latest/stm32f4xx_hal/gpio/index.html
use stm32f4xx_hal::{pac, prelude::*};

#[entry]
fn main() -> ! {
    // https://docs.rs/svd2rust/0.24.1/svd2rust/#peripheral-api
    // let core = cortex_m::Peripherals::take().unwrap();
    let peripherals = pac::Peripherals::take().unwrap();

    // Configure Reset and Clock Control (RCC)
    let rcc = peripherals.RCC.constrain();
    // Use HSE (High Speed External) clock source
    // Configure the clock source and frequency to 168MHz
    let clocks = rcc
        .cfgr
        .use_hse(8.MHz())
        .sysclk(168.MHz())
        .pclk1(24.MHz())
        .i2s_clk(86.MHz())
        .freeze();

    // Configure General Purpose Input Output (GPIO) for LEDs
    let gpioa = peripherals.GPIOA.split();
    // LED4: PD12 Green
    // LED3: PD13 Orange
    // LED5: PD14 Red
    // LED6: PD15 Blue
    let mut led = gpioa.pa15.into_push_pull_output();

    // Configure Timers
    let mut delay = peripherals.TIM2.delay_ms(&clocks);

    // Configure Real Time Transfer (RTT)
    rtt_target::rtt_init!();

    rprintln!("Hello Patrick\n");
    // Main Loop
    loop {
        rprintln!(".");
        led.set_high();
        delay.delay_ms(1_000_u16);
        led.set_low();
        delay.delay_ms(1_000_u16);
    }
}
