use bevy::prelude::*;
use big_brain::prelude::*;

use crate::{
    core::types::ActionType, debug_ai, gameplay::turns::components::TurnActor,
    prelude::gameplay::enemies::IdleAction,
};

/// System that handles idle behavior
pub fn idle_action_system(
    mut action_query: Query<(&Actor, &mut ActionState), With<IdleAction>>,
    mut ai_query: Query<&mut TurnActor>,
) {
    for (Actor(actor_entity), mut action_state) in action_query.iter_mut() {
        if let Ok(mut turn_actor) = ai_query.get_mut(*actor_entity) {
            match *action_state {
                ActionState::Init => {
                    *action_state = ActionState::Requested;
                }
                ActionState::Requested => {
                    // Only process if the entity doesn't already have actions queued
                    if turn_actor.peek_next_action().is_some() {
                        // Already has an action queued, wait for it to be processed
                        *action_state = ActionState::Executing;
                        continue;
                    }

                    debug_ai!("AI entity {:?} performing idle action", actor_entity);

                    // Add a wait action to the queue for idle behavior
                    turn_actor.queue_action(ActionType::Wait);

                    *action_state = ActionState::Executing;
                }
                ActionState::Executing => {
                    // Check if the action has been processed (no more actions in queue)
                    if turn_actor.peek_next_action().is_none() {
                        *action_state = ActionState::Success;
                    }
                }
                ActionState::Success | ActionState::Failure => {
                    // Reset to init for next cycle
                    *action_state = ActionState::Init;
                }
                ActionState::Cancelled => {
                    *action_state = ActionState::Init;
                }
            }
        }
    }
}
