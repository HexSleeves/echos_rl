use crate::model::{
    ModelConstants,
    components::{Position, TerrainType},
    resources::Map,
};
use bevy::prelude::*;
use bitvec::prelude::*;

/// Field of view map using bit-level storage for memory efficiency.
/// This implementation uses the BitVec crate to store boolean values as individual bits.
#[derive(Resource)]
pub struct FovMap {
    width: usize,
    height: usize,
    revealed: BitVec,
    visible: BitVec,
}

impl FromWorld for FovMap {
    fn from_world(_world: &mut World) -> Self {
        let size = (ModelConstants::MAP_WIDTH, ModelConstants::MAP_HEIGHT);
        Self::new(size.0, size.1)
    }
}

impl FovMap {
    pub fn new(width: usize, height: usize) -> Self {
        let size = width * height;
        Self { width, height, revealed: bitvec![0; size], visible: bitvec![0; size] }
    }

    /// Converts 2D coordinates to a 1D index
    fn coords_to_index(&self, x: i32, y: i32) -> Option<usize> {
        if x < 0 || y < 0 || x >= self.width as i32 || y >= self.height as i32 {
            return None;
        }
        Some(y as usize * self.width + x as usize)
    }

    /// Checks if a position is revealed (has been seen before)
    pub fn is_revealed(&self, pos: Position) -> bool {
        let (x, y) = pos.into();
        self.coords_to_index(x, y).map(|idx| self.revealed[idx]).unwrap_or(false)
    }

    /// Checks if a position is currently visible
    pub fn is_visible(&self, pos: Position) -> bool {
        let (x, y) = pos.into();
        self.coords_to_index(x, y).map(|idx| self.visible[idx]).unwrap_or(false)
    }

    /// Marks a position as revealed
    pub fn set_revealed(&mut self, pos: Position, value: bool) {
        let (x, y) = pos.into();
        if let Some(idx) = self.coords_to_index(x, y) {
            self.revealed.set(idx, value);
        }
    }

    /// Marks a position as visible
    pub fn set_visible(&mut self, pos: Position, value: bool) {
        let (x, y) = pos.into();
        if let Some(idx) = self.coords_to_index(x, y) {
            self.visible.set(idx, value);
            if value {
                self.revealed.set(idx, true);
            }
        }
    }

    /// Clears all visibility flags (called at the start of each turn)
    pub fn clear_visibility(&mut self) { self.visible.fill(false); }

    /// Updates the FOV for an entity at the given position with the given radius
    pub fn compute_fov(&mut self, q_terrain: &Query<&TerrainType>, map: &Map, origin: Position, radius: i32) {
        self.clear_visibility();

        // Always mark the origin as visible
        self.set_visible(origin, true);

        // Process all 8 octants using shadowcasting
        for octant in 0..8 {
            self.cast_light(q_terrain, map, origin, radius, 1, 0.0, 1.0, octant);
        }
    }

    /// Transform local coordinates (within an octant) to world coordinates
    fn transform_octant(&self, origin: Position, row: i32, col: i32, octant: i32) -> Position {
        let (origin_x, origin_y) = origin.into();

        let (x, y) = match octant {
            0 => (origin_x + col, origin_y - row), // Top-right
            1 => (origin_x + row, origin_y - col), // Right-top
            2 => (origin_x + row, origin_y + col), // Right-bottom
            3 => (origin_x + col, origin_y + row), // Bottom-right
            4 => (origin_x - col, origin_y + row), // Bottom-left
            5 => (origin_x - row, origin_y + col), // Left-bottom
            6 => (origin_x - row, origin_y - col), // Left-top
            7 => (origin_x - col, origin_y - row), // Top-left
            _ => panic!("Invalid octant: {}", octant),
        };

        Position::new(x, y)
    }

    /// Recursive function to calculate FOV for a single octant using shadowcasting
    fn cast_light(
        &mut self,
        q_terrain: &Query<&TerrainType>,
        map: &Map,
        origin: Position,
        radius: i32,
        row: i32,
        mut start_slope: f32,
        mut end_slope: f32,
        octant: i32,
    ) {
        // If the start slope is greater than the end slope, we're done
        if start_slope > end_slope {
            return;
        }

        // Limit by radius
        let max_row = radius;

        // Avoid going out of bounds
        if row > max_row {
            return;
        }

        // Calculate the range of columns in this row that might be visible
        let min_col = (row as f32 * start_slope).ceil() as i32;
        let max_col = (row as f32 * end_slope).floor() as i32;

        // Track whether we're currently blocked
        let mut was_blocked = false;

        // Scan each column in this row
        for col in min_col..=max_col {
            // Transform from octant coordinates to world coordinates
            let pos = self.transform_octant(origin, row, col, octant);

            // Skip if out of bounds
            if !map.in_bounds(pos) {
                continue;
            }

            // Calculate squared distance for circular FOV check
            let dx = pos.x() - origin.x();
            let dy = pos.y() - origin.y();
            let distance_squared = dx * dx + dy * dy;

            // Skip if beyond radius
            if distance_squared > radius * radius {
                continue;
            }

            // Mark the position as visible
            self.set_visible(pos, true);

            // Determine if this tile blocks vision
            let is_blocking = map
                .get_terrain(pos)
                .and_then(|entity| q_terrain.get(entity).ok())
                .map(|terrain| terrain.blocks_vision())
                .unwrap_or(false);

            if was_blocked {
                // We were previously blocked
                if is_blocking {
                    // Still blocked, continue
                    continue;
                } else {
                    // Transition from wall to open space
                    was_blocked = false;
                    // New start slope for the next scan
                    let new_start_slope = (col as f32 - 0.5) / row as f32;
                    // Continue with narrower beam
                    start_slope = new_start_slope;
                }
            } else {
                // We weren't previously blocked
                if is_blocking {
                    // This is a wall, calculate the end slope for the next scan
                    let new_end_slope = (col as f32 + 0.5) / row as f32;

                    // Recursively scan up to this wall
                    if col < max_col {
                        // Only if not the last column
                        self.cast_light(q_terrain, map, origin, radius, row + 1, start_slope, new_end_slope, octant);
                    }

                    // Mark that we're now blocked
                    was_blocked = true;
                    // Adjust end slope for scanning the rest of this row
                    end_slope = new_end_slope;
                }
            }
        }

        // If we reach the end without being blocked, continue to the next row
        if !was_blocked {
            self.cast_light(q_terrain, map, origin, radius, row + 1, start_slope, end_slope, octant);
        }
    }
}
