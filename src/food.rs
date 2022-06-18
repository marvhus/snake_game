use piston::input::*;
use opengl_graphics::GlGraphics;

use rand::{rngs::ThreadRng, Rng};
use std::collections::LinkedList;

use crate::{YELLOW, WIDTH, HEIGHT};

pub struct Food {
    pub rng: ThreadRng,
    pub pos_x: i32,
    pub pos_y: i32,
    pub moved: bool,
}

impl Food {

    pub fn render(&mut self, gl: &mut GlGraphics, arg: &RenderArgs) {
        let square = graphics::rectangle::square(
            (self.pos_x * 20) as f64,
            (self.pos_y * 20) as f64,
            20_f64
        );

        gl.draw(arg.viewport(), |c, gl| {
            let transform = c.transform;
            graphics::rectangle(YELLOW, square, transform, gl);
        });
    }

    pub fn new_pos(&mut self, snake: &LinkedList<(i32, i32)>) {
        // Get all free positions
        let free_positions =
            (0..HEIGHT).flat_map(|y| (0..WIDTH).map(move |x| (x, y)))
            .filter(|pos| !snake.contains(pos))
            .collect::<Vec<_>>();
        // Set new position that is free
        let pos = free_positions.get(self.rng.gen_range(0..free_positions.len())).unwrap();
        self.pos_x = pos.0;
        self.pos_y = pos.1;
    }

    pub fn update(&mut self, snake: &LinkedList<(i32, i32)>) {
        let snake_head = snake.front().unwrap();
        if self.pos_x == snake_head.0 && self.pos_y == snake_head.1 {
            self.new_pos(snake);
            self.moved = true;
        }
    }
}
