use crate::coord::Coord;
use crate::world::moves::{Move, Moveset};
use crate::world::team::Team;

pub const COOLDOWN: f64 = 2.0;

#[derive(Clone)]
pub struct Piece {
    pub initial_pos: Coord,
    pub pos: Coord,
    pub moveset: Moveset,
    pub team: Team,
    pub moved: bool,
    pub alive: bool,
    pub cooldown_s: Option<f64>,
}

impl Piece {
    pub fn new(pos: Coord, movement: Move, team: Team) -> Self {
        Self {
            initial_pos: pos,
            pos,
            moveset: vec![movement],
            team,
            moved: false,
            alive: true,
            cooldown_s: None,
        }
    }
    pub fn set_pos(&mut self, new_pos: Coord) {
        self.pos = new_pos;
        self.initial_pos = new_pos;
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

