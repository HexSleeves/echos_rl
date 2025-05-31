use bevy::prelude::*;

// This file will contain shared resources used across the game
// For now, it's empty but ready for future additions

/// Example shared resource for game-wide configuration
#[derive(Resource, Debug)]
pub struct GameConfig {
    pub debug_mode: bool,
    pub show_fps: bool,
}

impl Default for GameConfig {
    /// Returns a `GameConfig` instance with both `debug_mode` and `show_fps` set to `false`.
    ///
    /// # Examples
    ///
    /// ```
    /// let config = GameConfig::default();
    /// assert!(!config.debug_mode);
    /// assert!(!config.show_fps);
    /// ```
    fn default() -> Self {
        Self {
            debug_mode: false,
            show_fps: false,
        }
    }
}
