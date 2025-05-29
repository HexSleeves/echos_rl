use bevy::prelude::*;

use crate::model::{
    components::{PlayerTag, Position, ViewShed},
    resources::{CurrentMap, FovAlgorithm, FovMap},
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
