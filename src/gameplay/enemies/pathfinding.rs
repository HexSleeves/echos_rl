use bevy::prelude::*;
use big_brain::prelude::*;
use brtk::prelude::Direction;

use crate::{
    core::{
        actions::Walk,
        components::{PlayerTag, Position},
        pathfinding::{Pathfinder, PathfindingAlgorithm, PathResult},
        resources::{CurrentMap, FovMap, TurnQueue},
        types::{BuildableGameAction, GameActionBuilder},
    },
    gameplay::{
        enemies::components::{AIAction, AIBehavior, AIBehaviorType, AIState},
        turns::components::TurnActor,
    },
};

/// Enhanced pathfinding component for AI entities
#[derive(Component, Debug, Clone)]
pub struct AIPathfinding {
    pub current_path: Vec<Position>,
    pub target_position: Option<Position>,
    pub pathfinder: Pathfinder,
    pub recalculate_threshold: u32,
    pub last_recalculation: u64,
}

impl Default for AIPathfinding {
    fn default() -> Self {
        Self {
            current_path: Vec::new(),
            target_position: None,
            pathfinder: Pathfinder::new(PathfindingAlgorithm::AStar)
                .with_max_iterations(1000)
                .with_diagonal(false),
            recalculate_threshold: 5, // Recalculate path every 5 turns
            last_recalculation: 0,
        }
    }
}

impl AIPathfinding {
    pub fn new(algorithm: PathfindingAlgorithm) -> Self {
        Self {
            pathfinder: Pathfinder::new(algorithm)
                .with_max_iterations(1000)
                .with_diagonal(false),
            ..Default::default()
        }
    }

    pub fn with_diagonal(mut self, allow_diagonal: bool) -> Self {
        self.pathfinder = self.pathfinder.with_diagonal(allow_diagonal);
        self
    }

    pub fn with_max_iterations(mut self, max_iterations: usize) -> Self {
        self.pathfinder = self.pathfinder.with_max_iterations(max_iterations);
        self
    }

    /// Check if the current path is still valid
    pub fn is_path_valid(&self, current_pos: Position, map: &CurrentMap) -> bool {
        if self.current_path.is_empty() {
            return false;
        }

        // Check if we're still on the path
        if let Some(next_step_index) = self.current_path.iter().position(|&pos| pos == current_pos) {
            // Check if the remaining path is still walkable
            for i in (next_step_index + 1)..self.current_path.len() {
                let pos = self.current_path[i];
                if !map.is_walkable(pos) {
                    return false;
                }
                // Allow movement through target position even if it has an actor
                if self.target_position != Some(pos) && map.get_actor(pos).is_some() {
                    return false;
                }
            }
            return true;
        }

        false
    }

    /// Calculate a new path to the target
    pub fn calculate_path(
        &mut self,
        start: Position,
        target: Position,
        map: &CurrentMap,
        current_turn: u64,
    ) -> bool {
        self.target_position = Some(target);
        self.last_recalculation = current_turn;

        let result = self.pathfinder.find_path(start, target, map);
        
        if !result.is_empty() {
            self.current_path = result.path;
            true
        } else {
            // Try to find a path to a nearby position
            let nearby_result = self.pathfinder.find_path_to_nearest(start, target, map, 3);
            if !nearby_result.is_empty() {
                self.current_path = nearby_result.path;
                true
            } else {
                self.current_path.clear();
                false
            }
        }
    }

    /// Get the next step in the current path
    pub fn get_next_step(&self, current_pos: Position) -> Option<Position> {
        if let Some(current_index) = self.current_path.iter().position(|&pos| pos == current_pos) {
            self.current_path.get(current_index + 1).copied()
        } else {
            self.current_path.first().copied()
        }
    }

    /// Check if path needs recalculation
    pub fn needs_recalculation(&self, current_turn: u64) -> bool {
        current_turn.saturating_sub(self.last_recalculation) >= self.recalculate_threshold as u64
    }

    /// Clear the current path
    pub fn clear_path(&mut self) {
        self.current_path.clear();
        self.target_position = None;
    }
}

