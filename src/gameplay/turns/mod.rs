use bevy::prelude::*;

pub mod components;
pub mod systems;
mod turn_manager;
mod turn_processor;

use crate::{core::states::GameState, rendering::screens::ScreenState};

/// Turns plugin that handles the turn-based system
pub fn plugin(app: &mut App) {
    app.add_plugins(turn_processor::TurnSystemPlugin);

    // Add turn processing system with proper system set organization
    app.add_systems(
        Update,
        systems::process_turns
            .run_if(in_state(GameState::ProcessTurns))
            .run_if(in_state(ScreenState::Gameplay))
            .in_set(super::GameplaySystemSet::ActionProcessing),
    );
}
