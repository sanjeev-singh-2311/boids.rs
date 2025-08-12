use raylib::prelude::*;
use boids::Boid;

mod boids;

const WIN_WIDTH: f32 = 1050.0;
const WIN_HEIGHT: f32 = 600.0;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WIN_WIDTH as i32, WIN_HEIGHT as i32)
        .title("Boids")
        .build();

    let mut flock: [Boid; 50] = std::array::from_fn(|_| Boid::new());

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        for boid in &mut flock {
            boid.update();
        }

        d.clear_background(Color::BLACK);

        for boid in &flock {
            boid.draw(&mut d);
        }
    }
}
