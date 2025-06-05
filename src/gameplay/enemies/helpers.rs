use bevy::prelude::*;
use brtk::prelude::Direction;

use crate::core::{components::Position, pathfinding, resources::CurrentMap};

/// Calculate direction from one position to another using simple vector math
pub fn calculate_direction_to_target(from: Position, to: Position) -> Option<Direction> {
    let dx = to.x() - from.x();
    let dy = to.y() - from.y();

    // If we're already at the target, no direction needed
    if dx == 0 && dy == 0 {
        return None;
    }

    // Convert to unit direction (normalize to -1, 0, or 1)
    let dir_x = dx.signum();
    let dir_y = dy.signum();

    Some(Direction::from_coord((dir_x, dir_y)))
}

/// Calculate direction away from a target position
pub fn calculate_direction_away_from_target(from: Position, away_from: Position) -> Option<Direction> {
    let dx = from.x() - away_from.x();
    let dy = from.y() - away_from.y();

    // If we're at the same position, pick a random direction
    if dx == 0 && dy == 0 {
        return Some(Direction::NORTH); // Default escape direction
    }

    // Convert to unit direction (normalize to -1, 0, or 1)
    let dir_x = dx.signum();
    let dir_y = dy.signum();

    Some(Direction::from_coord((dir_x, dir_y)))
}

/// Enhanced direction calculation using A* pathfinding when map is available
pub fn calculate_direction_to_target_with_pathfinding(
    from: Position,
    to: Position,
    map: &mut CurrentMap,
) -> Option<Direction> {
    // Try A* pathfinding first
    if let Some(next_pos) = pathfinding::utils::find_next_step(from, to, map) {
        let dx = next_pos.x() - from.x();
        let dy = next_pos.y() - from.y();
        Some(Direction::from_coord((dx, dy)))
    } else {
        // Fallback to simple direction calculation
        calculate_direction_to_target(from, to)
    }
}

/// Enhanced flee direction calculation using escape route pathfinding
pub fn calculate_direction_away_from_target_with_pathfinding(
    from: Position,
    away_from: Position,
    map: &mut CurrentMap,
    escape_distance: u32,
) -> Option<Direction> {
    // Try to find an intelligent escape route
    if let Some(escape_pos) = pathfinding::utils::find_escape_route(from, away_from, map, escape_distance) {
        let dx = escape_pos.x() - from.x();
        let dy = escape_pos.y() - from.y();
        Some(Direction::from_coord((dx, dy)))
    } else {
        // Fallback to simple direction calculation
        calculate_direction_away_from_target(from, away_from)
    }
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
            let distance = test_pos.pathfinding_distance(&away_from);
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
