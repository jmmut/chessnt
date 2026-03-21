use crate::core::coord::{Coord, ICoord};
use crate::world::board::{PieceIndex, PieceIndexSmall};
use crate::world::piece::Piece;
use crate::world::team::Team;

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

pub fn possible_moves(size: ICoord, pieces: &Vec<Piece>, piece_index: usize) -> Vec<ICoord> {
    let occupied = &to_occupied_matrix(pieces, size);
    possible_moves_matrix(size, pieces, piece_index, occupied)
}
pub fn possible_moves_matrix(
    size: ICoord,
    pieces: &Vec<Piece>,
    piece_index: usize,
    occupied: &Vec<Vec<Option<Team>>>,
) -> Vec<ICoord> {
    let mut valid_moves = Vec::new();
    possible_moves_matrix_mut(size, pieces, piece_index, occupied, &mut valid_moves);
    valid_moves
}
pub fn possible_moves_matrix_mut(
    size: ICoord,
    pieces: &Vec<Piece>,
    piece_index: usize,
    occupied: &Vec<Vec<Option<Team>>>,
    valid_moves: &mut Vec<ICoord>,
) {
    let piece = &pieces[piece_index];
    for movement in &piece.moveset {
        piece_moves_matrix_mut(movement, pieces, piece_index, occupied, size, valid_moves);
    }
}

fn piece_moves(
    movement: &Move,
    pieces: &Vec<Piece>,
    piece_index: usize,
    board_size: ICoord,
) -> Vec<ICoord> {
    let occupied = &to_occupied_matrix(pieces, board_size);
    piece_moves_matrix(movement, pieces, piece_index, occupied, board_size)
}
fn piece_moves_matrix(
    movement: &Move,
    pieces: &Vec<Piece>,
    piece_index: usize,
    occupied: &Vec<Vec<Option<Team>>>,
    board_size: ICoord,
) -> Vec<ICoord> {
    let mut moves = Vec::new();
    piece_moves_matrix_mut(
        movement,
        pieces,
        piece_index,
        occupied,
        board_size,
        &mut moves,
    );
    moves
}
fn piece_moves_matrix_mut(
    movement: &Move,
    pieces: &Vec<Piece>,
    piece_index: usize,
    occupied: &Vec<Vec<Option<Team>>>,
    board_size: ICoord,
    moves: &mut Vec<ICoord>,
) {
    const KING: &[ICoord] = &[
        ICoord::new_i(-1, 0),
        ICoord::new_i(1, 0),
        ICoord::new_i(0, -1),
        ICoord::new_i(0, 1),
        ICoord::new_i(-1, -1),
        ICoord::new_i(1, -1),
        ICoord::new_i(1, 1),
        ICoord::new_i(-1, 1),
    ];
    const KNIGHT: &[ICoord] = &[
        ICoord::new_i(-2, -1),
        ICoord::new_i(-1, -2),
        ICoord::new_i(2, 1),
        ICoord::new_i(1, 2),
        ICoord::new_i(2, -1),
        ICoord::new_i(1, -2),
        ICoord::new_i(-1, 2),
        ICoord::new_i(-2, 1),
    ];
    if !pieces[piece_index].alive {
        return;
    }
    let piece = &pieces[piece_index];
    match movement {
        Move::Pawn => get_pawn_positions_mut(piece_index, pieces, occupied, board_size, moves),
        Move::Bishop => get_bishop_positions_mut(piece, occupied, board_size, moves),
        Move::Knight => get_positions_mut(piece, KNIGHT, occupied, board_size, moves),
        Move::Rook => get_rook_positions_mut(piece, occupied, board_size, moves),
        Move::King => get_positions_mut(piece, KING, occupied, board_size, moves),
        Move::Queen => {
            get_rook_positions_mut(piece, occupied, board_size, moves);
            get_bishop_positions_mut(piece, occupied, board_size, moves)
        }
    };
}

