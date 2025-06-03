//! Pathfinding module for roguelike games
//!
//! This module provides A* and other pathfinding algorithms that integrate
//! with the brtk grid and direction systems. It supports different movement
//! types, partial paths, and cost calculations.

pub mod astar;
pub mod pathing_traits;
pub mod pathing_utils;

// Re-export main types for convenience
pub use astar::{AStar, AStarNode};
pub use pathing_traits::{PathAlgorithm, PathProvider};
pub use pathing_utils::IndexList;

/// Main pathfinding algorithm selector
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PathFinder {
    /// A* pathfinding algorithm
    AStar,
    // Future: Dijkstra, JPS, etc.
}

impl Default for PathFinder {
    fn default() -> Self { Self::AStar }
}

impl PathFinder {
    /// Compute a path using the selected algorithm
    pub fn compute<P: PathProvider>(
        &self,
        origin: (i32, i32),
        destination: (i32, i32),
        movement_type: u8,
        partial_path_on_failure: bool,
        provider: &mut P,
    ) -> Option<Vec<(i32, i32)>> {
        match self {
            Self::AStar => {
                AStar::compute_path(origin, destination, movement_type, partial_path_on_failure, provider)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Simple test map provider for testing
    struct TestMapProvider {
        width: i32,
        height: i32,
        walls: Vec<(i32, i32)>,
    }

    impl TestMapProvider {
        fn new(width: i32, height: i32) -> Self { Self { width, height, walls: Vec::new() } }

        fn add_wall(&mut self, x: i32, y: i32) { self.walls.push((x, y)); }
    }

    impl PathProvider for TestMapProvider {
        fn is_walkable(&mut self, position: (i32, i32), _movement_type: u8) -> bool {
            let (x, y) = position;

            // Check bounds
            if x < 0 || y < 0 || x >= self.width || y >= self.height {
                return false;
            }

            // Check if it's a wall
            !self.walls.contains(&position)
        }

        fn cost(&mut self, position: (i32, i32), _movement_type: u8) -> u32 {
            if self.is_walkable(position, 0) {
                1
            } else {
                u32::MAX
            }
        }
    }

    #[test]
    fn test_simple_path() {
        let mut map = TestMapProvider::new(10, 10);
        let pathfinder = PathFinder::AStar;

        let path = pathfinder.compute((0, 0), (3, 0), 0, false, &mut map);
        assert!(path.is_some());

        let path = path.unwrap();
        assert_eq!(path.len(), 3); // Should be [(1,0), (2,0), (3,0)]
        assert_eq!(path[0], (1, 0));
        assert_eq!(path[1], (2, 0));
        assert_eq!(path[2], (3, 0));
    }

    #[test]
    fn test_path_with_obstacle() {
        let mut map = TestMapProvider::new(10, 10);
        map.add_wall(1, 0); // Block direct path

        let pathfinder = PathFinder::AStar;
        let path = pathfinder.compute((0, 0), (2, 0), 0, false, &mut map);

        assert!(path.is_some());
        let path = path.unwrap();

        // Should find a path around the obstacle
        assert!(path.len() > 2); // Longer than direct path
        assert_eq!(path.last(), Some(&(2, 0))); // Should reach destination
    }

    #[test]
    fn test_no_path_available() {
        let mut map = TestMapProvider::new(10, 10);

        // Create a wall that completely blocks the path
        for y in 0..10 {
            map.add_wall(5, y);
        }

        let pathfinder = PathFinder::AStar;
        let path = pathfinder.compute((0, 0), (9, 0), 0, false, &mut map);

        assert!(path.is_none()); // No path should be found
    }
}
