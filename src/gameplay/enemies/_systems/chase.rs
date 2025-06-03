use bevy::prelude::*;
use big_brain::prelude::*;
use echos_assets::entities::AIBehaviorType;

use crate::{
    core::{
        components::Position,
        resources::{CurrentMap, TurnQueue},
        types::ActionType,
    },
    gameplay::enemies::components::{AIBehavior, AIComponent},
    prelude::{
        core::{FieldOfView, FovMap, PlayerTag},
        gameplay::enemies::ChasePlayerScorer,
    },
};
use brtk::pathfinding::PathFinder;

pub fn chase_player_scorer_system(
    player_query: Query<&Position, With<PlayerTag>>,
    fov_map: Res<FovMap>,
    turn_queue: Res<TurnQueue>,
    mut ai_query: Query<(&Position, &mut AIBehavior)>,
    mut scorer_query: Query<(&Actor, &mut Score), With<ChasePlayerScorer>>,
) {
    println!("Chase player scorer system x1");

    let Ok(player_pos) = player_query.single() else {
        return;
    };

    let current_turn = turn_queue.current_time();

    for (Actor(actor_entity), mut score) in scorer_query.iter_mut() {
        println!("Chase player scorer system x2");

        if let Ok((ai_pos, mut ai_behavior)) = ai_query.get_mut(*actor_entity) {
            println!("Chase player scorer system x3");
            let mut chase_score = 0.0;

            // Only hostile enemies want to chase
            if ai_behavior.behavior_type == AIBehaviorType::Hostile {
                // Check if AI can see the player
                if fov_map.is_visible(*player_pos) {
                    let distance = crate::utils::calculate_distance(*ai_pos, *player_pos);

                    if distance <= ai_behavior.detection_range as f32 {
                        // Update AI's knowledge of player position
                        ai_behavior.update_player_sighting(*player_pos, current_turn);
                        chase_score = 1.0;

                        info!(
                            "AI entity {:?} can see player at distance {:.1}, chase score: {:.2}",
                            actor_entity, distance, chase_score
                        );
                    }
                } else if let Some(last_known_pos) = ai_behavior.last_known_player_position {
                    // Player not visible, but AI remembers where they were
                    if !ai_behavior.should_switch_to_wander(current_turn) {
                        let distance = crate::utils::calculate_distance(*ai_pos, last_known_pos);
                        // Lower score for remembered positions
                        chase_score = 0.3 * (1.0 - (distance / ai_behavior.detection_range as f32));
                        chase_score = chase_score.clamp(0.0, 0.5);
                    }
                }
            }

            score.set(chase_score);
        }
    }
}

#[derive(Debug, Default, Component, Clone, ActionBuilder)]
pub struct ChaseActor {
    generated_path: bool,
    last_seen_pt: Option<Position>,
}

pub fn chase_action(
    mut current_map: ResMut<CurrentMap>,
    player_query: Query<&Position, With<PlayerTag>>,
    mut action_q: Query<(&Actor, &mut ActionState, &mut ChaseActor)>,
    mut mobs_q: Query<(&Position, &FieldOfView, &Name, &mut AIComponent)>,
) {
    use ActionState::*;

    let Ok(player_position) = player_query.single() else {
        info!("No player found!");
        return;
    };

    for (Actor(actor), mut action_state, mut chase) in action_q.iter_mut() {
        let Ok((&ai_position, fov, name, mut ai_component)) = mobs_q.get_mut(*actor) else {
            info!("Actor must have required components");
            continue;
        };

        if ai_component.preferred_action.is_some() {
            // already chasing, quick return;
            continue;
        }

        match *action_state {
            // Success | Failure
            Success | Failure => {
                info!("{} chase state: {:?}", name, action_state);
                ai_component.preferred_action = None;

                // if let Ok(mut target_visualizer) = target_q.get_mut(*actor) {
                //     target_visualizer.clear(&mut commands);
                // }

                continue;
            }
            Cancelled => {
                info!("{} cancelled chase!", name);
                *action_state = Failure;
                ai_component.preferred_action = None;

                // if let Ok(mut target_visualizer) = target_q.get_mut(*actor) {
                //     target_visualizer.clear(&mut commands);
                // }

                continue;
            }
            Init | Requested => {
                info!("{} gonna start chasing!", name);
                *action_state = Executing;

                chase.generated_path = false;
                chase.last_seen_pt = Some(*player_position);
                ai_component.preferred_action = Some(ActionType::Move(*player_position));

                // if let Ok(mut target_visualizer) = target_q.get_mut(*actor) {
                //     target_visualizer.set_color(Color::RED);
                //     target_visualizer.set_style(TargetVisualizerStyle::Target);
                // }
            }
            Executing => {}
        }

        info!("{} executing chase!", name);

        // Simple distance-based chase for now
        // TODO: Integrate with FOV system for proper line-of-sight checking
        let distance_to_player = ai_position.distance(player_position);

        let target_position = if distance_to_player <= 1.5 {
            // Close enough to attack
            *action_state = Success;
            continue;
        } else if distance_to_player <= fov.0 as f32 {
            // Can see player, chase directly
            chase.last_seen_pt = Some(*player_position);
            chase.generated_path = false;
            *player_position
        } else {
            // Lost sight of player, go to last known position
            let Some(last_seen) = chase.last_seen_pt else {
                error!("Executing chase with no target.");
                ai_component.preferred_action = Some(ActionType::Wait);
                continue;
            };

            // We reached the last seen position and still don't see the player
            if last_seen == ai_position {
                *action_state = Failure;
                continue;
            }

            // Use pathfinding to get to last seen position
            if !chase.generated_path {
                let pathfinder = PathFinder::default();
                if let Some(path) = pathfinder.compute(
                    (ai_position.x, ai_position.y),
                    (last_seen.x, last_seen.y),
                    0,     // movement type
                    false, // partial path on failure
                    &mut *current_map,
                ) {
                    if let Some(&(next_x, next_y)) = path.get(0) {
                        let next_step = Position::new(next_x, next_y);
                        chase.generated_path = true;
                        chase.last_seen_pt = Some(next_step);
                        next_step
                    } else {
                        // Path too short, go directly to target
                        last_seen
                    }
                } else {
                    // No path available, give up
                    *action_state = Failure;
                    continue;
                }
            } else {
                last_seen
            }
        };

        ai_component.preferred_action = Some(ActionType::Move(target_position));
    }
}
