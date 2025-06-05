use bevy::{ecs::system::SystemState, prelude::*, time::common_conditions::on_timer};
use std::time::Instant;

use crate::{
    core::{
        components::PlayerTag,
        states::GameState,
        types::{GameAction, GameError},
    },
    gameplay::{
        player::components::AwaitingInput,
        turns::{components::TurnActor, turn_manager::TurnManager},
    },
};

const MAX_TURNS_PER_FRAME: u32 = 50; // Prevent infinite loops

#[derive(Debug, PartialEq)]
enum PlayerTurnDecision {
    ExecuteQueuedAction,
    SkipTurn,
    WaitForInput,
}

/// Determine what action a player should take
fn decide_player_action(actor: &TurnActor) -> PlayerTurnDecision {
    if actor.has_action() {
        if actor.peek_next_action().is_some() {
            PlayerTurnDecision::ExecuteQueuedAction
        } else {
            PlayerTurnDecision::SkipTurn
        }
    } else {
        PlayerTurnDecision::WaitForInput
    }
}

#[derive(Debug, PartialEq)]
enum AiTurnDecision {
    ExecuteQueuedAction,
    Wait,
}

/// Determine what action an AI should take
fn decide_ai_action(actor: &TurnActor) -> AiTurnDecision {
    if actor.has_action() { AiTurnDecision::ExecuteQueuedAction } else { AiTurnDecision::Wait }
}

#[derive(Debug, PartialEq)]
enum TurnFlowControl {
    Continue,
    EndProcessing,
    WaitForInput,
}

/// Process the result of a player action execution
fn handle_player_action_result(
    result: Result<u32, GameError>,
    entity: Entity,
    turn_manager: &mut TurnManager,
) -> TurnFlowControl {
    match result {
        Ok(time_spent) => {
            let _ = turn_manager.end_entity_turn(entity, time_spent);
            TurnFlowControl::EndProcessing
        }
        Err(e) => {
            error!("Player queued action failed: {:?}", e);
            let _ = turn_manager.end_entity_turn(entity, 0);
            TurnFlowControl::EndProcessing
        }
    }
}

/// Process the result of an AI action execution
fn handle_ai_action_result(
    result: Result<u32, GameError>,
    entity: Entity,
    name: String,
    turn_manager: &mut TurnManager,
) -> TurnFlowControl {
    match result {
        Ok(time_spent) => {
            let _ = turn_manager.end_entity_turn(entity, time_spent);
            TurnFlowControl::Continue
        }
        Err(e) => {
            warn!("AI {} action failed: {:?}, scheduling retry", name, e);
            let _ = turn_manager.end_entity_turn(entity, 50); // Short delay before retry
            TurnFlowControl::Continue
        }
    }
}

/// Handle player waiting for input
fn handle_player_wait_for_input(entity: Entity, turn_manager: &mut TurnManager) -> TurnFlowControl {
    let _ = turn_manager.end_entity_turn(entity, 0); // Put player back in queue
    debug!("Player awaiting input");
    TurnFlowControl::WaitForInput
}

/// Handle AI waiting (no actions available)
fn handle_ai_wait(entity: Entity, name: &str, turn_manager: &mut TurnManager) -> TurnFlowControl {
    debug!("AI {} has no actions, waiting", name);
    let _ = turn_manager.end_entity_turn(entity, 100); // Default wait time
    TurnFlowControl::Continue
}

/// Handle player skip turn (empty action queue)
fn handle_player_skip_turn(entity: Entity, turn_manager: &mut TurnManager) -> TurnFlowControl {
    let _ = turn_manager.end_entity_turn(entity, 0);
    TurnFlowControl::Continue
}

/// Log action execution for debugging
fn log_action_execution(action_category: impl ToString, entity_name: impl ToString, is_player: bool) {
    if is_player {
        info!("Player performing queued action: {}", action_category.to_string());
    } else {
        info!("AI {} performing action: {}", entity_name.to_string(), action_category.to_string());
    }
}

