//! Basic visibility map implementation

use crate::fov::traits::FovReceiver;
use std::collections::HashSet;

/// A simple visibility map using a HashSet to store visible positions
#[derive(Default, Debug, Clone)]
pub struct VisibilityMap {
    visible_positions: HashSet<(i32, i32)>,
    explored_positions: HashSet<(i32, i32)>,
    /// Cache the last computed range for performance optimization
    last_range: Option<u32>,
}

impl VisibilityMap {
    #[inline]
    pub fn new() -> Self {
        Self { visible_positions: HashSet::new(), explored_positions: HashSet::new(), last_range: None }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            visible_positions: HashSet::with_capacity(capacity),
            explored_positions: HashSet::with_capacity(capacity),
            last_range: None,
        }
    }

    /// Create a visibility map optimized for a specific range
    pub fn with_range_capacity(range: u32) -> Self {
        // Estimate capacity based on circular area: π * r²
        let estimated_capacity = ((range * range) as f32 * std::f32::consts::PI) as usize;
        Self::with_capacity(estimated_capacity.max(64)) // Minimum 64 for small ranges
    }

    pub fn visible_count(&self) -> usize { self.visible_positions.len() }

    pub fn is_empty(&self) -> bool { self.visible_positions.is_empty() }

    pub fn get_all_visible(&self) -> Vec<(i32, i32)> { self.visible_positions.iter().copied().collect() }

    pub fn get_visible_set(&self) -> &HashSet<(i32, i32)> { &self.visible_positions }

    pub fn get_all_explored(&self) -> Vec<(i32, i32)> { self.explored_positions.iter().copied().collect() }

    pub fn get_explored_set(&self) -> &HashSet<(i32, i32)> { &self.explored_positions }

    pub fn set_explored(&mut self, position: (i32, i32)) { self.explored_positions.insert(position); }

    pub fn get_explored(&self, position: (i32, i32)) -> bool { self.explored_positions.contains(&position) }

    pub fn clear_explored(&mut self) { self.explored_positions.clear(); }

    /// Efficiently resize the internal capacity if needed
    pub fn optimize_for_range(&mut self, range: u32) {
        if self.last_range != Some(range) {
            let estimated_capacity = ((range * range) as f32 * std::f32::consts::PI) as usize;
            let target_capacity = estimated_capacity.max(64);

            // Only resize if significantly different
            if self.visible_positions.capacity() < target_capacity / 2
                || self.visible_positions.capacity() > target_capacity * 2
            {
                self.visible_positions.reserve(target_capacity);
                self.explored_positions.reserve(target_capacity);
            }

            self.last_range = Some(range);
        }
    }
}

impl FovReceiver for VisibilityMap {
    fn set_visible(&mut self, position: (i32, i32)) {
        self.visible_positions.insert(position);
        self.explored_positions.insert(position); // Mark as explored when visible
    }

    fn get_visible(&self, position: (i32, i32)) -> bool { self.visible_positions.contains(&position) }

    fn clear_visible(&mut self) { self.visible_positions.clear(); }
}
