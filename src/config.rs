use raylib::color::Color;

// TODO: Read values from a config file
pub const WIN_WIDTH: f32 = 1050.0;
pub const WIN_HEIGHT: f32 = 600.0;
pub const FLOCK_SIZE: usize = 100;

pub const BOID_SIZE: f32 = 12.0;
pub const PERCEPTION_RADIUS: f32 = 50.0;
pub const BLIND_SPOT: f32 = 60.0;
pub const VELOCITY_LIMIT: f32 = 3.0;
pub const DAMPING_FACTOR: f32 = 1.0;

pub const BACKGROUND_COLOR: Color = Color {
    r: 99,
    g: 143,
    b: 186,
    a: 50,
};
