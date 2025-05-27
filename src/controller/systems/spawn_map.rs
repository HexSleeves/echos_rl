use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::{
    model::{ModelConstants, components::Position, generation::GenConfig, resources::CurrentMap},
    view::resources::TextureAssets,
};

pub fn spawn_map(
    mut commands: Commands,
    mut current_map: ResMut<CurrentMap>,
    texture_assets: Res<TextureAssets>,
) {
    let mut rng = fastrand::Rng::new();
    let mut generator = GenConfig::new(1, ModelConstants::MAP_WIDTH, ModelConstants::MAP_HEIGHT);

    // Generate terrain types
    let terrain_grid = generator.generate(&mut rng);

    // Create a tilemap entity
    let tilemap_entity = commands.spawn_empty().id();

    // Generate tile storage and update our map's tiles with the terrain and tile entities
    let tile_storage =
        generator.generate_tile_storage(&mut commands, TilemapId(tilemap_entity), &terrain_grid);

    // Update our map with the generated terrain and tile entities
    for x in 0..current_map.size.0 {
        for y in 0..current_map.size.1 {
            let position = Position::new(x as i32, y as i32);
            let terrain = terrain_grid.get(position.into()).copied().unwrap_or_default();

            // Set terrain in our unified tile structure
            current_map.set_terrain(position, terrain);

            // Link the tile entity if it exists
            if let Some(tile_entity) = tile_storage.get(&TilePos::new(x as u32, y as u32)) {
                current_map.set_tile_entity(position, tile_entity);
            }
        }
    }

    let tile_size = TilemapTileSize { x: 12.0, y: 12.0 };
    let grid_size = tile_size.into();
    let map_type = TilemapType::default();
    let texture_handle: Handle<Image> = texture_assets.urizen_tileset.clone();

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        tile_size,
        storage: tile_storage.clone(),
        anchor: TilemapAnchor::Center,
        texture: TilemapTexture::Single(texture_handle),
        size: TilemapSize::new(current_map.size.0 as u32, current_map.size.1 as u32),
        ..Default::default()
    });

    // Update the tile_storage in our map (for rendering compatibility)
    current_map.tile_storage = tile_storage;
}
