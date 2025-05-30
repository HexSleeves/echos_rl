use bevy::prelude::*;
use big_brain::prelude::*;

use crate::model::{
    actions::WaitBuilder,
    components::{AIAction, AIState, IdleAction, TurnActor},
    types::GameActionBuilder,
};

// ============================================================================
// SCORER SYSTEMS (AI "Eyes" - Evaluate the world and assign scores)
// ============================================================================

/// System that handles idle behavior
pub fn idle_action_system(
    mut action_query: Query<(&Actor, &mut ActionState), With<IdleAction>>,
    mut ai_query: Query<(&mut TurnActor, &mut AIState)>,
) {
    for (Actor(actor_entity), mut action_state) in action_query.iter_mut() {
        if let Ok((mut turn_actor, mut ai_state)) = ai_query.get_mut(*actor_entity) {
            match *action_state {
                ActionState::Requested => {
                    info!("AI entity {:?} performing idle action (waiting)", actor_entity);
                    ai_state.current_action = AIAction::Idle;

                    // Queue a wait action so the turn system can continue
                    turn_actor.add_action(WaitBuilder::new().with_entity(*actor_entity).build());

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
