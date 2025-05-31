use bevy::{platform::collections::HashMap, prelude::*};
use bitvec::prelude::*;

use crate::{
    core::components::Position,
    core::{constants::ModelConstants, resources::Map},
};

/// FOV algorithm selection
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FovAlgorithm {
    /// Simple raycasting using Bresenham's line algorithm - more reliable for wall blocking
    Raycasting,
    #[default]
    /// Traditional shadowcasting algorithm - more efficient for large view distances
    Shadowcasting,
}

/// Field of view map using bit-level storage for memory efficiency.
/// This implementation uses the BitVec crate to store boolean values as individual bits.
#[derive(Resource)]
pub struct FovMap {
    width: usize,
    height: usize,
    visible: BitVec,
    revealed: BitVec,
    algorithm: FovAlgorithm,

    /// Cache for memoizing ray calculations based on direction vectors
    ray_cache: HashMap<(i32, i32), Vec<(i32, i32)>>,
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
            ray_cache: HashMap::new(),
        }
    }

    /// Sets the FOV algorithm to use
    pub fn set_algorithm(&mut self, algorithm: FovAlgorithm) { self.algorithm = algorithm; }

    /// Gets the current FOV algorithm
    pub fn get_algorithm(&self) -> FovAlgorithm { self.algorithm }

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
    pub fn compute_fov(&mut self, map: &Map, origin: Position, radius: u8) {
        self.clear_visibility();

        // Always mark the origin as visible
        self.set_visible(origin, true);

        // Dispatch to the appropriate algorithm
        match self.algorithm {
            FovAlgorithm::Raycasting => self.compute_fov_raycasting(map, origin, radius as i32),
            FovAlgorithm::Shadowcasting => self.compute_fov_shadowcasting(map, origin, radius as i32),
        }
    }
}

// RAYCASTING
impl FovMap {
    /// Clears the ray cache (useful for memory management or when map changes)
    pub fn clear_ray_cache(&mut self) { self.ray_cache.clear(); }

    /// Gets the current size of the ray cache (for debugging/monitoring)
    pub fn ray_cache_size(&self) -> usize { self.ray_cache.len() }

    /// Add all 8 octant points for the circle
    fn add_circle_octant_points(&self, points: &mut Vec<(i32, i32)>, x: i32, y: i32) {
        points.push((x, y));
        points.push((-x, y));
        points.push((x, -y));
        points.push((-x, -y));
        points.push((y, x));
        points.push((-y, x));
        points.push((y, -x));
        points.push((-y, -x));
    }

    /// Optimized raycasting FOV implementation using boundary ray casting
    /// Complexity: O(r²) instead of O(r³)
    fn compute_fov_raycasting(&mut self, map: &Map, origin: Position, radius: i32) {
        let (origin_x, origin_y) = origin.into();

        // Generate boundary points for the circle using Bresenham's circle algorithm
        let boundary_points = self.get_circle_boundary_points(radius);

        // Cast rays to each boundary point
        for &(dx, dy) in &boundary_points {
            // Get or compute the ray path for this direction
            let ray_path = self.get_or_compute_ray_path(dx, dy, radius);

            // Trace along the ray path from origin
            let mut blocked = false;
            for &(step_dx, step_dy) in &ray_path {
                let step_x = origin_x + step_dx;
                let step_y = origin_y + step_dy;
                let step_pos = Position::new(step_x, step_y);

                // Skip if out of bounds
                if !map.in_bounds(step_pos) {
                    break;
                }

                // Skip origin (already marked visible)
                if step_pos == origin {
                    continue;
                }

                // If not blocked, mark as visible
                if !blocked {
                    self.set_visible(step_pos, true);
                }

                // Check if this position blocks further vision
                if let Some(terrain) = map.get_terrain(step_pos)
                    && terrain.blocks_vision()
                {
                    blocked = true;
                    // Continue the ray to mark tiles behind walls as not visible
                    // but don't break - we still need to process the rest of the ray
                }
            }
        }
    }

