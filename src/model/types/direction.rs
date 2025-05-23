use std::ops::Add;

use bevy::prelude::*;

use crate::model::components::Position;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MoveDirection {
    North,
    South,
    East,
    West,
}

impl MoveDirection {
    pub const ALL_DIRECTIONS: [MoveDirection; 4] =
        [MoveDirection::North, MoveDirection::South, MoveDirection::East, MoveDirection::West];

    pub fn delta(&self) -> (i32, i32) {
        match self {
            MoveDirection::North => (0, 1),
            MoveDirection::South => (0, -1),
            MoveDirection::East => (1, 0),
            MoveDirection::West => (-1, 0),
        }
    }

    pub fn all_directions() -> [MoveDirection; 4] { MoveDirection::ALL_DIRECTIONS }

    pub fn random_direction() -> MoveDirection {
        let mut rng = fastrand::Rng::new();
        MoveDirection::ALL_DIRECTIONS[rng.usize(0..MoveDirection::ALL_DIRECTIONS.len())]
    }
}

impl Add<Position> for MoveDirection {
    type Output = Position;

    fn add(self, rhs: Position) -> Self::Output {
        let (dx, dy) = self.delta();
        Position(IVec2::new(rhs.0.x + dx, rhs.0.y + dy))
    }
}

impl Add<MoveDirection> for Position {
    type Output = Position;

    fn add(self, rhs: MoveDirection) -> Self::Output { rhs + self }
}

impl From<MoveDirection> for IVec2 {
    fn from(direction: MoveDirection) -> Self {
        match direction {
            MoveDirection::North => IVec2::new(0, 1),
            MoveDirection::South => IVec2::new(0, -1),
            MoveDirection::East => IVec2::new(1, 0),
            MoveDirection::West => IVec2::new(-1, 0),
        }
    }
}
