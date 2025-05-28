//! The screen state for the main gameplay.

use bevy::prelude::*;

use super::ScreenState;
use crate::{
    controller::systems::{
        camera_movement, compute_fov, monsters_turn, process_turns, spawn_map, spawn_player,
    },
    model::{
        GameState,
        commands::process_spawn_commands,
        resources::{CurrentMap, FovMap, SpawnPoint, TurnQueue},
    },
    view::{
        resources::TileMap,
        systems::{
            add_sprite_to_player, debug_fov_visualization, position_to_transform, update_sprite_visibility,
            update_tilemap_visibility,
        },
    },
};

// System sets for better organization
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameplaySystems {
    /// Initial setup when entering gameplay
    Initialization,
    /// Core game logic during turns
    TurnProcessing,
    /// Visual updates after game logic
    Rendering,
    /// Camera and transform updates
    Presentation,
}

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(bevy_ecs_tilemap::TilemapPlugin);

    // Initialize resources
    app.init_state::<GameState>()
        .init_resource::<TileMap>()
        .init_resource::<TurnQueue>()
        .init_resource::<CurrentMap>()
        .init_resource::<FovMap>()
        .init_resource::<SpawnPoint>();

    // Configure system sets ordering
    app.configure_sets(
        PostUpdate,
        (GameplaySystems::TurnProcessing, GameplaySystems::Rendering, GameplaySystems::Presentation)
            .chain()
            .run_if(in_state(ScreenState::Gameplay)),
    );

    // === INITIALIZATION ===
    // Only run setup systems when entering the gameplay screen
    app.add_systems(
        OnEnter(ScreenState::Gameplay),
        (spawn_map, spawn_player).chain().in_set(GameplaySystems::Initialization),
    );

    // === MAIN GAME LOOP ===
    // Core turn processing
    app.add_systems(
        Update,
        (
            process_spawn_commands,
            process_turns.run_if(in_state(GameState::ProcessTurns)),
            monsters_turn.run_if(in_state(GameState::MonstersTurn)),
        )
            .in_set(GameplaySystems::TurnProcessing)
            .run_if(in_state(ScreenState::Gameplay)),
    );

    // === VISUAL UPDATES ===
    // Always running systems for new entities
    app.add_systems(PostUpdate, add_sprite_to_player.run_if(in_state(ScreenState::Gameplay)));

    // Rendering updates after turn processing
    app.add_systems(
        PostUpdate,
        (compute_fov, update_tilemap_visibility, update_sprite_visibility, debug_fov_visualization)
            .chain()
            .in_set(GameplaySystems::Rendering)
            .run_if(in_state(GameState::ProcessTurns)),
    );

    // === PRESENTATION ===
    // Camera and transform updates
    app.add_systems(
        PostUpdate,
        (camera_movement, position_to_transform).chain().in_set(GameplaySystems::Presentation),
    );
}
