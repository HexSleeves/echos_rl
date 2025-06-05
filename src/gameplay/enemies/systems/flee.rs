use bevy::prelude::*;
use big_brain::prelude::*;

use crate::{
    core::{
        actions::Walk,
        components::{PlayerTag, Position},
        resources::{CurrentMap, FovMap, TurnQueue},
        types::{BuildableGameAction, GameActionBuilder},
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
        // No player found or multiple players - skip AI processing
        return;
    };

    let current_turn = turn_queue.current_time();

    for (Actor(actor_entity), mut score) in scorer_query.iter_mut() {
        if let Ok((ai_pos, mut ai_behavior, turn_actor)) = ai_query.get_mut(*actor_entity) {
            // Skip scoring if this entity already has an action queued
            if turn_actor.peek_next_action().is_some() {
                continue;
            }

            let mut flee_score = 0.0;

            // Only passive enemies want to flee
            if ai_behavior.behavior_type == AIBehaviorType::Passive {
                // Check if AI can see the player using the AI's own FOV
                if FovMap::can_see_entity(*ai_pos, ai_behavior.detection_range, *player_pos, &current_map) {
                    let distance = ai_pos.ai_detection_distance(player_pos);
                    if distance <= ai_behavior.detection_range as f32 {
                        // Update AI's knowledge of player position
                        ai_behavior.update_player_sighting(*player_pos, current_turn);

                        // Higher score for closer threats (more urgent to flee)
                        flee_score = 1.0 - (distance / ai_behavior.detection_range as f32);
                        flee_score = flee_score.clamp(0.0, 1.0);

                        info!(
                            "AI entity {:?} sees threat at distance {:.1}, flee score: {:.2}",
                            actor_entity, distance, flee_score
                        );
                    }
                }
            }

            score.set(flee_score);
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
                            let _ = turn_actor.queue_action(
                                Walk::builder().with_entity(*actor_entity).with_direction(dir).build(),
                            );
                            *action_state = ActionState::Executing;
                        } else {
                            // Try alternative directions if direct flee path is blocked
                            if let Some(alt_dir) =
                                helpers::find_alternative_flee_direction(*ai_pos, *player_pos, &current_map)
                            {
                                let _ = turn_actor.queue_action(
                                    Walk::builder()
                                        .with_entity(*actor_entity)
                                        .with_direction(alt_dir)
                                        .build(),
                                );
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
