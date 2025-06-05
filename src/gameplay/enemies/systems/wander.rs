use bevy::prelude::*;
use big_brain::prelude::*;

use crate::{
    core::{
        actions::Walk,
        components::Position,
        resources::{CurrentMap, TurnQueue},
        types::{BuildableGameAction, GameActionBuilder},
    },
    gameplay::{
        enemies::{
            components::{AIAction, AIBehavior, AIState, WanderAction, WanderScorer},
            helpers,
        },
        turns::components::TurnActor,
    },
    prelude::assets::AIBehaviorType,
};

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
            if turn_actor.peek_next_action().is_some() {
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
                    // Only add action if the entity doesn't already have actions queued
                    if turn_actor.peek_next_action().is_some() {
                        // Already has an action queued, wait for it to be processed
                        *action_state = ActionState::Executing;
                        continue;
                    }

                    info!("AI entity {:?} performing wander action at {:?}", actor_entity, ai_pos);
                    ai_state.current_action = Some(AIAction::Wander);

                    // Find a random walkable direction
                    if let Some(direction) = helpers::find_random_walkable_direction(*ai_pos, &current_map) {
                        let _ = turn_actor.queue_action(
                            Walk::builder().with_entity(*actor_entity).with_direction(direction).build(),
                        );
                        *action_state = ActionState::Executing;
                    } else {
                        info!("AI entity {:?} cannot find walkable direction for wandering", actor_entity);
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
