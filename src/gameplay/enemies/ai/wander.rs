use bevy::prelude::*;
use big_brain::prelude::*;
use brtk::random::Random;
use once_cell::sync::Lazy;
use rand::{
    RngCore,
    distr::{Distribution, Uniform},
    seq::IteratorRandom,
};

use crate::{
    core::{components::Position, resources::CurrentMap, types::ActionType},
    gameplay::enemies::components::AIComponent,
};

static WANDER_RANGE: Lazy<Uniform<u32>> = Lazy::new(|| match Uniform::new_inclusive(3, 10) {
    Ok(uniform) => uniform,
    Err(e) => {
        error!("Failed to create WANDER_RANGE: {}", e);
        Uniform::new_inclusive(3, 10).unwrap()
    }
});

#[derive(Debug, Reflect, Component, Clone, Eq, PartialEq)]
#[reflect(Component)]
pub struct Wander {
    destination: Option<Position>,
    my_previous_location: Position,
}

pub fn wander_action(
    mut commands: Commands,
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
