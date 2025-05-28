use bevy::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;

use entities::EntityDefinition;

pub mod entities;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((RonAssetPlugin::<EntityDefinition>::new(&["definition.ron"]), entities::plugin));
}
