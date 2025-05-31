use bevy::prelude::*;

pub mod ai;
pub mod components;
pub mod enhanced_spawning;
pub mod pathfinding;
pub mod pathfinding_demo;
pub mod systems;

/// Enemies plugin that handles all enemy/AI functionality
pub fn plugin(app: &mut App) {
    // Initialize pathfinding configuration
    app.init_resource::<enhanced_spawning::AIPathfindingConfig>();

    // Register AI components for reflection
    app.register_type::<components::AIBehavior>()
        .register_type::<components::AIState>()
        .register_type::<pathfinding::AIPathfinding>();

    // Add pathfinding upgrade system
    app.add_systems(
        Update,
        (
            enhanced_spawning::upgrade_ai_entities_with_pathfinding,
            enhanced_spawning::apply_pathfinding_config,
        ),
    );

    // Add AI plugin
    app.add_plugins(ai::plugin);
}
