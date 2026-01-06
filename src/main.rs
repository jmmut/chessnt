mod coord;
mod theme;

use crate::coord::coord_to_pixel;
use crate::theme::Theme;
use coord::Coord;
use juquad::draw::draw_segment;
use juquad::lazy::add_contour;
use macroquad::camera::{set_camera, Camera3D};
use macroquad::color::{Color, DARKGRAY, DARKGREEN};
use macroquad::input::{is_key_pressed, KeyCode};
use macroquad::math::{vec2, vec3, Rect, Vec2, Vec3};
use macroquad::models::{draw_line_3d, draw_mesh, Mesh, Vertex};
use macroquad::prelude::{
    clear_background, next_frame, screen_height, screen_width, Conf, LIGHTGRAY,
};

pub const COLUMNS: i32 = 8;
pub const ROWS: i32 = 8;

const DEFAULT_ASPECT_RATIO: f32 = 16.0 / 9.0;
pub const DEFAULT_WINDOW_WIDTH: i32 = 992;
pub const DEFAULT_WINDOW_HEIGHT: i32 = width_to_height_default(DEFAULT_WINDOW_WIDTH as f32) as i32;
pub const DEFAULT_WINDOW_TITLE: &str = "Chessn't!";

pub type AnyError = Box<dyn std::error::Error>;

fn window_conf() -> Conf {
    Conf {
        window_title: DEFAULT_WINDOW_TITLE.to_owned(),
        window_width: DEFAULT_WINDOW_WIDTH,
        window_height: DEFAULT_WINDOW_HEIGHT,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let theme = Theme::default();
    let mut cursor = Coord::new_i(4, 4);
    loop {
        clear_background(LIGHTGRAY);

        set_camera(&Camera3D {
            position: vec3(0.0, 7., 7.0),
            up: vec3(0., 1., 0.),
            target: vec3(0., 0., 0.),
            ..Default::default()
        });

        // draw_grid(20, 1., BLACK, GRAY);

        if is_key_pressed(KeyCode::Escape) {
            return;
        }

        if is_key_pressed(KeyCode::Right) {
            cursor += Coord::new_i(1, 0);
        }
        if is_key_pressed(KeyCode::Left) {
            cursor += Coord::new_i(-1, 0);
        }
        if is_key_pressed(KeyCode::Up) {
            cursor += Coord::new_i(0, -1);
        }
        if is_key_pressed(KeyCode::Down) {
            cursor += Coord::new_i(0, 1);
        }

        let screen = Vec2::new(screen_width(), screen_height());
        let rect = add_contour(Rect::new(0.0, 0.0, screen.x, screen.y), -screen * 0.25);
        for column in 0..=COLUMNS {
            let start = Coord::new_i(column, 0);
            let end = Coord::new_i(column, ROWS);
            // draw_line_3d(start.into(), end.into(), DARKGRAY);
        }
        for row in 0..=ROWS {
            let start = Coord::new_i(0, row);
            let end = Coord::new_i(COLUMNS, row);
            // draw_line_3d(start.into(), end.into(), DARKGRAY);
        }
        for column in 0..COLUMNS {
            for row in 0..ROWS {
                let color = if (row + column) % 2 == 0 {
                    theme.palette.white_cells
                } else {
                    theme.palette.black_cells
                };

                draw_coord(Coord::new_i(column, row), color);
                // let pixel_00 = coord_to_pixel(coord_00, rect);
                // let pixel_10 = coord_to_pixel(coord_10, rect);
                // let pixel_01 = coord_to_pixel(coord_01, rect);
                // let pixel_11 = coord_to_pixel(coord_11, rect);
                // let color = if (row + column )%2 == 0 { theme.palette.white_cells } else {theme.palette.black_cells};
                // draw_triangle(pixel_00, pixel_10, pixel_01, color);
                // draw_triangle(pixel_01, pixel_10, pixel_11, color);
            }
        }

        draw_coord_h(cursor, DARKGREEN, 0.3);
        next_frame().await
    }
}

fn draw_coord(coord: Coord, color: Color) {
    draw_coord_h(coord, color, 0.0)
}
fn draw_coord_h(coord: Coord, color: Color, height: f32) {
    let coord_00: Vec3 = coord.to_vec3(height);
    let coord_10 = (coord + Coord::new_i(1, 0)).to_vec3(height);
    let coord_01 = (coord + Coord::new_i(0, 1)).to_vec3(height);
    let coord_11 = (coord + Coord::new_i(1, 1)).to_vec3(height);
    let mesh = to_mesh([coord_00, coord_10, coord_01, coord_11], color);
    draw_mesh(&mesh);
}

pub fn to_mesh(corners: [Vec3; 4], color: Color) -> Mesh {
    let coords = corners.to_vec();
    let mut vertices = Vec::new();
    for position in coords {
        let vertex = Vertex {
            position,
            uv: vec2(0.0, 0.0),
            color,
        };
        vertices.push(vertex);
    }
    Mesh {
        vertices,
        indices: vec![0, 1, 2, 2, 1, 3],
        texture: None,
    }
}
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
