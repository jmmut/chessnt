use crate::board::Piece;
use crate::coord::Coord;
use macroquad::color::Color;
use macroquad::math::{vec2, vec3, Vec2, Vec3};
use macroquad::models::{Mesh, Vertex};
use macroquad::prelude::Texture2D;

pub fn mesh_coord(coord: Coord, color: Color) -> Mesh {
    mesh_coord_h(coord, color, 0.0)
}
pub fn mesh_coord_h(coord: Coord, color: Color, height: f32) -> Mesh {
    let corners = floor_corners(coord, height);
    let mesh = to_mesh(corners, color);
    mesh
}

pub fn floor_corners(coord: Coord, height: f32) -> [Vec3; 4] {
    let coord_00: Vec3 = coord.to_vec3(height);
    let coord_10 = (coord + Coord::new_i(1, 0)).to_vec3(height);
    let coord_01 = (coord + Coord::new_i(0, 1)).to_vec3(height);
    let coord_11 = (coord + Coord::new_i(1, 1)).to_vec3(height);
    let corners = [coord_00, coord_10, coord_01, coord_11];
    corners
}

pub fn mesh_cursor(coord: Coord, color: Color, height: f32) -> Vec<Mesh> {
    let mut meshes = Vec::new();
    let coord_00: Vec3 = coord.to_vec3(height);
    let coord_10 = coord_00 + vec3(1.0, 0.0, 0.0);
    let coord_01 = coord_00 + vec3(0.0, 0.0, 0.5);
    let coord_11 = coord_00 + vec3(1.0, 0.0, 0.5);
    meshes.push(to_mesh([coord_00, coord_10, coord_01, coord_11], color));
    // draw_mesh(&mesh);
    let coord_00: Vec3 = coord_00 + vec3(0.0, 0.0, 0.5);
    let coord_10 = coord_00 + vec3(1.0, 0.0, 0.0);
    let coord_01 = coord_00 + vec3(0.0, 0.0, 0.5);
    let coord_11 = coord_00 + vec3(1.0, 0.0, 0.5);
    meshes.push(to_mesh([coord_00, coord_10, coord_01, coord_11], color));
    meshes
}

// pub fn mesh_figure(piece: &Piece, color: Color) -> Mesh {
//     let coord_00 = (piece.pos + Coord::new_f(0.0, 0.5)).to_vec3(0.0);
//     mesh_vertical_texture(coord_00, 2.0, color, None)
// }
pub fn mesh_figure_texture(piece: &Piece, color: Color, texture: Texture2D, size: Vec2) -> Mesh {
    let coord_00 = (piece.pos + Coord::new_f(0.5 - size.x * 0.5, 0.5)).to_vec3(0.0);
    mesh_vertical_texture(coord_00, color, Some(texture), false, size)
}
pub fn mesh_vertical_texture(
    coord_00: Vec3,
    color: Color,
    texture: Option<Texture2D>,
    flip_horiz: bool,
    size: Vec2,
) -> Mesh {
    let coord_10 = coord_00 + vec3(size.x, 0.0, 0.0);
    let coord_01 = coord_00 + vec3(0.0, size.y, 0.0);
    let coord_11 = coord_00 + vec3(size.x, size.y, 0.0);
    let mesh = to_mesh_texture_quad(
        [coord_00, coord_10, coord_01, coord_11],
        color,
        texture,
        flip_horiz,
        false,
    );
    mesh
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
pub fn to_mesh_texture_quad(
    corners: [Vec3; 4],
    color: Color,
    texture: Option<Texture2D>,
    flip_horiz: bool,
    flip_vert: bool,
) -> Mesh {
    let coords = corners.to_vec();
    let mut vertices = Vec::new();
    let yes_horiz_flip = flip_horiz as i32 as f32;
    let not_horiz_flip = !flip_horiz as i32 as f32;
    let yes_vert_flip = flip_vert as i32 as f32;
    let not_vert_flip = !flip_vert as i32 as f32;
    let uvs = vec![
        vec2(yes_horiz_flip, not_vert_flip),
        vec2(not_horiz_flip, not_vert_flip),
        vec2(yes_horiz_flip, yes_vert_flip),
        vec2(not_horiz_flip, yes_vert_flip),
    ];
    for (position, uv) in coords.into_iter().zip(uvs) {
        let vertex = Vertex {
            position,
            uv,
            color,
        };
        vertices.push(vertex);
    }
    Mesh {
        vertices,
        indices: vec![0, 1, 2, 2, 1, 3],
        texture,
    }
}

pub fn to_mesh_triangle(corners: [Vec3; 3], color: Color) -> Mesh {
    let mut vertices = Vec::new();
    for corner in corners {
        vertices.push(Vertex {
            position: corner,
            uv: Default::default(),
            color,
        });
    }

    Mesh {
        vertices,
        indices: vec![0, 1, 2],
        texture: None,
    }
}
