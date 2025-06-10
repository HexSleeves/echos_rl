use bevy::prelude::*;

use crate::core::{
    components::{Position, light::Light},
    resources::{CurrentMap, LightMap},
};

use brtk::fov::{
    algorithms::shadowcast::Shadowcast,
    implementations::{map_provider::MapProvider, visibility_map::VisibilityMap},
    traits::{FovAlgorithm, FovProvider},
};

/// System that calculates the light map based on all light sources in the world.
pub fn calculate_light_map(
    mut light_map: ResMut<LightMap>,
    map: Res<CurrentMap>,
    query: Query<(&Position, &Light)>,
) {
    light_map.clear();

    for (light_pos, light) in query.iter() {
        let mut visibility_map = VisibilityMap::with_range_capacity(light.range);
        let map_provider = MapProvider::new(&*map, |map_ref, pos, _vision_type| map_ref.is_opaque(pos, 0));

        Shadowcast::compute_fov(
            (light_pos.x, light_pos.y),
            0, // vision_type - not used for light calculation, but needed by trait
            light.range,
            &map_provider,
            &mut visibility_map,
        );

        for visible_pos in visibility_map.get_all_visible() {
            let visible_position = Position::new(visible_pos.0, visible_pos.1);
            let distance = light_pos.distance(&visible_position);
            let intensity = (1.0 - (distance / light.range as f32).powf(light.falloff)).max(0.0);

            // Convert color to linear RGB for calculations
            let light_linear = light.color.to_linear();
            let multiplied_color = Color::linear_rgb(
                light_linear.red * intensity,
                light_linear.green * intensity,
                light_linear.blue * intensity,
            );

            // Combine light from multiple sources: take the brightest color component
            let existing_linear = light_map.get_light(visible_pos).to_linear();
            let multiplied_linear = multiplied_color.to_linear();
            let combined_color = Color::linear_rgb(
                existing_linear.red.max(multiplied_linear.red),
                existing_linear.green.max(multiplied_linear.green),
                existing_linear.blue.max(multiplied_linear.blue),
            );

            light_map.set_light(visible_pos, combined_color);
        }
    }
}
