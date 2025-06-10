// Core ECS components for the roguelike game
//
// This module contains the fundamental components that form the foundation
// of the game's Entity-Component-System architecture.

pub mod position;
pub use position::*;

pub mod tag;
pub use tag::*;

mod health;
pub use health::*;

mod stats;
pub use stats::*;

mod inventory;
pub use inventory::*;

pub mod light;
pub use light::*;

// Re-export commonly used types for convenience
pub use bevy::prelude::{Component, Entity};

use bevy::prelude::*;

/// Description component for entities
#[derive(Component, Reflect, Default, Debug, Clone, Deref, DerefMut)]
#[reflect(Component)]
pub struct Description(pub String);

impl Description {
    pub fn new(description: impl ToString) -> Self { Self(description.to_string()) }
}

#[derive(Reflect, Component, Deref, DerefMut)]
#[reflect(Component)]
pub struct FieldOfView(pub u8);

impl Default for FieldOfView {
    fn default() -> Self { Self(4) }
}

impl FieldOfView {
    pub fn new(radius: u8) -> Self { Self(radius) }
}

/// Bundle containing the most basic components every game entity should have
#[derive(Bundle)]
pub struct BaseEntityBundle {
    pub position: Position,
    pub health: Health,
    pub stats: Stats,
}

impl BaseEntityBundle {
    /// Create a new base entity bundle with default values
    pub fn new(position: Position) -> Self {
        Self { position, health: Health::default(), stats: Stats::default() }
    }

    /// Create a new base entity bundle with custom health and stats
    pub fn new_with_stats(position: Position, health: Health, stats: Stats) -> Self {
        Self { position, health, stats }
    }
}

/// Bundle for player entities - composes BaseEntityBundle with player-specific components
#[derive(Bundle)]
pub struct PlayerBundle {
    /// Core entity components (position, health, stats)
    pub base: BaseEntityBundle,
    /// Player identification tag
    pub player_tag: PlayerTag,
    /// Player inventory system
    pub inventory: Inventory,
}

impl PlayerBundle {
    /// Create a new player bundle at the specified position
    pub fn new(position: Position) -> Self {
        Self {
            base: BaseEntityBundle::new_with_stats(
                position,
                Health::new(100),    // Players start with 100 health
                Stats::balanced(12), // Balanced stats at 12
            ),
            player_tag: PlayerTag,
            inventory: Inventory::new(30, 150.0), // 30 slots, 150 weight capacity
        }
    }

    /// Create a new player bundle with custom stats
    pub fn new_with_stats(position: Position, health: Health, stats: Stats) -> Self {
        Self {
            player_tag: PlayerTag,
            inventory: Inventory::new(30, 150.0),
            base: BaseEntityBundle::new_with_stats(position, health, stats),
        }
    }
}

/// Bundle for enemy entities - composes BaseEntityBundle with AI tag
#[derive(Bundle)]
pub struct EnemyBundle {
    /// Core entity components (position, health, stats)
    pub base: BaseEntityBundle,
    /// AI identification tag
    pub ai_tag: AITag,
}

impl EnemyBundle {
    /// Create a new enemy bundle at the specified position
    pub fn new(position: Position) -> Self { Self { base: BaseEntityBundle::new(position), ai_tag: AITag } }

    /// Create a new enemy bundle with custom health and stats
    pub fn new_with_stats(position: Position, health: Health, stats: Stats) -> Self {
        Self { base: BaseEntityBundle::new_with_stats(position, health, stats), ai_tag: AITag }
    }

    /// Create a warrior-type enemy
    pub fn warrior(position: Position) -> Self {
        Self::new_with_stats(position, Health::new(80), Stats::warrior())
    }

    /// Create a mage-type enemy
    pub fn mage(position: Position) -> Self { Self::new_with_stats(position, Health::new(60), Stats::mage()) }

    /// Create a rogue-type enemy
    pub fn rogue(position: Position) -> Self {
        Self::new_with_stats(position, Health::new(70), Stats::rogue())
    }
}
