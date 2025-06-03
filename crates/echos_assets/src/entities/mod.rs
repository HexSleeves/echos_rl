pub mod components;
pub mod definition;
pub mod loader;

pub use components::*;
pub use definition::*;
pub use loader::*;

use bevy::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(RonAssetPlugin::<EntityDefinition>::new(&["definition.ron"]));

    app.register_type::<definition::EntityComponents>()
        .register_type::<components::TurnActorData>()
        .register_type::<components::FieldOfViewData>()
        .register_type::<components::TileSpriteData>()
        .register_type::<components::AIBehaviorType>();
}
