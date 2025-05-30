use bevy::prelude::*;
use big_brain::prelude::*;
use std::f32;

use crate::model::{
    components::{AIBehavior, PlayerTag, PlayerVisibilityScorer, Position, TurnActor},
    resources::FovMap,
};

// ============================================================================
// SCORER SYSTEMS (AI "Eyes" - Evaluate the world and assign scores)
// ============================================================================

/// System that scores player visibility (used for decision making)
pub fn player_visibility_scorer_system(
    fov_map: Res<FovMap>,
    player_query: Query<&Position, With<PlayerTag>>,
    mut scorer_query: Query<(&Actor, &mut Score), With<PlayerVisibilityScorer>>,
    ai_query: Query<(&Position, &AIBehavior, &TurnActor)>,
) {
    let Ok(player_pos) = player_query.single() else {
        return;
    };

    for (Actor(actor_entity), mut score) in scorer_query.iter_mut() {
        if let Ok((ai_pos, ai_behavior, turn_actor)) = ai_query.get(*actor_entity) {
            // Skip scoring if this entity already has an action queued
            if turn_actor.peak_next_action().is_some() {
                continue;
            }

            let mut visibility_score = 0.0;

            // Check if AI can see the player
            if fov_map.is_visible(*player_pos) {
                let distance = crate::utils::calculate_distance(*ai_pos, *player_pos);

                if distance <= ai_behavior.detection_range as f32 {
                    visibility_score = 1.0;
                }
            }

            score.set(visibility_score);
        }
    }
}
