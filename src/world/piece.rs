use crate::core::coord::{Coord, ICoord};
use crate::world::moves::{Move, Moveset};
use crate::world::team::Team;

pub const COOLDOWN: f64 = 2.0;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Piece {
    pub initial_pos: ICoord,
    pos: Coord,
    pub moveset: Moveset,
    pub team: Team,
    pub moved: bool,
    pub alive: bool,
    pub cooldown_s: Option<f64>,
}
#[derive(Debug)]
pub struct PieceMock {
    pub initial_pos: ICoord,
    pos: Coord,
    pub moveset: Moveset,
    pub team: Team,
    pub moved: Option<bool>,
    pub alive: Option<bool>,
    pub cooldown_s: Option<f64>,
}
impl PieceMock {
    pub fn new(pos: Coord, moveset: Moveset, team: Team) -> Self {
        PieceMock {
            initial_pos: pos.into(),
            pos,
            moveset,
            team,
            moved: None,
            alive: None,
            cooldown_s: None,
        }
    }
    pub fn pos(mut self, pos: Coord) -> Self {
        self.pos = pos;
        self
    }
    pub fn initial_pos(mut self, initial_pos: ICoord) -> Self {
        self.initial_pos = initial_pos;
        self
    }
    pub fn moved(mut self, moved: bool) -> Self {
        self.moved = Some(moved);
        self
    }
    pub fn alive(mut self, alive: bool) -> Self {
        self.alive = Some(alive);
        self
    }
    pub fn cooldown(mut self, cooldown: Option<f64>) -> Self {
        self.cooldown_s = cooldown;
        self
    }
}
impl From<PieceMock> for Piece {
    fn from(value: PieceMock) -> Self {
        let mut piece = Self::new(value.pos, value.team, Move::Pawn);
        piece.initial_pos = value.initial_pos;
        piece.moveset = value.moveset;
        if let Some(moved) = value.moved {
            piece.moved = moved;
        }
        if let Some(alive) = value.alive {
            piece.alive = alive;
        }
        piece.cooldown_s = value.cooldown_s;
        piece
    }
}
impl From<&Piece> for PieceMock {
    fn from(value: &Piece) -> Self {
        Self {
            initial_pos: value.initial_pos,
            pos: value.pos,
            moveset: value.moveset.clone(),
            team: value.team,
            moved: Some(value.moved),
            alive: Some(value.alive),
            cooldown_s: value.cooldown_s,
        }
    }
}
impl Piece {
    pub fn new(pos: Coord, team: Team, movement: Move) -> Self {
        Self {
            initial_pos: pos.into(),
            pos,
            moveset: vec![movement],
            team,
            moved: false,
            alive: true,
            cooldown_s: None,
        }
    }
    pub fn set_pos_and_initial(&mut self, new_pos: Coord) {
        self.pos = new_pos;
        self.initial_pos = new_pos.into();
    }
    pub fn set_pos_and_initial_i(&mut self, new_pos: ICoord) {
        self.pos = new_pos.into();
        self.initial_pos = new_pos;
    }
    pub fn set_pos(&mut self, new_pos: Coord) {
        self.pos = new_pos;
    }
    pub fn move_rel(&mut self, delta: Coord) {
        self.pos += delta;
    }
    pub fn move_rel_and_initial(&mut self, delta: Coord) {
        self.pos += delta;
        self.initial_pos = self.pos.into();
    }
    pub fn pos_i(&self) -> Coord {
        self.pos.round()
    }
    pub fn pos_ii(&self) -> ICoord {
        self.pos.round().into()
    }
    pub fn pos_initial_i(&self) -> ICoord {
        self.initial_pos
    }
    pub fn pos_f(&self) -> Coord {
        self.pos
    }
    pub fn tick(&mut self, delta_s: f64) {
        self.moved = false;
        self.cooldown_s = if let Some(mut cooldown) = self.cooldown_s {
            cooldown += delta_s;
            if cooldown > COOLDOWN {
                None
            } else {
                Some(cooldown)
            }
        } else {
            None
        }
    }

    pub fn cooldown_progress(&self) -> Option<f64> {
        if let Some(cooldown) = self.cooldown_s {
            Some(cooldown / COOLDOWN)
        } else {
            None
        }
    }
}

// impl Ord for Piece {
//     fn cmp(&self, other: &Self) -> Ordering {
//         self.team.partial_cmp(&other.team).unwrap().then_with(|| {
//             self.moveset.partial_cmp(&other.moveset).unwrap().then_with(|| {
//                 self.pos.partial_cmp(&other.pos).unwrap()
//             })
//         })
//     }
// }
// impl Eq for Piece {
//
// }
