use bevy::prelude::*;
use big_brain::prelude::*;
use brtk::prelude::Direction;

use crate::{
    core::{
        components::{PlayerTag, Position},
        pathfinding,
        resources::{CurrentMap, FovMap, TurnQueue},
        types::ActionType,
    },
    gameplay::{
        enemies::{
            components::{AIAction, AIBehavior, AIState, ChasePlayerAction, ChasePlayerScorer},
            helpers,
        },
        turns::components::TurnActor,
    },
    prelude::assets::AIBehaviorType,
};

// ============================================================================
// SCORER SYSTEMS (Evaluate what the AI should do)
// ============================================================================

/// System that scores how much an AI wants to chase the player
pub fn chase_player_scorer_system(
    turn_queue: Res<TurnQueue>,
    current_map: Res<CurrentMap>,
    player_query: Query<&Position, With<PlayerTag>>,
    mut ai_query: Query<(&Position, &mut AIBehavior)>,
    mut scorer_query: Query<(&Actor, &mut Score), With<ChasePlayerScorer>>,
) {
    let Ok(player_pos) = player_query.single() else {
        return;
    };

    let current_turn = turn_queue.current_time();

    for (Actor(actor_entity), mut score) in scorer_query.iter_mut() {
        let Ok((&ai_pos, mut ai_behavior)) = ai_query.get_mut(*actor_entity) else {
            warn!("Actor must have required components");
            continue;
        };

        // if turn_actor.has_action() {
        //     info!("AI entity {:?} has actions, skipping chase", actor_entity);
        //     continue;
        // }

        let chase_score =
            calculate_chase_score(&ai_pos, &mut ai_behavior, player_pos, current_turn, &current_map);

        score.set(chase_score);
    }
}

fn calculate_chase_score(
    ai_pos: &Position,
    ai_behavior: &mut AIBehavior,
    player_pos: &Position,
    current_turn: u64,
    current_map: &CurrentMap,
) -> f32 {
    if ai_behavior.behavior_type != AIBehaviorType::Hostile {
        return 0.0;
    }

    if FovMap::can_see_entity(*ai_pos, ai_behavior.detection_range, *player_pos, current_map) {
        calculate_visible_player_score(ai_pos, ai_behavior, player_pos, current_turn)
    } else if let Some(last_known_pos) = ai_behavior.last_known_player_position {
        calculate_remembered_position_score(ai_pos, ai_behavior, &last_known_pos, current_turn)
    } else {
        0.0
    }
}

fn calculate_visible_player_score(
    ai_pos: &Position,
    ai_behavior: &mut AIBehavior,
    player_pos: &Position,
    current_turn: u64,
) -> f32 {
    let distance = ai_pos.ai_detection_distance(player_pos);
    if distance <= ai_behavior.detection_range as f32 {
        let chase_score = 1.0;
        ai_behavior.update_player_sighting(*player_pos, current_turn);
        chase_score
    } else {
        0.0
    }
}

fn calculate_remembered_position_score(
    ai_pos: &Position,
    ai_behavior: &AIBehavior,
    last_known_pos: &Position,
    current_turn: u64,
) -> f32 {
    if ai_behavior.should_switch_to_wander(current_turn) {
        return 0.0;
    }

    let _ = ai_pos;
    let _ = last_known_pos;

    // let distance = ai_pos.ai_detection_distance(last_known_pos);
    // let chase_score = 0.3 * (1.0 - (distance / ai_behavior.detection_range as f32));
    // let clamped_score = chase_score.clamp(0.0, 0.5);

    1.0
}

// ============================================================================
// ACTION SYSTEMS (Execute the AI's actions)
// ============================================================================

