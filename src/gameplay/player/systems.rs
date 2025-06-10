use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use std::time::Duration;

use crate::{
    core::{states::GameState, types::ActionType},
    debug_turns,
    gameplay::{
        player::{actions::PlayerAction, components::AwaitingInput},
        turns::components::TurnActor,
    },
    prelude::core::PlayerTag,
};

// ============================================================================
// INPUT SYSTEMS
// ============================================================================

const REPEAT_DURATION: Duration = Duration::from_millis(100);
const PRESSED_DURATION: Duration = Duration::from_millis(500);

#[derive(Deref, DerefMut)]
pub struct PlayerTimer(pub Timer);

impl Default for PlayerTimer {
    fn default() -> Self { Self(Timer::new(REPEAT_DURATION, TimerMode::Once)) }
}

/// System that handles player input and converts it into game actions
pub fn player_input_system(
    time: Res<Time>,
    mut timer: Local<PlayerTimer>,

    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
    player_query: Single<(Entity, &ActionState<PlayerAction>, &mut TurnActor), With<PlayerTag>>,
) {
    // Tick timer until duration is met.
    if !timer.finished() {
        timer.tick(time.delta());
    }

    let mut action: Option<ActionType> = None;
    let (entity, action_state, mut p_actor) = player_query.into_inner();

    // Actions
    if action_state.just_pressed(&PlayerAction::Wait) {
        action = Some(ActionType::Wait);
    }

    // Movement
    for input_direction in PlayerAction::DIRECTIONS {
        if (action_state.just_pressed(&input_direction)
            || (action_state.pressed(&input_direction)
                && action_state.current_duration(&input_direction) > PRESSED_DURATION)
                && timer.finished())
            && let Some(direction) = input_direction.direction()
        {
            timer.reset();
            action = Some(ActionType::MoveDelta(direction));
        }
    }

    if let Some(action) = action {
        debug_turns!("Player queued action: {:?}", action);

        // Queue the action directly - no more builder pattern!
        p_actor.queue_action(action);

        commands.entity(entity).remove::<AwaitingInput>();

        // After player action is gathered, move to ProcessTurns to execute all actions in order
        next_state.set(GameState::ProcessTurns);
    }
}
