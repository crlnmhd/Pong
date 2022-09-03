use embedded_graphics::geometry::Point;
use embedded_graphics::geometry::Size;
use embedded_graphics::primitives;
use heapless::Vec;

trait GameObject {
    fn with_pos(&self, pos: Point) -> Self;
    fn as_shapes(&self) -> Vec<ScreenObject, 2>; // Note: needlessly increasing N leads to much larger
                                                 // vectors due to to enum size.
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

#[derive(Clone, Debug)]
pub enum ScreenObject {
    Rectangle(primitives::Rectangle),
    Circle(primitives::Circle),
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
}

#[derive(Clone, Copy)]
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
    pub fn get_content_to_display(&self) -> Vec<ScreenObject, 8> {
        let mut all_shapes: Vec<ScreenObject, 8> = Vec::new();
        // TODO: improve with less copying.
        all_shapes.extend(self.left_paddle.as_shapes().iter().cloned());
        all_shapes.extend(self.right_paddle.as_shapes().iter().cloned());
        all_shapes.extend(self.ball.as_shapes().iter().cloned());
        all_shapes
    }
    pub fn set_ball_position(&mut self, position: Point) {
        // Fixme bounds check
        self.ball.position = position;
    }
    pub fn set_left_paddle_position(&mut self, top_left_pos: Point) {
        // Fixme bounds check
        self.left_paddle.top_left_pos = top_left_pos;
    }
    pub fn set_right_paddle_position(&mut self, top_left_pos: Point) {
        // Fixme bounds check
        self.right_paddle.top_left_pos = top_left_pos;
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
    pub fn new(x_size: u32, y_size: u32) -> GameBuilder {
        let mut gamebuilder = GameBuilder::default();
        gamebuilder.x_size = x_size;
        gamebuilder.y_size = y_size;
        let gamebuilder = gamebuilder;
        gamebuilder
    }
    pub fn ball_radius(&self, radius: u32) -> GameBuilder {
        GameBuilder {
            left_paddle: self.left_paddle,
            right_paddle: self.right_paddle,
            ball: Ball {
                position: self.ball.position,
                radius,
                velocity: self.ball.velocity,
            },
            x_size: self.x_size,
            y_size: self.y_size,
        }
    }
    pub fn paddle_size(&self, size: Size) -> GameBuilder {
        GameBuilder {
            left_paddle: Paddle {
                top_left_pos: self.left_paddle.top_left_pos,
                x_size: size.width,
                y_size: size.height,
            },
            right_paddle: Paddle {
                top_left_pos: self.right_paddle.top_left_pos,
                x_size: size.width,
                y_size: size.height,
            },
            ball: self.ball,
            x_size: self.x_size,
            y_size: self.y_size,
        }
    }
    pub fn build(self) -> Game {
        Game {
            left_paddle: self.left_paddle,
            right_paddle: self.right_paddle,
            x_pixels: self.x_size,
            y_size: self.y_size,
            ball: self.ball,
        }
    }
}
