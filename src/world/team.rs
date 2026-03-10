use std::fmt::{Display, Formatter};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, PartialOrd)]
pub enum Team {
    White,
    Black,
}
impl Team {
    pub fn is_white(&self) -> bool {
        *self == Team::White
    }
    pub fn toggle(&mut self) {
        *self = self.opposite();
    }
    pub fn opposite(&self) -> Team {
        match self {
            Team::White => Team::Black,
            Team::Black => Team::White,
        }
    }
}
impl Display for Team {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Team::White => {
                write!(f, "WHITE")
            }
            Team::Black => {
                write!(f, "BLACK")
            }
        }
    }
}
