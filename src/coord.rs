use crate::{COLUMNS, ROWS};
use macroquad::math::{f32, vec2, vec3, IVec2, Rect, Vec2, Vec3};
use std::ops::{Add, Sub};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Coord {
    pub column: f32,
    pub row: f32,
}

impl Coord {
    pub fn new_f(column: f32, row: f32) -> Self {
        Coord { column, row }
    }
    pub fn new_i(column: i32, row: i32) -> Self {
        Coord {
            column: column as f32,
            row: row as f32,
        }
    }
    pub fn row(&self) -> i32 {
        self.row.floor() as i32
    }
    pub fn column(&self) -> i32 {
        self.column.floor() as i32
    }
    pub fn row_f(&self) -> f32 {
        self.row
    }
    pub fn column_f(&self) -> f32 {
        self.column
    }
    pub fn abs(self) -> Coord {
        Coord {
            row: self.row.abs(),
            column: self.column.abs(),
        }
    }
    pub fn floor(self) -> Coord {
        Coord {
            row: self.row.floor(),
            column: self.column.floor(),
        }
    }
    pub fn into<T: From<Coord>>(self) -> T {
        Into::<T>::into(self)
    }
}
pub fn to_ivec(v: Vec2) -> IVec2 {
    let Vec2 { x, y } = v.floor();
    IVec2::new(x as i32, y as i32)
}
impl From<IVec2> for Coord {
    fn from(value: IVec2) -> Self {
        Coord::new_i(value.x, value.y)
    }
}
impl From<Coord> for IVec2 {
    fn from(value: Coord) -> Self {
        to_ivec(value.into())
    }
}
impl From<Vec2> for Coord {
    fn from(value: Vec2) -> Self {
        Coord::new_f(value.x, value.y)
    }
}
impl From<Coord> for Vec2 {
    fn from(value: Coord) -> Self {
        vec2(value.column, value.row)
    }
}
impl From<Vec3> for Coord {
    fn from(value: Vec3) -> Self {
        Coord::new_f(value.z + COLUMNS as f32 * 0.5, value.x + ROWS as f32 * 0.5)
    }
}
impl From<Coord> for Vec3 {
    fn from(value: Coord) -> Self {
        vec3(
            value.column - COLUMNS as f32 * 0.5,
            0.0,
            value.row - ROWS as f32 * 0.5,
        )
    }
}
impl Add<Coord> for Coord {
    type Output = Coord;
    fn add(self, other: Coord) -> Self::Output {
        Coord {
            row: self.row + other.row,
            column: self.column + other.column,
        }
    }
}
impl Sub<Coord> for Coord {
    type Output = Coord;
    fn sub(self, other: Coord) -> Self::Output {
        Coord {
            row: self.row - other.row,
            column: self.column - other.column,
        }
    }
}

pub fn coord_to_pixel(coord: Coord, rect: Rect) -> Vec2 {
    let cell_width = rect.w / COLUMNS as f32;
    let cell_height = rect.h / ROWS as f32;
    rect.point()
        + vec2(
            cell_width * coord.column as f32,
            cell_height * coord.row as f32,
        )
}
