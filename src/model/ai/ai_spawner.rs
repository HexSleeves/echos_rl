use bevy::prelude::*;
use big_brain::prelude::*;

use crate::{
    assets::entities::{EntityDefinition, EntityDefinitions},
    model::{
        components::{
            AIBehavior, AIBehaviorType, AIState, AITag, ChasePlayerAction, ChasePlayerScorer,
            FleeFromPlayerAction, FleeFromPlayerScorer, IdleAction, Position, TurnActor, ViewShed,
            WanderAction, WanderScorer,
        },
        resources::CurrentMap,
    },
    view::components::TileSprite,
};

/// System to spawn enemies with AI behaviors
pub fn spawn_ai_enemies(
    mut commands: Commands,
    current_map: Res<CurrentMap>,
    entity_definitions: Option<Res<EntityDefinitions>>,
    assets: Option<Res<Assets<EntityDefinition>>>,
) {
    let Some(entity_definitions) = entity_definitions else {
        warn!("EntityDefinitions resource not available for AI spawning");
        return;
    };

    let Some(assets) = assets else {
        warn!("EntityDefinition assets not available for AI spawning");
        return;
    };

    println!("entity_definitions {:#?}", entity_definitions);

    // Spawn different types of AI enemies using simple names
    spawn_enemy_type(&mut commands, &current_map, &entity_definitions, &assets, "hostile_guard", 2);
    spawn_enemy_type(&mut commands, &current_map, &entity_definitions, &assets, "passive_critter", 3);
    spawn_enemy_type(&mut commands, &current_map, &entity_definitions, &assets, "neutral_wanderer", 2);
}

fn spawn_enemy_type(
    commands: &mut Commands,
    current_map: &CurrentMap,
    entity_definitions: &EntityDefinitions,
    assets: &Assets<EntityDefinition>,
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
            spawn_ai_entity(commands, definition, spawn_pos);
        } else {
            warn!("Could not find spawn position for {}", enemy_type);
        }
    }
}

fn spawn_ai_entity(commands: &mut Commands, definition: &EntityDefinition, position: Position) {
    let components = &definition.components;

    // Determine AI behavior type based on the entity name
    let ai_behavior = determine_ai_behavior_from_definition(definition);
    let behavior_type = ai_behavior.behavior_type.clone();

    // Create the appropriate thinker based on behavior type
    let thinker = create_thinker_for_behavior(&ai_behavior.behavior_type);

    let mut entity_commands = commands.spawn((
        // Core position and AI components
        position,
        AITag,
        ai_behavior,
        AIState::default(),
        // Big-brain thinker component
        thinker,
    ));

    // Add turn actor if specified
    if let Some(turn_data) = &components.turn_actor {
        entity_commands.insert(TurnActor::new(turn_data.speed));
    }

    // Add view shed if specified
    if let Some(view_data) = &components.view_shed {
        entity_commands.insert(ViewShed::new(view_data.radius));
    }

    // Add tile sprite if specified
    if let Some(sprite_data) = &components.tile_sprite {
        entity_commands.insert(TileSprite {
            tile_coords: sprite_data.tile_coords,
            tile_size: sprite_data.tile_size.map(|(x, y)| Vec2::new(x, y)).unwrap_or(Vec2::splat(12.0)),
            tint: sprite_data.tint.map(|t| Color::srgba(t.0, t.1, t.2, t.3)),
        });
    }

    info!("Spawned AI entity '{}' at {:?} with behavior {:?}", definition.name, position, behavior_type);
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
