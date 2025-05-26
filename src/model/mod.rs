use bevy::prelude::*;
use systems::FovSystemSet;

pub mod actions;
// pub mod commands;
pub mod components;
pub mod generation;
pub mod resources;
pub mod systems;
pub mod types;
pub mod utils;

mod model_constants;
pub use self::model_constants::*;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    PlayerTurn,
    MonstersTurn,
    ComputeFov,
    ProcessTurns,
    ProcessActions,
}

pub(super) fn plugin(app: &mut App) {
    app.register_type::<components::Description>()
        .register_type::<components::PlayerTag>()
        .register_type::<components::Position>()
        .register_type::<components::TerrainType>()
        .register_type::<components::ViewShed>()
        .register_type::<resources::SpawnPoint>();

    app.configure_sets(Update, (FovSystemSet::Compute, FovSystemSet::React).chain());
}
