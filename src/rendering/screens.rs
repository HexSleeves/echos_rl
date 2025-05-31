use bevy::prelude::*;

pub mod gameplay;

/// Screen states for the rendering system
#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum ScreenState {
    #[default]
    Loading,
    MainMenu,
    Gameplay,
    Paused,
}

/// Rendering screens plugin that manages different game screens
pub fn plugin(app: &mut App) {
    app.init_state::<ScreenState>();
    app.add_plugins(gameplay::plugin);
}
