use std::ops::{Add, AddAssign};

use bevy::prelude::*;

#[derive(Component, Reflect, Default, Debug, Clone, Copy, PartialEq, Eq, Hash, Deref, DerefMut)]
#[reflect(Component)]
pub struct Position(pub IVec2);

impl Position {
    pub fn new(x: i32, y: i32) -> Self { Self(IVec2::new(x, y)) }

    pub fn x(&self) -> i32 { self.0.x }

    pub fn y(&self) -> i32 { self.0.y }
}

impl From<IVec2> for Position {
    fn from(vec: IVec2) -> Self { Self(vec) }
}

impl From<Position> for (i32, i32) {
    fn from(value: Position) -> Self { (value.0.x, value.0.y) }
}

impl Add<Position> for Position {
    type Output = Self;

    fn add(self, rhs: Position) -> Self::Output { Self(self.0 + rhs.0) }
}

impl AddAssign<Position> for Position {
    fn add_assign(&mut self, rhs: Position) { self.0 += rhs.0; }
}

impl Add<IVec2> for Position {
    type Output = Self;

    fn add(self, rhs: IVec2) -> Self::Output { Self(self.0 + rhs) }
}

impl AddAssign<IVec2> for Position {
    fn add_assign(&mut self, rhs: IVec2) { self.0 += rhs; }
}

impl Add<(i32, i32)> for Position {
    type Output = Self;

    fn add(self, rhs: (i32, i32)) -> Self::Output { Self(IVec2::new(self.0.x + rhs.0, self.0.y + rhs.1)) }
}

impl AddAssign<(i32, i32)> for Position {
    fn add_assign(&mut self, rhs: (i32, i32)) {
        self.0.x += rhs.0;
        self.0.y += rhs.1;
    }
}

// impl Add<Direction> for Position {
//     type Output = Self;

//     fn add(self, rhs: Direction) -> Self::Output {
//         let (x, y) = rhs.coord();
//         Self(IVec2::new(self.0.x + x, self.0.y + y))
//     }
// }

// impl AddAssign<Direction> for Position {
//     fn add_assign(&mut self, rhs: Direction) {
//         let (x, y) = rhs.coord();
//         self.0.x += x;
//         self.0.y += y;
//     }
// }
