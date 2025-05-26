use bevy::prelude::*;

use crate::{
    model::{
        ModelConstants,
        components::{AITag, PlayerTag, Position, TerrainType, TurnActor, ViewShed},
        resources::{CurrentMap, SpawnPoint, TurnQueue},
    },
    view::{ViewConstants, components::TileSprite},
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
