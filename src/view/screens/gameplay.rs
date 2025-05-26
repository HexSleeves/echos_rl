//! The screen state for the main gameplay.

use bevy::prelude::*;

use crate::{
    model::{
        GameState,
        resources::{CurrentMap, FovMap, SpawnPoint, TurnQueue},
        systems::{
            FovSystemSet, camera_movement, compute_player_fov, monsters_turn, process_turns,
            spawn_map, spawn_player,
        },
    },
    view::{
        resources::TileMap,
        systems::{add_sprite_to_player, position_to_transform},
    },
};

use super::ScreenState;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(bevy_ecs_tilemap::TilemapPlugin);

    app.init_state::<GameState>()
        .init_resource::<TileMap>()
        .init_resource::<TurnQueue>()
        .init_resource::<CurrentMap>()
        .init_resource::<FovMap>()
        .init_resource::<SpawnPoint>();

    // ON ENTER
    app.add_systems(
        OnEnter(ScreenState::Gameplay),
        (spawn_map, spawn_player, compute_player_fov, process_turns).chain(),
    );

    // UPDATE
    app.add_systems(
        Update,
        (
            camera_movement,
            compute_player_fov.in_set(FovSystemSet::Compute),
            process_turns.run_if(in_state(GameState::ProcessTurns)),
            monsters_turn.run_if(in_state(GameState::MonstersTurn)),
        )
            .run_if(in_state(ScreenState::Gameplay)),
    );

    // POST UPDATE
    app.add_systems(
        PostUpdate,
        (position_to_transform, add_sprite_to_player).run_if(in_state(ScreenState::Gameplay)),
    );
}