/// Main turn processing system
pub fn process_turns(world: &mut World) {
    let mut state: SystemState<(
        ResMut<NextState<GameState>>,
        Query<(Entity, &mut TurnActor, Option<&PlayerTag>, &Name)>,
    )> = SystemState::new(world);

    world.resource_scope(|world, mut turn_manager: Mut<TurnManager>| {
        // Periodic cleanup of dead entities
        let cleanup_metrics = turn_manager.cleanup_dead_entities(world);

        // Log significant cleanups
        if cleanup_metrics.entities_removed > 5 {
            info!(
                "Turn queue cleanup: removed {} entities in {:?}",
                cleanup_metrics.entities_removed, cleanup_metrics.processing_time
            );
        }

        // Debug: Print current turn state
        if log::log_enabled!(log::Level::Debug) {
            debug!(
                "Turn {}, Time {}, Queue size: {}",
                turn_manager.current_turn(),
                turn_manager.current_time(),
                turn_manager.total_action_count()
            );
        }

        // Process turns until we need to wait for player input or queue is empty
        let mut turns_processed = 0;

        while turns_processed < MAX_TURNS_PER_FRAME {
            let Some(entity) = turn_manager.start_entity_turn() else {
                debug!("No more entities in turn queue");
                break;
            };

            turns_processed += 1;

            let (mut next_state, mut q_actor) = state.get_mut(world);
            let Ok((entity, mut actor, player_tag, name)) = q_actor.get_mut(entity) else {
                warn!("Entity {:?} missing required components, skipping turn", entity);
                continue;
            };

            // Validate entity is still alive and ready
            if !actor.is_alive() {
                debug!("Dead entity {} found in turn queue", name);
                continue;
            }

            let is_player = player_tag.is_some();

            // Handle player turn inline to avoid borrowing conflicts
            if is_player {
                let decision = decide_player_action(&actor);
                match decision {
                    PlayerTurnDecision::ExecuteQueuedAction => {
                        if let Some(action) = actor.next_action() {
                            log_action_execution(action.category(), "Player", true);
                            let action_result = execute_action_with_retry(entity, action, world);
                            let flow_control =
                                handle_player_action_result(action_result, entity, &mut turn_manager);
                            match flow_control {
                                TurnFlowControl::EndProcessing => {
                                    clear_ai_preferred_actions(world);
                                    return;
                                }
                                TurnFlowControl::Continue => continue,
                                TurnFlowControl::WaitForInput => return,
                            }
                        } else {
                            let flow_control = handle_player_skip_turn(entity, &mut turn_manager);
                            match flow_control {
                                TurnFlowControl::Continue => continue,
                                TurnFlowControl::EndProcessing => return,
                                TurnFlowControl::WaitForInput => return,
                            }
                        }
                    }
                    PlayerTurnDecision::SkipTurn => {
                        let flow_control = handle_player_skip_turn(entity, &mut turn_manager);
                        match flow_control {
                            TurnFlowControl::Continue => continue,
                            TurnFlowControl::EndProcessing => return,
                            TurnFlowControl::WaitForInput => return,
                        }
                    }
                    PlayerTurnDecision::WaitForInput => {
                        next_state.set(GameState::GatherActions);
                        world.entity_mut(entity).insert(AwaitingInput);
                        let flow_control = handle_player_wait_for_input(entity, &mut turn_manager);
                        match flow_control {
                            TurnFlowControl::WaitForInput => return,
                            TurnFlowControl::EndProcessing => return,
                            TurnFlowControl::Continue => continue,
                        }
                    }
                }
            } else {
                // Handle AI turn using extracted decision logic
                let decision = decide_ai_action(&actor);
                match decision {
                    AiTurnDecision::ExecuteQueuedAction => {
                        if let Some(action) = actor.next_action() {
                            log_action_execution(action.category(), name, false);

                            let name = name.to_string();
                            let action_result = execute_action_with_retry(entity, action, world);
                            let flow_control =
                                handle_ai_action_result(action_result, entity, name, &mut turn_manager);

                            match flow_control {
                                TurnFlowControl::Continue => continue,
                                TurnFlowControl::EndProcessing => continue,
                                TurnFlowControl::WaitForInput => continue,
                            }
                        } else {
                            let flow_control = handle_ai_wait(entity, name.as_ref(), &mut turn_manager);
                            match flow_control {
                                TurnFlowControl::Continue => continue,
                                TurnFlowControl::EndProcessing => continue,
                                TurnFlowControl::WaitForInput => continue,
                            }
                        }
                    }
                    AiTurnDecision::Wait => {
                        let flow_control = handle_ai_wait(entity, name.as_ref(), &mut turn_manager);
                        match flow_control {
                            TurnFlowControl::Continue => continue,
                            TurnFlowControl::EndProcessing => continue,
                            TurnFlowControl::WaitForInput => continue,
                        }
                    }
                }
            }
        }

        if turns_processed >= MAX_TURNS_PER_FRAME {
            warn!("Hit maximum turns per frame limit ({}), deferring remaining turns", MAX_TURNS_PER_FRAME);
        }

        // Check if the queue is empty and no turns were processed (e.g., game start or after all actions)
        if turn_manager.is_empty() && turns_processed == 0 {
            debug!("Turn queue was empty and no turns processed this frame.");
            // Optionally, transition to a different game state if nothing is happening
            // let (mut next_state, ..) = global_resources_state.get_mut(world);
            // next_state.set(GameState::Paused); // Example
            // global_resources_state.apply(world);
        } else if turn_manager.is_empty() {
            debug!("Turn queue became empty after processing turns this frame.");
        }
    });
}

