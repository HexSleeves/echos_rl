use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::model::{components::Position, resources::FovMap};

const VISIBLE_COLOR: Color = Color::srgba(0.8, 1.0, 0.8, 1.0);
const REVEALED_COLOR: Color = Color::srgba(0.8, 0.8, 1.0, 0.6);

/// System that updates tilemap tile visibility based on the FOV map
///
/// This system runs in the React phase of the FOV system set, after FOV computation.
/// It updates the visibility of individual tiles in the tilemap based on what's
/// currently visible and what has been explored.
///
/// # Visibility Logic
/// - **Visible tiles**: Fully opaque (alpha = 1.0)
/// - **Explored but not visible**: Semi-transparent (alpha = 0.4)
/// - **Unexplored tiles**: Hidden (TileVisible = false)
///
/// # Performance Notes
/// - Only processes tiles when the FOV map has changed
/// - Uses efficient position-to-tile mapping
/// - Leverages bevy_ecs_tilemap's built-in visibility system
pub fn update_tilemap_visibility(
    fov_map: Res<FovMap>,
    // current_map: Res<CurrentMap>,
    mut q_tiles: Query<(&mut TileVisible, &mut TileColor, &TilePos)>,
) {
    // Only update if the FOV map has changed
    if !fov_map.is_changed() {
        return;
    }

    // Update each tile's visibility based on FOV state
    for (mut tile_visible, mut tile_color, tile_pos) in q_tiles.iter_mut() {
        let position = Position::from(*tile_pos);

        if fov_map.is_visible(position) {
            // Tile is currently visible - show with full opacity
            tile_visible.0 = true;
            tile_color.0 = VISIBLE_COLOR;
        } else if fov_map.is_revealed(position) {
            // Tile has been explored but not currently visible - show dimmed
            tile_visible.0 = true;
            tile_color.0 = REVEALED_COLOR;
        } else {
            // Tile hasn't been explored - hide completely
            tile_visible.0 = false;
        }
    }
}

const DEBUG_VISIBLE_COLOR: Color = Color::srgba(0.8, 1.0, 0.8, 1.0);
const DEBUG_REVEALED_COLOR: Color = Color::srgba(0.8, 0.8, 1.0, 0.6);
const DEBUG_UNEXPLORED_COLOR: Color = Color::srgba(1.0, 1.0, 1.0, 1.0);

/// System that provides visual feedback for FOV debugging
///
/// Adds colored borders to tiles based on their FOV state:
/// - Green border: Currently visible
/// - Blue border: Revealed but not visible
/// - No border: Unexplored
///
/// Enable this system only during development/debugging.
#[allow(dead_code)]
pub fn debug_fov_visualization(
    fov_map: Res<FovMap>,
    mut q_tiles: Query<(&mut TileColor, &TilePos)>,
) {
    if !fov_map.is_changed() {
        return;
    }

    for (mut tile_color, tile_pos) in q_tiles.iter_mut() {
        let position = Position::from(*tile_pos);

        if fov_map.is_visible(position) {
            // Green tint for visible tiles
            tile_color.0 = DEBUG_VISIBLE_COLOR;
        } else if fov_map.is_revealed(position) {
            // Blue tint for revealed tiles
            tile_color.0 = DEBUG_REVEALED_COLOR;
        } else {
            // Default color for unexplored
            tile_color.0 = DEBUG_UNEXPLORED_COLOR;
        }
    }
}
