use bevy::prelude::*;

use crate::{
    core::components::{PlayerTag, Position, ViewShed},
    core::resources::{CurrentMap, FovAlgorithm, FovMap},
};

/// Generic cleanup system for removing entities with a specific component
pub fn cleanup_system<T: Component>(mut commands: Commands, q: Query<Entity, With<T>>) {
    for entity in q.iter() {
        commands.entity(entity).despawn();
    }
}

/// Cleanup component for entities that should be removed when the game exits
#[derive(Component)]
pub struct CleanupOnGameExit;

// ============================================================================
// FOV SYSTEMS
// ============================================================================

/// System that computes FOV for all entities with a ViewShed component
pub fn compute_fov(
    map: Res<CurrentMap>,
    mut fov_map: ResMut<FovMap>,
    query: Query<(&Position, &ViewShed), With<PlayerTag>>,
) {
    if let Ok((player_pos, view_shed)) = query.single() {
        debug!("Computing FOV");
        fov_map.compute_fov(&map, *player_pos, view_shed.radius as u8);
    }
}

/// System that allows toggling between FOV algorithms using the F key
pub fn toggle_fov_algorithm(keyboard_input: Res<ButtonInput<KeyCode>>, mut fov_map: ResMut<FovMap>) {
    if keyboard_input.just_pressed(KeyCode::KeyF) {
        let current_algorithm = fov_map.get_algorithm();
        let new_algorithm = match current_algorithm {
            FovAlgorithm::Raycasting => FovAlgorithm::Shadowcasting,
            FovAlgorithm::Shadowcasting => FovAlgorithm::Raycasting,
        };

        fov_map.set_algorithm(new_algorithm);
        info!("FOV algorithm changed from {:?} to {:?}", current_algorithm, new_algorithm);
    }
}
