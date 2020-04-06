#![no_std]
#![no_main]

extern crate panic_halt;

use core::ptr;
use cortex_m::asm;
use cortex_m_rt::entry;
use stm32f103xx;

const RCC_APB2ENR: *mut u32 = (0x4002_1000 + 0x18) as *mut u32;
const GPIOC_CRH: *mut u32 = (0x4001_1000 + 0x04) as *mut u32;
const GPIOC_BSRR: *mut u32 = (0x4001_1000 + 0x10) as *mut u32;

const APB2ENR_IOPCEN: usize = 4;
const CRH_MODE13: usize = 20;
const BSRR_BS13: usize = 13;
const BSRR_BR13: usize = 13 + 16;

#[entry]
fn main() -> ! {
    unsafe {
        // 启用 GPIOC
        ptr::write_volatile(RCC_APB2ENR, 1 << APB2ENR_IOPCEN);
        // 配置 GPIOC - PC13 为推挽输出
        ptr::write_volatile(GPIOC_CRH, 0b0011 << CRH_MODE13);
        // 重置 PC13 以输出低电平
        ptr::write_volatile(GPIOC_BSRR, 1 << BSRR_BR13);
    }

    loop {
        delay();

        // Reset：输出低电平，点亮 LED
        unsafe {
            ptr::write_volatile(GPIOC_BSRR, 1 << BSRR_BR13);
        }

        delay();

        // Set：输出高电平，LED 熄灭
        unsafe {
            ptr::write_volatile(GPIOC_BSRR, 1 << BSRR_BS13);
        }
    }
}

fn delay() {
    for _ in 0..2_000 {
        asm::nop();
    }
}
