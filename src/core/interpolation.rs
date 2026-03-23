use crate::core::coord::Coord;
use std::ops::{Add, Mul};

#[derive(Clone)]
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

pub fn smooth(t: f32) -> f32 {
    Interpolation::new(t * t, 1.0 - (1.0 - t) * (1.0 - t)).at(t)
}

#[allow(unused)]
fn quadratic(t: f32) -> f32 {
    Interpolation::new(0.0, t * t).at(t)
}
