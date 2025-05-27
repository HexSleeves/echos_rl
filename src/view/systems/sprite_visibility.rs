//! Sprite visibility systems for FOV-based rendering
//!
//! This module provides several systems for controlling sprite visibility based on
//! the Field of View (FOV) map. Choose the appropriate system based on your needs:
//!
//! ## Available Systems
//!
//! ### `update_sprite_visibility`
//! - **Use case**: General purpose sprite visibility
//! - **Behavior**: Shows all sprites with different alpha levels
//! - **States**: Visible (full), Revealed (dimmed), Unexplored (hidden)
//!
//! ### `update_entity_visibility`
//! - **Use case**: Simple binary visibility (show/hide only)
//! - **Behavior**: Only shows sprites in currently visible areas
//! - **States**: Visible or Hidden (no dimming)
//!
//! ### `update_typed_visibility`
//! - **Use case**: Different rules for different entity types
//! - **Behavior**: AI entities only visible in FOV, others visible in revealed areas
//! - **States**: Type-dependent visibility rules
//!
//! ## Integration Example
//! ```rust
//! app.add_systems(PostUpdate, update_sprite_visibility.in_set(FovSystemSet::React)); 
//! ```

use bevy::prelude::*;

use crate::model::{
    components::{AITag, PlayerTag, Position},
    resources::FovMap,
};

/// System that updates sprite visibility based on the FOV map
///
/// This system runs in the React phase of the FOV system set, after FOV computation.
/// It updates the visibility of sprites/entities based on what's currently visible
/// and what has been explored.
///
/// # Visibility Logic
/// - **Player**: Always visible (never hidden)
/// - **Visible entities**: Fully opaque (alpha = 1.0)
/// - **Revealed but not visible**: Semi-transparent (alpha = 0.4)
/// - **Unexplored entities**: Hidden (Visibility::Hidden)
///
/// # Performance Notes
/// - Only processes entities when the FOV map has changed
/// - Excludes player from visibility changes
/// - Uses efficient FOV map queries
pub fn update_sprite_visibility(
    fov_map: Res<FovMap>,
    mut q_sprites: Query<(&Position, &mut Visibility, &mut Sprite), (With<Sprite>, Without<PlayerTag>)>,
) {
    // Only update if the FOV map has changed
    if !fov_map.is_changed() {
        return;
    }

    // Update each sprite's visibility based on FOV state
    for (position, mut visibility, mut sprite) in q_sprites.iter_mut() {
        if fov_map.is_visible(*position) {
            // Entity is currently visible - show with full opacity
            *visibility = Visibility::Visible;
            sprite.color = Color::srgba(1.0, 1.0, 1.0, 1.0);
        }
        // else if fov_map.is_revealed(*position) {
        //     // Entity has been in explored area but not currently visible - show dimmed
        //     *visibility = Visibility::Visible;
        //     sprite.color = Color::srgba(0.7, 0.7, 0.7, 0.5);
        // }
        else {
            // Entity hasn't been in explored area - hide completely
            *visibility = Visibility::Hidden;
        }
    }
}

/// Alternative sprite visibility system that only affects non-player entities
/// and provides different visibility states
///
/// This version completely hides entities that aren't in visible areas,
/// which is more appropriate for enemies/NPCs that shouldn't be seen
/// through walls or in unexplored areas.
pub fn update_entity_visibility(
    fov_map: Res<FovMap>,
    mut q_entities: Query<(&Position, &mut Visibility), (Without<PlayerTag>, With<Sprite>)>,
) {
    // Only update if the FOV map has changed
    if !fov_map.is_changed() {
        return;
    }

    for (position, mut visibility) in q_entities.iter_mut() {
        if fov_map.is_visible(*position) {
            // Entity is in visible area
            *visibility = Visibility::Visible;
        } else {
            // Entity is not in visible area - hide it
            *visibility = Visibility::Hidden;
        }
    }
}

/// System that provides different visibility treatment for different entity types
///
/// This allows for more nuanced visibility rules:
/// - Living entities (monsters, NPCs): Only visible in current FOV
/// - Items: Visible in revealed areas (memory of where items were)
/// - Static objects: Visible in revealed areas
pub fn update_typed_visibility(
    fov_map: Res<FovMap>,
    // Entities that should only be visible in current FOV (enemies, NPCs)
    mut q_living: Query<(&Position, &mut Visibility), (With<Sprite>, Without<PlayerTag>, With<AITag>)>,
    // Entities that should be visible in revealed areas (items, decorations)
    mut q_static: Query<
        (&Position, &mut Visibility, &mut Sprite),
        (With<Sprite>, Without<PlayerTag>, Without<AITag>),
    >,
) {
    // Only update if the FOV map has changed
    if !fov_map.is_changed() {
        return;
    }

    // Update living entities - only visible in current FOV
    for (position, mut visibility) in q_living.iter_mut() {
        if fov_map.is_visible(*position) {
            *visibility = Visibility::Visible;
        } else {
            *visibility = Visibility::Hidden;
        }
    }

    // Update static entities - visible in revealed areas
    for (position, mut visibility, mut sprite) in q_static.iter_mut() {
        if fov_map.is_visible(*position) {
            // Currently visible - full opacity
            *visibility = Visibility::Visible;
            sprite.color = Color::srgba(1.0, 1.0, 1.0, 1.0);
        } else if fov_map.is_revealed(*position) {
            // In revealed area but not currently visible - dimmed
            *visibility = Visibility::Visible;
            sprite.color = Color::srgba(0.6, 0.6, 0.6, 0.7);
        } else {
            // Unexplored area - hidden
            *visibility = Visibility::Hidden;
        }
    }
}

/// Debug system that colors sprites based on their FOV state
///
/// Useful for debugging FOV calculations and sprite visibility logic.
/// Only enable during development.
#[allow(dead_code)]
pub fn debug_sprite_fov_visualization(
    fov_map: Res<FovMap>,
    mut q_sprites: Query<(&Position, &mut Sprite), (With<Sprite>, Without<PlayerTag>)>,
) {
    if !fov_map.is_changed() {
        return;
    }

    for (position, mut sprite) in q_sprites.iter_mut() {
        if fov_map.is_visible(*position) {
            // Green tint for visible sprites
            sprite.color = Color::srgba(0.8, 1.0, 0.8, 1.0);
        } else if fov_map.is_revealed(*position) {
            // Blue tint for revealed sprites
            sprite.color = Color::srgba(0.8, 0.8, 1.0, 0.7);
        } else {
            // Red tint for unexplored (this shouldn't normally be visible)
            sprite.color = Color::srgba(1.0, 0.8, 0.8, 0.3);
        }
    }
}
