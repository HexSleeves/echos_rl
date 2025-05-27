use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use bevy_asset_loader::prelude::*;
use iyes_progress::{ProgressPlugin, ProgressTracker};

use crate::{
    model::entities::EntityDefinitions,
    view::{resources::TextureAssets, screens::ScreenState},
};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((ProgressPlugin::<ScreenState>::new()
        .with_state_transition(ScreenState::Loading, ScreenState::Gameplay),))
        .add_loading_state(
            LoadingState::new(ScreenState::Loading)
                .with_dynamic_assets_file::<StandardDynamicAssetCollection>("textures.ron")
                .load_collection::<TextureAssets>()
                .load_collection::<EntityDefinitions>(),
        )
        .add_systems(
            Update,
            print_progress
                .run_if(in_state(ScreenState::Loading))
                .after(LoadingStateSet(ScreenState::Loading)),
        );
}

fn print_progress(
    progress: Res<ProgressTracker<ScreenState>>,
    diagnostics: Res<DiagnosticsStore>,
    mut last_done: Local<u32>,
) {
    let progress = progress.get_global_progress();
    if progress.done > *last_done {
        *last_done = progress.done;
        info!(
            "[Frame {}] Changed progress: {:?}",
            diagnostics
                .get(&FrameTimeDiagnosticsPlugin::FRAME_COUNT)
                .map(|diagnostic| diagnostic.value().unwrap_or(0.))
                .unwrap_or(0.),
            progress
        );
    }
}
