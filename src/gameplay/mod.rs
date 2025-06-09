use bevy::{platform::collections::HashSet, prelude::*};

pub mod enemies;
pub mod player;
pub mod turns;
pub mod world;

use crate::{core::states::GameState, debug_turns, rendering::screens::ScreenState, ui};

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

    let mut occupied: HashSet<Position> = HashSet::from([player_position]);
    if let Some(enemy_pos) = current_map.get_random_walkable_position().filter(|p| occupied.insert(*p)) {
        commands.spawn_ai("hostile_guard", enemy_pos);
    }
}

/// System to start the first turn after initialization
fn start_first_turn(mut next_state: ResMut<NextState<GameState>>) {
    debug_turns!("Starting first turn - transitioning to ProcessTurns state");
    next_state.set(GameState::ProcessTurns);
}
