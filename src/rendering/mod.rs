use bevy::prelude::*;

pub mod components;
pub mod systems;
pub mod resources;
pub mod screens;

/// Integrates rendering functionality into the Bevy application.
///
/// Adds the rendering screens plugin and registers core rendering components for reflection and serialization support.
///
/// # Examples
///
/// ```
/// let mut app = App::new();
/// rendering::plugin(&mut app);
/// ```
pub fn plugin(app: &mut App) {
    app.add_plugins(screens::plugin);
    
    // Register rendering components
    app.register_type::<components::TileSprite>()
        .register_type::<components::ViewShed>();
}
