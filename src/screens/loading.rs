//! A loading screen during which game assets are loaded.
//! This reduces stuttering, especially for audio on WASM.

use crate::{screens::ScreenState, theme::prelude::*};
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(ScreenState::Loading), spawn_loading_screen);

    app.add_loading_state(
        LoadingState::new(ScreenState::Loading)
            .continue_to_state(ScreenState::Gameplay)
            .load_collection::<AudioAssets>()
            .load_collection::<TextureAssets>(),
    );
}

fn spawn_loading_screen(mut commands: Commands) {
    commands.spawn((
        widget::ui_root("Loading Screen"),
        StateScoped(ScreenState::Loading),
        children![widget::label("Loading...")],
    ));
}

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {}

#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
    #[asset(path = "textures/curses_vector_24x36.png")]
    pub curses_24x36: Handle<Image>,
    #[asset(path = "textures/terminal_32x32.png")]
    pub terminal_32x32: Handle<Image>,
}
