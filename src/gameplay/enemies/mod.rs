use bevy::prelude::*;

pub mod components;
pub mod systems;
pub mod ai;

/// Enemies plugin that handles all enemy/AI functionality
pub fn plugin(app: &mut App) {
    // Register AI components for reflection
    app.register_type::<components::AIBehavior>()
        .register_type::<components::AIState>();

    // Add AI plugin
    app.add_plugins(ai::plugin);
}
