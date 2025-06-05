use bevy::prelude::*;
use std::fmt;

use crate::core::{components::Position, types::GameAction};

/// Comprehensive error type for all game operations
#[derive(Debug)]
pub enum GameError {
    // Action-related errors
    /// Action should be retried with a different action
    Retry(Box<dyn GameAction>),
    /// Action target is invalid or no longer exists
    InvalidTarget,
    /// Action is blocked by game state or conditions
    ActionBlocked,
    /// Not enough resources (mana, stamina, items, etc.)
    InsufficientResources,
    /// Action is on cooldown
    OnCooldown { remaining_time: u32 },
    /// Action requires a different game state
    InvalidGameState { required: String, current: String },

    // Entity-related errors
    /// Entity doesn't exist in the world
    EntityNotFound(Entity),
    /// Entity is missing required components
    MissingComponent { entity: Entity, component: &'static str },
    /// Entity is dead or inactive
    EntityDead(Entity),
    /// Entity doesn't have permission for this action
    InsufficientPermissions(Entity),

    // Movement-related errors
    /// Target position is blocked or invalid
    MovementBlocked { from: Position, to: Position, reason: String },
    /// Path to target cannot be found
    NoPathFound { from: Position, to: Position },
    /// Movement range exceeded
    OutOfRange { distance: f32, max_range: f32 },
    /// Terrain is impassable
    ImpassableTerrain(Position),

    // Combat-related errors
    /// Target is out of attack range
    AttackOutOfRange { distance: f32, weapon_range: f32 },
    /// No valid targets for the attack
    NoValidTargets,
    /// Weapon or attack method is invalid
    InvalidWeapon(String),
    /// Attack was dodged, blocked, or missed
    AttackMissed { attacker: Entity, target: Entity },
    /// Friendly fire prevention
    FriendlyFire { attacker: Entity, target: Entity },

    // Spell/Magic-related errors
    /// Not enough mana to cast spell
    InsufficientMana { required: u32, available: u32 },
    /// Spell is unknown or not learned
    UnknownSpell(String),
    /// Spell components are missing
    MissingSpellComponents(Vec<String>),
    /// Spell target is invalid
    InvalidSpellTarget,
    /// Magic is suppressed or blocked
    MagicSuppressed,
    /// Spell fizzled or failed to cast
    SpellFizzled { spell: String, reason: String },

    // Item-related errors
    /// Item doesn't exist or is invalid
    InvalidItem(String),
    /// Item is not in inventory
    ItemNotFound(String),
    /// Not enough quantity of item
    InsufficientQuantity { required: u32, available: u32 },
    /// Item cannot be used in current context
    ItemNotUsable(String),
    /// Inventory is full
    InventoryFull,
    /// Item is broken or needs repair
    ItemBroken(String),

    // Turn/Queue-related errors
    /// Turn queue is at maximum capacity
    QueueFull,
    /// Entity is not in the turn queue
    NotInQueue(Entity),
    /// Turn processing is paused or disabled
    TurnProcessingDisabled,
    /// Invalid turn timing
    InvalidTiming { requested: u32, minimum: u32 },

    // World/Environment errors
    /// Position is outside world bounds
    OutOfBounds(Position),
    /// Area or zone is restricted
    RestrictedArea(Position),
    /// Environmental hazard prevents action
    EnvironmentalHazard { position: Position, hazard: String },
    /// Weather or time conditions prevent action
    WeatherRestriction(String),

    // Resource/System errors
    /// System resource is unavailable
    ResourceUnavailable(String),
    /// Operation timed out
    Timeout { operation: String, duration: std::time::Duration },
    /// Concurrent modification detected
    ConcurrentModification,
    /// System is in an invalid state
    InvalidSystemState(String),

    // Network/Multiplayer errors
    /// Network connection issue
    NetworkError(String),
    /// Player is not authorized for this action
    Unauthorized,
    /// Action conflicts with another player's action
    ActionConflict { player1: Entity, player2: Entity },
    /// Server is overloaded
    ServerOverloaded,

    // Save/Load errors
    /// Failed to save game state
    SaveFailed(String),
    /// Failed to load game state
    LoadFailed(String),
    /// Save data is corrupted
    CorruptedData(String),
    /// Version mismatch in save data
    VersionMismatch { expected: String, found: String },

    // Configuration errors
    /// Invalid configuration value
    InvalidConfig { key: String, value: String },
    /// Required configuration is missing
    MissingConfig(String),

    // Custom/Mod errors
    /// Custom error from mods or plugins
    Custom(String),
    /// Multiple errors occurred
    Multiple(Vec<GameError>),
}

impl GameError {
    /// Check if this error should trigger a retry
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            GameError::Retry(_)
                | GameError::ActionBlocked
                | GameError::OnCooldown { .. }
                | GameError::MovementBlocked { .. }
                | GameError::AttackMissed { .. }
                | GameError::SpellFizzled { .. }
                | GameError::ConcurrentModification
                | GameError::NetworkError(_)
                | GameError::ServerOverloaded
        )
    }

    /// Check if this error is fatal and should stop processing
    pub fn is_fatal(&self) -> bool {
        matches!(
            self,
            GameError::EntityNotFound(_)
                | GameError::EntityDead(_)
                | GameError::MissingComponent { .. }
                | GameError::InvalidSystemState(_)
                | GameError::CorruptedData(_)
                | GameError::VersionMismatch { .. }
        )
    }

    /// Check if this error should be logged as a warning vs error
    pub fn is_warning(&self) -> bool {
        matches!(
            self,
            GameError::InvalidTarget
                | GameError::InsufficientResources
                | GameError::OnCooldown { .. }
                | GameError::OutOfRange { .. }
                | GameError::AttackMissed { .. }
                | GameError::NoValidTargets
                | GameError::ItemNotUsable(_)
                | GameError::InventoryFull
        )
    }

    /// Get a user-friendly error message
    pub fn user_message(&self) -> String {
        match self {
            GameError::Retry(_) => "Action needs to be retried".to_string(),
            GameError::InvalidTarget => "Invalid target selected".to_string(),
            GameError::ActionBlocked => "Action is currently blocked".to_string(),
            GameError::InsufficientResources => "Not enough resources".to_string(),
            GameError::OnCooldown { remaining_time } => {
                format!("Action on cooldown for {remaining_time} more time units")
            }
            GameError::InvalidGameState { required, current } => {
                format!("Wrong game state. Need: {required}, Current: {current}")
            }

            GameError::EntityNotFound(_) => "Entity not found".to_string(),
            GameError::MissingComponent { component, .. } => {
                format!("Missing required component: {component}")
            }
            GameError::EntityDead(_) => "Entity is dead".to_string(),
            GameError::InsufficientPermissions(_) => "Insufficient permissions".to_string(),

            GameError::MovementBlocked { reason, .. } => {
                format!("Movement blocked: {reason}")
            }
            GameError::NoPathFound { .. } => "No path to target found".to_string(),
            GameError::OutOfRange { distance, max_range } => {
                format!("Out of range: {distance:.1} > {max_range:.1}")
            }
            GameError::ImpassableTerrain(_) => "Terrain is impassable".to_string(),

            GameError::AttackOutOfRange { distance, weapon_range } => {
                format!("Attack out of range: {distance:.1} > {weapon_range:.1}")
            }
            GameError::NoValidTargets => "No valid targets".to_string(),
            GameError::InvalidWeapon(weapon) => format!("Invalid weapon: {weapon}"),
            GameError::AttackMissed { .. } => "Attack missed".to_string(),
            GameError::FriendlyFire { .. } => "Cannot attack friendly target".to_string(),

            GameError::InsufficientMana { required, available } => {
                format!("Not enough mana: need {required}, have {available}")
            }
            GameError::UnknownSpell(spell) => format!("Unknown spell: {spell}"),
            GameError::MissingSpellComponents(components) => {
                format!("Missing spell components: {}", components.join(", "))
            }
            GameError::InvalidSpellTarget => "Invalid spell target".to_string(),
            GameError::MagicSuppressed => "Magic is suppressed".to_string(),
            GameError::SpellFizzled { spell, reason } => {
                format!("Spell '{spell}' fizzled: {reason}")
            }

            GameError::InvalidItem(item) => format!("Invalid item: {item}"),
            GameError::ItemNotFound(item) => format!("Item not found: {item}"),
            GameError::InsufficientQuantity { required, available } => {
                format!("Not enough items: need {required}, have {available}")
            }
            GameError::ItemNotUsable(item) => format!("Item not usable: {item}"),
            GameError::InventoryFull => "Inventory is full".to_string(),
            GameError::ItemBroken(item) => format!("Item is broken: {item}"),

            GameError::QueueFull => "Turn queue is full".to_string(),
            GameError::NotInQueue(_) => "Entity not in turn queue".to_string(),
            GameError::TurnProcessingDisabled => "Turn processing is disabled".to_string(),
            GameError::InvalidTiming { requested, minimum } => {
                format!("Invalid timing: {requested} < {minimum}")
            }

            GameError::OutOfBounds(_) => "Position out of bounds".to_string(),
            GameError::RestrictedArea(_) => "Area is restricted".to_string(),
            GameError::EnvironmentalHazard { hazard, .. } => {
                format!("Environmental hazard: {hazard}")
            }
            GameError::WeatherRestriction(weather) => {
                format!("Weather restriction: {weather}")
            }

            GameError::ResourceUnavailable(resource) => {
                format!("Resource unavailable: {resource}")
            }
            GameError::Timeout { operation, duration } => {
                format!("Operation '{operation}' timed out after {duration:?}")
            }
            GameError::ConcurrentModification => "Concurrent modification detected".to_string(),
            GameError::InvalidSystemState(state) => {
                format!("Invalid system state: {state}")
            }

            GameError::NetworkError(msg) => format!("Network error: {msg}"),
            GameError::Unauthorized => "Unauthorized action".to_string(),
            GameError::ActionConflict { .. } => "Action conflicts with another player".to_string(),
            GameError::ServerOverloaded => "Server is overloaded".to_string(),

            GameError::SaveFailed(msg) => format!("Save failed: {msg}"),
            GameError::LoadFailed(msg) => format!("Load failed: {msg}"),
            GameError::CorruptedData(msg) => format!("Corrupted data: {msg}"),
            GameError::VersionMismatch { expected, found } => {
                format!("Version mismatch: expected {expected}, found {found}")
            }

            GameError::InvalidConfig { key, value } => {
                format!("Invalid config '{key}': {value}")
            }
            GameError::MissingConfig(key) => format!("Missing config: {key}"),

            GameError::Custom(msg) => msg.clone(),
            GameError::Multiple(errors) => {
                format!(
                    "Multiple errors: {}",
                    errors.iter().map(|e| e.user_message()).collect::<Vec<_>>().join("; ")
                )
            }
        }
    }

    /// Get the error category for logging/metrics
    pub fn category(&self) -> &'static str {
        match self {
            GameError::Retry(_)
            | GameError::InvalidTarget
            | GameError::ActionBlocked
            | GameError::InsufficientResources
            | GameError::OnCooldown { .. }
            | GameError::InvalidGameState { .. } => "Action",

            GameError::EntityNotFound(_)
            | GameError::MissingComponent { .. }
            | GameError::EntityDead(_)
            | GameError::InsufficientPermissions(_) => "Entity",

            GameError::MovementBlocked { .. }
            | GameError::NoPathFound { .. }
            | GameError::OutOfRange { .. }
            | GameError::ImpassableTerrain(_) => "Movement",

            GameError::AttackOutOfRange { .. }
            | GameError::NoValidTargets
            | GameError::InvalidWeapon(_)
            | GameError::AttackMissed { .. }
            | GameError::FriendlyFire { .. } => "Combat",

            GameError::InsufficientMana { .. }
            | GameError::UnknownSpell(_)
            | GameError::MissingSpellComponents(_)
            | GameError::InvalidSpellTarget
            | GameError::MagicSuppressed
            | GameError::SpellFizzled { .. } => "Magic",

            GameError::InvalidItem(_)
            | GameError::ItemNotFound(_)
            | GameError::InsufficientQuantity { .. }
            | GameError::ItemNotUsable(_)
            | GameError::InventoryFull
            | GameError::ItemBroken(_) => "Item",

            GameError::QueueFull
            | GameError::NotInQueue(_)
            | GameError::TurnProcessingDisabled
            | GameError::InvalidTiming { .. } => "Turn",

            GameError::OutOfBounds(_)
            | GameError::RestrictedArea(_)
            | GameError::EnvironmentalHazard { .. }
            | GameError::WeatherRestriction(_) => "Environment",

            GameError::ResourceUnavailable(_)
            | GameError::Timeout { .. }
            | GameError::ConcurrentModification
            | GameError::InvalidSystemState(_) => "System",

            GameError::NetworkError(_)
            | GameError::Unauthorized
            | GameError::ActionConflict { .. }
            | GameError::ServerOverloaded => "Network",

            GameError::SaveFailed(_)
            | GameError::LoadFailed(_)
            | GameError::CorruptedData(_)
            | GameError::VersionMismatch { .. } => "Persistence",

            GameError::InvalidConfig { .. } | GameError::MissingConfig(_) => "Configuration",

            GameError::Custom(_) => "Custom",
            GameError::Multiple(_) => "Multiple",
        }
    }

    /// Create a retry error with a new action
    pub fn retry_with(action: Box<dyn GameAction>) -> Self { GameError::Retry(action) }

    /// Create an insufficient resources error
    pub fn insufficient_resources() -> Self { GameError::InsufficientResources }

    /// Create a cooldown error
    pub fn on_cooldown(remaining_time: u32) -> Self { GameError::OnCooldown { remaining_time } }

    /// Create an out of range error
    pub fn out_of_range(distance: f32, max_range: f32) -> Self {
        GameError::OutOfRange { distance, max_range }
    }

    /// Create an insufficient mana error
    pub fn insufficient_mana(required: u32, available: u32) -> Self {
        GameError::InsufficientMana { required, available }
    }

    /// Create a missing component error
    pub fn missing_component(entity: Entity, component: &'static str) -> Self {
        GameError::MissingComponent { entity, component }
    }

    /// Combine multiple errors into one
    pub fn multiple(errors: Vec<GameError>) -> Self {
        if errors.len() == 1 { errors.into_iter().next().unwrap() } else { GameError::Multiple(errors) }
    }
}

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}", self.user_message()) }
}

impl std::error::Error for GameError {}

// Conversion from common error types
impl From<String> for GameError {
    fn from(msg: String) -> Self { GameError::Custom(msg) }
}

impl From<&str> for GameError {
    fn from(msg: &str) -> Self { GameError::Custom(msg.to_string()) }
}

// Result type alias for convenience
pub type GameResult<T> = Result<T, GameError>;

// Convenience macros for creating common errors
#[macro_export]
macro_rules! game_error {
    ($msg:expr) => {
        GameError::Custom($msg.to_string())
    };
    ($fmt:expr, $($arg:tt)*) => {
        GameError::Custom(format!($fmt, $($arg)*))
    };
}

#[macro_export]
macro_rules! insufficient_resources {
    () => {
        GameError::InsufficientResources
    };
}

#[macro_export]
macro_rules! invalid_target {
    () => {
        GameError::InvalidTarget
    };
}

#[macro_export]
macro_rules! entity_not_found {
    ($entity:expr) => {
        GameError::EntityNotFound($entity)
    };
}
