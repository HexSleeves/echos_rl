use bevy::prelude::*;

pub mod components;
pub mod constants;
pub mod resources;
pub mod screens;
pub mod systems;

pub use constants::RenderingConstants;

/// Rendering plugin that handles all visual presentation
pub fn plugin(app: &mut App) {
    app.add_plugins(screens::plugin);

    // Initialize rendering resources
    app.init_resource::<resources::TileMap>();

    // Register rendering components for reflection
    app.register_type::<components::TileSprite>();

    // Add rendering systems
    app.add_systems(Update, systems::camera_movement);
}
