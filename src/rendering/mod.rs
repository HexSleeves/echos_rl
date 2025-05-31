use bevy::prelude::*;

pub mod components;
pub mod systems;
pub mod resources;
pub mod screens;

/// Rendering plugin that handles all visual presentation
pub fn plugin(app: &mut App) {
    app.add_plugins(screens::plugin);
    
    // Register rendering components
    app.register_type::<components::TileSprite>()
        .register_type::<components::ViewShed>();
}
