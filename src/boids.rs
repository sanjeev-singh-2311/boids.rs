use std::f32::consts::{FRAC_PI_2, TAU};

use std::cell::RefCell;
use std::rc::Rc;

use rand::random_range;
use raylib::prelude::*;
use raylib::{color::Color, math::Vector2};

use crate::config::{BLIND_SPOT, PERCEPTION_RADIUS, VELOCITY_LIMIT, WIN_HEIGHT, WIN_WIDTH};

pub type BoidRef = Rc<RefCell<Boid>>;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Boid {
    cur_pos: Vector2,
    velocity: Vector2,
    acceleration: Vector2,

    local_flock: Vec<BoidRef>,
    steering_vectors: Vec<Vector2>,
}

impl Boid {
    pub fn new() -> BoidRef {
        let rand_pos = Vector2::new(
            random_range(0.0..=WIN_WIDTH),
            random_range(0.0..=WIN_HEIGHT),
        );

        let rand_angle = random_range(0.0..=TAU);
        let direction_vec = Vector2::new(rand_angle.cos(), rand_angle.sin());

        let speed = random_range(0.0..=VELOCITY_LIMIT);

        Rc::new(RefCell::new(Boid {
            cur_pos: rand_pos,
            velocity: direction_vec.scale_by(speed),
            ..Default::default()
        }))
    }

    pub fn update(&mut self, flock: &[BoidRef]) {
        self.find_local_flock(flock);
        self.velocity += self.acceleration;
        self.cur_pos += self.velocity;

        self.wrap(10.0);
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle, offset: f32, color: Color) {
        let angle = self.velocity.y.atan2(self.velocity.x) + FRAC_PI_2;
        let half_size = offset / 2.0;

        let mut relative_vertices = [
            Vector2 { x: 0.0, y: -offset },
            Vector2 {
                x: -half_size,
                y: half_size,
            },
            Vector2 {
                x: half_size,
                y: half_size,
            },
        ];

        for v in &mut relative_vertices {
            let rotated_v = Vector2::rotated(v, angle);
            *v = self.cur_pos + rotated_v;
        }

        d.draw_triangle(
            relative_vertices[0],
            relative_vertices[1],
            relative_vertices[2],
            color,
        );
    }

    fn wrap(&mut self, padding: f32) {
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

    fn find_local_flock(&mut self, flock: &[BoidRef]) {
        // remove all references before re-calculating local flock
        self.local_flock.clear();

        for boid in flock {
            if let Ok(k) = boid.try_borrow_mut() {
                // No check need for self-reference as if it's the same boid, we
                // get Err since it has already been borrowed in the main loop
                if self.cur_pos.distance_to(k.cur_pos) > PERCEPTION_RADIUS {
                    continue;
                }

                let angle_to_other_boid = self.velocity.angle_to(k.velocity);
                let is_visible = angle_to_other_boid < BLIND_SPOT.to_radians()
                    || angle_to_other_boid > (360.0 - BLIND_SPOT).to_radians();

                if is_visible {
                    self.local_flock.push(Rc::clone(boid));
                }
            }
        }
    }
}
