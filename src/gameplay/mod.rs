use bevy::prelude::*;

pub mod player;
pub mod enemies;
pub mod turns;
pub mod world;

/// Gameplay plugin that coordinates all gameplay-related features
pub fn plugin(app: &mut App) {
    app.add_plugins((
        player::plugin,
        enemies::plugin,
        turns::plugin,
        world::plugin,
    ));
}
