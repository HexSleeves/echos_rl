use bevy::prelude::*;

pub mod components;
pub mod resources;
pub mod screens;
pub mod systems;

mod view_constants;
pub use self::view_constants::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((screens::plugin,));

    // Register the TileSprite component for reflection
    app.register_type::<crate::view::components::TileSprite>();
}
