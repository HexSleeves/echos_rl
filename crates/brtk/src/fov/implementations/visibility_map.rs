//! Basic visibility map implementation

use crate::fov::traits::FovReceiver;
use std::collections::HashSet;

/// A simple visibility map using a HashSet to store visible positions
#[derive(Default, Debug, Clone)]
pub struct VisibilityMap {
    visible_positions: HashSet<(i32, i32)>,
}

impl VisibilityMap {
    #[inline]
    pub fn new() -> Self {
        Self {
            visible_positions: HashSet::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            visible_positions: HashSet::with_capacity(capacity),
        }
    }

    pub fn visible_count(&self) -> usize {
        self.visible_positions.len()
    }

    pub fn is_empty(&self) -> bool {
        self.visible_positions.is_empty()
    }

    pub fn get_all_visible(&self) -> Vec<(i32, i32)> {
        self.visible_positions.iter().copied().collect()
    }

    pub fn get_visible_set(&self) -> &HashSet<(i32, i32)> {
        &self.visible_positions
    }
}

impl FovReceiver for VisibilityMap {
    fn set_visible(&mut self, position: (i32, i32)) {
        self.visible_positions.insert(position);
    }

    fn get_visible(&self, position: (i32, i32)) -> bool {
        self.visible_positions.contains(&position)
    }

    fn clear_visible(&mut self) {
        self.visible_positions.clear();
    }
}
