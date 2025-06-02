/// Constants for rendering and visual presentation
pub struct RenderingConstants;

impl RenderingConstants {
    /// The size of each tile in pixels
    pub const TILE_SIZE: f32 = 12.0;

    /// Half tile size for centering calculations
    pub const HALF_TILE_SIZE: f32 = Self::TILE_SIZE / 2.0;

    /// Path to the tilemap texture
    pub const TILEMAP_TEXTURE_PATH: &'static str = "textures/urizen/urizen_onebit_tileset_no_spaces.png";

    /// Number of columns in the tilemap
    pub const TILEMAP_COLUMNS: usize = 102;

    /// Number of rows in the tilemap
    pub const TILEMAP_ROWS: usize = 49;

    /// Debug colors for FOV visualization
    pub const DEBUG_VISIBLE_COLOR: (f32, f32, f32, f32) = (0.8, 1.0, 0.8, 1.0);
    pub const DEBUG_REVEALED_COLOR: (f32, f32, f32, f32) = (0.8, 0.8, 1.0, 0.6);
    pub const DEBUG_UNEXPLORED_COLOR: (f32, f32, f32, f32) = (1.0, 1.0, 1.0, 1.0);
}
