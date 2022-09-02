#![no_std]
#![no_main]

use cortex_m;

use hal::hal::digital::v2::OutputPin;
use hal::serial::config::WordLength;
use stm32f4xx_hal as hal;

use hal::pac::USART2;
use hal::serial;
use hal::serial::Rx;
use hal::serial::Tx;
use hal::spi::Mode;
use hal::spi::Phase;
use hal::spi::Polarity;
use panic_halt as _;

use cortex_m_rt::ExceptionFrame;
use cortex_m_rt::{entry, exception};
use embedded_graphics::{draw_target::DrawTarget, pixelcolor::Rgb565, prelude::*};

use st7735_lcd::{Orientation, ST7735};

use hal::prelude::*;

mod game;

#[entry]
fn main() -> ! {
    let cp: cortex_m::Peripherals = cortex_m::Peripherals::take().unwrap();
    let dp: hal::pac::Peripherals = hal::pac::Peripherals::take().unwrap();

    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(84.MHz()).pclk1(42.MHz()).freeze();

    let gpioa = dp.GPIOA.split();

    /* set up UART*/
    let mut serial_config = serial::Config::default();
    serial_config.wordlength = WordLength::DataBits8;
    let serial_config = serial_config;

    let tx = gpioa.pa2.into_alternate::<7>();
    let rx = gpioa.pa3.into_alternate::<7>();

    let serial_port = serial::Serial2::new(dp.USART2, (tx, rx), serial_config, &clocks)
        .unwrap()
        .with_u8_data();

    let (mut tx, mut rx) = serial_port.split();

    /* Set up for ST7735*/
    let sck = gpioa.pa5.into_alternate::<5>();
    let miso = gpioa.pa6.into_alternate::<5>();
    let mosi = gpioa.pa7.into_alternate::<5>();

    let rst = gpioa.pa1.into_push_pull_output();
    let dc = gpioa.pa0.into_push_pull_output();

    let spi = hal::pac::SPI1::spi(
        dp.SPI1,
        (sck, miso, mosi),
        Mode {
            polarity: Polarity::IdleLow,
            phase: Phase::CaptureOnFirstTransition,
        },
        16.MHz().into(),
        &clocks,
    );

    let mut delay = cortex_m::delay::Delay::new(cp.SYST, clocks.hclk().to_Hz());
    //hal::timer::Timer::new(d&p.TIM2, &clocks);

    let mut disp = ST7735::new(spi, dc, rst, true, false, 160, 128);
    disp.init(&mut delay).unwrap();
    disp.set_orientation(&Orientation::Landscape).unwrap();
    disp.clear(Rgb565::BLACK).unwrap();

    loop {
        disp.play(&mut tx, &mut rx)
    }
}

trait PlayPong {
    fn play(&self, tx: &mut Tx<USART2>, rx: &mut Rx<USART2>) -> ();
}

impl<SPI, DC, RST> PlayPong for ST7735<SPI, DC, RST>
where
    SPI: hal::hal::blocking::spi::Write<u8>,
    DC: OutputPin,
    RST: OutputPin,
{
    fn play(&self, tx: &mut Tx<USART2>, rx: &mut Rx<USART2>) -> () {
        ()
    }
}

#[exception]
#[allow(non_snake_case)]
unsafe fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}
