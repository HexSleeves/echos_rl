//! Pathfinding algorithms with caching and performance optimization
//!
//! A* pathfinding with LRU caching, performance monitoring, and game-specific optimizations.

pub mod astar;
pub mod path_cache;
pub mod pathing_traits;
pub mod pathing_utils;

// Re-export main types for convenience
pub use astar::{AStar, AStarNode};
pub use path_cache::{CachedPath, PathCache, PathCacheConfig};
pub use pathing_traits::{PathAlgorithm, PathProvider};
pub use pathing_utils::IndexList;

/// Pathfinding algorithm selector with caching support
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

    /// Compute a path with caching support
    pub fn compute_cached<P: PathProvider>(
        &self,
        origin: (i32, i32),
        destination: (i32, i32),
        movement_type: u8,
        partial_path_on_failure: bool,
        provider: &mut P,
        cache: &mut PathCache,
    ) -> Option<Vec<(i32, i32)>> {
        // Check cache first
        if let Some(cached_path) = cache.get_path(origin, destination, movement_type) {
            return Some(cached_path.path.clone());
        }

        // Compute new path
        let path = self.compute(origin, destination, movement_type, partial_path_on_failure, provider)?;

        // Cache the result
        cache.store_path(origin, destination, movement_type, path.clone());

        Some(path)
    }
}

/// Pathfinding manager with caching and performance monitoring
#[derive(Debug)]
pub struct PathfindingManager {
    cache: PathCache,
    algorithm: PathFinder,
    stats: PathfindingStats,
}

impl Default for PathfindingManager {
    fn default() -> Self { Self::new(PathFinder::default(), PathCacheConfig::default()) }
}

impl PathfindingManager {
    /// Create a new pathfinding manager with specified algorithm and cache config
    pub fn new(algorithm: PathFinder, cache_config: PathCacheConfig) -> Self {
        Self { cache: PathCache::new(cache_config), algorithm, stats: PathfindingStats::default() }
    }

    /// Find a path with automatic caching and statistics tracking
    pub fn find_path<P: PathProvider>(
        &mut self,
        origin: (i32, i32),
        destination: (i32, i32),
        movement_type: u8,
        partial_path_on_failure: bool,
        provider: &mut P,
    ) -> Option<Vec<(i32, i32)>> {
        self.stats.total_requests += 1;

        // Check cache first
        if let Some(cached_path) = self.cache.get_path(origin, destination, movement_type) {
            self.stats.cache_hits += 1;
            return Some(cached_path.path.clone());
        }

        self.stats.cache_misses += 1;

        // Compute new path
        let start_time = std::time::Instant::now();
        let path =
            self.algorithm.compute(origin, destination, movement_type, partial_path_on_failure, provider);
        let computation_time = start_time.elapsed();

        self.stats.total_computation_time += computation_time;

        if let Some(ref path) = path {
            // Cache the result
            self.cache.store_path(origin, destination, movement_type, path.clone());
            self.stats.successful_computations += 1;
        } else {
            self.stats.failed_computations += 1;
        }

        path
    }

    /// Get pathfinding statistics
    pub fn stats(&self) -> &PathfindingStats { &self.stats }

    /// Clear the path cache
    pub fn clear_cache(&mut self) { self.cache.clear(); }

    /// Get cache statistics
    pub fn cache_stats(&self) -> (usize, usize, f32) {
        let (size, capacity) = self.cache.size_info();
        let hit_rate = if self.stats.total_requests > 0 {
            self.stats.cache_hits as f32 / self.stats.total_requests as f32
        } else {
            0.0
        };
        (size, capacity, hit_rate)
    }
}

/// Statistics for pathfinding performance monitoring
#[derive(Debug, Default)]
pub struct PathfindingStats {
    pub total_requests: u64,
    pub cache_hits: u64,
    pub cache_misses: u64,
    pub successful_computations: u64,
    pub failed_computations: u64,
    pub total_computation_time: std::time::Duration,
}

