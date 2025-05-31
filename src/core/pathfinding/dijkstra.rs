use std::collections::{BinaryHeap, HashMap, HashSet};

use crate::core::{components::Position, resources::CurrentMap};
use super::{PathNode, PathResult, get_neighbors, reconstruct_path};

/// Dijkstra's pathfinding algorithm implementation
/// Useful when you need to find paths to multiple goals or when heuristics are unreliable
pub fn find_path(
    start: Position,
    goal: Position,
    map: &CurrentMap,
    max_iterations: usize,
    allow_diagonal: bool,
) -> PathResult {
    if start == goal {
        return PathResult {
            path: vec![start],
            cost: 0,
            nodes_explored: 0,
        };
    }

    if !map.in_bounds(start) || !map.in_bounds(goal) {
        return PathResult::empty();
    }

    if !map.is_walkable(goal) {
        return PathResult::empty();
    }

    let mut open_set = BinaryHeap::new();
    let mut closed_set = HashSet::new();
    let mut came_from = HashMap::new();
    let mut distances = HashMap::new();
    let mut nodes_explored = 0;

    // Initialize start node (heuristic is 0 for Dijkstra)
    open_set.push(PathNode::new(start, 0, 0));
    distances.insert(start, 0);

    while let Some(current_node) = open_set.pop() {
        nodes_explored += 1;

        if nodes_explored > max_iterations {
            break;
        }

        let current_pos = current_node.position;

        if current_pos == goal {
            let path = reconstruct_path(&came_from, start, goal);
            return PathResult {
                path,
                cost: current_node.cost,
                nodes_explored,
            };
        }

        if closed_set.contains(&current_pos) {
            continue;
        }

        closed_set.insert(current_pos);

        // Explore neighbors
        for neighbor_pos in get_neighbors(current_pos, allow_diagonal) {
            if !map.in_bounds(neighbor_pos) {
                continue;
            }

            if !map.is_walkable(neighbor_pos) {
                continue;
            }

            // Check if there's an actor at this position (but allow goal position)
            if neighbor_pos != goal && map.get_actor(neighbor_pos).is_some() {
                continue;
            }

            if closed_set.contains(&neighbor_pos) {
                continue;
            }

            // Calculate movement cost
            let movement_cost = if allow_diagonal && is_diagonal_move(current_pos, neighbor_pos) {
                14 // Diagonal movement cost (approximately sqrt(2) * 10)
            } else {
                10 // Cardinal movement cost
            };

            let tentative_distance = current_node.cost + movement_cost;

            // Check if this path to neighbor is better than any previous one
            if let Some(&existing_distance) = distances.get(&neighbor_pos) {
                if tentative_distance >= existing_distance {
                    continue;
                }
            }

            // This path is the best until now. Record it!
            came_from.insert(neighbor_pos, current_pos);
            distances.insert(neighbor_pos, tentative_distance);

            // For Dijkstra, heuristic is always 0
            let neighbor_node = PathNode::new(neighbor_pos, tentative_distance, 0);
            open_set.push(neighbor_node);
        }
    }

    // No path found
    PathResult {
        path: Vec::new(),
        cost: 0,
        nodes_explored,
    }
}

/// Find the shortest path to any of multiple goals
pub fn find_path_to_any(
    start: Position,
    goals: &[Position],
    map: &CurrentMap,
    max_iterations: usize,
    allow_diagonal: bool,
) -> PathResult {
    if goals.is_empty() {
        return PathResult::empty();
    }

    if goals.contains(&start) {
        return PathResult {
            path: vec![start],
            cost: 0,
            nodes_explored: 0,
        };
    }

    if !map.in_bounds(start) {
        return PathResult::empty();
    }

    let goals_set: HashSet<Position> = goals.iter().copied().collect();

    let mut open_set = BinaryHeap::new();
    let mut closed_set = HashSet::new();
    let mut came_from = HashMap::new();
    let mut distances = HashMap::new();
    let mut nodes_explored = 0;

    // Initialize start node
    open_set.push(PathNode::new(start, 0, 0));
    distances.insert(start, 0);

    while let Some(current_node) = open_set.pop() {
        nodes_explored += 1;

        if nodes_explored > max_iterations {
            break;
        }

        let current_pos = current_node.position;

        if goals_set.contains(&current_pos) {
            let path = reconstruct_path(&came_from, start, current_pos);
            return PathResult {
                path,
                cost: current_node.cost,
                nodes_explored,
            };
        }

        if closed_set.contains(&current_pos) {
            continue;
        }

        closed_set.insert(current_pos);

        // Explore neighbors
        for neighbor_pos in get_neighbors(current_pos, allow_diagonal) {
            if !map.in_bounds(neighbor_pos) {
                continue;
            }

            if !map.is_walkable(neighbor_pos) {
                continue;
            }

            // Allow movement to goal positions even if they have actors
            if !goals_set.contains(&neighbor_pos) && map.get_actor(neighbor_pos).is_some() {
                continue;
            }

            if closed_set.contains(&neighbor_pos) {
                continue;
            }

            let movement_cost = if allow_diagonal && is_diagonal_move(current_pos, neighbor_pos) {
                14
            } else {
                10
            };

            let tentative_distance = current_node.cost + movement_cost;

            if let Some(&existing_distance) = distances.get(&neighbor_pos) {
                if tentative_distance >= existing_distance {
                    continue;
                }
            }

            came_from.insert(neighbor_pos, current_pos);
            distances.insert(neighbor_pos, tentative_distance);

            let neighbor_node = PathNode::new(neighbor_pos, tentative_distance, 0);
            open_set.push(neighbor_node);
        }
    }

    PathResult {
        path: Vec::new(),
        cost: 0,
        nodes_explored,
    }
}

/// Check if a move is diagonal
fn is_diagonal_move(from: Position, to: Position) -> bool {
    let dx = (to.x() - from.x()).abs();
    let dy = (to.y() - from.y()).abs();
    dx == 1 && dy == 1
}
