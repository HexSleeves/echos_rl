use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::model::{
    ModelConstants,
    components::TerrainType,
    resources::{CurrentMap, GameMap},
};

pub fn spawn_map(mut commands: Commands, mut current_map: ResMut<CurrentMap>, asset_server: Res<AssetServer>) {
    // let mut rng = fastrand::Rng::new();
    // let mut generator = GenConfig::new(1, ModelConstants::MAP_WIDTH, ModelConstants::MAP_HEIGHT);

    // // Generate terrain types
    // let terrain_grid = generator.generate(&mut rng);

    // // Generate entities and update the map
    // let terrain_entities = generator.generate_entities(&mut commands, &terrain_grid);

    // // Update the current map
    // current_map.terrain = terrain_entities;

    let texture_handle: Handle<Image> = asset_server.load("tiles.png");
    let map_size = TilemapSize { x: ModelConstants::MAP_WIDTH as u32, y: ModelConstants::MAP_HEIGHT as u32 };

    // Create a tilemap entity a little early.
    let tilemap_entity = commands.spawn_empty().id();

    // To begin creating the map we will need a `TileStorage` component.
    let mut tile_storage = TileStorage::empty(map_size);

    // Create a level
    let mut rng = fastrand::Rng::new();
    let mut level = GameMap::new(map_size.x as i32, map_size.y as i32, 0, None);

    // Generate a surface level for now
    for y in 0..level.height {
        for x in 0..level.width {
            if x == 0 || x == level.width - 1 || y == 0 || y == level.height - 1 {
                level.set_tile(x, y, TerrainType::Wall);
            } else {
                level.set_tile(x, y, TerrainType::Floor);
            }
        }
    }

    // Add a staircase down to an underground level
    let stairs_x = rng.i32(5..level.width - 5);
    let stairs_y = rng.i32(5..level.height - 5);
    level.set_tile(stairs_x, stairs_y, TerrainType::StairsDown);

    // Spawn the elements of the tilemap.
    crate::model::utils::fill_tilemap(map_size, TilemapId(tilemap_entity), &mut commands, &mut tile_storage, &level);

    let map_type = TilemapType::default();
    let tile_size = TilemapTileSize { x: 16.0, y: 16.0 };
    let grid_size = tile_size.into();

    commands.entity(tilemap_entity).insert((
        TilemapBundle {
            grid_size,
            map_type,
            tile_size,
            size: map_size,
            storage: tile_storage,
            texture: TilemapTexture::Single(texture_handle),
            anchor: TilemapAnchor::Center,
            ..Default::default()
        },
        level,
    ));
}
