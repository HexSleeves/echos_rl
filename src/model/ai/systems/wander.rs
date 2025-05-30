use bevy::prelude::*;
use big_brain::prelude::*;

use crate::model::{
    actions::WalkBuilder,
    components::{
        AIAction, AIBehavior, AIBehaviorType, AIState, Position, TurnActor, WanderAction, WanderScorer,
    },
    resources::{CurrentMap, TurnQueue},
    types::GameActionBuilder,
};

// ============================================================================
// SCORER SYSTEMS (AI "Eyes" - Evaluate the world and assign scores)
// ============================================================================

/// System that scores how much an AI wants to wander randomly
pub fn wander_scorer_system(
    turn_queue: Res<TurnQueue>,
    mut scorer_query: Query<(&Actor, &mut Score), With<WanderScorer>>,
    ai_query: Query<(&AIBehavior, &TurnActor)>,
) {
    let current_turn = turn_queue.current_time();

    for (Actor(actor_entity), mut score) in scorer_query.iter_mut() {
        if let Ok((ai_behavior, turn_actor)) = ai_query.get(*actor_entity) {
            // Skip scoring if this entity already has an action queued
            if turn_actor.peak_next_action().is_some() {
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
                    0.15 + fastrand::f32() * 0.25 // 0.15 to 0.4
                }
            };

            info!(
                "Wander scorer for AI entity {:?}: {:.2} (behavior: {:?})",
                actor_entity, wander_score, ai_behavior.behavior_type
            );
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
                ActionState::Requested => {
                    info!("AI entity {:?} performing wander action at {:?}", actor_entity, ai_pos);
                    ai_state.current_action = AIAction::Wander;

                    // Find a random walkable direction
                    if let Some(direction) =
                        crate::utils::find_random_walkable_direction(*ai_pos, &current_map)
                    {
                        turn_actor.add_action(
                            WalkBuilder::new().with_entity(*actor_entity).with_direction(direction).build(),
                        );
                        info!("AI entity {:?} queued walk action in direction {:?}", actor_entity, direction);
                        *action_state = ActionState::Success;
                    } else {
                        // No valid moves, just wait
                        info!("AI entity {:?} has no valid moves, staying idle", actor_entity);
                        *action_state = ActionState::Success;
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

// ============================================================================
// ACTION SYSTEMS (AI "Hands" - Execute behaviors)
// ============================================================================
