#![deny(warnings)]

use core::panic::PanicInfo;
use cortex_m::interrupt;
use rtt_target::rprintln;
use stm32f4xx_hal::{pac, prelude::*};

#[panic_handler]
unsafe fn panic(info: &PanicInfo) -> ! {
    interrupt::disable();

    rprintln!("{}", info);

    // Blink the red LED to indicate a panic
    let device = pac::Peripherals::steal();
    let gpiod = device.GPIOD.split();
    let mut led_red = gpiod.pd14.into_push_pull_output();
    loop {
        for _ in 0..5_000 {
            led_red.set_high();
        }

        for _ in 0..10_000 {
            led_red.set_low();
        }
    }
}
