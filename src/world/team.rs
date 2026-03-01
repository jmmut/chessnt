#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Team {
    White,
    Black,
}
impl Team {
    pub fn is_white(&self) -> bool {
        *self == Team::White
    }
    pub fn toggle(&mut self) {
        match self {
            Team::White => *self = Team::Black,
            Team::Black => *self = Team::White,
        }
    }
}
