use bevy::prelude::*;
use systems::player_input_system;

use crate::model::GameState;

pub mod systems;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, player_input_system.run_if(in_state(GameState::GatherActions)));
}
