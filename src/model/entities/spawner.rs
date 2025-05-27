use bevy::prelude::*;

use crate::{
    model::{
        components::{AITag, AwaitingInput, PlayerTag, Position, TurnActor, ViewShed},
        entities::{EntityDefinition, EntityDefinitions},
        resources::{CurrentMap, TurnQueue},
    },
    view::{ViewConstants, components::TileSprite},
};

/// Spawn an entity from a definition with optional position override
pub fn spawn_entity_from_definition(
    commands: &mut Commands,
    definition: &EntityDefinition,
    position: Position,
    current_map: &mut CurrentMap,
    turn_queue: &mut TurnQueue,
) -> Result<Entity, SpawnError> {
    let mut entity_commands = commands.spawn(position);

    // Add components based on entity type
    if definition.is_player() {
        entity_commands.insert((PlayerTag, AwaitingInput));
    } else if definition.is_ai() {
        entity_commands.insert(AITag);
    }

    // Add TurnActor component if specified
    if let Some(turn_actor_data) = &definition.components.turn_actor {
        let turn_actor: TurnActor = turn_actor_data.into();
        entity_commands.insert(turn_actor);
    }

    // Add ViewShed component if specified
    if let Some(view_shed_data) = &definition.components.view_shed {
        let view_shed: ViewShed = view_shed_data.into();
        entity_commands.insert(view_shed);
    }

    // Add TileSprite component if specified
    if let Some(tile_sprite_data) = &definition.components.tile_sprite {
        let tile_sprite: TileSprite = tile_sprite_data.into();
        entity_commands.insert(tile_sprite);
    }

    let entity_id = entity_commands.id();

    // Place the entity on the map
    current_map.place_actor(position, entity_id).map_err(|_| SpawnError::PositionOccupied(position))?;

    // Schedule the entity's turn if it has a TurnActor component
    if definition.components.turn_actor.is_some() {
        let current_time = turn_queue.current_time();
        turn_queue.schedule_turn(entity_id, current_time);
    }

    Ok(entity_id)
}

/// Spawn the player from entity definitions
pub fn spawn_player_from_definition(
    commands: &mut Commands,
    entity_definitions: &EntityDefinitions,
    assets: &Assets<EntityDefinition>,
    position: Position,
    current_map: &mut CurrentMap,
    turn_queue: &mut TurnQueue,
) -> Result<Entity, SpawnError> {
    let player_handle =
        entity_definitions.get_player().ok_or(SpawnError::DefinitionNotFound("player".to_string()))?;

    let player_definition =
        assets.get(player_handle).ok_or(SpawnError::AssetNotLoaded("player".to_string()))?;

    spawn_entity_from_definition(commands, player_definition, position, current_map, turn_queue)
}

/// Spawn an enemy from entity definitions by name
pub fn spawn_enemy_from_definition(
    commands: &mut Commands,
    entity_definitions: &EntityDefinitions,
    assets: &Assets<EntityDefinition>,
    enemy_name: &str,
    position: Position,
    current_map: &mut CurrentMap,
    turn_queue: &mut TurnQueue,
) -> Result<Entity, SpawnError> {
    let enemy_handle = entity_definitions
        .get(enemy_name)
        .ok_or_else(|| SpawnError::DefinitionNotFound(enemy_name.to_string()))?;

    let enemy_definition =
        assets.get(enemy_handle).ok_or_else(|| SpawnError::AssetNotLoaded(enemy_name.to_string()))?;

    spawn_entity_from_definition(commands, enemy_definition, position, current_map, turn_queue)
}

/// Spawn a random enemy from available definitions
pub fn spawn_random_enemy_from_definition(
    commands: &mut Commands,
    entity_definitions: &EntityDefinitions,
    assets: &Assets<EntityDefinition>,
    position: Position,
    current_map: &mut CurrentMap,
    turn_queue: &mut TurnQueue,
) -> Result<Entity, SpawnError> {
    let enemy_handle = entity_definitions.get_random_enemy().ok_or(SpawnError::NoEnemiesAvailable)?;

    let enemy_definition =
        assets.get(enemy_handle).ok_or(SpawnError::AssetNotLoaded("random_enemy".to_string()))?;

    spawn_entity_from_definition(commands, enemy_definition, position, current_map, turn_queue)
}

/// Fallback spawning functions that match the original hardcoded behavior exactly
pub mod fallback {
    use super::*;

