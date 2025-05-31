use bevy::prelude::*;
use big_brain::prelude::*;

/// Component that marks an entity as having AI behavior
#[derive(Component, Debug, Clone)]
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

    pub fn neutral() -> Self { Self::default() }

    /// Check if the AI should switch from chasing to wandering based on turns elapsed
    pub fn should_switch_to_wander(&self, current_turn: u64) -> bool {
        if let Some(last_seen) = self.last_player_seen_turn {
            current_turn.saturating_sub(last_seen) >= self.turns_before_wander
        } else {
            false
        }
    }

    /// Update the last seen turn when player is spotted
    pub fn mark_player_seen(&mut self, current_turn: u64, player_position: Position) {
        self.last_player_seen_turn = Some(current_turn);
        self.last_known_player_position = Some(player_position);
    }
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

use crate::core::components::Position;
