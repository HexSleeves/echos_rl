use bevy::prelude::*;

pub mod actions;
pub mod ai;
pub mod commands;
pub mod components;
pub mod generation;
pub mod resources;
pub mod types;

mod model_constants;
pub use self::model_constants::*;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    /// Gather actions from all entities (player + monsters) without processing them
    GatherActions,
    /// Process all queued actions in turn queue order
    #[default]
    ProcessTurns,
}

pub(super) fn plugin(app: &mut App) {
    // Register component types
    app.register_type::<components::Description>()
        .register_type::<components::PlayerTag>()
        .register_type::<components::Position>()
        .register_type::<components::TerrainType>()
        .register_type::<components::ViewShed>()
        .register_type::<resources::SpawnPoint>();

    app.add_plugins(ai::plugin);
}
