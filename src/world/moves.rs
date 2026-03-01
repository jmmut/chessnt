use crate::coord::Coord;
use crate::world::board::any_other_piece_at;
use crate::world::piece::Piece;

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

pub fn moves_to_string(moveset: &Moveset) -> String {
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

pub fn possible_moves(size: Coord, pieces: &Vec<Piece>, piece_index: usize) -> Vec<Coord> {
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
    let front = direction + piece_pos;
    if any_other_piece_at(front, piece_index, pieces).is_none() {
        moves.push(front);
    }

    let mut add_if_enemy_is_at = |attack| {
        if let Some(other) = any_other_piece_at(attack, piece_index, pieces) {
            if inside(attack, board_size) && pieces[other].team != pieces[piece_index].team {
                moves.push(attack);
            }
        }
    };
    add_if_enemy_is_at(piece_pos + direction + Coord::new_i(0, 1));
    add_if_enemy_is_at(piece_pos + direction + Coord::new_i(0, -1));
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