fn get_positions(
    piece: &Piece,
    possible: &[ICoord],
    occupied: &Vec<Vec<Option<Team>>>,
    board_size: ICoord,
) -> Vec<ICoord> {
    let piece_pos = piece.initial_pos;
    possible
        .iter()
        .map(|p| *p + piece_pos)
        .filter(|coord| inside(*coord, board_size))
        .filter(|coord| {
            if let Some(other_team) = is_occupied(*coord, occupied) {
                piece.team != other_team
            } else {
                true
            }
        })
        .collect()
}

fn get_positions_mut(
    piece: &Piece,
    possible: &[ICoord],
    occupied: &Vec<Vec<Option<Team>>>,
    board_size: ICoord,
    moves: &mut Vec<ICoord>,
) {
    for p in possible {
        let absolute = *p + piece.initial_pos;
        if inside(absolute, board_size) {
            let can_move_or_kill = if let Some(other_team) = is_occupied(absolute, occupied) {
                piece.team != other_team
            } else {
                true
            };
            if can_move_or_kill {
                moves.push(absolute);
            }
        }
    }
}

fn get_pawn_positions(
    piece_index: usize,
    pieces: &Vec<Piece>,
    occupied: &Vec<Vec<Option<Team>>>,
    board_size: ICoord,
) -> Vec<ICoord> {
    let mut moves = vec![];
    get_pawn_positions_mut(piece_index, pieces, occupied, board_size, &mut moves);
    moves
}
fn get_pawn_positions_mut(
    piece_index: usize,
    pieces: &Vec<Piece>,
    occupied: &Vec<Vec<Option<Team>>>,
    board_size: ICoord,
    moves: &mut Vec<ICoord>,
) {
    let piece_pos = pieces[piece_index].initial_pos;
    let team = pieces[piece_index].team;
    let direction = ICoord::new_i(if team.is_white() { -1 } else { 1 }, 0);
    let starting_pawn_column = if team.is_white() {
        board_size.column() - 2
    } else {
        1
    };
    let front = direction + piece_pos;
    if inside(front, board_size) {
        if is_occupied(front, occupied).is_none() {
            moves.push(front);
            let double_start = direction + front;
            if piece_pos.column() == starting_pawn_column
                && is_occupied(double_start, occupied).is_none()
            {
                moves.push(double_start);
            }
        }

        let mut add_if_enemy_is_at = |attack| {
            if inside(attack, board_size) {
                if let Some(other_team) = is_occupied(attack, occupied) {
                    if other_team != pieces[piece_index].team {
                        moves.push(attack);
                    }
                }
            }
        };
        add_if_enemy_is_at(piece_pos + direction + ICoord::new_i(0, 1));
        add_if_enemy_is_at(piece_pos + direction + ICoord::new_i(0, -1));
    }
}

fn get_rook_positions(
    piece: &Piece,
    occupied: &Vec<Vec<Option<Team>>>,
    board_size: ICoord,
) -> Vec<ICoord> {
    let mut positions = Vec::new();
    get_rook_positions_mut(piece, occupied, board_size, &mut positions);
    positions
}
fn get_rook_positions_mut(
    piece: &Piece,
    occupied: &Vec<Vec<Option<Team>>>,
    board_size: ICoord,
    positions: &mut Vec<ICoord>,
) {
    const DIRECTIONS: [ICoord; 4] = [
        ICoord::new_i(-1, 0),
        ICoord::new_i(1, 0),
        ICoord::new_i(0, -1),
        ICoord::new_i(0, 1),
    ];
    for dir in DIRECTIONS {
        add_direction(piece, board_size, &occupied, dir, positions);
    }
}

fn get_bishop_positions(
    piece: &Piece,
    occupied: &Vec<Vec<Option<Team>>>,
    board_size: ICoord,
) -> Vec<ICoord> {
    let mut positions = Vec::new();
    get_bishop_positions_mut(piece, occupied, board_size, &mut positions);
    positions
}
fn get_bishop_positions_mut(
    piece: &Piece,
    occupied: &Vec<Vec<Option<Team>>>,
    board_size: ICoord,
    positions: &mut Vec<ICoord>,
) {
    const DIRECTIONS: [ICoord; 4] = [
        ICoord::new_i(-1, -1),
        ICoord::new_i(-1, 1),
        ICoord::new_i(1, -1),
        ICoord::new_i(1, 1),
    ];
    for dir in DIRECTIONS {
        add_direction(piece, board_size, &occupied, dir, positions);
    }
}

