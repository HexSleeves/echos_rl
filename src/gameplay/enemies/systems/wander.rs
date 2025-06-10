use bevy::prelude::*;
use big_brain::prelude::*;
use brtk::{prelude::Direction, random::Random};

use crate::{
    core::{
        components::{PlayerTag, Position},
        pathfinding,
        resources::{CurrentMap, FovMap, TurnQueue},
    },
    debug_ai,
    gameplay::{
        enemies::{
            components::{AIBehavior, WanderAction, WanderScorer, WanderType},
            helpers,
        },
        turns::components::TurnActor,
    },
    prelude::assets::AIBehaviorType,
    warn_ai,
};

// ============================================================================
// SCORER SYSTEMS (Evaluate what the AI should do)
// ============================================================================

/// System that scores how much an AI wants to wander
pub fn wander_scorer_system(
    turn_queue: Res<TurnQueue>,
    current_map: Res<CurrentMap>,
    player_query: Query<&Position, With<PlayerTag>>,
    mut ai_query: Query<(&Position, &mut AIBehavior)>,
    mut scorer_query: Query<(&Actor, &mut Score), With<WanderScorer>>,
) {
    let Ok(player_pos) = player_query.single() else {
        return;
    };

    let current_turn = turn_queue.current_time();

    for (Actor(actor_entity), mut score) in scorer_query.iter_mut() {
        let Ok((&ai_pos, mut ai_behavior)) = ai_query.get_mut(*actor_entity) else {
            warn_ai!("Actor must have required components");
            continue;
        };

        let wander_score =
            calculate_wander_score(&ai_pos, &mut ai_behavior, player_pos, current_turn, &current_map);

        score.set(wander_score);
    }
}

fn calculate_wander_score(
    ai_pos: &Position,
    ai_behavior: &mut AIBehavior,
    player_pos: &Position,
    current_turn: u64,
    current_map: &CurrentMap,
) -> f32 {
    // Base wander score depends on behavior type
    let base_score = match ai_behavior.behavior_type {
        AIBehaviorType::Neutral => 0.4, // Neutral entities wander frequently
        AIBehaviorType::Hostile => 0.2, // Hostile entities prefer chasing
        AIBehaviorType::Passive => 0.3, // Passive entities wander moderately
    };

    // If player is visible, reduce wander score (other actions should take priority)
    if FovMap::can_see_entity(*ai_pos, ai_behavior.detection_range, *player_pos, current_map) {
        ai_behavior.update_player_sighting(*player_pos, current_turn);
        base_score * 0.1 // Very low priority when player is visible
    } else if ai_behavior.should_switch_to_wander(current_turn) {
        // If enough time has passed since last player sighting, increase wander score
        base_score * 1.5
    } else {
        base_score
    }
}

// ============================================================================
// ACTION SYSTEMS (Execute the AI's actions)
// ============================================================================

