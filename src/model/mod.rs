use bevy::prelude::*;

pub mod actions;
pub mod assets;
pub mod commands;
pub mod components;
pub mod generation;
pub mod resources;
pub mod types;

mod model_constants;
pub use self::model_constants::*;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    PlayerTurn,
    MonstersTurn,
    #[default]
    ProcessTurns,
}

pub(super) fn plugin(app: &mut App) {
    app.register_type::<components::Description>()
        .register_type::<components::PlayerTag>()
        .register_type::<components::Position>()
        .register_type::<components::TerrainType>()
        .register_type::<components::ViewShed>()
        .register_type::<resources::SpawnPoint>();

    app.add_plugins(assets::plugin);
}
