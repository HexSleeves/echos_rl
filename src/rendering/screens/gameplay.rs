//! The rendering screen for gameplay presentation.

use bevy::prelude::*;

use super::ScreenState;
use crate::{
    core::states::GameState,
    rendering::systems::{
        add_sprite_to_entities, position_to_transform, update_sprite_visibility, update_tilemap_visibility,
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
        (RenderingSystems::Rendering, RenderingSystems::Presentation)
            .chain()
            .run_if(in_state(ScreenState::Gameplay)),
    );

    // === SPRITE MANAGEMENT ===
    // Always running systems for new entities
    app.add_systems(PostUpdate, add_sprite_to_entities.run_if(in_state(ScreenState::Gameplay)));

    // === VISUAL UPDATES ===
    // Rendering updates after turn processing
    app.add_systems(
        PostUpdate,
        (update_tilemap_visibility, update_sprite_visibility)
            .chain()
            .in_set(RenderingSystems::Rendering)
            .run_if(in_state(GameState::ProcessTurns))
            .run_if(in_state(ScreenState::Gameplay)),
    );

    // === PRESENTATION ===
    // Transform updates - only updates when Position components have changed (using change detection)
    app.add_systems(
        Last,
        position_to_transform.in_set(RenderingSystems::Presentation).run_if(in_state(ScreenState::Gameplay)),
    );
}

// /// System that provides visual feedback for FOV debugging
// ///
// /// Adds colored borders to tiles based on their FOV state:
// /// - Green border: Currently visible
// /// - Blue border: Revealed but not visible
// /// - No border: Unexplored
// ///
// /// Enable this system only during development/debugging.
// #[allow(dead_code)]
// pub fn debug_fov_visualization(fov_map: Res<FovMap>, mut q_tiles: Query<(&mut TileColor,
// &TilePos)>) {     if !fov_map.is_changed() {
//         return;
//     }

//     for (mut tile_color, tile_pos) in &mut q_tiles {
//         let position = Position::from(*tile_pos);

//         if fov_map.is_visible(position) {
//             let (r, g, b, a) = RenderingConstants::DEBUG_VISIBLE_COLOR;
//             tile_color.0 = Color::srgba(r, g, b, a);
//         } else if fov_map.is_revealed(position) {
//             let (r, g, b, a) = RenderingConstants::DEBUG_REVEALED_COLOR;
//             tile_color.0 = Color::srgba(r, g, b, a);
//         } else {
//             let (r, g, b, a) = RenderingConstants::DEBUG_UNEXPLORED_COLOR;
//             tile_color.0 = Color::srgba(r, g, b, a);
//         }
//     }
// }
