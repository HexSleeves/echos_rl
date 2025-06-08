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
    debug_ai,
    gameplay::{
        enemies::{
            components::{AIBehavior, ChasePlayerAction, ChasePlayerScorer},
            helpers,
        },
        turns::components::TurnActor,
    },
    prelude::assets::AIBehaviorType,
};

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

/// Generate or regenerate A* path for chasing
fn generate_chase_path(
    chase_action: &mut ChasePlayerAction,
    ai_pos: Position,
    target_pos: Position,
    current_map: &mut CurrentMap,
    ai_name: &str,
) -> bool {
    if let Some(path) = pathfinding::utils::find_path(ai_pos, target_pos, current_map, true) {
        debug_ai!("{} generated A* path with {} steps", ai_name, path.len());

        chase_action.current_path = path;
        chase_action.path_index = 0;
        chase_action.target_when_path_generated = Some(target_pos);
        chase_action.ai_pos_when_path_generated = Some(ai_pos);
        chase_action.generated_path = true;
        chase_action.last_seen_pt = Some(target_pos);

        true
    } else {
        debug_ai!("{} A* pathfinding failed", ai_name);
        chase_action.generated_path = false;
        chase_action.current_path.clear();
        chase_action.last_seen_pt = Some(target_pos);

        false
    }
}

/// Execute movement towards target
fn execute_chase_movement(
    ai_actor: &mut TurnActor,
    direction: Direction,
    target_position: Position,
    ai_name: &str,
) {
    ai_actor.queue_action(ActionType::MoveDelta(direction));
    debug_ai!("{} chasing: moving {:?} towards {:?}", ai_name, direction, target_position);
}

/// Determine the target position for chasing based on visibility and stored paths
fn determine_chase_target(
    chase_action: &mut ChasePlayerAction,
    ai_pos: Position,
    player_pos: Position,
    ai_behavior: &AIBehavior,
    current_map: &mut CurrentMap,
    ai_name: &str,
) -> Option<Position> {
    let player_visible = FovMap::can_see_entity(ai_pos, ai_behavior.detection_range, player_pos, current_map);

    if player_visible {
        // Player is visible - update last seen position and potentially regenerate path
        chase_action.last_seen_pt = Some(player_pos);

        // If we don't have a current path or it's to a different target, regenerate
        if !chase_action.generated_path || chase_action.target_when_path_generated != Some(player_pos) {
            generate_chase_path(chase_action, ai_pos, player_pos, current_map, ai_name);
        }

        Some(player_pos)
    } else {
        // Player not visible - use last known position
        let last_seen = chase_action.last_seen_pt?;

        // Check if we've reached the last seen position
        if last_seen == ai_pos {
            return None; // Signal failure
        }

        Some(last_seen)
    }
}

/// Get the next movement direction, either from stored path or direct calculation
fn get_next_chase_direction(
    chase_action: &mut ChasePlayerAction,
    ai_pos: Position,
    target_position: Position,
    current_map: &CurrentMap,
) -> Option<Direction> {
    if chase_action.generated_path && !chase_action.current_path.is_empty() {
        follow_stored_path(chase_action, ai_pos, current_map)
    } else {
        helpers::calculate_direction_to_target(ai_pos, target_position)
    }
}

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

    1.0
}

// ============================================================================
// ACTION SYSTEMS (Execute the AI's actions)
// ============================================================================

