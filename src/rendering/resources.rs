use bevy::{prelude::*, sprite::Anchor};
use bevy_asset_loader::prelude::*;

use crate::rendering::RenderingConstants;

/// Resource that manages texture assets for rendering
#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
    #[asset(image(sampler(filter = nearest)))]
    #[asset(path = "textures/urizen/urizen_onebit_tileset_no_spaces.png")]
    pub urizen_tileset: Handle<Image>,
}

/// Resource that manages a tilemap for rendering sprites
#[derive(Resource, Clone, Debug)]
pub struct TileMap {
    /// The texture atlas handle for the tilemap
    pub texture_atlas: Handle<TextureAtlasLayout>,
    /// The image handle for the tilemap
    pub texture: Handle<Image>,
    /// The size of each tile in the tilemap
    pub tile_size: Vec2,
    /// The number of columns in the tilemap
    pub columns: usize,
    /// The number of rows in the tilemap
    pub rows: usize,
}

impl TileMap {
    /// Create a new tilemap from an image path
    pub fn new(
        asset_server: &AssetServer,
        texture: Handle<Image>,
        tile_size: Vec2,
        columns: usize,
        rows: usize,
    ) -> Self {
        // Create a texture atlas layout
        let layout = TextureAtlasLayout::from_grid(
            UVec2::new(tile_size.x as u32, tile_size.y as u32),
            columns as u32,
            rows as u32,
            None,
            None,
        );

        let texture_atlas = asset_server.add(layout);

        Self { texture_atlas, texture, tile_size, columns, rows }
    }

    /// Convert tile coordinates to a sprite index
    pub fn coords_to_index(&self, coords: (u32, u32)) -> usize {
        let (x, y) = coords;
        (y as usize * self.columns + x as usize) % (self.columns * self.rows)
    }

    /// Generate a sprite for a tile using its coordinates in the texture atlas
    ///
    /// Note: Handle<T> in Bevy is a lightweight reference (essentially an ID),
    /// so we can clone it without significant overhead.
    pub fn generate_sprite_for_tile(&self, coords: (u32, u32)) -> Sprite {
        let index = self.coords_to_index(coords);
        Sprite {
            image: self.texture.clone(),
            texture_atlas: Some(TextureAtlas { layout: self.texture_atlas.clone(), index }),
            custom_size: Some(Vec2::splat(RenderingConstants::TILE_SIZE)),
            anchor: Anchor::BottomLeft,
            ..Default::default()
        }
    }

    /// Generate a sprite for a terrain type using its index in the texture atlas
    ///
    /// Note: Handle<T> in Bevy is a lightweight reference (essentially an ID),
    /// so we can clone it without significant overhead.
    pub fn generate_sprite_for_terrain(&self, index: usize) -> Sprite {
        Sprite {
            image: self.texture.clone(),
            texture_atlas: Some(TextureAtlas { layout: self.texture_atlas.clone(), index }),
            custom_size: Some(Vec2::splat(RenderingConstants::TILE_SIZE)),
            anchor: Anchor::BottomLeft,
            ..Default::default()
        }
    }
}

impl FromWorld for TileMap {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        let texture = asset_server.load(RenderingConstants::TILEMAP_TEXTURE_PATH);
        Self::new(
            asset_server,
            texture,
            Vec2::splat(RenderingConstants::TILE_SIZE),
            RenderingConstants::TILEMAP_COLUMNS,
            RenderingConstants::TILEMAP_ROWS,
        )
    }
}
