use embedded_graphics::primitives::{Primitive, PrimitiveStyle};
use embedded_graphics::Drawable;
use embedded_graphics::{pixelcolor::Rgb565, prelude::*};
use heapless::Vec;
use st7735_lcd::ST7735;
use stm32f4xx_hal::hal::digital::v2::OutputPin;

use super::game_objects::ScreenObject;
use hal::hal::blocking::spi;

use stm32f4xx_hal as hal;
/*
*
   mut disp: ST7735<SPI, DC, RST>,
*
   const PL: char,
   const PR: char,
   const NL: u8,
   const NR: u8,
* */
pub struct Display<'a, SPI: spi::Write<u8>, DC: OutputPin, RST: OutputPin> {
    pub display: &'a mut ST7735<SPI, DC, RST>,
}

pub trait Graphics {
    fn clear(&mut self, objects: &Vec<ScreenObject, 8>);
    fn draw(&mut self, objects: &Vec<ScreenObject, 8>);
}

struct ObjectColors {
    paddle_color: Rgb565,
    ball_color: Rgb565,
}

impl<'a, SPI: spi::Write<u8>, DC: OutputPin, RST: OutputPin> Graphics
    for Display<'a, SPI, DC, RST>
{
    fn clear(&mut self, objects: &Vec<ScreenObject, 8>) {
        self.draw_objects_in_colors(&objects, self.get_clear_object_colors());
    }
    fn draw(&mut self, objects: &Vec<ScreenObject, 8>) {
        self.draw_objects_in_colors(&objects, self.get_object_colors());
    }
}

impl<'a, SPI: spi::Write<u8>, DC: OutputPin, RST: OutputPin> Display<'a, SPI, DC, RST> {
    fn draw_objects_in_colors(&mut self, objects: &Vec<ScreenObject, 8>, colors: ObjectColors) {
        for shape in objects.iter() {
            match shape {
                ScreenObject::Rectangle(rectangle) => {
                    rectangle
                        .into_styled(PrimitiveStyle::with_fill(colors.paddle_color))
                        .draw(self.display)
                        .unwrap();
                }
                ScreenObject::Circle(circle) => {
                    circle
                        .into_styled(PrimitiveStyle::with_fill(colors.ball_color))
                        .draw(self.display)
                        .unwrap();
                }
            }
        }
    }
    fn get_object_colors(&self) -> ObjectColors {
        ObjectColors {
            paddle_color: Rgb565::YELLOW,
            ball_color: Rgb565::GREEN,
        }
    }
    fn get_clear_object_colors(&self) -> ObjectColors {
        ObjectColors {
            paddle_color: Rgb565::BLACK,
            ball_color: Rgb565::BLACK,
        }
    }
}
