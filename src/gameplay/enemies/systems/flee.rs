use bevy::prelude::*;
use big_brain::prelude::*;
use brtk::prelude::Direction;

use crate::{
    core::{
        components::{PlayerTag, Position},
        pathfinding,
        resources::{CurrentMap, FovMap, TurnQueue},
    },
    debug_ai,
    gameplay::{
        enemies::{
            components::{AIBehavior, FleeFromPlayerAction, FleeFromPlayerScorer},
            helpers,
        },
        turns::components::TurnActor,
    },
    prelude::assets::AIBehaviorType,
};

/// System that scores how much an AI wants to flee from the player
pub fn flee_from_player_scorer_system(
    turn_queue: Res<TurnQueue>,
    current_map: Res<CurrentMap>,
    player_query: Query<&Position, With<PlayerTag>>,
    mut ai_query: Query<(&Position, &mut AIBehavior)>,
    mut scorer_query: Query<(&Actor, &mut Score), With<FleeFromPlayerScorer>>,
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

        let flee_score =
            calculate_flee_score(&ai_pos, &mut ai_behavior, player_pos, current_turn, &current_map);

        score.set(flee_score);
    }
}

fn calculate_flee_score(
    ai_pos: &Position,
    ai_behavior: &mut AIBehavior,
    player_pos: &Position,
    current_turn: u64,
    current_map: &CurrentMap,
) -> f32 {
    if ai_behavior.behavior_type != AIBehaviorType::Passive {
        return 0.0;
    }

    if FovMap::can_see_entity(*ai_pos, ai_behavior.detection_range, *player_pos, current_map) {
        let distance = ai_pos.ai_detection_distance(player_pos);
        if distance <= ai_behavior.detection_range as f32 {
            ai_behavior.update_player_sighting(*player_pos, current_turn);

            // Base flee score starts high when threat is visible
            let mut flee_score = 0.8;

            // Enhanced threat proximity calculation with exponential scaling
            // Closer threats cause exponentially higher panic
            let threat_proximity = 1.0 - (distance / ai_behavior.detection_range as f32);
            let proximity_bonus = threat_proximity.powf(2.0) * 0.4; // Up to 0.4 bonus with quadratic scaling
            flee_score += proximity_bonus;

            // Enhanced panic bonus - escalating fear from sustained threat exposure
            if let Some(last_seen_turn) = ai_behavior.last_player_seen_turn {
                let turns_in_danger = current_turn.saturating_sub(last_seen_turn);
                if turns_in_danger > 0 && turns_in_danger <= 5 {
                    // Escalating panic for sustained threat exposure
                    let panic_bonus = (turns_in_danger as f32 / 5.0).powf(1.2) * 0.25;
                    flee_score += panic_bonus;
                }
            }

            // Critical distance modifier - immediate panic when very close
            if distance <= 1.5 {
                flee_score += 0.15; // Critical proximity panic bonus
            }

            // Ensure we don't exceed 1.0
            flee_score.min(1.0)
        } else {
            0.0
        }
    } else {
        0.0 // Don't flee if player is not visible
    }
}

