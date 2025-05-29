use bevy::prelude::*;
use big_brain::prelude::*;

use crate::model::components::{AIAction, AIState, IdleAction};

// ============================================================================
// SCORER SYSTEMS (AI "Eyes" - Evaluate the world and assign scores)
// ============================================================================

/// System that handles idle behavior
pub fn idle_action_system(
    mut action_query: Query<(&Actor, &mut ActionState), With<IdleAction>>,
    mut ai_query: Query<&mut AIState>,
) {
    for (Actor(actor_entity), mut action_state) in action_query.iter_mut() {
        if let Ok(mut ai_state) = ai_query.get_mut(*actor_entity) {
            match *action_state {
                ActionState::Requested => {
                    ai_state.current_action = AIAction::Idle;
                    *action_state = ActionState::Success;
                }
                ActionState::Cancelled => {
                    *action_state = ActionState::Failure;
                }
                _ => {}
            }
        }
    }
}
