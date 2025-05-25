use bevy::prelude::*;

use crate::model::components::Description;

/// Represents the different types of underground environments
#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect)]
pub enum UndergroundType {
    Mine,
    Cave,
}

#[derive(Component, Reflect, Default, PartialEq, Eq, Clone, Debug, Copy)]
#[reflect(Component)]
#[require(Description)]
pub enum TerrainType {
    #[default]
    Floor,
    Wall,

    // Underground tiles
    // MineWall,
    // MineFloor,
    // CaveWall,
    // CaveFloor,

    // Ore types
    // IronOre,
    // CopperOre,
    // SonoriteOre,       // Resonant ore
    // GlimmerstoneOre,   // Luminous ore
    // WhisperingIronOre, // Exotic/Cursed ore

    // Special features
    // Door,
    StairsUp,
    StairsDown,
    // UnstableWall,
}

impl TerrainType {
    pub fn texture_index(self) -> u32 {
        match self {
            // Surface tiles
            TerrainType::Wall => 206,
            TerrainType::Floor => 515,

            // Underground tiles
            // TerrainType::MineWall => 2,
            // TerrainType::MineFloor => 3,
            // TerrainType::CaveWall => 4,
            // TerrainType::CaveFloor => 5,

            // Ore types
            // TerrainType::IronOre => 6,
            // TerrainType::CopperOre => 7,
            // TerrainType::SonoriteOre => 8,
            // TerrainType::GlimmerstoneOre => 9,
            // TerrainType::WhisperingIronOre => 10,

            // Special features
            // TerrainType::Door => 11,
            TerrainType::StairsUp => 127,
            TerrainType::StairsDown => 126,
            // TerrainType::UnstableWall => 14,
        }
    }

    pub fn description(&self) -> String {
        match self {
            TerrainType::Floor => "Floor".to_string(),
            TerrainType::Wall => "Wall".to_string(),

            // TerrainType::MineWall => "Mine Wall".to_string(),
            // TerrainType::MineFloor => "Mine Floor".to_string(),
            // TerrainType::CaveWall => "Cave Wall".to_string(),
            // TerrainType::CaveFloor => "Cave Floor".to_string(),

            // TerrainType::IronOre => "Iron Ore".to_string(),
            // TerrainType::CopperOre => "Copper Ore".to_string(),
            // TerrainType::SonoriteOre => "Sonorite Ore".to_string(),
            // TerrainType::GlimmerstoneOre => "Glimmerstone Ore".to_string(),
            // TerrainType::WhisperingIronOre => "Whispering Iron Ore".to_string(),

            // TerrainType::Door => "Door".to_string(),
            TerrainType::StairsUp => "Stairs leading up".to_string(),
            TerrainType::StairsDown => "Stairs leading down".to_string(),
            // TerrainType::UnstableWall => "Unstable Wall".to_string(),
        }
    }

    /// Returns true if this terrain type blocks vision (walls, etc.)
    pub fn blocks_vision(&self) -> bool {
        match self {
            TerrainType::Wall => true,
            // TerrainType::Door => true, // Closed doors block vision
            _ => false,
        }
    }

    /// Returns true if this tile type is walkable
    pub fn is_walkable(self) -> bool {
        match self {
            TerrainType::Floor
            // | TerrainType::MineFloor
            // | TerrainType::CaveFloor
            | TerrainType::StairsUp
            | TerrainType::StairsDown => true,
            _ => false,
        }
    }

    /// Returns true if this tile type is mineable
    pub fn is_mineable(self) -> bool {
        match self {
            // | TerrainType::MineWall
            // | TerrainType::CaveWall
            // | TerrainType::IronOre
            // | TerrainType::CopperOre
            // | TerrainType::SonoriteOre
            // | TerrainType::GlimmerstoneOre
            // | TerrainType::WhisperingIronOre
            // | TerrainType::UnstableWall => true,
            _ => false,
        }
    }

    /// Returns true if this tile type is an ore
    pub fn is_ore(self) -> bool {
        match self {
            // TerrainType::IronOre
            // | TerrainType::CopperOre
            // | TerrainType::SonoriteOre
            // | TerrainType::GlimmerstoneOre
            // | TerrainType::WhisperingIronOre => true,
            _ => false,
        }
    }
}