/// System that handles chasing the player
pub fn chase_player_action_system(
    player_query: Query<&Position, With<PlayerTag>>,
    mut current_map: ResMut<CurrentMap>,
    mut ai_query: Query<(&Position, &mut TurnActor, &mut AIState, &AIBehavior, &Name)>,
    mut action_query: Query<(&Actor, &mut ActionState, &mut ChasePlayerAction)>,
) {
    let Ok(player_pos) = player_query.single() else {
        // No player found or multiple players - skip AI processing
        return;
    };

    for (Actor(actor_entity), mut action_state, mut chase_action) in action_query.iter_mut() {
        let Ok((ai_pos, mut ai_actor, mut ai_state, ai_behavior, ai_name)) = ai_query.get_mut(*actor_entity)
        else {
            warn!("Actor must have required components");
            continue;
        };

        if ai_actor.has_action() {
            continue;
        }

        match *action_state {
            // Success | Failure
            ActionState::Success | ActionState::Failure => {
                info!("{} chase state: {:?}", ai_name, action_state);
                ai_state.current_action = None;
                ai_state.target_position = None;

                continue;
            }
            ActionState::Cancelled => {
                info!("{} cancelled chase!", ai_name);
                *action_state = ActionState::Failure;
                ai_state.current_action = None;
                ai_state.target_position = None;

                continue;
            }
            ActionState::Init | ActionState::Requested => {
                info!("{} gonna start chasing!", ai_name);
                *action_state = ActionState::Executing;

                // Generate A* path to player using enhanced pathfinding
                if let Some(path) =
                    pathfinding::utils::find_path(*ai_pos, *player_pos, &mut current_map, true)
                {
                    info!("{} generated A* path with {} steps", ai_name, path.len());

                    // Store the complete path and tracking information
                    chase_action.current_path = path;
                    chase_action.path_index = 0;
                    chase_action.target_when_path_generated = Some(*player_pos);
                    chase_action.ai_pos_when_path_generated = Some(*ai_pos);
                    chase_action.generated_path = true;
                    chase_action.last_seen_pt = Some(*player_pos);

                    // Use first step of path for immediate movement
                    if let Some(&next_pos) = chase_action.current_path.get(1) {
                        // Skip current position (index 0)
                        let direction = helpers::calculate_direction_to_target(*ai_pos, next_pos);
                        if let Some(dir) = direction {
                            ai_actor.queue_action(ActionType::MoveDelta(dir));
                            chase_action.path_index = 1; // Mark that we're moving to step 1

                            ai_state.current_action = Some(AIAction::ChasePlayer);
                            ai_state.target_position = Some(next_pos);

                            info!("{} starting A* chase, moving {:?} towards {:?}", ai_name, dir, next_pos);
                        } else {
                            info!(
                                "{} A* path generated but cannot calculate direction, falling back",
                                ai_name
                            );
                            *action_state = ActionState::Failure;
                        }
                    } else {
                        info!("{} A* path too short, already at target", ai_name);
                        *action_state = ActionState::Success;
                    }
                } else {
                    // Fallback to simple direction calculation if A* fails
                    info!("{} A* pathfinding failed, using simple direction", ai_name);

                    chase_action.generated_path = false;
                    chase_action.last_seen_pt = Some(*player_pos);
                    chase_action.current_path.clear();

                    ai_state.current_action = Some(AIAction::ChasePlayer);
                    ai_state.target_position = Some(*player_pos);

                    let direction = helpers::calculate_direction_to_target(*ai_pos, *player_pos);
                    if let Some(dir) = direction {
                        ai_actor.queue_action(ActionType::MoveDelta(dir));
                        *action_state = ActionState::Executing;
                    } else {
                        info!("AI entity {:?} cannot find path to player, action failed", actor_entity);
                        *action_state = ActionState::Failure;
                    }
                }
            }
            ActionState::Executing => {}
        }

        info!("{} executing chase!", ai_name);

        // Check if we need to regenerate the path
        let should_regenerate_path = should_regenerate_chase_path(
            &chase_action,
            *ai_pos,
            *player_pos,
            &current_map,
            ai_behavior.detection_range,
        );

        if should_regenerate_path {
            info!("{} regenerating A* path due to changed conditions", ai_name);

            // Regenerate path to current player position
            if let Some(path) = pathfinding::utils::find_path(*ai_pos, *player_pos, &mut current_map, true) {
                chase_action.current_path = path;
                chase_action.path_index = 0;
                chase_action.target_when_path_generated = Some(*player_pos);
                chase_action.ai_pos_when_path_generated = Some(*ai_pos);
                chase_action.generated_path = true;
                chase_action.last_seen_pt = Some(*player_pos);

                info!("{} regenerated A* path with {} steps", ai_name, chase_action.current_path.len());
            } else {
                info!("{} failed to regenerate A* path, falling back to simple chase", ai_name);
                chase_action.generated_path = false;
                chase_action.current_path.clear();
            }
        }

        // Determine target position based on visibility and path availability
        let target_position =
            if FovMap::can_see_entity(*ai_pos, ai_behavior.detection_range, *player_pos, &current_map) {
                // Player is visible - update last seen position and potentially regenerate path
                chase_action.last_seen_pt = Some(*player_pos);

                // If we don't have a current path or it's to a different target, regenerate
                if (!chase_action.generated_path
                    || chase_action.target_when_path_generated != Some(*player_pos))
                    && let Some(path) =
                        pathfinding::utils::find_path(*ai_pos, *player_pos, &mut current_map, true)
                {
                    chase_action.current_path = path;
                    chase_action.path_index = 0;
                    chase_action.target_when_path_generated = Some(*player_pos);
                    chase_action.ai_pos_when_path_generated = Some(*ai_pos);
                    chase_action.generated_path = true;

                    info!("{} updated A* path to visible player", ai_name);
                }

                *player_pos
            } else {
                // Player not visible - use last known position or stored path
                let Some(last_seen) = chase_action.last_seen_pt else {
                    error!("Executing chase with no target.");
                    ai_state.current_action = Some(AIAction::Idle);
                    ai_actor.queue_action(ActionType::Wait);
                    continue;
                };

                // Check if we've reached the last seen position
                if last_seen == *ai_pos {
                    *action_state = ActionState::Failure;
                    continue;
                }

                last_seen
            };

        // Follow the stored A* path or calculate next move
        let next_move_result = if chase_action.generated_path && !chase_action.current_path.is_empty() {
            follow_stored_path(&mut chase_action, *ai_pos, &current_map)
        } else {
            // Fallback to simple direction calculation
            helpers::calculate_direction_to_target(*ai_pos, target_position)
        };

        // Execute the movement
        if let Some(direction) = next_move_result {
            ai_state.current_action = Some(AIAction::ChasePlayer);
            ai_state.target_position = Some(target_position);
            ai_actor.queue_action(ActionType::MoveDelta(direction));

            info!("{} chasing: moving {:?} towards {:?}", ai_name, direction, target_position);
        } else {
            info!("AI entity {:?} cannot find path to player, action failed", actor_entity);
            *action_state = ActionState::Failure;
        }
    }
}

