use embedded_graphics::primitives::Rectangle;

use super::game_objects::{GameObject, GameOver};

#[derive(Copy, Clone, Default)]
pub struct TimeTick {
    pub max_paddle_movement: u32,
    pub max_ball_movement: u32,
}

pub trait BouncableObject {
    fn bounce(&self, screen: &Rectangle, time: &TimeTick) -> Result<Self, GameOver>
    where
        Self: Sized;
}
