use bevy::prelude::*;
use big_brain::prelude::*;

use crate::{core::states::GameState, gameplay::enemies::systems, rendering::screens::ScreenState};

/// AI plugin that handles big-brain AI behavior
pub fn plugin(app: &mut App) {
    // Add big-brain plugin for AI
    app.add_plugins(BigBrainPlugin::new(PreUpdate));

    // AI systems should run during both GatherActions and ProcessTurns states
    // This allows AI to generate actions both when gathering actions and when processing turns
    app.add_systems(
        PreUpdate,
        (
            systems::chase_player_scorer_system,
            systems::flee_from_player_scorer_system,
            systems::wander_scorer_system,
        )
            .in_set(BigBrainSet::Scorers)
            .run_if(in_state(GameState::GatherActions))
            .run_if(in_state(ScreenState::Gameplay)),
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
            .run_if(in_state(GameState::GatherActions))
            .run_if(in_state(ScreenState::Gameplay)),
    );
}
