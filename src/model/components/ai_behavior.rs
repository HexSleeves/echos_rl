use bevy::prelude::*;
use big_brain::prelude::*;

/// Component that marks an entity as having AI behavior
#[derive(Component, Debug, Clone)]
pub struct AIBehavior {
    pub detection_range: i32,
    pub behavior_type: AIBehaviorType,
    pub last_known_player_position: Option<Position>,
}

/// Different types of AI behavior patterns
#[derive(Debug, Clone, PartialEq)]
pub enum AIBehaviorType {
    /// Hostile enemies that chase and attack the player
    Hostile,
    /// Passive entities that flee when threatened
    Passive,
    /// Neutral entities that ignore the player unless provoked
    Neutral,
}

impl Default for AIBehavior {
    fn default() -> Self {
        Self { behavior_type: AIBehaviorType::Neutral, detection_range: 8, last_known_player_position: None }
    }
}

impl AIBehavior {
    pub fn hostile(detection_range: i32) -> Self {
        Self { behavior_type: AIBehaviorType::Hostile, detection_range, last_known_player_position: None }
    }

    pub fn passive(detection_range: i32) -> Self {
        Self { behavior_type: AIBehaviorType::Passive, detection_range, last_known_player_position: None }
    }

    pub fn neutral() -> Self { Self::default() }
}

// ============================================================================
// BIG-BRAIN SCORERS (The "eyes" of the AI system)
// ============================================================================

/// Scorer that evaluates how much the AI wants to chase the player
#[derive(Component, Debug, Clone, ScorerBuilder)]
pub struct ChasePlayerScorer;

/// Scorer that evaluates how much the AI wants to flee from the player
#[derive(Component, Debug, Clone, ScorerBuilder)]
pub struct FleeFromPlayerScorer;

/// Scorer that evaluates how much the AI wants to wander randomly
#[derive(Component, Debug, Clone, ScorerBuilder)]
pub struct WanderScorer;

/// Scorer that evaluates if the AI can see the player
#[derive(Component, Debug, Clone, ScorerBuilder)]
pub struct PlayerVisibilityScorer;

// ============================================================================
// BIG-BRAIN ACTIONS (What the AI actually does)
// ============================================================================

/// Action for moving toward the player
#[derive(Component, Debug, Clone, ActionBuilder)]
pub struct ChasePlayerAction;

/// Action for fleeing from the player
#[derive(Component, Debug, Clone, ActionBuilder)]
pub struct FleeFromPlayerAction;

/// Action for wandering randomly
#[derive(Component, Debug, Clone, ActionBuilder)]
pub struct WanderAction;

/// Action for staying idle
#[derive(Component, Debug, Clone, ActionBuilder)]
pub struct IdleAction;

// ============================================================================
// HELPER COMPONENTS
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub enum AIAction {
    ChasePlayer,
    FleeFromPlayer,
    Wander,
    Idle,
}

/// Component to track AI state for debugging and behavior
#[derive(Component, Debug, Clone)]
pub struct AIState {
    pub current_action: AIAction,
    pub target_position: Option<Position>,
    pub last_action_time: f32,
}

impl Default for AIState {
    fn default() -> Self {
        Self { current_action: AIAction::Idle, target_position: None, last_action_time: 0.0 }
    }
}

use crate::model::components::Position;
