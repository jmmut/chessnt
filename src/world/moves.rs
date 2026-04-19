use crate::core::coord::{Coord, ICoord};
use crate::world::board::tracking::{AllowedCastle, EverMoved};
use crate::world::board::{PieceIndex, PieceIndexSmall};
use crate::world::piece::{Piece, Pieces};
use crate::world::team::Team;

pub type Occupied = Matrix<Team>;
pub type PieceIndexes = Matrix<PieceIndexSmall>;

pub struct Matrix<T> {
    inner: Vec<Option<T>>,
    board_size: ICoord,
}

impl<T: Copy + PartialEq> Matrix<T> {
    pub fn new(board_size: ICoord) -> Self {
        Self {
            inner: vec![None; (board_size.row * board_size.column) as usize],
            board_size,
        }
    }
    fn index(&self, pos: ICoord) -> usize {
        (pos.row * self.board_size.column + pos.column) as usize
    }

    pub fn rows(&self) -> i32 {
        self.board_size.row
    }
    pub fn columns(&self) -> i32 {
        self.board_size.column
    }
    pub fn size(&self) -> ICoord {
        self.board_size
    }
    pub fn contains(&self, pos: ICoord) -> bool {
        pos.row() >= 0
            && pos.row() < self.rows()
            && pos.column() >= 0
            && pos.column() < self.columns()
    }
    pub fn get(&self, pos: ICoord, value: T) -> bool {
        self.get_any(pos) == Some(value)
    }
    pub fn get_any(&self, pos: ICoord) -> Option<T> {
        let i = self.index(pos);
        unsafe {*self.inner.get_unchecked(i) }
    }
    pub fn set_any(&mut self, pos: ICoord, value: Option<T>) {
        let i = self.index(pos);
        *unsafe {self.inner.get_unchecked_mut(i) } = value;
    }
}

#[derive(PartialEq, Clone, Debug, PartialOrd)]
pub struct Moveset {
    pub moves: Vec<Move>,
}
impl Moveset {
    pub fn new(movement: Move) -> Self {
        Self {
            moves: vec![movement],
        }
    }
    pub fn single(&self) -> Move {
        *self.moves.first().unwrap()
    }
    pub fn contains(&self, movement: &Move) -> bool {
        self.moves.contains(movement)
    }
    pub fn iter(&self) -> impl Iterator<Item = &Move> {
        self.moves.iter()
    }
}
impl IntoIterator for Moveset {
    type Item = Move;
    type IntoIter = <Vec<Move> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.moves.into_iter()
    }
}
impl From<Vec<Move>> for Moveset {
    fn from(moves: Vec<Move>) -> Self {
        Self { moves }
    }
}

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
    let moves = moveset.moves.iter().map(|m| move_to_string(*m)).collect();
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

pub fn possible_moves(
    piece_index: usize,
    pieces: &Vec<Piece>,
    size: ICoord,
    ever_moved: &EverMoved,
) -> Vec<ICoord> {
    let indexes = &to_piece_index_matrix_small(pieces, size);
    possible_moves_matrix(piece_index, pieces, size, ever_moved, indexes)
}
pub fn possible_moves_matrix(
    piece_index: usize,
    pieces: &Vec<Piece>,
    size: ICoord,
    ever_moved: &EverMoved,
    indexes: &PieceIndexes,
) -> Vec<ICoord> {
    // capacity benchmarks:
    // new(): 8650.928ms
    // 0:  9235.926ms
    // 10: 7625.944ms
    // 17: 7379.700ms
    // 20: 7349.646ms
    // 30: 7405.413ms
    // 40: 7515.943ms
    // 50: 7515.134ms
    let mut valid_moves = Vec::with_capacity(17);
    possible_moves_matrix_mut(
        piece_index,
        pieces,
        size,
        indexes,
        ever_moved,
        &mut valid_moves,
    );
    valid_moves
}
pub fn possible_moves_matrix_mut(
    piece_index: usize,
    pieces: &Vec<Piece>,
    size: ICoord,
    indexes: &PieceIndexes,
    ever_moved: &EverMoved,
    valid_moves: &mut Vec<ICoord>,
) {
    let piece = &pieces[piece_index];
    for movement in piece.moveset.iter() {
        piece_moves_matrix_mut(
            piece_index,
            movement,
            pieces,
            size,
            indexes,
            ever_moved,
            valid_moves,
        );
    }
}

