use bevy::prelude::*;

pub mod components;
pub mod systems;
pub mod events;

/// Player plugin that handles all player-related functionality
pub fn plugin(app: &mut App) {
    // Register player components
    app.register_type::<components::PlayerTag>();
    
    // Add player events
    app.add_event::<events::PlayerMoved>()
        .add_event::<events::PlayerDied>();
    
    // Player systems will be added here
    // app.add_systems(Update, systems::player_input_system);
}
