use bevy::prelude::*;

use crate::core::components::Position;

#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct SpawnPoint {
    pub player_spawn: Option<Position>,
}

impl SpawnPoint {
    pub fn new(position: Position) -> Self { Self { player_spawn: Some(position) } }
}