fn piece_moves_matrix_mut(
    piece_index: usize,
    movement: &Move,
    pieces: &Vec<Piece>,
    board_size: ICoord,
    indexes: &PieceIndexes,
    ever_moved: &EverMoved,
    moves: &mut Vec<ICoord>,
) {
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
        Move::Pawn => {
            get_pawn_positions_mut(piece_index, pieces, board_size, indexes, ever_moved, moves)
        }
        Move::Bishop => get_bishop_positions_mut(piece, pieces, indexes, board_size, moves),
        Move::Knight => get_positions_mut(piece, KNIGHT, pieces, indexes, board_size, moves),
        Move::Rook => get_rook_positions_mut(piece, pieces, indexes, board_size, moves),
        Move::King => get_king_positions_mut(piece, pieces, indexes, board_size, ever_moved, moves),
        Move::Queen => {
            get_rook_positions_mut(piece, pieces, indexes, board_size, moves);
            get_bishop_positions_mut(piece, pieces, indexes, board_size, moves)
        }
    };
}

fn get_positions_mut(
    piece: &Piece,
    possible: &[ICoord],
    pieces: &Pieces,
    indexes: &PieceIndexes,
    board_size: ICoord,
    moves: &mut Vec<ICoord>,
) {
    for p in possible {
        let absolute = *p + piece.initial_pos;
        if inside(absolute, board_size) {
            let can_move_or_kill = if let Some(other_index) = index_at(absolute, indexes) {
                piece.team != pieces[other_index as usize].team
            } else {
                true
            };
            if can_move_or_kill {
                moves.push(absolute);
            }
        }
    }
}

fn get_pawn_positions_mut(
    piece_index: usize,
    pieces: &Vec<Piece>,
    board_size: ICoord,
    indexes: &PieceIndexes,
    ever_moved: &EverMoved,
    moves: &mut Vec<ICoord>,
) {
    let piece_pos = pieces[piece_index].initial_pos;
    let team = pieces[piece_index].team;
    let direction = ICoord::new_i(if team.is_white() { -1 } else { 1 }, 0);
    let starting_pawn_column = starting_pawn_column(board_size, team);
    let front = direction + piece_pos;
    if inside(front, board_size) {
        if index_at(front, indexes).is_none() {
            moves.push(front);
            let double_start = direction + front;
            if piece_pos.column() == starting_pawn_column
                && inside(double_start, board_size)
                && index_at(double_start, indexes).is_none()
            {
                moves.push(double_start);
            }
        }

        add_if_enemy_is_at(
            ICoord::new_i(0, 1),
            piece_index,
            pieces,
            board_size,
            indexes,
            ever_moved,
            moves,
            piece_pos,
            direction,
        );
        add_if_enemy_is_at(
            ICoord::new_i(0, -1),
            piece_index,
            pieces,
            board_size,
            indexes,
            ever_moved,
            moves,
            piece_pos,
            direction,
        );
    }
}

fn add_if_enemy_is_at(
    side: ICoord,
    piece_index: usize,
    pieces: &Vec<Piece>,
    board_size: ICoord,
    indexes: &PieceIndexes,
    ever_moved: &EverMoved,
    moves: &mut Vec<ICoord>,
    piece_pos: ICoord,
    direction: ICoord,
) {
    let attack = piece_pos + direction + side;
    let passant = piece_pos + side;
    if inside(attack, board_size) {
        if let Some(other_index) = index_at(attack, indexes)
            && pieces[other_index as usize].team != pieces[piece_index].team
        {
            moves.push(attack);
        } else if let Some(passant_index) = index_at(passant, indexes)
            && pieces[passant_index as usize].team != pieces[piece_index].team
            && ever_moved.en_passantable(passant_index as PieceIndex)
        {
            moves.push(attack);
        }
    }
}

pub fn starting_pawn_column(board_size: ICoord, team: Team) -> i32 {
    if team.is_white() {
        board_size.column() - 2
    } else {
        1
    }
}

fn get_pawn_attacks_mut(
    piece_index: usize,
    pieces: &Vec<Piece>,
    board_size: ICoord,
    indexes: &PieceIndexes,
    ever_moved: &EverMoved,
    moves: &mut Vec<ICoord>,
) {
    if !pieces[piece_index].alive {
        return;
    }
    let piece_pos = pieces[piece_index].initial_pos;
    let team = pieces[piece_index].team;
    let direction = get_direction(team);

    add_if_enemy_is_at(
        ICoord::new_i(0, 1),
        piece_index,
        pieces,
        board_size,
        indexes,
        ever_moved,
        moves,
        piece_pos,
        direction,
    );
    add_if_enemy_is_at(
        ICoord::new_i(0, -1),
        piece_index,
        pieces,
        board_size,
        indexes,
        ever_moved,
        moves,
        piece_pos,
        direction,
    );
}

fn get_direction(team: Team) -> ICoord {
    let direction = get_direction_sideways(team, 0);
    direction
}
fn get_direction_sideways(team: Team, row_diff: i32) -> ICoord {
    let direction = ICoord::new_i(if team.is_white() { -1 } else { 1 }, row_diff);
    direction
}

