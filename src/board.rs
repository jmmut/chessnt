use crate::coord::Coord;
use crate::referee::Referee;
use crate::render::{
    floor_corners, mesh_coord, mesh_cursor, mesh_figure_texture, mesh_vertical_texture,
    to_mesh_texture_quad, to_mesh_triangle,
};
use crate::theme::{color_average, CameraPos, Theme};
use crate::ui::render_text_font;
use crate::{set_3d_camera, TRANSPARENT};
use juquad::widgets::anchor::Anchor;
use macroquad::camera::set_default_camera;
use macroquad::color::{Color, DARKBLUE, DARKGREEN, DARKPURPLE, RED, WHITE};
use macroquad::math::{vec2, vec3, Vec2, Vec3};
use macroquad::models::{draw_mesh, Mesh};

const SELECTION: Color = color_average(DARKBLUE, TRANSPARENT);
// const RADAR: Color = color_average(color_average(RED, LIGHTGRAY), TRANSPARENT);
const RADAR: Color = color_average(RED, TRANSPARENT);
const GHOST: Color = color_average(DARKPURPLE, TRANSPARENT);
const CURSOR: Color = color_average(DARKGREEN, TRANSPARENT);
// const FIGURE: Color = color_average(PINK, TRANSPARENT);
const CURSOR_HEIGHT: f32 = 0.1;
const SELECTION_HEIGHT: f32 = CURSOR_HEIGHT * 0.5;
const RADAR_HEIGHT: f32 = SELECTION_HEIGHT * 0.9;
const FLOOR_PIECE_HEIGHT: f32 = RADAR_HEIGHT * 0.8;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Team {
    White,
    Black,
}
impl Team {
    pub fn is_white(&self) -> bool {
        *self == Team::White
    }
}

#[derive(Clone)]
pub struct Piece {
    pub initial_pos: Coord,
    pub pos: Coord,
    pub moveset: Moveset,
    pub team: Team,
    pub moved: bool,
    pub alive: bool,
}
impl Piece {
    pub fn new(pos: Coord, movement: Move, team: Team) -> Self {
        Self {
            initial_pos: pos,
            pos,
            moveset: vec![movement],
            team,
            moved: false,
            alive: true,
        }
    }
    pub fn set_pos(&mut self, new_pos: Coord) {
        self.pos = new_pos;
        self.initial_pos = new_pos;
    }
}

