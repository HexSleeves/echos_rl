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

// New module structure
pub mod core;
pub mod gameplay;
pub mod prelude;
pub mod rendering;

// Existing modules (to be migrated)
// pub mod controller; // Deleted - migrated to gameplay modules
#[cfg(feature = "dev")]
pub mod dev;
// pub mod model; // Deleted - migrated to core and gameplay modules
pub mod ui;
pub mod utils;
// pub mod view; // Deleted - migrated to rendering

mod app;
pub use self::app::*;
mod app_constants;
pub use self::app_constants::*;
mod app_settings;
pub use self::app_settings::*;

pub struct EchosInTheDark {
    app: App,
    brt_plugin: BrtkPlugin,
    app_settings: AppSettings,
}

impl Default for EchosInTheDark {
    fn default() -> Self { Self::new() }
}

impl EchosInTheDark {
    pub fn new() -> Self {
        let app = App::new();

        let brt_plugin = BrtkPlugin::new(
            AppConstants::BASE,
            AppConstants::DOMAIN,
            AppConstants::COMPANY,
            AppConstants::APP_NAME,
        )
        .with_icon(include_bytes!("../build/macos/AppIcon.iconset/icon_256x256.png"));

        // Load AppSettings
        let app_settings = AppSettings::load(brt_plugin.folders(), true);

        Self { app, app_settings, brt_plugin }
    }

    fn configure_sets(&mut self) -> &mut Self {
        self.app.configure_sets(
            Update,
            (AppSystems::TickTimers, AppSystems::RecordInput, AppSystems::Update).chain(),
        );

        self
    }

    fn default_plugins(&mut self) -> &mut Self {
        let defaults = DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: AppConstants::APP_NAME.to_string(),
                    resolution: WindowResolution::new(
                        self.app_settings.window_width(),
                        self.app_settings.window_height(),
                    ),
                    mode: if self.app_settings.fullscreen() {
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
            .set(ImagePlugin::default_nearest())
            .set(app::log_plugin())
            .build();

        self.app.add_plugins(defaults);

        // #[cfg(feature = "release")]
        // {
        //     defaults.add_before::<bevy::asset::AssetPlugin,
        // _>(bevy_embedded_assets::EmbeddedAssetPlugin); }

        self
    }

    fn app_plugins(&mut self) -> &mut Self {
        // Assign plugins
        self.app.add_plugins((
            self.brt_plugin.clone(),
            echos_assets::EchosAssetsPlugin,
            core::plugin,      // New core plugin
            gameplay::plugin,  // New gameplay plugin
            rendering::plugin, // New rendering plugin
            ui::plugin,
            #[cfg(feature = "dev")]
            dev::plugin,
        ));

        self
    }

    fn run(&mut self) {
        self.app.insert_resource(self.app_settings.clone()).insert_resource(ClearColor(Color::BLACK)).run();
    }
}

fn main() { EchosInTheDark::new().default_plugins().app_plugins().configure_sets().run(); }
