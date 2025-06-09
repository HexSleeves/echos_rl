//! A* pathfinding algorithm implementation
//!
//! Optimized A* implementation with performance enhancements, intelligent heuristics,
//! and caching support for game pathfinding scenarios.

use crate::{
    direction::Direction,
    pathfinding::{
        pathing_traits::{PathAlgorithm, PathProvider},
        pathing_utils::euclidean_distance,
    },
};
use std::collections::{BinaryHeap, HashMap, HashSet};

// Movement costs (scaled by 10 for precision)
const CARDINAL_COST: u32 = 10; // 1.0 * 10
const ORDINAL_COST: u32 = 14; // 1.4 * 10

// Performance optimization constants
const MAX_SEARCH_NODES: usize = 2000; // Limit search space for performance

/// A* node with f-score optimization for priority queue
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AStarNode {
    position: (i32, i32),
    g_score: u32, // Cost from start
    h_score: u32, // Heuristic cost to goal
    f_score: u32, // Total cost (g + h)
}

impl AStarNode {
    /// Create a new starting node
    pub fn new(origin: (i32, i32), destination: (i32, i32)) -> Self {
        let h_score = euclidean_distance(origin, destination);
        Self { position: origin, g_score: 0, h_score, f_score: h_score }
    }

    /// Create a node with specific scores
    pub fn with_scores(position: (i32, i32), g_score: u32, h_score: u32, f_score: u32) -> Self {
        Self { position, g_score, h_score, f_score }
    }

    /// Get the position of this node
    pub fn position(&self) -> (i32, i32) { self.position }

    /// Get the g-score (cost from start)
    pub fn g_score(&self) -> u32 { self.g_score }

    /// Get the h-score (heuristic cost to goal)
    pub fn h_score(&self) -> u32 { self.h_score }

    /// Get the f-score (total cost)
    pub fn f_score(&self) -> u32 { self.f_score }
}

// Min-heap behavior: lower f_score = higher priority
impl Ord for AStarNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .f_score
            .cmp(&self.f_score)
            .then_with(|| other.h_score.cmp(&self.h_score))
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for AStarNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}

/// A* pathfinding algorithm implementation with performance optimizations
pub struct AStar;
impl AStar {
    /// Heuristic with direction consistency penalty to encourage straighter paths
    fn improved_heuristic(pos: (i32, i32), destination: (i32, i32), came_from: (i32, i32)) -> u32 {
        let base_heuristic = euclidean_distance(pos, destination);

        let dx_to_dest = destination.0 - pos.0;
        let dy_to_dest = destination.1 - pos.1;
        let dx_from_prev = pos.0 - came_from.0;
        let dy_from_prev = pos.1 - came_from.1;

        let direction_penalty =
            if dx_to_dest.signum() == dx_from_prev.signum() && dy_to_dest.signum() == dy_from_prev.signum() {
                0
            } else {
                2
            };

        base_heuristic.saturating_add(direction_penalty)
    }

    /// Sort directions by proximity to target for better exploration order
    fn get_ordered_directions(current: (i32, i32), destination: (i32, i32)) -> Vec<Direction> {
        let mut directions = Direction::iter_cardinal_ordinal().collect::<Vec<_>>();

        directions.sort_by_key(|&direction| {
            let coord = direction.coord();
            let new_pos = (current.0 + coord.0, current.1 + coord.1);
            euclidean_distance(new_pos, destination)
        });

        directions
    }

    /// Optimized path reconstruction with pre-allocated capacity
    fn reconstruct_path(
        destination: (i32, i32),
        came_from: &HashMap<(i32, i32), (i32, i32)>,
    ) -> Vec<(i32, i32)> {
        // Estimate path length for better allocation
        let estimated_length = Self::estimate_path_length(destination, came_from);
        let mut path = Vec::with_capacity(estimated_length);

        let mut current = destination;
        path.push(current);

        while let Some(&parent) = came_from.get(&current) {
            current = parent;
            path.push(current);
        }

        path.reverse();
        path
    }

