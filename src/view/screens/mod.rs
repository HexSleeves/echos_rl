//! The game's main screen states and transitions between them.

// mod credits;
mod gameplay;
mod loading;
// mod settings;
// mod splash;
// mod title;

use bevy::prelude::*;

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

pub(super) fn plugin(app: &mut App) {
    app.init_state::<ScreenState>();

    app.add_plugins((
        // credits::plugin,
        gameplay::plugin,
        loading::plugin,
        // settings::plugin,
        // splash::plugin,
        // title::plugin,
    ));
}
