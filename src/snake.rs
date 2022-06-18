use piston::input::*;
use opengl_graphics::GlGraphics;

use std::collections::LinkedList;

use crate::{RED, Direction};

pub struct Snake {
    pub body: LinkedList<(i32, i32)>,
    pub dir: Direction,
    pub changed_dir: bool,
}

impl Snake {
    pub fn render(&mut self, gl: &mut GlGraphics, arg: &RenderArgs) {
        let squares: Vec<graphics::types::Rectangle> = self.body
            .iter()
            .map(|&(x, y)| {
                graphics::rectangle::square(
                    (x * 20) as f64,
                    (y * 20) as f64,
                    20_f64
                )
            })
            .collect();

        gl.draw(arg.viewport(), |c, gl| {
           let transform = c.transform;
            squares.into_iter()
                .for_each(|square| graphics::rectangle(RED, square, transform, gl))
        });
    }

    pub fn update(&mut self, food_moved: &bool) {
        let mut new_head = (*self.body.front().expect("Snake has no body")).clone();
        match self.dir {
            Direction::Left => new_head.0 -= 1,
            Direction::Right => new_head.0 += 1,
            Direction::Up => new_head.1 -= 1,
            Direction::Down => new_head.1 += 1,
        }

        self.body.push_front(new_head);

        if !food_moved {
            self.body.pop_back().unwrap();
        }
        self.changed_dir = false;
    }
}
