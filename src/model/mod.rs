use bevy::prelude::*;

pub mod actions;
// pub mod commands;
pub mod components;
pub mod entities;
pub mod generation;
pub mod resources;
pub mod systems;
pub mod types;
pub mod utils;

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
        .register_type::<resources::SpawnPoint>()
        // Register entity definition types
        .register_type::<entities::EntityDefinition>()
        .register_type::<entities::EntityComponents>()
        .register_type::<entities::TurnActorData>()
        .register_type::<entities::ViewShedData>()
        .register_type::<entities::TileSpriteData>()
        // Register EntityDefinition as an asset type
        .init_asset::<entities::EntityDefinition>();
}
