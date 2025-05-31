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
    /// Creates a new `TileSprite` with the specified tile coordinates and tile size, without a tint color.
    ///
    /// # Examples
    ///
    /// ```
    /// use bevy::math::Vec2;
    /// use my_crate::rendering::components::TileSprite;
    ///
    /// let sprite = TileSprite::new((3, 5), Vec2::new(32.0, 32.0));
    /// assert_eq!(sprite.tile_coords, (3, 5));
    /// assert_eq!(sprite.tile_size, Vec2::new(32.0, 32.0));
    /// assert!(sprite.tint.is_none());
    /// ```
    pub fn new(tile_coords: (u32, u32), tile_size: Vec2) -> Self {
        Self { 
            tile_coords, 
            tile_size, 
            tint: None 
        }
    }

    /// Returns a copy of the `TileSprite` with the specified tint color applied.
    ///
    /// # Examples
    ///
    /// ```
    /// let sprite = TileSprite::new((1, 2), Vec2::new(16.0, 16.0));
    /// let tinted = sprite.with_tint(Color::RED);
    /// assert_eq!(tinted.tint, Some(Color::RED));
    /// ```
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
    /// Creates a new `ViewShed` component with the specified field of view radius.
    ///
    /// # Panics
    ///
    /// Panics in debug mode if `radius` is negative.
    ///
    /// # Examples
    ///
    /// ```
    /// let viewshed = ViewShed::new(8);
    /// assert_eq!(viewshed.radius, 8);
    /// ```
    pub fn new(radius: i32) -> Self {
        debug_assert!(radius >= 0, "ViewShed radius must be non-negative");
        Self { radius }
    }
}
