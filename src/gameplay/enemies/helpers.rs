use bevy::prelude::*;
use brtk::prelude::Direction;

use crate::core::{components::Position, resources::CurrentMap};

/// Calculate the direction to move toward a target
pub fn calculate_direction_to_target(from: Position, to: Position) -> Option<Direction> {
    let diff = to.0 - from.0;

    if diff.x == 0 && diff.y == 0 {
        return None; // Already at target
    }

    // Prioritize the axis with the larger difference
    if diff.x.abs() > diff.y.abs() {
        if diff.x > 0 { Some(Direction::EAST) } else { Some(Direction::WEST) }
    } else if diff.y > 0 {
        Some(Direction::SOUTH)
    } else {
        Some(Direction::NORTH)
    }
}

/// Calculate the direction to move away from a target
pub fn calculate_direction_away_from_target(from: Position, away_from: Position) -> Option<Direction> {
    let diff = from.0 - away_from.0;

    if diff.x == 0 && diff.y == 0 {
        // At same position, pick a random direction
        let directions: Vec<Direction> = Direction::iter_cardinal().collect();
        return Some(directions[fastrand::usize(..directions.len())]);
    }

    // Prioritize the axis with the larger difference
    if diff.x.abs() > diff.y.abs() {
        if diff.x > 0 { Some(Direction::EAST) } else { Some(Direction::WEST) }
    } else if diff.y > 0 {
        Some(Direction::SOUTH)
    } else {
        Some(Direction::NORTH)
    }
}

/// Find an alternative direction when the direct path is blocked
pub fn find_alternative_direction(from: Position, to: Position, map: &CurrentMap) -> Option<Direction> {
    let directions: Vec<Direction> = Direction::iter_cardinal().collect();

    // Try all directions and pick the one that gets us closest to the target
    let mut best_direction = None;
    let mut best_distance = f32::INFINITY;

    for &dir in &directions {
        let (dx, dy) = dir.coord();
        let test_pos = from + (dx, dy);
        if map.is_walkable(test_pos) {
            let distance = crate::utils::calculate_distance(test_pos, to);
            if distance < best_distance {
                best_distance = distance;
                best_direction = Some(dir);
            }
        }
    }

    best_direction
}

/// Find an alternative direction for fleeing when the direct path is blocked
pub fn find_alternative_flee_direction(
    from: Position,
    away_from: Position,
    map: &CurrentMap,
) -> Option<Direction> {
    let directions: Vec<Direction> = Direction::iter_cardinal().collect();

    // Try all directions and pick the one that gets us furthest from the target
    let mut best_direction = None;
    let mut best_distance = 0.0;

    for &dir in &directions {
        let (dx, dy) = dir.coord();
        let test_pos = from + (dx, dy);
        if map.is_walkable(test_pos) {
            let distance = crate::utils::calculate_distance(test_pos, away_from);
            if distance > best_distance {
                best_distance = distance;
                best_direction = Some(dir);
            }
        }
    }

    best_direction
}

/// Find a random walkable direction
pub fn find_random_walkable_direction(from: Position, map: &CurrentMap) -> Option<Direction> {
    let directions: Vec<Direction> = Direction::iter_cardinal().collect();
    let mut walkable_directions = Vec::new();

    for &dir in &directions {
        let (dx, dy) = dir.coord();
        let test_pos = from + (dx, dy);
        if map.is_walkable(test_pos) {
            walkable_directions.push(dir);
        }
    }

    if walkable_directions.is_empty() {
        None
    } else {
        Some(walkable_directions[fastrand::usize(..walkable_directions.len())])
    }
}
