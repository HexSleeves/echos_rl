//! Game pathfinding integration
//!
//! PathProvider implementation for game maps and utility functions for AI movement.

use crate::core::{components::Position, resources::CurrentMap};
use brtk::pathfinding::{PathCacheConfig, PathProvider, PathfindingManager};
use once_cell::sync::Lazy;
use parking_lot::RwLock;
use std::time::Duration;

/// Global pathfinding manager for the game
static PATHFINDING_MANAGER: Lazy<RwLock<PathfindingManager>> = Lazy::new(|| {
    // Initialize the global pathfinding manager
    let cache_config = PathCacheConfig {
        max_entries: 2000,                // Larger cache for game use
        max_age: Duration::from_secs(60), // Paths valid for 1 minute
        enable_stats: true,
    };
    RwLock::new(PathfindingManager::new(brtk::pathfinding::PathFinder::AStar, cache_config))
});

/// Get a reference to the global pathfinding manager for read operations
fn with_pathfinding_manager_read<F, R>(f: F) -> Option<R>
where
    F: FnOnce(&PathfindingManager) -> R,
{
    match PATHFINDING_MANAGER.try_read() {
        Some(guard) => Some(f(&guard)),
        None => {
            log::warn!("Failed to acquire read lock on pathfinding manager, operation skipped");
            None
        }
    }
}

