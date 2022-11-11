use embedded_graphics::{
    prelude::{Point, Size},
    primitives::{self, Rectangle},
};
use heapless::Vec;

use crate::game::physics::{MovingObject, Velocity};

use super::{
    super::physics::{BouncableObject, TimeTick},
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
    fn bounce(&mut self, screen: &Rectangle, new_postion: &Point) -> Result<Self, GameOver> {
        self.move_with_bounce(screen, new_postion);
        if let Some(winner) = self.get_winner(screen) {
            return Err(winner);
        }
        Ok(*self)
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
    fn left_player_has_lost_ball(&self, screen: &Rectangle) -> bool {
        self.position.x < screen.top_left.x
    }
    fn right_player_has_lost_ball(&self, screen: &Rectangle) -> bool {
        self.position.x > screen.top_left.x + (screen.size.width as i32)
    }
    fn move_with_bounce(&mut self, screen: &Rectangle, new_postion: &Point) {
        // TODO: support for multiple bounces?
        self.bounce_against_top_wall(screen, new_postion);
        self.bounce_against_bottom_wall(screen, new_postion);
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

    fn get_winner(&self, screen: &Rectangle) -> Option<GameOver> {
        if self.left_player_has_lost_ball(screen) {
            return Some(GameOver::RightWins);
        } else if self.right_player_has_lost_ball(screen) {
            return Some(GameOver::LeftWins);
        }
        None
    }
}
