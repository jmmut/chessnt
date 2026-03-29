pub mod board_draw;
pub mod board_ui;

use crate::AnyResult;
use crate::core::coord::{Coord, ICoord};
use crate::world::moves::{
    Move, Moveset, board_to_str, compute_attackers, inside_f, pieces_to_str, possible_moves,
};
use crate::world::piece::{Piece, Pieces};
use crate::world::referee::Referee;
use crate::world::team::{OneForEachTeam, Team};
use macroquad::math::{Vec2, vec2};
use std::fmt::{Display, Formatter};

pub type PieceIndex = usize;
pub type PieceIndexSmall = u8;
type IndexesMoves = Vec<i32>;

#[derive(Clone, Copy)]
pub struct RooksKing {
    king_index: PieceIndex,
    rook_1_index: PieceIndex,
    rook_2_index: PieceIndex,
}
pub struct RooksKingBuilder {
    rooks_index: Vec<PieceIndex>,
    king_index: Option<PieceIndex>,
}
impl RooksKingBuilder {
    pub fn new() -> Self {
        Self {
            rooks_index: Vec::new(),
            king_index: None,
        }
    }
    pub fn add(&mut self, piece_index: PieceIndex, move_type: Move) {
        if move_type == Move::Rook {
            self.rooks_index.push(piece_index);
        }
        if move_type == Move::King {
            self.king_index = Some(piece_index);
        }
    }
    pub fn build(self) -> Option<RooksKing> {
        if let (Some(king_index), Ok([rook_1_index, rook_2_index])) = (
            self.king_index,
            <[PieceIndex; 2]>::try_from(self.rooks_index),
        ) {
            Some(RooksKing {
                king_index,
                rook_1_index,
                rook_2_index,
            })
        } else {
            None
        }
    }
}
#[derive(Clone)]
pub struct EverMoved {
    indexes_moves: IndexesMoves,
    // castle_allowed: OneForEachTeam<Option<bool>>,
    rooks_king: OneForEachTeam<Option<RooksKing>>,
}
#[derive(PartialEq)]
pub enum AllowedCastle {
    Yes,
    RookMissing,
    RookMoved,
    NotChess,
}
impl EverMoved {
    pub fn new_from(pieces: &Pieces) -> Self {
        let mut indexes_moves = Vec::new();
        indexes_moves.resize(pieces.len(), 0);
        // let castle_allowed = OneForEachTeam::new(Some(true), Some(true));
        let mut rooks_king_opt =
            OneForEachTeam::new(RooksKingBuilder::new(), RooksKingBuilder::new());

        for piece_index in 0..pieces.len() {
            let move_type = pieces[piece_index].moveset.single();
            if [Move::King, Move::Rook].contains(&move_type) {
                let team = pieces[piece_index].team;
                // indexes_moves[piece_index] = 0;
                rooks_king_opt.get_mut(team).add(piece_index, move_type)
            }
        }
        let [white, black] = rooks_king_opt.take();
        let rooks_king = OneForEachTeam::new(white.build(), black.build());
        Self {
            indexes_moves,
            // castle_allowed,
            rooks_king,
        }
    }
    pub const fn new_forbidden() -> Self {
        Self {
            indexes_moves: Vec::new(),
            rooks_king: OneForEachTeam::new(None, None),
        }
    }

