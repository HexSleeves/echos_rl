use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
    #[asset(image(sampler(filter = nearest)))]
    #[asset(path = "textures/urizen/urizen_onebit_tileset.png")]
    pub urizen_tileset: Handle<Image>,
}
