use crate::core::coord::Coord;
use crate::world::board::{Board, PieceIndex};
use crate::world::moves::possible_moves;
use crate::world::piece::Piece;
use crate::world::team::Team;

pub const WAIT_CURSOR: i32 = 20;

enum Plan {
    None,
    Select {
        piece_index: PieceIndex,
        destination: Coord,
        wait: i32,
    },
    Move {
        piece_index: PieceIndex,
        destination: Coord,
    },
}
pub struct Bot {
    team: Team,
    plan: Plan,
}

impl Bot {
    pub fn new(team: Team) -> Self {
        Self {
            team,
            plan: Plan::None,
        }
    }
    pub fn tick(&mut self, _delta_s: f64, board: &mut Board) {
        match self.plan {
            Plan::None => {
                if let Some((piece_index, _piece, destination)) = self.any_movement(board) {
                    if let Some(index) = board.selected(self.team) {
                        if index == piece_index {
                            self.plan = Plan::Move {
                                piece_index,
                                destination,
                            }
                        } else {
                            board.deselect(self.team);
                            self.plan = Plan::Select {
                                piece_index,
                                destination,
                                wait: WAIT_CURSOR,
                            };
                        }
                    } else {
                        self.plan = Plan::Select {
                            piece_index,
                            destination,
                            wait: WAIT_CURSOR,
                        };
                    }
                } else {
                    // nothing to do...?
                }
            }
            Plan::Select {
                piece_index,
                destination,
                wait,
            } => {
                if let Some(selected) = board.selected(self.team) {
                    if selected == piece_index {
                        self.plan = Plan::Move {
                            piece_index,
                            destination,
                        };
                    } else {
                        board.deselect(self.team);
                    }
                } else {
                    let cursor_pos = board.cursor(self.team);
                    let piece_pos = &board.pieces()[piece_index].pos_f();
                    if piece_pos.round() == cursor_pos.round() {
                        board.select(self.team);
                        self.plan = Plan::Move {
                            piece_index,
                            destination,
                        };
                    } else {
                        if wait <= 0 {
                            let diff = *piece_pos - cursor_pos;
                            let movement = if diff.column_f().abs() >= diff.row_f().abs() {
                                Coord::new_i(diff.column().signum(), 0)
                            } else {
                                Coord::new_i(0, diff.row().signum())
                            };
                            board.move_cursor_rel(movement, self.team);
                            self.plan = Plan::Select {
                                piece_index,
                                destination,
                                wait: WAIT_CURSOR,
                            };
                        } else {
                            self.plan = Plan::Select {
                                piece_index,
                                destination,
                                wait: wait - 1,
                            }
                        }
                    }
                }
            }
            Plan::Move {
                piece_index,
                destination,
            } => {
                if let Some(selected) = board.selected(self.team) {
                    if selected == piece_index {
                        let max = 0.05;
                        let cursor_pos = board.cursor(self.team);
                        if cursor_pos.round() == destination.round() {
                            board.deselect(self.team);
                            self.plan = Plan::None;
                        } else {
                            let diff = destination - cursor_pos;
                            let diff = diff.normalize();
                            let diff = diff * max;
                            board.move_cursor_rel(diff, self.team);
                        }
                    } else {
                        board.deselect(self.team);
                        self.plan = Plan::Select {
                            piece_index,
                            destination,
                            wait: WAIT_CURSOR,
                        };
                    }
                } else {
                    self.plan = Plan::Select {
                        piece_index,
                        destination,
                        wait: WAIT_CURSOR,
                    };
                }
            }
        }
    }

    fn any_movement<'a>(&self, board: &'a Board) -> Option<(usize, &'a Piece, Coord)> {
        for (i, piece) in board.pieces().iter().enumerate() {
            if piece.team == self.team {
                let moves = possible_moves(board.size(), board.pieces(), i);
                if let Some(movement) = moves.first() {
                    return Some((i, piece, *movement));
                }
            }
        }
        None
    }
}
