use embedded_graphics::{
    prelude::{Point, Size},
    primitives::{self, Rectangle},
};
use heapless::Vec;

use super::super::physics::{BouncableObject, TimeTick};

use super::{GameObject, ScreenObject, Velocity};

#[derive(Clone, Copy, Debug)]
pub struct Ball {
    pub position: Point,
    pub radius: u32,
    pub velocity: Velocity,
    pub has_moved: bool,
}

impl Default for Ball {
    fn default() -> Self {
        Ball {
            position: Point { x: 0, y: 0 },
            radius: 1,
            velocity: Velocity { vx: 0, vy: 0 },
            has_moved: false,
        }
    }
}

impl GameObject for Ball {
    fn set_position(&self, pos: Point) -> Self {
        Self {
            position: pos,
            radius: self.radius,
            velocity: self.velocity,
            has_moved: self.has_moved,
        }
    }
    fn as_shapes(&self) -> Vec<ScreenObject, 2> {
        let mut shapes: Vec<ScreenObject, 2> = Vec::new();
        shapes
            .push(ScreenObject::Circle(primitives::Circle {
                top_left: self.position,
                diameter: self.radius * 2,
            }))
            .unwrap();
        shapes
    }
    fn get_box_covering_object(&self) -> Rectangle {
        let top_left = Point {
            x: self.position.x - self.radius as i32,
            y: self.position.y - self.radius as i32,
        };
        let diameter = self.radius * 2;
        Rectangle {
            top_left,
            size: Size {
                width: diameter,
                height: diameter,
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

impl BouncableObject for Ball {
    fn update_location<T: GameObject>(time: &TimeTick, object: T) -> T {
        object // FIXME implement
    }
}
