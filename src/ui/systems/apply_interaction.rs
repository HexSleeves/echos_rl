use bevy::prelude::*;

use crate::ui::components::InteractionPalette;

pub fn apply_interaction_palette(
    mut palette_query: Query<(&Interaction, &InteractionPalette, &mut BackgroundColor), Changed<Interaction>>,
) {
    for (interaction, palette, mut background) in &mut palette_query {
        *background = match interaction {
            Interaction::None => palette.none,
            Interaction::Hovered => palette.hovered,
            Interaction::Pressed => palette.pressed,
        }
        .into();
    }
}
