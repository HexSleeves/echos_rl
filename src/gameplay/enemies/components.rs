use bevy::prelude::*;
use big_brain::prelude::*;

use crate::core::components::Position;

/// Component that marks an entity as having AI behavior
#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct AIBehavior {
    pub detection_range: u8,
    pub behavior_type: AIBehaviorType,
    pub last_known_player_position: Option<Position>,
    /// Turn number when the player was last seen
    pub last_player_seen_turn: Option<u64>,
    /// Number of turns to wait before switching from chase to wander
    pub turns_before_wander: u64,
}

/// Different types of AI behavior patterns
#[derive(Debug, Clone, PartialEq, Reflect)]
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
        Self {
            behavior_type: AIBehaviorType::Neutral,
            detection_range: 8,
            last_known_player_position: None,
            last_player_seen_turn: None,
            turns_before_wander: 5, // Default to 5 turns
        }
    }
}

impl AIBehavior {
    pub fn hostile(detection_range: u8) -> Self {
        Self {
            behavior_type: AIBehaviorType::Hostile,
            detection_range,
            last_known_player_position: None,
            last_player_seen_turn: None,
            turns_before_wander: 5, // Hostile enemies give up chase after 5 turns
        }
    }

    pub fn passive(detection_range: u8) -> Self {
        Self {
            behavior_type: AIBehaviorType::Passive,
            detection_range,
            last_known_player_position: None,
            last_player_seen_turn: None,
            turns_before_wander: 3, // Passive enemies give up faster
        }
    }

    pub fn neutral(detection_range: u8) -> Self {
        Self {
            behavior_type: AIBehaviorType::Neutral,
            detection_range,
            last_known_player_position: None,
            last_player_seen_turn: None,
            turns_before_wander: 10, // Neutral entities don't really chase
        }
    }

    /// Check if the AI should switch from chase to wander behavior
    pub fn should_switch_to_wander(&self, current_turn: u64) -> bool {
        if let Some(last_seen) = self.last_player_seen_turn {
            current_turn.saturating_sub(last_seen) >= self.turns_before_wander
        } else {
            true // Never seen player, should wander
        }
    }

    /// Update the last known player position and turn
    pub fn update_player_sighting(&mut self, position: Position, turn: u64) {
        self.last_known_player_position = Some(position);
        self.last_player_seen_turn = Some(turn);
    }
}

/// Component that marks an entity as AI-controlled
#[derive(Component, Debug)]
pub struct AITag;

// ============================================================================
// BIG-BRAIN SCORERS (How the AI evaluates what to do)
// ============================================================================

/// Scorer that evaluates if the AI should chase the player
#[derive(Component, Debug, Clone, ScorerBuilder)]
pub struct ChasePlayerScorer;

/// Scorer that evaluates if the AI should flee from the player
#[derive(Component, Debug, Clone, ScorerBuilder)]
pub struct FleeFromPlayerScorer;

/// Scorer that evaluates if the AI should wander randomly
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

#[derive(Debug, Clone, PartialEq, Reflect)]
pub enum AIAction {
    ChasePlayer,
    FleeFromPlayer,
    Wander,
    Idle,
}

/// Component to track AI state for debugging and behavior
#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
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
