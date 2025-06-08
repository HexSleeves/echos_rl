use bevy::prelude::*;
use big_brain::prelude::*;
use echos_assets::entities::{AIBehaviorType, EntityDefinition, EntityDefinitions};

use crate::{
    core::{
        bundles::{EnemyBundle, PlayerBundle},
        components::{FieldOfView, Position},
        resources::{CurrentMap, TurnQueue},
    },
    gameplay::{
        enemies::components::{ChasePlayerAction, ChasePlayerScorer, IdleAction, WanderAction, WanderScorer},
        turns::components::TurnActor,
    },
    rendering::components::TileSprite,
};

/// Configuration for entity-specific components and defaults
struct EntitySpawnConfig {
    default_view_radius: u8,
    default_turn_speed: u32,
}

impl EntitySpawnConfig {
    fn player() -> Self { Self { default_view_radius: 8, default_turn_speed: 1000 } }
    fn ai() -> Self { Self { default_view_radius: 6, default_turn_speed: 1000 } }
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

    let mut entity_commands = commands.spawn(PlayerBundle::new(&definition.name, position));

    // Add common components using helper function
    let config = EntitySpawnConfig::player();
    add_common_components(&mut entity_commands, definition, &config);

    let player_id = entity_commands.id();

    // Finalize spawn using helper function
    finalize_entity_spawn(player_id, position, "player", current_map, turn_queue)
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
    let enemy_bundle = EnemyBundle::new(&definition.name, &definition.description, position, behavior_type);
    let mut entity_commands = commands.spawn(enemy_bundle);

    // Add common components using helper function
    let config = EntitySpawnConfig::ai();

    // Add common components
    add_common_components(&mut entity_commands, definition, &config);
    // Add big-brain AI components based on behavior type
    add_big_brain_components(&mut entity_commands, behavior_type);

    let ai_id = entity_commands.id();

    // Finalize spawn using helper function
    finalize_entity_spawn(ai_id, position, "AI", current_map, turn_queue)
}

/// Helper function to add common components from entity definition
fn add_common_components(
    entity_commands: &mut EntityCommands,
    definition: &EntityDefinition,
    config: &EntitySpawnConfig,
) {
    // Add TurnActor component
    entity_commands.insert(TurnActor::new(
        definition.components.turn_actor.as_ref().map(|data| data.speed).unwrap_or(config.default_turn_speed),
    ));

    // Add FieldOfView component
    entity_commands.insert(FieldOfView::new(
        definition.components.field_of_view.as_ref().map(|data| data.0).unwrap_or(config.default_view_radius),
    ));

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
    current_map: &mut CurrentMap,
    turn_queue: &mut TurnQueue,
) -> Result<Entity, String> {
    // Add to turn queue (schedule first turn immediately)
    turn_queue.schedule_now(entity_id);

    // Update map with entity position
    current_map
        .place_actor(position, entity_id)
        .map_err(|e| format!("Failed to place {entity_type}: {e}"))?;

    Ok(entity_id)
}

/// Add big-brain components to an AI entity based on its behavior type
fn add_big_brain_components(entity_commands: &mut EntityCommands, behavior_type: AIBehaviorType) {
    let _ = behavior_type;

    let thinker = Thinker::build()
        .picker(FirstToScore { threshold: 0.6 })
        .when(ChasePlayerScorer, ChasePlayerAction::default())
        .otherwise(WanderAction::default());
    // .when(WanderScorer, WanderAction)
    // .otherwise(IdleAction);

    entity_commands.insert((
        thinker,
        ChasePlayerScorer,
        WanderScorer,
        ChasePlayerAction::default(),
        WanderAction::default(),
        IdleAction,
    ));

    // match behavior_type {
    //     AIBehaviorType::Hostile => {
    //         // Hostile enemies prioritize chasing when they see the player
    //         let thinker = Thinker::build()
    //             .picker(FirstToScore { threshold: 0.6 })
    //             .when(ChasePlayerScorer, ChasePlayerAction::default())
    //             .when(WanderScorer, WanderAction)
    //             .otherwise(IdleAction);

    //         entity_commands.insert((
    //             thinker,
    //             ChasePlayerScorer,
    //             WanderScorer,
    //             ChasePlayerAction::default(),
    //             WanderAction,
    //             IdleAction,
    //         ));
    //     }
    //     AIBehaviorType::Passive => {
    //         // Passive enemies prioritize fleeing from threats
    //         let thinker = Thinker::build()
    //             .picker(FirstToScore { threshold: 0.5 })
    //             .when(FleeFromPlayerScorer, FleeFromPlayerAction)
    //             .when(WanderScorer, WanderAction)
    //             .otherwise(IdleAction);

    //         entity_commands.insert((
    //             thinker,
    //             FleeFromPlayerScorer,
    //             WanderScorer,
    //             FleeFromPlayerAction,
    //             WanderAction,
    //             IdleAction,
    //         ));
    //     }
    //     AIBehaviorType::Neutral => {
    //         // Neutral enemies just wander around
    //         let thinker = Thinker::build()
    //             .picker(FirstToScore { threshold: 0.3 })
    //             .when(WanderScorer, WanderAction)
    //             .otherwise(IdleAction);

    //         entity_commands.insert((thinker, WanderScorer, WanderAction, IdleAction));
    //     }
    // }
}
