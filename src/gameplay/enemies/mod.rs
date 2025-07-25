use bevy::prelude::*;

pub mod ai;
pub mod components;
pub mod systems;

mod helpers;

/// Enemies plugin that handles all enemy/AI functionality
pub fn plugin(app: &mut App) {
    // Register AI components for reflection
    app.register_type::<components::AIBehavior>();

    // Add AI plugin
    app.add_plugins(ai::plugin);
}
