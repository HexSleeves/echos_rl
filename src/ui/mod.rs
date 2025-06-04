use bevy::prelude::*;

pub mod components;
pub mod systems;
pub mod utils;

mod ui_constants;

pub use self::ui_constants::*;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<components::GameCamera>();
    app.register_type::<components::InteractionPalette>();
    app.add_systems(Update, (systems::apply_interaction_palette,));
}
