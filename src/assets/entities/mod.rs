pub mod components;
pub mod definition;
pub mod loader;
pub mod spawner;

pub use components::*;
pub use definition::*;
pub use loader::*;
pub use spawner::*;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<definition::EntityComponents>()
        .register_type::<components::TurnActorData>()
        .register_type::<components::ViewShedData>()
        .register_type::<components::TileSpriteData>();
}
