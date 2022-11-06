use super::game_objects::GameObject;
use core::time::Duration;

pub struct TimeTick {
    max_paddle_movement: u32,
    max_ball_movement: u32,
    delay: Duration,
}

pub trait BouncableObject {
    fn update_location<T: GameObject>(time: &TimeTick, object: T) -> T;
}
