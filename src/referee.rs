use crate::board::Piece;
use crate::coord::Coord;
use crate::COLUMNS;
use macroquad::math::{vec2, vec3, Vec2, Vec3};

pub struct Interpolation {
    start: Coord,
    end: Coord,
}
impl Interpolation {
    pub fn new(start: Coord, end: Coord) -> Self {
        Self { start, end }
    }
    pub fn at_linear(&self, t: f32) -> Coord {
        let t = t.clamp(0.0, 1.0);
        self.end * t + self.start * (1.0 - t)
    }
}
pub struct Referee {
    position: Vec2,
    prev_position: Vec2,
    direction: Vec2,
    focused: Option<Focus>,
    last_time_s: f64,
    interpolation_s: f64,
    interpolation: Interpolation,
}
#[derive(Copy, Clone)]
pub struct Focus {
    last_movement_time_s: f64,
    piece_index: usize,
}

const INITIAL_X: f32 = COLUMNS as f32 * 0.5 - 0.5;
const DIR_MULTIPLIER: f32 = 10.0;
const VIGILANCE_TIMER: f64 = 1.0;
const REFEREE_TRIP_TIME: f64 = 2.0;

impl Referee {
    pub fn new() -> Self {
        let initial_c = Coord::new_f(INITIAL_X, -1.0);
        let initial = initial_c.into();
        Self {
            position: initial,
            prev_position: initial,
            direction: vec2(0.0, 1.0),
            focused: None,
            last_time_s: 0.0,
            interpolation_s: 0.0,
            interpolation: Interpolation::new(initial_c, initial_c + Coord::new_f(1.0, 0.0)),
        }
    }
    pub fn tick(&mut self, time_s: f64, pieces: &Vec<Piece>) {
        self.maybe_focus(time_s, pieces);
        if let Some(focus) = &self.focused {
            self.direction = (pieces[focus.piece_index].pos.into::<Vec2>() - self.position).normalize();
        } else {
            if self.last_time_s != 0.0 {
                let delta_s = time_s - self.last_time_s;
                self.interpolation_s += delta_s;

                self.prev_position = self.position;
                self.position = self.interpolation.at_linear((self.interpolation_s / REFEREE_TRIP_TIME) as f32).into();
                // self.direction.x = cos as f32;
            }
            self.last_time_s = time_s;
        }
    }

    fn maybe_focus(&mut self, time_s: f64, pieces: &Vec<Piece>) {
        for (piece_index, piece) in pieces.iter().enumerate() {
            if piece.moved && triangle_contains(self.radar(), piece.pos) {
                self.focused = Some(Focus { last_movement_time_s: time_s, piece_index });
                return;
            }
        }
        if let Some(focus) = self.focused {
            if time_s - focus.last_movement_time_s > VIGILANCE_TIMER {
                self.focused = None;
            }
        }
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
    pub fn radar(&self) -> [Coord; 3] {
        let radar_base = self.pos_c();
        let dir = self.dir_c();
        let left = rotate_90(dir) * 0.7;
        let radar_left = radar_base + dir + left;
        let radar_right = radar_base + dir - left;
        [radar_base, radar_right, radar_left]
    }
    pub fn dir_v3(&self) -> Vec3 {
        vec3(self.direction.x, 0.0, self.direction.y) * DIR_MULTIPLIER
    }
}

fn triangle_contains(triangle: [Coord; 3], point: Coord) -> bool {
    counter_clockwise_triangle([triangle[0], triangle[1], point])
    &&   counter_clockwise_triangle([triangle[1], triangle[2], point])
    &&   counter_clockwise_triangle([triangle[2], triangle[0], point])
}

fn counter_clockwise_triangle(triangle: [Coord; 3]) -> bool {
    (triangle[1] - triangle[0])
        .into::<Vec2>()
        .perp_dot((triangle[2] - triangle[0]).into::<Vec2>())
        >= 0.0
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
pub fn rotate_90(v: Coord) -> Coord {
    Coord {
        column: -v.row,
        row: v.column,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TRIANGLE: [Coord; 3] = [Coord::new_i(4, 0), Coord::new_i(2, 2), Coord::new_i(2, 0)];

    #[test]
    fn test_triangle_contains() {
        let contains = triangle_contains(TRIANGLE, Coord::new_f(2.5, 0.5));
        assert_eq!(contains, true);
    }
    #[test]
    fn test_triangle_does_not_contain_01() {
        let contains = triangle_contains(TRIANGLE, Coord::new_f(3.5, 1.5));
        assert_eq!(contains, false);
    }
    #[test]
    fn test_triangle_does_not_contain_12() {
        let contains = triangle_contains(TRIANGLE, Coord::new_f(0.0, 0.5));
        assert_eq!(contains, false);
    }
    #[test]
    fn test_triangle_does_not_contain_20() {
        let contains = triangle_contains(TRIANGLE, Coord::new_f(2.5, -0.5));
        assert_eq!(contains, false);
    }
    #[test]
    fn test_triangle_does_not_contain_01_12() {
        let contains = triangle_contains(TRIANGLE, Coord::new_f(-0.5, 10.0));
        assert_eq!(contains, false);
    }
}
