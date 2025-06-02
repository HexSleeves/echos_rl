use bevy::prelude::*;
use big_brain::prelude::*;

use crate::{
    core::states::GameState,
    gameplay::{enemies::systems, turns::components::TurnActor},
    prelude::gameplay::enemies::AIBehavior,
    rendering::screens::ScreenState,
};

/// System condition to check if any AI entities need to make decisions
fn ai_needs_decisions(ai_query: Query<&TurnActor, With<AIBehavior>>) -> bool {
    ai_query.iter().any(|turn_actor| turn_actor.peek_next_action().is_none())
}

/// AI plugin that handles big-brain AI behavior
pub fn plugin(app: &mut App) {
    // Add big-brain plugin for AI
    app.add_plugins(BigBrainPlugin::new(PreUpdate));

    app.add_systems(
        PreUpdate,
        (
            // Actions
            (
                systems::chase_player_action_system,
                systems::flee_from_player_action_system,
                systems::wander_action_system,
                systems::idle_action_system,
            )
                .in_set(BigBrainSet::Actions)
                .run_if(in_state(GameState::GatherActions))
                .run_if(in_state(ScreenState::Gameplay)),
            // Scorers
            (
                systems::chase_player_scorer_system,
                systems::flee_from_player_scorer_system,
                systems::wander_scorer_system,
            )
                .in_set(BigBrainSet::Scorers)
                .run_if(in_state(GameState::GatherActions))
                .run_if(in_state(ScreenState::Gameplay)),
        ),
    );
}
