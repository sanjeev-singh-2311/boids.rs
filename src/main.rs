use raylib::prelude::*;
use boids::Boid;

mod boids;

const WIN_WIDTH: i32 = 1050;
const WIN_HEIGHT: i32 = 600;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WIN_WIDTH, WIN_HEIGHT)
        .title("Boids")
        .build();

    let flock: [Boid; 50] = std::array::from_fn(|_| Boid::new());

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        for boid in &flock {
            boid.draw(&mut d);
        }
    }
}
