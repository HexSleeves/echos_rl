use bevy::prelude::*;

pub struct ViewPlugin;
impl Plugin for ViewPlugin {
    fn build(&self, app: &mut App) {
        // Register the TileSprite component for reflection
        // app.register_type::<crate::view::components::TileSprite>();

        // // Initialize the tilemap during startup
        // app.add_systems(Startup, init_tilemap)
        //     .add_systems(PostUpdate, ((add_sprite_to_player, add_sprite_to_tile), position_to_transform));
    }
}
