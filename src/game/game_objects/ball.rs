use embedded_graphics::{
    prelude::{Point, Size},
    primitives::{self, Rectangle},
};
use heapless::Vec;

use crate::game::{
    input::LeftRightPosition,
    physics::{MovingObject, Velocity},
};

use super::{
    super::physics::{BouncableObject, TimeTick},
    paddle::Paddle,
    GameOver, GameState,
};

use super::{GameObject, ScreenObject};

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
    fn bounce_aginst_walls(&mut self, screen: &Rectangle, new_position: &Point) {
        self.bounce_against_top_wall(screen, new_position);
        self.bounce_against_bottom_wall(screen, new_position);
    }
    fn bounce_against_paddles(
        &mut self,
        left_paddle: &super::paddle::Paddle,
        right_paddle: &super::paddle::Paddle,
    ) {
        let moving_towards_left_paddle = self.velocity.vx < 0;

        let has_collided = match moving_towards_left_paddle {
            true => self.has_hit_paddle(left_paddle),
            false => self.has_hit_paddle(right_paddle),
        };
        if has_collided {
            self.invert_horizontal_velocity();
        }
    }
}

impl MovingObject for Ball {
    fn get_velocity(&self) -> Velocity {
        self.velocity.clone()
    }
    fn get_relative_movement(&self, time: &TimeTick) -> Point {
        let velocity = self.get_velocity();
        let dx = velocity.vx * (time.time_step as i32);
        let dy = velocity.vy * (time.time_step as i32);
        Point { x: dx, y: dy }
    }
}

impl Ball {
    pub fn left_player_has_lost_ball(&self, screen: &Rectangle) -> bool {
        self.position.x < screen.top_left.x
    }
    pub fn right_player_has_lost_ball(&self, screen: &Rectangle) -> bool {
        self.position.x > screen.top_left.x + (screen.size.width as i32)
    }

    fn bounce_against_top_wall(&mut self, screen: &Rectangle, new_postion: &Point) {
        let top_overshoot: i32 = screen.top_left.y - new_postion.y;
        self.position = *new_postion;
        if top_overshoot > 0 {
            let new_height = screen.top_left.y + top_overshoot; // y grows downward
            self.position.y = new_height;
            self.invert_vertical_velocity();
        }
    }

    fn bounce_against_bottom_wall(&mut self, screen: &Rectangle, new_postion: &Point) {
        let bottom_overshoot = new_postion.y - screen.bottom_right().unwrap().y;
        self.position = *new_postion;
        if bottom_overshoot > 0 {
            let new_height = screen.bottom_right().unwrap().y - bottom_overshoot;
            self.position.y = new_height;
            self.invert_vertical_velocity();
        }
    }

    fn invert_vertical_velocity(&mut self) {
        self.velocity.vy *= -1;
    }

    fn invert_horizontal_velocity(&mut self) {
        self.velocity.vx *= -1;
    }

    fn has_hit_paddle(&self, paddle: &Paddle) -> bool {
        let paddle_area = paddle.get_box_covering_object();
        paddle_area.contains(self.position) // FIXME: improved checking.
    }
}
