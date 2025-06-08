pub mod categories;
pub mod config;
pub mod file_logger;
pub mod macros;
#[cfg(feature = "debug")]
pub mod systems;

pub use categories::DebugCategory;
pub use config::DebugConfig;
pub use file_logger::{FileLogger, generate_bug_report, log_to_file};

use bevy::prelude::*;

#[cfg(feature = "debug")]
use crate::prelude::{GameState, ScreenState};
#[cfg(feature = "debug")]
use bevy::input::common_conditions::input_just_pressed;
#[cfg(feature = "debug")]
use bevy_inspector_egui::{
    DefaultInspectorConfigPlugin,
    bevy_egui::{EguiContextPass, EguiPlugin},
};

/// Debug plugin that sets up the debug system
pub struct DebugPlugin;
impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        // Initialize debug config
        app.init_resource::<DebugConfig>();

        #[cfg(feature = "debug")]
        app.add_plugins((
            EguiPlugin { enable_multipass_for_primary_context: true },
            DefaultInspectorConfigPlugin,
        ))
        .add_systems(EguiContextPass, systems::unified_debug_ui_system)
        .add_systems(
            Update,
            (
                // Log `ScreenState` state transitions.
                systems::log_transitions::<ScreenState>,
                // Log `GameState` state transitions.
                systems::log_transitions::<GameState>,
                // Toggle the debug overlay for UI.
                toggle_debug_ui.run_if(input_just_pressed(KeyCode::Backquote)),
            ),
        );
    }
}

#[cfg(feature = "debug")]
fn toggle_debug_ui(mut options: ResMut<UiDebugOptions>) { options.toggle(); }

/// Check if a debug category is enabled
pub fn is_category_enabled(category: DebugCategory) -> bool {
    // Try to get the config from the world, fallback to default behavior
    // This is a simplified check - in practice you'd want to access the actual resource
    match category {
        DebugCategory::AI => std::env::var("DEBUG_AI").is_ok(),
        DebugCategory::Turns => std::env::var("DEBUG_TURNS").is_ok(),
        DebugCategory::Combat => std::env::var("DEBUG_COMBAT").is_ok(),
        DebugCategory::World => std::env::var("DEBUG_WORLD").is_ok(),
        DebugCategory::Input => std::env::var("DEBUG_INPUT").is_ok(),
        DebugCategory::Rendering => std::env::var("DEBUG_RENDERING").is_ok(),
        DebugCategory::Performance => std::env::var("DEBUG_PERFORMANCE").is_ok(),
        DebugCategory::General => std::env::var("DEBUG_GENERAL").is_ok(),
        DebugCategory::StateTransitions => std::env::var("DEBUG_STATE_TRANSITIONS").is_ok(),
    }
}
