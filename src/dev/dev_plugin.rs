use bevy::{
    dev_tools::states::log_transitions,
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    input::common_conditions::input_just_pressed,
    prelude::*,
    ui::UiDebugOptions,
};

use crate::screens::ScreenState;

pub struct DevPlugin;
impl Plugin for DevPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((FrameTimeDiagnosticsPlugin::default(), LogDiagnosticsPlugin::default()));

        // Log `ScreenState` state transitions.
        app.add_systems(Update, log_transitions::<ScreenState>);

        // Toggle the debug overlay for UI.
        app.add_systems(Update, toggle_debug_ui.run_if(input_just_pressed(TOGGLE_KEY)));
    }
}

const TOGGLE_KEY: KeyCode = KeyCode::Backquote;
fn toggle_debug_ui(mut options: ResMut<UiDebugOptions>) { options.toggle(); }
