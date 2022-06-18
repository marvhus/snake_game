use piston::input::*;
use opengl_graphics::GlGraphics;

use crate::{GREEN, Direction, snake::Snake, food::Food, WIDTH, HEIGHT};

pub struct Game {
    pub gl: GlGraphics,
    pub snake: Snake,
    pub food: Food,
    pub points: i32,
    pub game_over: bool,
}

impl Game {
    pub fn render(&mut self, arg: &RenderArgs) {
        self.gl.draw(arg.viewport(), |_c, gl| {
            graphics::clear(GREEN, gl);
        });

        self.snake.render(&mut self.gl, arg);
        self.food.render(&mut self.gl, arg);
    }

    pub fn update(&mut self) {
        self.snake.update(&self.food.moved);
        self.food.moved = false;
        self.food.update(&self.snake.body);

        if self.food.moved {
            self.points += 1;
            println!("Another one, {}", self.points);
        }

        let mut snake_body_clone = self.snake.body.clone();
        snake_body_clone.pop_front();
        if snake_body_clone.contains(self.snake.body.front().unwrap())
            || self.snake.body.front().unwrap().0 < 0
            || self.snake.body.front().unwrap().1 < 0
            || self.snake.body.front().unwrap().0 >= WIDTH
            || self.snake.body.front().unwrap().1 >= HEIGHT
        {
            self.game_over = true;
            println!("Final score: {}", self.points);
        }
    }

    pub fn pressed(&mut self, btn: &Button) {
        let last_direction = self.snake.dir.clone();

        if !self.snake.changed_dir {
            self.snake.dir = match btn {
                &Button::Keyboard(Key::Up)
                    if last_direction != Direction::Down => Direction::Up,
                &Button::Keyboard(Key::Down)
                    if last_direction != Direction::Up => Direction::Down,
                &Button::Keyboard(Key::Left)
                    if last_direction != Direction::Right => Direction::Left,
                &Button::Keyboard(Key::Right)
                    if last_direction != Direction::Left => Direction::Right,
                _ => last_direction.clone()
            };
        }

        if self.snake.dir != last_direction {
            self.snake.changed_dir = true;
        }

    }
}