    /// Estimate path length for better memory allocation
    fn estimate_path_length(destination: (i32, i32), came_from: &HashMap<(i32, i32), (i32, i32)>) -> usize {
        let mut length = 1;
        let mut current = destination;

        // Count backwards up to a reasonable limit
        while let Some(&parent) = came_from.get(&current) {
            length += 1;
            current = parent;

            // Prevent infinite loops and excessive counting
            if length > 100 {
                break;
            }
        }

        length
    }

    /// Find best partial path when no complete path exists
    fn find_best_partial_path(
        g_score: &HashMap<(i32, i32), u32>,
        came_from: &HashMap<(i32, i32), (i32, i32)>,
        destination: (i32, i32),
    ) -> Option<Vec<(i32, i32)>> {
        // Find the position closest to the destination with the best cost/distance ratio
        let mut best_pos = None;
        let mut best_score = f32::MAX;

        for (&pos, &cost) in g_score {
            let distance = euclidean_distance(pos, destination) as f32;
            let cost_distance_ratio = (cost as f32) / (distance + 1.0); // +1 to avoid division by zero

            if cost_distance_ratio < best_score {
                best_score = cost_distance_ratio;
                best_pos = Some(pos);
            }
        }

        // Reconstruct the path to the best position found
        best_pos.map(|pos| Self::reconstruct_path(pos, came_from))
    }
}

impl PathAlgorithm for AStar {
    fn compute_path<P: PathProvider>(
        origin: (i32, i32),
        destination: (i32, i32),
        movement_type: u8,
        partial_path_on_failure: bool,
        provider: &mut P,
    ) -> Option<Vec<(i32, i32)>> {
        // Early exit for same position
        if origin == destination {
            return Some(vec![origin]);
        }

        // Use standard collections for a simpler, more reliable implementation
        let mut open_set = BinaryHeap::new();
        let mut closed_set = HashSet::new();
        let mut came_from = HashMap::new();
        let mut g_score = HashMap::new();
        let mut nodes_explored = 0;

        // Initialize starting node
        let start_node = AStarNode::new(origin, destination);
        open_set.push(start_node);
        g_score.insert(origin, 0u32);

        while let Some(current) = open_set.pop() {
            let current_pos = current.position();
            nodes_explored += 1;

            // Performance optimization: limit search space
            if nodes_explored > MAX_SEARCH_NODES {
                break;
            }

            // Check if we've reached the destination
            if current_pos == destination {
                return Some(Self::reconstruct_path(destination, &came_from));
            }

            closed_set.insert(current_pos);

            // Check all neighbors (cardinal + ordinal) with improved ordering
            for direction in Self::get_ordered_directions(current_pos, destination) {
                let coord = direction.coord();
                let neighbor_pos = (current_pos.0 + coord.0, current_pos.1 + coord.1);

                // Skip if already processed
                if closed_set.contains(&neighbor_pos) {
                    continue;
                }

                // Skip if not walkable
                if !provider.is_walkable(neighbor_pos, movement_type) {
                    continue;
                }

                // Calculate movement cost with terrain consideration
                let is_diagonal = direction.is_ordinal();
                let movement_cost = if is_diagonal { ORDINAL_COST } else { CARDINAL_COST };
                let terrain_cost = provider.cost(neighbor_pos, movement_type);

                if terrain_cost == u32::MAX {
                    continue; // Impassable
                }

                let tentative_g_score = g_score
                    .get(&current_pos)
                    .unwrap_or(&u32::MAX)
                    .saturating_add(movement_cost)
                    .saturating_add(terrain_cost);

                let current_g_score = g_score.get(&neighbor_pos).unwrap_or(&u32::MAX);

                if tentative_g_score < *current_g_score {
                    // This path is better
                    came_from.insert(neighbor_pos, current_pos);
                    g_score.insert(neighbor_pos, tentative_g_score);

                    // Use improved heuristic combining distance and direction
                    let h_score = Self::improved_heuristic(neighbor_pos, destination, current_pos);
                    let f_score = tentative_g_score.saturating_add(h_score);

                    let neighbor_node =
                        AStarNode::with_scores(neighbor_pos, tentative_g_score, h_score, f_score);

                    open_set.push(neighbor_node);
                }
            }
        }

        // No path found
        if partial_path_on_failure {
            Self::find_best_partial_path(&g_score, &came_from, destination)
        } else {
            None
        }
    }
}
