use crate::coord::Coord;
use crate::world::board::RADAR;
use crate::world::piece::Piece;
use crate::world::referee::texture_pos_to_v3;
use macroquad::color::Color;
use macroquad::math::{vec2, vec3, Vec2, Vec3};
use macroquad::models::{Mesh, Vertex};
use macroquad::prelude::Texture2D;

pub fn mesh_coord(coord: Coord, color: Color) -> Mesh {
    mesh_coord_h(coord, color, 0.0)
}
pub fn mesh_coord_h(coord: Coord, color: Color, height: f32) -> Mesh {
    let corners = floor_corners(coord, height, 1.0);
    let mesh = mesh_quad(corners, color);
    mesh
}

pub fn floor_corners(coord: Coord, height: f32, tile_size: f32) -> [Vec3; 4] {
    let coord_00: Vec3 = coord.to_vec3(height);
    horizontal_quad(coord_00, tile_size, tile_size)
}

pub fn mesh_progress_bar(texture_pos: Coord, piece_size: Vec2, progress: Option<f64>) -> Vec<Mesh> {
    if let Some(progress) = progress {
        let width = 1.0 - progress as f32;
        let pos = texture_pos_to_v3(texture_pos, width, piece_size.y * 1.2);
        vec![mesh_quad(vertical_quad(pos, width, 0.3), RADAR)]
    } else {
        Vec::new()
    }
}

pub fn mesh_cursor(coord: Coord, color: Color, height: f32) -> Vec<Mesh> {
    let width = 0.1;
    let mut meshes = Vec::new();
    let q = horizontal_quad(coord.to_vec3(height), 1.0, width);
    meshes.push(mesh_quad(q, color));
    let coord_00 = coord.to_vec3(height) + vec3(0.0, 0.0, 1.0 - width);
    let q = horizontal_quad(coord_00, 1.0, width);
    meshes.push(mesh_quad(q, color));

    let q = horizontal_quad(coord.to_vec3(height), width, 1.0);
    meshes.push(mesh_quad(q, color));
    let coord_00: Vec3 = coord.to_vec3(height) + vec3(1.0 - width, 0.0, 0.0);
    let q = horizontal_quad(coord_00, width, 1.0);
    meshes.push(mesh_quad(q, color));

    meshes
}

fn horizontal_quad(coord_00: Vec3, x: f32, z: f32) -> [Vec3; 4] {
    quad(coord_00, vec3(x, 0.0, 0.0), vec3(0.0, 0.0, z))
}
pub fn vertical_quad(coord_00: Vec3, x: f32, y: f32) -> [Vec3; 4] {
    quad(coord_00, vec3(x, 0.0, 0.0), vec3(0.0, y, 0.0))
}
fn quad(coord_00: Vec3, a: Vec3, b: Vec3) -> [Vec3; 4] {
    let coord_10 = coord_00 + a;
    let coord_01 = coord_00 + b;
    let coord_11 = coord_00 + a + b;
    [coord_00, coord_10, coord_01, coord_11]
}

// pub fn mesh_figure(piece: &Piece, color: Color) -> Mesh {
//     let coord_00 = (piece.pos + Coord::new_f(0.0, 0.5)).to_vec3(0.0);
//     mesh_vertical_texture(coord_00, 2.0, color, None)
// }
pub fn mesh_figure_texture(piece: &Piece, color: Color, texture: Texture2D, size: Vec2) -> Mesh {
    let coord_00 = (piece.pos_f() + Coord::new_f(0.5 - size.x * 0.5, 0.5)).to_vec3(0.0);
    mesh_vertical_texture(coord_00, color, Some(texture), piece.team.is_white(), size)
}
pub fn mesh_vertical_texture(
    coord_00: Vec3,
    color: Color,
    texture: Option<Texture2D>,
    flip_horiz: bool,
    size: Vec2,
) -> Mesh {
    let corners = vertical_quad(coord_00, size.x, size.y);
    let mesh = mesh_texture_quad(corners, color, texture, flip_horiz, false);
    mesh
}
pub fn mesh_quad(corners: [Vec3; 4], color: Color) -> Mesh {
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
pub fn mesh_texture_quad(
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

pub fn mesh_triangle(corners: [Vec3; 3], color: Color) -> Mesh {
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
