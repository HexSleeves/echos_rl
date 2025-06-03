//! Pathfinding integration for the game
//!
//! This module provides PathProvider implementations for our game's map systems
//! and utility functions for integrating pathfinding with our AI and movement systems.

use crate::core::{components::Position, resources::CurrentMap};
use brtk::pathfinding::PathProvider;

/// PathProvider implementation for CurrentMap
impl PathProvider for CurrentMap {
    fn is_walkable(&mut self, position: (i32, i32), _movement_type: u8) -> bool {
        let pos = Position::new(position.0, position.1);

        // Check bounds first
        if !self.in_bounds(pos) {
            return false;
        }

        // Check if terrain is walkable and no actor is present
        // Note: using the Map's is_walkable method, not the trait method
        (**self).is_walkable(pos) && self.get_actor(pos).is_none()
    }

    fn cost(&mut self, position: (i32, i32), _movement_type: u8) -> u32 {
        let pos = Position::new(position.0, position.1);

        // Basic cost system - can be extended later for different terrain types
        if !self.in_bounds(pos) {
            return u32::MAX; // Impassable
        }

        // For now, all walkable terrain has the same cost
        // Future: different terrain types could have different costs
        if (**self).is_walkable(pos) {
            1 // Base movement cost
        } else {
            u32::MAX // Impassable
        }
    }
}

/// Utility functions for pathfinding integration
pub mod utils {
    use super::*;
    use brtk::pathfinding::PathFinder;

    /// Find a path from origin to destination using A* pathfinding
    pub fn find_path(
        origin: Position,
        destination: Position,
        map: &mut CurrentMap,
        allow_partial: bool,
    ) -> Option<Vec<Position>> {
        let path_coords = PathFinder::AStar.compute(
            (origin.x(), origin.y()),
            (destination.x(), destination.y()),
            0, // movement_type - can be extended for different entity types
            allow_partial,
            map,
        )?;

        // Convert coordinates back to Position
        Some(path_coords.into_iter().map(|(x, y)| Position::new(x, y)).collect())
    }

    /// Find a path and return the next step
    pub fn find_next_step(origin: Position, destination: Position, map: &mut CurrentMap) -> Option<Position> {
        find_path(origin, destination, map, false)?.into_iter().next()
    }

    /// Find the best position to move towards a target (allows partial paths)
    pub fn find_best_move_towards(
        origin: Position,
        destination: Position,
        map: &mut CurrentMap,
    ) -> Option<Position> {
        // Try full path first
        if let Some(next_step) = find_next_step(origin, destination, map) {
            return Some(next_step);
        }

        // Fall back to partial path
        find_path(origin, destination, map, true)?.into_iter().next()
    }

    /// Check if there's a clear path between two positions
    pub fn has_clear_path(origin: Position, destination: Position, map: &mut CurrentMap) -> bool {
        find_path(origin, destination, map, false).is_some()
    }

    /// Find an escape route away from a threat position
    pub fn find_escape_route(
        origin: Position,
        threat_position: Position,
        map: &mut CurrentMap,
        escape_distance: u32,
    ) -> Option<Position> {
        use brtk::direction::Direction;

        // Try to find a position that's further from the threat
        let mut best_position = None;
        let mut best_distance = origin.distance(&threat_position);

        // Check positions in expanding circles
        for radius in 1..=escape_distance {
            for direction in Direction::iter_cardinal_ordinal() {
                let coord = direction.coord();
                let test_pos = origin + (coord.0 * radius as i32, coord.1 * radius as i32);

                if (**map).is_walkable(test_pos) && map.get_actor(test_pos).is_none() {
                    let distance_from_threat = test_pos.distance(&threat_position);

                    if distance_from_threat > best_distance {
                        // Check if we can actually reach this position
                        if has_clear_path(origin, test_pos, map) {
                            best_position = Some(test_pos);
                            best_distance = distance_from_threat;
                        }
                    }
                }
            }

            // If we found a good escape position, return the next step towards it
            if let Some(escape_pos) = best_position {
                return find_next_step(origin, escape_pos, map);
            }
        }

        None
    }
}
