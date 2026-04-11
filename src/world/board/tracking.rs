use crate::core::coord::ICoord;
use crate::world::board::{IndexesMoves, PieceIndex};
use crate::world::moves::{Move, starting_pawn_column};
use crate::world::piece::Pieces;
use crate::world::team::{OneForEachTeam, Team};
use std::vec;

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
    pawns_double_jump_turn: Vec<Option<i32>>,
    turn: i32,
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
        let indexes_moves = vec::from_elem(0, pieces.len());
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
        let pawns_double_jump_turn = vec::from_elem(None, pieces.len());
        Self {
            indexes_moves,
            // castle_allowed,
            rooks_king,
            pawns_double_jump_turn,
            turn: 0,
        }
    }
    pub const fn new_forbidden() -> Self {
        Self {
            indexes_moves: Vec::new(),
            rooks_king: OneForEachTeam::new(None, None),
            pawns_double_jump_turn: Vec::new(),
            turn: 0,
        }
    }

    pub fn register_movement(
        &mut self,
        piece_index: PieceIndex,
        pieces: &Pieces,
        from: ICoord,
        to: ICoord,
        board_size: ICoord,
    ) {
        // TODO: need something like Vec<MovementsSincePawnDoubleJumped> indexed by piece index
        let count = &mut self.indexes_moves[piece_index];
        *count += 1;
        self.turn += 1;
        self.pawns_double_jump_turn[piece_index] = if pieces[piece_index].moveset.single()
            == Move::Pawn
            && (to - from).length_squared() == 2 * 2
            && from.column() == starting_pawn_column(board_size, pieces[piece_index].team)
        {
            Some(self.turn)
        } else {
            None
        };
    }
    pub fn undo_movement(&mut self, piece_index: PieceIndex) {
        let count = &mut self.indexes_moves[piece_index];
        *count -= 1;
        self.turn -= 1;
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
    pub fn en_passantable(&self, piece_index: PieceIndex) -> bool {
        if let Some(Some(turn)) = self.pawns_double_jump_turn.get(piece_index) {
            *turn == self.turn
        } else {
            false
        }
    }
}
