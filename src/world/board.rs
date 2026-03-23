pub mod board_draw;
pub mod board_ui;

use crate::core::coord::{Coord, ICoord};
use crate::world::moves::{Move, compute_attackers, inside_f, possible_moves};
use crate::world::piece::Piece;
use crate::world::referee::Referee;
use crate::world::team::{OneForEachTeam, Team};
use macroquad::math::{Vec2, vec2};

pub type PieceIndex = usize;
pub type PieceIndexSmall = u8;

pub struct Board {
    cursor: OneForEachTeam<Coord>,
    selected: OneForEachTeam<Option<PieceIndex>>,
    size: ICoord,
    pieces: Vec<Piece>,
    pub referee: Referee,
    pub piece_size: Vec2,
    pub winning_team: Option<Team>,
}

pub const DEFAULT_PIECE_SIZE: Vec2 = vec2(0.3, 1.0);

impl Board {
    pub fn new(cursor_white: Coord, cursor_black: Coord, size: ICoord, pieces: Vec<Piece>) -> Self {
        Self {
            cursor: OneForEachTeam::new(cursor_white, cursor_black),
            selected: OneForEachTeam::new(None, None),
            size,
            pieces,
            piece_size: DEFAULT_PIECE_SIZE,
            referee: Referee::new(),
            winning_team: None,
        }
    }
    pub fn new_chess(cursor_white: Coord, cursor_black: Coord) -> Self {
        let back_column = vec![
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
        Self::new(cursor_white, cursor_black, size, pieces)
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
    pub fn deselect(&mut self, team: Team) {
        if let Some(selected_i) = self.selected(team) {
            self.pieces[selected_i].cooldown_s = Some(0.0);
            let initial = self.pieces[selected_i].initial_pos;
            let overlaps_i = self.overlapping_pieces(selected_i);
            assert!(
                overlaps_i.len() <= 1,
                "killing several pieces in the same tile is unsupported"
            );
            if let Some(overlap_i) = overlaps_i.last().cloned() {
                let moves = possible_moves(self.size, &self.pieces, selected_i);
                let referee_saw = self.referee.saw_any_piece(&self.pieces, vec![selected_i]);
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
                        self.pieces[selected_i].set_pos_and_initial_i(initial);
                    }
                } else {
                    let initial = self.pieces[selected_i].initial_pos;
                    self.pieces[overlap_i].set_pos_and_initial_i(initial);
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
                let rounded_pos = self.pieces[selected_i].pos_i().into::<ICoord>(); // TODO: leave positions unrounded
                let initial_pos = self.pieces[selected_i].initial_pos;
                let moves = possible_moves(self.size, &self.pieces, selected_i);
                let referee_saw = self.referee.saw_any_piece(&self.pieces, vec![selected_i]);
                self.pieces[selected_i].set_pos_and_initial_i(rounded_pos);
                if referee_saw {
                    if initial_pos == rounded_pos {
                        // grabbed and dropped in the same place: ok for both teams
                    } else if !self.pieces[selected_i].alive {
                        self.kill(selected_i);
                    } else if !moves.contains(&rounded_pos) {
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
            *self.selected_mut(team) = None;
            *self.cursor_mut(team) = self.cursor(team).round();
        } else {
            panic!("logic error: deselecting but there was no selection");
        }
    }
    pub fn is_selected(&self, team: Team) -> bool {
        self.selected(team).is_some()
    }
    pub fn toggle_select(&mut self, team: Team) {
        if self.is_selected(team) {
            self.deselect(team);
        } else {
            self.select(team);
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
            let attacks = compute_attackers(king, self.size, &self.pieces);
            if attacks.len() > 0 {
                return Some(king);
            }
        }
        None
    }
}

pub fn find_first(team: Team, move_type: Move, pieces: &Vec<Piece>) -> Option<PieceIndex> {
    let moveset = vec![move_type];
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
mod tests {
    use super::*;
    use crate::world::moves::tests::parse_board_cursor;
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

    fn build_board(text: &str) -> Board {
        let (size, pieces, white_cursor, black_cursor) = parse_board_cursor(text);
        let mut board = Board::new(white_cursor, black_cursor, size, pieces);
        board.set_all_seeing_referee(true);
        board
    }

    #[test]
    fn test_move_pawn() {
        let mut board = Board::new_chess(Coord::new_i(6, 0), Coord::new_i(1, 0));
        board.select(Team::White);
        board.move_cursor_rel(Coord::new_i(-1, 0), Team::White);
        board.deselect(Team::White);
        let expected_pos = Coord::new_i(5, 0);
        let at_5_0 = pieces_at(expected_pos, &board.pieces);
        assert_eq!(
            at_5_0,
            vec![
                &PieceMock::new(expected_pos, vec![Move::Pawn], Team::White)
                    .cooldown(Some(0.0))
                    .moved(true)
                    .into()
            ]
        )
    }
    #[test]
    fn test_deselect_when_killed() {
        let mut board = build_board("wqO brX");
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
        board.deselect(Team::White);
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
        let board = build_board("
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

        board.deselect(Team::White);
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
        board.deselect(Team::White);
        assert_eq!(board.referee.turn, Team::White);
        board.select(Team::Black);
        board.tick(0.1);
        board.deselect(Team::Black);
        assert_eq!(board.referee.turn, Team::White);
        assert_eq!(
            team_alive_pos(&board.pieces),
            vec![
                (Team::White, true, pos_left),
                (Team::Black, true, pos_right),
            ]
        );
    }
}
