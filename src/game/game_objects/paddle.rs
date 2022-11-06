use embedded_graphics::{
    prelude::{Point, Size},
    primitives::{self, Rectangle},
};
use heapless::Vec;

use super::{GameObject, ScreenObject};

#[derive(Clone, Copy, Debug)]
pub struct Paddle {
    pub top_left_pos: Point,
    pub x_size: u32,
    pub y_size: u32,
    pub has_moved: bool,
}

impl Default for Paddle {
    fn default() -> Self {
        Paddle {
            top_left_pos: Point { x: 0, y: 0 },
            y_size: 1,
            x_size: 1,
            has_moved: false,
        }
    }
}

impl GameObject for Paddle {
    fn set_position(&self, pos: Point) -> Self {
        Self {
            top_left_pos: pos,
            y_size: self.y_size,
            x_size: self.x_size,
            has_moved: self.has_moved,
        }
    }
    fn as_shapes(&self) -> Vec<ScreenObject, 2> {
        let mut shapes: Vec<ScreenObject, 2> = Vec::new();
        shapes
            .push(ScreenObject::Rectangle(primitives::Rectangle {
                top_left: self.top_left_pos,
                size: Size {
                    width: self.x_size,
                    height: self.y_size,
                },
            }))
            .unwrap();
        shapes
    }
    fn get_box_covering_object(&self) -> Rectangle {
        Rectangle {
            top_left: self.top_left_pos,
            size: Size {
                width: self.x_size,
                height: self.y_size,
            },
        }
    }
    fn is_within(&self, rectange: &Rectangle) -> bool {
        // TODO: refactor into class?
        let box_covering_object = self.get_box_covering_object();
        let considered_corners = [
            box_covering_object.top_left,
            box_covering_object.bottom_right().unwrap(),
        ];

        considered_corners
            .iter()
            .all(|corner| rectange.contains(*corner))
    }
}
