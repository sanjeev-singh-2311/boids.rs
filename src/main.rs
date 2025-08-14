use boids::{Boid, BoidRef};
use raylib::prelude::*;

use crate::config::{FLOCK_SIZE, WIN_HEIGHT, WIN_WIDTH};

mod boids;
mod config;
mod utils;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WIN_WIDTH as i32, WIN_HEIGHT as i32)
        .title("Boids")
        .build();

    let global_flock: Vec<BoidRef> = (0..FLOCK_SIZE).map(|_| Boid::new()).collect();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);

        for boid in &global_flock {
            boid.borrow_mut().update(&global_flock);
        }

        for boid in &global_flock {
            boid.borrow_mut().draw(&mut d, 5.0);
        }
    }
}
