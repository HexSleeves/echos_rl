use bevy::{platform::collections::HashMap, prelude::*};
use bevy_asset_loader::prelude::*;
use std::fmt;

use crate::entities::EntityComponents;

use super::definition::EntityDefinition;

/// Pretty-prints HashMaps with sorted keys and multi-line formatting
struct PrettyHashMap<'a, K, V>(&'a HashMap<K, V>);

impl<'a, K: fmt::Debug + Ord, V: fmt::Debug> fmt::Debug for PrettyHashMap<'a, K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut entries: Vec<_> = self.0.iter().collect();
        // Sort by borrowed key to avoid moving/cloning
        entries.sort_by(|(k1, _), (k2, _)| k1.cmp(k2));

        let mut map = f.debug_map();
        for (key, value) in entries {
            map.entry(key, value);
        }
        map.finish()
    }
}

/// Resource containing all loaded entity definitions
#[derive(AssetCollection, Resource)]
pub struct EntityDefinitions {
    #[asset(key = "player")]
    pub player: Handle<EntityDefinition>,

    /// All entity definition files loaded from the entities folder
    #[asset(key = "entities", collection(typed, mapped))]
    pub definitions: HashMap<String, Handle<EntityDefinition>>,

    /// All enemy definition files loaded from the entities folder
    #[asset(key = "enemies", collection(typed, mapped))]
    pub enemies: HashMap<String, Handle<EntityDefinition>>,
}

impl fmt::Debug for EntityDefinitions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("EntityDefinitions")
            .field("player", &self.player)
            .field("definitions", &PrettyHashMap(&self.definitions))
            .field("enemies", &PrettyHashMap(&self.enemies))
            .finish()
    }
}

impl EntityDefinitions {
    /// Get an entity definition by name
    pub fn get(&self, name: &str) -> Option<&Handle<EntityDefinition>> { self.definitions.get(name) }

    /// Get an entity definition by simple name (extracts from file path)
    pub fn get_by_name(&self, simple_name: &str) -> Option<&Handle<EntityDefinition>> {
        // First try direct lookup
        if let Some(handle) = self.definitions.get(simple_name) {
            return Some(handle);
        }

        // Then try to find by extracting name from file path
        for (path, handle) in &self.definitions {
            if let Some(extracted_name) = extract_name_from_path(path)
                && extracted_name == simple_name
            {
                return Some(handle);
            }
        }

        None
    }

    /// Get the player entity definition
    pub fn get_player(&self) -> &Handle<EntityDefinition> { &self.player }

    /// Get all enemy entity definitions
    pub fn get_enemies(&self) -> Vec<&Handle<EntityDefinition>> { self.enemies.values().collect() }

    /// Get a random enemy definition handle
    pub fn get_random_enemy(&self) -> Option<&Handle<EntityDefinition>> {
        let enemies = self.get_enemies();
        if enemies.is_empty() {
            None
        } else {
            use fastrand;
            let index = fastrand::usize(..enemies.len());
            Some(enemies[index])
        }
    }

    /// Check if all definitions are loaded
    pub fn is_loaded(&self, asset_server: &AssetServer) -> bool {
        self.definitions.values().all(|h| asset_server.is_loaded_with_dependencies(h))
            && self.enemies.values().all(|h| asset_server.is_loaded_with_dependencies(h))
    }

    /// Get all definition names
    pub fn get_names(&self) -> Vec<&String> { self.definitions.keys().collect() }

    /// Validate all loaded entity definitions
    pub fn validate_all(&self, assets: &Assets<EntityDefinition>) -> ValidationReport {
        let mut report = ValidationReport::new();

        // Validate player definition
        if let Some(player_def) = assets.get(&self.player) {
            if let Err(errors) = player_def.validate() {
                report.add_errors("player", errors);
            }
        } else {
            report.add_error("player", "Player definition not loaded".to_string());
        }

        // Validate all entity definitions
        for (name, handle) in &self.definitions {
            if let Some(definition) = assets.get(handle) {
                if let Err(errors) = definition.validate() {
                    report.add_errors(name, errors);
                }
            } else {
                report.add_error(name, format!("Definition '{name}' not loaded"));
            }
        }

        // Validate enemy definitions
        for (name, handle) in &self.enemies {
            if let Some(definition) = assets.get(handle) {
                if let Err(errors) = definition.validate() {
                    report.add_errors(&format!("enemies/{name}"), errors);
                }
            } else {
                report.add_error(&format!("enemies/{name}"), format!("Enemy definition '{name}' not loaded"));
            }
        }

        report
    }

    /// Get a fallback entity definition for when loading fails
    pub fn get_fallback_player() -> EntityDefinition {
        use super::components::*;

        EntityDefinition::new("Fallback Player", "Default player when definition loading fails")
            .with_components(
                EntityComponents::new()
                    .as_player()
                    .with_turn_actor(TurnActorData::new(100))
                    .with_field_of_view(FieldOfViewData::new(8))
                    .with_health(HealthData::new(100))
                    .with_stats(StatsData::balanced())
                    .with_inventory(InventoryData::new(30, 150.0))
                    .with_description(DescriptionData::new("A brave adventurer"))
                    .with_tile_sprite(TileSpriteData::new((0, 16))),
            )
    }

