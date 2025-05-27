use bevy::prelude::*;

use crate::model::{
    commands::SpawnEntityCommands,
    components::Position,
    resources::{CurrentMap, SpawnPoint},
};

pub fn spawn_player(
    mut commands: Commands,
    current_map: Res<CurrentMap>,
    spawn_point: Option<Res<SpawnPoint>>,
) {
    // Determine where to spawn the player
    let player_position = spawn_point
        .and_then(|sp| sp.player_spawn)
        .or_else(|| current_map.get_random_walkable_position().map(|pos| pos.into()))
        .unwrap_or_else(|| {
            warn!("No valid spawn point found, using default position");
            Position::new(0, 0)
        });

    // Use the command-based spawning
    commands.spawn_player(player_position);

    info!("Queued player spawn at {:?}", player_position);
}

pub fn spawn_enemies(mut commands: Commands, current_map: Res<CurrentMap>) {
    // Find a valid position for enemy spawning
    let enemy_position =
        current_map.get_random_walkable_position().map(|pos| pos.into()).unwrap_or_else(|| {
            warn!("No valid enemy spawn point found, using default position");
            Position::new(1, 1)
        });

    // Use the command-based spawning for a random enemy
    commands.spawn_random_enemy(enemy_position);

    info!("Queued enemy spawn at {:?}", enemy_position);
}
