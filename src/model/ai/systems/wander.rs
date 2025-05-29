use bevy::prelude::*;
use big_brain::prelude::*;

use crate::model::{
    actions::WalkBuilder,
    components::{AIAction, AIBehavior, AIState, Position, TurnActor, WanderAction, WanderScorer},
    resources::CurrentMap,
    types::GameActionBuilder,
};

// ============================================================================
// SCORER SYSTEMS (AI "Eyes" - Evaluate the world and assign scores)
// ============================================================================

/// System that scores how much an AI wants to wander randomly
pub fn wander_scorer_system(
    mut scorer_query: Query<(&Actor, &mut Score), With<WanderScorer>>,
    ai_query: Query<&AIBehavior>,
) {
    for (Actor(actor_entity), mut score) in scorer_query.iter_mut() {
        if let Ok(_ai_behavior) = ai_query.get(*actor_entity) {
            // Base wander score - always some desire to move around
            let wander_score = 0.1 + fastrand::f32() * 0.2; // 0.1 to 0.3
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
                    ai_state.current_action = AIAction::Wander;

                    // Find a random walkable direction
                    if let Some(direction) =
                        crate::utils::find_random_walkable_direction(*ai_pos, &current_map)
                    {
                        turn_actor.add_action(
                            WalkBuilder::new()
                                .with_entity(*actor_entity)
                                .with_direction(direction)
                                .build(),
                        );
                        *action_state = ActionState::Success;
                    } else {
                        // No valid moves, just wait
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
