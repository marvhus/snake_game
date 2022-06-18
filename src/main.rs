use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};

use std::collections::LinkedList;
use std::iter::FromIterator;

pub const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
pub const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
pub const YELLOW: [f32; 4] = [1.0, 1.0, 0.0, 1.0];

pub const WIDTH: i32 = 40;
pub const HEIGHT: i32 = 40;

#[derive(Clone, PartialEq)]
pub enum Direction {
    Right, Left, Up, Down
}

mod game;
mod snake;
mod food;

use game::Game;
use snake::Snake;
use food::Food;

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: GlutinWindow = WindowSettings::new(
        "Snake Game",
        [40 * 20, 40 * 20]
    ).opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap()
    ;

    let mut game = Game {
        gl: GlGraphics::new(opengl),
        snake: Snake {
            body: LinkedList::from_iter((vec![
                          (WIDTH / 2, HEIGHT / 2),
                          (WIDTH / 2, HEIGHT / 2 + 1),
                          (WIDTH / 2, HEIGHT / 2 + 2),
                          (WIDTH / 2, HEIGHT / 2 + 3)
            ]).into_iter()),
            dir: Direction::Up,
            changed_dir: false,
        },
        food: Food {
            rng: rand::thread_rng(),
            pos_x: 0,
            pos_y: 0,
            moved: false,
        },
        points: 0,
        game_over: false,
    };
    game.food.new_pos(&game.snake.body);

    let mut events = Events::new(EventSettings::new()).ups(4);
    while let Some(e) = events.next(&mut window) {

        if game.game_over {
            break;
        }

        if let Some(r) = e.render_args() {
            game.render(&r);
        }

        if let Some(_u) = e.update_args() {
            game.update();
        }

        if let Some(k) = e.button_args() {
            if k.state == ButtonState::Press {
                game.pressed(&k.button);
            }
        }

    }

}
