use crate::core::coord::Coord;
use crate::world::board::{other_pieces_at, PieceIndex};
use crate::world::piece::Piece;

pub type Moveset = Vec<Move>;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd)]
pub enum Move {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
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
    if other_pieces_at(front, piece_index, pieces).len() == 0 {
        moves.push(front);
    }

    let mut add_if_enemy_is_at = |attack| {
        let attackable = other_pieces_at(attack, piece_index, pieces);
        assert!(
            attackable.len() <= 1,
            "killing several pieces in the same tile is unsupported"
        );
        if let Some(other) = attackable.last().cloned() {
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

pub fn compute_attackers(i: PieceIndex, board_size: Coord, pieces: &Vec<Piece>) -> Vec<PieceIndex> {
    let target = &pieces[i];
    let target_pos = target.pos_initial_i();
    let mut attackers = Vec::new();
    for (other_i, _other_piece) in pieces.iter().enumerate() {
        if i != other_i {
            let moves = possible_moves(board_size, pieces, other_i);
            if moves.contains(&target_pos) {
                attackers.push(other_i)
            };
        }
    }
    attackers
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::coord::Coord;
    use crate::world::board::find_first;
    use crate::world::piece::Piece;
    use crate::world::team::Team;

    fn parse_board(text: &str) -> (Coord, Vec<Piece>) {
        let mut max_columns = None;
        let mut pieces = Vec::new();
        let mut line_count = 0;
        for line in text.lines() {
            let trimmed = line.trim();
            if trimmed.len() > 0 {
                let tiles = trimmed.split(' ').collect::<Vec<_>>();
                if let Some(max) = max_columns {
                    if tiles.len() as i32 > max {
                        max_columns = Some(tiles.len() as i32);
                    }
                } else {
                    max_columns = Some(tiles.len() as i32);
                }
                for (column, tile) in tiles.iter().enumerate() {
                    let color = tile.as_bytes()[0];
                    let team = if color == b'w' {
                        Some(Team::White)
                    } else if color == b'b' {
                        Some(Team::Black)
                    } else {
                        None
                    };
                    if let Some(team) = team {
                        let piece = tile.as_bytes()[1];
                        let coord = Coord::new_i(column as i32, line_count);
                        let movement = if piece == b'k' {
                            Some(Move::King)
                        } else if piece == b'q' {
                            Some(Move::Queen)
                        } else if piece == b'b' {
                            Some(Move::Bishop)
                        } else if piece == b'h' {
                            Some(Move::Knight)
                        } else if piece == b'r' {
                            Some(Move::Rook)
                        } else if piece == b'p' {
                            Some(Move::Pawn)
                        } else {
                            None
                        };
                        if let Some(movement) = movement {
                            pieces.push(Piece::new(coord, movement, team));
                        }
                    }
                }
                line_count += 1;
            }
        }
        (Coord::new_i(max_columns.unwrap_or(0), line_count), pieces)
    }
    #[test]
    fn test_parse_board() {
        #[rustfmt::skip]
        let (size, parsed_pieces) = parse_board("
            -- -- wb --
            -- -- -- wr
            bk -- -- --
            -- wp -- --
            -- -- -- --
        ");
        let pieces = vec![
            Piece::new(Coord::new_i(2, 0), Move::Bishop, Team::White),
            Piece::new(Coord::new_i(3, 1), Move::Rook, Team::White),
            Piece::new(Coord::new_i(0, 2), Move::King, Team::Black),
            Piece::new(Coord::new_i(1, 3), Move::Pawn, Team::White),
        ];
        // parsed_pieces.sort();
        // pieces.sort();
        assert_eq!(size, Coord::new_i(4, 5));
        assert_eq!(parsed_pieces, pieces);
    }
    #[test]
    fn test_check() {
        #[rustfmt::skip]
        let (board_size, pieces) = parse_board("
            -- -- wb --
            -- -- -- wr
            bk -- -- --
            -- wp -- --
        ");
        let king_index = find_first(Team::Black, Move::King, &pieces).unwrap();
        let bishop_index = find_first(Team::White, Move::Bishop, &pieces).unwrap();
        let pawn_index = find_first(Team::White, Move::Pawn, &pieces).unwrap();
        let attackers = compute_attackers(king_index, board_size, &pieces);
        assert_eq!(attackers, vec![bishop_index, pawn_index]);
    }
}
