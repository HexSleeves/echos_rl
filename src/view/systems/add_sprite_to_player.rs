use bevy::prelude::*;

use crate::view::{components::TileSprite, resources::TileMap};

/// System that adds sprite components to the player entity
pub fn add_sprite_to_player(
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
