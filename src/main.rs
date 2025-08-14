
use boids::{Boid, BoidRef};
use boids::{FLOCK_SIZE, WIN_HEIGHT, WIN_WIDTH};
use raylib::prelude::*;

mod boids;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WIN_WIDTH as i32, WIN_HEIGHT as i32)
        .title("Boids")
        .build();

    // let mut global_flock: VecDeque<Boid> = (0..FLOCK_SIZE).map(|_| Boid::new()).collect();
    let global_flock: Vec<BoidRef> = (0..FLOCK_SIZE).map(|_| Boid::new()).collect();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);

        for boid in global_flock.iter() {
            let mut curr_boid = boid.borrow_mut();
            (*curr_boid).update(&global_flock);
        }

        for boid in global_flock.iter() {
            let curr_boid = boid.borrow_mut();
            (*curr_boid).draw(&mut d, 5.0, Color::WHITE);
        }
    }
}