fn get_rook_positions_mut(
    piece: &Piece,
    pieces: &Pieces,
    indexes: &PieceIndexes,
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
        add_direction(piece, board_size, pieces, indexes, dir, positions);
    }
}

fn get_bishop_positions_mut(
    piece: &Piece,
    pieces: &Pieces,
    indexes: &PieceIndexes,
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
        add_direction(piece, board_size, pieces, indexes, dir, positions);
    }
}
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
fn get_king_positions_mut(
    piece: &Piece,
    pieces: &Pieces,
    indexes: &PieceIndexes,
    board_size: ICoord,
    ever_moved: &EverMoved,
    positions: &mut Vec<ICoord>,
) {
    get_positions_mut(piece, KING, pieces, indexes, board_size, positions);
    if ever_moved.castle_allowed_king(piece.team) {
        let sideways = ICoord::new_i(0, -1); // TODO: fails if board is rotated
        add_castle(
            piece, pieces, indexes, board_size, ever_moved, positions, sideways,
        );
        let sideways = -sideways;
        add_castle(
            piece, pieces, indexes, board_size, ever_moved, positions, sideways,
        );
    }
}

fn add_castle(
    piece: &Piece,
    pieces: &Pieces,
    indexes: &PieceIndexes,
    board_size: ICoord,
    ever_moved: &EverMoved,
    positions: &mut Vec<ICoord>,
    sideways: ICoord,
) {
    const SKIP_CASTLE: EverMoved = EverMoved::new_forbidden();
    let adjacent = piece.initial_pos + sideways;
    let jump = piece.initial_pos + sideways * 2;
    let rook_close = piece.initial_pos + sideways * 3;
    let rook_far = piece.initial_pos + sideways * 4;
    let team = piece.team;
    if inside(adjacent, board_size)
        && inside(jump, board_size)
        && index_at(adjacent, indexes).is_none()
        && index_at(jump, indexes).is_none()
    {
        if inside(rook_close, board_size) {
            let allowed_castle = ever_moved.castle_allowed_rook_pos(team, rook_close, pieces);
            #[rustfmt::skip]
            match allowed_castle {
                AllowedCastle::Yes => {
                    let targets = [piece.initial_pos, adjacent, jump];
                    if !is_any_attacked_2(&targets, team, pieces, board_size, &SKIP_CASTLE, indexes) {
                        positions.push(jump);
                    }
                }
                AllowedCastle::RookMissing => {
                    if inside(rook_far, board_size)
                            && index_at(rook_close, indexes).is_none()
                            && ever_moved.castle_allowed_rook_pos(team, rook_far, pieces) == AllowedCastle::Yes {
                        let targets = [piece.initial_pos, adjacent, jump];
                        if !is_any_attacked_2(&targets, team, pieces, board_size, &SKIP_CASTLE, indexes) {
                            positions.push(jump);
                        }
                    }
                }
                AllowedCastle::RookMoved => {}
                AllowedCastle::NotChess => {}
            };
        }
    }
}

pub fn to_occupied_matrix(pieces: &Vec<Piece>, board_size: ICoord) -> Occupied {
    let mut occupied = Occupied::new(board_size);
    for i in 0..pieces.len() {
        let piece = &pieces[i];
        let pos = piece.initial_pos;
        if inside(pos, board_size) && pieces[i].alive {
            if occupied.get_any(pos).is_some() {
                panic!("unsupported several pieces in the same tile");
            }
            occupied.set_any(pos, Some(piece.team));
        }
    }
    occupied
}

pub fn is_occupied(test: ICoord, occupied: &Occupied) -> Option<Team> {
    occupied.get_any(test)
}
pub fn is_occupied_by(test: ICoord, occupied: &Occupied, team: Team) -> bool {
    occupied.get(test, team)
}
pub fn set_occupied(test: ICoord, team: Option<Team>, occupied: &mut Occupied) {
    occupied.set_any(test, team)
}

// pub fn is_occupied_2(test:ICoord, pieces: &Pieces, indexes: PieceIndexes) -> Option<Team> {
// if let Some(indexes.get_any(pos)
// }

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
pub fn to_piece_index_matrix_small(pieces: &Vec<Piece>, board_size: ICoord) -> PieceIndexes {
    let mut occupied = PieceIndexes::new(board_size);
    for i in 0..pieces.len() {
        let pos = pieces[i].initial_pos;
        if inside(pos, board_size) && pieces[i].alive {
            if occupied.get_any(pos).is_some() {
                panic!("unsupported several pieces in the same tile");
            }
            occupied.set_any(pos, Some(i as PieceIndexSmall));
        }
    }
    occupied
}

