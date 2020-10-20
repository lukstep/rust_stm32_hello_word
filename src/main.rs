#![no_main]
#![no_std]

#[allow(unused)]
use panic_halt;

use cortex_m_rt as rt;
use stm32f4xx_hal as hal;
use hal::prelude::*; // need for the GpioExt trait (-> .split)

#[inline(never)]
fn delay() {
    for _ in 0..9_999 {}
}

#[rt::entry]
fn main() -> ! {
    if let Some(peripherals) = hal::stm32::Peripherals::take() {
        let gpioa = peripherals.GPIOA.split();
        let mut led = gpioa.pa5.into_push_pull_output();

        loop {
            led.set_low().unwrap();
            delay();
            led.set_high().unwrap();
            delay();
        }
    }
    loop {}
}
