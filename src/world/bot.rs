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
impl From<PlanSelect> for Plan {
    fn from(value: PlanSelect) -> Self {
        Plan::Select(value)
    }
}
impl From<PlanMove> for Plan {
    fn from(value: PlanMove) -> Self {
        Plan::Move(value)
    }
}
impl Plan {
    pub fn piece_index(&self) -> PieceIndex {
        match self {
            Plan::Select(PlanSelect { piece_index, .. })
            | Plan::Move(PlanMove { piece_index, .. }) => *piece_index,
        }
    }
}

#[derive(Copy, Clone)]
struct PlanSelect {
    piece_index: PieceIndex,
    destination: Coord,
    wait: i32,
}
impl From<Plan> for PlanSelect {
    fn from(value: Plan) -> Self {
        match value {
            Plan::Select(plan) => plan,
            Plan::Move(plan) => plan.into(),
        }
    }
}
impl From<PlanMove> for PlanSelect {
    fn from(value: PlanMove) -> Self {
        Self::new(value.piece_index, value.destination)
    }
}
impl PlanSelect {
    pub fn new(piece_index: PieceIndex, destination: Coord) -> PlanSelect {
        Self {
            piece_index,
            destination,
            wait: WAIT_CURSOR,
        }
    }
    pub fn wait(self) -> PlanSelect {
        Self {
            piece_index: self.piece_index,
            destination: self.destination,
            wait: self.wait - 1,
        }
    }
    pub fn reset(self) -> PlanSelect {
        PlanSelect::new(self.piece_index, self.destination)
    }
}
#[derive(Copy, Clone)]
struct PlanMove {
    piece_index: PieceIndex,
    destination: Coord,
}
impl From<Plan> for PlanMove {
    fn from(value: Plan) -> Self {
        match value {
            Plan::Select(plan) => plan.into(),
            Plan::Move(plan) => plan,
        }
    }
}
impl From<PlanSelect> for PlanMove {
    fn from(value: PlanSelect) -> Self {
        Self::new(value.piece_index, value.destination)
    }
}

impl PlanMove {
    pub fn new(piece_index: PieceIndex, destination: Coord) -> PlanMove {
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
                    advance_plan_move(plan.into(), team, board)
                } else {
                    board.deselect(team);
                    Some(PlanSelect::from(plan).into())
                }
            } else {
                Some(advance_plan_select(plan.into(), team, board))
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
        PlanMove::from(plan).into()
    } else {
        if plan.wait <= 0 {
            move_cursor(cursor_pos, piece_pos, team, board);
            PlanSelect::reset(plan).into()
        } else {
            PlanSelect::wait(plan).into()
        }
    }
}

/// assumes correct piece is selected
fn advance_plan_move(plan: PlanMove, team: Team, board: &mut Board) -> Option<Plan> {
    let cursor_pos = board.cursor(team);
    if close_to_center_of(cursor_pos, plan.destination) {
        // at destination. should deselect now or wait?
        let others = other_pieces_at(plan.destination, plan.piece_index, board.pieces());
        if others.len() > 1 {
            panic!(
                "unsupported for several pieces ({}) to own a tile {:?}",
                others.len(),
                plan.destination
            );
        }
        if let Some(other) = others.first() {
            advance_plan_kill(plan, team, other, board)
        } else {
            advance_plan_move_empty(plan, team, board)
        }
    } else {
        // selected but not in destination, need to move
        move_selected(cursor_pos, plan.destination, team, board);
        Some(PlanMove::from(plan).into())
    }
}

/// assumes there's another piece at destination
fn advance_plan_kill(
    plan: PlanMove,
    team: Team,
    other: &PieceIndex,
    board: &mut Board,
) -> Option<Plan> {
    if board.pieces()[*other].team == team {
        panic!("swapping pieces of bot's team is unsupported");
    }
    // the other piece is from the enemy team
    if board.referee.turn == team {
        // allowed to kill
        if board.referee.saw_piece(board.pieces(), plan.piece_index) {
            board.deselect(team);
            None
        } else {
            // wait until referee sees and allows me to kill
            Some(PlanMove::from(plan).into())
        }
    } else {
        // should wait until it's my turn
        Some(PlanMove::from(plan).into())
    }
}

/// assumes destination is empty
fn advance_plan_move_empty(plan: PlanMove, team: Team, board: &mut Board) -> Option<Plan> {
    if board.referee.turn == team {
        // allowed to move
        board.deselect(team);
        None
    } else {
        // not our turn
        if board.referee.saw_piece(board.pieces(), plan.piece_index) {
            // should wait until referee doesn't see us
            Some(PlanMove::from(plan).into())
        } else {
            // forbidden movement but referee doesn't see
            board.deselect(team);
            None
        }
    }
}

fn choose_target(board: &Board, team: Team) -> Option<Plan> {
    for (i, piece) in board.pieces().iter().enumerate() {
        if piece.team == team {
            let moves = possible_moves(board.size(), board.pieces(), i);
            if let Some(movement) = moves.first() {
                return Some(PlanSelect::new(i, *movement).into());
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
