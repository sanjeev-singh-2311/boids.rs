use std::collections::VecDeque;

use boids::Boid;
use boids::{FLOCK_SIZE, WIN_HEIGHT, WIN_WIDTH};
use raylib::prelude::*;

mod boids;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WIN_WIDTH as i32, WIN_HEIGHT as i32)
        .title("Boids")
        .build();

    let mut flock: [Boid; FLOCK_SIZE] = std::array::from_fn(|_| Boid::new());

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        for boid in &mut flock {
            boid.update();
        }

        d.clear_background(Color::BLACK);

        for boid in &flock {
            boid.draw(&mut d, 5.0, Color::WHITE);
        }
    }
}