/// System that handles fleeing from the player
pub fn flee_from_player_action_system(
    player_query: Query<&Position, With<PlayerTag>>,
    mut current_map: ResMut<CurrentMap>,
    mut action_query: Query<(&Actor, &mut ActionState, &mut FleeFromPlayerAction)>,
    mut ai_query: Query<(&Position, &mut TurnActor, &AIBehavior, &Name)>,
) {
    let Ok(player_pos) = player_query.single() else {
        return;
    };

    for (Actor(actor_entity), mut action_state, mut flee_action) in action_query.iter_mut() {
        let Ok((ai_pos, mut ai_actor, _ai_behavior, ai_name)) = ai_query.get_mut(*actor_entity) else {
            warn!("Actor must have required components");
            continue;
        };

        if ai_actor.has_action() {
            continue;
        }

        match *action_state {
            ActionState::Success | ActionState::Failure => {
                debug_ai!("{} flee state: {:?}", ai_name, action_state);
                continue;
            }
            ActionState::Cancelled => {
                debug_ai!("{} cancelled flee!", ai_name);
                *action_state = ActionState::Failure;
                continue;
            }
            ActionState::Init | ActionState::Requested => {
                debug_ai!("{} gonna start fleeing!", ai_name);
                *action_state = ActionState::Executing;

                // Generate escape path using enhanced pathfinding
                if let Some(escape_target) = find_escape_destination(*ai_pos, *player_pos, &current_map) {
                    if let Some(path) =
                        pathfinding::utils::find_path(*ai_pos, escape_target, &mut current_map, true)
                    {
                        debug_ai!("{} generated A* escape path with {} steps", ai_name, path.len());

                        // Store the complete escape path and tracking information
                        flee_action.escape_path = path;
                        flee_action.path_index = 0;
                        flee_action.escape_target = Some(escape_target);
                        flee_action.threat_pos_when_path_generated = Some(*player_pos);
                        flee_action.ai_pos_when_path_generated = Some(*ai_pos);

                        // Use first step of path for immediate movement
                        if let Some(&next_pos) = flee_action.escape_path.get(1) {
                            // Skip current position (index 0)
                            let direction = helpers::calculate_direction_to_target(ai_pos, &next_pos);

                            if let Some(dir) = direction {
                                execute_flee_movement(&mut ai_actor, dir, next_pos, ai_name);
                                flee_action.path_index = 1; // Mark that we're moving to step 1
                            } else {
                                debug_ai!(
                                    "{} A* escape path generated but cannot calculate direction, falling back",
                                    ai_name
                                );
                                *action_state = ActionState::Failure;
                            }
                        } else {
                            debug_ai!("{} A* escape path too short, already at destination", ai_name);
                            *action_state = ActionState::Success;
                        }
                    } else {
                        debug_ai!(
                            "{} A* pathfinding to escape destination failed, using simple flee",
                            ai_name
                        );
                        // Fallback to simple direction calculation away from player
                        let direction = helpers::calculate_direction_away_from_target(ai_pos, player_pos);
                        if let Some(dir) = direction {
                            execute_flee_movement(&mut ai_actor, dir, escape_target, ai_name);
                        } else {
                            debug_ai!(
                                "AI entity {:?} cannot find escape direction, action failed",
                                actor_entity
                            );
                            *action_state = ActionState::Failure;
                        }
                    }
                } else {
                    debug_ai!("{} cannot find escape destination, action failed", ai_name);
                    *action_state = ActionState::Failure;
                }
            }
            ActionState::Executing => {
                debug_ai!("{} executing flee!", ai_name);

                // Check if we need to regenerate the escape path
                if should_regenerate_escape_path(&flee_action, *ai_pos, *player_pos, &current_map) {
                    debug_ai!("{} regenerating A* escape path due to changed conditions", ai_name);

                    if let Some(new_escape_target) =
                        find_escape_destination(*ai_pos, *player_pos, &current_map)
                    {
                        if let Some(path) =
                            pathfinding::utils::find_path(*ai_pos, new_escape_target, &mut current_map, true)
                        {
                            flee_action.escape_path = path;
                            flee_action.path_index = 0;
                            flee_action.escape_target = Some(new_escape_target);
                            flee_action.threat_pos_when_path_generated = Some(*player_pos);
                            flee_action.ai_pos_when_path_generated = Some(*ai_pos);

                            debug_ai!(
                                "{} regenerated A* escape path with {} steps",
                                ai_name,
                                flee_action.escape_path.len()
                            );
                        } else {
                            debug_ai!(
                                "{} failed to regenerate A* escape path, falling back to simple flee",
                                ai_name
                            );
                            flee_action.escape_path.clear();
                        }
                    }
                }

                // Follow the stored A* path or calculate next move
                let next_move_result = if !flee_action.escape_path.is_empty() {
                    follow_stored_escape_path(&mut flee_action, *ai_pos, &current_map)
                } else {
                    // Fallback to simple direction calculation away from player
                    helpers::calculate_direction_away_from_target(ai_pos, player_pos)
                };

                // Execute the movement
                if let Some(direction) = next_move_result {
                    let target_position = flee_action.escape_target.unwrap_or(*ai_pos);
                    execute_flee_movement(&mut ai_actor, direction, target_position, ai_name);
                } else {
                    debug_ai!("AI entity {:?} cannot find escape path, action failed", actor_entity);
                    *action_state = ActionState::Failure;
                }
            }
        }
    }
}

