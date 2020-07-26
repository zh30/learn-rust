#![no_std]
#![no_main]

extern crate panic_halt;
extern crate stm32f103xx_hal as hal;
#[macro_use]
extern crate nb;

use cortex_m_rt::entry;

use hal::prelude::*;
use hal::stm32f103xx;
use hal::timer::Timer;

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32f103xx::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    // 设置时钟总线
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    // 设置通用引脚 (GPIO)
    // let mut gpioc = dp.GPIOC.split(&mut rcc.apb2);

    // LED 对应的 PC13 引脚
    // let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

    // 淘宝上有些版本的核心板的 LED 会接在 PB12 引脚上，这样的话用下面两行替换
    let mut gpiob = dp.GPIOB.split(&mut rcc.apb2);
    let mut led = gpiob.pb12.into_push_pull_output(&mut gpiob.crh);

    let mut timer = Timer::syst(cp.SYST, 20.hz(), clocks);
    loop {
        block!(timer.wait()).unwrap();
        // 点亮 LED
        led.set_high();
        block!(timer.wait()).unwrap();
        // 关闭 LED
        led.set_low();
    }
}