pub fn to_occupied_matrix(pieces: &Vec<Piece>, board_size: ICoord) -> Vec<Vec<Option<Team>>> {
    let mut occupied = vec![vec![None; board_size.column as usize]; board_size.row as usize];
    for i in 0..pieces.len() {
        let piece = &pieces[i];
        let pos = piece.initial_pos;
        if inside(pos, board_size) && pieces[i].alive {
            if occupied[pos.row() as usize][pos.column() as usize].is_some() {
                panic!("unsupported several pieces in the same tile");
            }
            occupied[pos.row() as usize][pos.column() as usize] = Some(piece.team);
        }
    }
    occupied
}
pub fn is_occupied(test: ICoord, occupied: &Vec<Vec<Option<Team>>>) -> Option<Team> {
    occupied[test.row() as usize][test.column() as usize]
}
pub fn set_occupied(test: ICoord, team: Option<Team>, occupied: &mut Vec<Vec<Option<Team>>>) {
    occupied[test.row() as usize][test.column() as usize] = team;
}
pub fn to_piece_index_matrix(
    pieces: &Vec<Piece>,
    board_size: ICoord,
) -> Vec<Vec<Option<PieceIndex>>> {
    let mut occupied = vec![vec![None; board_size.column as usize]; board_size.row as usize];
    for i in 0..pieces.len() {
        let pos = pieces[i].initial_pos;
        if inside(pos, board_size) && pieces[i].alive {
            if occupied[pos.row() as usize][pos.column() as usize].is_some() {
                panic!("unsupported several pieces in the same tile");
            }
            occupied[pos.row() as usize][pos.column() as usize] = Some(i);
        }
    }
    occupied
}
pub fn to_piece_index_matrix_small(
    pieces: &Vec<Piece>,
    board_size: ICoord,
) -> Vec<Vec<Option<PieceIndexSmall>>> {
    let mut occupied = vec![vec![None; board_size.column as usize]; board_size.row as usize];
    for i in 0..pieces.len() {
        let pos = pieces[i].initial_pos;
        if inside(pos, board_size) && pieces[i].alive {
            if occupied[pos.row() as usize][pos.column() as usize].is_some() {
                panic!("unsupported several pieces in the same tile");
            }
            occupied[pos.row() as usize][pos.column() as usize] = Some(i as u8);
        }
    }
    occupied
}

pub fn index_at(
    test: ICoord,
    occupied: &Vec<Vec<Option<PieceIndexSmall>>>,
) -> Option<PieceIndexSmall> {
    occupied[test.row() as usize][test.column() as usize]
}

pub fn set_index_at(
    test: ICoord,
    index: Option<PieceIndexSmall>,
    occupied: &mut Vec<Vec<Option<PieceIndexSmall>>>,
) {
    occupied[test.row() as usize][test.column() as usize] = index;
}

fn add_direction(
    piece: &Piece,
    board_size: ICoord,
    occupied: &Vec<Vec<Option<Team>>>,
    delta: ICoord,
    positions: &mut Vec<ICoord>,
) {
    let mut test = piece.initial_pos;
    loop {
        test += delta;
        if inside(test, board_size) {
            if let Some(other_team) = is_occupied(test, occupied) {
                if piece.team != other_team {
                    positions.push(test);
                }
                break;
            } else {
                positions.push(test);
            }
        } else {
            break;
        }
    }
}

pub fn inside_f(pos: Coord, board_size: ICoord) -> bool {
    pos.column >= 0.0
        && pos.column < board_size.column_f()
        && pos.row >= 0.0
        && pos.row < board_size.row_f()
}
pub fn inside(pos: ICoord, board_size: ICoord) -> bool {
    pos.column >= 0 && pos.column < board_size.column && pos.row >= 0 && pos.row < board_size.row
}

