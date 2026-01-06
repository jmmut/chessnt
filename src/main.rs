mod coord;
mod theme;

use crate::coord::coord_to_pixel;
use crate::theme::Theme;
use coord::Coord;
use juquad::draw::draw_segment;
use juquad::lazy::add_contour;
use macroquad::camera::{set_camera, Camera3D};
use macroquad::color::{Color, DARKGRAY};
use macroquad::input::{is_key_pressed, KeyCode};
use macroquad::math::{vec2, vec3, Rect, Vec2, Vec3};
use macroquad::models::{draw_line_3d, draw_mesh, Mesh, Vertex};
use macroquad::prelude::{clear_background, next_frame, screen_height, screen_width, LIGHTGRAY};

pub const COLUMNS: i32 = 8;
pub const ROWS: i32 = 8;

#[macroquad::main("chessnt")]
async fn main() {
    let theme = Theme::default();
    loop {
        clear_background(LIGHTGRAY);

        set_camera(&Camera3D {
            position: vec3(-7., 7., 0.0),
            up: vec3(0., 1., 0.),
            target: vec3(0., 0., 0.),
            ..Default::default()
        });

        // draw_grid(20, 1., BLACK, GRAY);

        if is_key_pressed(KeyCode::Escape) {
            return;
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

                let coord_00 = Coord::new_i(column + 0, row + 0);
                let coord_10 = Coord::new_i(column + 1, row + 0);
                let coord_01 = Coord::new_i(column + 0, row + 1);
                let coord_11 = Coord::new_i(column + 1, row + 1);
                let mesh = to_mesh([coord_00, coord_10, coord_01, coord_11], color);
                draw_mesh(&mesh);
                // let pixel_00 = coord_to_pixel(coord_00, rect);
                // let pixel_10 = coord_to_pixel(coord_10, rect);
                // let pixel_01 = coord_to_pixel(coord_01, rect);
                // let pixel_11 = coord_to_pixel(coord_11, rect);
                // let color = if (row + column )%2 == 0 { theme.palette.white_cells } else {theme.palette.black_cells};
                // draw_triangle(pixel_00, pixel_10, pixel_01, color);
                // draw_triangle(pixel_01, pixel_10, pixel_11, color);
            }
        }

        next_frame().await
    }
}

pub fn to_mesh(corners: [Coord; 4], color: Color) -> Mesh {
    let coords = corners.to_vec();
    let mut vertices = Vec::new();
    for coord in coords {
        let corner: Vec3 = coord.into();
        let vertex = Vertex {
            position: corner,
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
