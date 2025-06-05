use bevy::prelude::*;
use big_brain::prelude::*;

use crate::{
    core::{
        components::{PlayerTag, Position},
        resources::{CurrentMap, FovMap, TurnQueue},
        types::ActionType,
    },
    gameplay::{
        enemies::{
            components::{AIAction, AIBehavior, AIState},
            helpers,
        },
        turns::components::TurnActor,
    },
    prelude::{
        assets::AIBehaviorType,
        gameplay::enemies::{FleeFromPlayerAction, FleeFromPlayerScorer},
    },
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
    current_map: Res<CurrentMap>,
    mut action_query: Query<(&Actor, &mut ActionState), With<FleeFromPlayerAction>>,
    mut ai_query: Query<(&Position, &mut TurnActor, &mut AIState)>,
) {
    let Ok(player_pos) = player_query.single() else {
        // No player found or multiple players - skip AI processing
        return;
    };

    for (Actor(actor_entity), mut action_state) in action_query.iter_mut() {
        if let Ok((ai_pos, mut turn_actor, mut ai_state)) = ai_query.get_mut(*actor_entity) {
            match *action_state {
                ActionState::Init => {
                    *action_state = ActionState::Requested;
                }
                ActionState::Requested => {
                    // Only add action if the entity doesn't already have actions queued
                    if turn_actor.peek_next_action().is_some() {
                        // Already has an action queued, wait for it to be processed
                        *action_state = ActionState::Executing;
                        continue;
                    }

                    info!(
                        "AI entity {:?} performing flee action away from player at {:?}",
                        actor_entity, player_pos
                    );
                    ai_state.current_action = Some(AIAction::FleeFromPlayer);

                    // Calculate direction away from player
                    let direction = helpers::calculate_direction_away_from_target(*ai_pos, *player_pos);

                    if let Some(dir) = direction {
                        // Check if the flee path is walkable
                        let (dx, dy) = dir.coord();
                        let target_pos = *ai_pos + (dx, dy);
                        if current_map.is_walkable(target_pos) {
                            turn_actor.queue_action(ActionType::MoveDelta(dir));
                            *action_state = ActionState::Executing;
                        } else {
                            // Try alternative directions if direct flee path is blocked
                            if let Some(alt_dir) =
                                helpers::find_alternative_flee_direction(*ai_pos, *player_pos, &current_map)
                            {
                                turn_actor.queue_action(ActionType::MoveDelta(alt_dir));
                                *action_state = ActionState::Executing;
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
                    // Check if the action has been processed (no more actions in queue)
                    if turn_actor.peek_next_action().is_none() {
                        *action_state = ActionState::Success;
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
}
