use bevy::prelude::*;
use big_brain::prelude::*;
use brtk::prelude::Direction;

use crate::{
    gameplay::{
        enemies::components::{
            AIAction, AIBehavior, AIBehaviorType, AIState, ChasePlayerAction, ChasePlayerScorer,
            FleeFromPlayerAction, FleeFromPlayerScorer, IdleAction, WanderAction, WanderScorer,
        },
        turns::{components::TurnActor, resources::TurnQueue},
    },
    model::{
        actions::Walk,
        components::{PlayerTag, Position},
        resources::{CurrentMap, FovMap},
        types::{BuildableGameAction, GameActionBuilder},
    },
};

// ============================================================================
// SCORER SYSTEMS (Evaluate what the AI should do)
// ============================================================================

/// System that scores how much an AI wants to chase the player
pub fn chase_player_scorer_system(
    player_query: Query<&Position, With<PlayerTag>>,
    fov_map: Res<FovMap>,
    turn_queue: Res<TurnQueue>,
    mut scorer_query: Query<(&Actor, &mut Score), With<ChasePlayerScorer>>,
    mut ai_query: Query<(&Position, &mut AIBehavior, &TurnActor)>,
) {
    let Ok(player_pos) = player_query.single() else {
        return;
    };

    let current_turn = turn_queue.current_time();

    for (Actor(actor_entity), mut score) in scorer_query.iter_mut() {
        if let Ok((ai_pos, mut ai_behavior, turn_actor)) = ai_query.get_mut(*actor_entity) {
            // Skip scoring if this entity already has an action queued
            if turn_actor.peak_next_action().is_some() {
                continue;
            }

            let mut chase_score = 0.0;

            // Only hostile enemies want to chase
            if ai_behavior.behavior_type == AIBehaviorType::Hostile {
                // Check if AI can see the player
                if fov_map.is_visible(*player_pos) {
                    let distance = crate::utils::calculate_distance(*ai_pos, *player_pos);

                    if distance <= ai_behavior.detection_range as f32 {
                        // Update AI's knowledge of player position
                        ai_behavior.update_player_sighting(*player_pos, current_turn);

                        // Higher score for closer players
                        chase_score = 1.0 - (distance / ai_behavior.detection_range as f32);
                        chase_score = chase_score.clamp(0.0, 1.0);

                        info!(
                            "AI entity {:?} can see player at distance {:.1}, chase score: {:.2}",
                            actor_entity, distance, chase_score
                        );
                    }
                } else if let Some(last_known_pos) = ai_behavior.last_known_player_position {
                    // Player not visible, but AI remembers where they were
                    if !ai_behavior.should_switch_to_wander(current_turn) {
                        let distance = crate::utils::calculate_distance(*ai_pos, last_known_pos);
                        // Lower score for remembered positions
                        chase_score = 0.3 * (1.0 - (distance / ai_behavior.detection_range as f32));
                        chase_score = chase_score.clamp(0.0, 0.5);
                    }
                }
            }

            score.set(chase_score);
        }
    }
}

/// System that scores how much an AI wants to flee from the player
pub fn flee_from_player_scorer_system(
    player_query: Query<&Position, With<PlayerTag>>,
    fov_map: Res<FovMap>,
    mut scorer_query: Query<(&Actor, &mut Score), With<FleeFromPlayerScorer>>,
    ai_query: Query<(&Position, &AIBehavior, &TurnActor)>,
) {
    let Ok(player_pos) = player_query.single() else {
        return;
    };

    for (Actor(actor_entity), mut score) in scorer_query.iter_mut() {
        if let Ok((ai_pos, ai_behavior, turn_actor)) = ai_query.get(*actor_entity) {
            // Skip scoring if this entity already has an action queued
            if turn_actor.peak_next_action().is_some() {
                continue;
            }

            let mut flee_score = 0.0;

            // Only passive enemies want to flee
            if ai_behavior.behavior_type == AIBehaviorType::Passive {
                // Check if AI can see the player
                if fov_map.is_visible(*player_pos) {
                    let distance = crate::utils::calculate_distance(*ai_pos, *player_pos);

                    // Higher score for closer players (more urgent to flee)
                    if distance <= ai_behavior.detection_range as f32 {
                        flee_score = 1.0 - (distance / ai_behavior.detection_range as f32);
                        flee_score = flee_score.clamp(0.0, 1.0);
                    }
                }
            }

            score.set(flee_score);
        }
    }
}

