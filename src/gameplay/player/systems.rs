use bevy::{platform::collections::HashMap, prelude::*};
use brtk::prelude::Direction;
use leafwing_input_manager::prelude::*;
use once_cell::sync::Lazy;
use std::time::Duration;

use crate::{
    core::{
        actions::Walk,
        states::GameState,
        types::{ActionType, BuildableGameAction, GameActionBuilder},
    },
    gameplay::{player::components::AwaitingInput, turns::components::TurnActor},
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
    mut commands: Commands,
    time: Res<Time>,
    mut timer: Local<PlayerTimer>,
    input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut player_query: Query<&ActionState<PlayerAction>>,

    action_state: Single<(Entity, &ActionState<PlayerAction>), With<PlayerTag>>,
    q_awaiting_input: Option<Single<(Entity, &mut TurnActor), With<AwaitingInput>>>,
) {
    // Tick timer until duration is met.
    if !timer.finished() {
        timer.tick(time.delta());
    }

    // if let Some(a) = q_awaiting_input {
    //     let (entity, mut p_actor) = a.into_inner();
    //     let mut action: Option<ActionType> = None;

    //     for (act, keys) in ACTION_KEYS.iter() {
    //         if keys.iter().any(|key| input.just_pressed(*key)) {
    //             info!("Player input: {:?}", act);
    //             action = Some(*act);
    //             break;
    //         }
    //     }

    //     if let Some(act) = &action {
    //         if let ActionType::Move(dir) = act {
    //             p_actor.add_action(Walk::builder().with_entity(entity).with_direction(*dir).build());
    //         }

    //         commands.entity(entity).remove::<AwaitingInput>();

    //         // After player action is gathered, move to ProcessTurns to execute all actions in order
    //         next_state.set(GameState::ProcessTurns);
    //     }
    // }

    for action_state in player_query.iter_mut() {
        let did_action = false;

        // Actions
        if action_state.just_pressed(PlayerAction::Wait) {
            action_queue.add_action(ActionType::Wait);
            did_action = true;
            info!("Player gave input: WAIT");
        }

        // Movement
        for input_direction in PlayerAction::DIRECTIONS {
            if action_state.just_pressed(input_direction)
                || (action_state.pressed(input_direction)
                    && action_state.current_duration(input_direction) > PRESSED_DURATION)
                    && timer.finished()
            {
                if let Some(direction) = input_direction.direction() {
                    timer.reset();
                    action_queue.add_action(ActionType::Movement(*player_position + direction));
                    did_action = true;

                    info!("Player gave input: MOVE");
                }
            }
        }

        if did_action {
            commands.entity(entity).remove::<AwaitingInput>();

            // After player action is gathered, move to ProcessTurns to execute all actions in order
            next_state.set(GameState::ProcessTurns);
        }
    }
}
