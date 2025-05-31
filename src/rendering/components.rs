use bevy::prelude::*;
use echos_assets::entities::TileSpriteData;

use crate::rendering::RenderingConstants;

/// Component for entities that are rendered using a sprite from a tilemap
#[derive(Component, Reflect, Default, Clone)]
#[reflect(Component)]
pub struct TileSprite {
    /// The coordinates in the tilemap (x, y)
    pub tile_coords: (u32, u32),
    /// The size of each tile in the tilemap
    pub tile_size: Vec2,
    /// Custom tint color for the sprite
    pub tint: Option<Color>,
}

impl TileSprite {
    pub fn new(tile_coords: (u32, u32), tile_size: Vec2) -> Self {
        Self { tile_coords, tile_size, tint: None }
    }

    pub fn with_tint(mut self, tint: Color) -> Self {
        self.tint = Some(tint);
        self
    }
}

impl From<TileSpriteData> for TileSprite {
    fn from(data: TileSpriteData) -> Self {
        let tile_size = data
            .tile_size
            .map(|(w, h)| Vec2::new(w, h))
            .unwrap_or_else(|| Vec2::splat(RenderingConstants::TILE_SIZE));

        let tint = data.tint.map(|(r, g, b, a)| Color::srgba(r, g, b, a));

        TileSprite { tile_coords: data.tile_coords, tile_size, tint }
    }
}

impl From<&TileSpriteData> for TileSprite {
    fn from(data: &TileSpriteData) -> Self {
        let tile_size = data
            .tile_size
            .map(|(w, h)| Vec2::new(w, h))
            .unwrap_or_else(|| Vec2::splat(RenderingConstants::TILE_SIZE));

        let tint = data.tint.map(|(r, g, b, a)| Color::srgba(r, g, b, a));

        TileSprite { tile_coords: data.tile_coords, tile_size, tint }
    }
}

/// Component for entities that can see (field of view)
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct ViewShed {
    pub radius: i32,
}

impl ViewShed {
    pub fn new(radius: i32) -> Self {
        debug_assert!(radius >= 0, "ViewShed radius must be non-negative");
        Self { radius }
    }
}
