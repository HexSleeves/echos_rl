use bevy::prelude::*;
use big_brain::prelude::*;
use echos_assets::entities::{AIBehaviorType, EntityDefinition, EntityDefinitions};

use crate::{
    core::{
        components::{AITag, Description, PlayerTag, Position, ViewShed},
        resources::{CurrentMap, TurnQueue},
    },
    gameplay::{
        enemies::components::{
            AIBehavior, AIState, ChasePlayerAction, ChasePlayerScorer, FleeFromPlayerAction,
            FleeFromPlayerScorer, IdleAction, WanderAction, WanderScorer,
        },
        player::components::AwaitingInput,
        turns::components::TurnActor,
    },
    rendering::components::TileSprite,
};

/// Configuration for entity-specific components and defaults
struct EntitySpawnConfig {
    default_view_radius: i32,
    default_turn_speed: u64,
}

impl EntitySpawnConfig {
    fn player() -> Self { Self { default_view_radius: 8, default_turn_speed: 100 } }
    fn ai() -> Self { Self { default_view_radius: 6, default_turn_speed: 100 } }
}

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
    let definition = assets.get(player_handle).ok_or("Player definition not loaded")?;

    let mut entity_commands =
        commands.spawn((position, PlayerTag, AwaitingInput, Description::new(&definition.name)));

    // Add common components using helper function
    let config = EntitySpawnConfig::player();
    add_common_components(&mut entity_commands, definition, &config);

    let player_id = entity_commands.id();

    // Finalize spawn using helper function
    finalize_entity_spawn(player_id, position, "player", &definition.name, current_map, turn_queue)
}

/// Spawn a specific AI entity from definition data
pub fn spawn_ai_from_definition(
    commands: Commands,
    entity_definitions: &EntityDefinitions,
    assets: &Assets<EntityDefinition>,
    ai_name: &str,
    position: Position,
    current_map: &mut CurrentMap,
    turn_queue: &mut TurnQueue,
) -> Result<Entity, String> {
    let ai_handle = entity_definitions
        .get_by_name(ai_name)
        .ok_or_else(|| format!("AI definition '{ai_name}' not found"))?;

    let definition = assets.get(ai_handle).ok_or_else(|| format!("AI definition '{ai_name}' not loaded"))?;

    spawn_ai_entity(commands, definition, position, current_map, turn_queue)
}

/// Spawn a random AI entity from available definitions
pub fn spawn_random_ai_from_definition(
    commands: Commands,
    entity_definitions: &EntityDefinitions,
    assets: &Assets<EntityDefinition>,
    position: Position,
    current_map: &mut CurrentMap,
    turn_queue: &mut TurnQueue,
) -> Result<Entity, String> {
    let random_handle = entity_definitions.get_random_enemy().ok_or("No enemy definitions available")?;

    let definition = assets.get(random_handle).ok_or("Random enemy definition not loaded")?;

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
    // Get AI behavior type directly from entity definition
    let behavior_type = definition.ai_behavior_type();
    let ai_behavior = create_ai_behavior_for_type(behavior_type.clone());

    let mut entity_commands = commands.spawn((
        position,
        AITag,
        Description::new(&definition.name),
        ai_behavior,
        AIState::default(),
    ));

    // Add common components using helper function
    let config = EntitySpawnConfig::ai();
    add_common_components(&mut entity_commands, definition, &config);

    // Add big-brain AI components based on behavior type
    add_big_brain_components(&mut entity_commands, behavior_type);

    let ai_id = entity_commands.id();

    // Finalize spawn using helper function
    finalize_entity_spawn(ai_id, position, "AI", &definition.name, current_map, turn_queue)
}

/// Helper function to add common components from entity definition
fn add_common_components(
    entity_commands: &mut EntityCommands,
    definition: &EntityDefinition,
    config: &EntitySpawnConfig,
) {
    // Add TurnActor component
    if let Some(turn_data) = &definition.components.turn_actor {
        entity_commands.insert(TurnActor::new(turn_data.speed));
    } else {
        entity_commands.insert(TurnActor::new(config.default_turn_speed));
    }

    // Add ViewShed component
    if let Some(view_data) = &definition.components.view_shed {
        entity_commands.insert(ViewShed::new(view_data.radius as i32));
    } else {
        entity_commands.insert(ViewShed::new(config.default_view_radius));
    }

    // Add TileSprite component
    if let Some(sprite_data) = &definition.components.tile_sprite {
        entity_commands.insert(TileSprite {
            tile_coords: sprite_data.tile_coords,
            tile_size: sprite_data.tile_size.unwrap_or((12.0, 12.0)).into(),
            ..Default::default()
        });
    }
}

/// Helper function to handle turn queue scheduling and map placement
fn finalize_entity_spawn(
    entity_id: Entity,
    position: Position,
    entity_type: &str,
    entity_name: &str,
    current_map: &mut CurrentMap,
    turn_queue: &mut TurnQueue,
) -> Result<Entity, String> {
    // Add to turn queue (schedule first turn immediately)
    turn_queue.schedule_now(entity_id);

    // Update map with entity position
    current_map
        .place_actor(position, entity_id)
        .map_err(|e| format!("Failed to place {}: {e}", entity_type))?;

    info!("Spawned {} '{}' at {:?}", entity_type, entity_name, position);
    Ok(entity_id)
}

/// Create AI behavior component for the given type
fn create_ai_behavior_for_type(behavior_type: AIBehaviorType) -> AIBehavior {
    match behavior_type {
        AIBehaviorType::Hostile => AIBehavior::hostile(6),
        AIBehaviorType::Passive => AIBehavior::passive(5),
        AIBehaviorType::Neutral => AIBehavior::neutral(3),
    }
}

/// Add big-brain components to an AI entity based on its behavior type
fn add_big_brain_components(entity_commands: &mut EntityCommands, behavior_type: AIBehaviorType) {
    match behavior_type {
        AIBehaviorType::Hostile => {
            // Hostile enemies prioritize chasing when they see the player
            let thinker = Thinker::build()
                .picker(FirstToScore { threshold: 0.6 })
                .when(ChasePlayerScorer, ChasePlayerAction)
                .when(WanderScorer, WanderAction)
                .otherwise(IdleAction);

            entity_commands.insert((
                thinker,
                ChasePlayerScorer,
                WanderScorer,
                ChasePlayerAction,
                WanderAction,
                IdleAction,
            ));
        }
        AIBehaviorType::Passive => {
            // Passive enemies prioritize fleeing from threats
            let thinker = Thinker::build()
                .picker(FirstToScore { threshold: 0.5 })
                .when(FleeFromPlayerScorer, FleeFromPlayerAction)
                .when(WanderScorer, WanderAction)
                .otherwise(IdleAction);

            entity_commands.insert((
                thinker,
                FleeFromPlayerScorer,
                WanderScorer,
                FleeFromPlayerAction,
                WanderAction,
                IdleAction,
            ));
        }
        AIBehaviorType::Neutral => {
            // Neutral enemies just wander around
            let thinker = Thinker::build()
                .picker(FirstToScore { threshold: 0.3 })
                .when(WanderScorer, WanderAction)
                .otherwise(IdleAction);

            entity_commands.insert((thinker, WanderScorer, WanderAction, IdleAction));
        }
    }
}
