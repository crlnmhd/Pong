use super::physics::TimeTick;
use embedded_graphics::geometry::Point;
use embedded_graphics::geometry::Size;
use embedded_graphics::primitives;
use embedded_graphics::primitives::Rectangle;
use heapless::Vec;

trait GameObject {
    fn set_position(&self, pos: Point) -> Self;
    fn as_shapes(&self) -> Vec<ScreenObject, 2>; // Note: needlessly increasing N leads to much larger
                                                 // vectors due to to enum size.
    fn get_box_covering_object(&self) -> Rectangle;
    fn is_within(&self, rectange: &Rectangle) -> bool;
}

#[derive(Clone, Copy, Debug)]
struct Velocity {
    // Direction of movement from the balls frame of reference.
    vx: i32,
    vy: i32,
}

#[derive(Clone, Copy, Debug)]
struct Paddle {
    top_left_pos: Point,
    x_size: u32,
    y_size: u32,
    has_moved: bool,
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

trait BouncableObject {
    fn update_location<T: GameObject>(time: &TimeTick, object: T) -> T;
}

#[derive(Clone, Copy, Debug)]
struct Ball {
    position: Point,
    radius: u32,
    velocity: Velocity,
    has_moved: bool,
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

#[derive(Debug)]
enum DrawableGameOject<'a> {
    Paddle(&'a mut Paddle),
    Ball(&'a mut Ball),
}

impl DrawableGameOject<'_> {
    fn set_moved_status(&mut self, new_status: bool) {
        match self {
            DrawableGameOject::Ball(ref mut ball) => ball.has_moved = new_status,
            DrawableGameOject::Paddle(ref mut paddle) => paddle.has_moved = new_status,
        };
    }
    fn get_moved_status(&self) -> bool {
        match &self {
            DrawableGameOject::Ball(ball) => ball.has_moved.clone(),
            DrawableGameOject::Paddle(paddle) => paddle.has_moved.clone(),
        }
    }
    fn as_shapes(&self) -> Vec<ScreenObject, 2> {
        match self {
            DrawableGameOject::Ball(ball) => ball.as_shapes(),
            DrawableGameOject::Paddle(paddle) => paddle.as_shapes(),
        }
    }
}

pub struct Game {
    left_paddle: Paddle,
    right_paddle: Paddle,
    x_pixels: u32,
    y_pixels: u32,
    ball: Ball,
}

impl Game {
    pub fn builder() -> GameBuilder {
        GameBuilder::default()
    }
    pub fn valid_board(&self) -> bool {
        true
        //         for object in self.on_screen_objects(){
        //             match object{
        //                 Paddle { top_left_pos, x_size, y_size, has_moved } =>
        //             }
        //         }
    }

    fn on_screen_objects(&mut self) -> Vec<DrawableGameOject, 3> {
        let mut objects: Vec<DrawableGameOject, 3> = Vec::new();
        objects
            .push(DrawableGameOject::Ball(&mut self.ball))
            .unwrap();
        objects
            .push(DrawableGameOject::Paddle(&mut self.right_paddle))
            .unwrap();
        objects
            .push(DrawableGameOject::Paddle(&mut self.left_paddle))
            .unwrap();
        objects
    }
    pub fn get_content_to_display(&mut self) -> Vec<ScreenObject, 8> {
        let mut all_shapes: Vec<ScreenObject, 8> = Vec::new();
        // TODO: improve with less copying. from slices?
        all_shapes.extend(self.left_paddle.as_shapes().iter().cloned());
        all_shapes.extend(self.right_paddle.as_shapes().iter().cloned());
        all_shapes.extend(self.ball.as_shapes().iter().cloned());

        all_shapes
    }
    pub fn reset_position_update_indicators(&mut self) {
        for object in self.on_screen_objects().iter_mut() {
            object.set_moved_status(false);
        }
    }
    pub fn get_moved_content(&mut self) -> Vec<ScreenObject, 8> {
        let mut moved_shapes: Vec<ScreenObject, 8> = Vec::new();
        for moved_object in self
            .on_screen_objects()
            .iter_mut()
            .filter(|object| object.get_moved_status())
        {
            moved_shapes.extend(moved_object.as_shapes());
        }
        moved_shapes
    }
    pub fn set_ball_position(&mut self, position: Point) -> Point {
        if position != self.ball.position {
            let moved_ball = Ball {
                position,
                radius: self.ball.radius,
                velocity: self.ball.velocity,
                has_moved: true,
            };
            let screen = self.get_screen_dimensions();
            if moved_ball.is_within(&screen) {
                self.ball = moved_ball;
            }
        }
        self.ball.position.clone()
    }
    pub fn set_left_paddle_position(&mut self, top_left_pos: Point) -> Point {
        if top_left_pos != self.left_paddle.top_left_pos {
            let moved_paddle = Paddle {
                top_left_pos,
                x_size: self.left_paddle.x_size,
                y_size: self.left_paddle.y_size,
                has_moved: true,
            };
            let screen = self.get_screen_dimensions();
            if moved_paddle.is_within(&screen) {
                self.left_paddle = moved_paddle;
            }
        }
        self.left_paddle.top_left_pos.clone()
    }
    pub fn set_right_paddle_position(&mut self, top_left_pos: Point) -> Point {
        if top_left_pos != self.right_paddle.top_left_pos {
            let moved_paddle = Paddle {
                top_left_pos,
                x_size: self.right_paddle.x_size,
                y_size: self.right_paddle.y_size,
                has_moved: true,
            };
            let screen = self.get_screen_dimensions();
            if moved_paddle.is_within(&screen) {
                self.right_paddle = moved_paddle;
            }
        }
        self.right_paddle.top_left_pos.clone()
    }
    fn get_screen_dimensions(&self) -> Rectangle {
        Rectangle {
            top_left: Point { x: 0, y: 0 },
            size: Size {
                width: self.x_pixels,
                height: self.y_pixels,
            },
        }
    }
}

#[derive(Default)]
pub struct GameBuilder {
    left_paddle: Paddle,
    right_paddle: Paddle,
    ball: Ball,
    x_pixels: u32,
    y_pixels: u32,
}

impl GameBuilder {
    pub fn new(x_size: u32, y_size: u32) -> GameBuilder {
        let mut gamebuilder = GameBuilder::default();
        gamebuilder.x_pixels = x_size;
        gamebuilder.y_pixels = y_size;
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
                has_moved: true,
            },
            x_pixels: self.x_pixels,
            y_pixels: self.y_pixels,
        }
    }
    pub fn paddle_size(&self, size: Size) -> GameBuilder {
        GameBuilder {
            left_paddle: Paddle {
                top_left_pos: self.left_paddle.top_left_pos,
                x_size: size.width,
                y_size: size.height,
                has_moved: true,
            },
            right_paddle: Paddle {
                top_left_pos: self.right_paddle.top_left_pos,
                x_size: size.width,
                y_size: size.height,
                has_moved: true,
            },
            ball: self.ball,
            x_pixels: self.x_pixels,
            y_pixels: self.y_pixels,
        }
    }
    pub fn build(self) -> Game {
        Game {
            left_paddle: self.left_paddle,
            right_paddle: self.right_paddle,
            x_pixels: self.x_pixels,
            y_pixels: self.y_pixels,
            ball: self.ball,
        }
    }
}
