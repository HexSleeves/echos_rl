use bevy::prelude::*;

/// Despawns all entities that have the specified component type.
///
/// This generic system can be used to remove entities tagged with a particular component, such as marker components for cleanup.
///
/// # Examples
///
/// ```
/// // Schedules cleanup of all entities with the CleanupOnGameExit marker.
/// app.add_system(cleanup_system::<CleanupOnGameExit>);
/// ```
pub fn cleanup_system<T: Component>(mut commands: Commands, q: Query<Entity, With<T>>) {
    for entity in q.iter() {
        commands.entity(entity).despawn();
    }
}

/// Cleanup component for entities that should be removed when the game exits
#[derive(Component)]
pub struct CleanupOnGameExit;
