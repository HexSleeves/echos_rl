use std::path::PathBuf;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use brtk::prelude::Folders;

#[derive(Serialize, Deserialize, Resource, Reflect, Clone)]
#[reflect(Resource)]
pub struct AppSettings {
    // Graphics
    /// Fullscreen mode
    pub fullscreen: bool,

    /// Display size of each tile
    pub tile_size: u32,

    /// View size in tiles
    pub view_size: (u32, u32),
    // Zoom amount of UI
    // pub interface_zoom: f32,
    // /// Show blood / slime / ect
    // pub decorations: bool,

    // /// Show animations on teleport / torches / ect
    // pub prop_animations: bool,

    // /// Show creature wiggle
    // pub creature_animation: bool,

    // /// Randomize creature wiggle start frame
    // pub creature_animation_synchronous: bool,

    // /// How fast the wiggle happens
    // pub creature_animation_speed: f32,

    // /// How far to wiggle
    // pub creature_animation_depth: u32,

    // /// How long to pause after wiggle
    // pub creature_animation_pause: f32,

    // /// Flash the creature on damage
    // pub creature_flash_damage: bool,

    // Font
    // pub font_path: Path,
    // pub font_height: u32,

    // Audio
    // pub sound_effects: bool,
    // pub master_volume: f32,
    // pub creature_volume: f32,
    // pub footsteps_volume: f32,
    // pub item_volume: f32,
    // pub combat_volume: f32,
    // pub ambient_volume: f32,
    // pub menu_music_volume: f32,
    // pub menu_click_volume: f32,

    // Mouse
    // Gamepad

    // Gameplay
    // Companions
    // Health Warning
    // Auto-explore
}

impl Default for AppSettings {
    fn default() -> Self { Self { tile_size: 16, fullscreen: false, view_size: (60, 40) } }
}

impl AppSettings {
    #[must_use]
    pub const fn fullscreen(&self) -> bool { self.fullscreen }

    #[must_use]
    pub const fn window_width(&self) -> f32 {
        crate::AppConstants::WINDOW_WIDTH
        // (self.tile_size * self.view_size.0 + UiConstants::STATS_WIDTH) as f32
    }

    #[must_use]
    pub const fn window_height(&self) -> f32 {
        crate::AppConstants::WINDOW_HEIGHT

        // (self.tile_size * self.view_size.1 + UiConstants::LOG_HEIGHT) as f32
    }
}

impl AppSettings {
    const FILE_NAME: &str = "app_settings";
    const PATH: &str = "settings";

    fn file_path() -> PathBuf {
        let mut file_path = PathBuf::from(Self::PATH);
        file_path.push(Self::FILE_NAME);
        file_path.set_extension("toml");
        file_path
    }

    /// Load the `AppSettings` from the default location
    pub fn load(folders: &Folders, local: bool) -> Self {
        let file_path = Self::file_path();

        if let Ok(settings) =
            if local { folders.read_base(&file_path) } else { folders.read_config(&file_path) }
        {
            toml::from_str(&settings).unwrap_or_default()
        } else {
            let settings = AppSettings::default();
            settings.save(folders, local);
            settings
        }
    }

    /// Save the `AppSettings` to the default location
    pub fn save(&self, folders: &Folders, local: bool) {
        let file_path = Self::file_path();

        match toml::ser::to_string_pretty(self) {
            Ok(contents) => {
                if let Err(e) = if local {
                    folders.write_base(file_path, contents)
                } else {
                    folders.write_config(file_path, contents)
                } {
                    warn!("Error saving AppSettings: {e}");
                }
            }
            Err(e) => warn!("Error saving AppSettings: {e}"),
        }
    }
}
