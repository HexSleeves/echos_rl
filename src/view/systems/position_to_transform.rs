use bevy::prelude::*;

use crate::{
    model::{ModelConstants, components::Position},
    view::ViewConstants,
};

pub fn transform_from_position(position: &Position) -> Transform {
    Transform::from_xyz(
        position.x() as f32 * ViewConstants::TILE_SIZE
            - (ModelConstants::MAP_WIDTH as f32 * ViewConstants::HALF_TILE_SIZE),
        position.y() as f32 * ViewConstants::TILE_SIZE
            - (ModelConstants::MAP_HEIGHT as f32 * ViewConstants::HALF_TILE_SIZE),
        1.0,
    )
}

pub fn position_to_transform(mut q_objects: Query<(&Position, &mut Transform)>) {
    for (position, mut transform) in &mut q_objects {
        *transform = transform_from_position(position);
    }
}
