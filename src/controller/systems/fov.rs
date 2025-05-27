use bevy::prelude::*;

use crate::model::{
    components::{PlayerTag, Position, ViewShed},
    resources::{CurrentMap, FovMap},
};

/// System that computes FOV for all entities with a ViewShed component
pub fn compute_fov(
    map: Res<CurrentMap>,
    mut fov_map: ResMut<FovMap>,
    query: Query<(&Position, &ViewShed), With<PlayerTag>>,
) {
    if let Ok((player_pos, view_shed)) = query.single() {
        info!("Computing FOV for player at {:?}", player_pos);
        fov_map.compute_fov(&map, *player_pos, view_shed.radius);
    }
}
