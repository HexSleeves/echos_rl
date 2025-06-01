use bevy::prelude::*;

pub mod enemies;
pub mod player;
pub mod turns;
pub mod world;

use crate::{
    core::{
        components::{AITag, PlayerTag},
        states::GameState,
    },
    rendering::screens::ScreenState,
    ui,
};

/// System sets for organizing gameplay systems
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameplaySystemSet {
    /// Initialize world and entities
    Initialization,
    /// Process spawn commands
    Spawning,
    /// Gather actions from entities
    ActionGathering,
    /// Process queued actions
    ActionProcessing,
    /// Update world state
    WorldUpdate,
}

/// Gameplay plugin that coordinates all gameplay-related features
pub fn plugin(app: &mut App) {
    // Configure system set ordering
    app.configure_sets(
        Update,
        (
            GameplaySystemSet::Spawning,
            GameplaySystemSet::ActionGathering,
            GameplaySystemSet::ActionProcessing,
            GameplaySystemSet::WorldUpdate,
        )
            .chain()
            .run_if(in_state(ScreenState::Gameplay)),
    );

    // Add initialization systems
    app.add_systems(
        OnEnter(ScreenState::Gameplay),
        (ui::systems::spawn_camera, world::systems::spawn_map, spawn_initial_entities, start_first_turn)
            .chain()
            .in_set(GameplaySystemSet::Initialization),
    );

    // Add monitoring systems for debugging
    app.add_systems(
        Update,
        monitor_gameplay_state.run_if(in_state(ScreenState::Gameplay)).in_set(GameplaySystemSet::WorldUpdate),
    );

    // Add gameplay plugins
    app.add_plugins((player::plugin, enemies::plugin, turns::plugin, world::plugin));
}

/// System to spawn initial entities when gameplay starts
fn spawn_initial_entities(
    mut commands: Commands,
    current_map: Res<crate::core::resources::CurrentMap>,
    spawn_point: Option<Res<crate::core::resources::SpawnPoint>>,
) {
    use crate::core::{commands::SpawnEntityCommands, components::Position};

    // Spawn player
    let player_position = spawn_point
        .and_then(|sp| sp.player_spawn)
        .or_else(|| current_map.get_random_walkable_position())
        .unwrap_or_else(|| {
            warn!("No valid spawn point found for player, using default position");
            Position::new(0, 0)
        });

    commands.spawn_player(player_position);
    info!("Queued player spawn at {:?}", player_position);

    // Spawn some initial enemies
    for _ in 0..3 {
        if let Some(enemy_position) = current_map.get_random_walkable_position() {
            commands.spawn_random_enemy(enemy_position);
            info!("Queued enemy spawn at {:?}", enemy_position);
        }
    }
}

/// System to start the first turn after initialization
fn start_first_turn(mut next_state: ResMut<NextState<GameState>>) {
    info!("Starting first turn - transitioning to ProcessTurns state");
    next_state.set(GameState::ProcessTurns);
}

/// System to monitor gameplay state for debugging
fn monitor_gameplay_state(
    game_state: Res<State<GameState>>,
    players: Query<Entity, With<PlayerTag>>,
    enemies: Query<Entity, With<AITag>>,
) {
    let player_count = players.iter().count();
    let enemy_count = enemies.iter().count();

    // Only log when we have entities to avoid spam
    if player_count > 0 || enemy_count > 0 {
        debug!(
            "Gameplay State: {:?} | Players: {} | Enemies: {}",
            game_state.get(),
            player_count,
            enemy_count
        );
    }
}
