use crate::board::Piece;
use crate::coord::Coord;
use crate::COLUMNS;
use macroquad::math::{vec2, vec3, Vec2, Vec3};
use std::ops::{Add, Mul};

pub struct Interpolation<T> {
    start: T,
    end: T,
}
impl<T: Mul<f32, Output = T> + Add<f32, Output = T> + Add<T, Output = T> + Copy> Interpolation<T> {
    pub fn new(start: T, end: T) -> Self {
        Self { start, end }
    }
    /// Note that you can compound interpolation transformations like:
    /// `interp.at(smooth(quadratic(t)))`
    pub fn at(&self, t: f32) -> T {
        let t = t.clamp(0.0, 1.0);
        (self.start * (1.0 - t)) + (self.end * t)
    }
}

#[allow(unused)]
fn linear_raw(start: Coord, end: Coord, t: f32) -> Coord {
    let t = t.clamp(0.0, 1.0);
    (start * (1.0 - t)) + (end * t)
}
fn smooth(t: f32) -> f32 {
    Interpolation::new(t * t, 1.0 - (1.0 - t) * (1.0 - t)).at(t)
}

#[allow(unused)]
fn quadratic(t: f32) -> f32 {
    Interpolation::new(0.0, t * t).at(t)
}

pub struct Referee {
    position: Vec2,
    prev_position: Vec2,
    direction: Vec2,
    focused: Option<Focus>,
    interpolation_s: f64,
    interpolation: Interpolation<Coord>,
    radar_start: Option<Vec2>,
    pub trip_time: f64,
}

#[derive(Copy, Clone)]
pub struct Focus {
    time_still_s: f64,
    piece_index: usize,
}

const INITIAL_X: f32 = COLUMNS as f32 * 0.5 - 0.5;
const INITIAL_RIGHT: Coord = Coord::new_f(INITIAL_X + 1.0, -1.0);
const INITIAL_LEFT: Coord = Coord::new_f(INITIAL_X - 1.0, -1.0);
const DIR_MULTIPLIER: f32 = 9.0;
const VIGILANCE_TIMER: f64 = 1.0;
const REFEREE_TRIP_TIME: f64 = 4.0;
const REFEREE_SPEED: f32 =
    (INITIAL_RIGHT.column - INITIAL_LEFT.column).abs() / REFEREE_TRIP_TIME as f32;

impl Referee {
    pub fn new() -> Self {
        let initial_c = Coord::new_f(INITIAL_X, -1.0);
        let initial = initial_c.into();
        let trip = Coord::new_f(1.0, 0.0);
        let interpolation = Interpolation::new(initial_c, initial_c + trip);
        Self {
            position: initial,
            prev_position: initial,
            direction: vec2(0.0, 1.0),
            focused: None,
            interpolation_s: 0.0,
            interpolation,
            radar_start: None,
            trip_time: (trip.column.abs() / REFEREE_SPEED) as f64,
        }
    }
    pub fn tick(&mut self, delta_s: f64, pieces: &Vec<Piece>) {
        self.maybe_focus(delta_s, pieces);
        if let Some(focus) = &self.focused {
            self.direction =
                (pieces[focus.piece_index].pos.into::<Vec2>() - self.position).normalize();
        } else {
            self.interpolation_s += delta_s;
            self.prev_position = self.position;
            self.position = self
                .interpolation
                .at(smooth((self.interpolation_s / self.trip_time) as f32))
                .into();
            let y = 1.0;
            let x = (self.position.x - self.prev_position.x) / delta_s as f32
                * self.trip_time as f32
                * 0.4;
            let dir_end = vec2(x, y);
            if let Some(radar_start) = self.radar_start {
                self.direction = Interpolation::new(radar_start, dir_end)
                    .at(smooth((self.interpolation_s / self.trip_time) as f32));
            } else {
                self.direction = dir_end;
            }
        }
        if self.interpolation_s >= self.trip_time {
            self.reset_referee_movement_interp();
        }
    }

    fn reset_referee_movement_interp(&mut self) {
        let (start, end) = if self.position.x > INITIAL_X {
            (INITIAL_RIGHT, INITIAL_LEFT)
        } else {
            (INITIAL_LEFT, INITIAL_RIGHT)
        };
        self.reset_referee_movement_interp_from(start, end);
    }

    fn reset_referee_movement_interp_from(&mut self, initial: Coord, end: Coord) {
        self.interpolation = Interpolation::new(initial, end);
        self.interpolation_s = 0.0;
        self.trip_time = ((end - initial).column.abs() / REFEREE_SPEED) as f64;
        self.radar_start = None;
    }

    fn maybe_focus(&mut self, delta_s: f64, pieces: &Vec<Piece>) {
        for (piece_index, piece) in pieces.iter().enumerate() {
            if piece.moved && triangle_contains(self.radar(), piece.pos) {
                self.focused = Some(Focus {
                    time_still_s: 0.0,
                    piece_index,
                });
                return;
            }
        }
        if let Some(focus) = &mut self.focused {
            focus.time_still_s += delta_s;
            if focus.time_still_s > VIGILANCE_TIMER {
                self.focused = None;
                let end = if self.position.x > INITIAL_X {
                    INITIAL_LEFT
                } else {
                    INITIAL_RIGHT
                };
                let radar_start = self.direction;
                self.reset_referee_movement_interp_from(self.pos_c(), end);
                self.radar_start = Some(radar_start);
            }
        }
    }

    pub fn looking_leftwards(&self) -> bool {
        self.direction.x < 0.0
    }
    pub fn pos_c(&self) -> Coord {
        self.position.into()
    }
    pub fn pos_v2(&self) -> Vec2 {
        self.position
    }
    pub fn dir_c(&self) -> Coord {
        let mut d: Coord = self.direction.into();
        d = d.normalize();
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

    pub fn saw_any_piece(&self, pieces: &Vec<Piece>, indexes: Vec<usize>) -> bool {
        let radar = self.radar();
        indexes
            .iter()
            .any(|index| triangle_contains(radar, pieces[*index].pos))
    }
}

fn triangle_contains(triangle: [Coord; 3], point: Coord) -> bool {
    counter_clockwise_triangle([triangle[0], triangle[1], point])
        && counter_clockwise_triangle([triangle[1], triangle[2], point])
        && counter_clockwise_triangle([triangle[2], triangle[0], point])
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
