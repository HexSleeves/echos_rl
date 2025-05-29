use bevy::prelude::*;
use big_brain::prelude::*;

use crate::{
    model::{
        components::{
            AIBehavior, AIBehaviorType, AIState, AITag, AwaitingInput, ChasePlayerAction, ChasePlayerScorer,
            FleeFromPlayerAction, FleeFromPlayerScorer, IdleAction, PlayerTag, Position, TurnActor, ViewShed,
            WanderAction, WanderScorer,
        },
        resources::{CurrentMap, TurnQueue},
    },
    view::components::TileSprite,
};

use super::{EntityDefinition, EntityDefinitions};

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

/// Spawn the player from entity definitions
pub fn spawn_player_from_definition(
    commands: Commands,
    entity_definitions: &EntityDefinitions,
    assets: &Assets<EntityDefinition>,
    position: Position,
    current_map: &mut CurrentMap,
    turn_queue: &mut TurnQueue,
) -> Result<Entity, SpawnError> {
    let player_handle = entity_definitions.get_player();

    let player_definition =
        assets.get(player_handle).ok_or(SpawnError::AssetNotLoaded("player".to_string()))?;

    spawn_entity_from_definition(commands, player_definition, current_map, turn_queue, Some(position))
}

/// Spawn an enemy from entity definitions by name
pub fn spawn_ai_from_definition(
    commands: Commands,
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

    spawn_entity_from_definition(commands, enemy_definition, current_map, turn_queue, Some(position))
}

/// Spawn a random enemy from available definitions
pub fn spawn_random_ai_from_definition(
    commands: Commands,
    entity_definitions: &EntityDefinitions,
    assets: &Assets<EntityDefinition>,
    position: Position,
    current_map: &mut CurrentMap,
    turn_queue: &mut TurnQueue,
) -> Result<Entity, SpawnError> {
    let enemy_handle = entity_definitions.get_random_enemy().ok_or(SpawnError::NoEnemiesAvailable)?;

    let enemy_definition =
        assets.get(enemy_handle).ok_or(SpawnError::AssetNotLoaded("random_enemy".to_string()))?;

    spawn_entity_from_definition(commands, enemy_definition, current_map, turn_queue, Some(position))
}

/// System to spawn enemies with AI behaviors
    #[rustfmt::skip]
pub fn spawn_ai_enemies(
    mut commands: Commands,
    mut turn_queue: ResMut<TurnQueue>,
    mut current_map: ResMut<CurrentMap>,
    assets: Option<Res<Assets<EntityDefinition>>>,
    entity_definitions: Option<Res<EntityDefinitions>>,
) {
    let Some(entity_definitions) = entity_definitions else {
        warn!("EntityDefinitions resource not available for AI spawning");
        return;
    };

    let Some(assets) = assets else {
        warn!("EntityDefinition assets not available for AI spawning");
        return;
    };

    // Spawn different types of AI enemies using simple names
    spawn_enemy_type(commands.reborrow(), &assets, &entity_definitions, &mut current_map, &mut turn_queue, "hostile_guard", 2);
    spawn_enemy_type(commands.reborrow(), &assets, &entity_definitions, &mut current_map, &mut turn_queue, "passive_critter", 3);
    spawn_enemy_type(commands, &assets, &entity_definitions, &mut current_map, &mut turn_queue, "neutral_wanderer", 2);
}

fn spawn_enemy_type(
    mut commands: Commands,
    assets: &Assets<EntityDefinition>,
    entity_definitions: &EntityDefinitions,
    current_map: &mut CurrentMap,
    turn_queue: &mut TurnQueue,
    enemy_type: &str,
    count: usize,
) {
    let Some(definition_handle) = entity_definitions.get_by_name(enemy_type) else {
        warn!("Enemy definition '{}' not found", enemy_type);
        return;
    };

    let Some(definition) = assets.get(definition_handle) else {
        warn!("Enemy definition asset for '{}' not loaded", enemy_type);
        return;
    };

    for _ in 0..count {
        if let Some(spawn_pos) = current_map.get_random_walkable_position() {
            // spawn_ai_entity(commands, definition, spawn_pos);
            match spawn_entity_from_definition(
                commands.reborrow(),
                definition,
                current_map,
                turn_queue,
                Some(spawn_pos),
            ) {
                Ok(entity) => {
                    info!("[Enemy] Entity {} Spawned enemy '{}' at {:?}", entity, enemy_type, spawn_pos);
                }
                Err(e) => {
                    warn!("[Enemy] Failed to spawn enemy '{}': {}", enemy_type, e);
                }
            }
        } else {
            warn!("Could not find spawn position for {}", enemy_type);
        }
    }
}

