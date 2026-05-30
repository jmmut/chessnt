use macroquad::camera::{Camera3D, set_camera};
use macroquad::color::Color;
use macroquad::math::vec3;
use macroquad::miniquad::date::now;
use macroquad::prelude::RenderTarget;
use screen::camera::CameraPos;
use screen::ui_dev::DevUiMenu;

pub mod core {
    pub mod array_union;
    pub mod clipboard;
    pub mod coord;
    pub mod input;
    pub mod interpolation;
    pub mod time;
}
pub mod screen {
    pub mod anchorer;
    pub mod camera;
    pub mod render;
    pub mod screen;
    pub mod shader;
    pub mod theme;
    pub mod ui;
    pub mod ui_dev;
}
pub mod world {
    pub mod board;
    pub mod bot;
    pub mod bot_chess;
    pub mod moves;
    pub mod piece;
    pub mod referee;
    pub mod team;
}

pub const COLUMNS: i32 = 8;
pub const ROWS: i32 = 8;

pub const INITIAL_DEV_UI: DevUiMenu = DevUiMenu::Hidden;

pub const TRANSPARENT: Color = Color::new(1.0, 1.0, 1.0, 0.0);

pub const FPS_AVERAGE_FRAMES: i32 = 6;
pub const DEFAULT_FONT_SIZE: f32 = 16.0;

pub const DEFAULT_ASPECT_RATIO: f32 = 16.0 / 9.0;
pub const DEFAULT_WINDOW_WIDTH: i32 = 992;
pub const DEFAULT_WINDOW_HEIGHT: i32 = width_to_height_default(DEFAULT_WINDOW_WIDTH as f32) as i32;
pub const DEFAULT_WINDOW_TITLE: &str = "Chessn't!";

pub const PROFILER_ENABLED: bool = false;

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

pub fn set_3d_camera(camera: &CameraPos, texture_target: RenderTarget) {
    let board_displacement = vec3(COLUMNS as f32 * 0.5, 0.0, ROWS as f32 * 0.5);
    set_camera(&Camera3D {
        position: camera.pos() + board_displacement,
        up: camera.up(),
        target: camera.target() + board_displacement,
        fovy: camera.fovy,
        render_target: Some(texture_target),
        ..Default::default()
    });
}

pub struct Profiler {
    start: f64,
    enabled: bool,
}

impl Profiler {
    #[cfg(not(target_arch = "wasm32"))]
    pub fn new(enabled: bool) -> Self {
        if enabled {
            Self {
                start: now(),
                enabled,
            }
        } else {
            Self {
                start: 0.0,
                enabled,
            }
        }
    }
    #[cfg(target_arch = "wasm32")]
    pub fn new(_enabled: bool) -> Self {
        Self {
            start: 0.0,
            enabled: false,
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn end_section(&mut self, section_name: &str) {
        if self.enabled {
            let new_time = now();
            eprintln!(
                "section '{}' took {:.2} ms",
                section_name,
                (new_time - self.start) * 1000.0
            );
            self.start = new_time;
        }
    }
    #[cfg(target_arch = "wasm32")]
    pub fn end_section(&mut self, section_name: &str) {}

    pub fn separator(&self) {
        if self.enabled {
            eprintln!();
        }
    }
}
