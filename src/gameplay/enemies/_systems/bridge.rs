use bevy::prelude::*;

use crate::{
    core::{
        actions::{WaitBuilder, Walk},
        components::Position,
        types::{ActionType, BuildableGameAction, GameActionBuilder},
    },
    gameplay::{enemies::components::AIComponent, turns::components::TurnActor},
    utils::calculate_direction_to_target,
};

/// Bridge system that converts AI preferred actions into TurnActor actions
/// This system runs after big-brain actions have set preferred_action on AIComponent
pub fn ai_action_bridge_system(mut ai_query: Query<(Entity, &Position, &mut AIComponent, &mut TurnActor)>) {
    for (entity, position, mut ai_component, mut turn_actor) in ai_query.iter_mut() {
        // Only process if AI has a preferred action and TurnActor has no queued actions
        if let Some(preferred_action) = ai_component.preferred_action.take() {
            if turn_actor.peek_next_action().is_none() {
                info!("Converting AI preferred action {:?} for entity {:?}", preferred_action, entity);

                match preferred_action {
                    ActionType::Wait => {
                        let action = WaitBuilder::new().with_entity(entity).build();
                        turn_actor.add_action(action);
                    }
                    ActionType::Move(target_position) => {
                        // Calculate direction from current position to target
                        if let Some(direction) = calculate_direction_to_target(*position, target_position) {
                            let action =
                                Walk::builder().with_entity(entity).with_direction(direction).build();
                            turn_actor.add_action(action);
                        } else {
                            warn!(
                                "Could not calculate direction from {:?} to {:?} for entity {:?}",
                                position, target_position, entity
                            );
                            // Fallback to wait action
                            let action = WaitBuilder::new().with_entity(entity).build();
                            turn_actor.add_action(action);
                        }
                    }
                    ActionType::MoveDelta(direction) => {
                        let action = Walk::builder().with_entity(entity).with_direction(direction).build();
                        turn_actor.add_action(action);
                    }
                    ActionType::Attack(target_position) => {
                        // TODO: Implement attack action when available
                        warn!(
                            "Attack action not yet implemented for entity {:?}, using wait instead",
                            entity
                        );
                        let action = WaitBuilder::new().with_entity(entity).build();
                        turn_actor.add_action(action);
                    }
                }
            }
        }
    }
}
