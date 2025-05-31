use bevy::prelude::*;

pub mod player;
pub mod enemies;
pub mod turns;
pub mod world;

/// Integrates all gameplay-related plugins into the Bevy application.
///
/// Adds the player, enemies, turns, and world plugins to the provided app, enabling core gameplay features.
///
/// # Examples
///
/// ```
/// let mut app = App::new();
/// gameplay::plugin(&mut app);
/// ```
pub fn plugin(app: &mut App) {
    app.add_plugins((
        player::plugin,
        enemies::plugin,
        turns::plugin,
        world::plugin,
    ));
}
