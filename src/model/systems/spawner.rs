use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::{
    model::{
        ModelConstants,
        components::{AITag, AwaitingInput, PlayerTag, Position, TerrainType, TurnActor, ViewShed},
        generation::GenConfig,
        resources::{CurrentMap, SpawnPoint, TurnQueue},
    },
    view::{ViewConstants, components::TileSprite, resources::TextureAssets},
};

pub fn spawn_player(
    mut commands: Commands,
    mut current_map: ResMut<CurrentMap>,
    mut turn_system: ResMut<TurnQueue>,
    spawn_point: Option<Res<SpawnPoint>>,
) {
    // Determine where to spawn the player
    let player_position = if let Some(spawn_point) = spawn_point {
        if let Some(pos) = spawn_point.player_spawn {
            pos
        } else {
            find_valid_position(&current_map)
        }
    } else {
        find_valid_position(&current_map)
    };

    // Spawn the player
    let player_id = commands
        .spawn((
            player_position,
            PlayerTag,
            AwaitingInput,
            TurnActor::new(100),
            ViewShed { radius: 8 },
            TileSprite {
                tile_coords: (10, 18),
                tile_size: Vec2::splat(ViewConstants::TILE_SIZE),
                ..Default::default()
            },
        ))
        .id();

    // Spawn an enemy at a random location (whale)
    let actor_position = find_valid_position(&current_map);
    let actor_id = commands
        .spawn((
            actor_position,
            AITag,
            TurnActor::new(120),
            TileSprite {
                tile_coords: (0, 16),
                tile_size: Vec2::splat(ViewConstants::TILE_SIZE),
                ..Default::default()
            },
        ))
        .id();

    // Set the player and actor on the map
    let _ = current_map.place_actor(player_position, player_id);
    let _ = current_map.place_actor(actor_position, actor_id);

    // Schedule the player and actor to take turns
    let current_time = turn_system.current_time();
    turn_system.schedule_turn(player_id, current_time);
    turn_system.schedule_turn(actor_id, current_time);
}

// Helper function to find a valid floor position
fn find_valid_position(current_map: &CurrentMap) -> Position {
    let mut rng = fastrand::Rng::new();
    let mut valid_positions = Vec::new();

    for y in 1..ModelConstants::MAP_HEIGHT - 1 {
        for x in 1..ModelConstants::MAP_WIDTH - 1 {
            if let Some(terrain_type) = current_map.get_terrain(Position::new(x as i32, y as i32)) {
                if terrain_type == TerrainType::Floor {
                    valid_positions.push(Position::new(x as i32, y as i32));
                }
            }
        }
    }

    if valid_positions.is_empty() {
        // If no valid positions found, return a default position
        Position::new(ModelConstants::MAP_WIDTH as i32 / 2, ModelConstants::MAP_HEIGHT as i32 / 2)
    } else {
        valid_positions[rng.usize(0..valid_positions.len())]
    }
}

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
