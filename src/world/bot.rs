use crate::core::coord::Coord;
use crate::world::board::{other_pieces_at, Board, PieceIndex};
use crate::world::moves::possible_moves;
use crate::world::piece::Piece;
use crate::world::team::{OneForEachTeam, Team};
use macroquad::math::Vec2;

pub const WAIT_CURSOR: i32 = 20;

pub struct Bots {
    pub bots: OneForEachTeam<Bot>,
}

impl Bots {
    pub fn new() -> Self {
        Self {
            bots: OneForEachTeam::new_from_factory(Bot::new),
        }
    }
    pub fn tick(&mut self, delta_s: f64, board: &mut Board) {
        for bot in self.bots.iter_mut() {
            if bot.enabled {
                bot.tick(delta_s, board)
            }
        }
    }
}

#[derive(Copy, Clone)]
enum Plan {
    None,
    Select(PlanSelect),
    Move(PlanMove),
}

#[derive(Copy, Clone)]
struct PlanSelect {
    piece_index: PieceIndex,
    destination: Coord,
    wait: i32,
}
impl PlanSelect {
    pub fn new(piece_index: PieceIndex, destination: Coord) -> Plan {
        Plan::Select(Self {
            piece_index,
            destination,
            wait: WAIT_CURSOR,
        })
    }
    pub fn to_move(&self) -> Plan {
        PlanMove::new(self.piece_index, self.destination)
    }
    pub fn to_select(&self) -> Plan {
        PlanSelect::new(self.piece_index, self.destination)
    }
    pub fn to_select_wait(&self) -> Plan {
        Plan::Select(Self {
            piece_index: self.piece_index,
            destination: self.destination,
            wait: self.wait - 1,
        })
    }
}
#[derive(Copy, Clone)]
struct PlanMove {
    piece_index: PieceIndex,
    destination: Coord,
}
impl PlanMove {
    pub fn new(piece_index: PieceIndex, destination: Coord) -> Plan {
        Plan::Move(Self {
            piece_index,
            destination,
        })
    }
    pub fn to_select(&self) -> Plan {
        PlanSelect::new(self.piece_index, self.destination)
    }
    pub fn to_move(&self) -> Plan {
        PlanMove::new(self.piece_index, self.destination)
    }
}

pub struct Bot {
    team: Team,
    plan: Plan,
    pub enabled: bool,
}

impl Bot {
    pub fn new(team: Team) -> Self {
        Self {
            team,
            plan: Plan::None,
            enabled: false,
        }
    }
    pub fn toggle(&mut self) {
        self.enabled = !self.enabled;
    }

    pub fn tick(&mut self, _delta_s: f64, board: &mut Board) {
        self.plan = advance_plan(self.plan, self.team, board);
    }
}

fn advance_plan(plan: Plan, team: Team, board: &mut Board) -> Plan {
    match plan {
        Plan::None => advance_plan_none(team, board),
        Plan::Select(plan) => advance_plan_select(plan, team, board),
        Plan::Move(plan) => advance_plan_move(plan, team, board),
    }
}

fn advance_plan_none(team: Team, board: &mut Board) -> Plan {
    if let Some((piece_index, _piece, destination)) = choose_target(board, team) {
        if let Some(index) = board.selected(team) {
            if index == piece_index {
                PlanMove::new(piece_index, destination)
            } else {
                board.deselect(team);
                PlanSelect::new(piece_index, destination)
            }
        } else {
            PlanSelect::new(piece_index, destination)
        }
    } else {
        // nothing to do...?
        Plan::None
    }
}

fn advance_plan_select(plan: PlanSelect, team: Team, board: &mut Board) -> Plan {
    if let Some(selected) = board.selected(team) {
        if selected == plan.piece_index {
            plan.to_move()
        } else {
            board.deselect(team);
            plan.to_select()
        }
    } else {
        let cursor_pos = board.cursor(team);
        let piece_pos = board.pieces()[plan.piece_index].pos_f();
        if piece_pos.round() == cursor_pos.round() {
            board.select(team);
            plan.to_move()
        } else {
            if plan.wait <= 0 {
                move_cursor(cursor_pos, piece_pos, team, board);
                plan.to_select()
            } else {
                plan.to_select_wait()
            }
        }
    }
}

fn advance_plan_move(plan: PlanMove, team: Team, board: &mut Board) -> Plan {
    if let Some(selected) = board.selected(team) {
        if selected == plan.piece_index {
            let cursor_pos = board.cursor(team);
            if close_to_center_of(cursor_pos, plan.destination) {
                // at destination. should deselect now or wait?
                let others = other_pieces_at(plan.destination, selected, board.pieces());
                if others.len() > 1 {
                    panic!(
                        "unsupported for several pieces ({}) to own a tile {:?}",
                        others.len(),
                        plan.destination
                    );
                }
                if let Some(other) = others.first() {
                    // there's another piece at destination
                    if board.pieces()[*other].team == team {
                        panic!("swapping pieces of bot's team is unsupported");
                    }
                    // the other piece is from the enemy team
                    if board.referee.turn == team {
                        // allowed to kill
                        if board.referee.saw_piece(board.pieces(), selected) {
                            board.deselect(team);
                            Plan::None
                        } else {
                            // wait until referee sees and allows me to kill
                            plan.to_move()
                        }
                    } else {
                        // should wait until it's my turn
                        plan.to_move()
                    }
                } else {
                    // no other piece at destination
                    if board.referee.turn == team {
                        // allowed to move
                        board.deselect(team);
                        Plan::None
                    } else {
                        // not our turn
                        if board.referee.saw_piece(board.pieces(), selected) {
                            // should wait until referee doesn't see us
                            plan.to_move()
                        } else {
                            // forbidden movement but referee doesn't see
                            board.deselect(team);
                            Plan::None
                        }
                    }
                }
            } else {
                // selected but not in destination, need to move
                move_selected(cursor_pos, plan.destination, team, board);
                Plan::Move(plan)
            }
        } else {
            // selected wrong piece
            board.deselect(team);
            plan.to_select()
        }
    } else {
        // have no piece selected
        plan.to_select()
    }
}

fn choose_target(board: &Board, team: Team) -> Option<(usize, &Piece, Coord)> {
    for (i, piece) in board.pieces().iter().enumerate() {
        if piece.team == team {
            let moves = possible_moves(board.size(), board.pieces(), i);
            if let Some(movement) = moves.first() {
                return Some((i, piece, *movement));
            }
        }
    }
    None
}

fn close_to_center_of(cursor_pos: Coord, destination: Coord) -> bool {
    (cursor_pos - destination.round())
        .into::<Vec2>()
        .length_squared()
        < 0.05
}

fn move_cursor(cursor_pos: Coord, piece_pos: Coord, team: Team, board: &mut Board) {
    let diff = piece_pos - cursor_pos;
    let movement = if diff.column_f().abs() >= diff.row_f().abs() {
        Coord::new_i(diff.column().signum(), 0)
    } else {
        Coord::new_i(0, diff.row().signum())
    };
    board.move_cursor_rel(movement, team);
}

fn move_selected(cursor_pos: Coord, destination: Coord, team: Team, board: &mut Board) {
    let max = 0.05;
    let diff = destination - cursor_pos;
    let diff = diff.normalize();
    let diff = diff * max;
    board.move_cursor_rel(diff, team);
}
