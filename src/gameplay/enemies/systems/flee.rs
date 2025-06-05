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
            components::{AIAction, AIBehavior, AIState, FleeFromPlayerAction, FleeFromPlayerScorer},
            helpers,
        },
        turns::components::TurnActor,
    },
    prelude::assets::AIBehaviorType,
};

/// System that scores how much an AI wants to flee from the player
pub fn flee_from_player_scorer_system(
    current_map: Res<CurrentMap>,
    turn_queue: Res<TurnQueue>,
    player_query: Query<&Position, With<PlayerTag>>,
    mut ai_query: Query<(&Position, &mut AIBehavior, &TurnActor)>,
    mut scorer_query: Query<(&Actor, &mut Score), With<FleeFromPlayerScorer>>,
) {
    let Ok(player_pos) = player_query.single() else {
        return;
    };

    let current_turn = turn_queue.current_time();

    for (Actor(actor_entity), mut score) in scorer_query.iter_mut() {
        let Ok((&ai_pos, mut ai_behavior, turn_actor)) = ai_query.get_mut(*actor_entity) else {
            warn!("Actor must have required components");
            continue;
        };

        // Don't score if already has actions queued
        if turn_actor.has_action() {
            score.set(0.0);
            continue;
        }

        // Only passive entities should flee
        if ai_behavior.behavior_type != AIBehaviorType::Passive {
            score.set(0.0);
            continue;
        }

        // Check if player is visible and close enough to trigger flee
        if FovMap::can_see_entity(ai_pos, ai_behavior.detection_range, *player_pos, &current_map) {
            let distance = ai_pos.distance(player_pos);

            // Flee if player is within detection range
            if distance <= ai_behavior.detection_range as f32 {
                ai_behavior.last_player_seen_turn = Some(current_turn);
                ai_behavior.last_known_player_position = Some(*player_pos);

                // Higher score for closer players (more urgent to flee)
                let urgency = 1.0 - (distance / ai_behavior.detection_range as f32);
                let flee_score = 0.8 + (urgency * 0.2); // 0.8 to 1.0

                info!(
                    "Passive AI entity {:?} wants to flee from player at distance {}",
                    actor_entity, distance
                );
                score.set(flee_score);
                continue;
            }
        }

        // No immediate threat
        score.set(0.0);
    }
}

