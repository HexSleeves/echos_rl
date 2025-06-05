use bevy::prelude::*;
use big_brain::prelude::*;

use crate::{
    core::{
        components::Position,
        resources::{CurrentMap, TurnQueue},
        types::ActionType,
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
        let Ok((ai_behavior, turn_actor)) = ai_query.get(*actor_entity) else {
            warn!("Actor must have required components");
            continue;
        };

        // Don't score if already has actions queued
        if turn_actor.has_action() {
            score.set(0.0);
            continue;
        }

        // Calculate wander score based on behavior type and time
        let base_score = match ai_behavior.behavior_type {
            AIBehaviorType::Hostile => 0.2, // Low priority for hostile enemies
            AIBehaviorType::Passive => 0.4, // Medium priority for passive entities
            AIBehaviorType::Neutral => 0.6, // High priority for neutral wanderers
        };

        // Add some randomness and time-based variation
        let time_factor = (current_turn % 1000) as f32 / 1000.0;
        let final_score = base_score + (time_factor * 0.2);

        score.set(final_score);
    }
}

/// System that handles wandering behavior
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
                        turn_actor.queue_action(ActionType::MoveDelta(direction));
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
