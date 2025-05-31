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

/// Core plugin that provides fundamental game systems and components
/// used across all game features.
pub fn plugin(app: &mut App) {
    // Initialize core states
    app.init_state::<states::GameState>();
    
    // Register core components for reflection
    app.register_type::<components::Position>()
        .register_type::<components::Description>()
        .register_type::<components::ViewShed>()
        .register_type::<components::PlayerTag>()
        .register_type::<components::AITag>()
        .register_type::<components::DeadTag>();
    
    // Register core events
    app.add_event::<events::GameStarted>()
        .add_event::<events::GameEnded>();
    
    // Add core systems
    app.add_systems(Update, (
        systems::cleanup_system::<systems::CleanupOnGameExit>,
        systems::compute_fov,
        systems::toggle_fov_algorithm,
    ));
}
