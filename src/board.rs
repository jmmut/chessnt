use crate::coord::Coord;
use crate::render::{draw_coord, draw_coord_h, draw_figure};
use crate::theme::Theme;
use macroquad::camera::set_default_camera;
use macroquad::color::DARKGREEN;

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
    size: Coord,
    pieces: Vec<Piece>,
}

impl Board {
    pub fn new(cursor: Coord, size: Coord, pieces: Vec<Piece>) -> Self {
        Self {
            cursor,
            size,
            pieces,
        }
    }
    pub fn new_chess(cursor: Coord, size: Coord) -> Self {
        let pieces = vec![Piece::new(Coord::new_i(0, 0)), Piece::new(size * 0.5)];
        Self {
            cursor,
            size,
            pieces,
        }
    }

    pub fn move_cursor_rel(&mut self, delta: Coord) {
        self.cursor += delta;
    }
    pub fn draw(&self, theme: &mut Theme) {
        for column in 0..self.size.column() {
            for row in 0..self.size.row() {
                let color = if (row + column) % 2 == 0 {
                    theme.palette.white_cells
                } else {
                    theme.palette.black_cells
                };
                draw_coord(Coord::new_i(column, row), color);
            }
        }
        draw_coord_h(self.cursor, DARKGREEN, 0.3);

        for piece in &self.pieces {
            draw_figure(piece)
        }
        set_default_camera();
    }
}