/// Execute movement away from threat
fn execute_flee_movement(
    ai_actor: &mut TurnActor,
    direction: Direction,
    target_position: Position,
    ai_name: &str,
) {
    ai_actor.queue_move_delta(direction);
    debug_ai!("{} fleeing: moving {:?} towards {:?}", ai_name, direction, target_position);
}

/// Find a good escape destination away from the threat
fn find_escape_destination(ai_pos: Position, threat_pos: Position, map: &CurrentMap) -> Option<Position> {
    let escape_distance = 8; // Try to get at least 8 tiles away
    let max_attempts = 20;

    for attempt in 0..max_attempts {
        // Calculate direction away from threat
        let dx = ai_pos.x - threat_pos.x;
        let dy = ai_pos.y - threat_pos.y;

        // Normalize and extend the escape vector
        let escape_magnitude = (escape_distance + attempt) as f32;
        let distance = ((dx * dx + dy * dy) as f32).sqrt().max(1.0);
        let escape_x = ai_pos.x + ((dx as f32 / distance) * escape_magnitude) as i32;
        let escape_y = ai_pos.y + ((dy as f32 / distance) * escape_magnitude) as i32;

        let escape_pos = Position::new(escape_x, escape_y);

        // Check if the escape position is valid and walkable
        if map.is_walkable(escape_pos) {
            return Some(escape_pos);
        }

        // Try slight variations if the direct escape route is blocked
        for offset_x in -2..=2 {
            for offset_y in -2..=2 {
                let variant_pos = Position::new(escape_x + offset_x, escape_y + offset_y);
                if map.is_walkable(variant_pos) {
                    return Some(variant_pos);
                }
            }
        }
    }

    // If no good escape destination found, try any walkable position away from threat
    for radius in 1..=5 {
        for dx in -radius..=radius {
            for dy in -radius..=radius {
                if dx == 0 && dy == 0 {
                    continue;
                }

                let candidate_pos = Position::new(ai_pos.x + dx, ai_pos.y + dy);
                if map.is_walkable(candidate_pos) {
                    let distance_from_threat = candidate_pos.distance(&threat_pos);
                    let distance_from_ai = candidate_pos.distance(&ai_pos);

                    // Prefer positions that are further from threat and not too far from AI
                    if distance_from_threat > ai_pos.distance(&threat_pos) && distance_from_ai <= 3.0 {
                        return Some(candidate_pos);
                    }
                }
            }
        }
    }

    None
}

/// Check if the escape path should be regenerated based on current conditions
fn should_regenerate_escape_path(
    flee_action: &FleeFromPlayerAction,
    current_ai_pos: Position,
    current_threat_pos: Position,
    map: &CurrentMap,
) -> bool {
    // No path exists
    if flee_action.escape_path.is_empty() {
        return true;
    }

    // Threat moved significantly from when path was generated
    if let Some(old_threat_pos) = flee_action.threat_pos_when_path_generated {
        let threat_moved_distance = old_threat_pos.distance(&current_threat_pos);
        if threat_moved_distance > 2.0 {
            // Regenerate if threat moved more than 2 tiles
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
        let direction = helpers::calculate_direction_to_target(&current_ai_pos, next_pos);

        // Update path index for next time
        flee_action.path_index = next_index;

        direction
    } else {
        // Reached end of escape path
        None
    }
}
