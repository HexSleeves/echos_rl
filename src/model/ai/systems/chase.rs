use bevy::prelude::*;
use big_brain::prelude::*;
use std::f32;

use crate::model::{
    actions::WalkBuilder,
    components::{
        AIAction, AIBehavior, AIBehaviorType, AIState, ChasePlayerAction, ChasePlayerScorer, PlayerTag,
        Position, TurnActor,
    },
    resources::{CurrentMap, FovMap, TurnQueue},
    types::GameActionBuilder,
};

// ============================================================================
// SCORER SYSTEMS (AI "Eyes" - Evaluate the world and assign scores)
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

                    // Higher score for closer players, within detection range
                    if distance <= ai_behavior.detection_range as f32 {
                        // Player is visible! Update last seen turn
                        ai_behavior.mark_player_seen(current_turn, *player_pos);

                        chase_score = (ai_behavior.detection_range as f32 - distance)
                            / ai_behavior.detection_range as f32;
                        chase_score = chase_score.clamp(0.0, 1.0);
                    }
                }
                // If player was recently seen but not currently visible, still want to chase
                else if let Some(last_pos) = ai_behavior.last_known_player_position {
                    if !ai_behavior.should_switch_to_wander(current_turn) {
                        let distance = crate::utils::calculate_distance(*ai_pos, last_pos);
                        if distance <= ai_behavior.detection_range as f32 {
                            chase_score = 0.5; // Moderate score for pursuing last known position
                        }
                    }
                }
            }

            score.set(chase_score);
        }
    }
}

// ============================================================================
// ACTION SYSTEMS (AI "Hands" - Execute behaviors)
// ============================================================================

pub fn chase_player_action_system(
    player_query: Query<&Position, With<PlayerTag>>,
    current_map: Res<CurrentMap>,
    turn_queue: Res<TurnQueue>,
    mut action_query: Query<(&Actor, &mut ActionState), With<ChasePlayerAction>>,
    mut ai_query: Query<(&Position, &mut TurnActor, &mut AIBehavior, &mut AIState)>,
) {
    let Ok(player_pos) = player_query.single() else {
        return;
    };

    let current_turn = turn_queue.current_time();

    for (Actor(actor_entity), mut action_state) in action_query.iter_mut() {
        if let Ok((ai_pos, mut turn_actor, mut ai_behavior, mut ai_state)) = ai_query.get_mut(*actor_entity) {
            match *action_state {
                ActionState::Requested => {
                    // Update last known player position and turn
                    ai_behavior.mark_player_seen(current_turn, *player_pos);
                    ai_state.current_action = AIAction::ChasePlayer;
                    ai_state.target_position = Some(*player_pos);

                    // Calculate direction to player
                    let direction = crate::utils::calculate_direction_to_target(*ai_pos, *player_pos);

                    if let Some(dir) = direction {
                        let new_position = *ai_pos + dir.coord();

                        // Check if the new position is walkable
                        if current_map.is_walkable(new_position)
                            && current_map.get_actor(new_position).is_none()
                        {
                            // Queue the walk action
                            turn_actor.add_action(
                                WalkBuilder::new().with_entity(*actor_entity).with_direction(dir).build(),
                            );
                            *action_state = ActionState::Success;
                        } else {
                            // Can't move in that direction, try alternative
                            if let Some(alt_dir) =
                                crate::utils::find_alternative_direction(*ai_pos, *player_pos, &current_map)
                            {
                                turn_actor.add_action(
                                    WalkBuilder::new()
                                        .with_entity(*actor_entity)
                                        .with_direction(alt_dir)
                                        .build(),
                                );
                                *action_state = ActionState::Success;
                            } else {
                                *action_state = ActionState::Failure;
                            }
                        }
                    } else {
                        *action_state = ActionState::Failure;
                    }
                }
                ActionState::Cancelled => {
                    *action_state = ActionState::Failure;
                }
                _ => {}
            }
        }
    }
}
