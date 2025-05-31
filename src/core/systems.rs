use bevy::prelude::*;

/// Generic cleanup system for removing entities with a specific component
pub fn cleanup_system<T: Component>(mut commands: Commands, q: Query<Entity, With<T>>) {
    for entity in q.iter() {
        commands.entity(entity).despawn();
    }
}

/// Cleanup component for entities that should be removed when the game exits
#[derive(Component)]
pub struct CleanupOnGameExit;
