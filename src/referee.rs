use crate::coord::Coord;
use crate::COLUMNS;
use macroquad::math::{vec2, vec3, Vec2, Vec3};

pub struct Referee {
    position: Vec2,
    prev_position: Vec2,
    direction: Vec2,
}

const INITIAL_X: f32 = COLUMNS as f32 * 0.5 - 0.5;
const DIR_MULTIPLIER: f32 = 5.0;

impl Referee {
    pub fn new() -> Self {
        let initial = Coord::new_f(INITIAL_X, -1.0).into();
        Self {
            position: initial,
            prev_position: initial,
            direction: vec2(0.0, 1.0),
        }
    }
    pub fn tick(&mut self, time_s: f64) {
        self.prev_position = self.position;
        self.direction.x = time_s.sin() as f32;
        self.position.x = INITIAL_X + self.direction.x;
    }
    pub fn side(&self) -> bool {
        (self.position.x - self.prev_position.x) < 0.0
    }
    pub fn pos_c(&self) -> Coord {
        self.position.into()
    }
    pub fn pos_v2(&self) -> Vec2 {
        self.position
    }
    pub fn dir_c(&self) -> Coord {
        let mut d = self.direction.into();
        d *= DIR_MULTIPLIER;
        d
    }
    pub fn dir_v3(&self) -> Vec3 {
        vec3(self.direction.x, 0.0, self.direction.y) * DIR_MULTIPLIER
    }
}

#[allow(unused)]
pub fn rotate_y(mut v: Vec3, angle: f32) -> Vec3 {
    let (sin, cos) = angle.sin_cos();
    v.x = v.x * cos - v.z * sin;
    v.z = v.x * sin + v.z * cos;
    v
}

pub fn rotate_y_90(v: Vec3) -> Vec3 {
    vec3(-v.z, v.y, v.x)
}
