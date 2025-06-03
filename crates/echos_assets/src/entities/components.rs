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
    pub speed: u64,
    /// Maximum number of actions that can be queued
    pub action_queue_size: Option<usize>,
}

impl TurnActorData {
    pub fn new(speed: u64) -> Self { Self { speed, action_queue_size: None } }

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