    pub fn register_movement(&mut self, piece_index: PieceIndex) {
        let count = &mut self.indexes_moves[piece_index];
        *count += 1;
    }
    pub fn undo_movement(&mut self, piece_index: PieceIndex) {
        let count = &mut self.indexes_moves[piece_index];
        *count -= 1;
    }
    pub fn castle_allowed_rook(&self, team: Team, rook_index: PieceIndex) -> bool {
        if let Some(RooksKing { king_index, .. }) = self.rooks_king.get(team) {
            self.indexes_moves[*king_index] == 0 && self.indexes_moves[rook_index] == 0
        } else {
            false
        }
    }
    pub fn castle_allowed_rook_pos(
        &self,
        team: Team,
        pos: ICoord,
        pieces: &Pieces,
    ) -> AllowedCastle {
        if let Some(RooksKing {
            king_index,
            rook_1_index,
            rook_2_index,
        }) = self.rooks_king.get(team)
        {
            if pieces[*rook_1_index].initial_pos == pos {
                if self.indexes_moves[*king_index] == 0 && self.indexes_moves[*rook_1_index] == 0 {
                    AllowedCastle::Yes
                } else {
                    AllowedCastle::RookMoved
                }
            } else if pieces[*rook_2_index].initial_pos == pos {
                if self.indexes_moves[*king_index] == 0 && self.indexes_moves[*rook_2_index] == 0 {
                    AllowedCastle::Yes
                } else {
                    AllowedCastle::RookMoved
                }
            } else {
                AllowedCastle::RookMissing
            }
        } else {
            AllowedCastle::NotChess
        }
    }
    /// This is not enough to allow castling; castle_allowed_rook also has to return true
    pub fn castle_allowed_king(&self, team: Team) -> bool {
        if let Some(RooksKing {
            rook_1_index,
            rook_2_index,
            king_index,
        }) = self.rooks_king.get(team)
        {
            self.indexes_moves[*king_index] == 0
                && (self.indexes_moves[*rook_1_index] == 0
                    || self.indexes_moves[*rook_2_index] == 0)
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct Board {
    cursor: OneForEachTeam<Coord>,
    selected: OneForEachTeam<Option<PieceIndex>>,
    size: ICoord,
    pieces: Vec<Piece>,
    pub referee: Referee,
    pub piece_size: Vec2,
    pub winning_team: Option<Team>,
    ever_moved: EverMoved,
}

pub const DEFAULT_PIECE_SIZE: Vec2 = vec2(0.3, 1.0);

impl Board {
    pub fn new(
        cursor_white: Coord,
        cursor_black: Coord,
        size: ICoord,
        pieces: Vec<Piece>,
        ever_moved: EverMoved,
    ) -> Self {
        Self {
            cursor: OneForEachTeam::new(cursor_white, cursor_black),
            selected: OneForEachTeam::new(None, None),
            size,
            pieces,
            piece_size: DEFAULT_PIECE_SIZE,
            referee: Referee::new(),
            winning_team: None,
            ever_moved,
        }
    }
    pub fn new_chess(cursor_white: Coord, cursor_black: Coord) -> Self {
        let back_column = [
            Move::Rook,
            Move::Knight,
            Move::Bishop,
            Move::King,
            Move::Queen,
            Move::Bishop,
            Move::Knight,
            Move::Rook,
        ];
        let mut pieces = Vec::new();
        for (row, movement) in back_column.into_iter().enumerate() {
            let row = row as i32;
            pieces.push(Piece::new(Coord::new_i(7, row), Team::White, movement));
            pieces.push(Piece::new(Coord::new_i(0, row), Team::Black, movement));
            pieces.push(Piece::new(Coord::new_i(6, row), Team::White, Move::Pawn));
            pieces.push(Piece::new(Coord::new_i(1, row), Team::Black, Move::Pawn));
        }
        let size = ICoord::new_i(8, 8);
        let ever_moved = EverMoved::new_from(&pieces);
        Self::new(cursor_white, cursor_black, size, pieces, ever_moved)
    }
    pub fn reset(&mut self) {
        *self = Self::new_chess(
            self.cursor.get(Team::White).round(),
            self.cursor.get(Team::Black).round(),
        );
    }
    pub fn set_all_seeing_referee(&mut self, value: bool) {
        self.referee.set_all_seeing(value)
    }
    pub fn tick(&mut self, delta_s: f64) {
        self.referee.tick(delta_s, &self.pieces);
        for piece in &mut self.pieces {
            piece.tick(delta_s);
        }
    }
    pub fn size(&self) -> ICoord {
        self.size
    }
    pub fn get_selected_piece(&self, team: Team) -> Option<&Piece> {
        if let Some(index) = self.selected(team) {
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
    fn get_selected_piece_mut(&mut self, team: Team) -> Option<&mut Piece> {
        if let Some(index) = self.selected(team) {
            self.pieces.get_mut(index)
        } else {
            None
        }
    }
    pub fn move_cursor_rel(&mut self, delta: Coord, team: Team) {
        if let Some(piece) = self.get_selected_piece_mut(team) {
            piece.set_pos(piece.pos_f() + delta);
            piece.moved = true;
        }
        *self.cursor_mut(team) += delta;
    }
    pub fn move_cursor_abs(&mut self, new_pos: Coord, team: Team) {
        if let Some(piece) = self.get_selected_piece_mut(team) {
            piece.set_pos(new_pos);
            piece.moved = true;
        }
        *self.cursor_mut(team) = new_pos;
    }
    pub fn select(&mut self, team: Team) {
        let new_selection = self.cursor(team).round();
        if let Some(_index) = self.selected(team) {
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
                if piece.pos_i() == new_selection && piece.team == team {
                    if piece.cooldown_s.is_none() {
                        *self.selected_mut(team) = Some(i);
                        return;
                    }
                }
            }
        }
    }
    pub fn deselect(&mut self, team: Team) -> AnyResult<()> {
        if let Some(selected_i) = self.selected(team) {
            self.pieces[selected_i].cooldown_s = Some(0.0);
            let initial_pos = self.pieces[selected_i].initial_pos;
            let final_pos = self.pieces[selected_i].pos_ii();
            let overlaps_i = self.overlapping_pieces(selected_i);
            let referee_saw = self.referee.saw_piece(&self.pieces, selected_i);
            assert!(
                overlaps_i.len() <= 1,
                "killing several pieces in the same tile is unsupported"
            );
            let moves = possible_moves(selected_i, &self.pieces, self.size, self.ever_moved());
            if let Some(overlap_i) = overlaps_i.last().cloned() {
                let rounded_overlap_initial = self.pieces[overlap_i].pos_initial_i();
                let overlap_team = self.pieces[overlap_i].team;
                let selected_team = self.pieces[selected_i].team;
                let enemy = overlap_team != selected_team;
                if enemy {
                    if referee_saw {
                        if moves.contains(&rounded_overlap_initial)
                            && self.referee.turn == selected_team
                        {
                            self.kill(overlap_i);
                            self.pieces[selected_i].set_pos_and_initial_i(rounded_overlap_initial);
                            self.referee.turn.toggle();
                        } else {
                            self.kill(selected_i);
                        }
                    } else {
                        self.pieces[selected_i].set_pos_and_initial_i(initial_pos);
                    }
                } else {
                    self.pieces[overlap_i].set_pos_and_initial_i(initial_pos);
                    self.pieces[selected_i].set_pos_and_initial_i(rounded_overlap_initial);
                    if self
                        .referee
                        .saw_any_piece(&self.pieces, vec![selected_i, overlap_i])
                    {
                        // TODO: defer after animation
                        self.pieces[overlap_i].set_pos_and_initial_i(rounded_overlap_initial);
                        self.kill(selected_i);
                    }
                }
            } else {
                if self.pieces[selected_i].moveset.single() == Move::King
                    && (final_pos - initial_pos).length_squared() == 2 * 2
                    && moves.contains(&final_pos)
                {
                    // castling
                    let mut rooks = Vec::new();
                    for i in 0..self.pieces.len() {
                        let piece = &self.pieces[i];
                        if piece.alive && piece.team == team && piece.moveset.single() == Move::Rook
                        {
                            rooks.push(i);
                        }
                    }
                    let to_rook = (final_pos - initial_pos) / 2;
                    let rook_1 = final_pos + to_rook;
                    let rook_2 = final_pos + to_rook * 2;
                    let rook_index = find_at(rook_1, &self.pieces)
                        .or_else(|| find_at(rook_2, &self.pieces))
                        .ok_or_else(|| {
                            format!(
                                "invalid castle, from {:?} to {:?} with board\n{}",
                                initial_pos,
                                final_pos,
                                pieces_to_str(&self.pieces)
                            )
                        })?;
                    if self.pieces[rook_index].moveset != vec![Move::Rook].into() {
                        return Err(format!(
                            "invalid castle, from {:?} to {:?}, expected rook at {:?} or {:?} with board:\n{}",
                            initial_pos, final_pos, rook_1, rook_2, pieces_to_str(&self.pieces)).into()
                        );
                    }

                    self.pieces[rook_index].set_pos_and_initial_i(final_pos - to_rook);
                }
                self.pieces[selected_i].set_pos_and_initial_i(final_pos);
                if referee_saw {
                    if initial_pos == final_pos {
                        // grabbed and dropped in the same place: ok for both teams
                    } else if !self.pieces[selected_i].alive {
                        self.kill(selected_i);
                    } else if !moves.contains(&final_pos) {
                        self.kill(selected_i);
                    } else if self.referee.turn != self.pieces[selected_i].team {
                        self.kill(selected_i);
                    } else {
                        self.referee.turn.toggle();
                    }
                } else {
                    if inside_f(self.pieces[selected_i].pos_i(), self.size) {
                        self.pieces[selected_i].alive = true;
                    }
                }
            }
            if initial_pos != final_pos {
                self.ever_moved.register_movement(selected_i);
            }
            if referee_saw && self.pieces[selected_i].moveset.single() == Move::Pawn {
                if team == Team::White && final_pos.column == 0
                    || team == Team::Black && final_pos.column == self.size.column - 1
                {
                    // TODO: allow user to choose promotion
                    self.pieces[selected_i].moveset = Moveset::new(Move::Queen)
                }
            }
            *self.selected_mut(team) = None;
            *self.cursor_mut(team) = self.cursor(team).round();
        } else {
            return Err("logic error: deselecting but there was no selection".into());
        }
        Ok(())
    }
    pub fn is_selected(&self, team: Team) -> bool {
        self.selected(team).is_some()
    }
    pub fn toggle_select(&mut self, team: Team) -> AnyResult<()> {
        if self.is_selected(team) {
            self.deselect(team)
        } else {
            self.select(team);
            Ok(())
        }
    }
    pub fn selected(&self, team: Team) -> Option<PieceIndex> {
        *self.selected.get(team)
    }
    fn selected_mut(&mut self, team: Team) -> &mut Option<PieceIndex> {
        self.selected.get_mut(team)
    }
    pub fn cursor(&self, team: Team) -> Coord {
        *self.cursor.get(team)
    }
    fn cursor_mut(&mut self, team: Team) -> &mut Coord {
        self.cursor.get_mut(team)
    }

    fn force_deselect(&mut self, selected_i: PieceIndex, team: Team) {
        if let Some(selected) = self.selected(team) {
            if selected == selected_i {
                *self.selected_mut(team) = None;
                *self.cursor_mut(team) = self.cursor(team).round();
            }
        }
    }
    fn kill(&mut self, selected_i: PieceIndex) {
        self.force_deselect(selected_i, Team::Black);
        self.force_deselect(selected_i, Team::White);
        self.pieces[selected_i].alive = false;
        if self.pieces[selected_i].moveset.contains(&Move::King) {
            self.winning_team = Some(self.pieces[selected_i].team.opposite());
        }

        let column = self.pieces[selected_i].pos_i().column();
        self.pieces[selected_i].set_pos_and_initial(Coord::new_i(column, -2));
        while self.overlapping_pieces(selected_i).len() > 0 {
            self.pieces[selected_i].move_rel_and_initial(Coord::new_i(0, -1));
        }
    }

    fn overlapping_pieces(&mut self, selected_i: PieceIndex) -> Vec<PieceIndex> {
        overlapping_piece(selected_i, &self.pieces)
    }
    pub fn pieces(&self) -> &Vec<Piece> {
        &self.pieces
    }
    pub fn pieces_mut(&mut self) -> &mut Vec<Piece> {
        &mut self.pieces
    }
    pub fn ever_moved(&self) -> &EverMoved {
        &self.ever_moved
    }
    pub fn in_check(&self) -> Vec<(Team, PieceIndex)> {
        let mut checks = Vec::new();
        self.add_check(Team::White, &mut checks);
        self.add_check(Team::Black, &mut checks);
        checks
    }

    fn add_check(&self, team: Team, checks: &mut Vec<(Team, PieceIndex)>) {
        if let Some(king) = self.is_in_check(team) {
            checks.push((team, king));
        }
    }
    pub fn is_in_check(&self, team: Team) -> Option<PieceIndex> {
        if let Some(king) = find_first(team, Move::King, self.pieces()) {
            let attacks = compute_attackers(king, &self.pieces, self.size, self.ever_moved());
            if attacks.len() > 0 {
                return Some(king);
            }
        }
        None
    }
}
impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", board_to_str(self.pieces(), self.size()))
    }
}

pub fn find_first(team: Team, move_type: Move, pieces: &Vec<Piece>) -> Option<PieceIndex> {
    let moveset: Moveset = vec![move_type].into();
    for (i, piece) in pieces.iter().enumerate() {
        if piece.team == team && piece.moveset == moveset {
            return Some(i);
        }
    }
    None
}
pub fn find_at(pos: ICoord, pieces: &Vec<Piece>) -> Option<PieceIndex> {
    for (i, piece) in pieces.iter().enumerate() {
        if piece.initial_pos == pos {
            return Some(i);
        }
    }
    None
}

fn overlapping_piece(selected_i: PieceIndex, pieces: &Vec<Piece>) -> Vec<PieceIndex> {
    let selected_rounded = pieces[selected_i].pos_ii();
    other_pieces_at(selected_rounded, selected_i, pieces)
}

pub fn other_pieces_at(pos: ICoord, index: PieceIndex, pieces: &Vec<Piece>) -> Vec<PieceIndex> {
    let mut others = Vec::new();
    for (i, piece) in pieces.iter().enumerate() {
        if i != index && piece.pos_initial_i() == pos {
            others.push(i);
        }
    }
    others
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::world::moves::tests::parse_pieces_cursor;
    use crate::world::piece::PieceMock;

    fn pieces_at(pos: Coord, pieces: &[Piece]) -> Vec<&Piece> {
        let mut found = Vec::new();
        for piece in pieces {
            if piece.pos_i() == pos {
                found.push(piece);
            }
        }
        found
    }

    pub fn parse_board(text: &str) -> Board {
        let (size, pieces, white_cursor, black_cursor, ever_moved) = parse_pieces_cursor(text);
        let mut board = Board::new(white_cursor, black_cursor, size, pieces, ever_moved);
        board.set_all_seeing_referee(true);
        board
    }

    #[test]
    fn test_move_pawn() {
        let mut board = Board::new_chess(Coord::new_i(6, 0), Coord::new_i(1, 0));
        board.select(Team::White);
        board.move_cursor_rel(Coord::new_i(-1, 0), Team::White);
        board.deselect(Team::White).unwrap();
        let expected_pos = Coord::new_i(5, 0);
        let at_5_0 = pieces_at(expected_pos, &board.pieces);
        assert_eq!(
            at_5_0,
            vec![
                &PieceMock::new(expected_pos, vec![Move::Pawn].into(), Team::White)
                    .cooldown(Some(0.0))
                    .moved(true)
                    .into()
            ]
        )
    }
    #[test]
    fn test_deselect_when_killed() {
        let mut board = parse_board("wqO brX");
        let pos_right = board.cursor(Team::Black).into();

        board.select(Team::White);
        board.select(Team::Black);
        board.move_cursor_rel(Coord::new_i(1, 0), Team::White);
        board.move_cursor_rel(Coord::new_f(0.1, 0.0), Team::Black);
        assert_eq!(board.cursor(Team::Black), Coord::new_f(1.1, 0.0));
        let black_piece = board.get_selected_piece(Team::Black);
        let mock = PieceMock::from(&board.pieces[1]).initial_pos(pos_right);
        let mock = mock.into();
        let expected_black = Some(&mock);
        assert_eq!(black_piece, expected_black);
        board.deselect(Team::White).unwrap();
        assert_eq!(board.cursor(Team::Black), Coord::new_f(1.0, 0.0));
        assert_eq!(board.get_selected_piece(Team::Black), None);
    }

    fn team_alive_pos(pieces: &[Piece]) -> Vec<(Team, bool, Coord)> {
        pieces
            .iter()
            .map(|piece| (piece.team, piece.alive, piece.pos_i()))
            .collect()
    }

    fn rook_bishop_3_3() -> (Coord, Coord, Board) {
        #[rustfmt::skip]
        let board = parse_board("
            wrO --- bbX
            --- --- ---
            --- --- ---
        ");
        (board.cursor(Team::White), board.cursor(Team::Black), board)
    }

    #[test]
    fn test_kill_a_moved_piece() {
        let (_, pos_right, mut board) = rook_bishop_3_3();

        board.select(Team::White);
        board.select(Team::Black);

        board.move_cursor_rel(Coord::new_i(2, 0), Team::White);
        board.move_cursor_rel(Coord::new_i(-1, 1), Team::Black);

        board.deselect(Team::White).unwrap();
        assert_eq!(
            team_alive_pos(&board.pieces),
            vec![
                (Team::White, true, pos_right),
                (Team::Black, false, Coord::new_i(1, -2)),
            ]
        );
    }

    #[test]
    fn test_grab_and_drop_in_the_same_place() {
        let (pos_left, pos_right, mut board) = rook_bishop_3_3();
        assert_eq!(board.referee.turn, Team::White);
        board.select(Team::White);
        board.tick(0.1);
        board.deselect(Team::White).unwrap();
        assert_eq!(board.referee.turn, Team::White);
        board.select(Team::Black);
        board.tick(0.1);
        board.deselect(Team::Black).unwrap();
        assert_eq!(board.referee.turn, Team::White);
        assert_eq!(
            team_alive_pos(&board.pieces),
            vec![
                (Team::White, true, pos_left),
                (Team::Black, true, pos_right),
            ]
        );
    }

    #[test]
    fn test_pawn_promotion() {
        #[rustfmt::skip]
        let mut board = parse_board("-- wpO -- -- bpX --");
        board.select(Team::White);
        board.move_cursor_rel(Coord::new_i(-1, 0), Team::White);
        board.deselect(Team::White).unwrap();
        assert_eq!(board.to_string(), "wq -- -- -- bp -- \n");
    }
}
