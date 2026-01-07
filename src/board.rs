use chessnt::coord::Coord;
use chessnt::render::{draw_coord, draw_coord_h};
use chessnt::theme::Theme;
use chessnt::{COLUMNS, ROWS};
use macroquad::camera::set_default_camera;
use macroquad::color::DARKGREEN;

pub struct Board {
    cursor: Coord,
    size: Coord,
}

impl Board {
    pub fn new(cursor: Coord, size: Coord) -> Self {
        Self { cursor, size }
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

        set_default_camera();
    }
}
