use std::collections::{BinaryHeap, HashMap, HashSet};

use crate::core::{components::Position, resources::CurrentMap};
use super::{PathNode, PathResult, get_neighbors, manhattan_distance, reconstruct_path};

/// A* pathfinding algorithm implementation
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
    let mut g_score = HashMap::new();
    let mut nodes_explored = 0;

    // Initialize start node
    let start_heuristic = manhattan_distance(start, goal);
    open_set.push(PathNode::new(start, 0, start_heuristic));
    g_score.insert(start, 0);

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

            let tentative_g_score = current_node.cost + movement_cost;

            // Check if this path to neighbor is better than any previous one
            if let Some(&existing_g_score) = g_score.get(&neighbor_pos) {
                if tentative_g_score >= existing_g_score {
                    continue;
                }
            }

            // This path is the best until now. Record it!
            came_from.insert(neighbor_pos, current_pos);
            g_score.insert(neighbor_pos, tentative_g_score);

            let heuristic = manhattan_distance(neighbor_pos, goal);
            let neighbor_node = PathNode::new(neighbor_pos, tentative_g_score, heuristic);
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

/// Check if a move is diagonal
fn is_diagonal_move(from: Position, to: Position) -> bool {
    let dx = (to.x() - from.x()).abs();
    let dy = (to.y() - from.y()).abs();
    dx == 1 && dy == 1
}

/// A* with early termination for partial paths
pub fn find_partial_path(
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

    let mut open_set = BinaryHeap::new();
    let mut closed_set = HashSet::new();
    let mut came_from = HashMap::new();
    let mut g_score = HashMap::new();
    let mut nodes_explored = 0;
    let mut best_node = None;
    let mut best_distance = f32::INFINITY;

    // Initialize start node
    let start_heuristic = manhattan_distance(start, goal);
    open_set.push(PathNode::new(start, 0, start_heuristic));
    g_score.insert(start, 0);

    while let Some(current_node) = open_set.pop() {
        nodes_explored += 1;

        if nodes_explored > max_iterations {
            break;
        }

        let current_pos = current_node.position;

        // Check if this is the closest we've gotten to the goal
        let distance_to_goal = super::calculate_distance(current_pos, goal);
        if distance_to_goal < best_distance {
            best_distance = distance_to_goal;
            best_node = Some(current_pos);
        }

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

            // For partial paths, we're more lenient about actors
            if map.get_actor(neighbor_pos).is_some() {
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

            let tentative_g_score = current_node.cost + movement_cost;

            if let Some(&existing_g_score) = g_score.get(&neighbor_pos) {
                if tentative_g_score >= existing_g_score {
                    continue;
                }
            }

            came_from.insert(neighbor_pos, current_pos);
            g_score.insert(neighbor_pos, tentative_g_score);

            let heuristic = manhattan_distance(neighbor_pos, goal);
            let neighbor_node = PathNode::new(neighbor_pos, tentative_g_score, heuristic);
            open_set.push(neighbor_node);
        }
    }

    // Return partial path to the closest position we found
    if let Some(best_pos) = best_node {
        if best_pos != start {
            let path = reconstruct_path(&came_from, start, best_pos);
            if !path.is_empty() {
                return PathResult {
                    path,
                    cost: g_score.get(&best_pos).copied().unwrap_or(0),
                    nodes_explored,
                };
            }
        }
    }

    PathResult {
        path: Vec::new(),
        cost: 0,
        nodes_explored,
    }
}
