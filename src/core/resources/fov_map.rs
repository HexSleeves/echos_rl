use bevy::prelude::*;
use bitvec::prelude::*;
use brtk::fov::{FovAlgorithm as BrtkFovAlgorithm, FovReceiver, Shadowcast, VisibilityMap};

use crate::core::{components::Position, constants::ModelConstants, resources::Map};

/// Field of view map using bit-level storage for memory efficiency.
/// This implementation uses the BitVec crate to store boolean values as individual bits.
#[derive(Resource)]
pub struct FovMap {
    width: usize,
    height: usize,
    visible: BitVec,
    revealed: BitVec,
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
    pub fn compute_fov(&mut self, map: &Map, origin: Position, radius: u8) {
        self.clear_visibility();

        // Always mark the origin as visible
        self.set_visible(origin, true);

        // Dispatch to the appropriate algorithm
        self.compute_fov_advanced_shadowcasting(map, origin, radius as u32)
    }

    // ============================================================================
    // ENTITY VISIBILITY UTILITY FUNCTIONS
    // ============================================================================

    /// Checks if an observer at one position can see a target position
    ///
    /// This function computes a temporary FOV from the observer's position and checks
    /// if the target position is visible within the given range.
    ///
    /// # Arguments
    /// * `observer_pos` - Position of the entity doing the observing
    /// * `target_pos` - Position to check visibility for
    /// * `range` - Maximum vision range for the observer
    /// * `map` - Map to use for opacity calculations
    ///
    /// # Returns
    /// `true` if the target position is visible from the observer position, `false` otherwise
    pub fn can_see_position(observer_pos: Position, target_pos: Position, range: u8, map: &Map) -> bool {
        // Quick distance check - if target is beyond range, it's not visible
        let distance = observer_pos.fov_range_distance(&target_pos);
        if distance > range as f32 {
            return false;
        }

        // Create temporary visibility map for this calculation
        let mut temp_visibility = VisibilityMap::new();

        // Compute FOV from observer position
        let observer_coords = (observer_pos.x(), observer_pos.y());
        Shadowcast::compute_fov(observer_coords, 0, range as u32, map, &mut temp_visibility);

        // Check if target position is visible
        let target_coords = (target_pos.x(), target_pos.y());
        temp_visibility.get_visible(target_coords)
    }

    /// Checks if an observer entity can see a target entity
    ///
    /// This is a semantic wrapper around `can_see_position` that makes entity-to-entity
    /// visibility checks more explicit and readable.
    ///
    /// # Arguments
    /// * `observer_pos` - Position of the observing entity
    /// * `observer_range` - Vision range of the observing entity
    /// * `target_pos` - Position of the target entity
    /// * `map` - Map to use for opacity calculations
    ///
    /// # Returns
    /// `true` if the observer can see the target entity, `false` otherwise
    pub fn can_see_entity(
        observer_pos: Position,
        observer_range: u8,
        target_pos: Position,
        map: &Map,
    ) -> bool {
        Self::can_see_position(observer_pos, target_pos, observer_range, map)
    }

    /// Computes a temporary FOV for an entity and returns the visibility map
    ///
    /// This function is useful when you need to perform multiple visibility queries
    /// for the same observer, as it allows you to compute the FOV once and reuse it.
    ///
    /// # Arguments
    /// * `observer_pos` - Position of the observing entity
    /// * `range` - Vision range for the observer
    /// * `map` - Map to use for opacity calculations
    ///
    /// # Returns
    /// A `VisibilityMap` containing all positions visible from the observer position
    pub fn compute_temporary_fov(observer_pos: Position, range: u8, map: &Map) -> VisibilityMap {
        let mut temp_visibility = VisibilityMap::new();
        let observer_coords = (observer_pos.x(), observer_pos.y());
        Shadowcast::compute_fov(observer_coords, 0, range as u32, map, &mut temp_visibility);
        temp_visibility
    }
}