    /// Generate boundary points of a circle using a modified Bresenham's circle algorithm
    /// Returns relative coordinates (dx, dy) from the center
    fn get_circle_boundary_points(&self, radius: i32) -> Vec<(i32, i32)> {
        let mut points = Vec::new();

        if radius == 0 {
            return points;
        }

        // Use Bresenham's circle algorithm to get boundary points
        let mut x = 0;
        let mut y = radius;
        let mut d = 3 - 2 * radius;

        // Add the initial points
        self.add_circle_octant_points(&mut points, x, y);

        while y >= x {
            x += 1;

            if d > 0 {
                y -= 1;
                d = d + 4 * (x - y) + 10;
            } else {
                d = d + 4 * x + 6;
            }

            self.add_circle_octant_points(&mut points, x, y);
        }

        // Add additional rays for better coverage - cast rays to square boundary as well
        for i in -radius..=radius {
            // Top and bottom edges
            points.push((i, radius));
            points.push((i, -radius));
            // Left and right edges
            points.push((radius, i));
            points.push((-radius, i));
        }

        // Remove duplicates and sort for consistent ordering
        points.sort_unstable();
        points.dedup();

        points
    }
    /// Get or compute the ray path for a given direction vector
    /// Uses memoization to avoid redundant calculations
    fn get_or_compute_ray_path(&mut self, dx: i32, dy: i32, max_radius: i32) -> Vec<(i32, i32)> {
        // Normalize the direction vector to its simplest form for better cache hits
        let gcd = crate::utils::gcd(dx.abs(), dy.abs());
        let normalized_dx = if gcd > 0 { dx / gcd } else { dx };
        let normalized_dy = if gcd > 0 { dy / gcd } else { dy };

        let cache_key = (normalized_dx, normalized_dy);

        if let Some(cached_path) = self.ray_cache.get(&cache_key) {
            // Filter the cached path to only include points within our radius
            return cached_path
                .iter()
                .filter(|&&(step_dx, step_dy)| {
                    let dist_sq = step_dx * step_dx + step_dy * step_dy;
                    dist_sq <= max_radius * max_radius
                })
                .copied()
                .collect();
        }

        // Compute the ray path using Bresenham's line algorithm
        let mut path = Vec::new();
        let mut current_x = 0;
        let mut current_y = 0;

        // Extend the ray beyond the target to ensure we cover the full radius
        let extended_radius = max_radius + 2;
        let target_x = normalized_dx * extended_radius;
        let target_y = normalized_dy * extended_radius;

        let dx_abs = target_x.abs();
        let dy_abs = target_y.abs();
        let sx = if target_x > 0 { 1 } else { -1 };
        let sy = if target_y > 0 { 1 } else { -1 };
        let mut err = dx_abs - dy_abs;

        loop {
            path.push((current_x, current_y));

            // Stop if we've gone far enough
            let dist_sq = current_x * current_x + current_y * current_y;
            if dist_sq > extended_radius * extended_radius {
                break;
            }

            if current_x == target_x && current_y == target_y {
                break;
            }

            let e2 = 2 * err;
            if e2 > -dy_abs {
                err -= dy_abs;
                current_x += sx;
            }
            if e2 < dx_abs {
                err += dx_abs;
                current_y += sy;
            }
        }

        // Cache the computed path
        self.ray_cache.insert(cache_key, path.clone());

        // Filter to return only points within our radius
        path.into_iter()
            .filter(|&(step_dx, step_dy)| {
                let dist_sq = step_dx * step_dx + step_dy * step_dy;
                dist_sq <= max_radius * max_radius
            })
            .collect()
    }
}