    /// Fallback player spawning with exact hardcoded values
    pub fn spawn_player_hardcoded(
        commands: &mut Commands,
        position: Position,
        current_map: &mut CurrentMap,
        turn_queue: &mut TurnQueue,
    ) -> Entity {
        let player_id = commands
            .spawn((
                position,
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

        // Place on map and schedule turn
        let _ = current_map.place_actor(position, player_id);
        let current_time = turn_queue.current_time();
        turn_queue.schedule_turn(player_id, current_time);

        player_id
    }

    /// Fallback enemy spawning with exact hardcoded values (whale)
    pub fn spawn_enemy_hardcoded(
        commands: &mut Commands,
        position: Position,
        current_map: &mut CurrentMap,
        turn_queue: &mut TurnQueue,
    ) -> Entity {
        let enemy_id = commands
            .spawn((
                position,
                AITag,
                TurnActor::new(120),
                TileSprite {
                    tile_coords: (0, 16),
                    tile_size: Vec2::splat(ViewConstants::TILE_SIZE),
                    ..Default::default()
                },
            ))
            .id();

        // Place on map and schedule turn
        let _ = current_map.place_actor(position, enemy_id);
        let current_time = turn_queue.current_time();
        turn_queue.schedule_turn(enemy_id, current_time);

        enemy_id
    }
}

/// Errors that can occur during entity spawning
#[derive(Debug, thiserror::Error)]
pub enum SpawnError {
    #[error("Entity definition '{0}' not found")]
    DefinitionNotFound(String),

    #[error("Asset '{0}' not loaded")]
    AssetNotLoaded(String),

    #[error("Position {0:?} is already occupied")]
    PositionOccupied(Position),

    #[error("No enemy definitions available")]
    NoEnemiesAvailable,

    #[error("Failed to place entity on map")]
    MapPlacementFailed,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::entities::{EntityComponents, TileSpriteData, TurnActorData, ViewShedData};

    fn create_test_player_definition() -> EntityDefinition {
        EntityDefinition {
            name: "Test Player".to_string(),
            description: Some("Test player for unit tests".to_string()),
            components: EntityComponents {
                turn_actor: Some(TurnActorData::new(100)),
                view_shed: Some(ViewShedData::new(8)),
                tile_sprite: Some(TileSpriteData::new((10, 18))),
                is_player: Some(true),
                is_ai: Some(false),
                ..Default::default()
            },
        }
    }

    fn create_test_enemy_definition() -> EntityDefinition {
        EntityDefinition {
            name: "Test Enemy".to_string(),
            description: Some("Test enemy for unit tests".to_string()),
            components: EntityComponents {
                turn_actor: Some(TurnActorData::new(120)),
                view_shed: None,
                tile_sprite: Some(TileSpriteData::new((0, 16))),
                is_player: Some(false),
                is_ai: Some(true),
                ..Default::default()
            },
        }
    }

    #[test]
    fn test_spawn_error_display() {
        let error = SpawnError::DefinitionNotFound("test".to_string());
        assert_eq!(error.to_string(), "Entity definition 'test' not found");

        let error = SpawnError::AssetNotLoaded("test".to_string());
        assert_eq!(error.to_string(), "Asset 'test' not loaded");

        let error = SpawnError::PositionOccupied(Position::new(5, 5));
        assert!(error.to_string().contains("Position"));
        assert!(error.to_string().contains("occupied"));
    }

    #[test]
    fn test_entity_definition_validation() {
        let player_def = create_test_player_definition();
        assert!(player_def.is_player());
        assert!(!player_def.is_ai());
        assert_eq!(player_def.components.turn_actor.as_ref().unwrap().speed, 100);

        let enemy_def = create_test_enemy_definition();
        assert!(!enemy_def.is_player());
        assert!(enemy_def.is_ai());
        assert_eq!(enemy_def.components.turn_actor.as_ref().unwrap().speed, 120);
    }

    #[test]
    fn test_component_conversion_compatibility() {
        // Test that our data structures convert to the same components as hardcoded
        let player_def = create_test_player_definition();

        // TurnActor conversion
        let turn_actor_data = player_def.components.turn_actor.as_ref().unwrap();
        let turn_actor: TurnActor = turn_actor_data.into();
        assert_eq!(turn_actor.speed, 100);
        assert!(turn_actor.is_alive());

        // ViewShed conversion
        let view_shed_data = player_def.components.view_shed.as_ref().unwrap();
        let view_shed: ViewShed = view_shed_data.into();
        assert_eq!(view_shed.radius, 8);

        // TileSprite conversion
        let tile_sprite_data = player_def.components.tile_sprite.as_ref().unwrap();
        let tile_sprite: TileSprite = tile_sprite_data.into();
        assert_eq!(tile_sprite.tile_coords, (10, 18));
        assert_eq!(tile_sprite.tile_size, Vec2::splat(ViewConstants::TILE_SIZE));
    }

    #[test]
    fn test_ron_file_integration() {
        // Test that our actual RON files work with the spawning system

        // Test player.ron
        let player_ron = include_str!("../../../assets/entities/player.ron");
        let player_def: EntityDefinition = ron::from_str(player_ron).expect("Failed to parse player.ron");

        assert!(player_def.is_player());
        assert!(!player_def.is_ai());
        assert_eq!(player_def.components.turn_actor.as_ref().unwrap().speed, 100);
        assert_eq!(player_def.components.view_shed.as_ref().unwrap().radius, 8);
        assert_eq!(player_def.components.tile_sprite.as_ref().unwrap().tile_coords, (10, 18));

        // Test whale.ron
        let whale_ron = include_str!("../../../assets/entities/enemies/whale.ron");
        let whale_def: EntityDefinition = ron::from_str(whale_ron).expect("Failed to parse whale.ron");

        assert!(!whale_def.is_player());
        assert!(whale_def.is_ai());
        assert_eq!(whale_def.components.turn_actor.as_ref().unwrap().speed, 120);
        assert_eq!(whale_def.components.tile_sprite.as_ref().unwrap().tile_coords, (0, 16));
        assert!(whale_def.components.view_shed.is_none());

        // Test basic_enemy.ron
        let basic_enemy_ron = include_str!("../../../assets/entities/enemies/basic_enemy.ron");
        let basic_enemy_def: EntityDefinition =
            ron::from_str(basic_enemy_ron).expect("Failed to parse basic_enemy.ron");

        assert!(!basic_enemy_def.is_player());
        assert!(basic_enemy_def.is_ai());
        assert_eq!(basic_enemy_def.components.turn_actor.as_ref().unwrap().speed, 100);
        assert_eq!(basic_enemy_def.components.tile_sprite.as_ref().unwrap().tile_coords, (1, 16));
    }
}
