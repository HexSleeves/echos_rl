use bevy::{ecs::system::SystemState, prelude::*};
use brtk::prelude::Direction;

use crate::{
    core::{
        actions::WaitAction,
        components::{PlayerTag, Position},
        resources::{CurrentMap, TurnQueue},
        states::GameState,
        types::{ActionType, GameAction, GameError},
    },
    gameplay::{
        player::components::AwaitingInput, turns::components::TurnActor, world::components::TerrainType,
    },
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
            let (is_player, action_opt);
            {
                // Borrow world only for this inner scope
                let (mut next_state, mut q_actor) = state.get_mut(world);

                let Ok((_, mut actor, player)) = q_actor.get_mut(entity) else {
                    error!("Actor not found: {entity:?}");
                    continue;
                };

                if !actor.is_alive() {
                    info!("Actor is dead. Why is it still in the queue?");
                    continue;
                }

                is_player = player.is_some();
                action_opt = actor.next_action();

                if is_player && action_opt.is_none() {
                    next_state.set(GameState::GatherActions);
                    world.entity_mut(entity).insert(AwaitingInput);
                    turn_queue.schedule_turn(entity, time);
                    return;
                }
            } // ← all borrows of `world` released here

            let Some(action) = action_opt else {
                info!("No action for entity: {:?}. Rescheduling turn.", entity);
                turn_queue.schedule_turn(entity, time);
                continue;
            };

            // Process the action directly
            match perform_action(world, entity, action) {
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

/// Perform an action for the given entity
fn perform_action(world: &mut World, entity: Entity, action: ActionType) -> Result<u64, GameError> {
    match action {
        ActionType::Wait => WaitAction::new(entity).perform(world),
        ActionType::MoveDelta(direction) => match perform_move_delta(world, entity, direction) {
            Ok(()) => Ok(action.get_base_time_to_perform()),
            Err(e) => Err(e),
        },
        ActionType::Teleport(target_pos) => match perform_teleport(world, entity, target_pos) {
            Ok(()) => Ok(action.get_base_time_to_perform()),
            Err(e) => Err(e),
        },
        ActionType::Attack(target_pos) => match perform_attack(world, entity, target_pos) {
            Ok(()) => Ok(action.get_base_time_to_perform()),
            Err(e) => Err(e),
        },
        // ActionType::MoveTowards(target_pos) => match perform_move_towards(world, entity, target_pos) {
        //     Ok(()) => Ok(action.get_base_time_to_perform()),
        //     Err(e) => Err(e),
        // },
        _ => Err(GameError::Custom("Not implemented".to_string())),
    }
}

/// Perform a movement in a specific direction
fn perform_move_delta(world: &mut World, entity: Entity, direction: Direction) -> Result<(), GameError> {
    let mut state: SystemState<(ResMut<CurrentMap>, Query<&mut Position>)> = SystemState::new(world);
    let (current_map, mut q_position) = state.get_mut(world);

    // Get the entity's current position
    if let Ok(mut current_pos) = q_position.get_mut(entity) {
        let new_pos = *current_pos + direction.coord();

        let Some(terrain_type) = current_map.get_terrain(new_pos) else {
            error!("Failed to get terrain type for entity: {entity}");
            return Err(GameError::MissingComponent { entity, component: "TerrainType" });
        };

        match terrain_type {
            TerrainType::Wall => {
                error!("Wall in the way");
                return Err(GameError::MovementBlocked {
                    to: new_pos,
                    from: *current_pos,
                    reason: "Wall in the way".to_string(),
                });
            }
            _ => {
                *current_pos = new_pos;
                info!("Entity {} moved from {:?} to {:?}", entity, current_pos, new_pos);
            }
        }
    } else {
        return Err(GameError::EntityNotFound(entity));
    }

    // Apply the system state changes
    state.apply(world);
    Ok(())
}

/// Perform a movement to a specific position
fn perform_teleport(world: &mut World, entity: Entity, target_pos: Position) -> Result<(), GameError> {
    let mut state: SystemState<(ResMut<CurrentMap>, Query<&mut Position>)> = SystemState::new(world);
    let (current_map, mut q_position) = state.get_mut(world);

    // Get the entity's current position
    if let Ok(mut current_pos) = q_position.get_mut(entity) {
        let Some(terrain_type) = current_map.get_terrain(target_pos) else {
            error!("Failed to get terrain type for target position: {:?}", target_pos);
            return Err(GameError::MissingComponent { entity, component: "TerrainType" });
        };

        match terrain_type {
            TerrainType::Wall => {
                error!("Target position is blocked by wall");
                return Err(GameError::MovementBlocked {
                    from: *current_pos,
                    to: target_pos,
                    reason: "Target position blocked by wall".to_string(),
                });
            }
            _ => {
                *current_pos = target_pos;
                info!("Entity {} moved to {:?}", entity, target_pos);
            }
        }
    } else {
        return Err(GameError::EntityNotFound(entity));
    }

    // Apply the system state changes
    state.apply(world);
    Ok(())
}

/// Perform an attack action (placeholder implementation)
fn perform_attack(_world: &mut World, entity: Entity, target_pos: Position) -> Result<(), GameError> {
    // Placeholder implementation - just log the attack for now
    info!("Entity {} attacks position {:?}", entity, target_pos);

    // TODO: Implement actual attack logic when combat system is added
    // - Check if target position has an entity
    // - Calculate damage
    // - Apply damage to target
    // - Handle death/destruction

    Ok(())
}