// fn generate_last_seen_path(
//     ai_pos: Position,
//     target_pos: Position,
//     map_provider: &mut CurrentMap,
// ) -> Vec<Position> {
//     pathfinding::utils::find_path(ai_pos, target_pos, map_provider, true).unwrap_or_default()
// }

/// Check if the chase path should be regenerated based on current conditions
fn should_regenerate_chase_path(
    chase_action: &ChasePlayerAction,
    current_ai_pos: Position,
    current_player_pos: Position,
    map: &CurrentMap,
    _detection_range: u8,
) -> bool {
    // No path exists
    if !chase_action.generated_path || chase_action.current_path.is_empty() {
        return true;
    }

    // Player moved significantly from when path was generated
    if let Some(old_target) = chase_action.target_when_path_generated {
        let player_moved_distance = old_target.distance(&current_player_pos);
        if player_moved_distance > 2.0 {
            // Regenerate if player moved more than 2 tiles
            return true;
        }
    }

    // AI moved significantly from when path was generated (shouldn't happen normally)
    if let Some(old_ai_pos) = chase_action.ai_pos_when_path_generated {
        let ai_moved_distance = old_ai_pos.distance(&current_ai_pos);
        if ai_moved_distance > 3.0 {
            // Regenerate if AI somehow moved more than 3 tiles
            return true;
        }
    }

    // Current path step is blocked
    if let Some(next_pos) = chase_action.current_path.get(chase_action.path_index + 1)
        && !map.is_walkable(*next_pos)
    {
        return true;
    }

    // Path is exhausted
    if chase_action.path_index >= chase_action.current_path.len().saturating_sub(1) {
        return true;
    }

    false
}

/// Follow the stored A* path and return the next direction to move
fn follow_stored_path(
    chase_action: &mut ChasePlayerAction,
    current_ai_pos: Position,
    map: &CurrentMap,
) -> Option<Direction> {
    // Ensure we have a valid path
    if chase_action.current_path.is_empty() {
        return None;
    }

    // Find our current position in the path or the closest position
    let mut current_path_index = chase_action.path_index;

    // Verify we're at the expected position in the path
    if let Some(expected_pos) = chase_action.current_path.get(current_path_index)
        && *expected_pos != current_ai_pos
    {
        // Try to find our actual position in the path
        if let Some(found_index) = chase_action.current_path.iter().position(|&pos| pos == current_ai_pos) {
            current_path_index = found_index;
            chase_action.path_index = found_index;
        } else {
            // We're not on the path anymore, need to regenerate
            return None;
        }
    }

    // Get the next step in the path
    let next_index = current_path_index + 1;
    if let Some(next_pos) = chase_action.current_path.get(next_index) {
        // Check if the next position is walkable
        if !map.is_walkable(*next_pos) {
            // Path is blocked, need to regenerate
            return None;
        }

        // Calculate direction to next position
        let direction = helpers::calculate_direction_to_target(current_ai_pos, *next_pos);

        // Update path index for next time
        chase_action.path_index = next_index;

        direction
    } else {
        // Reached end of path
        None
    }
}

// ============================================================================
// DEBUG SYSTEMS
// ============================================================================

/// Debug system to verify AI entity components
pub fn debug_ai_components_system(
    ai_entities: Query<Entity, With<ChasePlayerAction>>,
    position_query: Query<&Position>,
    turn_actor_query: Query<&TurnActor>,
    ai_state_query: Query<&AIState>,
    ai_behavior_query: Query<&AIBehavior>,
) {
    for entity in ai_entities.iter() {
        let has_position = position_query.get(entity).is_ok();
        let has_turn_actor = turn_actor_query.get(entity).is_ok();
        let has_ai_state = ai_state_query.get(entity).is_ok();
        let has_ai_behavior = ai_behavior_query.get(entity).is_ok();

        println!(
            "DEBUG: Entity {entity:?} components - Position: {has_position}, TurnActor: {has_turn_actor}, AIState: {has_ai_state}, AIBehavior: {has_ai_behavior}"
        );
    }
}
