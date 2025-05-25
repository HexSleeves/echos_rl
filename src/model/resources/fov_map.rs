use crate::model::{
    ModelConstants,
    components::{Position, TerrainType},
    resources::Map,
};
use bevy::prelude::*;
use bitvec::prelude::*;
use std::collections::HashMap;
use thiserror::Error;

/// Errors that can occur during FOV computation
#[derive(Debug, Error)]
pub enum FovError {
    #[error("Invalid octant: {0}. Must be 0-7")]
    InvalidOctant(i32),
    #[error("Position out of bounds: ({0}, {1})")]
    OutOfBounds(i32, i32),
}

/// Represents the 8 octants used in shadowcasting algorithm
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Octant {
    TopRight = 0,
    RightTop = 1,
    RightBottom = 2,
    BottomRight = 3,
    BottomLeft = 4,
    LeftBottom = 5,
    LeftTop = 6,
    TopLeft = 7,
}

impl Octant {
    /// All 8 octants in order
    pub const ALL: [Self; 8] = [
        Self::TopRight,
        Self::RightTop,
        Self::RightBottom,
        Self::BottomRight,
        Self::BottomLeft,
        Self::LeftBottom,
        Self::LeftTop,
        Self::TopLeft,
    ];

    /// Transform local octant coordinates to world coordinates
    pub fn transform_coords(self, origin: Position, row: i32, col: i32) -> Position {
        let (origin_x, origin_y) = origin.into();

        let (x, y) = match self {
            Self::TopRight => (origin_x + col, origin_y - row),
            Self::RightTop => (origin_x + row, origin_y - col),
            Self::RightBottom => (origin_x + row, origin_y + col),
            Self::BottomRight => (origin_x + col, origin_y + row),
            Self::BottomLeft => (origin_x - col, origin_y + row),
            Self::LeftBottom => (origin_x - row, origin_y + col),
            Self::LeftTop => (origin_x - row, origin_y - col),
            Self::TopLeft => (origin_x - col, origin_y - row),
        };

        Position::new(x, y)
    }
}

impl TryFrom<i32> for Octant {
    type Error = FovError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::TopRight),
            1 => Ok(Self::RightTop),
            2 => Ok(Self::RightBottom),
            3 => Ok(Self::BottomRight),
            4 => Ok(Self::BottomLeft),
            5 => Ok(Self::LeftBottom),
            6 => Ok(Self::LeftTop),
            7 => Ok(Self::TopLeft),
            _ => Err(FovError::InvalidOctant(value)),
        }
    }
}

/// Field of view map using bit-level storage for memory efficiency.
///
/// This implementation uses the BitVec crate to store boolean values as individual bits,
/// providing excellent memory efficiency for large maps. It uses the recursive shadowcasting
/// algorithm to compute field of view, which divides the visible area into 8 octants and
/// processes each one separately.
///
/// # Algorithm Details
///
/// The shadowcasting algorithm maintains a "light cone" defined by start and end slopes.
/// When it encounters blocking terrain, it recursively processes the area before the blocker
/// and narrows the cone for the area after. This ensures accurate visibility calculations
/// while maintaining good performance.
#[derive(Resource)]
pub struct FovMap {
    width: usize,
    height: usize,
    revealed: BitVec,
    visible: BitVec,
    /// Cache for terrain blocking status to avoid repeated ECS queries
    terrain_cache: HashMap<Position, bool>,
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
        Self { width, height, revealed: bitvec![0; size], visible: bitvec![0; size], terrain_cache: HashMap::new() }
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
    pub fn clear_visibility(&mut self) {
        self.visible.fill(false);
        // Clear terrain cache to ensure fresh data each turn
        self.terrain_cache.clear();
    }

    /// Updates the FOV for an entity at the given position with the given radius
    ///
    /// # Arguments
    /// * `q_terrain` - Query for terrain components to check blocking status
    /// * `map` - The game map for bounds checking and terrain lookup
    /// * `origin` - The center point from which to calculate visibility
    /// * `radius` - Maximum visibility distance (circular)
    ///
    /// # Performance Notes
    /// This method uses caching to avoid repeated ECS queries for the same terrain tiles.
    /// The cache is cleared at the start of each computation to ensure fresh data.
    pub fn compute_fov(&mut self, q_terrain: &Query<&TerrainType>, map: &Map, origin: Position, radius: i32) {
        // Input validation
        if radius <= 0 {
            self.clear_visibility();
            self.set_visible(origin, true);
            return;
        }

        // Handle single-tile radius specially for performance
        if radius == 1 {
            self.compute_fov_radius_one(map, origin);
            return;
        }

        self.clear_visibility();

        // Always mark the origin as visible
        self.set_visible(origin, true);

        // Process all 8 octants using shadowcasting
        for octant in Octant::ALL {
            self.cast_light(q_terrain, map, origin, radius, 1, 0.0, 1.0, octant);
        }
    }

