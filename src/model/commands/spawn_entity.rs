use bevy::prelude::*;

use crate::{
    assets::entities::{
        EntityDefinition, EntityDefinitions, spawn_enemy_from_definition,
        spawner::{spawn_player_from_definition, spawn_random_enemy_from_definition},
    },
    model::{
        components::Position,
        resources::{CurrentMap, TurnQueue},
    },
};

/// Entity command for spawning a player
#[derive(Component)]
pub struct SpawnPlayerCommand {
    pub position: Position,
}

/// Entity command for spawning an enemy
#[derive(Component)]
pub struct SpawnEnemyCommand {
    pub position: Position,
    pub enemy_name: Option<String>, // None for random enemy
}

/// System to process spawn commands
pub fn process_spawn_commands(
    mut commands: Commands,
    mut current_map: ResMut<CurrentMap>,
    mut turn_queue: ResMut<TurnQueue>,
    entity_definitions: Option<Res<EntityDefinitions>>,
    assets: Option<Res<Assets<EntityDefinition>>>,
    player_commands: Query<(Entity, &SpawnPlayerCommand)>,
    enemy_commands: Query<(Entity, &SpawnEnemyCommand)>,
) {
    // Process player spawn commands
    for (entity, spawn_cmd) in player_commands.iter() {
        let player_id = if let (Some(entity_definitions), Some(assets)) =
            (entity_definitions.as_ref(), assets.as_ref())
        {
            match spawn_player_from_definition(
                commands.reborrow(),
                entity_definitions,
                assets,
                spawn_cmd.position,
                &mut current_map,
                &mut turn_queue,
            ) {
                Ok(id) => {
                    info!("Successfully spawned player from definition at {:?}", spawn_cmd.position);
                    id
                }
                Err(e) => {
                    panic!("Failed to spawn player from definition: {}", e);
                }
            }
        } else {
            panic!("Entity definitions not available, using hardcoded player spawning");
        };

        info!("Spawned player {:?} at {:?}", player_id, spawn_cmd.position);

        // Remove the command entity
        commands.entity(entity).despawn();
    }

    // Process enemy spawn commands
    for (entity, spawn_cmd) in enemy_commands.iter() {
        let enemy_id = if let (Some(entity_definitions), Some(assets)) =
            (entity_definitions.as_ref(), assets.as_ref())
        {
            let spawn_result = match &spawn_cmd.enemy_name {
                Some(name) => {
                    // Spawn specific enemy by name
                    spawn_enemy_from_definition(
                        commands.reborrow(),
                        entity_definitions,
                        assets,
                        name,
                        spawn_cmd.position,
                        &mut current_map,
                        &mut turn_queue,
                    )
                }
                None => {
                    // Spawn random enemy
                    spawn_random_enemy_from_definition(
                        commands.reborrow(),
                        entity_definitions,
                        assets,
                        spawn_cmd.position,
                        &mut current_map,
                        &mut turn_queue,
                    )
                }
            };

            match spawn_result {
                Ok(id) => {
                    info!("Successfully spawned enemy from definition at {:?}", spawn_cmd.position);
                    id
                }
                Err(e) => {
                    panic!("Failed to spawn enemy from definition: {}", e);
                }
            }
        } else {
            panic!("Entity definitions not available, using hardcoded enemy spawning");
        };

        let enemy_type = spawn_cmd.enemy_name.as_deref().unwrap_or("random");
        info!("Spawned {} enemy {:?} at {:?}", enemy_type, enemy_id, spawn_cmd.position);

        // Remove the command entity
        commands.entity(entity).despawn();
    }
}

/// Convenience trait to add spawning commands to Commands
pub trait SpawnEntityCommands {
    /// Spawn a player at the given position
    fn spawn_player(&mut self, position: Position);

    /// Spawn a random enemy at the given position
    fn spawn_random_enemy(&mut self, position: Position);

    /// Spawn a specific enemy by name at the given position
    fn spawn_enemy(&mut self, enemy_name: &str, position: Position);
}

impl SpawnEntityCommands for Commands<'_, '_> {
    fn spawn_player(&mut self, position: Position) { self.spawn(SpawnPlayerCommand { position }); }

    fn spawn_random_enemy(&mut self, position: Position) {
        self.spawn(SpawnEnemyCommand { position, enemy_name: None });
    }

    fn spawn_enemy(&mut self, enemy_name: &str, position: Position) {
        self.spawn(SpawnEnemyCommand { position, enemy_name: Some(enemy_name.to_string()) });
    }
}
