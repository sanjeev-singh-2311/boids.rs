use std::ops::Range;

use rand::random_range;
use raylib::{color::Color, math::Vector2};

use crate::config::BACKGROUND_COLOR as BG;

pub fn move_vec_towards(vec: Vector2, target: Vector2, step: f32) -> Vector2 {
    let diff = target - vec;
    let dist_sq = diff.length_sqr();

    if dist_sq == 0.0 || step >= 0.0 && diff.length_sqr() <= step * step {
        target
    } else {
        vec + diff * (step / dist_sq.sqrt())
    }
}

pub fn clamp_vector_magnitude(v: Vector2, range: Range<f32>) -> Vector2 {
    let length_sq = v.x * v.x + v.y * v.y;
    if length_sq == 0.0 {
        return v;
    }

    let length = length_sq.sqrt();

    if length < range.start {
        let scale = range.start / length;
        Vector2 {
            x: v.x * scale,
            y: v.y * scale,
        }
    } else if length > range.end {
        let scale = range.end / length;
        Vector2 {
            x: v.x * scale,
            y: v.y * scale,
        }
    } else {
        v
    }
}

pub fn get_random_color() -> Color {
    let sat = 0.9;
    let val = 0.9;

    loop {
        let hue = random_range(0.0..=360.0);
        let color = Color::color_from_hsv(hue, sat, val);

        let dist = ((color.r as f32 - BG.r as f32).powi(2)
            + (color.g as f32 - BG.g as f32).powi(2)
            + (color.b as f32 - BG.b as f32).powi(2))
        .sqrt();

        if dist >= 100.0 {
            return color;
        }
    }
}
