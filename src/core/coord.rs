use macroquad::math::{IVec2, Vec2, Vec3, f32, ivec2, vec2, vec3};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Coord {
    pub column: f32,
    pub row: f32,
}

impl Coord {
    pub const fn new_f(column: f32, row: f32) -> Self {
        Coord { column, row }
    }
    pub const fn new_i(column: ITile, row: ITile) -> Self {
        Coord {
            column: column as f32,
            row: row as f32,
        }
    }
    pub fn row(&self) -> ITile {
        self.row.floor() as ITile
    }
    pub fn column(&self) -> ITile {
        self.column.floor() as ITile
    }
    pub fn row_f(&self) -> f32 {
        self.row
    }
    pub fn column_f(&self) -> f32 {
        self.column
    }
    pub fn abs(self) -> Self {
        Self {
            row: self.row.abs(),
            column: self.column.abs(),
        }
    }
    pub fn floor(self) -> Self {
        Self {
            row: self.row.floor(),
            column: self.column.floor(),
        }
    }
    pub fn round(self) -> Self {
        Self {
            row: self.row.round(),
            column: self.column.round(),
        }
    }
    pub fn normalize(self) -> Self {
        let v: Vec2 = self.into();
        v.normalize().into()
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
impl From<ICoord> for Coord {
    fn from(value: ICoord) -> Self {
        Coord::new_i(value.column, value.row)
    }
}
impl From<IVec2> for Coord {
    fn from(value: IVec2) -> Self {
        Coord::new_i(value.x as ITile, value.y as ITile)
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
impl Neg for Coord {
    type Output = Coord;

    fn neg(self) -> Self::Output {
        self * -1.0
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

/// sad that the format spec doesn't support variable alignment like `{:*.*}, 7, 4, -0.0001`
pub fn fmt_vec2(v: Vec2) -> String {
    format!("[{:7.4}, {:7.4}]", v.x, v.y)
}

pub type ITile = i16;
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct ICoord {
    pub column: ITile,
    pub row: ITile,
}
impl ICoord {
    pub const fn new_f(column: f32, row: f32) -> Self {
        Self {
            column: column as ITile,
            row: row as ITile,
        }
    }
    pub const fn new_i(column: ITile, row: ITile) -> Self {
        Self { column, row }
    }
    pub fn row(&self) -> ITile {
        self.row
    }
    pub fn column(&self) -> ITile {
        self.column
    }
    pub fn row_f(&self) -> f32 {
        self.row as f32
    }
    pub fn column_f(&self) -> f32 {
        self.column as f32
    }
    pub fn abs(self) -> Self {
        Self {
            row: self.row.abs(),
            column: self.column.abs(),
        }
    }
    pub fn normalize(self) -> Self {
        let v: Vec2 = self.into();
        v.normalize().into()
    }
    pub fn length_squared(self) -> ITile {
        self.row * self.row + self.column * self.column
    }
    pub fn to_vec3(&self, y: f32) -> Vec3 {
        vec3(self.column_f(), y, self.row_f())
    }
    pub fn into<T: From<ICoord>>(self) -> T {
        Into::<T>::into(self)
    }
}

impl From<Coord> for ICoord {
    fn from(value: Coord) -> Self {
        ICoord::new_f(value.column, value.row)
    }
}
impl From<IVec2> for ICoord {
    fn from(value: IVec2) -> Self {
        ICoord::new_i(value.x as ITile, value.y as ITile)
    }
}
impl From<ICoord> for IVec2 {
    fn from(value: ICoord) -> Self {
        ivec2(value.column as i32, value.row as i32)
    }
}
impl From<Vec2> for ICoord {
    fn from(value: Vec2) -> Self {
        ICoord::new_f(value.x, value.y)
    }
}
impl From<ICoord> for Vec2 {
    fn from(value: ICoord) -> Self {
        vec2(value.column_f(), value.row_f())
    }
}
impl From<Vec3> for ICoord {
    fn from(value: Vec3) -> Self {
        ICoord::new_f(value.z, value.x)
    }
}
impl From<ICoord> for Vec3 {
    fn from(value: ICoord) -> Self {
        value.to_vec3(0.0)
    }
}
impl Add<ICoord> for ICoord {
    type Output = ICoord;
    fn add(mut self, other: ICoord) -> Self::Output {
        self += other;
        self
    }
}
impl AddAssign<ICoord> for ICoord {
    fn add_assign(&mut self, other: ICoord) {
        self.column += other.column;
        self.row += other.row;
    }
}
impl Add<ITile> for ICoord {
    type Output = ICoord;
    fn add(mut self, other: ITile) -> Self::Output {
        self += other;
        self
    }
}
impl AddAssign<ITile> for ICoord {
    fn add_assign(&mut self, rhs: ITile) {
        self.column += rhs;
        self.row += rhs;
    }
}
impl Neg for ICoord {
    type Output = ICoord;

    fn neg(self) -> Self::Output {
        self * -1
    }
}
impl Sub<ICoord> for ICoord {
    type Output = ICoord;
    fn sub(mut self, other: ICoord) -> Self::Output {
        self -= other;
        self
    }
}

impl SubAssign<ICoord> for ICoord {
    fn sub_assign(&mut self, other: ICoord) {
        self.column -= other.column;
        self.row -= other.row;
    }
}
impl Mul<ICoord> for ICoord {
    type Output = ICoord;

    fn mul(mut self, other: ICoord) -> Self::Output {
        self *= other;
        self
    }
}
impl Mul<ITile> for ICoord {
    type Output = ICoord;

    fn mul(mut self, other: ITile) -> Self::Output {
        self.column *= other;
        self.row *= other;
        self
    }
}
impl MulAssign<ICoord> for ICoord {
    fn mul_assign(&mut self, other: ICoord) {
        self.column *= other.column;
        self.row *= other.row;
    }
}
impl MulAssign<ITile> for ICoord {
    fn mul_assign(&mut self, other: ITile) {
        self.column *= other;
        self.row *= other;
    }
}
impl Div<ICoord> for ICoord {
    type Output = ICoord;

    fn div(mut self, other: ICoord) -> Self::Output {
        self /= other;
        self
    }
}
impl Div<ITile> for ICoord {
    type Output = ICoord;

    fn div(mut self, other: ITile) -> Self::Output {
        self.column /= other;
        self.row /= other;
        self
    }
}
impl DivAssign<ICoord> for ICoord {
    fn div_assign(&mut self, other: ICoord) {
        self.column /= other.column;
        self.row /= other.row;
    }
}
impl DivAssign<ITile> for ICoord {
    fn div_assign(&mut self, other: ITile) {
        self.column /= other;
        self.row /= other;
    }
}
