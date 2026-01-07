use crate::coord::Coord;
use crate::render::{mesh_coord, mesh_coord_h, mesh_cursor, mesh_figure};
use crate::theme::{color_average, Theme};
use crate::TRANSPARENT;
use macroquad::camera::set_default_camera;
use macroquad::color::{DARKBLUE, DARKGREEN, PINK};
use macroquad::models::{draw_mesh, Mesh};

pub struct Piece {
    pub pos: Coord,
}
impl Piece {
    pub fn new(pos: Coord) -> Self {
        Self { pos }
    }
}

pub struct Board {
    cursor: Coord,
    selected: Option<Coord>,
    size: Coord,
    pieces: Vec<Piece>,
}

impl Board {
    pub fn new(cursor: Coord, size: Coord, pieces: Vec<Piece>) -> Self {
        Self {
            cursor,
            selected: None,
            size,
            pieces,
        }
    }
    pub fn new_chess(cursor: Coord, size: Coord) -> Self {
        let pieces = vec![Piece::new(Coord::new_i(0, 0)), Piece::new(size * 0.5), Piece::new(size * 0.75)];
        Self::new(cursor, size, pieces)
    }

    pub fn move_cursor_rel(&mut self, delta: Coord) {
        self.cursor += delta;
    }
    pub fn select(&mut self) {
        let new_selection =self.cursor;
        if let Some(old_selection) = self.selected {
            for piece in &mut self.pieces {
                if piece.pos == old_selection {
                    piece.pos = new_selection;
                    self.selected = None;
                    return;
                }
            }
            self.selected = Some(self.cursor)
        } else {
            self.selected = Some(self.cursor)
        }
    }
    pub fn deselect(&mut self) {
        self.selected = None
    }
    pub fn draw(&self, theme: &mut Theme) {
        let mut meshes = Vec::new();
        for column in 0..self.size.column() {
            for row in 0..self.size.row() {
                let color = if (row + column) % 2 == 0 {
                    theme.palette.white_cells
                } else {
                    theme.palette.black_cells
                };
                draw_mesh(&mesh_coord(Coord::new_i(column, row), color));
            }
        }
        meshes.extend(mesh_cursor(
            self.cursor,
            color_average(DARKGREEN, TRANSPARENT),
            0.3,
        ));
        if let Some(selected) = self.selected {
            meshes.extend(mesh_cursor(
                selected,
                color_average(DARKBLUE, TRANSPARENT),
                0.2,
            ));
        }

        for piece in &self.pieces {
            meshes.push(mesh_figure(piece, color_average(PINK, TRANSPARENT)));
        }
        meshes.sort_by(|a, b| depth(a).total_cmp(&depth(b)));
        for mesh in meshes {
            draw_mesh(&mesh); // can't render cursor and figures online because of intersecting quads with transparencies
        }
        set_default_camera();
    }
}

/// assumes meshes are just quads, with vertices in zig-zag order. (top left, top right, bottom left, bottom right).
fn depth(mesh: &Mesh) -> f32 {
    (mesh.vertices[0].position.z + mesh.vertices[2].position.z) * 0.5
    // + (mesh.vertices[0].position.y + mesh.vertices[2].position.y) * 0.5
}
