use bevy::prelude::*;

pub mod components;
pub mod systems;
pub mod resources;

/// Registers the turn-based system plugin with the Bevy app.
///
/// Intended as the entry point for integrating turn system logic into the application. Currently serves as a placeholder for future turn system setup.
///
/// # Examples
///
/// ```
/// use bevy::prelude::*;
/// use my_game::gameplay::turns::plugin;
///
/// let mut app = App::new();
/// plugin(&mut app);
/// ```
pub fn plugin(app: &mut App) {
    // Turn system will be migrated here
}
