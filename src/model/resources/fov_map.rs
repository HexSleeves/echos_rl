use crate::model::{ModelConstants, components::Position, resources::Map};
use bevy::prelude::*;
use bitvec::prelude::*;

/// FOV algorithm selection
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FovAlgorithm {
    /// Simple raycasting using Bresenham's line algorithm - more reliable for wall blocking
    Raycasting,
    /// Traditional shadowcasting algorithm - more efficient for large view distances
    Shadowcasting,
}

impl Default for FovAlgorithm {
    fn default() -> Self {
        Self::Raycasting // Default to the more reliable algorithm
    }
}

/// Field of view map using bit-level storage for memory efficiency.
/// This implementation uses the BitVec crate to store boolean values as individual bits.
#[derive(Resource)]
pub struct FovMap {
    width: usize,
    height: usize,
    revealed: BitVec,
    visible: BitVec,
    algorithm: FovAlgorithm,
}

impl FromWorld for FovMap {
    fn from_world(_world: &mut World) -> Self {
        let size = (ModelConstants::MAP_WIDTH as usize, ModelConstants::MAP_HEIGHT as usize);
        Self::new(size.0, size.1)
    }
}

impl FovMap {
    pub fn new(width: usize, height: usize) -> Self {
        let size = width * height;
        Self {
            width,
            height,
            revealed: bitvec![0; size],
            visible: bitvec![0; size],
            algorithm: FovAlgorithm::default(),
        }
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

    /// Sets the FOV algorithm to use
    pub fn set_algorithm(&mut self, algorithm: FovAlgorithm) { self.algorithm = algorithm; }

    /// Gets the current FOV algorithm
    pub fn get_algorithm(&self) -> FovAlgorithm { self.algorithm }

    /// Updates the FOV for an entity at the given position with the given radius
    pub fn compute_fov(&mut self, map: &Map, origin: Position, radius: i32) {
        self.clear_visibility();

        // Always mark the origin as visible
        self.set_visible(origin, true);

        // Dispatch to the appropriate algorithm
        match self.algorithm {
            FovAlgorithm::Raycasting => self.compute_fov_raycasting(map, origin, radius),
            FovAlgorithm::Shadowcasting => self.compute_fov_shadowcasting(map, origin, radius),
        }
    }

    /// Raycasting FOV implementation using Bresenham's line algorithm
    fn compute_fov_raycasting(&mut self, map: &Map, origin: Position, radius: i32) {
        let (origin_x, origin_y) = origin.into();

        for y in (origin_y - radius)..=(origin_y + radius) {
            for x in (origin_x - radius)..=(origin_x + radius) {
                let target = Position::new(x, y);

                // Skip if out of bounds
                if !map.in_bounds(target) {
                    continue;
                }

                // Check distance
                let dx = x - origin_x;
                let dy = y - origin_y;
                let distance_squared = dx * dx + dy * dy;

                if distance_squared > radius * radius {
                    continue;
                }

                // Skip origin (already marked visible)
                if target == origin {
                    continue;
                }

                // Check line of sight using Bresenham's line algorithm
                if self.has_line_of_sight(map, origin, target) {
                    self.set_visible(target, true);
                }
            }
        }
    }

    /// Shadowcasting FOV implementation
    fn compute_fov_shadowcasting(&mut self, map: &Map, origin: Position, radius: i32) {
        // Process all 8 octants using shadowcasting
        for octant in 0..8 {
            self.cast_light(map, origin, radius, 1, 0.0, 1.0, octant);
        }
    }

    /// Check if there's a clear line of sight between two points using Bresenham's line algorithm
    fn has_line_of_sight(&self, map: &Map, start: Position, end: Position) -> bool {
        let (x0, y0) = start.into();
        let (x1, y1) = end.into();

        let dx = (x1 - x0).abs();
        let dy = (y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx - dy;

        let mut x = x0;
        let mut y = y0;

        loop {
            // Check if current position blocks vision (but not the end point)
            let current_pos = Position::new(x, y);
            if current_pos != start && current_pos != end {
                if let Some(terrain) = map.get_terrain(current_pos) {
                    if terrain.blocks_vision() {
                        return false; // Line of sight blocked
                    }
                }
            }

            // Reached the end point
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

        true // Line of sight is clear
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
        map: &Map,
        origin: Position,
        radius: i32,
        row: i32,
        start_slope: f32,
        end_slope: f32,
        octant: i32,
    ) {
        // Base cases
        if start_slope > end_slope || row > radius {
            return;
        }

        // Calculate the range of columns in this row that might be visible
        let min_col = ((row as f32 * start_slope) + 0.5).floor() as i32;
        let max_col = ((row as f32 * end_slope) - 0.5).ceil() as i32;

        // Track blocking state
        let mut prev_blocked = false;
        let mut new_start = start_slope;

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

            // Calculate slopes for this column
            let left_slope = (col as f32 - 0.5) / (row as f32 + 0.5);
            let right_slope = (col as f32 + 0.5) / (row as f32 - 0.5);

            // Determine if this tile blocks vision
            let is_blocking = map.get_terrain(pos).map(|terrain| terrain.blocks_vision()).unwrap_or(false);

            // Check if this column is within our visible range and mark as visible
            if start_slope < right_slope && end_slope > left_slope {
                self.set_visible(pos, true);
            }

            // Handle blocking logic - only for tiles that are fully within our scan range
            if start_slope <= left_slope && end_slope >= right_slope {
                if prev_blocked {
                    // We were in a shadow
                    if !is_blocking {
                        // Exiting shadow - start new scan from this column
                        new_start = left_slope;
                        prev_blocked = false;
                    }
                } else {
                    // We were not in a shadow
                    if is_blocking {
                        // Entering shadow - recursive call for the visible area before this blocker
                        // Use right_slope to ensure we don't see past this blocker
                        self.cast_light(map, origin, radius, row + 1, new_start, right_slope, octant);
                        prev_blocked = true;
                    }
                }
            }
        }

        // Continue to next row if not completely blocked
        if !prev_blocked {
            self.cast_light(map, origin, radius, row + 1, new_start, end_slope, octant);
        }
    }
}
