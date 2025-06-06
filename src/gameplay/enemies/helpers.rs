use brtk::prelude::Direction;

use crate::core::components::Position;

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
