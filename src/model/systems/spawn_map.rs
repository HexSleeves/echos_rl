use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::{
    model::{ModelConstants, generation::GenConfig, resources::CurrentMap, utils},
    view::resources::TextureAssets,
};

pub fn spawn_map(mut commands: Commands, mut current_map: ResMut<CurrentMap>, texture_assets: Res<TextureAssets>) {
    let mut rng = fastrand::Rng::new();
    let mut generator = GenConfig::new(1, ModelConstants::MAP_WIDTH, ModelConstants::MAP_HEIGHT);

    // Generate terrain types
    let terrain_grid = generator.generate(&mut rng);
    // Create a tilemap entity a little early.
    let tilemap_entity = commands.spawn_empty().id();
    // Generate tile storage
    let tile_storage = generator.generate_tile_storage(&mut commands, TilemapId(tilemap_entity), &terrain_grid);

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

    // // Update the current map
    current_map.terrain = terrain_grid;
    current_map.tile_storage = tile_storage;
}
