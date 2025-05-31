use bevy::prelude::*;

pub mod components;
pub mod events;
pub mod resources;
pub mod states;
pub mod systems;

/// Core plugin that provides fundamental game systems and components
/// Configures the core plugin for the Bevy app, registering essential states, components, events, and systems used throughout the game.
///
/// This function initializes the main game state, registers core component types for reflection, adds fundamental game events, and sets up a cleanup system to run during the update stage. It is intended to be included in every game feature to ensure consistent core functionality.
///
/// # Examples
///
/// ```
/// let mut app = App::new();
/// core::plugin(&mut app);
/// ```
pub fn plugin(app: &mut App) {
    // Initialize core states
    app.init_state::<states::GameState>();
    
    // Register core components for reflection
    app.register_type::<components::Position>()
        .register_type::<components::Description>()
        .register_type::<components::ViewShed>();
    
    // Register core events
    app.add_event::<events::GameStarted>()
        .add_event::<events::GameEnded>();
    
    // Add core systems
    app.add_systems(Update, systems::cleanup_system::<systems::CleanupOnGameExit>);
}
