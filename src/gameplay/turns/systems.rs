use bevy::{ecs::system::SystemState, prelude::*};

use crate::{
    core::states::GameState,
    gameplay::{
        player::components::AwaitingInput,
        turns::{components::TurnActor, resources::TurnQueue},
    },
    core::components::PlayerTag,
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
            info!(
                "Turn queue cleanup: removed {} entities in {:?}",
                metrics.entities_removed, metrics.processing_time
            );
        }
        turn_queue.print_queue();

        while let Some((entity, time)) = turn_queue.get_next_actor() {
            let (mut next_state, mut q_actor) = state.get_mut(world);

            let Ok((entity, mut actor, player)) = q_actor.get_mut(entity) else {
                log::error!("Actor not found: {entity:?}");
                continue;
            };

            if !actor.is_alive() {
                info!("Actor is dead. Why is it still in the queue?");
                continue;
            }

            let is_player = player.is_some();
            let has_action = actor.peak_next_action().is_some();

            // Player is waiting for input
            if is_player && !has_action {
                info!("Player is awaiting input: {:?}", entity);
                next_state.set(GameState::GatherActions);
                world.entity_mut(entity).insert(AwaitingInput);
                turn_queue.schedule_turn(entity, time);
                return;
            }

            let Some(action) = actor.next_action() else {
                info!("No action for entity: {:?}. Rescheduling turn.", entity);
                turn_queue.schedule_turn(entity, time);
                return;
            };

            // Get next action and drop turn_queue borrow temporarily
            match action.perform(world) {
                Ok(d_time) => turn_queue.schedule_turn(entity, time + d_time),
                Err(e) => {
                    log::error!("Failed to perform action: {e:?}");

                    if is_player {
                        turn_queue.schedule_turn(entity, time);
                    } else {
                        turn_queue.schedule_turn(entity, time + 100);
                    }
                }
            }
        }
    });
}