/// System that handles fleeing from the player
pub fn flee_from_player_action_system(
    player_query: Query<&Position, With<PlayerTag>>,
    mut current_map: ResMut<CurrentMap>,
    mut action_query: Query<(&Actor, &mut ActionState, &mut FleeFromPlayerAction)>,
    mut ai_query: Query<(&Position, &mut TurnActor, &mut AIState, &AIBehavior, &Name)>,
) {
    let Ok(player_pos) = player_query.single() else {
        // No player found or multiple players - skip AI processing
        return;
    };

    for (Actor(actor_entity), mut action_state, mut flee_action) in action_query.iter_mut() {
        let Ok((ai_pos, mut ai_actor, mut ai_state, ai_behavior, ai_name)) = ai_query.get_mut(*actor_entity)
        else {
            warn!("Actor must have required components");
            continue;
        };

        if ai_actor.has_action() {
            continue;
        }

        match *action_state {
            ActionState::Init | ActionState::Requested => {
                info!("{} gonna start fleeing!", ai_name);
                *action_state = ActionState::Executing;

                // Generate A* escape path using intelligent escape route finding
                if let Some(escape_target) =
                    find_escape_target(*ai_pos, *player_pos, &mut current_map, ai_behavior.detection_range)
                {
                    if let Some(path) =
                        pathfinding::utils::find_path(*ai_pos, escape_target, &mut current_map, true)
                    {
                        info!(
                            "{} generated A* escape path with {} steps to {:?}",
                            ai_name,
                            path.len(),
                            escape_target
                        );

                        // Store the complete escape path and tracking information
                        flee_action.escape_path = path;
                        flee_action.path_index = 0;
                        flee_action.escape_target = Some(escape_target);
                        flee_action.threat_pos_when_path_generated = Some(*player_pos);
                        flee_action.ai_pos_when_path_generated = Some(*ai_pos);

                        // Use first step of path for immediate movement
                        if let Some(&next_pos) = flee_action.escape_path.get(1) {
                            // Skip current position (index 0)
                            let direction = helpers::calculate_direction_to_target(*ai_pos, next_pos);
                            if let Some(dir) = direction {
                                ai_actor.queue_action(ActionType::MoveDelta(dir));
                                flee_action.path_index = 1; // Mark that we're moving to step 1

                                ai_state.current_action = Some(AIAction::FleeFromPlayer);
                                ai_state.target_position = Some(next_pos);

                                info!(
                                    "{} starting A* escape, moving {:?} towards {:?}",
                                    ai_name, dir, next_pos
                                );
                            } else {
                                info!(
                                    "{} A* escape path generated but cannot calculate direction, falling back",
                                    ai_name
                                );
                                *action_state = ActionState::Failure;
                            }
                        } else {
                            info!("{} A* escape path too short, already at safe position", ai_name);
                            *action_state = ActionState::Success;
                        }
                    } else {
                        info!("{} failed to generate A* path to escape target, using simple flee", ai_name);
                        // Fallback to simple direction calculation
                        use_simple_flee_fallback(
                            &mut ai_actor,
                            &mut ai_state,
                            ai_pos,
                            player_pos,
                            ai_name,
                            &mut action_state,
                        );
                    }
                } else {
                    info!("{} could not find escape target, using simple flee", ai_name);
                    // Fallback to simple direction calculation
                    use_simple_flee_fallback(
                        &mut ai_actor,
                        &mut ai_state,
                        ai_pos,
                        player_pos,
                        ai_name,
                        &mut action_state,
                    );
                }
            }
            ActionState::Executing => {
                info!("{} executing flee!", ai_name);

                // Check if we need to regenerate the escape path
                let should_regenerate_path = should_regenerate_flee_path(
                    &flee_action,
                    *ai_pos,
                    *player_pos,
                    &current_map,
                    ai_behavior.detection_range,
                );

                if should_regenerate_path {
                    info!("{} regenerating A* escape path due to changed conditions", ai_name);

                    // Regenerate escape path
                    if let Some(escape_target) = find_escape_target(
                        *ai_pos,
                        *player_pos,
                        &mut current_map,
                        ai_behavior.detection_range,
                    ) {
                        if let Some(path) =
                            pathfinding::utils::find_path(*ai_pos, escape_target, &mut current_map, true)
                        {
                            flee_action.escape_path = path;
                            flee_action.path_index = 0;
                            flee_action.escape_target = Some(escape_target);
                            flee_action.threat_pos_when_path_generated = Some(*player_pos);
                            flee_action.ai_pos_when_path_generated = Some(*ai_pos);

                            info!(
                                "{} regenerated A* escape path with {} steps",
                                ai_name,
                                flee_action.escape_path.len()
                            );
                        } else {
                            info!(
                                "{} failed to regenerate A* escape path, falling back to simple flee",
                                ai_name
                            );
                            flee_action.escape_path.clear();
                        }
                    }
                }

                // Follow the stored A* escape path or use simple flee
                let next_move_result = if !flee_action.escape_path.is_empty() {
                    follow_stored_escape_path(&mut flee_action, *ai_pos, &current_map)
                } else {
                    // Fallback to simple direction calculation
                    helpers::calculate_direction_away_from_target(*ai_pos, *player_pos)
                };

                // Execute the movement
                if let Some(direction) = next_move_result {
                    ai_state.current_action = Some(AIAction::FleeFromPlayer);
                    ai_actor.queue_action(ActionType::MoveDelta(direction));

                    info!("{} fleeing: moving {:?} away from player", ai_name, direction);
                } else {
                    info!("AI entity {:?} cannot find escape path, action failed", actor_entity);
                    *action_state = ActionState::Failure;
                }
            }
            ActionState::Success | ActionState::Failure => {
                // Action completed, reset to init and wait for next decision cycle
                *action_state = ActionState::Init;
            }
            ActionState::Cancelled => {
                *action_state = ActionState::Init;
            }
        }
    }
}

