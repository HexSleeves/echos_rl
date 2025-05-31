use bevy::prelude::*;
use echos_assets::entities::{EntityDefinition, EntityDefinitions};

use crate::{
    core::{components::Position, resources::{CurrentMap, TurnQueue}},
    gameplay::world::spawning::{spawn_ai_from_definition, spawn_player_from_definition, spawn_random_ai_from_definition},
};

/// Entity command for spawning a player
#[derive(Component)]
pub struct SpawnPlayerCommand {
    pub position: Position,
}

/// Entity command for spawning an enemy
#[derive(Component)]
pub struct SpawnAICommand {
    pub position: Position,
    pub ai_name: Option<String>, // None for random enemy
}

/// System to process spawn commands
pub fn process_spawn_commands(
    mut commands: Commands,
    mut current_map: ResMut<CurrentMap>,
    mut turn_queue: ResMut<TurnQueue>,
    entity_definitions: Option<Res<EntityDefinitions>>,
    assets: Option<Res<Assets<EntityDefinition>>>,
    player_commands: Query<(Entity, &SpawnPlayerCommand)>,
    ai_commands: Query<(Entity, &SpawnAICommand)>,
) {
    // Process player spawn commands
    for (entity, spawn_cmd) in player_commands.iter() {
        if let (Some(entity_definitions), Some(assets)) = (entity_definitions.as_ref(), assets.as_ref()) {
            match spawn_player_from_definition(
                commands.reborrow(),
                entity_definitions,
                assets,
                spawn_cmd.position,
                &mut current_map,
                &mut turn_queue,
            ) {
                Ok(player_id) => {
                    info!(
                        "Successfully spawned player from definition at {:?} with ID {:?}",
                        spawn_cmd.position, player_id
                    );
                }
                Err(e) => {
                    warn!("Failed to spawn player from definition: {}. Skipping player spawn.", e);
                }
            }
        } else {
            warn!("Entity definitions not available, cannot spawn player");
        }

        // Remove the command entity regardless of success/failure
        commands.entity(entity).despawn();
    }

    // Process enemy spawn commands
    for (entity, spawn_cmd) in ai_commands.iter() {
        if let (Some(entity_definitions), Some(assets)) = (entity_definitions.as_ref(), assets.as_ref()) {
            let spawn_result: Result<Entity, String> = match &spawn_cmd.ai_name {
                Some(name) => {
                    // Spawn specific enemy by name
                    spawn_ai_from_definition(
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
                    spawn_random_ai_from_definition(
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
                Ok(enemy_id) => {
                    let enemy_type = spawn_cmd.ai_name.as_deref().unwrap_or("random");
                    info!(
                        "Successfully spawned {} enemy from definition at {:?} with ID {:?}",
                        enemy_type, spawn_cmd.position, enemy_id
                    );
                }
                Err(e) => {
                    let enemy_type = spawn_cmd.ai_name.as_deref().unwrap_or("random");
                    warn!(
                        "Failed to spawn {} enemy from definition: {}. Skipping enemy spawn.",
                        enemy_type, e
                    );
                }
            }
        } else {
            warn!("Entity definitions not available, cannot spawn enemy");
        }

        // Remove the command entity regardless of success/failure
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
    fn spawn_ai(&mut self, ai_name: &str, position: Position);
}

impl SpawnEntityCommands for Commands<'_, '_> {
    fn spawn_player(&mut self, position: Position) { self.spawn(SpawnPlayerCommand { position }); }

    fn spawn_random_enemy(&mut self, position: Position) {
        self.spawn(SpawnAICommand { position, ai_name: None });
    }

    fn spawn_ai(&mut self, ai_name: &str, position: Position) {
        self.spawn(SpawnAICommand { position, ai_name: Some(ai_name.to_string()) });
    }
}
