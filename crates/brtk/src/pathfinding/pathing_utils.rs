//! Utility data structures and functions for pathfinding

use std::collections::VecDeque;

/// A simple indexed list implementation for pathfinding nodes
/// This replaces the external IndexList with a standard Rust implementation
#[derive(Debug)]
pub struct IndexList<T> {
    items: VecDeque<T>,
}

impl<T> IndexList<T> {
    /// Create a new empty IndexList
    pub fn new() -> Self {
        Self {
            items: VecDeque::new(),
        }
    }

    /// Check if the list is empty
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Insert an item at the front of the list
    pub fn insert_first(&mut self, item: T) {
        self.items.push_front(item);
    }

    /// Insert an item at the back of the list
    pub fn insert_last(&mut self, item: T) {
        self.items.push_back(item);
    }

    /// Remove and return the first item
    pub fn remove_first(&mut self) -> Option<T> {
        self.items.pop_front()
    }

    /// Get the first index (always 0 if not empty)
    pub fn first_index(&self) -> Option<usize> {
        if self.items.is_empty() {
            None
        } else {
            Some(0)
        }
    }

    /// Get the next index after the given index
    pub fn next_index(&self, index: Option<usize>) -> Option<usize> {
        match index {
            Some(i) if i + 1 < self.items.len() => Some(i + 1),
            _ => None,
        }
    }

    /// Get a reference to an item at the given index
    pub fn get(&self, index: Option<usize>) -> Option<&T> {
        index.and_then(|i| self.items.get(i))
    }

    /// Remove an item at the given index
    pub fn remove(&mut self, index: Option<usize>) -> Option<T> {
        index.and_then(|i| self.items.remove(i))
    }

    /// Insert an item before the given index
    pub fn insert_before(&mut self, index: Option<usize>, item: T) {
        match index {
            Some(i) => self.items.insert(i, item),
            None => self.items.push_back(item),
        }
    }
}

impl<T> Default for IndexList<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Calculate Manhattan distance between two points
pub fn manhattan_distance(a: (i32, i32), b: (i32, i32)) -> u32 {
    ((a.0 - b.0).abs() + (a.1 - b.1).abs()) as u32
}

/// Calculate Euclidean distance between two points (scaled to u32)
pub fn euclidean_distance(a: (i32, i32), b: (i32, i32)) -> u32 {
    let dx = (a.0 - b.0) as f32;
    let dy = (a.1 - b.1) as f32;
    ((dx * dx + dy * dy).sqrt() * 10.0) as u32 // Scale by 10 for precision
}
