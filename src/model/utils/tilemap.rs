use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::model::resources::Map;

/// Fill a tilemap with tiles based on a level
pub fn fill_tilemap(
    map_size: TilemapSize,
    tilemap_id: TilemapId,
    commands: &mut Commands,
    tile_storage: &mut TileStorage,
    gamemap: &Map,
) {
    for y in 0..map_size.y {
        for x in 0..map_size.x {
            let tile_pos = TilePos { x, y };
            let tile_type = gamemap.get_terrain(tile_pos.into()).unwrap();
            let tile_entity = commands
                .spawn((
                    TileBundle {
                        position: tile_pos,
                        tilemap_id,
                        texture_index: TileTextureIndex(tile_type.texture_index()),
                        ..Default::default()
                    },
                    tile_type,
                ))
                .id();
            tile_storage.set(&tile_pos, tile_entity);
        }
    }
}

/// Fill a tilemap with a default tile type
pub fn fill_tilemap_default(
    texture_index: TileTextureIndex,
    map_size: TilemapSize,
    tilemap_id: TilemapId,
    commands: &mut Commands,
    tile_storage: &mut TileStorage,
) {
    for y in 0..map_size.y {
        for x in 0..map_size.x {
            let tile_pos = TilePos { x, y };
            let tile_entity =
                commands.spawn(TileBundle { position: tile_pos, tilemap_id, texture_index, ..Default::default() }).id();
            tile_storage.set(&tile_pos, tile_entity);
        }
    }
}
