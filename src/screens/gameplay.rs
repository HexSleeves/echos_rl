//! The screen state for the main gameplay.

use bevy::prelude::*;

use crate::{gameplay::map, screens::ScreenState, utils};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(ScreenState::Gameplay), map::generate_map)
        .add_systems(Update, utils::camera::camera_movement);
}
