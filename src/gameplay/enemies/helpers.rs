use brtk::prelude::Direction;
use fastrand::Rng;

use crate::core::components::Position;

/// Calculate direction from one position to another using simple vector math
pub fn calculate_direction_to_target(from: &Position, to: &Position) -> Option<Direction> {
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

/// Calculate the best tactical direction to target, with intelligent diagonal movement
/// This function implements actual tactical logic unlike the basic direction calculation
pub fn calculate_tactical_direction_to_target(from: &Position, to: &Position) -> Option<Direction> {
    let dx = to.x() - from.x();
    let dy = to.y() - from.y();

    // If we're already at the target, no direction needed
    if dx == 0 && dy == 0 {
        return None;
    }

    let dir_x = dx.signum();
    let dir_y = dy.signum();

    // If we can move diagonally (both dx and dy are non-zero), evaluate if it's tactical
    if dir_x != 0 && dir_y != 0 {
        let diagonal_dir = Direction::from_coord((dir_x, dir_y));
        let cardinal_x = Direction::from_coord((dir_x, 0));
        let cardinal_y = Direction::from_coord((0, dir_y));

        // Calculate tactical scores for each option
        let diagonal_pos = *from + diagonal_dir.coord();
        let cardinal_x_pos = *from + cardinal_x.coord();
        let cardinal_y_pos = *from + cardinal_y.coord();

        let diagonal_score = evaluate_tactical_position(&diagonal_pos, to);
        let cardinal_x_score = evaluate_tactical_position(&cardinal_x_pos, to);
        let cardinal_y_score = evaluate_tactical_position(&cardinal_y_pos, to);

        // Prefer diagonal if it's significantly better, or randomly if close
        if diagonal_score > cardinal_x_score.max(cardinal_y_score) + 0.5 {
            Some(diagonal_dir)
        } else if diagonal_score >= cardinal_x_score.max(cardinal_y_score) - 0.5 {
            // Close scores - add some randomness for unpredictability
            let mut rng = Rng::new();
            if rng.f32() < 0.6 {
                // 60% chance to prefer diagonal for tactical advantage
                Some(diagonal_dir)
            } else if cardinal_x_score > cardinal_y_score {
                Some(cardinal_x)
            } else {
                Some(cardinal_y)
            }
        } else {
            // Cardinal is clearly better
            if cardinal_x_score > cardinal_y_score { Some(cardinal_x) } else { Some(cardinal_y) }
        }
    } else {
        // Only one direction possible (cardinal movement)
        Some(Direction::from_coord((dir_x, dir_y)))
    }
}

/// Calculate direction away from a target position
pub fn calculate_direction_away_from_target(from: &Position, away_from: &Position) -> Option<Direction> {
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

/// Check if an attacker is in attack range of a victim
pub fn in_attack_range(attacker_pos: &Position, victim_pos: &Position) -> bool {
    attacker_pos.distance(victim_pos) <= 1.0
}

/// Evaluate the tactical value of a position relative to a target
/// Higher scores indicate better tactical positions
fn evaluate_tactical_position(pos: &Position, target: &Position) -> f32 {
    let distance = pos.distance(target);

    // Base score - closer is generally better, but not too close
    let distance_score = if distance <= 1.0 {
        10.0 // In attack range
    } else if distance <= 2.0 {
        8.0 // One move away from attack range
    } else {
        5.0 - distance.min(5.0) // Diminishing returns for distance
    };

    // Bonus for diagonal positions (more tactical flexibility)
    let diagonal_bonus = if is_diagonal_adjacent(pos, target) { 2.0 } else { 0.0 };

    distance_score + diagonal_bonus
}

/// Check if a position is diagonally adjacent to another
pub fn is_diagonal_adjacent(pos1: &Position, pos2: &Position) -> bool {
    let dx = (pos1.x() - pos2.x()).abs();
    let dy = (pos1.y() - pos2.y()).abs();

    // Diagonal adjacent means both dx and dy are 1
    dx == 1 && dy == 1
}
