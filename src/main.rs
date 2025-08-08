use raylib::prelude::*;

const WIDTH: i32 = 1050;
const HEIGHT: i32 = 600;
const FONT_SIZE: i32 = 18;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title("Boids")
        .build();

    let msg: &str = "This should have some boids soon.";

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        let x = (WIDTH - d.measure_text(msg, FONT_SIZE)) / 2;
        let y = HEIGHT / 2;

        d.clear_background(Color::BLACK);
        d.draw_text("This should have some boids soon.", x, y, FONT_SIZE, Color::WHITE);
    }
}
