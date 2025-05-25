//! The screen state for the main gameplay.

use bevy::prelude::*;

use super::ScreenState;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(bevy_ecs_tilemap::TilemapPlugin);

    app.add_systems(OnEnter(ScreenState::Gameplay), crate::model::systems::spawn_map);
    app.add_systems(Update, crate::model::systems::camera_movement.run_if(in_state(ScreenState::Gameplay)));
}
