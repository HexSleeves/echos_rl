use bevy::{platform::collections::HashMap, prelude::*};
use brtk::prelude::Direction;
use once_cell::sync::Lazy;

use crate::{
    core::states::GameState,
    gameplay::{
        player::components::AwaitingInput,
        turns::components::TurnActor,
    },
    model::{
        actions::Walk,
        types::{ActionType, BuildableGameAction, GameActionBuilder},
    },
};

// Define action keys for player input
static ACTION_KEYS: Lazy<HashMap<ActionType, Vec<KeyCode>>> = Lazy::new(|| {
    HashMap::from([
        (ActionType::Move(Direction::NORTH), vec![KeyCode::KeyW, KeyCode::ArrowUp]),
        (ActionType::Move(Direction::SOUTH), vec![KeyCode::KeyS, KeyCode::ArrowDown]),
        (ActionType::Move(Direction::WEST), vec![KeyCode::KeyA, KeyCode::ArrowLeft]),
        (ActionType::Move(Direction::EAST), vec![KeyCode::KeyD, KeyCode::ArrowRight]),
        (ActionType::Wait, vec![KeyCode::Space, KeyCode::Period]),
    ])
});

/// System that handles player input and converts it into game actions
pub fn player_input_system(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    q_awaiting_input: Option<Single<(Entity, &mut TurnActor), With<AwaitingInput>>>,
) {
    if let Some(a) = q_awaiting_input {
        let (entity, mut p_actor) = a.into_inner();
        let mut action: Option<ActionType> = None;

        for (act, keys) in ACTION_KEYS.iter() {
            if keys.iter().any(|key| input.just_pressed(*key)) {
                info!("Player input: {:?}", act);
                action = Some(*act);
                break;
            }
        }

        if let Some(act) = &action {
            if let ActionType::Move(dir) = act {
                p_actor.add_action(Walk::builder().with_entity(entity).with_direction(*dir).build());
            }

            commands.entity(entity).remove::<AwaitingInput>();

            // After player action is gathered, move to ProcessTurns to execute all actions in order
            next_state.set(GameState::ProcessTurns);
        }
    }
}

/// System that spawns the player entity
pub fn spawn_player(
    mut commands: Commands,
    current_map: Res<crate::model::resources::CurrentMap>,
    spawn_point: Option<Res<crate::model::resources::SpawnPoint>>,
) {
    use crate::{
        model::{commands::SpawnEntityCommands, components::Position},
    };

    // Determine where to spawn the player
    let player_position = spawn_point
        .and_then(|sp| sp.player_spawn)
        .or_else(|| current_map.get_random_walkable_position())
        .unwrap_or_else(|| {
            warn!("No valid spawn point found, using default position");
            Position::new(0, 0)
        });

    // Use the command-based spawning
    commands.spawn_player(player_position);

    info!("Queued player spawn at {:?}", player_position);
}