pub fn compute_attackers(
    i: PieceIndex,
    board_size: ICoord,
    pieces: &Vec<Piece>,
) -> Vec<PieceIndex> {
    let target = &pieces[i];
    let target_pos = target.pos_initial_i();
    let mut attackers = Vec::new();
    for (other_i, _other_piece) in pieces.iter().enumerate() {
        if i != other_i && target.team != pieces[other_i].team {
            let moves = possible_moves(board_size, pieces, other_i);
            if moves.contains(&target_pos) {
                attackers.push(other_i)
            };
        }
    }
    attackers
}

const LETTER_KING: u8 = b'k';
const LETTER_QUEEN: u8 = b'q';
const LETTER_BISHOP: u8 = b'b';
const LETTER_KNIGHT: u8 = b'h'; // horse (shrug)
const LETTER_ROOK: u8 = b'r';
const LETTER_PAWN: u8 = b'p';

pub fn is_better<T, U>(best: &Option<T>, new: U, right_better: fn(&T, &U) -> bool) -> bool {
    if let Some(best) = best {
        right_better(best, &new)
    } else {
        true
    }
}
pub fn store_better<T>(max: &mut Option<T>, new: T, right_better: impl Fn(&T, &T) -> bool)
where
    T: PartialOrd<T>,
{
    if let Some(current_max) = max {
        if right_better(current_max, &new) {
            *max = Some(new);
        }
    } else {
        *max = Some(new);
    }
}

pub fn store_max<T>(max: &mut Option<T>, new: T)
where
    T: PartialOrd<T>,
{
    store_better(max, new, |a, b| a < b)
}

