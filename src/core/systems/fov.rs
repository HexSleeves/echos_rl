use bevy::prelude::*;

use crate::core::{
    components::{FieldOfView, PlayerTag, Position},
    resources::{CurrentMap, FovMap},
};

/// System that computes FOV for all entities with a ViewShed component
pub fn compute_fov(
    map: Res<CurrentMap>,
    mut fov_map: ResMut<FovMap>,
    query: Query<(&Position, &FieldOfView), With<PlayerTag>>,
) {
    match query.single() {
        Ok((player_pos, view_shed)) => {
            debug!("Computing FOV for player at {:?}", player_pos);
            fov_map.compute_fov(&map, *player_pos, **view_shed);
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
