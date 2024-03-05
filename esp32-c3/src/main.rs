#![no_std]
#![no_main]

use esp32c3_hal::{clock::ClockControl, peripherals::Peripherals, prelude::*, Delay, IO};
use esp_backtrace as _;
use esp_println::println;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();

    let clocks = ClockControl::max(system.clock_control).freeze();
    let mut delay = Delay::new(&clocks);

    println!("Hello world!");

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let mut led = io.pins.gpio7.into_push_pull_output();
    led.set_high().unwrap();

    loop {
        println!("Loop...");
        led.toggle().unwrap();
        delay.delay_ms(500u32);
    }
}
