use bevy::prelude::*;

use crate::ui::{
    components::{GameCamera, InteractionPalette},
    systems::{apply_interaction_palette, spawn_camera},
};

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<GameCamera>();
        app.register_type::<InteractionPalette>();

        app.add_systems(Startup, spawn_camera).add_systems(Update, apply_interaction_palette);
    }
}