pub fn print_board(pieces: &Vec<Piece>) {
    println!("{}", board_to_str(pieces));
}
pub fn board_to_str(pieces: &Vec<Piece>) -> String {
    let mut message = String::new();
    let mut max_row = None;
    let mut max_column = None;
    for piece in pieces {
        store_max(&mut max_row, piece.initial_pos.row);
        store_max(&mut max_column, piece.initial_pos.column);
    }
    if let (Some(row), Some(column)) = (max_row, max_column) {
        let mut matrix = vec![vec![None; column as usize + 1]; row as usize + 1];
        let board_size = ICoord::new_i(column + 1, row + 1);
        for piece in pieces {
            if inside(piece.initial_pos, board_size) {
                matrix[piece.initial_pos.row as usize][piece.initial_pos.column as usize] =
                    Some((piece.team, piece.moveset.first().unwrap()));
            }
        }
        for row in matrix {
            for cell in row {
                if let Some((team, movement)) = cell {
                    message += &format!(
                        "{}{} ",
                        if team.is_white() { 'w' } else { 'b' },
                        match movement {
                            Move::Pawn => LETTER_PAWN,
                            Move::Knight => LETTER_KNIGHT,
                            Move::Bishop => LETTER_BISHOP,
                            Move::Rook => LETTER_ROOK,
                            Move::Queen => LETTER_QUEEN,
                            Move::King => LETTER_KING,
                        } as char
                    )
                } else {
                    message += "-- ";
                }
            }
            message += "\n";
        }
    }
    message
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::core::coord::Coord;
    use crate::world::board::find_first;
    use crate::world::piece::Piece;
    use crate::world::team::Team;
    use Move::*;
    use Team::*;

    pub fn parse_board(text: &str) -> (ICoord, Vec<Piece>) {
        let (size, pieces, _, _) = parse_board_cursor(text);
        (size, pieces)
    }

    pub fn parse_board_cursor(text: &str) -> (ICoord, Vec<Piece>, Coord, Coord) {
        let mut max_columns = None;
        let mut white_cursor = Coord::new_i(0, 0);
        let mut black_cursor = Coord::new_i(0, 0);
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
                        Some(White)
                    } else if color == b'b' {
                        Some(Black)
                    } else {
                        None
                    };
                    if let Some(team) = team {
                        let piece = tile.as_bytes()[1];
                        let coord = Coord::new_i(column as i32, line_count);
                        let movement = if piece == LETTER_KING {
                            Move::King
                        } else if piece == LETTER_QUEEN {
                            Move::Queen
                        } else if piece == LETTER_BISHOP {
                            Move::Bishop
                        } else if piece == LETTER_KNIGHT {
                            Move::Knight
                        } else if piece == LETTER_ROOK {
                            Move::Rook
                        } else if piece == LETTER_PAWN {
                            Move::Pawn
                        } else {
                            panic!(
                                "incorrect board: team without piece type on line (1-based) {}:{}",
                                line_count + 1,
                                text
                            );
                        };
                        pieces.push(Piece::new(coord, team, movement));
                        for cursor in tile.bytes().skip(2) {
                            if cursor == b'O' {
                                white_cursor = coord;
                            } else if cursor == b'X' {
                                black_cursor = coord;
                            } else {
                                // ignore: probably filler chars
                            }
                        }
                    }
                }
                line_count += 1;
            }
        }
        let size = ICoord::new_i(max_columns.unwrap_or(0), line_count);
        (size, pieces, white_cursor, black_cursor)
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
            Piece::new(Coord::new_i(2, 0), Team::White, Move::Bishop),
            Piece::new(Coord::new_i(3, 1), Team::White, Move::Rook),
            Piece::new(Coord::new_i(0, 2), Team::Black, Move::King),
            Piece::new(Coord::new_i(1, 3), Team::White, Move::Pawn),
        ];
        // parsed_pieces.sort();
        // pieces.sort();
        assert_eq!(size, ICoord::new_i(4, 5));
        assert_eq!(parsed_pieces, pieces);
    }
    #[test]
    fn test_parse_board_cursor() {
        #[rustfmt::skip]
        let (size, parsed_pieces) = parse_board("
            --- --- wbX ---
            --- --- --- wr-
            bk- --- --- ---
            --- wpO --- ---
            --- --- --- ---
        ");
        let pieces = vec![
            Piece::new(Coord::new_i(2, 0), White, Bishop),
            Piece::new(Coord::new_i(3, 1), White, Rook),
            Piece::new(Coord::new_i(0, 2), Black, King),
            Piece::new(Coord::new_i(1, 3), White, Pawn),
        ];
        // parsed_pieces.sort();
        // pieces.sort();
        assert_eq!(size, ICoord::new_i(4, 5));
        assert_eq!(parsed_pieces, pieces);
    }
    #[test]
    fn test_check() {
        #[rustfmt::skip]
        let (board_size, pieces) = parse_board("
            br -- wb --
            -- -- -- wr
            bk -- -- --
            -- wp -- --
        ");
        let king_index = find_first(Black, King, &pieces).unwrap();
        let bishop_index = find_first(White, Bishop, &pieces).unwrap();
        let pawn_index = find_first(White, Pawn, &pieces).unwrap();
        let attackers = compute_attackers(king_index, board_size, &pieces);
        assert_eq!(attackers, vec![bishop_index, pawn_index]);
    }
    #[test]
    fn test_jumping_pieces() {
        #[rustfmt::skip]
        let (board_size, pieces) = parse_board("
            bk bp wr wq
            -- bp -- --
            -- wh wb --
        ");
        let king_index = find_first(Black, King, &pieces).unwrap();
        let knight_index = find_first(White, Knight, &pieces).unwrap();
        let attackers = compute_attackers(king_index, board_size, &pieces);
        assert_eq!(attackers, vec![knight_index]);
    }

    #[test]
    fn test_pawn_movement() {
        #[rustfmt::skip]
        let (board_size, pieces) = parse_board("
            -- wh -- --
            -- wr wp --
            -- bp -- --
        ");
        let white_pawn = find_first(White, Pawn, &pieces).unwrap();
        let moves = possible_moves(board_size, &pieces, white_pawn);
        assert_eq!(moves, vec![ICoord::new_i(1, 2)]);
    }
    #[test]
    fn test_pawn_movement_border() {
        #[rustfmt::skip]
        let (board_size, pieces) = parse_board("
            wp --
        ");
        let white_pawn = find_first(White, Pawn, &pieces).unwrap();
        let moves = possible_moves(board_size, &pieces, white_pawn);
        assert_eq!(moves, vec![]);
    }
}
