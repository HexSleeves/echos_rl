use bevy::prelude::*;
use big_brain::prelude::*;

use crate::{
    core::{
        actions::Walk,
        components::{PlayerTag, Position},
        pathfinding,
        resources::{CurrentMap, FovMap, TurnQueue},
        types::{BuildableGameAction, GameActionBuilder},
    },
    gameplay::{
        enemies::{
            components::{AIAction, AIBehavior, AIState, ChasePlayerAction, ChasePlayerScorer},
            helpers,
        },
        turns::components::TurnActor,
    },
    prelude::assets::AIBehaviorType,
};

// ============================================================================
// SCORER SYSTEMS (Evaluate what the AI should do)
// ============================================================================

/// System that scores how much an AI wants to chase the player
pub fn chase_player_scorer_system(
    turn_queue: Res<TurnQueue>,
    current_map: Res<CurrentMap>,

    player_query: Query<(Entity, &Position), With<PlayerTag>>,
    mut ai_query: Query<(&Position, &mut AIBehavior, &TurnActor)>,
    mut scorer_query: Query<(&Actor, &mut Score), With<ChasePlayerScorer>>,
) {
    let Ok((player_entity, player_pos)) = player_query.single() else {
        // No player found or multiple players - skip AI processing
        return;
    };

    let current_turn = turn_queue.current_time();

    for (Actor(actor_entity), mut score) in scorer_query.iter_mut() {
        if *actor_entity == player_entity {
            continue;
        }

        let Ok((&ai_pos, mut ai_behavior, turn_actor)) = ai_query.get_mut(*actor_entity) else {
            info!("Actor must have required components");
            continue;
        };

        // Skip scoring if this entity already has an action queued
        if turn_actor.has_action() {
            continue;
        }

        let mut chase_score = 0.0;

        // Only hostile enemies want to chase
        if ai_behavior.behavior_type == AIBehaviorType::Hostile {
            // Check if AI can see the player using the AI's own FOV
            if FovMap::can_see_entity(ai_pos, ai_behavior.detection_range, *player_pos, &current_map) {
                let distance = ai_pos.ai_detection_distance(player_pos);

                if distance <= ai_behavior.detection_range as f32 {
                    // Higher score for closer players
                    // chase_score = 1.0 - (distance / ai_behavior.detection_range as f32);
                    // chase_score = chase_score.clamp(0.0, 1.0);

                    chase_score = 1.0;

                    info!(
                        "AI entity {:?} can see player at distance {:.1}, chase score: {:.2}",
                        actor_entity, distance, chase_score
                    );

                    // Update AI's knowledge of player position
                    ai_behavior.update_player_sighting(*player_pos, current_turn);
                }
            } else if let Some(last_known_pos) = ai_behavior.last_known_player_position {
                println!("Player not visible, but AI remembers where they were");

                // Player not visible, but AI remembers where they were
                if !ai_behavior.should_switch_to_wander(current_turn) {
                    let distance = ai_pos.ai_detection_distance(&last_known_pos);

                    // Lower score for remembered positions
                    chase_score = 0.3 * (1.0 - (distance / ai_behavior.detection_range as f32));
                    chase_score = chase_score.clamp(0.0, 0.5);
                }
            }

            println!("AI entity {:?} chase score: {:.2}", actor_entity, chase_score);
            score.set(chase_score);
        }
    }
}

/// System that handles chasing the player
pub fn chase_player_action_system(
    player_query: Query<&Position, With<PlayerTag>>,
    current_map: Res<CurrentMap>,
    mut action_query: Query<(&Actor, &mut ActionState), With<ChasePlayerAction>>,
    mut ai_query: Query<(&Position, &mut TurnActor, &mut AIState)>,
) {
    let Ok(player_pos) = player_query.single() else {
        // No player found or multiple players - skip AI processing
        return;
    };

    for (Actor(actor_entity), mut action_state) in action_query.iter_mut() {
        if let Ok((ai_pos, mut turn_actor, mut ai_state)) = ai_query.get_mut(*actor_entity) {
            match *action_state {
                ActionState::Init => {
                    // Initialize the action
                    *action_state = ActionState::Requested;
                }
                ActionState::Requested => {
                    // Only add action if the entity doesn't already have actions queued
                    if turn_actor.peek_next_action().is_some() {
                        // Already has an action queued, wait for it to be processed
                        *action_state = ActionState::Executing;
                        continue;
                    }

                    info!(
                        "AI entity {:?} performing chase action toward player at {:?}",
                        actor_entity, player_pos
                    );
                    ai_state.current_action = Some(AIAction::ChasePlayer);
                    ai_state.target_position = Some(*player_pos);

                    // Calculate direction toward player
                    let direction = helpers::calculate_direction_to_target(*ai_pos, *player_pos);

                    if let Some(dir) = direction {
                        // Check if the direct path is walkable
                        let (dx, dy) = dir.coord();
                        let target_pos = *ai_pos + (dx, dy);
                        if current_map.is_walkable(target_pos) {
                            turn_actor.add_action(
                                Walk::builder().with_entity(*actor_entity).with_direction(dir).build(),
                            );
                            *action_state = ActionState::Executing;
                        } else {
                            // Try alternative directions if direct path is blocked
                            if let Some(alt_dir) =
                                helpers::find_alternative_direction(*ai_pos, *player_pos, &current_map)
                            {
                                turn_actor.add_action(
                                    Walk::builder()
                                        .with_entity(*actor_entity)
                                        .with_direction(alt_dir)
                                        .build(),
                                );
                                *action_state = ActionState::Executing;
                            } else {
                                info!(
                                    "AI entity {:?} cannot find path to player, action failed",
                                    actor_entity
                                );
                                *action_state = ActionState::Failure;
                            }
                        }
                    } else {
                        // Already at player position or no valid direction
                        *action_state = ActionState::Success;
                    }
                }
                ActionState::Executing => {
                    // Check if the action has been processed (no more actions in queue)
                    if turn_actor.peek_next_action().is_none() {
                        *action_state = ActionState::Success;
                    }
                }
                ActionState::Success | ActionState::Failure => {
                    // Action completed, reset to init and wait for next decision cycle
                    ai_state.current_action = None; // or whatever "idle" enum you use
                    ai_state.target_position = None;
                    *action_state = ActionState::Init;
                }
                ActionState::Cancelled => {
                    // Action was cancelled, reset to init
                    *action_state = ActionState::Init;
                }
            }
        }
    }
}

fn generate_last_seen_path(
    ai_pos: Position,
    target_pos: Position,
    map_provider: &mut CurrentMap,
) -> Vec<Position> {
    pathfinding::utils::find_path(ai_pos, target_pos, map_provider, true).unwrap_or_default()
}
