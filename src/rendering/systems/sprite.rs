use bevy::prelude::*;

use crate::{
    core::{components::Position, constants::ModelConstants},
    rendering::{RenderingConstants, components::TileSprite, resources::TileMap},
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
/// Only updates transforms when positions have actually changed
pub fn position_to_transform(mut q_objects: Query<(&Position, &mut Transform), Changed<Position>>) {
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
