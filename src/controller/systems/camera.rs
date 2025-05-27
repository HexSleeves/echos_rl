use bevy::prelude::*;

use crate::{
    model::{
        ModelConstants,
        components::{PlayerTag, Position},
    },
    view::ViewConstants,
};

/// Camera system that follows the player with smooth interpolation and handles zoom controls.
pub fn camera_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    player_query: Query<&Position, With<PlayerTag>>,
    mut camera_query: Query<(&mut Transform, &mut Projection), With<Camera>>,
) {
    for (mut camera_transform, mut projection) in camera_query.iter_mut() {
        // Handle zoom controls (keep existing zoom functionality)
        let Projection::Orthographic(ortho) = &mut *projection else {
            continue;
        };

        if keyboard_input.pressed(KeyCode::KeyZ) {
            ortho.scale += 0.1;
        }

        if keyboard_input.pressed(KeyCode::KeyX) {
            ortho.scale -= 0.1;
        }

        if ortho.scale < 0.5 {
            ortho.scale = 0.5;
        }

        // Follow the player if they exist
        if let Ok(player_position) = player_query.single() {
            // Convert tile position to world coordinates (same calculation as
            // position_to_transform)
            let target_x = player_position.x() as f32 * ViewConstants::TILE_SIZE
                - (ModelConstants::MAP_WIDTH as f32 * ViewConstants::HALF_TILE_SIZE);
            let target_y = player_position.y() as f32 * ViewConstants::TILE_SIZE
                - (ModelConstants::MAP_HEIGHT as f32 * ViewConstants::HALF_TILE_SIZE);

            let target_position = Vec3::new(target_x, target_y, camera_transform.translation.z);

            // Snap camera directly to the player position
            camera_transform.translation = target_position;
        }
    }
}
