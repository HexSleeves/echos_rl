use bevy::prelude::*;

pub mod components;
pub mod systems;
pub mod events;

/// Configures the Bevy app with player-related components and events.
///
/// Registers the `PlayerTag` component and adds the `PlayerMoved` and `PlayerDied` events to the app. Intended as the entry point for integrating all player-specific logic into the game.
///
/// # Examples
///
/// ```
/// use bevy::prelude::*;
/// use my_game::gameplay::player;
///
/// let mut app = App::new();
/// player::plugin(&mut app);
/// ```
pub fn plugin(app: &mut App) {
    // Register player components
    app.register_type::<components::PlayerTag>();
    
    // Add player events
    app.add_event::<events::PlayerMoved>()
        .add_event::<events::PlayerDied>();
    
    // Player systems will be added here
    // app.add_systems(Update, systems::player_input_system);
}
