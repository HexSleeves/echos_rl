//! Path caching system for improved pathfinding performance
//!
//! LRU cache with expiration for pathfinding results to avoid redundant computations.

use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

/// Configuration for the path cache
#[derive(Debug, Clone)]
pub struct PathCacheConfig {
    /// Maximum number of cached paths
    pub max_entries: usize,
    /// Maximum age of cached paths before they expire
    pub max_age: Duration,
    /// Whether to enable cache statistics tracking
    pub enable_stats: bool,
}

impl Default for PathCacheConfig {
    fn default() -> Self {
        Self {
            max_entries: 1000,
            max_age: Duration::from_secs(30), // 30 seconds
            enable_stats: true,
        }
    }
}

/// A cached path with metadata
#[derive(Debug, Clone)]
pub struct CachedPath {
    /// The computed path
    pub path: Vec<(i32, i32)>,
    /// When this path was cached
    pub cached_at: Instant,
    /// How many times this path has been accessed
    pub access_count: u32,
    /// Last time this path was accessed
    pub last_accessed: Instant,
}

impl CachedPath {
    /// Create a new cached path
    pub fn new(path: Vec<(i32, i32)>) -> Self {
        let now = Instant::now();
        Self { path, cached_at: now, access_count: 0, last_accessed: now }
    }

    /// Check if this cached path has expired
    pub fn is_expired(&self, max_age: Duration) -> bool { self.cached_at.elapsed() > max_age }

    /// Mark this path as accessed
    pub fn mark_accessed(&mut self) {
        self.access_count += 1;
        self.last_accessed = Instant::now();
    }
}

/// Cache key for pathfinding requests
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct PathCacheKey {
    origin: (i32, i32),
    destination: (i32, i32),
    movement_type: u8,
}

impl PathCacheKey {
    fn new(origin: (i32, i32), destination: (i32, i32), movement_type: u8) -> Self {
        Self { origin, destination, movement_type }
    }
}

/// LRU cache with expiration for pathfinding results
#[derive(Debug)]
pub struct PathCache {
    cache: HashMap<PathCacheKey, CachedPath>,
    config: PathCacheConfig,
    // TODO: access_order is a plain Vec, so every cache hit (update_access_order) and every eviction
    // (evict_lru) performs a linear retain / remove(0) – worst-case O(n) with n = max_entries.
    access_order: Vec<PathCacheKey>, // For LRU eviction
}

impl PathCache {
    /// Create a new path cache with the given configuration
    pub fn new(config: PathCacheConfig) -> Self {
        let max_entries = config.max_entries;
        Self {
            cache: HashMap::with_capacity(max_entries),
            config,
            access_order: Vec::with_capacity(max_entries),
        }
    }

    /// Get a cached path if it exists and is still valid
    pub fn get_path(
        &mut self,
        origin: (i32, i32),
        destination: (i32, i32),
        movement_type: u8,
    ) -> Option<&CachedPath> {
        let key = PathCacheKey::new(origin, destination, movement_type);

        // Check if the path exists and is not expired
        let is_expired = if let Some(cached_path) = self.cache.get(&key) {
            cached_path.is_expired(self.config.max_age)
        } else {
            return None;
        };

        if is_expired {
            // Path has expired, remove it
            self.cache.remove(&key);
            self.access_order.retain(|k| k != &key);
            return None;
        }

        // Update access information
        if let Some(cached_path) = self.cache.get_mut(&key) {
            cached_path.mark_accessed();
        }
        self.update_access_order(&key);

        // Return immutable reference
        self.cache.get(&key)
    }

    /// Store a new path in the cache
    pub fn store_path(
        &mut self,
        origin: (i32, i32),
        destination: (i32, i32),
        movement_type: u8,
        path: Vec<(i32, i32)>,
    ) {
        let key = PathCacheKey::new(origin, destination, movement_type);

        // If we're at capacity, remove the least recently used entry
        if self.cache.len() >= self.config.max_entries && !self.cache.contains_key(&key) {
            self.evict_lru();
        }

        // Store the new path
        let cached_path = CachedPath::new(path);
        self.cache.insert(key.clone(), cached_path);
        self.update_access_order(&key);
    }

    /// Clear all cached paths
    pub fn clear(&mut self) {
        self.cache.clear();
        self.access_order.clear();
    }

    /// Remove expired entries from the cache
    pub fn cleanup_expired(&mut self) {
        let max_age = self.config.max_age;
        let expired_keys: Vec<_> = self
            .cache
            .iter()
            .filter(|(_, cached_path)| cached_path.is_expired(max_age))
            .map(|(key, _)| key.clone())
            .collect();

        for key in expired_keys {
            self.cache.remove(&key);
            self.access_order.retain(|k| k != &key);
        }
    }

    /// Get cache size information (current size, capacity)
    pub fn size_info(&self) -> (usize, usize) { (self.cache.len(), self.config.max_entries) }