/// Spawn an entity from a definition with optional position override
pub fn spawn_entity_from_definition(
    mut commands: Commands,
    definition: &EntityDefinition,
    current_map: &mut CurrentMap,
    turn_queue: &mut TurnQueue,
    position: Option<Position>,
) -> Result<Entity, SpawnError> {
    let position = match position {
        Some(pos) => pos,
        None => current_map.get_random_walkable_position().ok_or(SpawnError::MapPlacementFailed)?,
    };

    let mut entity_commands = commands.spawn(position);

    // Add components based on entity type
    if definition.is_player() {
        entity_commands.insert((PlayerTag, AwaitingInput));
    } else if definition.is_ai() {
        add_ai_components(&mut entity_commands, definition);
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

fn add_ai_components(entity_commands: &mut EntityCommands, definition: &EntityDefinition) {
    // Determine AI behavior type based on the entity name
    let ai_behavior = determine_ai_behavior_from_definition(definition);
    let behavior_type = ai_behavior.behavior_type.clone();

    // Create the appropriate thinker based on behavior type
    let thinker = create_thinker_for_behavior(&ai_behavior.behavior_type);

    entity_commands.insert((AITag, ai_behavior, AIState::default(), thinker));

    info!("Added AI components to entity '{}' with behavior {:?}", definition.name, behavior_type);
}

/// Determine AI behavior type from entity definition
fn determine_ai_behavior_from_definition(definition: &EntityDefinition) -> AIBehavior {
    let behavior_type = match definition.name.to_lowercase().as_str() {
        name if name.contains("hostile") || name.contains("guard") => AIBehaviorType::Hostile,
        name if name.contains("passive") || name.contains("critter") => AIBehaviorType::Passive,
        name if name.contains("neutral") || name.contains("wanderer") => AIBehaviorType::Neutral,
        _ => AIBehaviorType::Neutral, // Default to neutral
    };

    // Determine detection range based on view shed or use default
    let detection_range = definition.components.view_shed.as_ref().map(|vs| vs.radius).unwrap_or(5); // Default detection range

    AIBehavior { behavior_type, detection_range, last_known_player_position: None }
}

/// Create a big-brain Thinker based on the AI behavior type
fn create_thinker_for_behavior(behavior_type: &AIBehaviorType) -> ThinkerBuilder {
    match behavior_type {
        AIBehaviorType::Hostile => {
            // Hostile enemies prioritize chasing the player when they can see them
            Thinker::build()
                .picker(FirstToScore { threshold: 0.6 })
                .when(ChasePlayerScorer, ChasePlayerAction)
                .when(WanderScorer, WanderAction)
                .otherwise(IdleAction)
        }
        AIBehaviorType::Passive => {
            // Passive enemies prioritize fleeing from the player
            Thinker::build()
                .picker(FirstToScore { threshold: 0.5 })
                .when(FleeFromPlayerScorer, FleeFromPlayerAction)
                .when(WanderScorer, WanderAction)
                .otherwise(IdleAction)
        }
        AIBehaviorType::Neutral => {
            // Neutral enemies just wander around, ignoring the player
            Thinker::build()
                .picker(FirstToScore { threshold: 0.3 })
                .when(WanderScorer, WanderAction)
                .otherwise(IdleAction)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        assets::entities::{EntityComponents, TileSpriteData, TurnActorData, ViewShedData},
        view::ViewConstants,
    };

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
        let player_ron = include_str!("../../../assets/entities/player.definition.ron");
        let player_def: EntityDefinition = ron::from_str(player_ron).expect("Failed to parse player.ron");

        assert!(player_def.is_player());
        assert!(!player_def.is_ai());
        assert_eq!(player_def.components.turn_actor.as_ref().unwrap().speed, 100);
        assert_eq!(player_def.components.view_shed.as_ref().unwrap().radius, 8);
        assert_eq!(player_def.components.tile_sprite.as_ref().unwrap().tile_coords, (10, 18));

        // Test enemies/hostile_guard.ron
        let hostile_guard_ron = include_str!("../../../assets/entities/enemies/hostile_guard.definition.ron");
        let hostile_guard_def: EntityDefinition =
            ron::from_str(hostile_guard_ron).expect("Failed to parse hostile_guard.ron");

        assert!(!hostile_guard_def.is_player());
        assert!(hostile_guard_def.is_ai());
        assert_eq!(hostile_guard_def.components.turn_actor.as_ref().unwrap().speed, 110);
        assert_eq!(hostile_guard_def.components.tile_sprite.as_ref().unwrap().tile_coords, (1, 16));
        assert_eq!(hostile_guard_def.components.view_shed.as_ref().unwrap().radius, 6);

        // Test enemies/passive_critter.ron
        let passive_critter_ron =
            include_str!("../../../assets/entities/enemies/passive_critter.definition.ron");
        let passive_critter_def: EntityDefinition =
            ron::from_str(passive_critter_ron).expect("Failed to parse passive_critter.ron");

        assert!(!passive_critter_def.is_player());
        assert!(passive_critter_def.is_ai());
        assert_eq!(passive_critter_def.components.turn_actor.as_ref().unwrap().speed, 90);
        assert_eq!(passive_critter_def.components.tile_sprite.as_ref().unwrap().tile_coords, (2, 16));
        assert_eq!(passive_critter_def.components.view_shed.as_ref().unwrap().radius, 5);
    }
}
