//! The rendering screen for gameplay presentation.

use bevy::prelude::*;

use super::ScreenState;
use crate::{
    core::states::GameState,
    rendering::systems::{
        add_sprite_to_entities, debug_fov_visualization, position_to_transform, 
        update_sprite_visibility, update_tilemap_visibility,
    },
};

// System sets for better organization
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum RenderingSystems {
    /// Visual updates after game logic
    Rendering,
    /// Transform and presentation updates
    Presentation,
}

/// Gameplay rendering plugin that handles visual presentation during gameplay
pub fn plugin(app: &mut App) {
    // Add tilemap plugin for rendering
    app.add_plugins(bevy_ecs_tilemap::TilemapPlugin);

    // Configure system sets ordering
    app.configure_sets(
        PostUpdate,
        (
            RenderingSystems::Rendering,
            RenderingSystems::Presentation,
        )
            .chain()
            .run_if(in_state(ScreenState::Gameplay)),
    );

    // === SPRITE MANAGEMENT ===
    // Always running systems for new entities
    app.add_systems(
        PostUpdate, 
        add_sprite_to_entities.run_if(in_state(ScreenState::Gameplay))
    );

    // === VISUAL UPDATES ===
    // Rendering updates after turn processing
    app.add_systems(
        PostUpdate,
        (
            update_tilemap_visibility, 
            update_sprite_visibility, 
            debug_fov_visualization
        )
            .chain()
            .in_set(RenderingSystems::Rendering)
            .run_if(in_state(GameState::ProcessTurns))
            .run_if(in_state(ScreenState::Gameplay)),
    );

    // === PRESENTATION ===
    // Transform updates
    app.add_systems(
        PostUpdate,
        position_to_transform
            .in_set(RenderingSystems::Presentation)
            .run_if(in_state(ScreenState::Gameplay)),
    );
}
