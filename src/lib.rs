// // Support configuring Bevy lints within code.
// #![cfg_attr(bevy_lint, feature(register_tool), register_tool(bevy))]

// pub struct GamePlugin;
// impl Plugin for GamePlugin {
//     fn build(&self, app: &mut App) {
//         // Order new `AppSystems` variants by adding them here:
//         app.configure_sets(Update, (AppSystems::TickTimers, AppSystems::RecordInput, AppSystems::Update).chain());

//         // Spawn the main camera.
//         app.add_systems(Startup, spawn_camera);

//         // Add other plugins.
//         app.add_plugins((
//             #[cfg(feature = "dev")]
//             dev_tools::plugin,
//             screens::plugin,
//             theme::plugin,
//             gameplay::plugin,
//         ));
//     }
// }
