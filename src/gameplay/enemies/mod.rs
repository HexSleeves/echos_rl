use bevy::prelude::*;

pub mod components;
pub mod systems;
pub mod ai;

/// Registers enemy and AI-related systems and plugins to the application.
///
/// Intended to integrate all enemy and AI functionality into the main game app. Currently serves as a placeholder for future system registration.
///
/// # Examples
///
/// ```
/// let mut app = App::new();
/// gameplay::enemies::plugin(&mut app);
/// ```
pub fn plugin(app: &mut App) {
    // Enemy systems will be migrated here
    // app.add_plugins(ai::plugin);
}
