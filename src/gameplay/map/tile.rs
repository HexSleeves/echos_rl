use bevy::prelude::*;

/// Represents the different types of underground environments
#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect)]
pub enum UndergroundType {
    Mine,
    Cave,
    DarkAdventure,
}

/// Represents the different types of tiles in the game world.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect)]
pub enum TileType {
    // Surface tiles
    Wall,
    Floor,

    // Underground tiles
    MineWall,
    MineFloor,
    CaveWall,
    CaveFloor,

    // Ore types
    IronOre,
    CopperOre,
    SonoriteOre,       // Resonant ore
    GlimmerstoneOre,   // Luminous ore
    WhisperingIronOre, // Exotic/Cursed ore

    // Special features
    Stairs,
    StairsUp,
    StairsDown,
    AncientMachinery,
    GasPocket,
    UnstableWall,
}

impl TileType {
    pub fn texture_index(self) -> u32 {
        match self {
            // Surface tiles
            TileType::Wall => 0,
            TileType::Floor => 1,

            // Underground tiles
            TileType::MineWall => 2,
            TileType::MineFloor => 3,
            TileType::CaveWall => 4,
            TileType::CaveFloor => 5,

            // Ore types
            TileType::IronOre => 6,
            TileType::CopperOre => 7,
            TileType::SonoriteOre => 8,
            TileType::GlimmerstoneOre => 9,
            TileType::WhisperingIronOre => 10,

            // Special features
            TileType::Stairs => 11,
            TileType::StairsUp => 12,
            TileType::StairsDown => 13,
            TileType::AncientMachinery => 14,
            TileType::GasPocket => 15,
            TileType::UnstableWall => 16,
        }
    }

    /// Returns true if this tile type is walkable
    pub fn is_walkable(self) -> bool {
        match self {
            TileType::Floor
            | TileType::MineFloor
            | TileType::CaveFloor
            | TileType::Stairs
            | TileType::StairsUp
            | TileType::StairsDown => true,
            _ => false,
        }
    }

    /// Returns true if this tile type is mineable
    pub fn is_mineable(self) -> bool {
        match self {
            TileType::Wall
            | TileType::MineWall
            | TileType::CaveWall
            | TileType::IronOre
            | TileType::CopperOre
            | TileType::SonoriteOre
            | TileType::GlimmerstoneOre
            | TileType::WhisperingIronOre
            | TileType::UnstableWall => true,
            _ => false,
        }
    }

    /// Returns true if this tile type is an ore
    pub fn is_ore(self) -> bool {
        match self {
            TileType::IronOre
            | TileType::CopperOre
            | TileType::SonoriteOre
            | TileType::GlimmerstoneOre
            | TileType::WhisperingIronOre => true,
            _ => false,
        }
    }
}
