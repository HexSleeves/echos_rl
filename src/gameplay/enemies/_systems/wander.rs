use bevy::prelude::*;
use big_brain::prelude::*;

use echos_assets::entities::AIBehaviorType;
use once_cell::sync::Lazy;
use rand::{
    RngCore,
    distr::{Distribution, Uniform},
    seq::IteratorRandom,
};

use crate::{
    core::{
        components::Position,
        resources::{CurrentMap, TurnQueue},
        types::ActionType,
    },
    gameplay::enemies::components::{AIBehavior, AIComponent, WanderScorer},
};
use brtk::random::Random;

/// Scorer system for WanderScorer components
pub fn wander_scorer_system(
    turn_queue: Res<TurnQueue>,
    ai_query: Query<&AIBehavior>,
    mut scorer_query: Query<(&Actor, &mut Score), With<WanderScorer>>,
) {
    println!("Wander scorer system x1");
    let current_turn = turn_queue.current_time();

    for (Actor(actor_entity), mut score) in scorer_query.iter_mut() {
        println!("Wander scorer system x2");

        if let Ok(ai_behavior) = ai_query.get(*actor_entity) {
            println!("Wander scorer system x3");
            let wander_score = match ai_behavior.behavior_type {
                AIBehaviorType::Neutral => {
                    // Neutral entities should always want to wander
                    0.2 + fastrand::f32() * 0.3 // 0.2 to 0.5 - always above threshold
                }
                AIBehaviorType::Hostile => {
                    if ai_behavior.should_switch_to_wander(current_turn) {
                        // Hostile enemy hasn't seen player for a while, switch to wandering
                        info!(
                            "Hostile AI entity {:?} switching to wander (no player seen for {} turns)",
                            actor_entity,
                            current_turn.saturating_sub(ai_behavior.last_player_seen_turn.unwrap_or(0))
                        );
                        0.7 // High score to override chase behavior
                    } else {
                        // Low wander score when actively hostile
                        0.15 + fastrand::f32() * 0.1 // 0.15 to 0.25
                    }
                }
                AIBehaviorType::Passive => {
                    // Passive entities wander when not fleeing
                    0.3 + fastrand::f32() * 0.2 // 0.3 to 0.5
                }
            };

            score.set(wander_score);
        }
    }
}

static WANDER_RANGE: Lazy<Uniform<u32>> = Lazy::new(|| match Uniform::new_inclusive(3, 10) {
    Ok(uniform) => uniform,
    Err(e) => {
        error!("Failed to create WANDER_RANGE: {}", e);
        Uniform::new_inclusive(3, 10).unwrap()
    }
});

#[derive(Debug, Reflect, Component, Clone, Eq, PartialEq, ActionBuilder)]
#[reflect(Component)]
pub struct Wander {
    destination: Option<Position>,
    my_previous_location: Position,
}

impl Default for Wander {
    fn default() -> Self { Self { destination: None, my_previous_location: Position::new(0, 0) } }
}

pub fn wander_action(
    mut map: ResMut<CurrentMap>,
    mut shared_rng: ResMut<Random>,
    mut action_q: Query<(&Actor, &mut ActionState, &mut Wander)>,
    mut spatial_q: Query<(&Name, &Position, &mut AIComponent)>,
) {
    use ActionState::*;

    let mut rng = shared_rng.as_rngcore();

    for (Actor(actor), mut action_state, mut wander) in action_q.iter_mut() {
        let Ok((name, ai_position, mut ai_component)) = spatial_q.get_mut(*actor) else {
            info!("Actor must have spatial components");
            return;
        };

        if ai_component.preferred_action.is_some() {
            // already wandering, quick return;
            return;
        }

        match *action_state {
            // Success | Failure
            Success | Failure => {
                // Nothing to do here
                info!("{} wander state: {:?}", name, action_state);
                return;
            }
            Cancelled => {
                info!("{} cancelled wander", name);
                ai_component.preferred_action = None;
                *action_state = Failure;

                return;
            }

            // These two states will fall through to execution
            Init | Requested => {
                info!("{} gonna start wandering!", name);
                *action_state = Executing;
            }
            Executing => {}
        }

        info!("{} executing wander!", name);

        let destination = match std::mem::take(&mut wander.destination) {
            Some(destination) => {
                if ai_position.distance(&destination) <= 1.0 {
                    generate_wander_path(&mut rng, &mut map, *ai_position)
                } else {
                    destination
                }
            }
            None => generate_wander_path(&mut rng, &mut map, *ai_position),
        };

        wander.destination = Some(destination);
        wander.my_previous_location = *ai_position;
        ai_component.preferred_action = Some(ActionType::Move(destination));
    }
}

fn generate_wander_path(rng: &mut impl RngCore, map: &mut CurrentMap, ai_pos: Position) -> Position {
    use brtk::grid_shapes::{Circle, ShapeIter};

    let wander_radius = WANDER_RANGE.sample(rng);
    let wander_circle = Circle::new((0, 0), wander_radius);

    loop {
        // Default to the circle center
        let offset = wander_circle.iter().choose(rng).unwrap_or_else(|| wander_circle.center());
        let destination = ai_pos + offset;
        if map.can_place_actor(destination) {
            return destination;
        }
    }
}
