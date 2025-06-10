use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::{TileColor, TilePos};

use crate::core::{
    components::Position,
    resources::{FovMap, LightMap},
};

// ============================================================================
// TILEMAP SYSTEMS
// ============================================================================

/// System that updates tilemap visibility and lighting based on FOV and LightMap
pub fn update_tilemap_visibility(
    fov_map: Res<FovMap>,
    light_map: Res<LightMap>,
    mut q_tiles: Query<(&mut TileColor, &TilePos)>,
) {
    // Update when either FOV or lighting changes
    if !fov_map.is_changed() && !light_map.is_changed() {
        return;
    }

    for (mut tile_color, tile_pos) in &mut q_tiles {
        let position = Position::from(*tile_pos);

        if fov_map.is_visible(position) {
            // Get light color for this position
            let light_color = light_map.get_light((position.x, position.y));

            // Combine base white color with light color
            // Ensure minimum visibility even in darkness
            let min_light = 0.2; // Minimum light level for gameplay visibility
            let light_linear = light_color.to_linear();
            let final_color = Color::linear_rgb(
                (light_linear.red + min_light).min(1.0),
                (light_linear.green + min_light).min(1.0),
                (light_linear.blue + min_light).min(1.0),
            );

            tile_color.0 = final_color;
        } else if fov_map.is_revealed(position) {
            // Revealed areas are dimmed and desaturated (fog of war)
            tile_color.0 = Color::srgba(0.4, 0.4, 0.4, 1.0);
        } else {
            // Unexplored areas are black
            tile_color.0 = Color::BLACK;
        }
    }
}