/// Find a safe escape target position away from the threat
fn find_escape_target(
    ai_pos: Position,
    threat_pos: Position,
    map: &mut CurrentMap,
    detection_range: u8,
) -> Option<Position> {
    // Try to use the existing escape route finding utility
    if let Some(escape_pos) =
        pathfinding::utils::find_escape_route(ai_pos, threat_pos, map, detection_range as u32)
    {
        return Some(escape_pos);
    }

    // Fallback: find a position in the opposite direction
    let dx = ai_pos.x() - threat_pos.x();
    let dy = ai_pos.y() - threat_pos.y();

    // Normalize and extend the escape direction
    let escape_distance = (detection_range as i32 * 2).max(5); // At least 5 tiles away
    let escape_x = ai_pos.x() + (dx.signum() * escape_distance);
    let escape_y = ai_pos.y() + (dy.signum() * escape_distance);

    let potential_escape = Position::new(escape_x, escape_y);

    // Check if the escape position is valid and walkable
    if map.is_walkable(potential_escape) {
        Some(potential_escape)
    } else {
        // Try nearby positions if the direct escape is blocked
        for offset_x in -2..=2 {
            for offset_y in -2..=2 {
                let test_pos = Position::new(escape_x + offset_x, escape_y + offset_y);
                if map.is_walkable(test_pos) {
                    return Some(test_pos);
                }
            }
        }
        None
    }
}

/// Check if the flee path should be regenerated based on current conditions
fn should_regenerate_flee_path(
    flee_action: &FleeFromPlayerAction,
    current_ai_pos: Position,
    current_threat_pos: Position,
    map: &CurrentMap,
    _detection_range: u8,
) -> bool {
    // No path exists
    if flee_action.escape_path.is_empty() {
        return true;
    }

    // Threat moved significantly from when path was generated
    if let Some(old_threat) = flee_action.threat_pos_when_path_generated {
        let threat_moved_distance = old_threat.distance(&current_threat_pos);
        if threat_moved_distance > 3.0 {
            // Regenerate if threat moved more than 3 tiles
            return true;
        }
    }

    // AI moved significantly from when path was generated (shouldn't happen normally)
    if let Some(old_ai_pos) = flee_action.ai_pos_when_path_generated {
        let ai_moved_distance = old_ai_pos.distance(&current_ai_pos);
        if ai_moved_distance > 3.0 {
            // Regenerate if AI somehow moved more than 3 tiles
            return true;
        }
    }

    // Current path step is blocked
    if let Some(next_pos) = flee_action.escape_path.get(flee_action.path_index + 1)
        && !map.is_walkable(*next_pos)
    {
        return true;
    }

    // Path is exhausted
    if flee_action.path_index >= flee_action.escape_path.len().saturating_sub(1) {
        return true;
    }

    false
}

/// Follow the stored A* escape path and return the next direction to move
fn follow_stored_escape_path(
    flee_action: &mut FleeFromPlayerAction,
    current_ai_pos: Position,
    map: &CurrentMap,
) -> Option<Direction> {
    // Ensure we have a valid path
    if flee_action.escape_path.is_empty() {
        return None;
    }

    // Find our current position in the path or the closest position
    let mut current_path_index = flee_action.path_index;

    // Verify we're at the expected position in the path
    if let Some(expected_pos) = flee_action.escape_path.get(current_path_index)
        && *expected_pos != current_ai_pos
    {
        // Try to find our actual position in the path
        if let Some(found_index) = flee_action.escape_path.iter().position(|&pos| pos == current_ai_pos) {
            current_path_index = found_index;
            flee_action.path_index = found_index;
        } else {
            // We're not on the path anymore, need to regenerate
            return None;
        }
    }

    // Get the next step in the path
    let next_index = current_path_index + 1;
    if let Some(next_pos) = flee_action.escape_path.get(next_index) {
        // Check if the next position is walkable
        if !map.is_walkable(*next_pos) {
            // Path is blocked, need to regenerate
            return None;
        }

        // Calculate direction to next position
        let direction = helpers::calculate_direction_to_target(current_ai_pos, *next_pos);

        // Update path index for next time
        flee_action.path_index = next_index;

        direction
    } else {
        // Reached end of escape path - success!
        None
    }
}

/// Fallback to simple flee behavior when A* pathfinding fails
fn use_simple_flee_fallback(
    ai_actor: &mut TurnActor,
    ai_state: &mut AIState,
    ai_pos: &Position,
    player_pos: &Position,
    ai_name: &Name,
    action_state: &mut ActionState,
) {
    ai_state.current_action = Some(AIAction::FleeFromPlayer);

    // Calculate direction away from player
    let direction = helpers::calculate_direction_away_from_target(*ai_pos, *player_pos);

    if let Some(dir) = direction {
        ai_actor.queue_action(ActionType::MoveDelta(dir));
        *action_state = ActionState::Executing;
        info!("{} using simple flee, moving {:?}", ai_name, dir);
    } else {
        info!("{} cannot find simple flee direction, action failed", ai_name);
        *action_state = ActionState::Failure;
    }
}