impl PathfindingStats {
    /// Get the cache hit rate as a percentage
    pub fn cache_hit_rate(&self) -> f32 {
        if self.total_requests > 0 {
            (self.cache_hits as f32 / self.total_requests as f32) * 100.0
        } else {
            0.0
        }
    }

    /// Get the average computation time per request
    pub fn average_computation_time(&self) -> std::time::Duration {
        if self.cache_misses > 0 {
            self.total_computation_time.checked_div(self.cache_misses as u32).unwrap_or_default()
        } else {
            std::time::Duration::ZERO
        }
    }

    /// Reset all statistics
    pub fn reset(&mut self) { *self = Self::default(); }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::direction::Direction;

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
    fn test_direction_iterator() {
        // Test that the direction iterator works correctly
        let directions: Vec<_> = Direction::iter_cardinal_ordinal().collect();
        assert_eq!(directions.len(), 8);
    }

    #[test]
    fn test_simple_path() {
        let mut map = TestMapProvider::new(10, 10);
        let pathfinder = PathFinder::AStar;

        let path = pathfinder.compute((0, 0), (3, 0), 0, false, &mut map);
        assert!(path.is_some());

        let path = path.unwrap();

        // The path should include all steps from (0,0) to (3,0)
        assert_eq!(path.len(), 4); // Should be [(0,0), (1,0), (2,0), (3,0)]
        assert_eq!(path[0], (0, 0));
        assert_eq!(path[path.len() - 1], (3, 0)); // Check destination
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

    #[test]
    fn test_partial_path_reconstruction() {
        let mut map = TestMapProvider::new(10, 10);

        // Create a wall that completely blocks the path
        for y in 0..10 {
            map.add_wall(5, y);
        }

        let pathfinder = PathFinder::AStar;
        // Request partial path when no full path is available
        let path = pathfinder.compute((0, 0), (9, 0), 0, true, &mut map);

        assert!(path.is_some()); // Should find a partial path
        let path = path.unwrap();

        // Should start from origin
        assert_eq!(path[0], (0, 0)); // Should start from origin

        // For a completely blocked path, we might only get the origin
        // This is acceptable behavior for partial pathfinding
        if path.len() > 1 {
            // The path should get closer to the destination than the origin
            let origin_to_dest_dist = (9i32.pow(2) + 0i32.pow(2)) as f32;
            let last_pos = path.last().unwrap();
            let last_to_dest_dist = ((9i32 - last_pos.0).pow(2) + (0i32 - last_pos.1).pow(2)) as f32;
            assert!(last_to_dest_dist < origin_to_dest_dist);
        }
    }

    #[test]
    fn test_pathfinding_manager_caching() {
        let mut map = TestMapProvider::new(10, 10);
        let mut manager = PathfindingManager::default();

        // First request should be a cache miss
        let path1 = manager.find_path((0, 0), (3, 0), 0, false, &mut map);
        assert!(path1.is_some());
        assert_eq!(manager.stats().cache_hits, 0);
        assert_eq!(manager.stats().cache_misses, 1);

        // Second identical request should be a cache hit
        let path2 = manager.find_path((0, 0), (3, 0), 0, false, &mut map);
        assert!(path2.is_some());
        assert_eq!(path1, path2);
        assert_eq!(manager.stats().cache_hits, 1);
        assert_eq!(manager.stats().cache_misses, 1);
    }

    #[test]
    fn test_early_exit_optimization() {
        let mut map = TestMapProvider::new(10, 10);
        let pathfinder = PathFinder::AStar;

        // Test same position early exit
        let path = pathfinder.compute((5, 5), (5, 5), 0, false, &mut map);
        assert!(path.is_some());
        let path = path.unwrap();
        assert_eq!(path.len(), 1);
        assert_eq!(path[0], (5, 5));
    }
}
