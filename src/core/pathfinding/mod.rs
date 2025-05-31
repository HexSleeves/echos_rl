use bevy::prelude::*;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::cmp::Ordering;

use crate::core::{components::Position, resources::CurrentMap};

pub mod astar;
pub mod dijkstra;
pub mod path_cache;

pub use astar::*;
pub use dijkstra::*;
pub use path_cache::*;

/// Represents a node in the pathfinding algorithm
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PathNode {
    pub position: Position,
    pub cost: u32,
    pub heuristic: u32,
}

impl PathNode {
    pub fn new(position: Position, cost: u32, heuristic: u32) -> Self {
        Self { position, cost, heuristic }
    }

    pub fn total_cost(&self) -> u32 {
        self.cost + self.heuristic
    }
}

impl Ord for PathNode {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse ordering for min-heap behavior
        other.total_cost().cmp(&self.total_cost())
            .then_with(|| other.heuristic.cmp(&self.heuristic))
    }
}

impl PartialOrd for PathNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Pathfinding algorithm types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PathfindingAlgorithm {
    AStar,
    Dijkstra,
}

/// Pathfinding result
#[derive(Debug, Clone)]
pub struct PathResult {
    pub path: Vec<Position>,
    pub cost: u32,
    pub nodes_explored: usize,
}

impl PathResult {
    pub fn empty() -> Self {
        Self { path: Vec::new(), cost: 0, nodes_explored: 0 }
    }

    pub fn is_empty(&self) -> bool {
        self.path.is_empty()
    }

    pub fn len(&self) -> usize {
        self.path.len()
    }

    pub fn first(&self) -> Option<Position> {
        self.path.first().copied()
    }

    pub fn last(&self) -> Option<Position> {
        self.path.last().copied()
    }
}

/// Main pathfinding interface
pub struct Pathfinder {
    algorithm: PathfindingAlgorithm,
    max_iterations: usize,
    allow_diagonal: bool,
}

impl Default for Pathfinder {
    fn default() -> Self {
        Self {
            algorithm: PathfindingAlgorithm::AStar,
            max_iterations: 10000,
            allow_diagonal: false,
        }
    }
}

impl Pathfinder {
    pub fn new(algorithm: PathfindingAlgorithm) -> Self {
        Self { algorithm, ..Default::default() }
    }

    pub fn with_max_iterations(mut self, max_iterations: usize) -> Self {
        self.max_iterations = max_iterations;
        self
    }

    pub fn with_diagonal(mut self, allow_diagonal: bool) -> Self {
        self.allow_diagonal = allow_diagonal;
        self
    }

    /// Find a path from start to goal
    pub fn find_path(
        &self,
        start: Position,
        goal: Position,
        map: &CurrentMap,
    ) -> PathResult {
        match self.algorithm {
            PathfindingAlgorithm::AStar => {
                astar::find_path(start, goal, map, self.max_iterations, self.allow_diagonal)
            }
            PathfindingAlgorithm::Dijkstra => {
                dijkstra::find_path(start, goal, map, self.max_iterations, self.allow_diagonal)
            }
        }
    }

    /// Find a path to the closest reachable position near the goal
    pub fn find_path_to_nearest(
        &self,
        start: Position,
        goal: Position,
        map: &CurrentMap,
        max_distance: u32,
    ) -> PathResult {
        // First try direct path
        let direct_path = self.find_path(start, goal, map);
        if !direct_path.is_empty() {
            return direct_path;
        }

        // If direct path fails, try positions around the goal
        let neighbors = get_neighbors_in_radius(goal, max_distance, self.allow_diagonal);
        let mut best_path = PathResult::empty();
        let mut best_distance = f32::INFINITY;

        for neighbor in neighbors {
            if map.is_walkable(neighbor) && map.get_actor(neighbor).is_none() {
                let path = self.find_path(start, neighbor, map);
                if !path.is_empty() {
                    let distance = calculate_distance(neighbor, goal);
                    if distance < best_distance {
                        best_distance = distance;
                        best_path = path;
                    }
                }
            }
        }

        best_path
    }
}

/// Calculate Manhattan distance between two positions
pub fn manhattan_distance(a: Position, b: Position) -> u32 {
    ((a.x() - b.x()).abs() + (a.y() - b.y()).abs()) as u32
}

/// Calculate Euclidean distance between two positions
pub fn euclidean_distance(a: Position, b: Position) -> u32 {
    let dx = (a.x() - b.x()) as f32;
    let dy = (a.y() - b.y()) as f32;
    (dx * dx + dy * dy).sqrt() as u32
}

// Removed the duplicate implementation of `calculate_distance`.
// The function is now sourced from `utils.rs`.

/// Get neighbors of a position
pub fn get_neighbors(position: Position, allow_diagonal: bool) -> Vec<Position> {
    let mut neighbors = Vec::new();
    let (x, y) = (position.x(), position.y());

    // Cardinal directions
    neighbors.push(Position::new(x, y - 1)); // North
    neighbors.push(Position::new(x + 1, y)); // East
    neighbors.push(Position::new(x, y + 1)); // South
    neighbors.push(Position::new(x - 1, y)); // West

    if allow_diagonal {
        // Diagonal directions
        neighbors.push(Position::new(x - 1, y - 1)); // Northwest
        neighbors.push(Position::new(x + 1, y - 1)); // Northeast
        neighbors.push(Position::new(x + 1, y + 1)); // Southeast
        neighbors.push(Position::new(x - 1, y + 1)); // Southwest
    }

    neighbors
}

/// Get all positions within a radius
pub fn get_neighbors_in_radius(center: Position, radius: u32, allow_diagonal: bool) -> Vec<Position> {
    let mut positions = Vec::new();
    let radius = radius as i32;

    for dx in -radius..=radius {
        for dy in -radius..=radius {
            if dx == 0 && dy == 0 {
                continue; // Skip center
            }

            if !allow_diagonal && dx != 0 && dy != 0 {
                continue; // Skip diagonals if not allowed
            }

            let distance = if allow_diagonal {
                // Chebyshev distance for diagonal movement
                dx.abs().max(dy.abs())
            } else {
                // Manhattan distance for cardinal movement
                dx.abs() + dy.abs()
            };

            if distance <= radius {
                positions.push(Position::new(center.x() + dx, center.y() + dy));
            }
        }
    }

    positions
}

/// Reconstruct path from came_from map
pub fn reconstruct_path(
    came_from: &HashMap<Position, Position>,
    start: Position,
    goal: Position,
) -> Vec<Position> {
    let mut path = Vec::new();
    let mut current = goal;

    while current != start {
        path.push(current);
        if let Some(&next) = came_from.get(&current) {
            current = next;
        } else {
            // Path reconstruction failed
            return Vec::new();
        }
    }

    path.push(start);
    path.reverse();
    path
}
