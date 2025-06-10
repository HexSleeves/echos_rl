use bevy::prelude::*;
use big_brain::prelude::*;
use echos_assets::entities::{AIBehaviorType, EntityDefinition, EntityDefinitions};

use crate::{
    core::{
        bundles::{EnemyBundle, PlayerBundle},
        components::{Description, FieldOfView, Health, Inventory, InventoryItem, Position, Stats},
        resources::{CurrentMap, TurnQueue},
    },
    gameplay::{
        enemies::components::{
            ChasePlayerAction, ChasePlayerScorer, FleeFromPlayerAction, FleeFromPlayerScorer, IdleAction,
            WanderAction, WanderScorer,
        },
        turns::components::TurnActor,
    },
    prelude::gameplay::enemies::AttackAction,
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

    // Add Health component
    if let Some(health_data) = &definition.components.health {
        entity_commands.insert(Health::new_with_current(health_data.current, health_data.max));
    } else {
        // Default health if not specified
        entity_commands.insert(Health::new(100));
    }

    // Add Stats component
    if let Some(stats_data) = &definition.components.stats {
        entity_commands.insert(Stats {
            strength: stats_data.strength,
            defense: stats_data.defense,
            intelligence: stats_data.intelligence,
            agility: stats_data.agility,
            vitality: stats_data.vitality,
            luck: stats_data.luck,
        });
    } else {
        // Default balanced stats if not specified
        entity_commands.insert(Stats::balanced(10));
    }

    // Add Inventory component (only if specified)
    if let Some(inventory_data) = &definition.components.inventory {
        let mut inventory = Inventory::new(inventory_data.max_slots, inventory_data.max_weight);

        // Add starting items if specified
        if let Some(starting_items) = &inventory_data.starting_items {
            for (item_id, quantity) in starting_items {
                // Create a basic inventory item (in a real game, you'd look up item definitions)
                let item = InventoryItem::new(
                    item_id.clone(),
                    item_id.clone(), // Use ID as name for now
                    *quantity,
                    99,                     // Default max stack
                    1.0,                    // Default weight
                    format!("A {item_id}"), // Default description
                );

                if let Err(e) = inventory.add_item(item) {
                    warn!("Failed to add starting item '{}' to inventory: {:?}", item_id, e);
                }
            }
        }

        entity_commands.insert(inventory);
    }

    // Add Description component
    if let Some(description_data) = &definition.components.description {
        entity_commands.insert(Description::new(&description_data.text));
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
    match behavior_type {
        AIBehaviorType::Hostile => {
            // Hostile enemies are aggressive and prioritize chasing
            // Higher threshold (0.7) means they need strong conviction to act
            // They will chase when they see the player, otherwise wander to search
            let thinker = Thinker::build()
                .label("Hostile")
                .picker(FirstToScore { threshold: 0.7 })
                .when(ChasePlayerScorer, Steps::build().step(ChasePlayerAction::default()).step(AttackAction))
                .otherwise(WanderAction::default());

            entity_commands.insert((
                thinker,
                ChasePlayerScorer,
                ChasePlayerAction::default(),
                WanderScorer,
                WanderAction::default(),
            ));
        }
        AIBehaviorType::Passive => {
            // Passive enemies prioritize survival and flee from threats
            // Lower threshold (0.5) means they're more reactive to danger
            // They will flee when threatened, otherwise wander peacefully
            let thinker = Thinker::build()
                .label("Passive")
                .picker(FirstToScore { threshold: 0.5 })
                .when(FleeFromPlayerScorer, FleeFromPlayerAction::default())
                .when(WanderScorer, WanderAction::default())
                .otherwise(IdleAction);

            entity_commands.insert((
                thinker,
                FleeFromPlayerScorer,
                WanderScorer,
                FleeFromPlayerAction::default(),
                WanderAction::default(),
                IdleAction,
            ));
        }
        AIBehaviorType::Neutral => {
            // Neutral enemies are peaceful and just go about their business
            // Very low threshold (0.3) means they're content to just wander
            // They ignore the player completely and focus on their own activities
            let thinker = Thinker::build()
                .label("Neutral")
                .picker(FirstToScore { threshold: 0.3 })
                .when(WanderScorer, WanderAction::default())
                .otherwise(IdleAction);

            entity_commands.insert((thinker, WanderScorer, WanderAction::default(), IdleAction));
        }
    }
}
