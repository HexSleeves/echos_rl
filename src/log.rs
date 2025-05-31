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
