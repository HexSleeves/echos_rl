use bevy::prelude::*;
use big_brain::prelude::*;
use echos_assets::entities::AIBehaviorType;

use crate::core::components::Position;

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
#[derive(Component, Debug, Clone, ActionBuilder, Default)]
pub struct ChasePlayerAction {
    pub(crate) generated_path: bool,
    pub(crate) last_seen_pt: Option<Position>,
}

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

#[derive(Debug, Clone, PartialEq, Reflect, Copy)]
pub enum AIAction {
    ChasePlayer,
    FleeFromPlayer,
    Wander,
    Idle,
}

/// Component to track AI state for debugging and behavior
#[derive(Component, Debug, Clone, Reflect, Copy)]
#[reflect(Component)]
pub struct AIState {
    pub last_action_time: f32,
    pub current_action: Option<AIAction>,
    pub target_position: Option<Position>,
}

impl Default for AIState {
    fn default() -> Self { Self { current_action: None, target_position: None, last_action_time: 0.0 } }
}

/// Component that marks an entity as having AI behavior
#[derive(Component, Debug, Clone, Reflect, Copy)]
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
        info!("AI behavior new: {:?}", behavior_type);

        Self {
            behavior_type,
            detection_range,
            last_player_seen_turn: None,
            last_known_player_position: None,
            turns_before_wander: turns_before_wander * 1000,
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

#[derive(Reflect, Component, Default, Clone, Copy)]
#[reflect(Component)]
pub struct AIComponent {
    pub(crate) state: AIState,
    pub(crate) ai_type: AIBehaviorType,
    pub(crate) ai_behavior: AIBehavior,
}

impl AIComponent {
    pub fn new(ai_type: AIBehaviorType) -> Self {
        Self { state: AIState::default(), ai_type, ai_behavior: AIBehavior::default() }
    }
}

impl AIComponent {
    pub fn passive() -> Self {
        Self {
            state: AIState::default(),
            ai_type: AIBehaviorType::Passive,
            ai_behavior: AIBehavior::passive(8),
        }
    }
    pub fn hostile() -> Self {
        Self {
            state: AIState::default(),
            ai_type: AIBehaviorType::Hostile,
            ai_behavior: AIBehavior::hostile(8),
        }
    }
    pub fn neutral() -> Self {
        Self {
            state: AIState::default(),
            ai_type: AIBehaviorType::Neutral,
            ai_behavior: AIBehavior::neutral(8),
        }
    }
}
