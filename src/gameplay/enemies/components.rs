use bevy::prelude::*;
use big_brain::prelude::*;
use echos_assets::entities::AIBehaviorType;

use crate::core::{components::Position, types::ActionType};

#[derive(Reflect, Component, Default, Clone, Copy)]
#[reflect(Component)]
pub struct AIComponent {
    pub(crate) ai_type: AIBehaviorType,
    pub preferred_action: Option<ActionType>,
}

impl AIComponent {
    #[inline]
    pub const fn new(ai_type: AIBehaviorType) -> Self { Self { ai_type, preferred_action: None } }
}

impl AIComponent {
    pub const fn passive() -> Self { Self { ai_type: AIBehaviorType::Passive, preferred_action: None } }
    pub const fn hostile() -> Self { Self { ai_type: AIBehaviorType::Hostile, preferred_action: None } }
    pub const fn neutral() -> Self { Self { ai_type: AIBehaviorType::Neutral, preferred_action: None } }
}

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
    pub fn new(behavior_type: AIBehaviorType, detection_range: u8, turns_before_wander: u64) -> Self {
        Self {
            behavior_type,
            detection_range,
            turns_before_wander,
            last_known_player_position: None,
            last_player_seen_turn: None,
        }
    }

    pub fn hostile(detection_range: u8) -> Self { Self::new(AIBehaviorType::Hostile, detection_range, 5) }
    pub fn passive(detection_range: u8) -> Self { Self::new(AIBehaviorType::Passive, detection_range, 3) }
    pub fn neutral(detection_range: u8) -> Self { Self::new(AIBehaviorType::Neutral, detection_range, 10) }

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
