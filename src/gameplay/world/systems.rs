use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use brtk::{grid::Grid, random::Random};

use crate::{
    core::{
        commands::SpawnEntityCommands,
        components::Position,
        constants::ModelConstants,
        resources::{CurrentMap, SpawnPoint},
    },
    gameplay::world::{components::TerrainType, generation::GenConfig},
    rendering::resources::TextureAssets,
};

// ============================================================================
// ENTITY SPAWNING SYSTEMS
// ============================================================================

pub fn spawn_player(
    mut commands: Commands,
    current_map: Res<CurrentMap>,
    spawn_point: Option<Res<SpawnPoint>>,
) {
    // Determine where to spawn the player
    let player_position = spawn_point
        .and_then(|sp| sp.player_spawn)
        .or_else(|| current_map.get_random_walkable_position())
        .unwrap_or_else(|| {
            warn!("No valid spawn point found, using default position");
            Position::new(0, 0)
        });

    // Use the command-based spawning
    commands.spawn_player(player_position);

    info!("Queued player spawn at {:?}", player_position);
}

pub fn spawn_enemies(mut commands: Commands, current_map: Res<CurrentMap>) {
    // Find a valid position for enemy spawning
    let enemy_position = current_map.get_random_walkable_position().unwrap_or_else(|| {
        warn!("No valid enemy spawn point found, using default position");
        Position::new(1, 1)
    });

    // Use the command-based spawning for a random enemy
    commands.spawn_random_enemy(enemy_position);

    info!("Queued enemy spawn at {:?}", enemy_position);
}

// ============================================================================
// MAP SPAWNING SYSTEMS
// ============================================================================

/// Generate terrain and tile entities using the provided RNG
fn generate_terrain_and_tiles(
    commands: &mut Commands,
    rng: &mut fastrand::Rng,
    tilemap_entity: Entity,
) -> (Grid<TerrainType>, TileStorage) {
    let mut generator = GenConfig::new(1, ModelConstants::MAP_WIDTH, ModelConstants::MAP_HEIGHT);

    // Generate terrain types
    let terrain_grid = generator.generate(rng);

    // Generate tile storage with the terrain and tile entities
    let tile_storage =
        generator.generate_tile_storage(commands.reborrow(), TilemapId(tilemap_entity), &terrain_grid);

    (terrain_grid, tile_storage)
}

/// Update the current map with generated terrain and tile entities
fn update_current_map_with_terrain(
    current_map: &mut CurrentMap,
    terrain_grid: &Grid<TerrainType>,
    tile_storage: &TileStorage,
) {
    for x in 0..current_map.size.0 {
        for y in 0..current_map.size.1 {
            let position = Position::new(x as i32, y as i32);
            let terrain = terrain_grid.get(position.into()).copied().unwrap_or_default();

            // Set terrain in our unified tile structure
            current_map.set_terrain(position, terrain);

            // Link the tile entity if it exists
            if let Some(tile_entity) = tile_storage.get(&TilePos::new(x, y)) {
                current_map.set_tile_entity(position, tile_entity);
            }
        }
    }
}

/// Create and insert the TilemapBundle with rendering components
fn setup_tilemap_rendering(
    commands: &mut Commands,
    tilemap_entity: Entity,
    tile_storage: TileStorage,
    current_map: &CurrentMap,
    texture_assets: &TextureAssets,
) {
    let tile_size = TilemapTileSize { x: ModelConstants::TILE_SIZE, y: ModelConstants::TILE_SIZE };
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
        size: TilemapSize::new(current_map.size.0, current_map.size.1),
        ..Default::default()
    });
}

/// Main map spawning system - refactored to use helper functions and shared RNG
pub fn spawn_map(
    mut commands: Commands,
    mut current_map: ResMut<CurrentMap>,
    texture_assets: Res<TextureAssets>,
    mut shared_rng: ResMut<Random>,
) {
    // Use the shared RNG resource for better randomness distribution
    let mut rng = shared_rng.to_fastrand();

    // Create a tilemap entity
    let tilemap_entity = commands.spawn_empty().id();

    // Generate terrain and tile entities
    let (terrain_grid, tile_storage) = generate_terrain_and_tiles(&mut commands, &mut rng, tilemap_entity);

    // Update our map with the generated terrain and tile entities
    update_current_map_with_terrain(&mut current_map, &terrain_grid, &tile_storage);

    // Set up rendering components and insert the TilemapBundle
    setup_tilemap_rendering(
        &mut commands,
        tilemap_entity,
        tile_storage.clone(),
        &current_map,
        &texture_assets,
    );

    // Update the tile_storage in our map (for rendering compatibility)
    current_map.tile_storage = tile_storage;
}
