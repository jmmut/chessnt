use crate::coord::Coord;
use crate::render::{mesh_coord, mesh_cursor, mesh_figure_texture};
use crate::theme::{color_average, Theme};
use crate::ui::render_text_font;
use crate::{set_3d_camera, TRANSPARENT};
use juquad::widgets::anchor::Anchor;
use macroquad::camera::set_default_camera;
use macroquad::color::{Color, DARKBLUE, DARKGREEN, WHITE};
use macroquad::math::{vec2, Vec2};
use macroquad::models::{draw_mesh, Mesh};

const SELECTION: Color = color_average(DARKBLUE, TRANSPARENT);
const CURSOR: Color = color_average(DARKGREEN, TRANSPARENT);
// const FIGURE: Color = color_average(PINK, TRANSPARENT);
const SELECTION_HEIGHT: f32 = 0.05;
const CURSOR_HEIGHT: f32 = 0.1;

#[derive(Clone)]
pub struct Piece {
    pub pos: Coord,
    pub moveset: Moveset,
    pub white: bool,
}
impl Piece {
    pub fn new(pos: Coord, movement: Move, white: bool) -> Self {
        Self {
            pos,
            moveset: vec![movement],
            white,
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
    selected: Option<(usize, Coord)>,
    size: Coord,
    pieces: Vec<Piece>,
    pub piece_size: Vec2,
}

impl Board {
    pub fn new(cursor: Coord, size: Coord, pieces: Vec<Piece>) -> Self {
        Self {
            cursor,
            selected: None,
            size,
            pieces,
            piece_size: vec2(0.3, 1.0),
        }
    }
    pub fn new_chess(cursor: Coord, size: Coord) -> Self {
        let back_column = vec![
            (0, Move::Rook),
            (1, Move::Knight),
            (2, Move::Bishop),
            (3, Move::Queen),
            (4, Move::King),
            (5, Move::Bishop),
            (6, Move::Knight),
            (7, Move::Rook),
        ];
        let mut pieces = Vec::new();
        for (row, movement) in &back_column {
            pieces.push(Piece::new(Coord::new_i(7, *row), *movement, true));
            pieces.push(Piece::new(Coord::new_i(0, *row), *movement, false));
            pieces.push(Piece::new(Coord::new_i(6, *row), Move::Pawn, true));
            pieces.push(Piece::new(Coord::new_i(1, *row), Move::Pawn, false));
        }

        Self::new(cursor, size, pieces)
    }
    fn get_selected_piece(&self) -> Option<&Piece> {
        if let Some((index, _)) = self.selected {
            self.pieces.get(index)
        } else {
            None
        }
    }
    fn get_selected_piece_and_pos(&self) -> Option<(&Piece, Coord)> {
        if let Some((index, initial_pos)) = self.selected {
            Some((self.pieces.get(index).unwrap(), initial_pos))
        } else {
            None
        }
    }
    fn get_selected_piece_mut(&mut self) -> Option<&mut Piece> {
        if let Some((index, _)) = self.selected {
            self.pieces.get_mut(index)
        } else {
            None
        }
    }
    pub fn move_cursor_rel(&mut self, delta: Coord) {
        if let Some(piece) = self.get_selected_piece_mut() {
            piece.pos += delta;
        }
        self.cursor += delta;
    }
    pub fn select(&mut self) {
        let new_selection = self.cursor.round();
        if let Some((_index, _initial)) = self.selected {
            panic!("can't select if there's something already selected");
            // TODO: swap pieces?
            // let
            //
            // for piece in &mut self.pieces {
            //     if piece.pos == old_selection {
            //         piece.pos = new_selection;
            //         self.selected = None;
            //         return;
            //     }
            // }
            // self.selected = Some(new_selection)
        } else {
            for (i, piece) in self.pieces.iter().enumerate() {
                if piece.pos == new_selection {
                    self.selected = Some((i, piece.pos));
                    return;
                }
            }
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
                    theme.palette.black_cells
                } else {
                    theme.palette.white_cells
                };
                draw_mesh(&mesh_coord(Coord::new_i(column, row), color));
            }
        }
        if let Some(_selected) = self.get_selected_piece() {
            // meshes.extend(mesh_cursor(_selected.pos, SELECTION, SELECTION_HEIGHT));
        } else {
            meshes.extend(mesh_cursor(self.cursor, CURSOR, CURSOR_HEIGHT));
        }
        for piece in &self.pieces {
            meshes.push(mesh_figure_texture(
                piece,
                WHITE,
                theme.textures.placeholder,
                self.piece_size,
            ));
            // meshes.push(render_text_3d(
            //     &moves_to_string(&piece.moveset),
            //     Anchor::bottom_left(piece.pos.column, 2.0),
            //     piece.pos.row,
            //     theme,
            // ));
            if piece.pos.round() == self.cursor.round() {
                set_default_camera();
                render_text_font(
                    &format!(
                        "{} {}",
                        if piece.white { "WHITE" } else { "BLACK" },
                        moves_to_string(&piece.moveset).to_uppercase()
                    ),
                    Anchor::top_left(0.0, 0.0),
                    theme,
                    theme.font_title(),
                );
                set_3d_camera(theme);
            }
        }

        if let Some((piece, initial_pos)) = self.get_selected_piece_and_pos() {
            let mut ghost = piece.clone();
            ghost.pos = initial_pos;
            for movement in possible_moves(self.size, &ghost) {
                meshes.extend(mesh_cursor(movement, SELECTION, SELECTION_HEIGHT))
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