/// System that scores how much an AI wants to wander
pub fn wander_scorer_system(
    turn_queue: Res<TurnQueue>,
    mut scorer_query: Query<(&Actor, &mut Score), With<WanderScorer>>,
    ai_query: Query<(&AIBehavior, &TurnActor)>,
) {
    let current_turn = turn_queue.current_time();

    for (Actor(actor_entity), mut score) in scorer_query.iter_mut() {
        if let Ok((ai_behavior, turn_actor)) = ai_query.get(*actor_entity) {
            // Skip scoring if this entity already has an action queued
            if turn_actor.peak_next_action().is_some() {
                continue;
            }

            let wander_score = match ai_behavior.behavior_type {
                AIBehaviorType::Neutral => {
                    // Neutral entities should always want to wander
                    0.2 + fastrand::f32() * 0.3 // 0.2 to 0.5 - always above threshold
                }
                AIBehaviorType::Hostile => {
                    if ai_behavior.should_switch_to_wander(current_turn) {
                        // Hostile enemy hasn't seen player for a while, switch to wandering
                        info!(
                            "Hostile AI entity {:?} switching to wander (no player seen for {} turns)",
                            actor_entity,
                            current_turn.saturating_sub(ai_behavior.last_player_seen_turn.unwrap_or(0))
                        );
                        0.7 // High score to override chase behavior
                    } else {
                        // Low wander score when actively hostile
                        0.15 + fastrand::f32() * 0.1 // 0.15 to 0.25
                    }
                }
                AIBehaviorType::Passive => {
                    // Passive entities wander when not fleeing
                    0.3 + fastrand::f32() * 0.2 // 0.3 to 0.5
                }
            };

            score.set(wander_score);
        }
    }
}

// ============================================================================
// ACTION SYSTEMS (What the AI actually does)
// ============================================================================

/// System that handles chasing the player
pub fn chase_player_action_system(
    player_query: Query<&Position, With<PlayerTag>>,
    current_map: Res<CurrentMap>,
    mut action_query: Query<(&Actor, &mut ActionState), With<ChasePlayerAction>>,
    mut ai_query: Query<(&Position, &mut TurnActor, &mut AIState)>,
) {
    let Ok(player_pos) = player_query.single() else {
        return;
    };

    for (Actor(actor_entity), mut action_state) in action_query.iter_mut() {
        if let Ok((ai_pos, mut turn_actor, mut ai_state)) = ai_query.get_mut(*actor_entity) {
            match *action_state {
                ActionState::Init => {
                    // Initialize the action
                    *action_state = ActionState::Requested;
                }
                ActionState::Requested => {
                    info!(
                        "AI entity {:?} performing chase action toward player at {:?}",
                        actor_entity, player_pos
                    );
                    ai_state.current_action = AIAction::ChasePlayer;
                    ai_state.target_position = Some(*player_pos);

                    // Calculate direction toward player
                    let direction = calculate_direction_to_target(*ai_pos, *player_pos);

                    if let Some(dir) = direction {
                        // Check if the direct path is walkable
                        let (dx, dy) = dir.coord();
                        let target_pos = *ai_pos + (dx, dy);
                        if current_map.is_walkable(target_pos) {
                            turn_actor.add_action(
                                Walk::builder().with_entity(*actor_entity).with_direction(dir).build(),
                            );
                            *action_state = ActionState::Success;
                        } else {
                            // Try alternative directions if direct path is blocked
                            if let Some(alt_dir) =
                                find_alternative_direction(*ai_pos, *player_pos, &current_map)
                            {
                                turn_actor.add_action(
                                    Walk::builder()
                                        .with_entity(*actor_entity)
                                        .with_direction(alt_dir)
                                        .build(),
                                );
                                *action_state = ActionState::Success;
                            } else {
                                info!(
                                    "AI entity {:?} cannot find path to player, action failed",
                                    actor_entity
                                );
                                *action_state = ActionState::Failure;
                            }
                        }
                    } else {
                        // Already at player position or no valid direction
                        *action_state = ActionState::Success;
                    }
                }
                ActionState::Executing => {
                    // Action is being executed, wait for completion
                }
                ActionState::Success | ActionState::Failure => {
                    // Action completed, reset for next frame
                    *action_state = ActionState::Requested;
                }
                ActionState::Cancelled => {
                    // Action was cancelled, reset to init
                    *action_state = ActionState::Init;
                }
            }
        }
    }
}

