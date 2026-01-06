use crate::coord::Coord;
use macroquad::color::Color;
use macroquad::math::{vec2, Vec3};
use macroquad::models::{draw_mesh, Mesh, Vertex};

pub fn draw_coord(coord: Coord, color: Color) {
    draw_coord_h(coord, color, 0.0)
}
pub fn draw_coord_h(coord: Coord, color: Color, height: f32) {
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
