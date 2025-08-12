use std::{f32::consts::TAU, ops::Add};

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
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_circle_v(
            self.cur_pos,
            2.0,
            Color::WHITE,
        );
    }
}
