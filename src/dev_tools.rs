//! Development tools for the game. This plugin is only enabled in dev builds.

use bevy::{
    dev_tools::states::log_transitions, input::common_conditions::input_just_pressed, prelude::*, ui::UiDebugOptions,
};

use crate::screens::ScreenState;

#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

pub(super) fn plugin(app: &mut App) {
    // Log `ScreenState` state transitions.
    app.add_systems(Update, log_transitions::<ScreenState>);

    // Toggle the debug overlay for UI.
    app.add_systems(Update, toggle_debug_ui.run_if(input_just_pressed(TOGGLE_KEY)));

    #[cfg(debug_assertions)]
    {
        app.add_plugins((FrameTimeDiagnosticsPlugin::default(), LogDiagnosticsPlugin::default()));
    }
}

const TOGGLE_KEY: KeyCode = KeyCode::Backquote;

fn toggle_debug_ui(mut options: ResMut<UiDebugOptions>) { options.toggle(); }
