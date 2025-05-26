//! The screen state for the main gameplay.

use bevy::prelude::*;

use crate::{
    model::systems::{camera_movement, spawn_map, spawn_player},
    view::{
        resources::TileMap,
        systems::{add_sprite_to_player, position_to_transform},
    },
};

use super::ScreenState;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(bevy_ecs_tilemap::TilemapPlugin);

    app.init_resource::<TileMap>();

    app.add_systems(OnEnter(ScreenState::Gameplay), (spawn_map, spawn_player).chain())
        .add_systems(Update, (camera_movement,).run_if(in_state(ScreenState::Gameplay)))
        .add_systems(PostUpdate, (position_to_transform, add_sprite_to_player).run_if(in_state(ScreenState::Gameplay)));
}
