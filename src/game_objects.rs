use embedded_graphics::{mono_font::mapping::GlyphMapping, prelude::Point};

trait GameObject {
    fn with_pos(&self, pos: Point) -> Self;
}

#[derive(Clone, Copy)]
struct Velocity {
    // Direction of movement from the balls frame of reference.
    direction: Point,
    speed: u32,
}
#[derive(Clone, Copy)]
struct Paddle {
    top_left_pos: Point,
    x_size: u32,
    y_size: u32,
}

impl Default for Paddle {
    fn default() -> Self {
        Paddle {
            top_left_pos: Point { x: 0, y: 0 },
            y_size: 1,
            x_size: 1,
        }
    }
}

impl GameObject for Paddle {
    fn with_pos(&self, pos: Point) -> Self {
        Paddle {
            top_left_pos: pos,
            y_size: self.y_size,
            x_size: self.x_size,
        }
    }
}

struct Ball {
    position: Point,
    radius: u32,
    velocity: Velocity,
}

impl Default for Ball {
    fn default() -> Self {
        Ball {
            position: Point { x: 0, y: 0 },
            radius: 1,
            velocity: Velocity {
                direction: Point { x: 0, y: 0 },
                speed: 0,
            },
        }
    }
}

impl GameObject for Ball {
    fn with_pos(&self, pos: Point) -> Self {
        Ball {
            position: pos,
            radius: self.radius,
            velocity: self.velocity,
        }
    }
}
pub enum Paddles {
    Left,
    Right,
}

pub struct Game {
    left_paddle: Paddle,
    right_paddle: Paddle,
    x_pixels: u32,
    y_size: u32,
    ball: Ball,
}

impl Game {
    pub fn builder() -> GameBuilder {
        GameBuilder::default()
    }
}

#[derive(Default)]
pub struct GameBuilder {
    left_paddle: Paddle,
    right_paddle: Paddle,
    ball: Ball,
    x_size: u32,
    y_size: u32,
}

impl GameBuilder {
    pub fn new(self, x_size: u32, y_size: u32) -> GameBuilder {
        let mut gamebuilder = GameBuilder::default();
        gamebuilder.x_size = x_size;
        gamebuilder.y_size = y_size;
        let gamebuilder = gamebuilder;
        gamebuilder
    }
    fn ball_position(&self, point: Point) -> GameBuilder {
        GameBuilder {
            left_paddle: self.left_paddle,
            right_paddle: self.right_paddle,
            ball: Ball {
                position: point,
                radius: self.ball.radius,
                velocity: self.ball.velocity,
            },
            x_size: self.x_size,
            y_size: self.y_size,
        }
    }
    fn build(self) -> Game {
        Game {
            left_paddle: self.left_paddle,
            right_paddle: self.right_paddle,
            x_pixels: self.x_size,
            y_size: self.y_size,
            ball: self.ball,
        }
    }
}