/// System that handles fleeing from the player
pub fn flee_from_player_action_system(
    player_query: Query<&Position, With<PlayerTag>>,
    current_map: Res<CurrentMap>,
    mut action_query: Query<(&Actor, &mut ActionState), With<FleeFromPlayerAction>>,
    mut ai_query: Query<(&Position, &mut TurnActor, &mut AIState)>,
) {
    let Ok(player_pos) = player_query.single() else {
        return;
    };

    for (Actor(actor_entity), mut action_state) in action_query.iter_mut() {
        if let Ok((ai_pos, mut turn_actor, mut ai_state)) = ai_query.get_mut(*actor_entity) {
            match *action_state {
                ActionState::Init => {
                    *action_state = ActionState::Requested;
                }
                ActionState::Requested => {
                    info!(
                        "AI entity {:?} performing flee action away from player at {:?}",
                        actor_entity, player_pos
                    );
                    ai_state.current_action = AIAction::FleeFromPlayer;

                    // Calculate direction away from player
                    let direction = calculate_direction_away_from_target(*ai_pos, *player_pos);

                    if let Some(dir) = direction {
                        // Check if the flee path is walkable
                        let (dx, dy) = dir.coord();
                        let target_pos = *ai_pos + (dx, dy);
                        if current_map.is_walkable(target_pos) {
                            turn_actor.add_action(
                                Walk::builder().with_entity(*actor_entity).with_direction(dir).build(),
                            );
                            *action_state = ActionState::Success;
                        } else {
                            // Try alternative directions if direct flee path is blocked
                            if let Some(alt_dir) =
                                find_alternative_flee_direction(*ai_pos, *player_pos, &current_map)
                            {
                                turn_actor.add_action(
                                    Walk::builder()
                                        .with_entity(*actor_entity)
                                        .with_direction(alt_dir)
                                        .build(),
                                );
                                *action_state = ActionState::Success;
                            } else {
                                info!("AI entity {:?} cannot find flee path, action failed", actor_entity);
                                *action_state = ActionState::Failure;
                            }
                        }
                    } else {
                        *action_state = ActionState::Failure;
                    }
                }
                ActionState::Executing => {
                    // Action is being executed, wait for completion
                }
                ActionState::Success | ActionState::Failure => {
                    // Action completed, reset for next frame
                    *action_state = ActionState::Requested;
                }
                ActionState::Cancelled => {
                    *action_state = ActionState::Init;
                }
            }
        }
    }
}

/// System that handles random wandering
pub fn wander_action_system(
    current_map: Res<CurrentMap>,
    mut action_query: Query<(&Actor, &mut ActionState), With<WanderAction>>,
    mut ai_query: Query<(&Position, &mut TurnActor, &mut AIState)>,
) {
    for (Actor(actor_entity), mut action_state) in action_query.iter_mut() {
        if let Ok((ai_pos, mut turn_actor, mut ai_state)) = ai_query.get_mut(*actor_entity) {
            match *action_state {
                ActionState::Init => {
                    *action_state = ActionState::Requested;
                }
                ActionState::Requested => {
                    info!("AI entity {:?} performing wander action at {:?}", actor_entity, ai_pos);
                    ai_state.current_action = AIAction::Wander;

                    // Find a random walkable direction
                    if let Some(direction) = find_random_walkable_direction(*ai_pos, &current_map) {
                        turn_actor.add_action(
                            Walk::builder().with_entity(*actor_entity).with_direction(direction).build(),
                        );
                        *action_state = ActionState::Success;
                    } else {
                        info!("AI entity {:?} cannot find walkable direction for wandering", actor_entity);
                        *action_state = ActionState::Failure;
                    }
                }
                ActionState::Executing => {
                    // Action is being executed, wait for completion
                }
                ActionState::Success | ActionState::Failure => {
                    // Action completed, reset for next frame
                    *action_state = ActionState::Requested;
                }
                ActionState::Cancelled => {
                    *action_state = ActionState::Init;
                }
            }
        }
    }
}

