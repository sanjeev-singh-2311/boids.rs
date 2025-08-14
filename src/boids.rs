use std::f32::consts::{FRAC_PI_2, TAU};

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use rand::random_range;
use raylib::prelude::*;
use raylib::{color::Color, math::Vector2};

use crate::config::{
    BLIND_SPOT, DAMPING_FACTOR, MIN_VELOCITY, PERCEPTION_RADIUS, VELOCITY_LIMIT, WIN_HEIGHT,
    WIN_WIDTH,
};
use crate::utils::{clamp_vector_magnitude, get_random_color, move_vec_towards};

pub type BoidRef = Rc<RefCell<Boid>>;

#[derive(Debug, Default)]
pub struct Boid {
    #[allow(dead_code)]
    id: usize,

    cur_pos: Vector2,
    velocity: Vector2,
    acceleration: Vector2,

    local_flock: Vec<BoidRef>,

    color: Color,
}

thread_local! {
    static NEXT_ID: Cell<usize> = const { Cell::new(0) };
}

impl Boid {
    pub fn new() -> BoidRef {
        let id = NEXT_ID.with(|c| {
            let v = c.get();
            c.set(v + 1);
            v
        });

        let rand_pos = Vector2::new(
            random_range(0.0..=WIN_WIDTH),
            random_range(0.0..=WIN_HEIGHT),
        );

        let rand_angle = random_range(0.0..=TAU);
        let direction_vec = Vector2::new(rand_angle.cos(), rand_angle.sin());

        let speed = random_range(MIN_VELOCITY..=VELOCITY_LIMIT);

        Rc::new(RefCell::new(Boid {
            id,
            cur_pos: rand_pos,
            velocity: direction_vec.scale_by(speed),
            color: get_random_color(),
            ..Default::default()
        }))
    }

    pub fn update(&mut self, flock: &[BoidRef]) {
        self.find_local_flock(flock);
        let mut steering_vectors: Vec<Vector2> = Vec::new();

        if let Some(vec) = self.get_align_vector() {
            steering_vectors.push(vec);
        }

        if let Some(vec) = self.get_cohesion_vector() {
            steering_vectors.push(vec);
        }

        steering_vectors.append(&mut self.get_separation_vectors());

        let mut sum_steering_vec = Vector2::zero();
        if !steering_vectors.is_empty() {
            for &vec in &steering_vectors {
                sum_steering_vec += vec;
            }

            self.acceleration = sum_steering_vec.scale_by(1.0 / steering_vectors.len() as f32);
        }

        self.velocity += self.acceleration;

        let speed = self.velocity.length();
        if speed > VELOCITY_LIMIT {
            let target = self.velocity.normalized().scale_by(VELOCITY_LIMIT);
            let step = (speed - VELOCITY_LIMIT) * DAMPING_FACTOR;
            self.velocity = move_vec_towards(self.velocity, target, step);
        }

        self.cur_pos += self.velocity;
        self.wrap(10.0);

        // Acceleration is only applied for one frame, thus we clear it
        self.acceleration = Vector2::zero();
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle, offset: f32) {
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
            self.color,
        );
    }

    fn get_align_vector(&mut self) -> Option<Vector2> {
        let avg_velocity = self.get_avg_velocity();
        if avg_velocity.length() == 0.0 {
            return None;
        }

        Some(avg_velocity - self.velocity)
    }

    fn get_cohesion_vector(&mut self) -> Option<Vector2> {
        let avg_position = self.get_avg_position();
        if avg_position.length() == 0.0 {
            return None;
        }

        let mut steering_vec = avg_position - self.cur_pos;
        // Prevent cohesion from overwhelming other vectors
        steering_vec = clamp_vector_magnitude(
            steering_vec, MIN_VELOCITY..(VELOCITY_LIMIT / 3.0)
        );

        Some(steering_vec)
    }

    fn get_separation_vectors(&self) -> Vec<Vector2> {
        let mut steering_vecs = Vec::with_capacity(self.local_flock.len());

        for boid_ref in &self.local_flock {
            let boid = boid_ref.borrow();
            let direction_away = self.cur_pos - boid.cur_pos;
            let dist = direction_away.length();

            if dist > 0.0 {
                let scale = 50.0 / dist;
                let vec = direction_away.normalized().scale_by(scale);
                steering_vecs.push(vec);
            }
        }

        steering_vecs
    }

    fn get_avg_position(&self) -> Vector2 {
        let mut avg_postition = Vector2::zero();

        if self.local_flock.is_empty() {
            return avg_postition;
        }

        for boid in &self.local_flock {
            avg_postition += boid.borrow().cur_pos;
        }

        avg_postition.scale_by(1.0 / self.local_flock.len() as f32)
    }

    fn get_avg_velocity(&self) -> Vector2 {
        let mut avg_postition = Vector2::zero();

        if self.local_flock.is_empty() {
            return avg_postition;
        }

        for boid in &self.local_flock {
            avg_postition += boid.borrow().velocity;
        }

        avg_postition.scale_by(1.0 / self.local_flock.len() as f32)
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

    fn is_visible(&self, boid: &Boid) -> bool {
        if self.cur_pos.distance_to(boid.cur_pos) > PERCEPTION_RADIUS {
            return false;
        }

        let angle_to_other_boid = self.velocity.angle_to(boid.cur_pos - self.cur_pos);

        // Check if boid is outside blind spot
        angle_to_other_boid < BLIND_SPOT.to_radians()
            || angle_to_other_boid > (360.0 - BLIND_SPOT).to_radians()
    }

    fn find_local_flock(&mut self, flock: &[BoidRef]) {
        // remove all references before re-calculating local flock
        self.local_flock.clear();

        for boid in flock {
            if let Ok(b) = boid.try_borrow() {
                // No check need for self-reference as if it's the same boid, we
                // get Err since it has already been borrowed in the main loop
                if self.is_visible(&b) {
                    self.local_flock.push(Rc::clone(boid));
                }
            }
        }
    }
}