pub fn index_at(test: ICoord, occupied: &PieceIndexes) -> Option<PieceIndexSmall> {
    #[cfg(debug_assertions)]
    if !occupied.contains(test) {
        panic!(
            "index_at should receive coords in range: coord={:?}, occupied size=(columns: {}, rows: {})",
            test,
            occupied.columns(),
            occupied.rows(),
        );
    }
    occupied.get_any(test)
}
pub fn checked_index_at(test: ICoord, occupied: &PieceIndexes) -> Option<PieceIndexSmall> {
    if occupied.contains(test) {
        occupied.get_any(test)
    } else {
        None
    }
}

pub fn set_index_at(test: ICoord, index: Option<PieceIndexSmall>, occupied: &mut PieceIndexes) {
    occupied.set_any(test, index)
}

fn add_direction(
    piece: &Piece,
    board_size: ICoord,
    pieces: &Pieces,
    indexes: &PieceIndexes,
    delta: ICoord,
    positions: &mut Vec<ICoord>,
) {
    let mut test = piece.initial_pos;
    for _ in 0..8 {
        test += delta;
        if inside(test, board_size) {
            if let Some(other_index) = index_at(test, indexes) {
                if piece.team == pieces[other_index as usize].team {
                    break;
                } else {
                    positions.push(test);
                    break;
                }
            } else {
                positions.push(test);
            }
        } else {
            break;
        }
    }
}

