#![no_main]
#![no_std]

#[allow(unused)]
use panic_halt;

pub use stm32f4xx_hal as hal;

pub use crate::hal::prelude::*;
pub use crate::hal::stm32::interrupt::*;
pub use crate::hal::stm32::*;
pub use crate::hal::*;
pub use cortex_m_rt::*;

use cortex_m_rt::entry;

use stm32f4xx_hal::delay::Delay;

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32::Peripherals::take().unwrap();

    let syst = cp.SYST;

    let rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze();

    let gpioa = dp.GPIOA.split();

    let mut led = gpioa.pa5.into_push_pull_output();

    let mut delay_provider = Delay::new(syst, clocks);

    loop {
        delay_provider.delay_ms(500u32);
        let _ = led.set_high();
        delay_provider.delay_ms(500u32);
        let _ = led.set_low();
    }
}
