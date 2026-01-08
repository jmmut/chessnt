use macroquad::camera::{set_camera, Camera3D};
use macroquad::color::Color;
use macroquad::math::vec3;

pub mod board;
pub mod coord;
pub mod render;
pub mod theme;
pub mod ui;

pub const COLUMNS: i32 = 8;
pub const ROWS: i32 = 8;

pub const TRANSPARENT: Color = Color::new(1.0, 1.0, 1.0, 0.0);

pub const DEFAULT_FONT_SIZE: f32 = 16.0;

pub const DEFAULT_ASPECT_RATIO: f32 = 16.0 / 9.0;
pub const DEFAULT_WINDOW_WIDTH: i32 = 992;
pub const DEFAULT_WINDOW_HEIGHT: i32 = width_to_height_default(DEFAULT_WINDOW_WIDTH as f32) as i32;
pub const DEFAULT_WINDOW_TITLE: &str = "Chessn't!";

pub const FPS_AVERAGE_FRAMES: i32 = 6;

pub type AnyError = Box<dyn std::error::Error>;
pub type AnyResult<T> = Result<T, AnyError>;

pub const fn width_to_height_default(width: f32) -> f32 {
    width_to_height(width, DEFAULT_ASPECT_RATIO)
}
pub const fn width_to_height(width: f32, aspect_ratio: f32) -> f32 {
    width / aspect_ratio
}
pub const fn height_to_width_default(height: f32) -> f32 {
    width_to_height(height, DEFAULT_ASPECT_RATIO)
}
pub const fn height_to_width(height: f32, aspect_ratio: f32) -> f32 {
    height * aspect_ratio
}

pub fn set_3d_camera() {
    set_camera(&Camera3D {
        position: vec3(0.0, 6.0, 8.0),
        up: vec3(0.0, 1.0, 0.0),
        target: vec3(0.0, 0.0, 0.0),
        ..Default::default()
    });
}
