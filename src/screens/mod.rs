//! The game's main screen states and transitions between them.

mod gameplay;
pub mod loading;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<ScreenState>();
    app.add_plugins((gameplay::plugin, loading::plugin));
}

/// The game's main screen states.
#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
#[states(scoped_entities)]
pub enum ScreenState {
    // Splash,
    // Title,
    // Credits,
    // Settings,
    #[default]
    Loading,
    Gameplay,
}