fn is_reachable_direction(
    target: ICoord,
    piece: &Piece,
    board_size: ICoord,
    indexes: &PieceIndexes,
    delta: ICoord,
) -> bool {
    let mut test = piece.initial_pos;
    for _ in 0..8 {
        test += delta;
        if target == test {
            return true;
        } else {
            if inside(test, board_size) {
                if let Some(_other_index) = index_at(test, indexes) {
                    // if piece.team == pieces[other_index as usize].team {
                    //     // blocked by my team
                    //     return false;
                    // } else {
                    //     // blocked by a different piece of the other team
                    //     return false;
                    // }
                    return false;
                } else {
                    // continue
                }
            } else {
                // reached outside of the board
                return false;
            }
        }
    }
    false
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

#[allow(unused)] // don't know if ths is faster than is_any_attacked(vec![pos], ...)
fn is_attacked(
    pos: ICoord,
    team: Team,
    pieces: &Pieces,
    board_size: ICoord,
    ever_moved: &EverMoved,
    indexes: &PieceIndexes,
) -> bool {
    compute_attackers_matrix(pos, team, pieces, board_size, ever_moved, indexes).len() > 0
}
/// team is the attacked team
pub fn is_any_attacked(
    targets: &[ICoord],
    team: Team,
    pieces: &Vec<Piece>,
    board_size: ICoord,
    ever_moved: &EverMoved,
    indexes: &PieceIndexes,
) -> bool {
    const CHESS_SIZE: ICoord = ICoord::new_i(8, 8);
    static mut ATTACKED_CHESS: Vec<Vec<bool>> = Vec::new();
    static mut MOVES: Vec<ICoord> = Vec::new();

    if board_size == CHESS_SIZE {
        let mut attacked = unsafe { std::mem::take(&mut *(&raw mut ATTACKED_CHESS)) };
        let mut moves = unsafe { std::mem::take(&mut *(&raw mut MOVES)) };
        moves.clear();
        if attacked.len() == 0 {
            attacked = vec![vec![false; CHESS_SIZE.column as usize]; CHESS_SIZE.row as usize];
        }
        for row in attacked.iter_mut() {
            for b in row.iter_mut() {
                *b = false;
            }
        }
        for (other_i, _other_piece) in pieces.iter().enumerate() {
            if team != pieces[other_i].team {
                if pieces[other_i].moveset.single() != Move::Pawn {
                    possible_moves_matrix_mut(
                        other_i, pieces, board_size, indexes, ever_moved, &mut moves,
                    );
                } else {
                    get_pawn_attacks_mut(
                        other_i, pieces, board_size, indexes, ever_moved, &mut moves,
                    );
                };
            }
        }
        for movement in &moves {
            attacked[movement.row as usize][movement.column as usize] = true;
        }
        for target in targets {
            if attacked[target.row as usize][target.column as usize] {
                unsafe {
                    ATTACKED_CHESS = std::mem::take(&mut attacked);
                    MOVES = std::mem::take(&mut moves);
                }
                return true;
            }
        }
        unsafe {
            ATTACKED_CHESS = std::mem::take(&mut attacked);
            MOVES = std::mem::take(&mut moves);
        }
        false
    } else {
        let attacked = compute_attacked_matrix(team, pieces, board_size, ever_moved, indexes);
        for target in targets {
            if attacked[target.row as usize][target.column as usize] {
                return true;
            }
        }
        false
    }
}

/// team is the attacked team
pub fn is_any_attacked_2(
    targets: &[ICoord],
    team: Team,
    pieces: &Vec<Piece>,
    board_size: ICoord,
    ever_moved: &EverMoved,
    indexes: &PieceIndexes,
) -> bool {
    const CHESS_SIZE: ICoord = ICoord::new_i(8, 8);
    if board_size == CHESS_SIZE {
        for target in targets {
            for (other_i, _other_piece) in pieces.iter().enumerate() {
                if team != pieces[other_i].team {
                    if can_attack(*target, other_i, pieces, board_size, indexes) {
                        return true;
                    }
                }
            }
        }
        false
    } else {
        let attacked = compute_attacked_matrix(team, pieces, board_size, ever_moved, indexes);
        for target in targets {
            if attacked[target.row as usize][target.column as usize] {
                return true;
            }
        }
        false
    }
}

/// assumes the attacker is from the opposite team than the piece at target
fn can_attack(
    target: ICoord,
    attacker: usize,
    pieces: &Pieces,
    board_size: ICoord,
    indexes: &PieceIndexes,
) -> bool {
    match pieces[attacker].moveset.single() {
        Move::Pawn => can_pawn_attack(target, attacker, pieces),
        Move::Knight => can_knight_attack(target, attacker, pieces),
        Move::Bishop => can_bishop_attack(target, attacker, pieces, board_size, indexes),
        Move::Rook => can_rook_attack(target, attacker, pieces, board_size, indexes),
        Move::Queen => {
            can_bishop_attack(target, attacker, pieces, board_size, indexes)
                || can_rook_attack(target, attacker, pieces, board_size, indexes)
        }
        Move::King => can_king_attack(target, attacker, pieces),
    }
}

fn can_pawn_attack(target: ICoord, attacker: usize, pieces: &Pieces) -> bool {
    let pos = pieces[attacker].initial_pos;
    let diff = target - pos;
    let up = get_direction_sideways(pieces[attacker].team, -1);
    let down = get_direction_sideways(pieces[attacker].team, 1);
    diff == up || diff == down
}
fn can_knight_attack(target: ICoord, attacker: usize, pieces: &Pieces) -> bool {
    let pos = pieces[attacker].initial_pos;
    let diff = target - pos;
    diff.length_squared() == 5
}
fn can_bishop_attack(
    target: ICoord,
    attacker: usize,
    pieces: &Pieces,
    board_size: ICoord,
    indexes: &PieceIndexes,
) -> bool {
    let pos = pieces[attacker].initial_pos;
    let diff = target - pos;
    if diff.row.abs() == diff.column.abs() {
        let delta = ICoord::new_i(diff.column.signum(), diff.row.signum());
        is_reachable_direction(target, &pieces[attacker], board_size, indexes, delta)
    } else {
        false
    }
}
fn can_rook_attack(
    target: ICoord,
    attacker: usize,
    pieces: &Pieces,
    board_size: ICoord,
    indexes: &PieceIndexes,
) -> bool {
    let pos = pieces[attacker].initial_pos;
    let diff = target - pos;
    if diff.row == 0 || diff.column == 0 {
        let delta = ICoord::new_i(diff.column.signum(), diff.row.signum());
        is_reachable_direction(target, &pieces[attacker], board_size, indexes, delta)
    } else {
        false
    }
}
fn can_king_attack(target: ICoord, attacker: usize, pieces: &Pieces) -> bool {
    let pos = pieces[attacker].initial_pos;
    let diff = target - pos;
    diff.length_squared() <= 2
}

/// team is the attacked team
pub fn compute_attacked_matrix(
    team: Team,
    pieces: &Vec<Piece>,
    board_size: ICoord,
    ever_moved: &EverMoved,
    indexes: &PieceIndexes,
) -> Vec<Vec<bool>> {
    let mut attacked = vec![vec![false; board_size.column as usize]; board_size.row as usize];
    let mut moves = Vec::with_capacity(17);
    for (other_i, _other_piece) in pieces.iter().enumerate() {
        if team != pieces[other_i].team {
            if pieces[other_i].moveset.single() != Move::Pawn {
                possible_moves_matrix_mut(
                    other_i, pieces, board_size, indexes, ever_moved, &mut moves,
                );
            } else {
                get_pawn_attacks_mut(other_i, pieces, board_size, indexes, ever_moved, &mut moves);
            };
        }
    }
    for movement in moves {
        attacked[movement.row as usize][movement.column as usize] = true;
    }
    attacked
}

pub fn compute_attackers(
    i: PieceIndex,
    pieces: &Vec<Piece>,
    board_size: ICoord,
    ever_moved: &EverMoved,
) -> Vec<PieceIndex> {
    let target = &pieces[i];
    let team = target.team;
    let target_pos = target.pos_initial_i();
    compute_attackers_2(target_pos, team, pieces, board_size, ever_moved)
}

fn compute_attackers_2(
    target_pos: ICoord,
    team: Team,
    pieces: &Vec<Piece>,
    board_size: ICoord,
    ever_moved: &EverMoved,
) -> Vec<usize> {
    let indexes = &to_piece_index_matrix_small(pieces, board_size);
    compute_attackers_matrix(target_pos, team, pieces, board_size, ever_moved, indexes)
}

fn compute_attackers_matrix(
    target_pos: ICoord,
    team: Team,
    pieces: &Vec<Piece>,
    board_size: ICoord,
    ever_moved: &EverMoved,
    indexes: &PieceIndexes,
) -> Vec<usize> {
    let mut attackers = Vec::new();
    for (other_i, _other_piece) in pieces.iter().enumerate() {
        if team != pieces[other_i].team {
            let mut moves = Vec::new();
            if pieces[other_i].moveset.single() != Move::Pawn {
                possible_moves_matrix_mut(
                    other_i, pieces, board_size, indexes, ever_moved, &mut moves,
                )
            } else {
                get_pawn_attacks_mut(other_i, pieces, board_size, indexes, ever_moved, &mut moves);
            }
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
pub fn store_better<T: PartialOrd<T>>(
    max: &mut Option<T>,
    new: T,
    right_better: impl Fn(&T, &T) -> bool,
) {
    if let Some(current_max) = max {
        if right_better(current_max, &new) {
            *max = Some(new);
        }
    } else {
        *max = Some(new);
    }
}

pub fn store_max<T: PartialOrd<T>>(max: &mut Option<T>, new: T) {
    store_better(max, new, |a, b| a < b)
}

pub fn print_pieces(pieces: &Vec<Piece>) {
    println!("{}", pieces_to_str(pieces));
}
pub fn pieces_to_str(pieces: &Vec<Piece>) -> String {
    let mut max_row = None;
    let mut max_column = None;
    for piece in pieces {
        store_max(&mut max_row, piece.initial_pos.row);
        store_max(&mut max_column, piece.initial_pos.column);
    }
    if let (Some(row), Some(column)) = (max_row, max_column) {
        let board_size = ICoord::new_i(column + 1, row + 1);
        board_to_str(pieces, board_size)
    } else {
        "".to_string()
    }
}

pub fn board_to_str(pieces: &Vec<Piece>, board_size: ICoord) -> String {
    board_to_str_indent(pieces, board_size, 0)
}

pub fn board_to_str_indent(pieces: &Vec<Piece>, board_size: ICoord, indent: i32) -> String {
    let mut message = String::new();
    let mut matrix = vec![vec![None; board_size.column as usize]; board_size.row as usize];
    for piece in pieces {
        if inside(piece.initial_pos, board_size) {
            matrix[piece.initial_pos.row as usize][piece.initial_pos.column as usize] =
                Some((piece.team, piece.moveset.single()));
        }
    }
    for row in matrix {
        message += &"    ".repeat(indent.clamp(0, 20) as usize);
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
    message
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::core::coord::Coord;
    use crate::world::board::{Board, find_at, find_first};
    use crate::world::piece::Piece;
    use crate::world::referee::{Sight, rotate_90};
    use crate::world::team::Team;
    use Move::*;
    use Team::*;

    pub fn parse_pieces(text: &str) -> (ICoord, Vec<Piece>, EverMoved) {
        let (size, pieces, _, _, ever_moved) = parse_pieces_cursor(text);
        (size, pieces, ever_moved)
    }

    pub fn parse_pieces_cursor(text: &str) -> (ICoord, Vec<Piece>, Coord, Coord, EverMoved) {
        let mut max_columns = None;
        let mut white_cursor = Coord::new_i(0, 0);
        let mut black_cursor = Coord::new_i(0, 0);
        let mut pieces = Vec::new();
        let mut line_count = 0;
        for line in text.lines() {
            let trimmed = line.trim();
            if trimmed.len() > 0 {
                let tiles = trimmed.split_ascii_whitespace().collect::<Vec<_>>();
                store_max(&mut max_columns, tiles.len() as i32);
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
        let ever_moved = EverMoved::new_from(&pieces);
        (size, pieces, white_cursor, black_cursor, ever_moved)
    }
    #[test]
    fn test_parse_board() {
        #[rustfmt::skip]
        let (size, parsed_pieces, _) = parse_pieces("
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
        let (size, parsed_pieces, _) = parse_pieces("
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
        let (board_size, pieces, ever_moved) = parse_pieces("
            br -- wb --
            -- -- -- wr
            bk -- -- --
            -- wp -- --
        ");
        let king_index = find_first(Black, King, &pieces).unwrap();
        let bishop_index = find_first(White, Bishop, &pieces).unwrap();
        let pawn_index = find_first(White, Pawn, &pieces).unwrap();
        let attackers = compute_attackers(king_index, &pieces, board_size, &ever_moved);
        assert_eq!(attackers, vec![bishop_index, pawn_index]);
    }
    #[test]
    fn test_jumping_pieces() {
        #[rustfmt::skip]
        let (board_size, pieces, ever_moved) = parse_pieces("
            bk bp wr wq
            -- bp -- --
            -- wh wb --
        ");
        let king_index = find_first(Black, King, &pieces).unwrap();
        let knight_index = find_first(White, Knight, &pieces).unwrap();
        let attackers = compute_attackers(king_index, &pieces, board_size, &ever_moved);
        assert_eq!(attackers, vec![knight_index]);
    }

    #[test]
    fn test_pawn_movement() {
        #[rustfmt::skip]
        let (board_size, pieces, ever_moved) = parse_pieces("
            -- wh -- --
            -- wr wp --
            -- bp -- --
        ");
        let white_pawn = find_first(White, Pawn, &pieces).unwrap();
        let moves = possible_moves(white_pawn, &pieces, board_size, &ever_moved);
        assert_eq!(moves, vec![ICoord::new_i(1, 2)]);
    }
    #[test]
    fn test_pawn_movement_border() {
        #[rustfmt::skip]
        let (board_size, pieces, ever_moved) = parse_pieces("
            wp --
        ");
        let white_pawn = find_first(White, Pawn, &pieces).unwrap();
        let moves = possible_moves(white_pawn, &pieces, board_size, &ever_moved);
        assert_eq!(moves, vec![]);
    }
    #[test]
    fn test_castle() {
        let mut board = Board::new_chess(Coord::new_i(0, 0), Coord::new_i(0, 0));
        board.referee.set_sight(Sight::Blind);
        let king = find_first(Team::White, Move::King, board.pieces()).unwrap();
        let queen = find_first(Team::White, Move::Queen, board.pieces()).unwrap();
        let king_pos = board.pieces()[king].initial_pos;
        let to_queen = board.pieces()[queen].initial_pos - king_pos;
        let close_rook = find_at(king_pos - to_queen * 3, board.pieces()).unwrap();
        let close_knight = find_at(king_pos - to_queen * 2, board.pieces()).unwrap();
        let close_bishop = find_at(king_pos - to_queen * 1, board.pieces()).unwrap();
        let far_bishop = find_at(king_pos + to_queen * 2, board.pieces()).unwrap();
        let far_knight = find_at(king_pos + to_queen * 3, board.pieces()).unwrap();
        let far_rook = find_at(king_pos + to_queen * 4, board.pieces()).unwrap();
        let to_pawn_candidate: ICoord = rotate_90(to_queen.into()).into();
        let to_pawn = if find_at(king_pos + to_pawn_candidate, board.pieces()).is_some() {
            to_pawn_candidate
        } else if find_at(king_pos - to_pawn_candidate, board.pieces()).is_some() {
            -to_pawn_candidate
        } else {
            panic!("can't find pawn next to king");
        };

        // setup
        for index in [close_knight, far_bishop, far_knight] {
            move_rel(index, to_pawn * 2, &mut board);
        }
        let mut moves = possible_moves(king, board.pieces(), board.size(), board.ever_moved());
        let mut expected = vec![];
        assert_eq_sorted(&mut moves, &mut expected, board.pieces());

        // short castle
        let board_copy = board.clone();
        move_rel(close_bishop, to_pawn * 2, &mut board);
        let mut moves = possible_moves(king, board.pieces(), board.size(), board.ever_moved());
        let mut expected = vec![king_pos - to_queen, king_pos - to_queen * 2];
        assert_eq_sorted(&mut moves, &mut expected, board.pieces());

        move_rel(king, -to_queen * 2, &mut board);
        let expected_tower_pos = king_pos - to_queen;
        assert_eq!(board.pieces()[close_rook].initial_pos, expected_tower_pos);

        // long castle
        board = board_copy.clone();
        move_rel(queen, to_pawn * 2, &mut board);
        let mut moves = possible_moves(king, board.pieces(), board.size(), board.ever_moved());
        let mut expected = vec![king_pos + to_queen, king_pos + to_queen * 2];
        assert_eq_sorted(&mut moves, &mut expected, board.pieces());

        move_rel(king, to_queen * 2, &mut board);
        let expected_tower_pos = king_pos + to_queen;
        assert_eq!(board.pieces()[far_rook].initial_pos, expected_tower_pos);

        // king moved
        board = board_copy.clone();
        move_rel(queen, to_pawn * 2, &mut board);
        move_rel(king, to_queen, &mut board);
        move_rel(king, -to_queen, &mut board);
        let mut moves = possible_moves(king, board.pieces(), board.size(), board.ever_moved());
        let king_pos = board.pieces()[king].initial_pos;
        let mut expected = vec![king_pos + to_queen];
        assert_eq_sorted(&mut moves, &mut expected, board.pieces()); // requires memory of moved pieces 

        // rook moved
        board = board_copy.clone();
        move_rel(queen, to_pawn * 2, &mut board);
        move_rel(far_rook, -to_queen, &mut board);
        move_rel(far_rook, to_queen, &mut board);
        let mut moves = possible_moves(king, board.pieces(), board.size(), board.ever_moved());
        let king_pos = board.pieces()[king].initial_pos;
        let mut expected = vec![king_pos + to_queen];
        assert_eq_sorted(&mut moves, &mut expected, board.pieces()); // requires memory of moved pieces 

        board = board_copy.clone();
        move_rel(queen, to_pawn * 2, &mut board);
        move_rel(far_knight, -to_pawn * 2, &mut board);
        let mut moves = possible_moves(king, board.pieces(), board.size(), board.ever_moved());
        let king_pos = board.pieces()[king].initial_pos;
        let mut expected = vec![king_pos + to_queen];
        assert_eq_sorted(&mut moves, &mut expected, board.pieces()); // requires checking pieces in the path of the rook
    }

    fn move_rel(index: PieceIndex, movement: ICoord, board: &mut Board) {
        board.move_cursor_abs(board.pieces()[index].initial_pos.into(), Team::White);
        board.select(Team::White);
        board.move_cursor_rel(movement.into(), Team::White);
        board.deselect(Team::White).unwrap();
        board.pieces_mut()[index].cooldown_s = None;
    }

    fn assert_eq_sorted(moves: &mut Vec<ICoord>, expected: &mut Vec<ICoord>, pieces: &Vec<Piece>) {
        moves.sort();
        expected.sort();
        assert_eq!(moves, expected, "board:\n{}", pieces_to_str(pieces));
    }

    #[test]
    fn test_castle_forbidden_path_in_check() {
        #[rustfmt::skip]
        let (board_size, pieces, ever_moved) = parse_pieces("
            br bp -- -- -- -- -- wr 
            bh bp bb -- -- -- -- -- 
            bb bp -- -- -- -- -- -- 
            bk bp -- -- -- -- wp wk 
            bq bp -- -- -- -- wp -- 
            bb bp -- -- -- -- -- -- 
            bh bp bb -- -- -- -- -- 
            br bp -- -- -- -- -- wr 
        ");
        let king = find_first(Team::White, Move::King, &pieces).unwrap();
        let king_pos = pieces[king].initial_pos;
        let to_queen = ICoord::new_i(0, 1);
        let to_pawn_diagonal = ICoord::new_i(-1, -1);

        let mut moves = possible_moves(king, &pieces, board_size, &ever_moved);
        let mut expected = vec![
            king_pos + to_queen * 2,
            king_pos + to_queen,
            king_pos - to_queen,
            king_pos + to_pawn_diagonal,
        ];
        assert_eq_sorted(&mut moves, &mut expected, &pieces); // requires checking check in the path 
    }
    #[test]
    fn is_any_attacked() {
        #[rustfmt::skip]
        let (board_size, pieces, ever_moved) = parse_pieces("
            -- wh -- --
            -- wr wp --
            -- bp -- --
        ");
        let indexes = &to_piece_index_matrix_small(&pieces, board_size);
        let attacked =
            compute_attacked_matrix(Team::White, &pieces, board_size, &ever_moved, indexes);
        assert_eq!(
            attacked,
            [
                [false, false, false, false],
                [false, false, true, false],
                [false, false, false, false]
            ]
        );
    }
    #[test]
    fn test_is_attacked() {
        #[rustfmt::skip]
        let (board_size, pieces, ever_moved) = parse_pieces("
            -- wh -- --
            -- wr wp --
            -- bp -- --
        ");
        let indexes = &to_piece_index_matrix_small(&pieces, board_size);
        let mut attacked = vec![vec![false; board_size.column as usize]; board_size.row as usize];
        for (i_row, row) in attacked.iter_mut().enumerate() {
            for (i_column, b) in row.iter_mut().enumerate() {
                *b = is_attacked(
                    ICoord::new_i(i_column as i32, i_row as i32),
                    Team::White,
                    &pieces,
                    board_size,
                    &ever_moved,
                    indexes,
                );
            }
        }
        assert_eq!(
            attacked,
            [
                [false, false, false, false],
                [false, false, true, false],
                [false, false, false, false]
            ]
        );
    }
    // #[test]
    // fn test_en_passant() {
    //
    //     #[rustfmt::skip]
    //     let (board_size, pieces, ever_moved) = parse_board_cursor("
    //         -- -- -- wp -- -- -- --
    //         -- bp -- -- -- -- -- --
    //     ");
    //
    // }
}
