use std::fmt::{Display, Formatter};
use std::iter::Zip;

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

#[derive(Clone)]
pub struct OneForEachTeam<T> {
    white: T,
    black: T,
}

impl<T> OneForEachTeam<T> {
    pub const fn new(white: T, black: T) -> Self {
        Self { white, black }
    }
    pub fn new_from_factory(factory: fn(Team) -> T) -> Self {
        Self {
            white: factory(Team::White),
            black: factory(Team::Black),
        }
    }
    pub fn get(&self, team: Team) -> &T {
        if team.is_white() {
            &self.white
        } else {
            &self.black
        }
    }
    pub fn get_mut(&mut self, team: Team) -> &mut T {
        if team.is_white() {
            &mut self.white
        } else {
            &mut self.black
        }
    }
    pub fn take(self) -> [T; 2] {
        [self.white, self.black]
    }
    pub fn iter(&self) -> impl IntoIterator<Item = &T> {
        [&self.white, &self.black]
    }
    pub fn team_iter(&self) -> impl IntoIterator<Item = (Team, &T)> {
        [(Team::White, &self.white), (Team::Black, &self.black)]
    }
    pub fn iter_mut(&mut self) -> impl IntoIterator<Item = &mut T> {
        [&mut self.white, &mut self.black]
    }
    pub fn team_iter_mut(&mut self) -> impl IntoIterator<Item = (Team, &mut T)> {
        [
            (Team::White, &mut self.white),
            (Team::Black, &mut self.black),
        ]
    }
}
//
// #[derive(Clone)]
// pub struct OneForEachTeam<T> {
//     white_black: [T; 2],
// }
//
// impl<T> OneForEachTeam<T> {
//     pub const fn new(white: T, black: T) -> Self {
//         Self {
//             white_black: [white, black],
//         }
//     }
//     pub fn new_from_factory(factory: fn(Team) -> T) -> Self {
//         Self {
//             white_black: [factory(Team::White), factory(Team::Black)],
//         }
//     }
//     pub fn get(&self, team: Team) -> &T {
//         &self.white_black[team.index()]
//     }
//     pub fn get_mut(&mut self, team: Team) -> &mut T {
//         &mut self.white_black[team.index()]
//     }
//     pub fn take(self) -> [T; 2] {
//         self.white_black
//     }
//     pub fn iter(&self) -> impl IntoIterator<Item = &T> {
//         self.white_black.as_slice()
//     }
//     pub fn team_iter(&self) -> impl IntoIterator<Item = (Team, &T)> {
//         [
//             (Team::White, &self.white_black[0]),
//             (Team::Black, &self.white_black[1]),
//         ]
//     }
//     pub fn iter_mut(&mut self) -> impl IntoIterator<Item = &mut T> {
//         self.white_black.as_mut()
//     }
//     pub fn team_iter_mut(&mut self) -> impl IntoIterator<Item = (Team, &mut T)> {
//         [Team::White, Team::Black].into_iter().zip(self.iter_mut())
//     }
// }
