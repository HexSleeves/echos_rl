use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Different types of AI behavior patterns
#[derive(Debug, Clone, PartialEq, Reflect, Serialize, Deserialize, Copy)]
pub enum AIBehaviorType {
    /// Hostile enemies that chase and attack the player
    Hostile,
    /// Passive entities that flee when threatened
    Passive,
    /// Neutral entities that ignore the player unless provoked
    Neutral,
}

impl Default for AIBehaviorType {
    fn default() -> Self { Self::Neutral }
}

/// Data representation of TurnActor component for serialization
#[derive(Serialize, Deserialize, Debug, Clone, Reflect)]
pub struct TurnActorData {
    /// Speed value for turn timing
    pub speed: u32,
    /// Maximum number of actions that can be queued
    pub action_queue_size: Option<usize>,
}

impl TurnActorData {
    pub fn new(speed: u32) -> Self { Self { speed, action_queue_size: None } }

    pub fn with_queue_size(mut self, size: usize) -> Self {
        self.action_queue_size = Some(size);
        self
    }
}

/// Data representation of ViewShed component for serialization
#[derive(Serialize, Deserialize, Debug, Clone, Reflect, Deref, DerefMut)]
pub struct FieldOfViewData(pub u8);

impl FieldOfViewData {
    pub fn new(radius: u8) -> Self { Self(radius) }
}

/// Data representation of TileSprite component for serialization
#[derive(Serialize, Deserialize, Debug, Clone, Reflect)]
pub struct TileSpriteData {
    /// Tile coordinates in the tilemap (x, y)
    pub tile_coords: (u32, u32),
    /// Size of each tile (defaults to ViewConstants::TILE_SIZE if not specified)
    pub tile_size: Option<(f32, f32)>,
    /// Optional tint color
    pub tint: Option<(f32, f32, f32, f32)>, // RGBA
}

impl TileSpriteData {
    pub fn new(tile_coords: (u32, u32)) -> Self { Self { tile_coords, tile_size: None, tint: None } }

    pub fn with_size(mut self, width: f32, height: f32) -> Self {
        self.tile_size = Some((width, height));
        self
    }

    pub fn with_tint(mut self, r: f32, g: f32, b: f32, a: f32) -> Self {
        self.tint = Some((r, g, b, a));
        self
    }
}

/// Data representation of Health component for serialization
#[derive(Serialize, Deserialize, Debug, Clone, Reflect)]
pub struct HealthData {
    /// Current health points
    pub current: i32,
    /// Maximum health points
    pub max: i32,
}

impl HealthData {
    pub fn new(max_health: i32) -> Self { Self { current: max_health, max: max_health } }

    pub fn new_with_current(current: i32, max: i32) -> Self { Self { current: current.min(max), max } }
}

/// Data representation of Stats component for serialization
#[derive(Serialize, Deserialize, Debug, Clone, Reflect)]
pub struct StatsData {
    /// Physical strength - affects melee damage and carrying capacity
    pub strength: i32,
    /// Physical defense - reduces incoming physical damage
    pub defense: i32,
    /// Mental intelligence - affects magic damage and mana
    pub intelligence: i32,
    /// Agility/dexterity - affects accuracy, evasion, and initiative
    pub agility: i32,
    /// Constitution/vitality - affects health and stamina
    pub vitality: i32,
    /// Luck - affects critical hits and random events
    pub luck: i32,
}

impl StatsData {
    /// Create new stats with all values set to the same amount
    pub fn uniform(value: i32) -> Self {
        Self {
            strength: value,
            defense: value,
            intelligence: value,
            agility: value,
            vitality: value,
            luck: value,
        }
    }

    /// Create balanced stats (all 10)
    pub fn balanced() -> Self { Self::uniform(10) }

    /// Create warrior-type stats (high strength/defense, low intelligence)
    pub fn warrior() -> Self {
        Self { strength: 15, defense: 14, intelligence: 8, agility: 11, vitality: 13, luck: 9 }
    }

    /// Create mage-type stats (high intelligence, low strength/defense)
    pub fn mage() -> Self {
        Self { strength: 8, defense: 9, intelligence: 16, agility: 12, vitality: 10, luck: 15 }
    }

    /// Create rogue-type stats (high agility/luck, balanced others)
    pub fn rogue() -> Self {
        Self { strength: 11, defense: 10, intelligence: 13, agility: 16, vitality: 11, luck: 14 }
    }
}

/// Data representation of Inventory component for serialization
#[derive(Serialize, Deserialize, Debug, Clone, Reflect)]
pub struct InventoryData {
    /// Maximum number of item slots
    pub max_slots: usize,
    /// Maximum weight capacity
    pub max_weight: f32,
    /// Starting items (item_id, quantity pairs)
    pub starting_items: Option<Vec<(String, u32)>>,
}

impl InventoryData {
    pub fn new(max_slots: usize, max_weight: f32) -> Self {
        Self { max_slots, max_weight, starting_items: None }
    }

    pub fn with_starting_items(mut self, items: Vec<(String, u32)>) -> Self {
        self.starting_items = Some(items);
        self
    }
}

/// Data representation of Description component for serialization
#[derive(Serialize, Deserialize, Debug, Clone, Reflect)]
pub struct DescriptionData {
    /// Entity description text
    pub text: String,
}

impl DescriptionData {
    pub fn new(text: impl Into<String>) -> Self { Self { text: text.into() } }
}
