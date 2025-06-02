use bevy::prelude::*;
use brtk::prelude::Direction;
use leafwing_input_manager::prelude::*;

// This is the list of "things in the game I want to be able to do based on input"
#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum PlayerAction {
    // Movement
    NorthWest,
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,

    Wait,
}

impl PlayerAction {
    // Lists like this can be very useful for quickly matching subsets of actions
    pub const DIRECTIONS: [Self; 8] = [
        Self::NorthWest,
        Self::North,
        Self::NorthEast,
        Self::East,
        Self::SouthEast,
        Self::South,
        Self::SouthWest,
        Self::West,
    ];

    pub const fn direction(self) -> Option<Direction> {
        match self {
            Self::NorthWest => Some(Direction::NORTH_WEST),
            Self::North => Some(Direction::NORTH),
            Self::NorthEast => Some(Direction::NORTH_EAST),
            Self::East => Some(Direction::EAST),
            Self::SouthEast => Some(Direction::SOUTH_EAST),
            Self::South => Some(Direction::SOUTH),
            Self::SouthWest => Some(Direction::SOUTH_WEST),
            Self::West => Some(Direction::WEST),
            _ => None,
        }
    }
}