    /// Get a fallback enemy definition for when loading fails
    pub fn get_fallback_enemy() -> EntityDefinition {
        use super::components::*;

        EntityDefinition::new("Fallback Enemy", "Default enemy when definition loading fails")
            .with_components(
                EntityComponents::new()
                    .as_ai()
                    .with_ai_behavior_type(AIBehaviorType::Neutral)
                    .with_turn_actor(TurnActorData::new(100))
                    .with_field_of_view(FieldOfViewData::new(6))
                    .with_health(HealthData::new(50))
                    .with_stats(StatsData::balanced())
                    .with_description(DescriptionData::new("A mysterious creature"))
                    .with_tile_sprite(TileSpriteData::new((26, 2)))
                    .with_spawn_weight(1.0)
                    .with_level_range(1, 10),
            )
    }
}

/// Report containing validation results for entity definitions
#[derive(Debug, Default)]
pub struct ValidationReport {
    /// Errors found during validation, grouped by entity name
    pub errors: HashMap<String, Vec<String>>,
}

impl ValidationReport {
    pub fn new() -> Self { Self::default() }

    pub fn add_error(&mut self, entity_name: &str, error: String) {
        self.errors.entry(entity_name.to_string()).or_default().push(error);
    }

    pub fn add_errors(&mut self, entity_name: &str, errors: Vec<String>) {
        self.errors.entry(entity_name.to_string()).or_default().extend(errors);
    }

    pub fn has_errors(&self) -> bool { !self.errors.is_empty() }

    pub fn total_error_count(&self) -> usize { self.errors.values().map(|v| v.len()).sum() }

    pub fn print_report(&self) {
        if !self.has_errors() {
            info!("✅ All entity definitions are valid");
            return;
        }

        error!("❌ Entity definition validation failed with {} errors:", self.total_error_count());

        for (entity_name, errors) in &self.errors {
            error!("  Entity '{entity_name}' has {} error(s):", errors.len());
            for error in errors {
                error!("    - {}", error);
            }
        }
    }
}

/// Extract simple name from file path
/// e.g., "entities/enemies/hostile_guard.definition.ron" -> "hostile_guard"
fn extract_name_from_path(path: &str) -> Option<&str> {
    path.split('/').next_back()?.strip_suffix(".definition.ron")
}

/// System to validate entity definitions after they're loaded
pub fn validate_entity_definitions(
    entity_definitions: Option<Res<EntityDefinitions>>,
    assets: Option<Res<Assets<EntityDefinition>>>,
    asset_server: Option<Res<AssetServer>>,
    mut validation_done: Local<bool>,
) {
    // Only run validation once when both resources are available
    if *validation_done {
        return;
    }

    if let (Some(definitions), Some(assets), Some(asset_server)) =
        (entity_definitions.as_ref(), assets.as_ref(), asset_server.as_ref())
        && definitions.is_loaded(asset_server)
    {
        let report = definitions.validate_all(assets);
        report.print_report();
        *validation_done = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entity_definitions_creation() {
        // Test that we can create an empty EntityDefinitions resource
        let definitions = EntityDefinitions {
            definitions: HashMap::new(),
            player: Handle::default(),
            enemies: HashMap::new(),
        };

        assert!(definitions.get("player").is_none());
        assert!(definitions.get_enemies().is_empty());
        assert!(definitions.get_random_enemy().is_none());
        assert!(definitions.get_names().is_empty());
    }

    #[test]
    fn test_asset_loading_integration() {
        // This test verifies that our EntityDefinitions structure
        // is compatible with bevy_asset_loader patterns
        let definitions = EntityDefinitions {
            definitions: HashMap::new(),
            player: Handle::default(),
            enemies: HashMap::new(),
        };

        // Test basic functionality without requiring AssetServer
        assert!(definitions.get_enemies().is_empty());
        assert!(definitions.get_names().is_empty());
        assert!(definitions.get_random_enemy().is_none());
    }

    #[test]
    fn test_name_extraction() {
        // Test the helper function for extracting names from file paths
        assert_eq!(
            extract_name_from_path("entities/enemies/hostile_guard.definition.ron"),
            Some("hostile_guard")
        );
        assert_eq!(extract_name_from_path("entities/player.definition.ron"), Some("player"));
        assert_eq!(
            extract_name_from_path("entities/enemies/passive_critter.definition.ron"),
            Some("passive_critter")
        );
        assert_eq!(extract_name_from_path("invalid_path"), None);
        assert_eq!(extract_name_from_path(""), None);
    }

    #[test]
    fn test_get_by_name() {
        let mut definitions = HashMap::new();
        let handle = Handle::default();

        // Add a definition with full path as key
        definitions.insert("entities/enemies/hostile_guard.definition.ron".to_string(), handle.clone());

        let entity_definitions =
            EntityDefinitions { definitions, player: Handle::default(), enemies: HashMap::new() };

        // Test that we can find it by simple name
        assert!(entity_definitions.get_by_name("hostile_guard").is_some());
        assert!(entity_definitions.get_by_name("nonexistent").is_none());
    }

    #[test]
    fn test_validation_report() {
        let mut report = ValidationReport::new();
        assert!(!report.has_errors());
        assert_eq!(report.total_error_count(), 0);

        report.add_error("test_entity", "Test error".to_string());
        assert!(report.has_errors());
        assert_eq!(report.total_error_count(), 1);

        report.add_errors("test_entity", vec!["Error 2".to_string(), "Error 3".to_string()]);
        assert_eq!(report.total_error_count(), 3);
    }

    #[test]
    fn test_fallback_definitions() {
        let player_fallback = EntityDefinitions::get_fallback_player();
        assert_eq!(player_fallback.name, "Fallback Player");
        assert!(player_fallback.is_player());
        assert!(!player_fallback.is_ai());

        let enemy_fallback = EntityDefinitions::get_fallback_enemy();
        assert_eq!(enemy_fallback.name, "Fallback Enemy");
        assert!(!enemy_fallback.is_player());
        assert!(enemy_fallback.is_ai());
    }
}
