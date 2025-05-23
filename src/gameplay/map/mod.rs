//! Tilemap module for handling the game world representation.
//! This module uses bevy_ecs_tilemap to implement a roguelike world.

use crate::screens::loading::TextureAssets;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use gamemap::GameMap;
use rand::Rng;
use tile::{TileType, UndergroundType};

pub mod gamemap;
pub mod procgen;
pub mod tile;
mod utils;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(bevy_ecs_tilemap::TilemapPlugin);

    // Register components
    app.register_type::<gamemap::GameMap>();
}

/// System to generate a new map.
pub fn generate_map(mut commands: Commands, _texture_assets: Res<TextureAssets>, asset_server: Res<AssetServer>) {
    let texture_handle: Handle<Image> = asset_server.load("tiles.png");

    let map_size = TilemapSize { x: 80, y: 50 };

    // Create a tilemap entity a little early.
    let tilemap_entity = commands.spawn_empty().id();

    // To begin creating the map we will need a `TileStorage` component.
    let mut tile_storage = TileStorage::empty(map_size);

    // Create a level
    let mut rng = rand::rng();
    let mut level = GameMap::new(map_size.x as i32, map_size.y as i32, 0, None);

    // Generate a surface level for now
    for y in 0..level.height {
        for x in 0..level.width {
            if x == 0 || x == level.width - 1 || y == 0 || y == level.height - 1 {
                level.set_tile(x, y, TileType::Wall);
            } else {
                level.set_tile(x, y, TileType::Floor);
            }
        }
    }

    // Add a staircase down to an underground level
    let stairs_x = rng.random_range(5..level.width - 5);
    let stairs_y = rng.random_range(5..level.height - 5);
    level.set_tile(stairs_x, stairs_y, TileType::StairsDown);

    // Spawn the elements of the tilemap.
    utils::fill_tilemap(
        TileTextureIndex(0),
        map_size,
        TilemapId(tilemap_entity),
        &mut commands,
        &mut tile_storage,
        &level,
    );

    let tile_size = TilemapTileSize { x: 16.0, y: 16.0 };
    let grid_size = tile_size.into();
    let map_type = TilemapType::default();

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

    // Generate underground levels
    // generate_underground_levels(&mut commands, &asset_server);
}

/// Generate underground levels
fn generate_underground_levels(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let texture_handle: Handle<Image> = asset_server.load("tiles.png");
    let mut rng = rand::rng();

    // Generate a mine level
    let map_size = TilemapSize { x: 48, y: 48 };
    let tilemap_entity = commands.spawn_empty().id();
    let mut tile_storage = TileStorage::empty(map_size);

    let mut mine_level = GameMap::new(map_size.x as i32, map_size.y as i32, 1, Some(UndergroundType::Mine));

    // Generate the mine
    procgen::generate_mine(&mut mine_level, &mut rng, 0.5, 1);

    // Fill the tilemap
    utils::fill_tilemap(
        TileTextureIndex(0),
        map_size,
        TilemapId(tilemap_entity),
        commands,
        &mut tile_storage,
        &mine_level,
    );

    let tile_size = TilemapTileSize { x: 16.0, y: 16.0 };
    let grid_size = tile_size.into();
    let map_type = TilemapType::default();

    commands.entity(tilemap_entity).insert((
        TilemapBundle {
            grid_size,
            map_type,
            tile_size,
            size: map_size,
            storage: tile_storage,
            texture: TilemapTexture::Single(texture_handle.clone()),
            anchor: TilemapAnchor::Center,
            transform: Transform::from_xyz(0.0, -500.0, 0.0), // Position below the surface map
            ..Default::default()
        },
        mine_level,
    ));

    // Generate a cave level
    let map_size = TilemapSize { x: 64, y: 64 };
    let tilemap_entity = commands.spawn_empty().id();
    let mut tile_storage = TileStorage::empty(map_size);

    let mut cave_level = GameMap::new(map_size.x as i32, map_size.y as i32, 2, Some(UndergroundType::Cave));

    // Generate the cave
    procgen::generate_cave(&mut cave_level, &mut rng, 0.7, 2);

    // Fill the tilemap
    utils::fill_tilemap(
        TileTextureIndex(0),
        map_size,
        TilemapId(tilemap_entity),
        commands,
        &mut tile_storage,
        &cave_level,
    );

    commands.entity(tilemap_entity).insert((
        TilemapBundle {
            grid_size,
            map_type,
            tile_size,
            size: map_size,
            storage: tile_storage,
            texture: TilemapTexture::Single(texture_handle.clone()),
            anchor: TilemapAnchor::Center,
            transform: Transform::from_xyz(0.0, -1000.0, 0.0), // Position below the mine level
            ..Default::default()
        },
        cave_level,
    ));

    // Generate a dark adventure area
    let map_size = TilemapSize { x: 80, y: 80 };
    let tilemap_entity = commands.spawn_empty().id();
    let mut tile_storage = TileStorage::empty(map_size);

    let mut dark_adventure_level =
        GameMap::new(map_size.x as i32, map_size.y as i32, 3, Some(UndergroundType::DarkAdventure));

    // Generate the dark adventure area
    procgen::generate_dark_adventure(&mut dark_adventure_level, &mut rng, 3);

    // Fill the tilemap
    utils::fill_tilemap(
        TileTextureIndex(0),
        map_size,
        TilemapId(tilemap_entity),
        commands,
        &mut tile_storage,
        &dark_adventure_level,
    );

    commands.entity(tilemap_entity).insert((
        TilemapBundle {
            grid_size,
            map_type,
            tile_size,
            size: map_size,
            storage: tile_storage,
            texture: TilemapTexture::Single(texture_handle),
            anchor: TilemapAnchor::Center,
            transform: Transform::from_xyz(0.0, -1500.0, 0.0), // Position below the cave level
            ..Default::default()
        },
        dark_adventure_level,
    ));
}
