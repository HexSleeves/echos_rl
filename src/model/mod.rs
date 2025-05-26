use bevy::prelude::*;

// pub mod actions;
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
        // .register_type::<components::Renderable>()
        .register_type::<components::TerrainType>()
        .register_type::<components::ViewShed>()
        .register_type::<resources::SpawnPoint>();

    // app.init_resource::<TurnQueue>();
    app.init_state::<GameState>()
        .init_resource::<resources::CurrentMap>()
        .init_resource::<resources::FovMap>()
        .init_resource::<resources::SpawnPoint>();

    // app.add_systems(Startup, (spawn_player, compute_fov, process_turns).chain());
    // app.add_systems(Startup, (spawn_map, spawn_player, compute_fov, process_turns).chain());

    // app.add_systems(Update, process_turns.run_if(in_state(GameState::ProcessTurns)));
    // app.add_systems(Update, monsters_turn.run_if(in_state(GameState::MonstersTurn)));
    // app.add_systems(OnExit(GameState::ProcessTurns), compute_fov);
}
