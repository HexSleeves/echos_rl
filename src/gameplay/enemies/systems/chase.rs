use bevy::prelude::*;
use big_brain::prelude::*;

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

                chase_action.generated_path = false;
                chase_action.last_seen_pt = Some(*player_pos);
                // ai_component.preferred_action = Some(ActionType::Movement(player_position));

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
            ActionState::Executing => {}
        }

        info!("{} executing chase!", ai_name);

        let position =
            if FovMap::can_see_entity(*ai_pos, ai_behavior.detection_range, *player_pos, &current_map) {
                // if in_attack_range(ai_position, player_position) {
                //     *action_state = ActionState::Success;
                //     continue;
                // }

                chase_action.last_seen_pt = Some(*player_pos);
                chase_action.generated_path = false;
                player_pos
            } else {
                let Some(last_seen) = chase_action.last_seen_pt else {
                    error!("Executing chase with no target.");
                    // ai_component.preferred_action = Some(ActionType::Wait);

                    ai_state.current_action = Some(AIAction::Idle);
                    ai_actor.queue_action(ActionType::Wait);

                    continue;
                };

                // We reached the end of our chase path and we do not see the player :(
                if last_seen == *ai_pos {
                    *action_state = ActionState::Failure;
                    continue;
                }

                // We have lost sight of the player and need a path to their last seen position.
                // Our pathfinder will only generate a valid path to the last seen location, this includes
                // partial path. We can expect the first element in the path to be a valid location
                // that is closest to the last_seen_pt.
                &if !chase_action.generated_path {
                    let path = generate_last_seen_path(*ai_pos, last_seen, &mut current_map);
                    let point = path.first().unwrap_or(&last_seen);

                    chase_action.generated_path = true;
                    chase_action.last_seen_pt = Some(*point);
                    *point
                } else {
                    last_seen
                }
            };

        println!("position: {position:?}");
        let dir = helpers::calculate_direction_to_target(*ai_pos, *position);
        println!("dir: {dir:?}");
        if let Some(dir) = dir {
            ai_state.current_action = Some(AIAction::ChasePlayer);
            ai_state.target_position = Some(*position);

            ai_actor.queue_action(ActionType::MoveDelta(dir));
        } else {
            info!("AI entity {:?} cannot find path to player, action failed", actor_entity);
            *action_state = ActionState::Failure;
        }

        // match ai_query.get_mut(*actor_entity) {
        //     Ok((ai_pos, mut turn_actor, mut ai_state)) => {
        //         match *action_state {
        //             ActionState::Init => {
        //                 println!("chase_player_action_system: Entity {:?} initializing action",
        // actor_entity);                 // Initialize the action
        //                 *action_state = ActionState::Requested;
        //             }
        //             ActionState::Requested => {
        //                 println!(
        //                     "chase_player_action_system: Entity {:?} processing requested
        // action",                     actor_entity
        //                 );
        //                 // Only add action if the entity doesn't already have actions queued
        //                 if turn_actor.peek_next_action().is_some() {
        //                     println!(
        //                         "chase_player_action_system: Entity {:?} already has action
        // queued, setting to executing",                         actor_entity
        //                     );
        //                     // Already has an action queued, wait for it to be processed
        //                     *action_state = ActionState::Executing;
        //                     continue;
        //                 }

        //                 info!(
        //                     "AI entity {:?} performing chase action toward player at {:?}",
        //                     actor_entity, player_pos
        //                 );
        //                 ai_state.current_action = Some(AIAction::ChasePlayer);
        //                 ai_state.target_position = Some(*player_pos);

        //                 // Calculate direction toward player
        //                 let direction = helpers::calculate_direction_to_target(*ai_pos,
        // *player_pos);

        //                 if let Some(dir) = direction {
        //                     // Check if the direct path is walkable
        //                     let (dx, dy) = dir.coord();
        //                     let target_pos = *ai_pos + (dx, dy);
        //                     if current_map.is_walkable(target_pos) {
        //                         println!(
        //                             "chase_player_action_system: Entity {:?} queuing walk action
        // in direction {:?}",                             actor_entity, dir
        //                         );
        //                         let _ = turn_actor.queue_action(
        //
        // Walk::builder().with_entity(*actor_entity).with_direction(dir).build(),
        //                         );
        //                         *action_state = ActionState::Executing;
        //                     } else {
        //                         // Try alternative directions if direct path is blocked
        //                         if let Some(alt_dir) =
        //                             helpers::find_alternative_direction(*ai_pos, *player_pos,
        // &current_map)                         {
        //                             println!(
        //                                 "chase_player_action_system: Entity {:?} queuing
        // alternative walk action in direction {:?}",
        // actor_entity, alt_dir                             );
        //                             let _ = turn_actor.queue_action(
        //                                 Walk::builder()
        //                                     .with_entity(*actor_entity)
        //                                     .with_direction(alt_dir)
        //                                     .build(),
        //                             );
        //                             *action_state = ActionState::Executing;
        //                         } else {
        //                             info!(
        //                                 "AI entity {:?} cannot find path to player, action
        // failed",                                 actor_entity
        //                             );
        //                             *action_state = ActionState::Failure;
        //                         }
        //                     }
        //                 } else {
        //                     // Already at player position or no valid direction
        //                     *action_state = ActionState::Success;
        //                 }
        //             }
        //             ActionState::Executing => {
        //                 // Check if the action has been processed (no more actions in queue)
        //                 if turn_actor.peek_next_action().is_none() {
        //                     println!(
        //                         "chase_player_action_system: Entity {:?} action completed,
        // setting to success",                         actor_entity
        //                     );
        //                     *action_state = ActionState::Success;
        //                 }
        //             }
        //             ActionState::Success | ActionState::Failure => {
        //                 println!(
        //                     "chase_player_action_system: Entity {:?} resetting action state to
        // init",                     actor_entity
        //                 );
        //                 // Action completed, reset to init and wait for next decision cycle
        //                 ai_state.current_action = None; // or whatever "idle" enum you use
        //                 ai_state.target_position = None;
        //                 *action_state = ActionState::Init;
        //             }
        //             ActionState::Cancelled => {
        //                 println!(
        //                     "chase_player_action_system: Entity {:?} action cancelled, resetting
        // to init",                     actor_entity
        //                 );
        //                 // Action was cancelled, reset to init
        //                 *action_state = ActionState::Init;
        //             }
        //         }
        //     }
        //     Err(e) => {
        //         println!(
        //             "chase_player_action_system: Entity {:?} missing required components: {:?}",
        //             actor_entity, e
        //         );
        //         // Entity doesn't have required components, skip it
        //         continue;
        //     }
        // }
    }
}

////////////////////////////
// HELPER FUNCTIONS
////////////////////////////

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

fn generate_last_seen_path(
    ai_pos: Position,
    target_pos: Position,
    map_provider: &mut CurrentMap,
) -> Vec<Position> {
    pathfinding::utils::find_path(ai_pos, target_pos, map_provider, true).unwrap_or_default()
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
