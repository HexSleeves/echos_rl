use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};

use echos_in_the_dark::debug;

pub use crate::debug::file_logger::{generate_bug_report, log_to_file};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((FrameTimeDiagnosticsPlugin::default(), debug::DebugPlugin));
}
