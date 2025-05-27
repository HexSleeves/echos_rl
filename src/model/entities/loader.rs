use bevy::{platform::collections::HashMap, prelude::*};
use bevy_asset_loader::prelude::*;

use super::definition::EntityDefinition;

/// Resource containing all loaded entity definitions
#[derive(AssetCollection, Resource)]
pub struct EntityDefinitions {
    /// All entity definition files loaded from the entities folder
    #[asset(key = "entity_definitions", collection(typed, mapped))]
    pub definitions: HashMap<String, Handle<EntityDefinition>>,
}

impl EntityDefinitions {
    /// Get an entity definition by name
    pub fn get(&self, name: &str) -> Option<&Handle<EntityDefinition>> {
        self.definitions.get(name)
    }

    /// Get the player entity definition
    pub fn get_player(&self) -> Option<&Handle<EntityDefinition>> { self.get("player") }

    /// Get all enemy entity definitions
    pub fn get_enemies(&self) -> Vec<&Handle<EntityDefinition>> {
        self.definitions
            .iter()
            .filter(|(name, _)| name.starts_with("enemies/"))
            .map(|(_, handle)| handle)
            .collect()
    }

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
        self.definitions.values().all(|handle| asset_server.is_loaded_with_dependencies(handle))
    }

    /// Get all definition names
    pub fn get_names(&self) -> Vec<&String> { self.definitions.keys().collect() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entity_definitions_creation() {
        // Test that we can create an empty EntityDefinitions resource
        let definitions = EntityDefinitions { definitions: HashMap::new() };

        assert!(definitions.get("player").is_none());
        assert!(definitions.get_enemies().is_empty());
        assert!(definitions.get_random_enemy().is_none());
        assert!(definitions.get_names().is_empty());
    }

    #[test]
    fn test_enemy_filtering() {
        let mut definitions = HashMap::new();

        // Create mock handles (these won't actually load anything in tests)
        let player_handle = Handle::default();
        let whale_handle = Handle::default();
        let goblin_handle = Handle::default();

        definitions.insert("player".to_string(), player_handle);
        definitions.insert("enemies/whale".to_string(), whale_handle);
        definitions.insert("enemies/goblin".to_string(), goblin_handle);

        let entity_definitions = EntityDefinitions { definitions };

        // Test enemy filtering
        let enemies = entity_definitions.get_enemies();
        assert_eq!(enemies.len(), 2);

        // Test player access
        assert!(entity_definitions.get_player().is_some());

        // Test names
        let names = entity_definitions.get_names();
        assert_eq!(names.len(), 3);
        assert!(names.contains(&&"player".to_string()));
        assert!(names.contains(&&"enemies/whale".to_string()));
        assert!(names.contains(&&"enemies/goblin".to_string()));
    }

    #[test]
    fn test_asset_loading_integration() {
        // This test verifies that our asset loading configuration is correct
        // by checking that the RON files can be parsed as EntityDefinition assets

        // Test player.ron parsing
        let player_ron = include_str!("../../../assets/entities/player.ron");
        let player_def: EntityDefinition =
            ron::from_str(player_ron).expect("Failed to parse player.ron as EntityDefinition");

        assert_eq!(player_def.name, "Player");
        assert!(player_def.is_player());

        // Test whale.ron parsing
        let whale_ron = include_str!("../../../assets/entities/enemies/whale.ron");
        let whale_def: EntityDefinition =
            ron::from_str(whale_ron).expect("Failed to parse whale.ron as EntityDefinition");

        assert_eq!(whale_def.name, "Whale");
        assert!(whale_def.is_ai());

        // Test basic_enemy.ron parsing
        let basic_enemy_ron = include_str!("../../../assets/entities/enemies/basic_enemy.ron");
        let basic_enemy_def: EntityDefinition = ron::from_str(basic_enemy_ron)
            .expect("Failed to parse basic_enemy.ron as EntityDefinition");

        assert_eq!(basic_enemy_def.name, "Basic Enemy");
        assert!(basic_enemy_def.is_ai());
    }
}
