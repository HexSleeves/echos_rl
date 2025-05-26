use bevy::prelude::*;

use crate::{
    model::{ModelConstants, components::Position},
    view::ViewConstants,
};

pub fn position_to_transform(mut q_objects: Query<(&Position, &mut Transform)>) {
    for (position, mut transform) in &mut q_objects {
        // Update position
        transform.translation.x = position.x() as f32 * ViewConstants::TILE_SIZE
            - (ModelConstants::MAP_WIDTH as f32 * ViewConstants::HALF_TILE_SIZE);

        transform.translation.y = position.y() as f32 * ViewConstants::TILE_SIZE
            - (ModelConstants::MAP_HEIGHT as f32 * ViewConstants::HALF_TILE_SIZE);

        transform.translation.z = 1.0;
    }
}