/// Get a reference to the global pathfinding manager for write operations
fn with_pathfinding_manager_write<F, R>(f: F) -> Option<R>
where
    F: FnOnce(&mut PathfindingManager) -> R,
{
    match PATHFINDING_MANAGER.try_write() {
        Some(mut guard) => Some(f(&mut guard)),
        None => {
            log::warn!("Failed to acquire write lock on pathfinding manager, operation skipped");
            None
        }
    }
}

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

    /// Find a path from origin to destination using A* pathfinding with caching
    pub fn find_path(
        origin: Position,
        destination: Position,
        map: &mut CurrentMap,
        allow_partial: bool,
    ) -> Option<Vec<Position>> {
        with_pathfinding_manager_write(|manager| {
            let path_coords = manager.find_path(
                (origin.x(), origin.y()),
                (destination.x(), destination.y()),
                0, // movement_type - can be extended for different entity types
                allow_partial,
                map,
            )?;

            // Convert coordinates back to Position
            Some(path_coords.into_iter().map(|(x, y)| Position::new(x, y)).collect())
        })
        .flatten()
        .or_else(|| {
            log::warn!("Pathfinding failed, trying uncached path");
            find_path_uncached(origin, destination, map, allow_partial)
        })
    }

    /// Find a path without caching (for one-off calculations)
    pub fn find_path_uncached(
        origin: Position,
        destination: Position,
        map: &mut CurrentMap,
        allow_partial: bool,
    ) -> Option<Vec<Position>> {
        let path_coords = brtk::pathfinding::PathFinder::AStar.compute(
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
        find_path(origin, destination, map, false)?.into_iter().nth(1)
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
        find_path(origin, destination, map, true)?.into_iter().nth(1)
    }

    /// Check if there's a clear path between two positions
    pub fn has_clear_path(origin: Position, destination: Position, map: &mut CurrentMap) -> bool {
        find_path(origin, destination, map, false).is_some()
    }

    /// Find an escape route away from a threat position with improved algorithm
    pub fn find_escape_route(
        origin: Position,
        threat_position: Position,
        map: &mut CurrentMap,
        escape_distance: u32,
    ) -> Option<Position> {
        // Calculate the direction away from the threat
        let threat_vector = (origin.x() - threat_position.x(), origin.y() - threat_position.y());

        // Normalize the escape direction
        let distance =
            (((threat_vector.0 as i64).pow(2) + (threat_vector.1 as i64).pow(2)) as f64).sqrt() as f32;
        if distance < 1.0 {
            // If we're at the same position, pick a random direction
            return find_random_escape_position(origin, map, escape_distance);
        }

        let escape_direction = (
            (threat_vector.0 as f32 / distance) * escape_distance as f32,
            (threat_vector.1 as f32 / distance) * escape_distance as f32,
        );

        // Try positions in the escape direction with some variation
        for radius in 1..=escape_distance {
            for angle_offset in [-0.5, 0.0, 0.5, -1.0, 1.0] {
                let angle = escape_direction.1.atan2(escape_direction.0) + angle_offset;
                let test_pos = Position::new(
                    origin.x() + (angle.cos() * radius as f32) as i32,
                    origin.y() + (angle.sin() * radius as f32) as i32,
                );

                // Check basic walkability first
                let is_walkable = (**map).is_walkable(test_pos);
                let has_no_actor = map.get_actor(test_pos).is_none();

                if is_walkable && has_no_actor {
                    let distance_from_threat = test_pos.distance(&threat_position);

                    // Ensure we're moving away from the threat
                    if distance_from_threat > origin.distance(&threat_position) {
                        // Check if we can actually reach this position
                        if let Some(next_step) = find_next_step(origin, test_pos, map) {
                            return Some(next_step);
                        }
                    }
                }
            }
        }

        // Fallback: try any direction that moves away from the threat
        find_random_escape_position(origin, map, escape_distance)
    }

    /// Find a random escape position when directional escape fails
    fn find_random_escape_position(
        origin: Position,
        map: &mut CurrentMap,
        escape_distance: u32,
    ) -> Option<Position> {
        for _ in 0..20 {
            let angle = fastrand::f32() * 2.0 * std::f32::consts::PI;
            let radius = fastrand::u32(1..=escape_distance);
            let test_pos = Position::new(
                origin.x() + (angle.cos() * radius as f32) as i32,
                origin.y() + (angle.sin() * radius as f32) as i32,
            );

            if (**map).is_walkable(test_pos)
                && map.get_actor(test_pos).is_none()
                && let Some(next_step) = find_next_step(origin, test_pos, map)
            {
                return Some(next_step);
            }
        }

        None
    }

    /// Get pathfinding performance statistics
    pub fn get_pathfinding_stats() -> String {
        with_pathfinding_manager_read(|manager| {
            let stats = manager.stats();
            let (cache_size, cache_capacity, hit_rate) = manager.cache_stats();

            format!(
                "Pathfinding Stats:\n\
                 - Total Requests: {}\n\
                 - Cache Hits: {} ({:.1}%)\n\
                 - Cache Misses: {}\n\
                 - Successful Computations: {}\n\
                 - Failed Computations: {}\n\
                 - Average Computation Time: {:.2}ms\n\
                 - Cache Size: {}/{} ({:.1}% full)\n\
                 - Cache Hit Rate: {:.1}%",
                stats.total_requests,
                stats.cache_hits,
                stats.cache_hit_rate(),
                stats.cache_misses,
                stats.successful_computations,
                stats.failed_computations,
                stats.average_computation_time().as_secs_f64() * 1000.0,
                cache_size,
                cache_capacity,
                (cache_size as f32 / cache_capacity as f32) * 100.0,
                hit_rate * 100.0
            )
        })
        .unwrap_or_else(|| "Pathfinding stats unavailable (lock contention)".to_string())
    }

    /// Clear the pathfinding cache (useful for debugging or when map changes significantly)
    pub fn clear_pathfinding_cache() {
        if with_pathfinding_manager_write(|manager| {
            manager.clear_cache();
        })
        .is_none()
        {
            log::warn!("Failed to clear pathfinding cache due to lock contention");
        }
    }

    /// Validate a path to ensure it's still walkable
    pub fn validate_path(path: &[Position], map: &CurrentMap) -> bool {
        for &pos in path {
            if !(**map).is_walkable(pos) || map.get_actor(pos).is_some() {
                return false;
            }
        }
        true
    }

    /// Get the next valid step from a path, skipping any blocked positions
    pub fn get_next_valid_step(
        path: &[Position],
        current_index: usize,
        map: &CurrentMap,
    ) -> Option<(Position, usize)> {
        for (i, &pos) in path.iter().enumerate().skip(current_index + 1) {
            if (**map).is_walkable(pos) && map.get_actor(pos).is_none() {
                return Some((pos, i));
            }
        }
        None
    }

    /// Calculate path quality score (lower is better)
    pub fn calculate_path_quality(path: &[Position]) -> f32 {
        if path.len() < 2 {
            return 0.0;
        }

        let mut total_cost = 0.0;
        let mut direction_changes = 0;
        let mut last_direction = None;

        for window in path.windows(2) {
            let from = window[0];
            let to = window[1];

            // Calculate movement cost (diagonal moves cost more)
            let dx = (to.x() - from.x()).abs();
            let dy = (to.y() - from.y()).abs();
            let cost = if dx == 1 && dy == 1 {
                1.414 // Diagonal movement
            } else {
                1.0 // Cardinal movement
            };
            total_cost += cost;

            // Count direction changes
            let direction = (to.x() - from.x(), to.y() - from.y());
            if let Some(last_dir) = last_direction
                && last_dir != direction
            {
                direction_changes += 1;
            }
            last_direction = Some(direction);
        }

        // Quality score combines path length and direction changes
        total_cost + (direction_changes as f32 * 0.5)
    }
}
