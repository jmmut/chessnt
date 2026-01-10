use crate::coord::Coord;
use crate::COLUMNS;
use macroquad::math::{vec2, Vec2};

pub struct Referee {
    position: Vec2,
    direction: Vec2,
}

const INITIAL_X: f32 = COLUMNS as f32 * 0.5 - 0.5;

impl Referee {
    pub fn new() -> Self {
        Self {
            position: vec2(INITIAL_X, -1.0),
            direction: vec2(0.0, 1.0),
        }
    }
    pub fn tick(&mut self, time_s: f64) {
        self.direction.x = time_s.sin() as f32;
        self.position.x = INITIAL_X + self.direction.x;
    }
    pub fn side(&self) -> bool {
        self.direction.x < 0.0
    }
    pub fn pos_v2(&self) -> Vec2 {
        self.position
    }
    pub fn pos_c(&self) -> Coord {
        self.position.into()
    }
}