// SHADOWCASTING
impl FovMap {
    /// Shadowcasting FOV implementation
    fn compute_fov_shadowcasting(&mut self, map: &Map, origin: Position, radius: i32) {
        // Process all 8 octants using shadowcasting
        for octant in 0..8 {
            self.cast_light(map, origin, radius, 1, 0.0, 1.0, octant);
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
            _ => panic!("Invalid octant: {octant}"),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{components::TerrainType, resources::Map};
    use std::time::Instant;

    fn create_test_map(width: usize, height: usize) -> Map {
        let mut map = Map::new((width as u32, height as u32));

        // Create a simple test map with some walls
        for x in 0..width {
            for y in 0..height {
                let pos = Position::new(x as i32, y as i32);

                // Add walls around the border
                if x == 0 || y == 0 || x == width - 1 || y == height - 1 {
                    map.set_terrain(pos, TerrainType::Wall);
                } else if x == 5 && y > 2 && y < 8 {
                    // Add a vertical wall for testing line of sight
                    map.set_terrain(pos, TerrainType::Wall);
                } else {
                    map.set_terrain(pos, TerrainType::Floor);
                }
            }
        }

        map
    }

    #[test]
    fn test_fov_raycasting_basic() {
        let mut fov_map = FovMap::new(20, 20);
        let map = create_test_map(20, 20);
        let origin = Position::new(10, 10);
        let radius = 5;

        fov_map.set_algorithm(FovAlgorithm::Raycasting);
        fov_map.compute_fov(&map, origin, radius);

        // Origin should be visible
        assert!(fov_map.is_visible(origin));
        assert!(fov_map.is_revealed(origin));

        // Adjacent tiles should be visible
        assert!(fov_map.is_visible(Position::new(11, 10)));
        assert!(fov_map.is_visible(Position::new(9, 10)));
        assert!(fov_map.is_visible(Position::new(10, 11)));
        assert!(fov_map.is_visible(Position::new(10, 9)));

        // Tiles beyond radius should not be visible
        assert!(!fov_map.is_visible(Position::new(16, 10)));
        assert!(!fov_map.is_visible(Position::new(4, 10)));
    }

    #[test]
    fn test_fov_algorithms_consistency() {
        let mut fov_map = FovMap::new(20, 20);
        let map = create_test_map(20, 20);
        let origin = Position::new(10, 10);
        let radius = 5;

        // Test raycasting
        fov_map.set_algorithm(FovAlgorithm::Raycasting);
        fov_map.compute_fov(&map, origin, radius);
        let mut raycasting_visible = Vec::new();
        for x in 0..20 {
            for y in 0..20 {
                let pos = Position::new(x, y);
                if fov_map.is_visible(pos) {
                    raycasting_visible.push(pos);
                }
            }
        }

        // Test shadowcasting
        fov_map.set_algorithm(FovAlgorithm::Shadowcasting);
        fov_map.compute_fov(&map, origin, radius);
        let mut shadowcasting_visible = Vec::new();
        for x in 0..20 {
            for y in 0..20 {
                let pos = Position::new(x, y);
                if fov_map.is_visible(pos) {
                    shadowcasting_visible.push(pos);
                }
            }
        }

        // Both algorithms should see the origin
        assert!(raycasting_visible.contains(&origin));
        assert!(shadowcasting_visible.contains(&origin));

        // The number of visible tiles should be similar (within reasonable bounds)
        let diff = (raycasting_visible.len() as i32 - shadowcasting_visible.len() as i32).abs();
        assert!(
            diff < 10,
            "Algorithms should produce similar results. Raycasting: {}, Shadowcasting: {}",
            raycasting_visible.len(),
            shadowcasting_visible.len()
        );
    }

    #[test]
    fn test_ray_cache_functionality() {
        let mut fov_map = FovMap::new(20, 20);
        let map = create_test_map(20, 20);
        let origin = Position::new(10, 10);
        let radius = 5;

        fov_map.set_algorithm(FovAlgorithm::Raycasting);

        // Initial cache should be empty
        assert_eq!(fov_map.ray_cache_size(), 0);

        // First computation should populate cache
        fov_map.compute_fov(&map, origin, radius);
        let cache_size_after_first = fov_map.ray_cache_size();
        assert!(cache_size_after_first > 0);

        // Second computation should reuse cache (same size)
        fov_map.compute_fov(&map, origin, radius);
        assert_eq!(fov_map.ray_cache_size(), cache_size_after_first);

        // Clear cache
        fov_map.clear_ray_cache();
        assert_eq!(fov_map.ray_cache_size(), 0);
    }

    #[test]
    fn test_circle_boundary_points() {
        let fov_map = FovMap::new(20, 20);

        // Test radius 0
        let points_r0 = fov_map.get_circle_boundary_points(0);
        assert!(points_r0.is_empty());

        // Test radius 1
        let points_r1 = fov_map.get_circle_boundary_points(1);
        assert!(!points_r1.is_empty());

        // Should include cardinal directions
        assert!(points_r1.contains(&(1, 0)));
        assert!(points_r1.contains(&(-1, 0)));
        assert!(points_r1.contains(&(0, 1)));
        assert!(points_r1.contains(&(0, -1)));

        // Test radius 3
        let points_r3 = fov_map.get_circle_boundary_points(3);
        assert!(points_r3.len() > points_r1.len());
    }

    #[test]
    fn test_wall_blocking() {
        let mut fov_map = FovMap::new(20, 20);
        let map = create_test_map(20, 20);
        let origin = Position::new(3, 5);
        let radius = 8;

        fov_map.set_algorithm(FovAlgorithm::Raycasting);
        fov_map.compute_fov(&map, origin, radius);

        // Position behind the wall at x=5 should not be visible
        let behind_wall = Position::new(7, 5);
        assert!(!fov_map.is_visible(behind_wall), "Position behind wall should not be visible");

        // Position in front of the wall should be visible
        let in_front = Position::new(4, 5);
        assert!(fov_map.is_visible(in_front), "Position in front of wall should be visible");
    }

    #[test]
    #[ignore] // Use cargo test -- --ignored to run performance tests
    fn test_performance_comparison() {
        let mut fov_map = FovMap::new(100, 100);
        let map = create_test_map(100, 100);
        let origin = Position::new(50, 50);
        let radius = 20;
        let iterations = 100;

        // Test raycasting performance
        fov_map.set_algorithm(FovAlgorithm::Raycasting);
        fov_map.clear_ray_cache(); // Start with empty cache

        let start = Instant::now();
        for _ in 0..iterations {
            fov_map.compute_fov(&map, origin, radius);
        }
        let raycasting_time = start.elapsed();

        // Test shadowcasting performance
        fov_map.set_algorithm(FovAlgorithm::Shadowcasting);

        let start = Instant::now();
        for _ in 0..iterations {
            fov_map.compute_fov(&map, origin, radius);
        }
        let shadowcasting_time = start.elapsed();

        println!("Raycasting time: {raycasting_time:?}");
        println!("Shadowcasting time: {shadowcasting_time:?}");
        println!("Ray cache size: {}", fov_map.ray_cache_size());

        // The optimized raycasting should be reasonable compared to shadowcasting
        // Shadowcasting is expected to be faster, but raycasting should be within 10x
        // This is a significant improvement from the original O(r³) implementation
        assert!(
            raycasting_time.as_millis() < shadowcasting_time.as_millis() * 10,
            "Optimized raycasting should be within 10x of shadowcasting performance. Raycasting: {}ms, Shadowcasting: {}ms",
            raycasting_time.as_millis(),
            shadowcasting_time.as_millis()
        );
    }
}
