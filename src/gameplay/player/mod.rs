use bevy::prelude::*;

pub mod components;
pub mod events;
pub mod systems;

use crate::core::states::GameState;

/// Player plugin that handles all player-related functionality
pub fn plugin(app: &mut App) {
    // Register player components
    app.register_type::<components::PlayerTag>();

    // Add player events
    app.add_event::<events::PlayerMoved>().add_event::<events::PlayerDied>();

    // Add player systems
    app.add_systems(Update, systems::player_input_system.run_if(in_state(GameState::GatherActions)));
}
