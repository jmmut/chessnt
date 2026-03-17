use crate::core::coord::Coord;
use crate::world::board::{other_pieces_at, Board, PieceIndex};
use crate::world::moves::possible_moves;
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
    pub fn restart(&mut self) {
        for bot in self.bots.iter_mut() {
            bot.plan = None;
        }
    }
}

#[derive(Copy, Clone)]
enum Plan {
    Select(PlanSelect),
    Move(PlanMove),
}
impl Plan {
    pub fn piece_index(&self) -> PieceIndex {
        match self {
            Plan::Select(PlanSelect { piece_index, .. })
            | Plan::Move(PlanMove { piece_index, .. }) => *piece_index,
        }
    }
    pub fn destination(&self) -> Coord {
        match self {
            Plan::Select(PlanSelect { destination, .. })
            | Plan::Move(PlanMove { destination, .. }) => *destination,
        }
    }
}

#[derive(Copy, Clone)]
struct PlanSelect {
    piece_index: PieceIndex,
    destination: Coord,
    wait: i32,
}
impl PlanSelect {
    pub fn new(piece_index: PieceIndex, destination: Coord) -> Plan {
        Plan::Select(Self::new_raw(piece_index, destination))
    }
    pub fn new_raw(piece_index: PieceIndex, destination: Coord) -> PlanSelect {
        Self::new_raw_wait(piece_index, destination, WAIT_CURSOR)
    }
    pub fn new_raw_wait(piece_index: PieceIndex, destination: Coord, wait: i32) -> PlanSelect {
        Self {
            piece_index,
            destination,
            wait,
        }
    }
    pub fn raw_from(plan: Plan) -> PlanSelect {
        match plan {
            Plan::Select(plan) => plan,
            Plan::Move(plan) => Self::new_raw(plan.piece_index, plan.destination),
        }
    }
    pub fn wait(self) -> Plan {
        let raw = Self::new_raw_wait(self.piece_index, self.destination, self.wait - 1);
        Plan::Select(raw)
    }
    pub fn reset(self) -> Plan {
        Plan::Select(Self::new_raw(self.piece_index, self.destination))
    }
}
#[derive(Copy, Clone)]
struct PlanMove {
    piece_index: PieceIndex,
    destination: Coord,
}

impl PlanMove {
    pub fn new(piece_index: PieceIndex, destination: Coord) -> Plan {
        Plan::Move(Self::new_raw(piece_index, destination))
    }
    pub fn new_raw(piece_index: PieceIndex, destination: Coord) -> PlanMove {
        Self {
            piece_index,
            destination,
        }
    }
}

pub struct Bot {
    team: Team,
    plan: Option<Plan>,
    pub enabled: bool,
}

impl Bot {
    pub fn new(team: Team) -> Self {
        Self {
            team,
            plan: None,
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

fn advance_plan(plan_opt: Option<Plan>, team: Team, board: &mut Board) -> Option<Plan> {
    match plan_opt {
        None => choose_target(board, team), // nothing to do...?
        Some(plan) => {
            if let Some(selected) = board.selected(team) {
                if selected == plan.piece_index() {
                    advance_plan_move(plan.piece_index(), plan.destination(), team, board)
                } else {
                    board.deselect(team);
                    Some(Plan::Select(PlanSelect::raw_from(plan)))
                }
            } else {
                Some(advance_plan_select(PlanSelect::raw_from(plan), team, board))
            }
        }
    }
}

/// assumes no piece is selected
fn advance_plan_select(plan: PlanSelect, team: Team, board: &mut Board) -> Plan {
    let cursor_pos = board.cursor(team);
    let piece_pos = board.pieces()[plan.piece_index].pos_f();
    if piece_pos.round() == cursor_pos.round() {
        board.select(team);
        PlanMove::new(plan.piece_index, plan.destination)
    } else {
        if plan.wait <= 0 {
            move_cursor(cursor_pos, piece_pos, team, board);
            PlanSelect::reset(plan)
        } else {
            PlanSelect::wait(plan)
        }
    }
}

/// assumes correct piece is selected
fn advance_plan_move(
    piece_index: PieceIndex,
    destination: Coord,
    team: Team,
    board: &mut Board,
) -> Option<Plan> {
    let cursor_pos = board.cursor(team);
    if close_to_center_of(cursor_pos, destination) {
        // at destination. should deselect now or wait?
        let others = other_pieces_at(destination, piece_index, board.pieces());
        if others.len() > 1 {
            panic!(
                "unsupported for several pieces ({}) to own a tile {:?}",
                others.len(),
                destination
            );
        }
        if finished_moving(piece_index, team, board, others) {
            board.deselect(team);
            None
        } else {
            Some(PlanMove::new(piece_index, destination))
        }
    } else {
        // selected but not in destination, need to move
        move_selected(cursor_pos, destination, team, board);
        Some(PlanMove::new(piece_index, destination))
    }
}

fn finished_moving(
    piece_index: PieceIndex,
    team: Team,
    board: &mut Board,
    others: Vec<PieceIndex>,
) -> bool {
    if let Some(other) = others.first() {
        if board.pieces()[*other].team == team {
            panic!("swapping pieces of bot's team is unsupported");
        }
        // the other piece is from the enemy team
        // can only kill if it's my turn and the referee sees me, otherwise wait
        board.referee.turn == team && board.referee.saw_piece(board.pieces(), piece_index)
    } else {
        // can move if it's my turn or if referee doesn't see me
        board.referee.turn == team || !board.referee.saw_piece(board.pieces(), piece_index)
    }
}

fn choose_target(board: &Board, team: Team) -> Option<Plan> {
    for (i, piece) in board.pieces().iter().enumerate() {
        if piece.team == team {
            let moves = possible_moves(board.size(), board.pieces(), i);
            if let Some(movement) = moves.first() {
                return Some(PlanSelect::new(i, *movement));
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

// TODO: centralize movement by raising messages instead of modifying the board
fn move_selected(cursor_pos: Coord, destination: Coord, team: Team, board: &mut Board) {
    let max = 0.05;
    let diff = destination - cursor_pos;
    let diff = diff.normalize();
    let diff = diff * max;
    board.move_cursor_rel(diff, team);
}
