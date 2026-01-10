use crate::coord::Coord;
use crate::COLUMNS;
use macroquad::math::{vec2, Vec2};

pub struct Referee {
    position: Vec2,
    direction: Vec2,
}

impl Referee {
    pub fn new() -> Self {
        Self {
            position: vec2(COLUMNS as f32 * 0.5, -1.0),
            direction: vec2(0.0, 1.0),
        }
    }
    pub fn update(&self, delta_seconds: f32) {}
    pub fn draw(&self) {}
}
