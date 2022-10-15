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
use embedded_graphics::primitives::PrimitiveStyle;
use embedded_graphics::{draw_target::DrawTarget, pixelcolor::Rgb565, prelude::*};

use st7735_lcd::{Orientation, ST7735};

use hal::prelude::*;

mod game;
use game::game_object::*;
use game::input;

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
        28.MHz().into(),
        &clocks,
    );

    let mut delay = cortex_m::delay::Delay::new(cp.SYST, clocks.hclk().to_Hz());
    //hal::timer::Timer::new(d&p.TIM2, &clocks);

    let x_pixels: u32 = 160;
    let y_pixels: u32 = 128;

    let mut disp = ST7735::new(spi, dc, rst, true, false, x_pixels, y_pixels);
    disp.init(&mut delay).unwrap();
    disp.set_orientation(&Orientation::Landscape).unwrap();
    disp.clear(Rgb565::BLACK).unwrap();

    let mut pong: Game = GameBuilder::new(x_pixels, y_pixels)
        .ball_radius(5)
        .paddle_size(Size {
            width: 10,
            height: 40,
        })
        .build();
    pong.set_right_paddle_position(Point { x: 10, y: 20 });
    pong.set_left_paddle_position(Point { x: (50), y: 50 });

    let mut x: i32 = 5;
    let y: i32 = 50;

    loop {
        disp.clear(Rgb565::BLACK).unwrap();
        for shape in pong.get_content_to_display().into_iter() {
            match shape {
                ScreenObject::Rectangle(rectangle) => {
                    rectangle
                        .into_styled(PrimitiveStyle::with_fill(Rgb565::GREEN))
                        .draw(&mut disp)
                        .unwrap();
                }
                ScreenObject::Circle(circle) => {
                    circle
                        .into_styled(PrimitiveStyle::with_fill(Rgb565::BLACK))
                        .draw(&mut disp)
                        .unwrap();
                    x += 5;
                    x %= x_pixels as i32 - 5;
                    x += 5;
                    pong.set_ball_position(Point { x, y });
                    circle
                        .into_styled(PrimitiveStyle::with_fill(Rgb565::GREEN))
                        .draw(&mut disp)
                        .unwrap();
                }
            }
        }
    }
}
