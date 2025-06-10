use bevy::prelude::*;
use big_brain::prelude::*;

use crate::{
    core::components::{PlayerTag, Position},
    debug_ai,
    gameplay::{
        enemies::{components::AttackAction, helpers},
        turns::components::TurnActor,
    },
};

// ============================================================================
// ACTION SYSTEMS (Execute the AI's actions)
// ============================================================================

/// System that handles chasing the player
pub fn attack_player_action_system(
    player_query: Query<&Position, With<PlayerTag>>,
    mut ai_query: Query<(&Position, &mut TurnActor, &Name)>,
    mut action_query: Query<(&Actor, &mut ActionState), With<AttackAction>>,
) {
    let Ok(player_pos) = player_query.single() else {
        return;
    };

    for (Actor(actor_entity), mut action_state) in action_query.iter_mut() {
        let Ok((ai_pos, mut ai_actor, ai_name)) = ai_query.get_mut(*actor_entity) else {
            warn!("Actor must have required components");
            continue;
        };

        if ai_actor.has_action() {
            continue;
        }

        match *action_state {
            // Success | Failure
            ActionState::Success | ActionState::Failure => {
                // Nothing to do here
                debug_ai!("{} attack state: {:?}", ai_name, action_state);
                continue;
            }
            ActionState::Cancelled => {
                debug_ai!("{} cancelled attack!", ai_name);
                *action_state = ActionState::Failure;

                continue;
            }

            // these final two fall through to logic
            ActionState::Init | ActionState::Requested => {
                *action_state = ActionState::Executing;
            }
            ActionState::Executing => {}
        }

        if try_attack_player(&mut ai_actor, ai_pos, player_pos, ai_name) {
            *action_state = ActionState::Success;
        } else {
            *action_state = ActionState::Failure;

            if let Some(dir) = helpers::calculate_direction_to_target(ai_pos, player_pos) {
                ai_actor.queue_move_delta(dir);
            } else {
                warn!("{} is on the same tile as the player; skipping move queue", ai_name);
            }
        }
    }
}

fn try_attack_player(
    ai_actor: &mut TurnActor,
    ai_pos: &Position,
    player_pos: &Position,
    ai_name: &str,
) -> bool {
    if helpers::in_attack_range(ai_pos, player_pos) {
        // Use tactical direction calculation for smarter diagonal positioning
        if let Some(direction) = helpers::calculate_tactical_direction_to_target(ai_pos, player_pos) {
            ai_actor.queue_move_delta(direction);

            // Log different behavior based on attack type
            if helpers::is_diagonal_adjacent(ai_pos, player_pos) {
                debug_ai!("{} performing diagonal attack on player!", ai_name);
            } else {
                debug_ai!("{} moving toward player for tactical attack!", ai_name);
            }
            true
        } else {
            debug_ai!("{} cannot calculate tactical direction to player!", ai_name);
            false
        }
    } else {
        false
    }
}
