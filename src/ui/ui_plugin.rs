use bevy::prelude::*;

use crate::ui::{components::GameCamera, systems::spawn_camera};

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<GameCamera>();

        app.add_systems(Startup, spawn_camera);
    }
}
