use bevy::prelude::*;
use big_brain::prelude::*;
use echos_assets::entities::AIBehaviorType;

use crate::core::{actions::SwipePattern, components::Position};

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

    /// Complete A* path to target (first element is current position)
    pub(crate) current_path: Vec<Position>,
    /// Current index in the path (0 = current position, 1 = next step)
    pub(crate) path_index: usize,
    /// Target position when path was generated (for regeneration detection)
    pub(crate) target_when_path_generated: Option<Position>,
    /// AI position when path was generated (for regeneration detection)
    pub(crate) ai_pos_when_path_generated: Option<Position>,
}

/// Action for fleeing from the player
#[derive(Component, Debug, Clone, ActionBuilder, Default)]
pub struct FleeFromPlayerAction {
    /// Complete A* escape path (first element is current position)
    pub(crate) escape_path: Vec<Position>,
    /// Current index in the escape path (0 = current position, 1 = next step)
    pub(crate) path_index: usize,
    /// Final escape destination when path was generated
    pub(crate) escape_target: Option<Position>,
    /// Threat position when path was generated (for regeneration detection)
    pub(crate) threat_pos_when_path_generated: Option<Position>,
    /// AI position when path was generated (for regeneration detection)
    pub(crate) ai_pos_when_path_generated: Option<Position>,
}

/// Action for wandering randomly
#[derive(Component, Debug, Clone, ActionBuilder, Default)]
pub struct WanderAction {
    /// Type of wandering behavior
    pub wander_type: WanderType,
    /// Current A* path for wandering
    pub current_path: Vec<Position>,
    /// Current index in the path (0 = current position, 1 = next step)
    pub path_index: usize,
    /// Current target position
    pub current_target: Option<Position>,
    /// AI position when path was generated (for regeneration detection)
    pub ai_pos_when_path_generated: Option<Position>,
    /// For patrol: list of patrol points
    pub patrol_points: Vec<Position>,
    /// For patrol: current patrol point index
    pub current_patrol_index: usize,
    /// For area wander: preferred area bounds
    pub wander_area: Option<WanderArea>,
    /// Time when last target was set (for target refresh)
    pub last_target_time: Option<u64>,
}

/// Action for staying idle
#[derive(Component, Debug, Clone, ActionBuilder)]
pub struct IdleAction;

#[derive(Component, Debug, Clone, ActionBuilder, Default)]
pub struct AttackAction;

/// Component for AI entities that can perform swipe attacks
#[derive(Component, Clone, Debug)]
pub struct SwipeAttacker {
    /// The swipe pattern this entity uses
    pub pattern: SwipePattern,
    /// Chance to use swipe attack instead of regular attack (0.0 to 1.0)
    pub swipe_chance: f32,
    /// Minimum number of targets required to trigger swipe attack
    pub min_targets_for_swipe: usize,
}

impl SwipeAttacker {
    pub fn new(pattern: SwipePattern, swipe_chance: f32, min_targets_for_swipe: usize) -> Self {
        Self {
            pattern,
            swipe_chance: swipe_chance.clamp(0.0, 1.0),
            min_targets_for_swipe: min_targets_for_swipe.max(1),
        }
    }

    /// Create a horizontal swiper (good for hallway combat)
    pub fn horizontal_swiper() -> Self { Self::new(SwipePattern::Horizontal, 0.3, 1) }

    /// Create an arc swiper (good for crowd control)
    pub fn arc_swiper() -> Self { Self::new(SwipePattern::Arc, 0.4, 2) }

    /// Create a berserker that attacks everything adjacent
    pub fn berserker() -> Self { Self::new(SwipePattern::AllAdjacent, 0.6, 2) }

    /// Create a diagonal fighter (prefers diagonal attacks)
    pub fn diagonal_fighter() -> Self { Self::new(SwipePattern::Diagonal, 0.5, 1) }
}

/// Action for AI entities that can swipe attack
#[derive(Component, Clone, Debug, ActionBuilder)]
pub struct SwipeAttackPlayerAction {
    /// The swipe pattern to use
    pub pattern: SwipePattern,
    /// Whether this action has been initialized
    pub initialized: bool,
}

impl SwipeAttackPlayerAction {
    pub fn new(pattern: SwipePattern) -> Self { Self { pattern, initialized: false } }
}

// ============================================================================
// HELPER COMPONENTS
// ============================================================================

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
        Self {
            behavior_type,
            detection_range,
            last_player_seen_turn: None,
            last_known_player_position: None,
            turns_before_wander: turns_before_wander.saturating_mul(1000),
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
    pub(crate) ai_type: AIBehaviorType,
    pub(crate) ai_behavior: AIBehavior,
}

impl AIComponent {
    pub fn new(ai_type: AIBehaviorType) -> Self { Self { ai_type, ai_behavior: AIBehavior::default() } }
}

impl AIComponent {
    pub fn passive() -> Self {
        Self { ai_type: AIBehaviorType::Passive, ai_behavior: AIBehavior::passive(8) }
    }
    pub fn hostile() -> Self {
        Self { ai_type: AIBehaviorType::Hostile, ai_behavior: AIBehavior::hostile(8) }
    }
    pub fn neutral() -> Self {
        Self { ai_type: AIBehaviorType::Neutral, ai_behavior: AIBehavior::neutral(8) }
    }
}

/// Different types of wandering behavior
#[derive(Debug, Clone, PartialEq, Default)]
pub enum WanderType {
    #[default]
    Random, // Simple random movement (current behavior)
    AreaWander, // Wander within a specific area
    Patrol,     // Move between specific patrol points
    Explore,    // Seek unexplored areas
}

/// Area bounds for area-constrained wandering
#[derive(Debug, Clone)]
pub struct WanderArea {
    pub center: Position,
    pub radius: u32,
}
