use bevy::prelude::*;
use brtk::prelude::Direction;

use crate::core::{components::Position, resources::CurrentMap};

pub mod macros;
pub use macros::*;

mod tilemap;
pub use tilemap::*;

/// Compute the greatest common divisor using Euclidean algorithm
pub fn gcd(a: i32, b: i32) -> i32 { if b == 0 { a } else { gcd(b, a % b) } }

/// Calculate distance between two positions
pub fn calculate_distance(pos1: Position, pos2: Position) -> f32 {
    let dx = (pos2.x() - pos1.x()) as f32;
    let dy = (pos2.y() - pos1.y()) as f32;
    (dx * dx + dy * dy).sqrt()
}

/// Calculate direction from one position to another
pub fn calculate_direction_to_target(from: Position, to: Position) -> Option<Direction> {
    let dx = to.x() - from.x();
    let dy = to.y() - from.y();

    // Convert to direction
    // match (dx.signum(), dy.signum()) {
    //     (0, -1) => Some(Direction::NORTH),
    //     (1, -1) => Some(Direction::NORTH_EAST),
    //     (1, 0) => Some(Direction::EAST),
    //     (1, 1) => Some(Direction::SOUTH_EAST),
    //     (0, 1) => Some(Direction::SOUTH),
    //     (-1, 1) => Some(Direction::SOUTH_WEST),
    //     (-1, 0) => Some(Direction::WEST),
    //     (-1, -1) => Some(Direction::NORTH_WEST),
    //     _ => None, // Same position
    // }

    Some(Direction::from_coord((dx, dy)))
}

/// Find an alternative direction when direct path is blocked
pub fn find_alternative_direction(from: Position, to: Position, map: &CurrentMap) -> Option<Direction> {
    let primary_dir = calculate_direction_to_target(from, to)?;

    // Try directions adjacent to the primary direction
    let adjacent_dirs = get_adjacent_directions(primary_dir);

    for dir in adjacent_dirs {
        let new_pos = from + dir.coord();
        if map.is_walkable(new_pos) && map.get_actor(new_pos).is_none() {
            return Some(dir);
        }
    }

    None
}

/// Get directions adjacent to a given direction
pub fn get_adjacent_directions(dir: Direction) -> [Direction; 2] {
    match dir {
        d if d == Direction::NORTH => [Direction::NORTH_WEST, Direction::NORTH_EAST],
        d if d == Direction::NORTH_EAST => [Direction::NORTH, Direction::EAST],
        d if d == Direction::EAST => [Direction::NORTH_EAST, Direction::SOUTH_EAST],
        d if d == Direction::SOUTH_EAST => [Direction::EAST, Direction::SOUTH],
        d if d == Direction::SOUTH => [Direction::SOUTH_EAST, Direction::SOUTH_WEST],
        d if d == Direction::SOUTH_WEST => [Direction::SOUTH, Direction::WEST],
        d if d == Direction::WEST => [Direction::SOUTH_WEST, Direction::NORTH_WEST],
        d if d == Direction::NORTH_WEST => [Direction::WEST, Direction::NORTH],
        _ => [Direction::NORTH, Direction::EAST], // Default fallback
    }
}

/// Find a random walkable direction from a position
pub fn find_random_walkable_direction(from: Position, map: &CurrentMap) -> Option<Direction> {
    let mut directions = Direction::iter_cardinal_ordinal().collect::<Vec<_>>();

    // Shuffle directions for randomness
    for i in (1..directions.len()).rev() {
        let j = fastrand::usize(0..=i);
        directions.swap(i, j);
    }

    for dir in directions {
        let new_pos = from + dir.coord();
        if map.is_walkable(new_pos) && map.get_actor(new_pos).is_none() {
            return Some(dir);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd_function() {
        assert_eq!(gcd(12, 8), 4);
        assert_eq!(gcd(17, 13), 1);
        assert_eq!(gcd(0, 5), 5);
        assert_eq!(gcd(7, 0), 7);
    }
}
