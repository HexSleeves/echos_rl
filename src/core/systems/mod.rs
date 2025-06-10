use bevy::prelude::*;

pub mod combat;
pub mod fov;
pub mod light;

/// Cleanup component for entities that should be removed when the game exits
#[derive(Component)]
pub struct CleanupOnGameExit;

/// Generic cleanup system for removing entities with a specific component
pub fn cleanup_system<T: Component>(mut commands: Commands, q: Query<Entity, With<T>>) {
    for entity in q.iter() {
        commands.entity(entity).despawn();
    }
}
