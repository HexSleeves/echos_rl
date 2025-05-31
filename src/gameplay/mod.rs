use bevy::prelude::*;

pub mod enemies;
pub mod player;
pub mod turns;
pub mod world;

/// Gameplay plugin that coordinates all gameplay-related features
pub fn plugin(app: &mut App) {
    app.add_plugins((
        player::plugin,
        enemies::plugin, // Now includes AI systems
        turns::plugin,
        world::plugin,
    ));
}
