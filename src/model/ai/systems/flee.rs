use bevy::prelude::*;
use big_brain::prelude::*;
use brtk::prelude::Direction;

use crate::model::{
    actions::WalkBuilder,
    components::{
        AIAction, AIBehavior, AIBehaviorType, AIState, FleeFromPlayerAction, FleeFromPlayerScorer, PlayerTag,
        Position, TurnActor,
    },
    resources::{CurrentMap, FovMap},
    types::GameActionBuilder,
};

// ============================================================================
// SCORER SYSTEMS (AI "Eyes" - Evaluate the world and assign scores)
// ============================================================================

/// System that scores how much an AI wants to flee from the player
pub fn flee_from_player_scorer_system(
    player_query: Query<&Position, With<PlayerTag>>,
    fov_map: Res<FovMap>,
    mut scorer_query: Query<(&Actor, &mut Score), With<FleeFromPlayerScorer>>,
    ai_query: Query<(&Position, &AIBehavior)>,
) {
    let Ok(player_pos) = player_query.single() else {
        return;
    };

    for (Actor(actor_entity), mut score) in scorer_query.iter_mut() {
        if let Ok((ai_pos, ai_behavior)) = ai_query.get(*actor_entity) {
            let mut flee_score = 0.0;

            // Only passive enemies want to flee
            if ai_behavior.behavior_type == AIBehaviorType::Passive {
                // Check if AI can see the player
                if fov_map.is_visible(*player_pos) {
                    let distance = crate::utils::calculate_distance(*ai_pos, *player_pos);

                    // Higher score for closer players (more urgent to flee)
                    if distance <= ai_behavior.detection_range as f32 {
                        flee_score = 1.0 - (distance / ai_behavior.detection_range as f32);
                        flee_score = flee_score.clamp(0.0, 1.0);
                    }
                }
            }

            score.set(flee_score);
        }
    }
}

// ============================================================================
// ACTION SYSTEMS (AI "Hands" - Execute behaviors)
// ============================================================================

/// System that handles fleeing from the player
pub fn flee_from_player_action_system(
    player_query: Query<&Position, With<PlayerTag>>,
    current_map: Res<CurrentMap>,
    mut action_query: Query<(&Actor, &mut ActionState), With<FleeFromPlayerAction>>,
    mut ai_query: Query<(&Position, &mut TurnActor, &mut AIState)>,
) {
    let Ok(player_pos) = player_query.single() else {
        return;
    };

    for (Actor(actor_entity), mut action_state) in action_query.iter_mut() {
        if let Ok((ai_pos, mut turn_actor, mut ai_state)) = ai_query.get_mut(*actor_entity) {
            match *action_state {
                ActionState::Requested => {
                    ai_state.current_action = AIAction::FleeFromPlayer;

                    // Calculate direction away from player
                    let flee_direction = calculate_flee_direction(*ai_pos, *player_pos);

                    if let Some(dir) = flee_direction {
                        let new_position = *ai_pos + dir.coord();

                        // Check if the new position is walkable
                        if current_map.is_walkable(new_position)
                            && current_map.get_actor(new_position).is_none()
                        {
                            // Queue the walk action
                            turn_actor.add_action(
                                WalkBuilder::new()
                                    .with_entity(*actor_entity)
                                    .with_direction(dir)
                                    .build(),
                            );
                            *action_state = ActionState::Success;
                        } else {
                            // Try a random direction if can't flee directly
                            if let Some(random_dir) =
                                crate::utils::find_random_walkable_direction(*ai_pos, &current_map)
                            {
                                turn_actor.add_action(
                                    WalkBuilder::new()
                                        .with_entity(*actor_entity)
                                        .with_direction(random_dir)
                                        .build(),
                                );
                                *action_state = ActionState::Success;
                            } else {
                                *action_state = ActionState::Failure;
                            }
                        }
                    } else {
                        *action_state = ActionState::Failure;
                    }
                }
                ActionState::Cancelled => {
                    *action_state = ActionState::Failure;
                }
                _ => {}
            }
        }
    }
}

/// Calculate direction to flee from a target
pub(super) fn calculate_flee_direction(from: Position, threat: Position) -> Option<Direction> {
    let dx = from.x() - threat.x(); // Opposite direction
    let dy = from.y() - threat.y();

    // Convert to direction (away from threat)
    match (dx.signum(), dy.signum()) {
        (0, -1) => Some(Direction::NORTH),
        (1, -1) => Some(Direction::NORTH_EAST),
        (1, 0) => Some(Direction::EAST),
        (1, 1) => Some(Direction::SOUTH_EAST),
        (0, 1) => Some(Direction::SOUTH),
        (-1, 1) => Some(Direction::SOUTH_WEST),
        (-1, 0) => Some(Direction::WEST),
        (-1, -1) => Some(Direction::NORTH_WEST),
        _ => None, // Same position
    }
}
