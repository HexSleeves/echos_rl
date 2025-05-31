use bevy::prelude::*;
use std::collections::HashMap;
use std::time::{Duration, Instant};

use crate::core::components::Position;
use super::PathResult;

/// Cache key for pathfinding requests
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PathCacheKey {
    pub start: Position,
    pub goal: Position,
    pub allow_diagonal: bool,
}

impl PathCacheKey {
    pub fn new(start: Position, goal: Position, allow_diagonal: bool) -> Self {
        Self { start, goal, allow_diagonal }
    }
}

/// Cached path entry with timestamp
#[derive(Debug, Clone)]
pub struct CachedPath {
    pub result: PathResult,
    pub timestamp: Instant,
}

impl CachedPath {
    pub fn new(result: PathResult) -> Self {
        Self {
            result,
            timestamp: Instant::now(),
        }
    }

    pub fn is_expired(&self, max_age: Duration) -> bool {
        self.timestamp.elapsed() > max_age
    }
}

/// Path cache resource for storing computed paths
#[derive(Resource, Default)]
pub struct PathCache {
    cache: HashMap<PathCacheKey, CachedPath>,
    max_entries: usize,
    max_age: Duration,
    hits: u64,
    misses: u64,
    last_cleanup: Instant,
    cleanup_interval: Duration,
}

impl PathCache {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
            max_entries: 1000,
            max_age: Duration::from_secs(30),
            hits: 0,
            misses: 0,
            last_cleanup: Instant::now(),
            cleanup_interval: Duration::from_secs(5),
        }
    }

    pub fn with_capacity(mut self, max_entries: usize) -> Self {
        self.max_entries = max_entries;
        self
    }

    pub fn with_max_age(mut self, max_age: Duration) -> Self {
        self.max_age = max_age;
        self
    }

    /// Get a cached path if available and not expired
    pub fn get(&mut self, key: &PathCacheKey) -> Option<PathResult> {
        // Periodic cleanup
        if self.last_cleanup.elapsed() > self.cleanup_interval {
            self.cleanup_expired();
        }

        if let Some(cached) = self.cache.get(key) {
            if !cached.is_expired(self.max_age) {
                self.hits += 1;
                return Some(cached.result.clone());
            } else {
                // Remove expired entry
                self.cache.remove(key);
            }
        }

        self.misses += 1;
        None
    }

    /// Store a path in the cache
    pub fn insert(&mut self, key: PathCacheKey, result: PathResult) {
        // Don't cache empty paths
        if result.is_empty() {
            return;
        }

        // Enforce size limit
        if self.cache.len() >= self.max_entries {
            self.evict_oldest();
        }

        self.cache.insert(key, CachedPath::new(result));
    }

    /// Clear all cached paths
    pub fn clear(&mut self) {
        self.cache.clear();
        self.hits = 0;
        self.misses = 0;
    }

    /// Remove expired entries
    pub fn cleanup_expired(&mut self) {
        let max_age = self.max_age;
        self.cache.retain(|_, cached| !cached.is_expired(max_age));
        self.last_cleanup = Instant::now();
    }

    /// Evict the oldest entry to make room
    fn evict_oldest(&mut self) {
        if let Some((oldest_key, _)) = self.cache
            .iter()
            .min_by_key(|(_, cached)| cached.timestamp)
            .map(|(k, v)| (*k, v.clone()))
        {
            self.cache.remove(&oldest_key);
        }
    }

    /// Get cache statistics
    pub fn stats(&self) -> PathCacheStats {
        PathCacheStats {
            entries: self.cache.len(),
            max_entries: self.max_entries,
            hits: self.hits,
            misses: self.misses,
            hit_rate: if self.hits + self.misses > 0 {
                self.hits as f32 / (self.hits + self.misses) as f32
            } else {
                0.0
            },
        }
    }

    /// Invalidate paths that pass through a specific position
    /// Useful when the map changes (e.g., doors open/close, walls destroyed)
    pub fn invalidate_paths_through(&mut self, position: Position) {
        self.cache.retain(|_, cached| {
            !cached.result.path.contains(&position)
        });
    }

    /// Invalidate all paths involving a specific position as start or goal
    pub fn invalidate_paths_involving(&mut self, position: Position) {
        self.cache.retain(|key, _| {
            key.start != position && key.goal != position
        });
    }
}

/// Cache statistics for monitoring performance
#[derive(Debug, Clone)]
pub struct PathCacheStats {
    pub entries: usize,
    pub max_entries: usize,
    pub hits: u64,
    pub misses: u64,
    pub hit_rate: f32,
}

impl PathCacheStats {
    pub fn total_requests(&self) -> u64 {
        self.hits + self.misses
    }

    pub fn is_full(&self) -> bool {
        self.entries >= self.max_entries
    }
}

/// Enhanced pathfinder with caching
pub struct CachedPathfinder {
    pathfinder: super::Pathfinder,
    cache: PathCache,
}

impl Default for CachedPathfinder {
    fn default() -> Self {
        Self {
            pathfinder: super::Pathfinder::default(),
            cache: PathCache::new(),
        }
    }
}

impl CachedPathfinder {
    pub fn new(pathfinder: super::Pathfinder) -> Self {
        Self {
            pathfinder,
            cache: PathCache::new(),
        }
    }

    pub fn with_cache_config(mut self, max_entries: usize, max_age: Duration) -> Self {
        self.cache = self.cache.with_capacity(max_entries).with_max_age(max_age);
        self
    }

    /// Find a path with caching
    pub fn find_path(
        &mut self,
        start: Position,
        goal: Position,
        map: &crate::core::resources::CurrentMap,
    ) -> PathResult {
        let key = PathCacheKey::new(start, goal, false); // Assuming no diagonal for now

        // Try cache first
        if let Some(cached_result) = self.cache.get(&key) {
            return cached_result;
        }

        // Compute path
        let result = self.pathfinder.find_path(start, goal, map);

        // Cache the result
        self.cache.insert(key, result.clone());

        result
    }

    /// Find a path with diagonal movement
    pub fn find_path_diagonal(
        &mut self,
        start: Position,
        goal: Position,
        map: &crate::core::resources::CurrentMap,
    ) -> PathResult {
        let key = PathCacheKey::new(start, goal, true);

        if let Some(cached_result) = self.cache.get(&key) {
            return cached_result;
        }

        let pathfinder = self.pathfinder.with_diagonal(true);
        let result = pathfinder.find_path(start, goal, map);

        self.cache.insert(key, result.clone());

        result
    }

    /// Get cache statistics
    pub fn cache_stats(&self) -> PathCacheStats {
        self.cache.stats()
    }

    /// Clear the cache
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }

    /// Invalidate cache entries when map changes
    pub fn invalidate_position(&mut self, position: Position) {
        self.cache.invalidate_paths_through(position);
    }
}