/// Enhanced chase action system using pathfinding
pub fn enhanced_chase_player_action_system(
    player_query: Query<&Position, With<PlayerTag>>,
    current_map: Res<CurrentMap>,
    turn_queue: Res<TurnQueue>,
    mut action_query: Query<(&Actor, &mut ActionState), With<crate::gameplay::enemies::components::ChasePlayerAction>>,
    mut ai_query: Query<(&Position, &mut TurnActor, &mut AIState, &mut AIPathfinding)>,
) {
    let Ok(player_pos) = player_query.single() else {
        return;
    };

    let current_turn = turn_queue.current_time();

    for (Actor(actor_entity), mut action_state) in action_query.iter_mut() {
        if let Ok((ai_pos, mut turn_actor, mut ai_state, mut pathfinding)) = ai_query.get_mut(*actor_entity) {
            match *action_state {
                ActionState::Init => {
                    *action_state = ActionState::Requested;
                }
                ActionState::Requested => {
                    info!("AI entity {:?} starting enhanced chase", actor_entity);
                    ai_state.current_action = AIAction::ChasePlayer;
                    *action_state = ActionState::Executing;
                }
                ActionState::Executing => {
                    // Check if we need to recalculate the path
                    let needs_new_path = pathfinding.target_position != Some(*player_pos)
                        || !pathfinding.is_path_valid(*ai_pos, &current_map)
                        || pathfinding.needs_recalculation(current_turn);

                    if needs_new_path {
                        if pathfinding.calculate_path(*ai_pos, *player_pos, &current_map, current_turn) {
                            info!("AI entity {:?} calculated new path to player", actor_entity);
                        } else {
                            info!("AI entity {:?} failed to find path to player", actor_entity);
                            *action_state = ActionState::Failure;
                            continue;
                        }
                    }

                    // Get the next step
                    if let Some(next_pos) = pathfinding.get_next_step(*ai_pos) {
                        // Convert position to direction
                        if let Some(direction) = position_to_direction(*ai_pos, next_pos) {
                            turn_actor.add_action(
                                Walk::builder()
                                    .with_entity(*actor_entity)
                                    .with_direction(direction)
                                    .build(),
                            );
                            
                            // Check if we've reached the target
                            if next_pos == *player_pos {
                                *action_state = ActionState::Success;
                            }
                        } else {
                            info!("AI entity {:?} cannot determine direction to next step", actor_entity);
                            *action_state = ActionState::Failure;
                        }
                    } else {
                        info!("AI entity {:?} has no next step in path", actor_entity);
                        *action_state = ActionState::Failure;
                    }
                }
                ActionState::Success | ActionState::Failure => {
                    pathfinding.clear_path();
                    *action_state = ActionState::Init;
                }
                ActionState::Cancelled => {
                    pathfinding.clear_path();
                    *action_state = ActionState::Init;
                }
            }
        }
    }
}

