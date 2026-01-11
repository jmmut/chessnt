use crate::{COLUMNS, ROWS};
use macroquad::math::{f32, vec2, vec3, IVec2, Vec2, Vec3};
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Coord {
    pub column: f32,
    pub row: f32,
}

impl Coord {
    pub const fn new_f(column: f32, row: f32) -> Self {
        Coord { column, row }
    }
    pub const fn new_i(column: i32, row: i32) -> Self {
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
    pub fn round(self) -> Coord {
        Coord {
            row: self.row.round(),
            column: self.column.round(),
        }
    }
    pub fn to_vec3(&self, y: f32) -> Vec3 {
        vec3(self.column, y, self.row)
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
        Coord::new_f(value.z, value.x)
    }
}
impl From<Coord> for Vec3 {
    fn from(value: Coord) -> Self {
        value.to_vec3(0.0)
    }
}
impl Add<Coord> for Coord {
    type Output = Coord;
    fn add(mut self, other: Coord) -> Self::Output {
        self += other;
        self
    }
}
impl AddAssign<Coord> for Coord {
    fn add_assign(&mut self, other: Coord) {
        self.column += other.column;
        self.row += other.row;
    }
}
impl Add<f32> for Coord {
    type Output = Coord;
    fn add(mut self, other: f32) -> Self::Output {
        self += other;
        self
    }
}
impl AddAssign<f32> for Coord {
    fn add_assign(&mut self, rhs: f32) {
        self.column += rhs;
        self.row += rhs;
    }
}
impl Sub<Coord> for Coord {
    type Output = Coord;
    fn sub(mut self, other: Coord) -> Self::Output {
        self -= other;
        self
    }
}

impl SubAssign<Coord> for Coord {
    fn sub_assign(&mut self, other: Coord) {
        self.column -= other.column;
        self.row -= other.row;
    }
}
impl Mul<Coord> for Coord {
    type Output = Coord;

    fn mul(mut self, other: Coord) -> Self::Output {
        self *= other;
        self
    }
}
impl Mul<f32> for Coord {
    type Output = Coord;

    fn mul(mut self, other: f32) -> Self::Output {
        self.column *= other;
        self.row *= other;
        self
    }
}
impl MulAssign<Coord> for Coord {
    fn mul_assign(&mut self, other: Coord) {
        self.column *= other.column;
        self.row *= other.row;
    }
}
impl MulAssign<f32> for Coord {
    fn mul_assign(&mut self, other: f32) {
        self.column *= other;
        self.row *= other;
    }
}
