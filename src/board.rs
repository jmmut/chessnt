use crate::coord::Coord;
use crate::render::{mesh_coord, mesh_coord_h, mesh_cursor, mesh_figure, mesh_figure_texture};
use crate::theme::{color_average, Theme};
use crate::ui::{render_text, render_text_3d};
use crate::TRANSPARENT;
use juquad::widgets::anchor::Anchor;
use macroquad::camera::set_default_camera;
use macroquad::color::{Color, DARKBLUE, DARKGREEN, PINK, WHITE};
use macroquad::models::{draw_mesh, Mesh};

const SELECTION: Color = color_average(DARKBLUE, TRANSPARENT);
const CURSOR: Color = color_average(DARKGREEN, TRANSPARENT);
const FIGURE: Color = color_average(PINK, TRANSPARENT);
const SELECTION_HEIGHT: f32 = 0.2;
const CURSOR_HEIGHT: f32 = 0.3;

pub struct Piece {
    pub pos: Coord,
    pub moveset: Moveset,
    pub white: bool,
}
impl Piece {
    pub fn new(pos: Coord, movement: Move) -> Self {
        Self {
            pos,
            moveset: vec![movement],
            white: true,
        }
    }
}

pub type Moveset = Vec<Move>;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Move {
    Pawn,
    Bishop,
    Knight,
    Rook,
    King,
    Queen,
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
        let pieces = vec![
            Piece::new(Coord::new_i(0, 0), Move::Rook),
            Piece::new(size * 0.5, Move::Knight),
            Piece::new(Coord::new_i(5, 2), Move::Pawn),
            Piece::new(size * 0.75, Move::Queen),
        ];
        Self::new(cursor, size, pieces)
    }

    pub fn move_cursor_rel(&mut self, delta: Coord) {
        if self.selected.is_some() {
            for piece in &mut self.pieces {
                if piece.pos == self.cursor {
                    piece.pos += delta;
                }
            }
        }
        self.cursor += delta;
    }
    pub fn select(&mut self) {
        let rounded_cursor = self.cursor.round();
        let new_selection = rounded_cursor;
        if let Some(old_selection) = self.selected {
            for piece in &mut self.pieces {
                if piece.pos == old_selection {
                    piece.pos = new_selection;
                    self.selected = None;
                    return;
                }
            }
            self.selected = Some(rounded_cursor)
        } else {
            self.selected = Some(rounded_cursor)
        }
    }
    pub fn deselect(&mut self) {
        self.selected = None;
        for piece in &mut self.pieces {
            if piece.pos == self.cursor {
                piece.pos = piece.pos.round();
            }
        }
        self.cursor = self.cursor.round();
    }
    pub fn selected(&self) -> bool {
        self.selected.is_some()
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
        if let Some(selected) = self.selected {
            meshes.extend(mesh_cursor(selected, SELECTION, SELECTION_HEIGHT));
        } else {
            meshes.extend(mesh_cursor(self.cursor, CURSOR, CURSOR_HEIGHT));
        }

        for piece in &self.pieces {
            meshes.push(mesh_figure_texture(piece, WHITE, theme.textures.placeholder));
            // meshes.push(render_text_3d(
            //     &moves_to_string(&piece.moveset),
            //     Anchor::bottom_left(piece.pos.column, 2.0),
            //     piece.pos.row,
            //     theme,
            // ));
            if let Some(selected) = self.selected {
                if selected == piece.pos {
                    for movement in possible_moves(self.size, piece) {
                        meshes.extend(mesh_cursor(movement, SELECTION, SELECTION_HEIGHT))
                    }
                }
            }
        }
        meshes.sort_by(|a, b| depth(a).total_cmp(&depth(b)));
        for mesh in meshes {
            draw_mesh(&mesh); // can't render cursor and figures online because of intersecting quads with transparencies
        }
    }
}

