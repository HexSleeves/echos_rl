// Support configuring Bevy lints within code.
#![cfg_attr(bevy_lint, feature(register_tool), register_tool(bevy))]

#[cfg(feature = "dev")]
mod dev_tools;
mod gameplay;
mod screens;
mod theme;

use bevy::prelude::*;

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

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        // Order new `AppSystems` variants by adding them here:
        app.configure_sets(Update, (AppSystems::TickTimers, AppSystems::RecordInput, AppSystems::Update).chain());

        // Spawn the main camera.
        app.add_systems(Startup, spawn_camera);

        // Add other plugins.
        app.add_plugins((
            #[cfg(feature = "dev")]
            dev_tools::plugin,
            screens::plugin,
            theme::plugin,
            gameplay::plugin,
        ));
    }
}

fn spawn_camera(mut commands: Commands) {
    println!("Spawning camera");
    commands.spawn((Name::new("Main Camera"), Camera2d));
}