/// System that handles wandering behavior
pub fn wander_action_system(
    turn_queue: Res<TurnQueue>,
    mut random: ResMut<Random>,
    mut current_map: ResMut<CurrentMap>,
    mut ai_query: Query<(&Position, &mut TurnActor, &Name)>,
    mut action_query: Query<(&Actor, &mut ActionState, &mut WanderAction)>,
) {
    let current_turn = turn_queue.current_time();

    for (Actor(actor_entity), mut action_state, mut wander_action) in action_query.iter_mut() {
        let Ok((ai_pos, mut ai_actor, ai_name)) = ai_query.get_mut(*actor_entity) else {
            warn_ai!("Actor must have required components");
            continue;
        };

        if ai_actor.has_action() {
            continue;
        }

        match *action_state {
            ActionState::Success | ActionState::Failure => {
                debug_ai!("{} wander state: {:?}", ai_name, action_state);
                continue;
            }
            ActionState::Cancelled => {
                debug_ai!("{} cancelled wander!", ai_name);
                *action_state = ActionState::Failure;
                continue;
            }
            ActionState::Init | ActionState::Requested => {
                debug_ai!("{} gonna start wandering!", ai_name);
                *action_state = ActionState::Executing;

                let target_position = select_wander_target(
                    &mut wander_action,
                    *ai_pos,
                    &current_map,
                    current_turn,
                    &mut random,
                );

                if let Some(target) = target_position {
                    // Generate A* path to wander target
                    if let Some(path) = pathfinding::utils::find_path(*ai_pos, target, &mut current_map, true)
                    {
                        debug_ai!("{} generated A* wander path with {} steps", ai_name, path.len());

                        // Store the complete path and tracking information
                        wander_action.current_path = path;
                        wander_action.path_index = 0;
                        wander_action.current_target = Some(target);
                        wander_action.ai_pos_when_path_generated = Some(*ai_pos);

                        // Use first step of path for immediate movement
                        if let Some(&next_pos) = wander_action.current_path.get(1) {
                            let direction = helpers::calculate_direction_to_target(ai_pos, &next_pos);

                            if let Some(dir) = direction {
                                execute_wander_movement(&mut ai_actor, dir, target, ai_name);
                                wander_action.path_index = 1; // Mark that we're moving to step 1
                            } else {
                                debug_ai!(
                                    "{} A* wander path generated but cannot calculate direction",
                                    ai_name
                                );
                                *action_state = ActionState::Failure;
                            }
                        } else {
                            debug_ai!("{} A* wander path too short, already at target", ai_name);
                            *action_state = ActionState::Success;
                        }
                    } else {
                        // Fallback to simple random direction
                        debug_ai!("{} A* wander pathfinding failed, using simple movement", ai_name);
                        wander_action.current_path.clear();
                        wander_action.current_target = Some(target);

                        let direction = helpers::calculate_direction_to_target(ai_pos, &target);
                        if let Some(dir) = direction {
                            execute_wander_movement(&mut ai_actor, dir, target, ai_name);
                        } else {
                            debug_ai!(
                                "AI entity {:?} cannot find wander direction, action failed",
                                actor_entity
                            );
                            *action_state = ActionState::Failure;
                        }
                    }
                } else {
                    debug_ai!("{} cannot find wander target, action failed", ai_name);
                    *action_state = ActionState::Failure;
                }
            }
            ActionState::Executing => {
                debug_ai!("{} executing wander!", ai_name);

                // Check if we need to regenerate the wander path or select new target
                if should_regenerate_wander_path(&wander_action, *ai_pos, &current_map, current_turn) {
                    debug_ai!("{} regenerating wander path or selecting new target", ai_name);

                    let new_target = select_wander_target(
                        &mut wander_action,
                        *ai_pos,
                        &current_map,
                        current_turn,
                        &mut random,
                    );
                    if let Some(target) = new_target {
                        if let Some(path) =
                            pathfinding::utils::find_path(*ai_pos, target, &mut current_map, true)
                        {
                            wander_action.current_path = path;
                            wander_action.path_index = 0;
                            wander_action.current_target = Some(target);
                            wander_action.ai_pos_when_path_generated = Some(*ai_pos);

                            debug_ai!(
                                "{} regenerated wander path with {} steps",
                                ai_name,
                                wander_action.current_path.len()
                            );
                        } else {
                            debug_ai!("{} failed to regenerate wander path, using simple movement", ai_name);
                            wander_action.current_path.clear();
                            wander_action.current_target = Some(target);
                        }
                    }
                }

                // Follow the stored A* path or calculate next move
                let target_position = wander_action.current_target.unwrap_or(*ai_pos);

                // Check if we've reached our target
                if target_position == *ai_pos {
                    debug_ai!("{} reached wander target, selecting new target", ai_name);
                    *action_state = ActionState::Success;
                    continue;
                }

                // Get next movement direction
                let next_move_result = if !wander_action.current_path.is_empty() {
                    follow_stored_wander_path(&mut wander_action, *ai_pos, &current_map)
                } else {
                    helpers::calculate_direction_to_target(ai_pos, &target_position)
                };

                // Execute the movement
                if let Some(direction) = next_move_result {
                    execute_wander_movement(&mut ai_actor, direction, target_position, ai_name);
                } else {
                    debug_ai!("AI entity {:?} cannot find wander path, action failed", actor_entity);
                    *action_state = ActionState::Failure;
                }
            }
        }
    }
}

