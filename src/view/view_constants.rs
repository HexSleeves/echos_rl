pub struct ViewConstants;

impl ViewConstants {
    pub const TILE_SIZE: f32 = 12.0;
    pub const HALF_TILE_SIZE: f32 = ViewConstants::TILE_SIZE / 2.0;

    pub const TILEMAP_ROWS: usize = 49;
    pub const TILEMAP_COLUMNS: usize = 102;

    pub const TILEMAP_TEXTURE_PATH: &str = "textures/urizen/urizen_onebit_tileset.png";
}