// ADVANCED SHADOWCASTING
impl FovMap {
    /// Advanced shadowcasting FOV implementation using the brtk FOV system
    fn compute_fov_advanced_shadowcasting(&mut self, map: &Map, origin: Position, radius: u32) {
        // Use the advanced shadowcasting algorithm from brtk
        let origin_coords = (origin.x(), origin.y());
        Shadowcast::compute_fov(origin_coords, 0, radius, map, self);
    }
}

impl FovReceiver for FovMap {
    fn set_visible(&mut self, position: (i32, i32)) {
        let pos = Position::new(position.0, position.1);
        self.set_visible(pos, true);
    }

    fn get_visible(&self, position: (i32, i32)) -> bool {
        let pos = Position::new(position.0, position.1);
        self.is_visible(pos)
    }

    fn clear_visible(&mut self) { self.clear_visibility(); }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gameplay::world::components::TerrainType;

    fn create_test_map(width: u32, height: u32) -> Map {
        let mut map = Map::new((width, height));

        // Fill with floors (walkable, non-opaque)
        for y in 0..height {
            for x in 0..width {
                let pos = Position::new(x as i32, y as i32);
                map.set_terrain(pos, TerrainType::Floor);
            }
        }

        map
    }

    fn create_test_map_with_wall(width: u32, height: u32, wall_pos: Position) -> Map {
        let mut map = create_test_map(width, height);
        map.set_terrain(wall_pos, TerrainType::Wall);
        map
    }

    #[test]
    fn test_can_see_position_basic() {
        let map = create_test_map(10, 10);
        let observer_pos = Position::new(5, 5);
        let target_pos = Position::new(6, 5);
        let range = 5;

        let can_see = FovMap::can_see_position(observer_pos, target_pos, range, &map);
        assert!(can_see, "Observer should be able to see adjacent position");
    }

    #[test]
    fn test_can_see_position_out_of_range() {
        let map = create_test_map(20, 20);
        let observer_pos = Position::new(5, 5);
        let target_pos = Position::new(15, 15); // Distance > range
        let range = 5;

        let can_see = FovMap::can_see_position(observer_pos, target_pos, range, &map);
        assert!(!can_see, "Observer should not be able to see position beyond range");
    }

    #[test]
    fn test_can_see_position_blocked_by_wall() {
        let wall_pos = Position::new(6, 5);
        let map = create_test_map_with_wall(10, 10, wall_pos);

        let observer_pos = Position::new(5, 5);
        let target_pos = Position::new(7, 5); // Behind the wall
        let range = 5;

        let can_see = FovMap::can_see_position(observer_pos, target_pos, range, &map);
        assert!(!can_see, "Observer should not be able to see position blocked by wall");
    }

    #[test]
    fn test_can_see_entity_wrapper() {
        let map = create_test_map(10, 10);
        let observer_pos = Position::new(3, 3);
        let target_pos = Position::new(4, 3);
        let observer_range = 8;

        let can_see = FovMap::can_see_entity(observer_pos, observer_range, target_pos, &map);
        assert!(can_see, "can_see_entity should work as wrapper around can_see_position");
    }

    #[test]
    fn test_compute_temporary_fov() {
        let map = create_test_map(10, 10);
        let observer_pos = Position::new(5, 5);
        let range = 3;

        let visibility_map = FovMap::compute_temporary_fov(observer_pos, range, &map);

        // Observer position should be visible
        assert!(visibility_map.get_visible((5, 5)), "Observer position should be visible");

        // Adjacent positions should be visible
        assert!(visibility_map.get_visible((6, 5)), "Adjacent position should be visible");
        assert!(visibility_map.get_visible((4, 5)), "Adjacent position should be visible");
        assert!(visibility_map.get_visible((5, 6)), "Adjacent position should be visible");
        assert!(visibility_map.get_visible((5, 4)), "Adjacent position should be visible");

        // Positions beyond range should not be visible
        assert!(!visibility_map.get_visible((10, 10)), "Position beyond range should not be visible");
    }
}