/// System that handles chasing the player
pub fn chase_player_action_system(
    player_query: Query<&Position, With<PlayerTag>>,
    mut current_map: ResMut<CurrentMap>,
    mut ai_query: Query<(&Position, &mut TurnActor, &AIBehavior, &Name)>,
    mut action_query: Query<(&Actor, &mut ActionState, &mut ChasePlayerAction)>,
) {
    let Ok(player_pos) = player_query.single() else {
        return;
    };

    for (Actor(actor_entity), mut action_state, mut chase_action) in action_query.iter_mut() {
        let Ok((ai_pos, mut ai_actor, ai_behavior, ai_name)) = ai_query.get_mut(*actor_entity) else {
            warn!("Actor must have required components");
            continue;
        };

        if ai_actor.has_action() {
            continue;
        }

        match *action_state {
            ActionState::Success | ActionState::Failure => {
                debug_ai!("{} chase state: {:?}", ai_name, action_state);
                continue;
            }
            ActionState::Cancelled => {
                debug_ai!("{} cancelled chase!", ai_name);
                *action_state = ActionState::Failure;
                continue;
            }
            ActionState::Init | ActionState::Requested => {
                debug_ai!("{} gonna start chasing!", ai_name);
                *action_state = ActionState::Executing;

                // Generate initial path
                if generate_chase_path(&mut chase_action, *ai_pos, *player_pos, &mut current_map, ai_name) {
                    // Use first step of path for immediate movement
                    if let Some(&next_pos) = chase_action.current_path.get(1) {
                        let direction = helpers::calculate_direction_to_target(*ai_pos, next_pos);

                        if let Some(dir) = direction {
                            execute_chase_movement(&mut ai_actor, dir, next_pos, ai_name);
                            chase_action.path_index = 1; // Mark that we're moving to step 1
                        } else {
                            debug_ai!(
                                "{} A* path generated but cannot calculate direction, falling back",
                                ai_name
                            );
                            *action_state = ActionState::Failure;
                        }
                    } else {
                        debug_ai!("{} A* path too short, already at target", ai_name);
                        *action_state = ActionState::Success;
                    }
                } else {
                    // Fallback to simple direction calculation
                    debug_ai!("{} using simple direction fallback", ai_name);

                    let direction = helpers::calculate_direction_to_target(*ai_pos, *player_pos);
                    if let Some(dir) = direction {
                        execute_chase_movement(&mut ai_actor, dir, *player_pos, ai_name);
                    } else {
                        debug_ai!("AI entity {:?} cannot find path to player, action failed", actor_entity);
                        *action_state = ActionState::Failure;
                    }
                }
            }
            ActionState::Executing => {
                debug_ai!("{} executing chase!", ai_name);

                // Check if we need to regenerate the path
                if should_regenerate_chase_path(&chase_action, *ai_pos, *player_pos, &current_map) {
                    debug_ai!("{} regenerating A* path due to changed conditions", ai_name);
                    generate_chase_path(&mut chase_action, *ai_pos, *player_pos, &mut current_map, ai_name);
                }

                // Determine target position based on visibility and path availability
                let Some(target_position) = determine_chase_target(
                    &mut chase_action,
                    *ai_pos,
                    *player_pos,
                    ai_behavior,
                    &mut current_map,
                    ai_name,
                ) else {
                    *action_state = ActionState::Failure;
                    continue;
                };

                // Get next movement direction
                let Some(direction) =
                    get_next_chase_direction(&mut chase_action, *ai_pos, target_position, &current_map)
                else {
                    debug_ai!("AI entity {:?} cannot find path to player, action failed", actor_entity);
                    *action_state = ActionState::Failure;
                    continue;
                };

                // Execute the movement
                execute_chase_movement(&mut ai_actor, direction, target_position, ai_name);
            }
        }
    }
}

/// Check if the chase path should be regenerated based on current conditions
fn should_regenerate_chase_path(
    chase_action: &ChasePlayerAction,
    current_ai_pos: Position,
    current_player_pos: Position,
    map: &CurrentMap,
) -> bool {
    // No path exists
    if !chase_action.generated_path || chase_action.current_path.is_empty() {
        return true;
    }

    // Player moved significantly from when path was generated
    if let Some(old_target) = chase_action.target_when_path_generated {
        let player_moved_distance = old_target.distance(&current_player_pos);
        if player_moved_distance > 2.0 {
            return true;
        }
    }

    // AI moved significantly from when path was generated
    if let Some(old_ai_pos) = chase_action.ai_pos_when_path_generated {
        let ai_moved_distance = old_ai_pos.distance(&current_ai_pos);
        if ai_moved_distance > 3.0 {
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
    ai_behavior_query: Query<&AIBehavior>,
) {
    for entity in ai_entities.iter() {
        let has_position = position_query.get(entity).is_ok();
        let has_turn_actor = turn_actor_query.get(entity).is_ok();
        let has_ai_behavior = ai_behavior_query.get(entity).is_ok();

        println!(
            "DEBUG: Entity {entity:?} components - Position: {has_position}, TurnActor: {has_turn_actor}, AIBehavior: {has_ai_behavior}"
        );
    }
}
