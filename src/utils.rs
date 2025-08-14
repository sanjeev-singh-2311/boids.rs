use raylib::math::Vector2;

pub fn move_vec_towards(vec: Vector2, target: Vector2, step: f32) -> Vector2 {
    let diff = target - vec;
    let dist_sq = diff.length_sqr();

    if dist_sq == 0.0 || step >= 0.0 && diff.length_sqr() <= step * step {
        target
    } else {
        vec + diff * (step / dist_sq.sqrt())
    }
}