/// System that handles idle behavior
pub fn idle_action_system(
    mut action_query: Query<(&Actor, &mut ActionState), With<IdleAction>>,
    mut ai_query: Query<&mut AIState>,
) {
    for (Actor(actor_entity), mut action_state) in action_query.iter_mut() {
        if let Ok(mut ai_state) = ai_query.get_mut(*actor_entity) {
            match *action_state {
                ActionState::Init => {
                    *action_state = ActionState::Requested;
                }
                ActionState::Requested => {
                    info!("AI entity {:?} performing idle action", actor_entity);
                    ai_state.current_action = AIAction::Idle;
                    *action_state = ActionState::Success;
                }
                ActionState::Executing => {
                    // Action is being executed, wait for completion
                }
                ActionState::Success | ActionState::Failure => {
                    // Action completed, reset for next frame
                    *action_state = ActionState::Requested;
                }
                ActionState::Cancelled => {
                    *action_state = ActionState::Init;
                }
            }
        }
    }
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

/// Calculate the direction to move toward a target
fn calculate_direction_to_target(from: Position, to: Position) -> Option<Direction> {
    let diff = to.0 - from.0;

    if diff.x == 0 && diff.y == 0 {
        return None; // Already at target
    }

    // Prioritize the axis with the larger difference
    if diff.x.abs() > diff.y.abs() {
        if diff.x > 0 { Some(Direction::EAST) } else { Some(Direction::WEST) }
    } else {
        if diff.y > 0 { Some(Direction::SOUTH) } else { Some(Direction::NORTH) }
    }
}

/// Calculate the direction to move away from a target
fn calculate_direction_away_from_target(from: Position, away_from: Position) -> Option<Direction> {
    let diff = from.0 - away_from.0;

    if diff.x == 0 && diff.y == 0 {
        // At same position, pick a random direction
        let directions: Vec<Direction> = Direction::iter_cardinal().collect();
        return Some(directions[fastrand::usize(..directions.len())]);
    }

    // Prioritize the axis with the larger difference
    if diff.x.abs() > diff.y.abs() {
        if diff.x > 0 { Some(Direction::EAST) } else { Some(Direction::WEST) }
    } else {
        if diff.y > 0 { Some(Direction::SOUTH) } else { Some(Direction::NORTH) }
    }
}

/// Find an alternative direction when the direct path is blocked
fn find_alternative_direction(from: Position, to: Position, map: &CurrentMap) -> Option<Direction> {
    let directions: Vec<Direction> = Direction::iter_cardinal().collect();

    // Try all directions and pick the one that gets us closest to the target
    let mut best_direction = None;
    let mut best_distance = f32::INFINITY;

    for &dir in &directions {
        let (dx, dy) = dir.coord();
        let test_pos = from + (dx, dy);
        if map.is_walkable(test_pos) {
            let distance = crate::utils::calculate_distance(test_pos, to);
            if distance < best_distance {
                best_distance = distance;
                best_direction = Some(dir);
            }
        }
    }

    best_direction
}

/// Find an alternative direction for fleeing when the direct path is blocked
fn find_alternative_flee_direction(
    from: Position,
    away_from: Position,
    map: &CurrentMap,
) -> Option<Direction> {
    let directions: Vec<Direction> = Direction::iter_cardinal().collect();

    // Try all directions and pick the one that gets us furthest from the target
    let mut best_direction = None;
    let mut best_distance = 0.0;

    for &dir in &directions {
        let (dx, dy) = dir.coord();
        let test_pos = from + (dx, dy);
        if map.is_walkable(test_pos) {
            let distance = crate::utils::calculate_distance(test_pos, away_from);
            if distance > best_distance {
                best_distance = distance;
                best_direction = Some(dir);
            }
        }
    }

    best_direction
}

/// Find a random walkable direction
fn find_random_walkable_direction(from: Position, map: &CurrentMap) -> Option<Direction> {
    let directions: Vec<Direction> = Direction::iter_cardinal().collect();
    let mut walkable_directions = Vec::new();

    for &dir in &directions {
        let (dx, dy) = dir.coord();
        let test_pos = from + (dx, dy);
        if map.is_walkable(test_pos) {
            walkable_directions.push(dir);
        }
    }

    if walkable_directions.is_empty() {
        None
    } else {
        Some(walkable_directions[fastrand::usize(..walkable_directions.len())])
    }
}