    /// Get cache statistics
    pub fn stats(&self) -> CacheStats {
        let total_accesses: u32 = self.cache.values().map(|path| path.access_count).sum();
        let avg_path_length = if !self.cache.is_empty() {
            self.cache.values().map(|path| path.path.len()).sum::<usize>() as f32 / self.cache.len() as f32
        } else {
            0.0
        };

        CacheStats {
            total_entries: self.cache.len(),
            max_entries: self.config.max_entries,
            total_accesses,
            avg_path_length,
            oldest_entry_age: self.get_oldest_entry_age(),
        }
    }

    /// Update the access order for LRU eviction
    fn update_access_order(&mut self, key: &PathCacheKey) {
        // Remove the key if it already exists
        self.access_order.retain(|k| k != key);
        // Add it to the end (most recently used)
        self.access_order.push(key.clone());
    }

    /// Evict the least recently used entry
    fn evict_lru(&mut self) {
        if let Some(lru_key) = self.access_order.first().cloned() {
            self.cache.remove(&lru_key);
            self.access_order.remove(0);
        }
    }

    /// Get the age of the oldest entry in the cache
    fn get_oldest_entry_age(&self) -> Option<Duration> {
        self.cache.values().map(|path| path.cached_at.elapsed()).min()
    }
}

impl Default for PathCache {
    fn default() -> Self { Self::new(PathCacheConfig::default()) }
}

/// Statistics about cache performance
#[derive(Debug, Clone)]
pub struct CacheStats {
    pub total_entries: usize,
    pub max_entries: usize,
    pub total_accesses: u32,
    pub avg_path_length: f32,
    pub oldest_entry_age: Option<Duration>,
}

impl CacheStats {
    /// Get cache utilization as a percentage
    pub fn utilization_percent(&self) -> f32 {
        if self.max_entries > 0 {
            (self.total_entries as f32 / self.max_entries as f32) * 100.0
        } else {
            0.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_cache_basic_operations() {
        let mut cache = PathCache::default();
        let path = vec![(0, 0), (1, 0), (2, 0)];

        // Store a path
        cache.store_path((0, 0), (2, 0), 0, path.clone());

        // Retrieve the path
        let cached = cache.get_path((0, 0), (2, 0), 0);
        assert!(cached.is_some());
        assert_eq!(cached.unwrap().path, path);
    }

    #[test]
    fn test_cache_expiration() {
        let config = PathCacheConfig {
            max_entries: 100,
            max_age: Duration::from_millis(10), // Very short expiration
            enable_stats: true,
        };
        let mut cache = PathCache::new(config);
        let path = vec![(0, 0), (1, 0), (2, 0)];

        // Store a path
        cache.store_path((0, 0), (2, 0), 0, path);

        // Wait for expiration
        thread::sleep(Duration::from_millis(20));

        // Path should be expired and not returned
        let cached = cache.get_path((0, 0), (2, 0), 0);
        assert!(cached.is_none());
    }

    #[test]
    fn test_cache_lru_eviction() {
        let config = PathCacheConfig {
            max_entries: 2, // Very small cache
            max_age: Duration::from_secs(60),
            enable_stats: true,
        };
        let mut cache = PathCache::new(config);

        // Fill the cache
        cache.store_path((0, 0), (1, 0), 0, vec![(0, 0), (1, 0)]);
        cache.store_path((1, 0), (2, 0), 0, vec![(1, 0), (2, 0)]);

        // Access the first path to make it more recently used
        cache.get_path((0, 0), (1, 0), 0);

        // Add a third path, should evict the second one (LRU)
        cache.store_path((2, 0), (3, 0), 0, vec![(2, 0), (3, 0)]);

        // First path should still be there
        assert!(cache.get_path((0, 0), (1, 0), 0).is_some());
        // Second path should be evicted
        assert!(cache.get_path((1, 0), (2, 0), 0).is_none());
        // Third path should be there
        assert!(cache.get_path((2, 0), (3, 0), 0).is_some());
    }

    #[test]
    fn test_cache_stats() {
        let mut cache = PathCache::default();
        let path = vec![(0, 0), (1, 0), (2, 0)];

        cache.store_path((0, 0), (2, 0), 0, path);

        // Access the path multiple times
        cache.get_path((0, 0), (2, 0), 0);
        cache.get_path((0, 0), (2, 0), 0);

        let stats = cache.stats();
        assert_eq!(stats.total_entries, 1);
        assert_eq!(stats.total_accesses, 2);
        assert_eq!(stats.avg_path_length, 3.0);
    }

    #[test]
    fn test_cache_cleanup() {
        let config =
            PathCacheConfig { max_entries: 100, max_age: Duration::from_millis(10), enable_stats: true };
        let mut cache = PathCache::new(config);

        // Store some paths
        cache.store_path((0, 0), (1, 0), 0, vec![(0, 0), (1, 0)]);
        cache.store_path((1, 0), (2, 0), 0, vec![(1, 0), (2, 0)]);

        assert_eq!(cache.cache.len(), 2);

        // Wait for expiration
        thread::sleep(Duration::from_millis(20));

        // Clean up expired entries
        cache.cleanup_expired();

        assert_eq!(cache.cache.len(), 0);
    }
}
