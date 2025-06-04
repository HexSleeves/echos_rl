//! A* pathfinding algorithm implementation
//!
//! This module provides an A* pathfinding implementation adapted to work with
//! the brtk direction and grid systems.

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

/// A* pathfinding algorithm implementation
pub struct AStar;

impl PathAlgorithm for AStar {
    fn compute_path<P: PathProvider>(
        origin: (i32, i32),
        destination: (i32, i32),
        movement_type: u8,
        partial_path_on_failure: bool,
        provider: &mut P,
    ) -> Option<Vec<(i32, i32)>> {
        // Use standard collections for a simpler, more reliable implementation
        let mut open_set = BinaryHeap::new();
        let mut closed_set = HashSet::new();
        let mut came_from = HashMap::new();
        let mut g_score = HashMap::new();

        // Initialize starting node
        let start_node = AStarNode::new(origin, destination);
        open_set.push(start_node);
        g_score.insert(origin, 0u32);

        while let Some(current) = open_set.pop() {
            let current_pos = current.position();

            if current_pos == destination {
                // Reconstruct path
                return Some(Self::reconstruct_path_simple(destination, &came_from));
            }

            closed_set.insert(current_pos);

            // Check all neighbors (cardinal + ordinal)
            for direction in Direction::iter_cardinal_ordinal() {
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

                // Calculate movement cost
                let is_diagonal = direction.is_ordinal();
                let movement_cost = if is_diagonal { ORDINAL_COST } else { CARDINAL_COST };
                let terrain_cost = provider.cost(neighbor_pos, movement_type);

                if terrain_cost == u32::MAX {
                    continue; // Impassable
                }

                let tentative_g_score = g_score
                    .get(&current_pos)
                    .unwrap_or(&u32::MAX)
                    .saturating_add(movement_cost.saturating_mul(terrain_cost));

                let current_g_score = g_score.get(&neighbor_pos).unwrap_or(&u32::MAX);

                if tentative_g_score < *current_g_score {
                    // This path is better
                    came_from.insert(neighbor_pos, current_pos);
                    g_score.insert(neighbor_pos, tentative_g_score);

                    let h_score = euclidean_distance(neighbor_pos, destination);
                    let f_score = tentative_g_score.saturating_add(h_score);

                    let neighbor_node =
                        AStarNode::with_scores(neighbor_pos, tentative_g_score, h_score, f_score);

                    open_set.push(neighbor_node);
                }
            }
        }

        // No path found
        if partial_path_on_failure {
            Self::find_best_partial_path_simple(&g_score, &came_from, destination)
        } else {
            None
        }
    }
}

impl AStar {
    /// Reconstruct path from destination back to origin
    fn reconstruct_path_simple(
        destination: (i32, i32),
        came_from: &HashMap<(i32, i32), (i32, i32)>,
    ) -> Vec<(i32, i32)> {
        let mut path = Vec::new();
        let mut current = destination;
        path.push(current);

        while let Some(&parent) = came_from.get(&current) {
            current = parent;
            path.push(current);
        }

        path.reverse();
        path
    }

    /// Find the best partial path when full path fails
    fn find_best_partial_path_simple(
        g_score: &HashMap<(i32, i32), u32>,
        came_from: &HashMap<(i32, i32), (i32, i32)>,
        destination: (i32, i32),
    ) -> Option<Vec<(i32, i32)>> {
        // Find the position closest to the destination
        let mut best_pos = None;
        let mut best_distance = u32::MAX;

        for (&pos, &_score) in g_score {
            let distance = euclidean_distance(pos, destination);
            if distance < best_distance {
                best_distance = distance;
                best_pos = Some(pos);
            }
        }

        // Reconstruct the full path from origin to the best position found
        best_pos.map(|pos| Self::reconstruct_path_simple(pos, came_from))
    }
}

/// A* pathfinding node
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
}

// Implement ordering for priority queue behavior (BinaryHeap is a max-heap, so we reverse the
// ordering)
impl Ord for AStarNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Reverse ordering for min-heap behavior (lower f_score is better)
        other.f_score.cmp(&self.f_score).then_with(|| other.h_score.cmp(&self.h_score))
    }
}

impl PartialOrd for AStarNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
