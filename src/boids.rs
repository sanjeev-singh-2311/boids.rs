use std::f32::consts::TAU;

use rand::random_range;
use raylib::{
    color::Color,
    math::Vector2,
    prelude::{RaylibDraw, RaylibDrawHandle},
};

use crate::{WIN_HEIGHT, WIN_WIDTH};

#[derive(Debug, Default)]
pub struct Boid {
    cur_pos: Vector2,
    velocity: Vector2,
    acceleration: Vector2,

    local_flock: Vec<Boid>,
    steering_vectors: Vec<Vector2>,
}

impl Boid {
    pub fn new() -> Boid {
        let rand_pos = Vector2::new(
            random_range(0.0..=WIN_WIDTH),
            random_range(0.0..=WIN_HEIGHT),
        );

        let rand_angle = random_range(0.0..=TAU);
        let direction_vec = Vector2::new(rand_angle.cos(), rand_angle.sin());

        let speed = random_range(0.0..=5.0);

        Boid {
            cur_pos: rand_pos,
            velocity: direction_vec.scale_by(speed),
            ..Default::default()
        }
    }

    pub fn update(&mut self) {
        self.velocity += self.acceleration;
        self.cur_pos += self.velocity;

        self.wrap(10.0);
    }

    pub fn wrap(&mut self, padding: f32) {
        let padded_height = WIN_HEIGHT + padding;
        let padded_width = WIN_WIDTH + padding;

        if self.cur_pos.x > padded_width {
            self.cur_pos.x -= padded_width;
        } else if self.cur_pos.x < 0.0 - padding {
            self.cur_pos.x += padded_width;
        }

        if self.cur_pos.y > padded_height {
            self.cur_pos.y -= padded_height;
        } else if self.cur_pos.y < 0.0 - padding {
            self.cur_pos.y += padded_height;
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_circle_v(
            self.cur_pos,
            2.0,
            Color::WHITE,
        );
    }
}
