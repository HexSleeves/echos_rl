use bevy::prelude::*;

/// High-level groupings of systems for the app in the `Update` schedule.
/// When adding a new variant, make sure to order it in the `configure_sets`
/// call above.
#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum AppSystems {
    /// Tick timers.
    TickTimers,
    /// Record player input.
    RecordInput,
    /// Do everything else (consider splitting this into further variants).
    Update,
}

pub const fn get_log_level() -> bevy::log::Level {
    if cfg!(feature = "trace") {
        bevy::log::Level::TRACE
    } else if cfg!(feature = "debug") {
        bevy::log::Level::DEBUG
    } else if cfg!(feature = "dev") {
        bevy::log::Level::INFO
    } else {
        bevy::log::Level::ERROR
    }
}

pub fn log_plugin() -> bevy::log::LogPlugin {
    bevy::log::LogPlugin {
        level: get_log_level(),
        // bevy_app=debug,big_brain=debug
        filter: "wgpu_hal=warn,gfx_backend_metal=warn,wgpu_core=warn,bevy_render=info,lain=debug,\
               bevy_render::render_resource::pipeline_cache=warn,\
               sequence=debug,naga=warn"
            .to_string(),
        ..Default::default()
    }
}
