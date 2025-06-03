use bevy::prelude::*;

use crate::core::{
    components::{PlayerTag, Position, ViewShed},
    resources::{CurrentMap, FovMap},
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
    match query.single() {
        Ok((player_pos, view_shed)) => {
            debug!("Computing FOV for player at {:?}", player_pos);
            fov_map.compute_fov(&map, *player_pos, view_shed.radius as u8);
        }
        Err(bevy::ecs::query::QuerySingleError::NoEntities(_)) => {
            // No player entity found - this is normal during game initialization
            debug!("No player entity found for FOV computation");
        }
        Err(bevy::ecs::query::QuerySingleError::MultipleEntities(_)) => {
            // Multiple players found - this shouldn't happen in normal gameplay
            warn!("Multiple player entities found for FOV computation - using none to avoid ambiguity");
        }
    }
}
