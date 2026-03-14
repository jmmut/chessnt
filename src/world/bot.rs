use crate::core::coord::Coord;
use crate::world::board::{other_pieces_at, Board, PieceIndex};
use crate::world::moves::possible_moves;
use crate::world::piece::Piece;
use crate::world::team::Team;
use macroquad::math::Vec2;

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
                if let Some((piece_index, _piece, destination)) = self.choose_target(board) {
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
                        if close_to_center_of(cursor_pos, destination) {
                            // at destination. should deselect now or wait?
                            let others = other_pieces_at(destination, selected, board.pieces());
                            if others.len() > 1 {
                                panic!(
                                    "unsupported for several pieces ({}) to own a tile {:?}",
                                    others.len(),
                                    destination
                                );
                            }
                            if let Some(other) = others.first() {
                                // there's another piece at destination
                                if board.pieces()[*other].team == self.team {
                                    panic!("swapping pieces of bot's team is unsupported");
                                }
                                // the other piece is from the enemy team
                                if board.referee.turn == self.team {
                                    // allowed to kill

                                    if board.referee.saw_piece(board.pieces(), selected) {
                                        board.deselect(self.team);
                                        self.plan = Plan::None;
                                    } else {
                                        // wait until referee sees and allows me to kill
                                    }
                                } else {
                                    // should wait until it's my turn
                                }
                            } else {
                                // no other piece at destination
                                if board.referee.turn == self.team {
                                    // allowed to move
                                    board.deselect(self.team);
                                    self.plan = Plan::None;
                                } else {
                                    // not our turn
                                    if board.referee.saw_piece(board.pieces(), selected) {
                                        // should wait until referee doesn't see us
                                    } else {
                                        // forbidden movement but referee doesn't see
                                        board.deselect(self.team);
                                        self.plan = Plan::None;
                                    }
                                }
                            }
                        } else {
                            // selected but not in destination, need to move
                            let diff = destination - cursor_pos;
                            let diff = diff.normalize();
                            let diff = diff * max;
                            board.move_cursor_rel(diff, self.team);
                        }
                    } else {
                        // selected wrong piece
                        board.deselect(self.team);
                        self.plan = Plan::Select {
                            piece_index,
                            destination,
                            wait: WAIT_CURSOR,
                        };
                    }
                } else {
                    // have no piece selected
                    self.plan = Plan::Select {
                        piece_index,
                        destination,
                        wait: WAIT_CURSOR,
                    };
                }
            }
        }
    }

    fn choose_target<'a>(&self, board: &'a Board) -> Option<(usize, &'a Piece, Coord)> {
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

fn close_to_center_of(cursor_pos: Coord, destination: Coord) -> bool {
    (cursor_pos - destination.round())
        .into::<Vec2>()
        .length_squared()
        < 0.1
}
