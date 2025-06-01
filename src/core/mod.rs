use bevy::prelude::*;

pub mod actions;
pub mod commands;
pub mod components;
pub mod constants;
pub mod events;
pub mod resources;
pub mod states;
pub mod systems;
pub mod types;

use crate::rendering::screens::ScreenState;

/// Core plugin that provides fundamental game systems and components
/// used across all game features.
pub fn plugin(app: &mut App) {
    // Initialize core states
    app.init_state::<states::GameState>();

    // Initialize core resources
    app.init_resource::<resources::CurrentMap>()
        .init_resource::<resources::TurnQueue>()
        .init_resource::<resources::FovMap>()
        .init_resource::<resources::SpawnPoint>();

    // Register core components for reflection
    app.register_type::<components::Position>()
        .register_type::<components::Description>()
        .register_type::<components::ViewShed>()
        .register_type::<components::PlayerTag>()
        .register_type::<components::AITag>()
        .register_type::<components::DeadTag>();

    // Register core events
    app.add_event::<events::GameStarted>().add_event::<events::GameEnded>();

    // Add core systems organized by function
    app.add_systems(
        Update,
        (
            // Process spawn commands during gameplay - this should run early
            commands::process_spawn_commands
                .run_if(in_state(ScreenState::Gameplay))
                .in_set(crate::gameplay::GameplaySystemSet::Spawning),
            // Always-running systems
            systems::cleanup_system::<systems::CleanupOnGameExit>,
            systems::compute_fov
                .run_if(in_state(ScreenState::Gameplay))
                .in_set(crate::gameplay::GameplaySystemSet::WorldUpdate),
            systems::toggle_fov_algorithm,
        ),
    );
}
