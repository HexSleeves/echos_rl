use bevy::prelude::*;

use crate::model::{
    components::{Description, PlayerTag, Position, Renderable, TerrainType, ViewShed},
    resources::{CurrentMap, FovMap, SpawnPoint},
    systems::{camera_movement, spawn_map},
};

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    PlayerTurn,
    MonstersTurn,
    ComputeFov,
    ProcessTurns,
    ProcessActions,
}

pub struct ModelPlugin;
impl Plugin for ModelPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Description>()
            .register_type::<PlayerTag>()
            .register_type::<Position>()
            .register_type::<Renderable>()
            .register_type::<TerrainType>()
            .register_type::<ViewShed>()
            .register_type::<SpawnPoint>();

        // app.init_resource::<TurnQueue>();
        app.init_state::<GameState>()
            .init_resource::<CurrentMap>()
            .init_resource::<FovMap>()
            .init_resource::<SpawnPoint>();

        app.add_plugins(bevy_ecs_tilemap::TilemapPlugin);

        // app.add_systems(Startup, (spawn_player, compute_fov, process_turns).chain());
        // app.add_systems(Startup, (spawn_map, spawn_player, compute_fov, process_turns).chain());

        app.add_systems(Startup, spawn_map).add_systems(Update, camera_movement);

        // app.add_systems(Update, process_turns.run_if(in_state(GameState::ProcessTurns)));
        // app.add_systems(Update, monsters_turn.run_if(in_state(GameState::MonstersTurn)));
        // app.add_systems(OnExit(GameState::ProcessTurns), compute_fov);
    }
}
