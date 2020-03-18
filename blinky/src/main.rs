#![no_std]
#![no_main]

extern crate panic_halt;
extern crate stm32f103xx_hal as hal;

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    let mut n = 0;

    loop {
        let _fib = fib(n);
        n += 1;
    }
}

fn fib(n: usize) -> usize {
    if n < 2 {
        1
    } else {
        fib(n - 1) + fib(n - 2)
    }
}