/// Enhanced flee action system using pathfinding
pub fn enhanced_flee_from_player_action_system(
    player_query: Query<&Position, With<PlayerTag>>,
    current_map: Res<CurrentMap>,
    turn_queue: Res<TurnQueue>,
    mut action_query: Query<(&Actor, &mut ActionState), With<crate::gameplay::enemies::components::FleeFromPlayerAction>>,
    mut ai_query: Query<(&Position, &mut TurnActor, &mut AIState, &mut AIPathfinding)>,
) {
    let Ok(player_pos) = player_query.single() else {
        return;
    };

    let current_turn = turn_queue.current_time();

    for (Actor(actor_entity), mut action_state) in action_query.iter_mut() {
        if let Ok((ai_pos, mut turn_actor, mut ai_state, mut pathfinding)) = ai_query.get_mut(*actor_entity) {
            match *action_state {
                ActionState::Init => {
                    *action_state = ActionState::Requested;
                }
                ActionState::Requested => {
                    info!("AI entity {:?} starting enhanced flee", actor_entity);
                    ai_state.current_action = AIAction::FleeFromPlayer;
                    *action_state = ActionState::Executing;
                }
                ActionState::Executing => {
                    // Find a safe position to flee to
                    let flee_target = find_flee_target(*ai_pos, *player_pos, &current_map, 8);
                    
                    if let Some(target) = flee_target {
                        let needs_new_path = pathfinding.target_position != Some(target)
                            || !pathfinding.is_path_valid(*ai_pos, &current_map)
                            || pathfinding.needs_recalculation(current_turn);

                        if needs_new_path {
                            if pathfinding.calculate_path(*ai_pos, target, &current_map, current_turn) {
                                info!("AI entity {:?} calculated flee path", actor_entity);
                            } else {
                                // If we can't find a path, just move in any direction away from player
                                if let Some(direction) = find_flee_direction(*ai_pos, *player_pos, &current_map) {
                                    turn_actor.add_action(
                                        Walk::builder()
                                            .with_entity(*actor_entity)
                                            .with_direction(direction)
                                            .build(),
                                    );
                                }
                                *action_state = ActionState::Success;
                                continue;
                            }
                        }

                        // Follow the flee path
                        if let Some(next_pos) = pathfinding.get_next_step(*ai_pos) {
                            if let Some(direction) = position_to_direction(*ai_pos, next_pos) {
                                turn_actor.add_action(
                                    Walk::builder()
                                        .with_entity(*actor_entity)
                                        .with_direction(direction)
                                        .build(),
                                );
                                
                                // Check if we've reached a safe distance
                                let distance_to_player = crate::utils::calculate_distance(next_pos, *player_pos);
                                if distance_to_player > 6.0 {
                                    *action_state = ActionState::Success;
                                }
                            } else {
                                *action_state = ActionState::Failure;
                            }
                        } else {
                            *action_state = ActionState::Failure;
                        }
                    } else {
                        // No flee target found, just try to move away
                        if let Some(direction) = find_flee_direction(*ai_pos, *player_pos, &current_map) {
                            turn_actor.add_action(
                                Walk::builder()
                                    .with_entity(*actor_entity)
                                    .with_direction(direction)
                                    .build(),
                            );
                        }
                        *action_state = ActionState::Success;
                    }
                }
                ActionState::Success | ActionState::Failure => {
                    pathfinding.clear_path();
                    *action_state = ActionState::Init;
                }
                ActionState::Cancelled => {
                    pathfinding.clear_path();
                    *action_state = ActionState::Init;
                }
            }
        }
    }
}

/// Convert a position difference to a direction
fn position_to_direction(from: Position, to: Position) -> Option<Direction> {
    let dx = to.x() - from.x();
    let dy = to.y() - from.y();

    match (dx.signum(), dy.signum()) {
        (0, -1) => Some(Direction::NORTH),
        (1, 0) => Some(Direction::EAST),
        (0, 1) => Some(Direction::SOUTH),
        (-1, 0) => Some(Direction::WEST),
        _ => None, // Invalid or diagonal movement
    }
}

/// Find a good position to flee to
fn find_flee_target(from: Position, threat: Position, map: &CurrentMap, max_distance: i32) -> Option<Position> {
    let mut best_position = None;
    let mut best_score = 0.0;

    for dx in -max_distance..=max_distance {
        for dy in -max_distance..=max_distance {
            let candidate = Position::new(from.x() + dx, from.y() + dy);
            
            if !map.in_bounds(candidate) || !map.is_walkable(candidate) {
                continue;
            }

            if map.get_actor(candidate).is_some() {
                continue;
            }

            let distance_from_threat = crate::utils::calculate_distance(candidate, threat);
            let distance_from_self = crate::utils::calculate_distance(candidate, from);
            
            // Score based on distance from threat and reasonable distance from self
            let score = distance_from_threat - (distance_from_self * 0.1);
            
            if score > best_score {
                best_score = score;
                best_position = Some(candidate);
            }
        }
    }

    best_position
}

/// Find a direction to flee in (fallback when pathfinding fails)
fn find_flee_direction(from: Position, threat: Position, map: &CurrentMap) -> Option<Direction> {
    let directions = [Direction::NORTH, Direction::EAST, Direction::SOUTH, Direction::WEST];
    let mut best_direction = None;
    let mut best_distance = 0.0;

    for &direction in &directions {
        let (dx, dy) = direction.coord();
        let test_pos = from + (dx, dy);
        
        if map.is_walkable(test_pos) && map.get_actor(test_pos).is_none() {
            let distance = crate::utils::calculate_distance(test_pos, threat);
            if distance > best_distance {
                best_distance = distance;
                best_direction = Some(direction);
            }
        }
    }

    best_direction
}
