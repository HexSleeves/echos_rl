use bevy::prelude::*;

use crate::model::{components::Position, resources::CurrentMap};

/// Event for requesting actor movement
#[derive(Event, Debug)]
pub struct MoveActorEvent {
    pub actor: Entity,
    pub direction: Position, // Relative movement (e.g., Position::new(1, 0) for right)
}

/// Event for successful actor movement
#[derive(Event, Debug)]
pub struct ActorMovedEvent {
    pub actor: Entity,
    pub from: Position,
    pub to: Position,
}

/// System that handles actor movement requests
pub fn handle_actor_movement(
    mut move_events: EventReader<MoveActorEvent>,
    mut moved_events: EventWriter<ActorMovedEvent>,
    mut current_map: ResMut<CurrentMap>,
    mut actor_query: Query<&mut Position, With<Actor>>,
) {
    for move_event in move_events.read() {
        if let Ok(mut actor_position) = actor_query.get_mut(move_event.actor) {
            let current_pos = *actor_position;
            let new_pos =
                Position::new(current_pos.x() + move_event.direction.x(), current_pos.y() + move_event.direction.y());

            match current_map.move_actor(move_event.actor, new_pos) {
                Ok(old_pos) => {
                    // Update the actor's Position component
                    *actor_position = new_pos;

                    // Send success event
                    moved_events.send(ActorMovedEvent { actor: move_event.actor, from: old_pos, to: new_pos });
                }
                Err(error) => {
                    // Movement failed - could log or send failure event
                    warn!("Actor movement failed: {}", error);
                }
            }
        }
    }
}

/// System to sync actor positions with the map on startup or when actors are spawned
pub fn sync_actors_to_map(
    mut current_map: ResMut<CurrentMap>,
    actor_query: Query<(Entity, &Position), (With<Actor>, Added<Position>)>,
) {
    for (entity, position) in actor_query.iter() {
        if let Err(error) = current_map.place_actor(*position, entity) {
            warn!("Failed to place actor {:?} at {:?}: {}", entity, position, error);
        }
    }
}

/// System to remove actors from map when they're despawned
pub fn cleanup_despawned_actors(mut current_map: ResMut<CurrentMap>, mut removed_actors: RemovedComponents<Actor>) {
    for entity in removed_actors.read() {
        current_map.remove_actor(entity);
    }
}
