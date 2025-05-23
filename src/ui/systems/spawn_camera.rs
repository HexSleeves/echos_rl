use bevy::prelude::*;

use crate::ui::{components::GameCamera, UiConstants};

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera2d, UiConstants::GAME_LAYER, GameCamera));
}