pub type Moveset = Vec<Move>;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
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
    selected: Option<usize>,
    size: Coord,
    pieces: Vec<Piece>,
    pub referee: Referee,
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
            referee: Referee::new(),
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
            pieces.push(Piece::new(Coord::new_i(7, *row), *movement, Team::White));
            pieces.push(Piece::new(Coord::new_i(0, *row), *movement, Team::Black));
            pieces.push(Piece::new(Coord::new_i(6, *row), Move::Pawn, Team::White));
            pieces.push(Piece::new(Coord::new_i(1, *row), Move::Pawn, Team::Black));
        }

        Self::new(cursor, size, pieces)
    }
    pub fn tick(&mut self, delta_s: f64) {
        self.referee.tick(delta_s, &self.pieces);
        for piece in &mut self.pieces {
            piece.moved = false;
        }
    }
    fn get_selected_piece(&self) -> Option<&Piece> {
        if let Some(index) = self.selected {
            self.pieces.get(index)
        } else {
            None
        }
    }
    // fn get_selected_piece_and_pos(&self) -> Option<(&Piece, Coord)> {
    //     if let Some(index) = self.selected {
    //         Some((self.pieces.get(index).unwrap(), initial_pos))
    //     } else {
    //         None
    //     }
    // }
    fn get_selected_piece_mut(&mut self) -> Option<&mut Piece> {
        if let Some(index) = self.selected {
            self.pieces.get_mut(index)
        } else {
            None
        }
    }
    pub fn move_cursor_rel(&mut self, delta: Coord) {
        if let Some(piece) = self.get_selected_piece_mut() {
            piece.pos += delta;
            piece.moved = true;
        }
        self.cursor += delta;
    }
    pub fn select(&mut self) {
        let new_selection = self.cursor.round();
        if let Some(_index) = self.selected {
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
                    self.selected = Some(i);
                    return;
                }
            }
        }
    }
    pub fn deselect(&mut self) {
        if let Some(selected_i) = self.selected {
            let initial = self.pieces[selected_i].initial_pos;
            let any_overlap_i = self.any_overlapping_piece(selected_i);
            if any_overlap_i.is_some() {
                self.pieces[selected_i].pos = initial;
                // TODO: maybe no need for an different key press for swapping?
            } else {
                let rounded_pos = self.pieces[selected_i].pos.round();
                let mut moves = possible_moves(self.size, &self.pieces, selected_i);
                moves.push(self.pieces[selected_i].initial_pos);
                if !moves.contains(&rounded_pos)
                    && self.referee.saw_any_piece(&self.pieces, vec![selected_i])
                {
                    self.kill(selected_i);
                } else {
                    self.pieces[selected_i].set_pos(rounded_pos);
                }
            }
            self.selected = None;
            self.cursor = self.cursor.round();
        } else {
            panic!("logic error: deselecting but there was no selection");
        }
    }
    pub fn selected(&self) -> bool {
        self.selected.is_some()
    }
    pub fn swap_pieces(&mut self) {
        if let Some(selected_i) = self.selected {
            let any_overlap_i = self.any_overlapping_piece(selected_i);
            if let Some(overlap_i) = any_overlap_i {
                let initial = self.pieces[selected_i].initial_pos;
                self.pieces[overlap_i].set_pos(initial.round());
                let selected_pos_rounded = self.pieces[selected_i].pos.round();
                self.pieces[selected_i].set_pos(selected_pos_rounded);
                if self
                    .referee
                    .saw_any_piece(&self.pieces, vec![selected_i, overlap_i])
                {
                    // TODO: defer after animation
                    self.pieces[overlap_i].set_pos(selected_pos_rounded);
                    self.kill(selected_i);
                }
            }
            self.selected = None;
            self.cursor = self.cursor.round();
        } else {
            panic!("logic error: swapping pieces but there was no selection");
        }
    }

    fn kill(&mut self, selected_i: usize) {
        self.pieces[selected_i].alive = false;
        self.pieces[selected_i].pos.row = -2.0;
        while self.any_overlapping_piece(selected_i).is_some() {
            self.pieces[selected_i].pos.row -= 1.0;
        }
    }

    fn any_overlapping_piece(&mut self, selected_i: usize) -> Option<usize> {
        any_overlapping_piece(selected_i, &self.pieces)
    }
}

fn any_overlapping_piece(selected_i: usize, pieces: &Vec<Piece>) -> Option<usize> {
    let selected_rounded = pieces[selected_i].pos.round();
    for (i, piece) in pieces.iter().enumerate() {
        if i != selected_i && piece.pos.round() == selected_rounded {
            return Some(i);
        }
    }
    None
}
fn any_piece_at(pos: Coord, pieces: &Vec<Piece>) -> Option<usize> {
    for (i, piece) in pieces.iter().enumerate() {
        if piece.pos.round() == pos {
            return Some(i);
        }
    }
    None
}

impl Board {
    pub fn draw(&self, camera: &CameraPos, theme: &mut Theme) {
        let mut meshes = Vec::new();
        self.draw_floor(theme);

        meshes.extend(self.selection_meshes());
        meshes.extend(self.piece_meshes(theme));
        meshes.extend(self.referee_meshes(theme));
        meshes.extend(self.possible_moves_meshes());

        meshes.sort_by(|a, b| depth(a).total_cmp(&depth(b)));
        for mesh in meshes {
            draw_mesh(&mesh); // can't render cursor and figures online because of intersecting quads with transparencies
        }
        self.draw_piece_info(camera, theme);
    }

    fn draw_piece_info(&self, camera: &CameraPos, theme: &mut Theme) {
        for piece in &self.pieces {
            if piece.pos.round() == self.cursor.round() {
                set_default_camera();
                render_text_font(
                    &format!(
                        "{} {}",
                        if piece.team.is_white() {
                            "WHITE"
                        } else {
                            "BLACK"
                        },
                        moves_to_string(&piece.moveset).to_uppercase()
                    ),
                    Anchor::top_left(0.0, 0.0),
                    theme,
                    theme.font_title(),
                );
                set_3d_camera(camera);
            }
        }
    }

    fn selection_meshes(&self) -> Vec<Mesh> {
        if let Some(_selected) = self.get_selected_piece() {
            // meshes.extend(mesh_cursor(_selected.pos, SELECTION, SELECTION_HEIGHT));
            vec![]
        } else {
            mesh_cursor(self.cursor, CURSOR, CURSOR_HEIGHT)
        }
    }

