use rand::random_range;
use raylib::{color::Color, math::Vector2};

pub fn move_vec_towards(vec: Vector2, target: Vector2, step: f32) -> Vector2 {
    let diff = target - vec;
    let dist_sq = diff.length_sqr();

    if dist_sq == 0.0 || step >= 0.0 && diff.length_sqr() <= step * step {
        target
    } else {
        vec + diff * (step / dist_sq.sqrt())
    }
}

pub fn get_random_color() -> Color {
    let hue = random_range(0.0..=360.0);
    let sat = 0.9;
    let val = 0.9;
    Color::color_from_hsv(hue, sat, val)
}
