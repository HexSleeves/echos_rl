use bevy::{ecs::system::SystemState, prelude::*};

use crate::{
    core::{
        components::PlayerTag,
        resources::TurnQueue,
        states::GameState,
        types::{GameAction, GameError},
    },
    debug_turns,
    gameplay::{player::components::AwaitingInput, turns::components::TurnActor},
};

/// System that processes turns in the turn queue
pub fn process_turns(world: &mut World) {
    let mut state: SystemState<(
        ResMut<NextState<GameState>>,
        Query<(Entity, &mut TurnActor, Option<&PlayerTag>)>,
    )> = SystemState::new(world);

    world.resource_scope(|world, mut turn_queue: Mut<TurnQueue>| {
        // Periodically clean up the queue
        let metrics = turn_queue.cleanup_dead_entities(world);

        // Log significant cleanups
        if metrics.entities_removed > 10 {
            debug_turns!(
                "Turn queue cleanup: removed {} entities in {:?}",
                metrics.entities_removed,
                metrics.processing_time
            );
        }
        turn_queue.print_queue();

        while let Some((entity, time)) = turn_queue.get_next_actor() {
            let (is_player, action_opt);
            {
                // Borrow world only for this inner scope
                let (mut next_state, mut q_actor) = state.get_mut(world);

                let Ok((_, mut actor, player)) = q_actor.get_mut(entity) else {
                    error!("Actor not found: {entity:?}");
                    continue;
                };

                if !actor.is_alive() {
                    error!("Actor is dead. Why is it still in the queue?");
                    continue;
                }

                is_player = player.is_some();
                action_opt = actor.next_action();

                if is_player && action_opt.is_none() {
                    debug_turns!("Player has no action. Scheduling turn.");

                    next_state.set(GameState::GatherActions);
                    world.entity_mut(entity).insert(AwaitingInput);
                    turn_queue.schedule_turn(entity, time);
                    return;
                }
            } // ← all borrows of `world` released here

            let Some(action) = action_opt else {
                debug_turns!("No action for entity: {:?}. Rescheduling turn.", entity);
                turn_queue.schedule_turn(entity, time);
                continue;
            };

            // Process the action using the new trait-based system
            match execute_action(world, entity, action) {
                Ok(d_time) => turn_queue.schedule_turn(entity, time + d_time),
                Err(e) => {
                    error!("Failed to perform action: {e:?}");

                    if is_player {
                        turn_queue.schedule_turn(entity, time);
                    } else {
                        turn_queue.schedule_turn(entity, time + 1000);
                    }
                }
            }
        }
    });
}

/// Execute an action using the new trait-based system
fn execute_action(world: &mut World, entity: Entity, action: Box<dyn GameAction>) -> Result<u64, GameError> {
    debug_turns!("Executing action: {:?}", action.action_type());

    // Check if this is a wrapper action that needs to be converted to a proper action
    // We use a simple approach: check the debug string to identify wrapper actions
    let debug_str = format!("{action:?}");
    if debug_str.contains("ActionTypeWrapper") {
        // This is a wrapper - extract the ActionType and convert to proper action
        let action_type = action.action_type();
        let proper_action = action_type.to_action(entity);
        return proper_action.execute(world);
    }

    // Execute the action directly
    action.execute(world)
}
