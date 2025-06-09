use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use super::components::*;

/// Main entity definition loaded from RON files
#[derive(Asset, Serialize, Deserialize, Debug, Clone, TypePath)]
pub struct EntityDefinition {
    /// Human-readable name for the entity
    pub name: String,
    /// Optional description for documentation
    pub description: String,
    /// All component data for this entity
    pub components: EntityComponents,
}

/// Container for all possible component data
#[derive(Serialize, Deserialize, Debug, Clone, Reflect, Default)]
pub struct EntityComponents {
    // Core gameplay components
    pub turn_actor: Option<TurnActorData>,
    pub field_of_view: Option<FieldOfViewData>,
    pub tile_sprite: Option<TileSpriteData>,

    // core components from enhanced ECS system
    pub health: Option<HealthData>,
    pub stats: Option<StatsData>,
    pub inventory: Option<InventoryData>,
    pub description: Option<DescriptionData>,

    // Entity type tags
    pub is_player: Option<bool>,
    pub is_ai: Option<bool>,

    // AI-specific components
    pub ai_behavior_type: Option<AIBehaviorType>,

    // Spawning and gameplay properties
    pub spawn_weight: Option<f32>,
    pub level_range: Option<(u32, u32)>,
}

impl EntityDefinition {
    /// Create a new entity definition
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self { name: name.into(), description: description.into(), components: EntityComponents::default() }
    }

    /// Set the components
    pub fn with_components(mut self, components: EntityComponents) -> Self {
        self.components = components;
        self
    }

    /// Check if this entity should be treated as a player
    pub fn is_player(&self) -> bool { self.components.is_player.unwrap_or(false) }

    /// Check if this entity should be treated as AI
    pub fn is_ai(&self) -> bool { self.components.is_ai.unwrap_or(false) }

    /// Get spawn weight for random selection
    pub fn spawn_weight(&self) -> f32 { self.components.spawn_weight.unwrap_or(1.0) }

    /// Get AI behavior type (defaults to Neutral if not specified)
    pub fn ai_behavior_type(&self) -> AIBehaviorType { self.components.ai_behavior_type.unwrap_or_default() }

    /// Validate the entity definition for correctness
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        // Validate name
        if self.name.trim().is_empty() {
            errors.push("Entity name cannot be empty".to_string());
        }

        // Validate health data
        if let Some(health) = &self.components.health {
            if health.max <= 0 {
                errors.push("Health max must be positive".to_string());
            }
            if health.current < 0 {
                errors.push("Health current cannot be negative".to_string());
            }
            if health.current > health.max {
                errors.push("Health current cannot exceed max".to_string());
            }
        }

        // Validate stats data
        if let Some(stats) = &self.components.stats {
            let stat_values = [
                ("strength", stats.strength),
                ("defense", stats.defense),
                ("intelligence", stats.intelligence),
                ("agility", stats.agility),
                ("vitality", stats.vitality),
                ("luck", stats.luck),
            ];

            for (name, value) in stat_values {
                if value < 0 {
                    errors.push(format!("Stat '{name}' cannot be negative"));
                }
                if value > 100 {
                    errors.push(format!("Stat '{name}' cannot exceed 100"));
                }
            }
        }

        // Validate turn actor data
        if let Some(turn_actor) = &self.components.turn_actor {
            if turn_actor.speed == 0 {
                errors.push("Turn actor speed must be positive".to_string());
            }
            if turn_actor.speed > 10000 {
                errors.push("Turn actor speed cannot exceed 10000".to_string());
            }
        }

        // Validate field of view data
        if let Some(fov) = &self.components.field_of_view {
            if fov.0 == 0 {
                errors.push("Field of view radius must be positive".to_string());
            }
            if fov.0 > 50 {
                errors.push("Field of view radius cannot exceed 50".to_string());
            }
        }

        // Validate inventory data
        if let Some(inventory) = &self.components.inventory {
            if inventory.max_slots == 0 {
                errors.push("Inventory max slots must be positive".to_string());
            }
            if inventory.max_weight <= 0.0 {
                errors.push("Inventory max weight must be positive".to_string());
            }
        }

        // Validate level range
        if let Some((min, max)) = self.components.level_range {
            if min > max {
                errors.push(format!("Level range minimum ({min}) cannot exceed maximum ({max})"));
            }
            if min == 0 {
                errors.push("Level range minimum must be at least 1".to_string());
            }
        }

        // Validate spawn weight
        if let Some(weight) = self.components.spawn_weight
            && weight < 0.0
        {
            errors.push("Spawn weight cannot be negative".to_string());
        }

        if errors.is_empty() { Ok(()) } else { Err(errors) }
    }
}

impl EntityComponents {
    /// Create a new empty component set
    pub fn new() -> Self { Self::default() }

    /// Set turn actor data
    pub fn with_turn_actor(mut self, data: TurnActorData) -> Self {
        self.turn_actor = Some(data);
        self
    }

    /// Set view shed data
    pub fn with_field_of_view(mut self, data: FieldOfViewData) -> Self {
        self.field_of_view = Some(data);
        self
    }

    /// Set tile sprite data
    pub fn with_tile_sprite(mut self, data: TileSpriteData) -> Self {
        self.tile_sprite = Some(data);
        self
    }

    /// Set health data
    pub fn with_health(mut self, data: HealthData) -> Self {
        self.health = Some(data);
        self
    }

    /// Set stats data
    pub fn with_stats(mut self, data: StatsData) -> Self {
        self.stats = Some(data);
        self
    }

    /// Set inventory data
    pub fn with_inventory(mut self, data: InventoryData) -> Self {
        self.inventory = Some(data);
        self
    }

    /// Set description data
    pub fn with_description(mut self, data: DescriptionData) -> Self {
        self.description = Some(data);
        self
    }

    /// Mark as player entity
    pub fn as_player(mut self) -> Self {
        self.is_player = Some(true);
        self.is_ai = Some(false);
        self
    }

    /// Mark as AI entity
    pub fn as_ai(mut self) -> Self {
        self.is_ai = Some(true);
        self.is_player = Some(false);
        self
    }

    /// Set AI behavior type
    pub fn with_ai_behavior_type(mut self, behavior_type: AIBehaviorType) -> Self {
        self.ai_behavior_type = Some(behavior_type);
        self
    }

    /// Set spawn weight
    pub fn with_spawn_weight(mut self, weight: f32) -> Self {
        self.spawn_weight = Some(weight);
        self
    }

    /// Set level range
    pub fn with_level_range(mut self, min: u32, max: u32) -> Self {
        assert!(min <= max, "Level range minimum ({min}) cannot be greater than maximum ({max})");
        self.level_range = Some((min, max));
        self
    }
}
