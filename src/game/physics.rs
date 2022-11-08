use embedded_graphics::{prelude::Point, primitives::Rectangle};

use super::game_objects::GameOver;

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
    fn bounce(&mut self, screen: &Rectangle, new_position: &Point) -> Result<Self, GameOver>
    where
        Self: Sized;
}

pub trait MovingObject {
    fn get_relative_movement(&self, time: &TimeTick) -> Point;
    fn get_velocity(&self) -> Velocity;
}