    fn piece_meshes(&self, theme: &mut Theme) -> Vec<Mesh> {
        let mut meshes = Vec::new();
        for piece in &self.pieces {
            meshes.push(mesh_figure_texture(
                piece,
                WHITE,
                theme.textures.placeholder,
                self.piece_size,
            ));
            meshes.push(to_mesh_texture_quad(
                floor_corners(piece.pos.round(), FLOOR_PIECE_HEIGHT),
                WHITE,
                Some(theme.textures.pieces[&(piece.team, piece.moveset[0])]),
                false,
                true,
            ));

            // meshes.push(render_text_3d(
            //     &moves_to_string(&piece.moveset),
            //     Anchor::bottom_left(piece.pos.column, 2.0),
            //     piece.pos.row,
            //     theme,
            // ));
        }
        meshes
    }

    fn referee_meshes(&self, theme: &mut Theme) -> Vec<Mesh> {
        let coord_00 =
            (self.referee.pos_c() + Coord::new_f(0.5 - self.piece_size.x * 0.5, 0.5)).to_vec3(0.0);
        let looking_leftwards = self.referee.looking_leftwards();
        let mesh = mesh_vertical_texture(
            coord_00,
            WHITE,
            Some(theme.textures.placeholder),
            looking_leftwards,
            self.piece_size,
        );
        let [radar_base, radar_right, radar_left] = self.referee.radar();
        let square_offset = vec3(0.5, RADAR_HEIGHT, 0.5);
        let radar_base = radar_base.into::<Vec3>() + square_offset;
        let radar_right = radar_right.into::<Vec3>() + square_offset;
        let radar_left = radar_left.into::<Vec3>() + square_offset;
        let radar = to_mesh_triangle([radar_base, radar_right, radar_left], RADAR);
        vec![mesh, radar]
    }

    fn possible_moves_meshes(&self) -> Vec<Mesh> {
        let mut meshes = Vec::new();
        if let Some(index) = self.selected {
            meshes.extend(mesh_cursor(
                self.pieces[index].initial_pos,
                GHOST,
                SELECTION_HEIGHT,
            ));
            for movement in possible_moves(self.size, &self.pieces, index) {
                meshes.extend(mesh_cursor(movement, SELECTION, SELECTION_HEIGHT))
            }
        }
        meshes
    }

    fn draw_floor(&self, theme: &mut Theme) {
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
fn possible_moves(size: Coord, pieces: &Vec<Piece>, piece_index: usize) -> Vec<Coord> {
    let mut valid_moves = Vec::new();
    let piece = &pieces[piece_index];
    for movement in &piece.moveset {
        valid_moves.extend(piece_moves(movement, pieces, piece_index, size));
    }
    valid_moves
        .into_iter()
        .filter(|pos| inside(*pos, size))
        .collect()
}

fn piece_moves(
    movement: &Move,
    pieces: &Vec<Piece>,
    piece_index: usize,
    board_size: Coord,
) -> Vec<Coord> {
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
    let piece_pos = pieces[piece_index].initial_pos;
    match movement {
        Move::Pawn => get_pawn_positions(piece_index, pieces, board_size),
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

fn get_pawn_positions(piece_index: usize, pieces: &Vec<Piece>, board_size: Coord) -> Vec<Coord> {
    let piece_pos = pieces[piece_index].initial_pos;
    let team = pieces[piece_index].team;
    let mut direction = Coord::new_i(-1, 0);
    let mut moves = vec![];
    if team.is_white() {
        if piece_pos.column == board_size.column - 2.0 {
            moves.push(direction * 2.0 + piece_pos);
        }
    } else {
        direction *= -1.0;
        if piece_pos.column == 1.0 {
            moves.push(direction * 2.0 + piece_pos);
        }
    }
    let attack = piece_pos + direction + Coord::new_i(0, 1);
    if let Some(other) = any_piece_at(attack, pieces) {
        if inside(attack, board_size) && pieces[other].team != pieces[piece_index].team {
            moves.push(attack);
        }
    }
    let attack = piece_pos + direction + Coord::new_i(0, -1);
    if let Some(other) = any_piece_at(attack, pieces) {
        if inside(attack, board_size) && pieces[other].team != pieces[piece_index].team {
            moves.push(attack);
        }
    }
    moves.push(direction + piece_pos);
    moves
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
    (mesh.vertices[0].position.z + mesh.vertices[2].position.z) * 0.5 * 0.001
        + (mesh.vertices[0].position.y + mesh.vertices[2].position.y) * 0.5 * 10.0
}
