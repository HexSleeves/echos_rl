use bevy::{platform::collections::HashMap, prelude::*};
use bevy_asset_loader::prelude::*;
use std::fmt;

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
}

/// Extract simple name from file path
/// e.g., "entities/enemies/hostile_guard.definition.ron" -> "hostile_guard"
fn extract_name_from_path(path: &str) -> Option<&str> {
    path.split('/').next_back()?.strip_suffix(".definition.ron")
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
}
