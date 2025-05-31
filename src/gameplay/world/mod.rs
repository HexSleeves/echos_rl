use bevy::prelude::*;

pub mod components;
pub mod generation;
pub mod spawning;
pub mod systems;

/// World plugin that handles map generation and world management
pub fn plugin(app: &mut App) {
    // Register world components
    app.register_type::<components::TerrainType>()
        .register_type::<components::UndergroundType>();

    // Add world systems
    // Note: spawn systems are typically called from state transitions, not Update
    // They will be registered in the appropriate state handlers
}
