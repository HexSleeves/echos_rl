pub mod ai_spawner;
pub mod systems;

use bevy::prelude::*;
use big_brain::prelude::*;

use super::GameState;

pub(super) fn plugin(app: &mut App) {
    // Add big-brain plugin for AI
    app.add_plugins(BigBrainPlugin::new(PreUpdate));

    app.add_systems(
        PreUpdate,
        (
            systems::chase_player_scorer_system,
            systems::flee_from_player_scorer_system,
            systems::wander_scorer_system,
            systems::player_visibility_scorer_system,
        )
            .in_set(BigBrainSet::Scorers)
            .run_if(in_state(GameState::ProcessTurns)),
    );

    // Add AI action systems (run in PreUpdate with BigBrainSet::Actions)
    app.add_systems(
        PreUpdate,
        (
            systems::chase_player_action_system,
            systems::flee_from_player_action_system,
            systems::wander_action_system,
            systems::idle_action_system,
        )
            .in_set(BigBrainSet::Actions)
            .run_if(in_state(GameState::ProcessTurns)),
    );
}
