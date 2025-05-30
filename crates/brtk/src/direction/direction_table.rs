use crate::direction::Direction;

#[rustfmt::skip]
pub const DIRECTION_TABLE: &[Direction; 26] = &[
    Direction::NORTH,           // 0
    Direction::EAST,            // 1
    Direction::SOUTH,           // 2
    Direction::WEST,            // 3
    Direction::NORTH_EAST,      // 4
    Direction::SOUTH_EAST,      // 5
    Direction::SOUTH_WEST,      // 6
    Direction::NORTH_WEST,      // 7
    Direction::UP,              // 8
    Direction::DOWN,            // 9
    Direction::UP_NORTH,        // 10
    Direction::UP_NORTH_EAST,   // 11
    Direction::UP_EAST,         // 12
    Direction::UP_SOUTH_EAST,   // 13
    Direction::UP_SOUTH,        // 14
    Direction::UP_SOUTH_WEST,   // 15
    Direction::UP_WEST,         // 16
    Direction::UP_NORTH_WEST,   // 17
    Direction::DOWN_NORTH,      // 18
    Direction::DOWN_NORTH_EAST, // 19
    Direction::DOWN_EAST,       // 20
    Direction::DOWN_SOUTH_EAST, // 21
    Direction::DOWN_SOUTH,      // 22
    Direction::DOWN_SOUTH_WEST, // 23
    Direction::DOWN_WEST,       // 24
    Direction::DOWN_NORTH_WEST, // 25
];