    /// Optimized FOV computation for radius = 1
    fn compute_fov_radius_one(&mut self, map: &Map, origin: Position) {
        self.clear_visibility();
        self.set_visible(origin, true);

        // Check all 8 adjacent tiles
        let directions = [(-1, -1), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1)];

        for (dx, dy) in directions {
            let pos = Position::new(origin.x() + dx, origin.y() + dy);
            if map.in_bounds(pos) {
                self.set_visible(pos, true);
            }
        }
    }

    /// Recursive shadowcasting algorithm for computing field of view in a single octant.
    ///
    /// This implementation uses the recursive shadowcasting algorithm which maintains
    /// a "light cone" defined by start and end slopes. When it encounters blocking terrain,
    /// it recursively processes the area before the blocker and narrows the cone for the area after.
    ///
    /// # Parameters
    /// - `origin`: The center point from which to calculate visibility
    /// - `radius`: Maximum visibility distance (circular)
    /// - `row`: Current row being processed in octant coordinates
    /// - `start_slope`: Left boundary of the current visibility cone (0.0 to 1.0)
    /// - `end_slope`: Right boundary of the current visibility cone (0.0 to 1.0)
    /// - `octant`: Which of the 8 octants to process
    ///
    /// # Algorithm Details
    /// The algorithm maintains a "light cone" defined by start and end slopes.
    /// When it encounters blocking terrain, it recursively processes the area
    /// before the blocker and narrows the cone for the area after.
    fn cast_light(
        &mut self,
        q_terrain: &Query<&TerrainType>,
        map: &Map,
        origin: Position,
        radius: i32,
        row: i32,
        mut start_slope: f64,
        end_slope: f64,
        octant: Octant,
    ) {
        // If the start slope is greater than the end slope, we're done
        if start_slope > end_slope {
            return;
        }

        // Avoid going out of bounds
        if row > radius {
            return;
        }

        // Calculate the range of columns in this row that might be visible
        // Using f64 for better precision
        let min_col = (row as f64 * start_slope).ceil() as i32;
        let max_col = (row as f64 * end_slope).floor() as i32;

        // Pre-calculate radius squared for distance checks
        let radius_squared = radius * radius;

        // Track whether we're currently blocked
        let mut was_blocked = false;

        // Scan each column in this row
        for col in min_col..=max_col {
            // Transform from octant coordinates to world coordinates
            let pos = octant.transform_coords(origin, row, col);

            // Skip if out of bounds
            if !map.in_bounds(pos) {
                continue;
            }

            // Calculate squared distance for circular FOV check
            let dx = pos.x() - origin.x();
            let dy = pos.y() - origin.y();
            let distance_squared = dx * dx + dy * dy;

            // Skip if beyond radius
            if distance_squared > radius_squared {
                continue;
            }

            // Mark the position as visible
            self.set_visible(pos, true);

            // Determine if this tile blocks vision using cache
            let is_blocking = *self.terrain_cache.entry(pos).or_insert_with(|| {
                map.get_terrain_entity(pos)
                    .and_then(|entity| q_terrain.get(entity).ok())
                    .map(|terrain| terrain.blocks_vision())
                    .unwrap_or(false)
            });

            if was_blocked {
                // We were previously blocked
                if !is_blocking {
                    // Transition from wall to open space - start new scan
                    was_blocked = false;
                    let new_start_slope = (col as f64 - 0.5) / row as f64;
                    start_slope = new_start_slope;
                }
                // Continue processing current row regardless
            } else {
                // We weren't previously blocked
                if is_blocking {
                    // Hit a wall - recursively process area before wall
                    let new_end_slope = (col as f64 + 0.5) / row as f64;

                    // Recursively scan up to this wall
                    self.cast_light(q_terrain, map, origin, radius, row + 1, start_slope, new_end_slope, octant);

                    // Mark that we're now blocked
                    was_blocked = true;
                }
            }
        }

        // If we reach the end without being blocked, continue to the next row
        if !was_blocked {
            self.cast_light(q_terrain, map, origin, radius, row + 1, start_slope, end_slope, octant);
        }
    }

    /// Get the dimensions of the FOV map
    pub fn dimensions(&self) -> (usize, usize) { (self.width, self.height) }

    /// Get the total number of tiles in the map
    pub fn tile_count(&self) -> usize { self.width * self.height }

    /// Get the number of currently visible tiles
    pub fn visible_tile_count(&self) -> usize { self.visible.count_ones() }

    /// Get the number of revealed tiles
    pub fn revealed_tile_count(&self) -> usize { self.revealed.count_ones() }

    /// Check if any tiles are visible
    pub fn has_visible_tiles(&self) -> bool { self.visible.any() }

    /// Clear all revealed tiles (useful for level transitions)
    pub fn clear_revealed(&mut self) { self.revealed.fill(false); }

    /// Get all currently visible positions
    pub fn get_visible_positions(&self) -> Vec<Position> {
        let mut positions = Vec::new();
        for y in 0..self.height {
            for x in 0..self.width {
                let pos = Position::new(x as i32, y as i32);
                if self.is_visible(pos) {
                    positions.push(pos);
                }
            }
        }
        positions
    }

    /// Get all revealed positions
    pub fn get_revealed_positions(&self) -> Vec<Position> {
        let mut positions = Vec::new();
        for y in 0..self.height {
            for x in 0..self.width {
                let pos = Position::new(x as i32, y as i32);
                if self.is_revealed(pos) {
                    positions.push(pos);
                }
            }
        }
        positions
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // Simplified tests focusing on the core FOV data structures

    #[test]
    fn test_fov_map_creation() {
        let fov_map = FovMap::new(10, 10);
        assert_eq!(fov_map.dimensions(), (10, 10));
        assert_eq!(fov_map.tile_count(), 100);
        assert_eq!(fov_map.visible_tile_count(), 0);
        assert_eq!(fov_map.revealed_tile_count(), 0);
        assert!(!fov_map.has_visible_tiles());
    }

    #[test]
    fn test_coords_to_index() {
        let fov_map = FovMap::new(5, 5);

        // Test valid coordinates
        assert_eq!(fov_map.coords_to_index(0, 0), Some(0));
        assert_eq!(fov_map.coords_to_index(4, 4), Some(24));
        assert_eq!(fov_map.coords_to_index(2, 3), Some(17)); // 3 * 5 + 2

        // Test invalid coordinates
        assert_eq!(fov_map.coords_to_index(-1, 0), None);
        assert_eq!(fov_map.coords_to_index(0, -1), None);
        assert_eq!(fov_map.coords_to_index(5, 0), None);
        assert_eq!(fov_map.coords_to_index(0, 5), None);
    }

    #[test]
    fn test_octant_transform() {
        let origin = Position::new(5, 5);

        // Test each octant transformation
        assert_eq!(Octant::TopRight.transform_coords(origin, 1, 1), Position::new(6, 4));
        assert_eq!(Octant::RightTop.transform_coords(origin, 1, 1), Position::new(6, 4));
        assert_eq!(Octant::RightBottom.transform_coords(origin, 1, 1), Position::new(6, 6));
        assert_eq!(Octant::BottomRight.transform_coords(origin, 1, 1), Position::new(6, 6));
        assert_eq!(Octant::BottomLeft.transform_coords(origin, 1, 1), Position::new(4, 6));
        assert_eq!(Octant::LeftBottom.transform_coords(origin, 1, 1), Position::new(4, 6));
        assert_eq!(Octant::LeftTop.transform_coords(origin, 1, 1), Position::new(4, 4));
        assert_eq!(Octant::TopLeft.transform_coords(origin, 1, 1), Position::new(4, 4));
    }

    #[test]
    fn test_octant_try_from() {
        for i in 0..8 {
            assert!(Octant::try_from(i).is_ok());
        }
        assert!(Octant::try_from(-1).is_err());
        assert!(Octant::try_from(8).is_err());
    }

    #[test]
    fn test_visibility_setting() {
        let mut fov_map = FovMap::new(10, 10);
        let pos = Position::new(5, 5);

        // Initially not visible or revealed
        assert!(!fov_map.is_visible(pos));
        assert!(!fov_map.is_revealed(pos));

        // Set visible
        fov_map.set_visible(pos, true);
        assert!(fov_map.is_visible(pos));
        assert!(fov_map.is_revealed(pos)); // Should also be revealed

        // Clear visibility
        fov_map.clear_visibility();
        assert!(!fov_map.is_visible(pos));
        assert!(fov_map.is_revealed(pos)); // Should still be revealed

        // Set revealed directly
        let pos2 = Position::new(3, 3);
        fov_map.set_revealed(pos2, true);
        assert!(!fov_map.is_visible(pos2));
        assert!(fov_map.is_revealed(pos2));
    }

    #[test]
    fn test_utility_methods() {
        let mut fov_map = FovMap::new(5, 5);

        // Set some tiles as visible
        fov_map.set_visible(Position::new(2, 2), true);
        fov_map.set_visible(Position::new(3, 3), true);

        assert!(fov_map.has_visible_tiles());
        assert_eq!(fov_map.visible_tile_count(), 2);
        assert_eq!(fov_map.revealed_tile_count(), 2);

        let visible_positions = fov_map.get_visible_positions();
        assert_eq!(visible_positions.len(), 2);
        assert!(visible_positions.contains(&Position::new(2, 2)));
        assert!(visible_positions.contains(&Position::new(3, 3)));

        fov_map.clear_revealed();
        assert_eq!(fov_map.revealed_tile_count(), 0);
    }
}
