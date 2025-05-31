use bevy::prelude::*;

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
        Self { 
            tile_coords, 
            tile_size, 
            tint: None 
        }
    }

    pub fn with_tint(mut self, tint: Color) -> Self {
        self.tint = Some(tint);
        self
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
