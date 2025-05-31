use bevy::{
    dev_tools::states::log_transitions, diagnostic::FrameTimeDiagnosticsPlugin,
    input::common_conditions::input_just_pressed, prelude::*, ui::UiDebugOptions,
};

#[cfg(feature = "debug")]
use bevy::input::common_conditions::input_toggle_active;

#[cfg(feature = "debug")]
pub use bevy_inspector_egui::{
    DefaultInspectorConfigPlugin, bevy_egui::EguiPlugin, bevy_inspector::hierarchy::SelectedEntities, egui,
    prelude::*, quick::*,
};

use crate::{core::states::GameState, rendering::screens::ScreenState};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(FrameTimeDiagnosticsPlugin::default());

    #[cfg(feature = "debug")]
    {
        app.add_plugins(EguiPlugin { enable_multipass_for_primary_context: true })
            .add_plugins(WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)));
    }

    app.add_systems(
        Update,
        (
            // Log `ScreenState` state transitions.
            log_transitions::<ScreenState>,
            // Log `GameState` state transitions.
            log_transitions::<GameState>,
            // Toggle the debug overlay for UI.
            toggle_debug_ui.run_if(input_just_pressed(TOGGLE_KEY)),
        ),
    );
}

const TOGGLE_KEY: KeyCode = KeyCode::Backquote;
fn toggle_debug_ui(mut options: ResMut<UiDebugOptions>) { options.toggle(); }
