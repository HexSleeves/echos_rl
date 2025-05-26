use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
    // Kenny
    // #[asset(key = "kenny_textures", collection(typed))]
    // pub kenny_textures: Vec<Handle<Image>>,
    // #[asset(key = "kenny_textures", collection(typed, mapped))]
    // pub kenny_textures_mapped: HashMap<String, Handle<Image>>,
    // #[asset(key = "urizen_textures", collection(typed))]
    // pub urizen_textures: Vec<Handle<Image>>,
    // #[asset(key = "urizen_textures", collection(typed, mapped))]
    // pub urizen_textures_mapped: HashMap<String, Handle<Image>>,
    #[asset(image(sampler(filter = nearest)))]
    #[asset(path = "textures/urizen/urizen_onebit_tileset.png")]
    pub urizen_tileset: Handle<Image>,
}
