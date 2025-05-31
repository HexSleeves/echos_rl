use bevy::prelude::*;

pub mod components;
pub mod resources;
pub mod systems;

use crate::core::states::GameState;

/// Turns plugin that handles the turn-based system
pub fn plugin(app: &mut App) {
    // Initialize turn queue resource
    app.init_resource::<resources::TurnQueue>();

    // Add turn processing system
    app.add_systems(Update, systems::process_turns.run_if(in_state(GameState::ProcessTurns)));
}
