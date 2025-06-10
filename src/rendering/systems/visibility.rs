use bevy::prelude::*;

use crate::core::{
    components::{AITag, PlayerTag, Position},
    resources::{FovMap, LightMap},
};

// ============================================================================
// VISIBILITY SYSTEMS
// ============================================================================

/// Apply lighting to a sprite with minimum visibility threshold
fn apply_lighting_to_sprite(sprite: &mut Sprite, light_map: &LightMap, position: &Position) {
    let light_color = light_map.get_light((position.x, position.y));
    let light_linear = light_color.to_linear();

    let min_light = 0.3;
    let lit_color = Color::linear_rgb(
        (light_linear.red + min_light).min(1.0),
        (light_linear.green + min_light).min(1.0),
        (light_linear.blue + min_light).min(1.0),
    );

    sprite.color = lit_color;
}

/// System that updates sprite visibility based on the FOV map and lighting
///
/// This system runs after FOV computation and updates the visibility of sprites/entities
/// based on what's currently visible and what has been explored, with realistic lighting.
///
/// # Visibility Logic
/// - **Player**: Always visible (never hidden)
/// - **Visible entities**: Affected by lighting (color modulated by light)
/// - **Revealed but not visible**: Semi-transparent (alpha = 0.4)
/// - **Unexplored entities**: Hidden (Visibility::Hidden)
///
/// # Performance Notes
/// - Only processes entities when the FOV map or light map has changed
/// - Excludes player from visibility changes
/// - Uses efficient FOV map and light map queries
pub fn update_sprite_visibility(
    fov_map: Res<FovMap>,
    light_map: Res<LightMap>,
    mut q_sprites: Query<(&Position, &mut Visibility, &mut Sprite), (With<Sprite>, Without<PlayerTag>)>,
) {
    // Only update when FOV or lighting changes to avoid unnecessary work
    if !fov_map.is_changed() && !light_map.is_changed() {
        return;
    }

    for (position, mut visibility, mut sprite) in &mut q_sprites {
        if fov_map.is_visible(*position) {
            *visibility = Visibility::Visible;

            // Apply lighting to the sprite
            apply_lighting_to_sprite(&mut sprite, &light_map, position);
        }
        // else if fov_map.is_revealed(*position) {
        //     *visibility = Visibility::Visible;
        //     sprite.color.set_alpha(0.4); // Semi-transparent for revealed areas
        // }
        else {
            *visibility = Visibility::Hidden;
        }
    }
}

/// System that provides different visibility treatment for different entity types with lighting
///
/// This allows for more nuanced visibility rules:
/// - Living entities (monsters, NPCs): Only visible in current FOV with lighting
/// - Items: Visible in revealed areas (memory of where items were) with lighting
/// - Static objects: Visible in revealed areas with lighting
pub fn update_typed_visibility(
    fov_map: Res<FovMap>,
    light_map: Res<LightMap>,
    // Entities that should only be visible in current FOV (enemies, NPCs)
    mut q_living: Query<
        (&Position, &mut Visibility, &mut Sprite),
        (With<Sprite>, Without<PlayerTag>, With<AITag>),
    >,
    // Entities that should be visible in revealed areas (items, decorations)
    mut q_static: Query<
        (&Position, &mut Visibility, &mut Sprite),
        (With<Sprite>, Without<PlayerTag>, Without<AITag>),
    >,
) {
    if !fov_map.is_changed() && !light_map.is_changed() {
        return;
    }

    // Living entities: only visible in current FOV with lighting
    for (position, mut visibility, mut sprite) in &mut q_living {
        if fov_map.is_visible(*position) {
            *visibility = Visibility::Visible;

            // Apply lighting to living entities
            apply_lighting_to_sprite(&mut sprite, &light_map, position);
        } else {
            *visibility = Visibility::Hidden;
        }
    }

    // Static entities: visible in revealed areas with transparency and lighting
    for (position, mut visibility, mut sprite) in &mut q_static {
        if fov_map.is_visible(*position) {
            *visibility = Visibility::Visible;

            // Apply full lighting to visible static entities
            apply_lighting_to_sprite(&mut sprite, &light_map, position);
        } else if fov_map.is_revealed(*position) {
            *visibility = Visibility::Visible;
            // Revealed static entities are dimmed (fog of war effect)
            sprite.color = Color::srgba(0.6, 0.6, 0.6, 0.6);
        } else {
            *visibility = Visibility::Hidden;
        }
    }
}
