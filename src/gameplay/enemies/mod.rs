use bevy::prelude::*;

pub mod components;
pub mod systems;
pub mod ai;

/// Enemies plugin that handles all enemy/AI functionality
pub fn plugin(app: &mut App) {
    // Enemy systems will be migrated here
    // app.add_plugins(ai::plugin);
}
