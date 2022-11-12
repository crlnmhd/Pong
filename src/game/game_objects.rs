pub mod ball;
pub mod paddle;

use embedded_graphics::geometry::Point;
use embedded_graphics::geometry::Size;
use embedded_graphics::primitives;
use embedded_graphics::primitives::Rectangle;
use heapless::Vec;

use ball::Ball;
use paddle::Paddle;

use super::input::InpuDirection;
use super::input::LeftRightPosition;
use super::physics::BouncableObject;
use super::physics::MovingObject;
use super::physics::TimeTick;
use super::physics::Velocity;

#[derive(Debug)]
pub enum GameOver {
    LeftWins,
    RightWins,
}

#[derive(Debug)]
pub enum GameState {
    Ongoing,
    Finnished(GameOver),
}

pub trait GameObject {
    fn set_position(&self, pos: Point) -> Self;
    fn as_shapes(&self) -> Vec<ScreenObject, 2>; // Note: needlessly increasing N leads to much larger
                                                 // vectors due to to enum size.
    fn get_box_covering_object(&self) -> Rectangle;
    fn is_within(&self, rectange: &Rectangle) -> bool;
}

#[derive(Clone, Debug)]
pub enum ScreenObject {
    Rectangle(primitives::Rectangle),
    Circle(primitives::Circle),
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
    time_tick: TimeTick,
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
    pub fn let_ball_move(&mut self) -> GameState {
        let ball_movement = self.ball.get_relative_movement(&self.time_tick);
        let screen = self.get_screen_dimensions();

        // TODO: refactor into method. This is not clean.
        let mut new_postion = self.ball.position.clone();
        new_postion.x += ball_movement.x;
        new_postion.y += ball_movement.y;

        match self.bounce_ball(&screen, &new_postion) {
            Ok(ball) => {
                self.ball = ball;
                self.ball.has_moved = true;
                GameState::Ongoing
            }
            Err(game_over_with_winner) => GameState::Finnished(game_over_with_winner),
        }
    }
    pub fn move_paddle(&mut self, side: &LeftRightPosition, direction: InpuDirection) {
        let step_size = self.time_tick.max_paddle_movement as i32;
        match direction {
            InpuDirection::Up => self.move_paddle_in_y_direction(side, -step_size),
            InpuDirection::Down => self.move_paddle_in_y_direction(side, step_size),
            InpuDirection::Stay => {}
        };
    }
    pub fn start_new_game(&mut self) {
        self.ball = self.ball.set_position(self.get_default_ball_position());
    }

    fn move_paddle_in_y_direction(&mut self, side: &LeftRightPosition, y_step: i32) {
        let screen = self.get_screen_dimensions();
        let paddle = match side {
            LeftRightPosition::Left => &mut self.left_paddle,
            LeftRightPosition::Right => &mut self.right_paddle,
        };
        if y_step != 0 {
            let moved_paddle = Paddle {
                top_left_pos: Point {
                    x: paddle.top_left_pos.x,
                    y: paddle.top_left_pos.y + y_step,
                },
                x_size: paddle.x_size,
                y_size: paddle.y_size,
                has_moved: true,
            };
            if moved_paddle.is_within(&screen) {
                *paddle = moved_paddle;
            }
        }
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
    fn get_default_ball_position(&self) -> Point {
        Point { x: 50, y: 50 }
    }
    fn bounce_ball(&mut self, screen: &Rectangle, new_position: &Point) -> Result<Ball, GameOver> {
        self.ball.bounce_aginst_walls(screen, new_position);
        self.ball
            .bounce_against_paddles(&self.left_paddle, &self.right_paddle);
        if let Some(winner) = self.get_winner(screen) {
            Err(winner)
        } else {
            Ok(self.ball)
        }
    }
    fn get_winner(&self, screen: &Rectangle) -> Option<GameOver> {
        if self.ball.left_player_has_lost_ball(screen) {
            return Some(GameOver::RightWins);
        } else if self.ball.right_player_has_lost_ball(screen) {
            return Some(GameOver::LeftWins);
        }
        None
    }
}

#[derive(Default)]
pub struct GameBuilder {
    left_paddle: Paddle,
    right_paddle: Paddle,
    ball: Ball,
    x_pixels: u32,
    y_pixels: u32,
    time_tick: TimeTick,
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
            time_tick: self.time_tick,
        }
    }
    pub fn paddle_size(&self, size: Size) -> GameBuilder {
        GameBuilder {
            left_paddle: Paddle {
                top_left_pos: Point { x: 0, y: 0 },
                x_size: size.width,
                y_size: size.height,
                has_moved: true,
            },
            right_paddle: Paddle {
                top_left_pos: Point {
                    x: (self.x_pixels - size.width) as i32,
                    y: 0,
                },
                x_size: size.width,
                y_size: size.height,
                has_moved: true,
            },
            ball: self.ball,
            x_pixels: self.x_pixels,
            y_pixels: self.y_pixels,
            time_tick: self.time_tick,
        }
    }
    pub fn time_tick(&self, time_tick: TimeTick) -> GameBuilder {
        GameBuilder {
            left_paddle: self.left_paddle,
            right_paddle: self.right_paddle,
            ball: self.ball,
            x_pixels: self.x_pixels,
            y_pixels: self.y_pixels,
            time_tick,
        }
    }
    pub fn initial_ball_velocity(&self, velocity: Velocity) -> GameBuilder {
        GameBuilder {
            left_paddle: self.left_paddle,
            right_paddle: self.right_paddle,
            ball: Ball {
                position: self.ball.position,
                radius: self.ball.radius,
                velocity,
                has_moved: self.ball.has_moved,
            },
            x_pixels: self.x_pixels,
            y_pixels: self.y_pixels,
            time_tick: self.time_tick,
        }
    }

    pub fn build(self) -> Game {
        Game {
            left_paddle: self.left_paddle,
            right_paddle: self.right_paddle,
            x_pixels: self.x_pixels,
            y_pixels: self.y_pixels,
            ball: self.ball,
            time_tick: self.time_tick,
        }
    }
}
