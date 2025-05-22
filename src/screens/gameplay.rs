//! The screen state for the main gameplay.

use bevy::prelude::*;

use crate::screens::ScreenState;

pub(super) fn plugin(app: &mut App) {
    // app.add_systems(OnEnter(Screen::Gameplay), generate_map);
    // app.add_systems(
    //     Update,
    //     return_to_title_screen.run_if(in_state(Screen::Gameplay).and(input_just_pressed(KeyCode::Escape))),
    // );
}

// fn return_to_title_screen(mut next_screen: ResMut<NextState<Screen>>) { next_screen.set(Screen::Title); }
