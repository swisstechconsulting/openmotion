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

// #![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

// use core::num::Wrapping;
// use core::usize;
// use heapless::Vec;
use rtt_target::{rprintln, rtt_init_print};

// https://docs.rs/stm32f4xx-hal/latest/stm32f4xx_hal/gpio/index.html
use stm32f4xx_hal::{pac, prelude::*};

mod panic_led;

#[cortex_m_rt::entry]
fn main() -> ! {
    let delay_loops = 20_000;

    // Configure Real Time Transfer (RTT) only in Debug mode
    rtt_init_print!();

    // https://docs.rs/svd2rust/0.24.1/svd2rust/#peripheral-api
    // let core = cortex_m::Peripherals::take().unwrap();
    let device = pac::Peripherals::take().unwrap();

    // Configure General Purpose Input Output (GPIO) for LEDs
    rprintln!("Configure GPIOs");
    let gpiod = device.GPIOD.split();

    let mut led_green = gpiod.pd12.into_push_pull_output();
    let mut led_orange = gpiod.pd13.into_push_pull_output();
    let mut led_red = gpiod.pd14.into_push_pull_output();
    let mut led_blue = gpiod.pd15.into_push_pull_output();
    led_green.set_high();

    // Configure Reset and Clock Control (RCC)
    rprintln!("Configure Reset and Clock Control");
    let rcc = device.RCC.constrain();
    led_orange.set_high();
    // Use HSE (High Speed External) clock source
    // Configure the clock source and frequency to 168MHz
    let clocks = rcc
        .cfgr
        .use_hse(8.MHz())
        .sysclk(168.MHz())
        .pclk1(24.MHz())
        .pclk2(24.MHz())
        .freeze();

    led_red.set_high();

    // Configure Timers
    rprintln!("Configure Timers");
    let mut delay = device.TIM5.delay_us(&clocks);

    rprintln!("Hello Patrick");
    led_green.set_low();
    led_orange.set_low();
    led_red.set_low();
    led_blue.set_low();

    // Main Loop
    rprintln!("Start Infinite Loop");
    loop {
        rprintln!(".");
        led_green.toggle();
        delay.delay(1.secs());

        for _ in 0..delay_loops {
            led_green.set_high();
        }

        for _ in 0..delay_loops {
            led_green.set_low();
            led_orange.set_high();
        }

        for _ in 0..delay_loops {
            led_orange.set_low();
            led_red.set_high();
        }

        for _ in 0..delay_loops {
            led_red.set_low();
            led_blue.set_high();
        }

        for _ in 0..delay_loops {
            led_blue.set_low();
        }

        for _ in 0..5 * delay_loops {
            led_green.set_high();
            led_orange.set_high();
            led_red.set_high();
            led_blue.set_high();
        }

        for _ in 0..delay_loops {
            led_green.set_low();
            led_orange.set_low();
            led_red.set_low();
            led_blue.set_low();
        }
    }
}