fn moves_to_string(moveset: &Moveset) -> String {
    let moves = moveset.iter().map(|m| move_to_string(*m)).collect();
    join(moves, " + ")
}
fn join(words: Vec<String>, delimiter: &str) -> String {
    let words_len: usize = words.iter().map(|v| v.len()).sum();
    let mut result = String::with_capacity(words_len + delimiter.len() * words.len());
    let mut it = words.iter();
    result += it.next().unwrap();
    for word in it {
        result += delimiter;
        result += word;
    }
    result
}
fn move_to_string(movement: Move) -> String {
    match movement {
        Move::Pawn => "Pawn",
        Move::Bishop => "Bishop",
        Move::Knight => "Knight",
        Move::Rook => "Rook",
        Move::King => "King",
        Move::Queen => "Queen",
    }
    .to_string()
}
fn possible_moves(size: Coord, piece: &Piece) -> Vec<Coord> {
    let mut valid_moves = Vec::new();
    for movement in &piece.moveset {
        valid_moves.extend(piece_moves(movement, piece.pos, piece.white, size));
    }
    valid_moves
        .into_iter()
        .filter(|pos| inside(*pos, size))
        .collect()
}

fn piece_moves(
    movement: &Move,
    piece_pos: Coord,
    piece_is_white: bool,
    board_size: Coord,
) -> Vec<Coord> {
    const PAWN: &[Coord] = &[Coord::new_i(-1, 0)];
    const KING: &[Coord] = &[
        Coord::new_i(-1, 0),
        Coord::new_i(1, 0),
        Coord::new_i(0, -1),
        Coord::new_i(0, 1),
        Coord::new_i(-1, -1),
        Coord::new_i(1, -1),
        Coord::new_i(1, 1),
        Coord::new_i(-1, 1),
    ];
    const KNIGHT: &[Coord] = &[
        Coord::new_i(-2, -1),
        Coord::new_i(-1, -2),
        Coord::new_i(2, 1),
        Coord::new_i(1, 2),
        Coord::new_i(2, -1),
        Coord::new_i(1, -2),
        Coord::new_i(-1, 2),
        Coord::new_i(-2, 1),
    ];
    match movement {
        Move::Pawn => PAWN
            .iter()
            .map(|movement| {
                if piece_is_white {
                    *movement
                } else {
                    *movement * -1.0
                }
            })
            .map(|p| p + piece_pos)
            .collect(),
        Move::Bishop => get_bishop_positions(piece_pos, board_size),
        Move::Knight => KNIGHT.iter().map(|p| *p + piece_pos).collect(),
        Move::Rook => get_rook_positions(piece_pos, board_size),
        Move::King => KING.iter().map(|p| *p + piece_pos).collect(),
        Move::Queen => get_rook_positions(piece_pos, board_size)
            .into_iter()
            .chain(get_bishop_positions(piece_pos, board_size))
            .collect(),
    }
}

fn get_rook_positions(piece_pos: Coord, board_size: Coord) -> Vec<Coord> {
    let mut positions = Vec::new();
    for column in 0..board_size.column() {
        let coord = Coord::new_i(column, piece_pos.row());
        if coord != piece_pos {
            positions.push(coord)
        }
    }
    for row in 0..board_size.row() {
        let coord = Coord::new_i(piece_pos.column(), row);
        if coord != piece_pos {
            positions.push(coord)
        }
    }
    positions
}

fn get_bishop_positions(piece_pos: Coord, board_size: Coord) -> Vec<Coord> {
    let mut positions = Vec::new();
    add_diagonal(piece_pos, board_size, Coord::new_i(-1, -1), &mut positions);
    add_diagonal(piece_pos, board_size, Coord::new_i(-1, 1), &mut positions);
    add_diagonal(piece_pos, board_size, Coord::new_i(1, -1), &mut positions);
    add_diagonal(piece_pos, board_size, Coord::new_i(1, 1), &mut positions);
    positions
}

fn add_diagonal(piece_pos: Coord, board_size: Coord, delta: Coord, positions: &mut Vec<Coord>) {
    let mut diagonal = piece_pos;
    loop {
        diagonal += delta;
        if inside(diagonal, board_size) {
            positions.push(diagonal);
        } else {
            break;
        }
    }
}

fn inside(pos: Coord, board_size: Coord) -> bool {
    pos.column >= 0.0
        && pos.column < board_size.column
        && pos.row >= 0.0
        && pos.row < board_size.row
}

/// assumes meshes are just quads, with vertices in zig-zag order. (top left, top right, bottom left, bottom right).
fn depth(mesh: &Mesh) -> f32 {
    (mesh.vertices[0].position.z + mesh.vertices[2].position.z) * 0.5
        + (mesh.vertices[0].position.y + mesh.vertices[2].position.y) * 0.005
}