/// Execute wander movement towards target
fn execute_wander_movement(
    ai_actor: &mut TurnActor,
    direction: Direction,
    target_position: Position,
    ai_name: &str,
) {
    ai_actor.queue_move_delta(direction);
    debug_ai!("{} wandering: moving {:?} towards {:?}", ai_name, direction, target_position);

    // Suppress unused variable warnings when debug feature is disabled
    #[cfg(not(feature = "debug"))]
    {
        let _ = (target_position, ai_name);
    }
}

/// Select an appropriate wander target based on the wander type
fn select_wander_target(
    wander_action: &mut WanderAction,
    ai_pos: Position,
    map: &CurrentMap,
    current_turn: u64,
    random: &mut Random,
) -> Option<Position> {
    match wander_action.wander_type {
        WanderType::Random => select_random_wander_target(ai_pos, map, random),
        WanderType::AreaWander => select_area_wander_target(wander_action, ai_pos, map, random),
        WanderType::Patrol => select_patrol_target(wander_action, ai_pos, map, random),
        WanderType::Explore => select_exploration_target(ai_pos, map, random),
    }
    .inspect(|_| {
        wander_action.last_target_time = Some(current_turn);
    })
}

/// Select a random nearby position for wandering
fn select_random_wander_target(ai_pos: Position, map: &CurrentMap, random: &mut Random) -> Option<Position> {
    // Try to find a walkable position within a reasonable range
    for _ in 0..20 {
        let dx = random.i32(-5..=5);
        let dy = random.i32(-5..=5);
        let target = Position::new(ai_pos.x + dx, ai_pos.y + dy);

        if map.is_walkable(target) && target != ai_pos {
            return Some(target);
        }
    }

    None
}

/// Select a target within the designated wander area
fn select_area_wander_target(
    wander_action: &mut WanderAction,
    ai_pos: Position,
    map: &CurrentMap,
    random: &mut Random,
) -> Option<Position> {
    let area = wander_action.wander_area.as_ref()?;

    // Try to find a walkable position within the area
    for _ in 0..30 {
        let angle = random.f32(0.0..std::f32::consts::TAU);
        let distance = random.f32_inclusive(1.0..=area.radius as f32);

        let target_x = area.center.x + (angle.cos() * distance) as i32;
        let target_y = area.center.y + (angle.sin() * distance) as i32;
        let target = Position::new(target_x, target_y);

        if map.is_walkable(target) && target != ai_pos {
            return Some(target);
        }

        // Try slight variations if the direct escape route is blocked
        for offset_x in -2..=2 {
            for offset_y in -2..=2 {
                let variant_pos = Position::new(target_x + offset_x, target_y + offset_y);
                if map.is_walkable(variant_pos) {
                    return Some(variant_pos);
                }
            }
        }
    }

    // Fallback to random movement if area constraints are too restrictive
    select_random_wander_target(ai_pos, map, random)
}

/// Select the next patrol point in the patrol route
fn select_patrol_target(
    wander_action: &mut WanderAction,
    ai_pos: Position,
    _map: &CurrentMap,
    _random: &mut Random,
) -> Option<Position> {
    if wander_action.patrol_points.is_empty() {
        return None;
    }

    // Check if we've reached the current patrol point
    let current_target = wander_action.patrol_points.get(wander_action.current_patrol_index)?;
    if *current_target == ai_pos {
        // Move to next patrol point
        wander_action.current_patrol_index =
            (wander_action.current_patrol_index + 1) % wander_action.patrol_points.len();
    }

    wander_action.patrol_points.get(wander_action.current_patrol_index).copied()
}

