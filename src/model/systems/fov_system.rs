use bevy::prelude::*;

use crate::model::{
    components::{PlayerTag, Position, ViewShed},
    resources::CurrentMap,
};

/// System that updates field of view for entities with FieldOfView component
pub fn update_fov(
    mut current_map: ResMut<CurrentMap>,
    fov_query: Query<(&Position, &ViewShed), (Changed<Position>, With<PlayerTag>)>,
) {
    for (position, fov) in fov_query.iter() {
        // Clear all visibility first
        current_map.clear_visibility();

        // Calculate visible tiles using simple circular FOV
        calculate_circular_fov(&mut current_map, *position, fov.radius);
    }
}

/// Simple circular field of view calculation
fn calculate_circular_fov(map: &mut CurrentMap, center: Position, range: i32) {
    let (cx, cy) = center.into();

    for x in (cx - range)..=(cx + range) {
        for y in (cy - range)..=(cy + range) {
            let pos = Position::new(x, y);
            if !map.in_bounds(pos) {
                continue;
            }

            let distance_sq = (x - cx).pow(2) + (y - cy).pow(2);
            if distance_sq <= range.pow(2) {
                // Simple line-of-sight check
                if has_line_of_sight(map, center, pos) {
                    map.set_visible(pos, true);
                }
            }
        }
    }
}

/// Simple line of sight using Bresenham's line algorithm
fn has_line_of_sight(map: &CurrentMap, from: Position, to: Position) -> bool {
    let (x0, y0) = from.into();
    let (x1, y1) = to.into();

    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx - dy;

    let mut x = x0;
    let mut y = y0;

    loop {
        let pos = Position::new(x, y);

        // If we hit a wall (except at the destination), line of sight is blocked
        if pos != to && !map.is_walkable(pos) {
            return false;
        }

        if x == x1 && y == y1 {
            break;
        }

        let e2 = 2 * err;
        if e2 > -dy {
            err -= dy;
            x += sx;
        }
        if e2 < dx {
            err += dx;
            y += sy;
        }
    }

    true
}
