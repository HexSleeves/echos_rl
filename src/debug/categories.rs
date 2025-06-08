use serde::{Deserialize, Serialize};
use std::fmt;

/// Debug categories for different game systems
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DebugCategory {
    /// AI-related debugging (pathfinding, behavior, decision making)
    AI,
    /// Turn processing and queue management
    Turns,
    /// Combat systems and damage calculation
    Combat,
    /// World generation, map management, and terrain
    World,
    /// Input handling and player actions
    Input,
    /// Rendering, sprites, and visual systems
    Rendering,
    /// Performance metrics and optimization
    Performance,
    /// General game logic and miscellaneous systems
    General,
    /// State transitions
    StateTransitions,
}

impl DebugCategory {
    /// Get all available debug categories
    pub fn all() -> &'static [DebugCategory] {
        &[
            DebugCategory::AI,
            DebugCategory::Turns,
            DebugCategory::Combat,
            DebugCategory::World,
            DebugCategory::Input,
            DebugCategory::Rendering,
            DebugCategory::Performance,
            DebugCategory::General,
            DebugCategory::StateTransitions,
        ]
    }

    /// Get the display name for this category
    pub fn display_name(&self) -> &'static str {
        match self {
            DebugCategory::AI => "AI Systems",
            DebugCategory::Turns => "Turn Processing",
            DebugCategory::Combat => "Combat Systems",
            DebugCategory::World => "World & Map",
            DebugCategory::Input => "Input Handling",
            DebugCategory::Rendering => "Rendering",
            DebugCategory::Performance => "Performance",
            DebugCategory::General => "General",
            DebugCategory::StateTransitions => "State Transitions",
        }
    }

    /// Get the short name for this category (used in logs)
    pub fn short_name(&self) -> &'static str {
        match self {
            DebugCategory::AI => "ai",
            DebugCategory::Turns => "turns",
            DebugCategory::Combat => "combat",
            DebugCategory::World => "world",
            DebugCategory::Input => "input",
            DebugCategory::Rendering => "render",
            DebugCategory::Performance => "perf",
            DebugCategory::General => "general",
            DebugCategory::StateTransitions => "state_transitions",
        }
    }

    /// Get the environment variable name for this category
    pub fn env_var(&self) -> &'static str {
        match self {
            DebugCategory::AI => "DEBUG_AI",
            DebugCategory::Turns => "DEBUG_TURNS",
            DebugCategory::Combat => "DEBUG_COMBAT",
            DebugCategory::World => "DEBUG_WORLD",
            DebugCategory::Input => "DEBUG_INPUT",
            DebugCategory::Rendering => "DEBUG_RENDERING",
            DebugCategory::Performance => "DEBUG_PERFORMANCE",
            DebugCategory::General => "DEBUG_GENERAL",
            DebugCategory::StateTransitions => "DEBUG_STATE_TRANSITIONS",
        }
    }

    /// Check if this category is enabled via environment variable
    pub fn is_env_enabled(&self) -> bool { std::env::var(self.env_var()).is_ok() }
}

impl fmt::Display for DebugCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}", self.display_name()) }
}
