// Disable console on Windows for non-dev builds.
#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]
// Support configuring Bevy lints within code.
#![cfg_attr(bevy_lint, feature(register_tool), register_tool(bevy))]

use bevy::{
    asset::AssetMetaCheck,
    prelude::*,
    window::{WindowMode, WindowResolution},
};
use brtk::prelude::BrtkPlugin;

pub mod assets;
pub mod controller;
#[cfg(feature = "dev")]
pub mod dev;
pub mod model;
pub mod ui;
pub mod utils;
pub mod view;

mod app_constants;
pub use self::app_constants::*;
mod app_settings;
pub use self::app_settings::*;

/// High-level groupings of systems for the app in the `Update` schedule.
/// When adding a new variant, make sure to order it in the `configure_sets`
/// call above.
#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
enum AppSystems {
    /// Tick timers.
    TickTimers,
    /// Record player input.
    RecordInput,
    /// Do everything else (consider splitting this into further variants).
    Update,
}

fn main() {
    let mut app = App::new();

    let brt_plugin = BrtkPlugin::new(
        AppConstants::BASE,
        AppConstants::DOMAIN,
        AppConstants::COMPANY,
        AppConstants::APP_NAME,
    )
    .with_icon(include_bytes!("../build/macos/AppIcon.iconset/icon_256x256.png"));

    // Load AppSettings
    let app_settings = AppSettings::load(brt_plugin.folders(), true);

    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: AppConstants::APP_NAME.to_string(),
                    resolution: WindowResolution::new(
                        app_settings.window_width(),
                        app_settings.window_height(),
                    ),
                    mode: if app_settings.fullscreen() {
                        WindowMode::BorderlessFullscreen(MonitorSelection::Current)
                    } else {
                        WindowMode::Windowed
                    },
                    // Bind to canvas included in `index.html`
                    canvas: Some("#echos_in_the_dark".to_owned()),
                    fit_canvas_to_parent: true,
                    // Tells wasm not to override default event handling, like F5 and Ctrl+R
                    prevent_default_event_handling: false,
                    ..Default::default()
                }),
                ..Default::default()
            })
            .set(AssetPlugin {
                file_path: AppConstants::BASE.to_string(),
                // Wasm builds will check for meta files (that don't exist) if this isn't set.
                // This causes errors and even panics on web build on itch.
                // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
                meta_check: AssetMetaCheck::Never,
                ..Default::default()
            })
            .set(ImagePlugin::default_nearest()),
    );

    app
        // Order new `AppSystems` variants by adding them here:
        .configure_sets(Update, (AppSystems::TickTimers, AppSystems::RecordInput, AppSystems::Update).chain())
        // Insert resources
        .insert_resource(app_settings);

    #[cfg(feature = "dev")]
    app.add_plugins(dev::plugin);

    // Assign plugins
    app.add_plugins((
        brt_plugin,
        assets::plugin,
        controller::plugin,
        model::plugin,
        ui::plugin,
        view::plugin,
    ));

    match app.run() {
        AppExit::Success => std::process::exit(0),
        AppExit::Error(code) => std::process::exit(code.get() as i32),
    }
}
