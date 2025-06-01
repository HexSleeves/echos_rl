use bevy::prelude::*;

pub mod components;
pub mod events;
pub mod systems;

use crate::{core::states::GameState, rendering::screens::ScreenState};

/// Player plugin that handles all player-related functionality
pub fn plugin(app: &mut App) {
    // Add player events
    app.add_event::<events::PlayerMoved>().add_event::<events::PlayerDied>();

    // Add player systems with proper system set organization
    app.add_systems(
        Update,
        systems::player_input_system
            .run_if(in_state(GameState::GatherActions))
            .run_if(in_state(ScreenState::Gameplay))
            .in_set(super::GameplaySystemSet::ActionGathering),
    );
}
