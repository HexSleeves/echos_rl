use bevy::prelude::*;

use crate::ui::{UiConstants, components::GameCamera};

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Projection::from(OrthographicProjection {
            scale: 0.5,
            ..OrthographicProjection::default_2d()
        }),
        UiConstants::GAME_LAYER,
        GameCamera,
    ));
}
