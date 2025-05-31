use bevy::prelude::*;

pub mod components;
pub mod systems;
pub mod generation;

/// Registers world management and map generation systems with the Bevy app.
///
/// Intended as the entry point for integrating world-related functionality into the application.
/// Currently serves as a placeholder for future system registration.
///
/// # Examples
///
/// ```
/// use bevy::prelude::*;
/// use gameplay::world::plugin;
///
/// let mut app = App::new();
/// plugin(&mut app);
/// ```
pub fn plugin(app: &mut App) {
    // World systems will be migrated here
}
