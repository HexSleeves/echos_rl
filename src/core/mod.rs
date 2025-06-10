use bevy::prelude::*;

pub mod actions;
pub mod bundles;
pub mod commands;
pub mod components;
pub mod constants;
pub mod events;
pub mod pathfinding;
pub mod resources;
pub mod states;
pub mod systems;
pub mod types;

use crate::{core::states::GameState, rendering::screens::ScreenState};

/// Core plugin that provides fundamental game systems and components
/// used across all game features.
pub fn plugin(app: &mut App) {
    // Initialize core states
    app.init_state::<states::GameState>();

    // Initialize core resources
    app.init_resource::<resources::CurrentMap>()
        .init_resource::<resources::TurnQueue>()
        .init_resource::<resources::FovMap>()
        .init_resource::<resources::SpawnPoint>()
        .init_resource::<resources::DistanceSettings>()
        .init_resource::<resources::LightMap>();

    // Register core components for reflection
    app.register_type::<components::Position>()
        .register_type::<components::Description>()
        .register_type::<components::FieldOfView>()
        .register_type::<components::PlayerTag>()
        .register_type::<components::AITag>()
        .register_type::<components::DeadTag>()
        .register_type::<components::Light>()
        .register_type::<resources::DistanceSettings>();

    // Register core events
    app.add_event::<events::GameStarted>()
        .add_event::<events::GameEnded>()
        .add_event::<events::CombatEvent>()
        .add_event::<events::DamageDealtEvent>()
        .add_event::<events::EntityDeathEvent>();

    app.add_systems(
        PreUpdate,
        (
            // Process spawn commands during gameplay
            commands::process_spawn_commands
                .run_if(in_state(ScreenState::Gameplay))
                .in_set(crate::gameplay::GameplaySystemSet::Spawning),
        ),
    );

    // Add fov systems
    app.add_systems(
        Update,
        (systems::fov::compute_fov, systems::light::calculate_light_map)
            .run_if(in_state(ScreenState::Gameplay))
            .run_if(in_state(GameState::ProcessTurns))
            .in_set(crate::gameplay::GameplaySystemSet::WorldUpdate),
    );

    // Add combat systems
    app.add_systems(
        Update,
        (
            systems::combat::handle_entity_death,
            systems::combat::handle_combat_events,
            systems::combat::handle_damage_events,
        )
            .run_if(in_state(ScreenState::Gameplay))
            .in_set(crate::gameplay::GameplaySystemSet::ActionProcessing),
    );

    // Add cleanup system
    app.add_systems(
        Update,
        systems::cleanup_system::<systems::CleanupOnGameExit>.run_if(in_state(ScreenState::Gameplay)),
    );
}
