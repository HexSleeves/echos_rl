pub fn add_tileset_texture(
    &mut self,
    asset_server: &AssetServer,
    textures: &mut Assets<TextureAtlas>,
    path: &str,
) -> Handle<TextureAtlas> {
    let texture_handle = asset_server.load(path);
    let atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(12.0, 12.0),     // Individual tile size
        12,                        // Columns
        12,                        // Rows
        Some(Vec2::new(1.0, 1.0)), // Padding (space between tiles)
        Some(Vec2::new(1.0, 1.0)), // Offset (starting point)
    );
    textures.add(atlas)
}
