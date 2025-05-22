//! The screen state for the main gameplay.

use bevy::prelude::*;

use crate::{gameplay::map::generate_map, screens::ScreenState};

pub(super) fn plugin(app: &mut App) { app.add_systems(OnEnter(ScreenState::Gameplay), generate_map); }
