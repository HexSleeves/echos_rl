use bevy::{
    dev_tools::states::log_transitions, diagnostic::FrameTimeDiagnosticsPlugin,
    input::common_conditions::input_just_pressed, prelude::*, ui::UiDebugOptions,
};

#[cfg(feature = "dev_log")]
use bevy::diagnostic::LogDiagnosticsPlugin;

use crate::{model::GameState, view::screens::ScreenState};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(FrameTimeDiagnosticsPlugin::default());

    #[cfg(feature = "dev_log")]
    {
        app.add_plugins(LogDiagnosticsPlugin::default());
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
