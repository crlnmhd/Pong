use embedded_graphics::{prelude::Point, primitives::Rectangle};

use super::{
    game_objects::{paddle::Paddle, GameOver},
    input::LeftRightPosition,
};

#[derive(Clone, Copy, Debug)]
pub struct Velocity {
    // Direction of movement from the balls frame of reference.
    pub vx: i32,
    pub vy: i32,
}

#[derive(Copy, Clone, Default)]
pub struct TimeTick {
    pub max_paddle_movement: u32,
    pub max_ball_movement: u32,
    pub time_step: u32,
}

pub trait BouncableObject {
    fn bounce_aginst_walls(&mut self, screen: &Rectangle, new_position: &Point);
    fn bounce_against_paddles(&mut self, left_paddle: &Paddle, right_padde: &Paddle);
}

pub trait MovingObject {
    fn get_relative_movement(&self, time: &TimeTick) -> Point;
    fn get_velocity(&self) -> Velocity;
}
