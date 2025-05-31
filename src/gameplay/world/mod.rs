use bevy::prelude::*;

pub mod components;
pub mod systems;
pub mod generation;

/// World plugin that handles map generation and world management
pub fn plugin(app: &mut App) {
    // Register world components
    app.register_type::<components::TerrainType>()
        .register_type::<components::UndergroundType>();

    // World systems will be migrated here
}