/// Execute action with automatic retry mechanism for certain error types
fn execute_action_with_retry(
    entity: Entity,
    mut action: Box<dyn GameAction>,
    world: &mut World,
) -> Result<u32, GameError> {
    const MAX_RETRIES: u8 = 3;
    let mut retries = 0;
    let start_time = Instant::now();

    loop {
        match action.perform(world) {
            Ok(time_spent) => {
                // Log slow actions
                let execution_time = start_time.elapsed();
                if execution_time.as_millis() > 10 {
                    debug!(
                        "Slow action execution: {:?} took {:?} for entity {:?}",
                        action.category(),
                        execution_time,
                        entity
                    );
                }
                return Ok(time_spent as u32);
            }
            Err(GameError::Retry(new_action)) => {
                action = new_action;
                retries += 1;

                if retries >= MAX_RETRIES {
                    warn!("Action retry limit exceeded for entity {:?}, defaulting to wait", entity);
                    return Ok(100); // Default wait time
                }

                debug!("Retrying action for entity {:?} (attempt {})", entity, retries + 1);
            }
            Err(GameError::InvalidTarget) => {
                debug!("Invalid target for entity {:?}, defaulting to wait", entity);
                return Ok(50); // Short wait for invalid target
            }
            Err(GameError::InsufficientResources) => {
                debug!("Insufficient resources for entity {:?}, defaulting to wait", entity);
                return Ok(100); // Wait for resources to regenerate
            }
            Err(GameError::ActionBlocked) => {
                debug!("Action blocked for entity {:?}, defaulting to wait", entity);
                return Ok(25); // Short wait for blocked action
            }
            Err(e) => {
                error!("Unrecoverable action error for entity {:?}: {:?}", entity, e);
                return Err(e);
            }
        }
    }
}

/// Clear AI preferred actions after player turn
fn clear_ai_preferred_actions(world: &mut World) {
    let mut ai_query = world.query_filtered::<&mut TurnActor, Without<PlayerTag>>();
    let mut cleared_count = 0;

    for mut actor in ai_query.iter_mut(world) {
        if actor.get_preferred_action().is_some() {
            actor.clear_preferred_action();
            cleared_count += 1;
        }
    }

    if cleared_count > 0 {
        debug!("Cleared preferred actions for {} AI entities", cleared_count);
    }
}

/// System to validate turn queue integrity (runs occasionally)
pub fn validate_turn_queue(world: &mut World) {
    world.resource_scope(|world, turn_manager: Mut<TurnManager>| {
        let mut invalid_entities: Vec<Entity> = Vec::new();
        let mut total_checked = 0;

        // This would need to be implemented in TurnManager
        // for (turn, time, entity) in turn_manager.iter_entities() {
        //     total_checked += 1;
        //
        //     if !world.entities().contains(*entity) {
        //         invalid_entities.push(*entity);
        //         continue;
        //     }
        //
        //     let entity_ref = world.entity(*entity);
        //     if !entity_ref.contains::<TurnActor>() {
        //         invalid_entities.push(*entity);
        //     }
        // }

        if !invalid_entities.is_empty() {
            warn!(
                "Found {} invalid entities in turn queue out of {} total",
                invalid_entities.len(),
                total_checked
            );

            // Remove invalid entities
            // for entity in invalid_entities {
            //     turn_manager.remove_entity(entity);
            // }
        }
    });
}

/// System to handle turn timeouts (for multiplayer or AI debugging)
pub fn handle_turn_timeouts(
    mut turn_manager: ResMut<TurnManager>,
    mut q_actors: Query<(Entity, &mut TurnActor, &Name)>,
    time: Res<Time>,
) {
    const TURN_TIMEOUT_SECONDS: f32 = 30.0;

    // This would need turn start time tracking in TurnManager
    // if turn_manager.current_turn_duration() > TURN_TIMEOUT_SECONDS {
    //     if let Some((entity, name)) = turn_manager.get_current_actor() {
    //         warn!("Turn timeout for entity {} ({}), forcing wait action", entity, name);
    //
    //         if let Ok((entity, mut actor, name)) = q_actors.get_mut(entity) {
    //             actor.clear_all_actions();
    //             turn_manager.end_entity_turn(entity, 100);
    //         }
    //     }
    // }
}

/// Debug system to print turn queue state
pub fn debug_turn_queue(turn_manager: Res<TurnManager>) {
    if !log::log_enabled!(log::Level::Debug) {
        return;
    }

    debug!("=== Turn Queue Debug ===");
    debug!("Current Turn: {}", turn_manager.current_turn());
    debug!("Current Time: {}", turn_manager.current_time());
    debug!("Queue Empty: {}", turn_manager.is_empty());

    // Print next few entities in queue
    // This would need to be implemented in TurnManager
    // for (i, (turn, time, entity)) in turn_manager.peek_next_entities(5).enumerate() {
    //     debug!("  {}: Turn {}, Time {}, Entity {:?}", i, turn, time, entity);
    // }
}

/// Plugin to register all turn-related systems
pub struct TurnSystemPlugin;

impl Plugin for TurnSystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                process_turns.run_if(in_state(GameState::ProcessTurns)),
                validate_turn_queue
                    .run_if(on_timer(std::time::Duration::from_secs(10)))
                    .run_if(in_state(GameState::ProcessTurns)),
                handle_turn_timeouts.run_if(in_state(GameState::ProcessTurns)),
            ),
        )
        .add_systems(
            Update,
            debug_turn_queue
                .run_if(on_timer(std::time::Duration::from_secs(5)))
                .run_if(in_state(GameState::ProcessTurns))
                .run_if(|| log::log_enabled!(log::Level::Debug)),
        );
    }
}
