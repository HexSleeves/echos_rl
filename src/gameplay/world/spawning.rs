use bevy::prelude::*;
use echos_assets::entities::{EntityDefinition, EntityDefinitions};

use crate::{
    core::{
        components::{AITag, Description, PlayerTag, Position, ViewShed},
        resources::{CurrentMap, TurnQueue},
    },
    gameplay::{
        enemies::components::{AIBehavior, AIState},
        player::components::AwaitingInput,
        turns::components::TurnActor,
    },
    rendering::components::TileSprite,
};

/// Spawn a player entity from definition data
pub fn spawn_player_from_definition(
    mut commands: Commands,
    entity_definitions: &EntityDefinitions,
    assets: &Assets<EntityDefinition>,
    position: Position,
    current_map: &mut CurrentMap,
    turn_queue: &mut TurnQueue,
) -> Result<Entity, String> {
    let player_handle = entity_definitions.get_player();
    let definition = assets
        .get(player_handle)
        .ok_or("Player definition not loaded")?;

    let mut entity_commands = commands.spawn((
        position,
        PlayerTag,
        AwaitingInput,
        Description::new(&definition.name),
    ));

    // Add components based on definition
    if let Some(turn_data) = &definition.components.turn_actor {
        entity_commands.insert(TurnActor::new(turn_data.speed));
    } else {
        entity_commands.insert(TurnActor::new(100)); // Default speed
    }

    if let Some(view_data) = &definition.components.view_shed {
        entity_commands.insert(ViewShed::new(view_data.radius as i32));
    } else {
        entity_commands.insert(ViewShed::new(8)); // Default radius
    }

    if let Some(sprite_data) = &definition.components.tile_sprite {
        entity_commands.insert(TileSprite {
            tile_coords: sprite_data.tile_coords,
            tile_size: sprite_data.tile_size.unwrap_or((12.0, 12.0)).into(),
            ..Default::default()
        });
    }

    let player_id = entity_commands.id();

    // Add to turn queue (schedule first turn immediately)
    turn_queue.schedule_turn(player_id, 0);

    // Update map with player position
    current_map.place_actor(position, player_id).map_err(|e| format!("Failed to place player: {}", e))?;

    info!("Spawned player '{}' at {:?}", definition.name, position);
    Ok(player_id)
}

/// Spawn a specific AI entity from definition data
pub fn spawn_ai_from_definition(
    mut commands: Commands,
    entity_definitions: &EntityDefinitions,
    assets: &Assets<EntityDefinition>,
    ai_name: &str,
    position: Position,
    current_map: &mut CurrentMap,
    turn_queue: &mut TurnQueue,
) -> Result<Entity, String> {
    let ai_handle = entity_definitions
        .get_by_name(ai_name)
        .ok_or_else(|| format!("AI definition '{}' not found", ai_name))?;

    let definition = assets
        .get(ai_handle)
        .ok_or_else(|| format!("AI definition '{}' not loaded", ai_name))?;

    spawn_ai_entity(commands, definition, position, current_map, turn_queue)
}

/// Spawn a random AI entity from available definitions
pub fn spawn_random_ai_from_definition(
    mut commands: Commands,
    entity_definitions: &EntityDefinitions,
    assets: &Assets<EntityDefinition>,
    position: Position,
    current_map: &mut CurrentMap,
    turn_queue: &mut TurnQueue,
) -> Result<Entity, String> {
    let random_handle = entity_definitions
        .get_random_enemy()
        .ok_or("No enemy definitions available")?;

    let definition = assets
        .get(random_handle)
        .ok_or("Random enemy definition not loaded")?;

    spawn_ai_entity(commands, definition, position, current_map, turn_queue)
}

/// Helper function to spawn an AI entity from a definition
fn spawn_ai_entity(
    mut commands: Commands,
    definition: &EntityDefinition,
    position: Position,
    current_map: &mut CurrentMap,
    turn_queue: &mut TurnQueue,
) -> Result<Entity, String> {
    let mut entity_commands = commands.spawn((
        position,
        AITag,
        Description::new(&definition.name),
        AIBehavior::default(),
        AIState::default(),
    ));

    // Add components based on definition
    if let Some(turn_data) = &definition.components.turn_actor {
        entity_commands.insert(TurnActor::new(turn_data.speed));
    } else {
        entity_commands.insert(TurnActor::new(100)); // Default speed
    }

    if let Some(view_data) = &definition.components.view_shed {
        entity_commands.insert(ViewShed::new(view_data.radius as i32));
    } else {
        entity_commands.insert(ViewShed::new(6)); // Default AI radius
    }

    if let Some(sprite_data) = &definition.components.tile_sprite {
        entity_commands.insert(TileSprite {
            tile_coords: sprite_data.tile_coords,
            tile_size: sprite_data.tile_size.unwrap_or((12.0, 12.0)).into(),
            ..Default::default()
        });
    }

    let ai_id = entity_commands.id();

    // Add to turn queue (schedule first turn immediately)
    turn_queue.schedule_turn(ai_id, 0);

    // Update map with AI position
    current_map.place_actor(position, ai_id).map_err(|e| format!("Failed to place AI: {}", e))?;

    info!("Spawned AI '{}' at {:?}", definition.name, position);
    Ok(ai_id)
}
