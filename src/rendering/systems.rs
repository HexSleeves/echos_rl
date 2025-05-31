use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::{TileColor, TilePos};

use crate::{
    model::{
        components::{AITag, PlayerTag, Position},
        resources::FovMap,
        ModelConstants,
    },
    rendering::{
        components::TileSprite,
        resources::TileMap,
        RenderingConstants,
    },
};

// ============================================================================
// SPRITE MANAGEMENT SYSTEMS
// ============================================================================

/// System that adds sprite components to entities with TileSprite
pub fn add_sprite_to_entities(
    mut commands: Commands,
    tilemap: Option<Res<TileMap>>,
    q_actors: Query<(Entity, &TileSprite), Without<Sprite>>,
) {
    // If the tilemap resource isn't available yet, we can't add sprites
    let Some(tilemap) = tilemap else {
        return;
    };

    for (entity, tile_sprite) in &q_actors {
        // Generate the sprite using our helper method
        let index = tilemap.coords_to_index(tile_sprite.tile_coords);
        let sprite = tilemap.generate_sprite_for_terrain(index);

        // Add the sprite components to the entity
        commands.entity(entity).insert(sprite);
    }
}

/// System that converts position components to transform components
pub fn position_to_transform(mut q_objects: Query<(&Position, &mut Transform)>) {
    for (position, mut transform) in &mut q_objects {
        *transform = transform_from_position(position);
    }
}

/// Helper function to convert a position to a transform
pub fn transform_from_position(position: &Position) -> Transform {
    Transform::from_xyz(
        position.x() as f32 * RenderingConstants::TILE_SIZE
            - (ModelConstants::MAP_WIDTH as f32 * RenderingConstants::HALF_TILE_SIZE),
        position.y() as f32 * RenderingConstants::TILE_SIZE
            - (ModelConstants::MAP_HEIGHT as f32 * RenderingConstants::HALF_TILE_SIZE),
        1.0,
    )
}

// ============================================================================
// VISIBILITY SYSTEMS
// ============================================================================

/// System that updates sprite visibility based on the FOV map
///
/// This system runs after FOV computation and updates the visibility of sprites/entities
/// based on what's currently visible and what has been explored.
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
    // Only update when FOV changes to avoid unnecessary work
    if !fov_map.is_changed() {
        return;
    }

    for (position, mut visibility, mut sprite) in &mut q_sprites {
        if fov_map.is_visible(*position) {
            *visibility = Visibility::Visible;
            sprite.color.set_alpha(1.0);
        } else if fov_map.is_revealed(*position) {
            *visibility = Visibility::Visible;
            sprite.color.set_alpha(0.4); // Semi-transparent for revealed areas
        } else {
            *visibility = Visibility::Hidden;
        }
    }
}

/// System that provides simple binary visibility (show/hide only)
///
/// This is a simpler alternative to `update_sprite_visibility` that only shows
/// sprites in currently visible areas, hiding everything else.
pub fn update_entity_visibility(
    fov_map: Res<FovMap>,
    mut q_entities: Query<(&Position, &mut Visibility), (With<Sprite>, Without<PlayerTag>)>,
) {
    if !fov_map.is_changed() {
        return;
    }

    for (position, mut visibility) in &mut q_entities {
        *visibility = if fov_map.is_visible(*position) {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
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
    if !fov_map.is_changed() {
        return;
    }

    // Living entities: only visible in current FOV
    for (position, mut visibility) in &mut q_living {
        *visibility = if fov_map.is_visible(*position) {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }

    // Static entities: visible in revealed areas with transparency
    for (position, mut visibility, mut sprite) in &mut q_static {
        if fov_map.is_visible(*position) {
            *visibility = Visibility::Visible;
            sprite.color.set_alpha(1.0);
        } else if fov_map.is_revealed(*position) {
            *visibility = Visibility::Visible;
            sprite.color.set_alpha(0.6); // Less transparent than general sprites
        } else {
            *visibility = Visibility::Hidden;
        }
    }
}

// ============================================================================
// TILEMAP SYSTEMS
// ============================================================================

/// System that updates tilemap visibility based on FOV
pub fn update_tilemap_visibility(
    fov_map: Res<FovMap>,
    mut q_tiles: Query<(&mut TileColor, &TilePos)>,
) {
    if !fov_map.is_changed() {
        return;
    }

    for (mut tile_color, tile_pos) in &mut q_tiles {
        let position = Position::from(*tile_pos);

        if fov_map.is_visible(position) {
            tile_color.0 = Color::WHITE;
        } else if fov_map.is_revealed(position) {
            tile_color.0 = Color::srgba(0.6, 0.6, 0.6, 1.0);
        } else {
            tile_color.0 = Color::BLACK;
        }
    }
}

/// System that provides visual feedback for FOV debugging
///
/// Adds colored borders to tiles based on their FOV state:
/// - Green border: Currently visible
/// - Blue border: Revealed but not visible
/// - No border: Unexplored
///
/// Enable this system only during development/debugging.
#[allow(dead_code)]
pub fn debug_fov_visualization(fov_map: Res<FovMap>, mut q_tiles: Query<(&mut TileColor, &TilePos)>) {
    if !fov_map.is_changed() {
        return;
    }

    for (mut tile_color, tile_pos) in &mut q_tiles {
        let position = Position::from(*tile_pos);

        if fov_map.is_visible(position) {
            let (r, g, b, a) = RenderingConstants::DEBUG_VISIBLE_COLOR;
            tile_color.0 = Color::srgba(r, g, b, a);
        } else if fov_map.is_revealed(position) {
            let (r, g, b, a) = RenderingConstants::DEBUG_REVEALED_COLOR;
            tile_color.0 = Color::srgba(r, g, b, a);
        } else {
            let (r, g, b, a) = RenderingConstants::DEBUG_UNEXPLORED_COLOR;
            tile_color.0 = Color::srgba(r, g, b, a);
        }
    }
}

// ============================================================================
// CAMERA SYSTEMS
// ============================================================================

/// Camera system that follows the player with smooth interpolation and handles zoom controls.
pub fn camera_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    player_query: Query<&Position, With<PlayerTag>>,
    mut camera_query: Query<(&mut Transform, &mut Projection), With<Camera>>,
) {
    for (mut camera_transform, mut projection) in camera_query.iter_mut() {
        // Handle zoom controls (keep existing zoom functionality)
        let Projection::Orthographic(ortho) = &mut *projection else {
            continue;
        };

        if keyboard_input.pressed(KeyCode::KeyZ) {
            ortho.scale += 0.1;
        }

        if keyboard_input.pressed(KeyCode::KeyX) {
            ortho.scale -= 0.1;
        }

        if ortho.scale < 0.5 {
            ortho.scale = 0.5;
        }

        // Follow the player if they exist
        if let Ok(player_position) = player_query.single() {
            // Convert tile position to world coordinates (same calculation as
            // position_to_transform)
            let target_x = player_position.x() as f32 * RenderingConstants::TILE_SIZE
                - (crate::model::ModelConstants::MAP_WIDTH as f32 * RenderingConstants::HALF_TILE_SIZE);
            let target_y = player_position.y() as f32 * RenderingConstants::TILE_SIZE
                - (crate::model::ModelConstants::MAP_HEIGHT as f32 * RenderingConstants::HALF_TILE_SIZE);

            let target_position = Vec3::new(target_x, target_y, camera_transform.translation.z);

            // Snap camera directly to the player position
            camera_transform.translation = target_position;
        }
    }
}