/// Select a target for exploration (areas not recently visited)
fn select_exploration_target(ai_pos: Position, map: &CurrentMap, random: &mut Random) -> Option<Position> {
    // For now, use enhanced random selection that prefers distant positions
    let mut best_target = None;
    let mut best_distance = 0.0;

    for _ in 0..25 {
        let dx = random.i32(-8..=8);
        let dy = random.i32(-8..=8);
        let target = Position::new(ai_pos.x + dx, ai_pos.y + dy);

        if map.is_walkable(target) && target != ai_pos {
            let distance = ai_pos.distance(&target);
            if distance > best_distance {
                best_distance = distance;
                best_target = Some(target);
            }
        }
    }

    best_target
}

/// Check if the wander path should be regenerated or a new target selected
fn should_regenerate_wander_path(
    wander_action: &WanderAction,
    current_ai_pos: Position,
    map: &CurrentMap,
    current_turn: u64,
) -> bool {
    // Time-based regeneration (refresh target every 50-100 turns)
    // Note: Each turn advances time by 1000 units, so we need to divide by 1000 to get actual turns
    if let Some(last_target_time) = wander_action.last_target_time {
        let time_units_since_target = current_turn.saturating_sub(last_target_time);
        let turns_since_target = time_units_since_target / 1000;
        debug_ai!("turns since target: {turns_since_target} (time units: {time_units_since_target})");
        if turns_since_target > 75 {
            // Time to pick a new target
            return true;
        }
    }

    // No path exists
    if wander_action.current_path.is_empty() {
        debug_ai!("path is empty");
        return true;
    }

    // AI moved significantly from when path was generated
    if let Some(old_ai_pos) = wander_action.ai_pos_when_path_generated {
        let ai_moved_distance = old_ai_pos.distance(&current_ai_pos);
        if ai_moved_distance > 10.0 {
            debug_ai!("ai moved significantly from when path was generated");
            return true;
        }
    }

    // Current path step is blocked
    if let Some(next_pos) = wander_action.current_path.get(wander_action.path_index + 1)
        && !map.is_walkable(*next_pos)
    {
        debug_ai!("current path step is blocked");
        return true;
    }

    // Path is exhausted
    if wander_action.path_index >= wander_action.current_path.len().saturating_sub(1) {
        debug_ai!("path is exhausted");
        return true;
    }

    false
}

/// Follow the stored A* wander path and return the next direction to move
fn follow_stored_wander_path(
    wander_action: &mut WanderAction,
    current_ai_pos: Position,
    map: &CurrentMap,
) -> Option<Direction> {
    // Ensure we have a valid path
    if wander_action.current_path.is_empty() {
        return None;
    }

    // Find our current position in the path or the closest position
    let mut current_path_index = wander_action.path_index;

    // Verify we're at the expected position in the path
    if let Some(expected_pos) = wander_action.current_path.get(current_path_index)
        && *expected_pos != current_ai_pos
    {
        // Try to find our actual position in the path
        if let Some(found_index) = wander_action.current_path.iter().position(|&pos| pos == current_ai_pos) {
            current_path_index = found_index;
            wander_action.path_index = found_index;
        } else {
            // We're not on the path anymore, need to regenerate
            return None;
        }
    }

    // Get the next step in the path
    let next_index = current_path_index + 1;
    if let Some(next_pos) = wander_action.current_path.get(next_index) {
        // Check if the next position is walkable
        if !map.is_walkable(*next_pos) {
            // Path is blocked, need to regenerate
            return None;
        }

        // Calculate direction to next position
        let direction = helpers::calculate_direction_to_target(&current_ai_pos, next_pos);

        // Update path index for next time
        wander_action.path_index = next_index;

        direction
    } else {
        // Reached end of path
        None
    }
}
