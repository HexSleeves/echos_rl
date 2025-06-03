//! Advanced shadowcasting algorithm implementation

use super::{quadrant::Quadrant, row::Row};
use crate::{
    direction::Direction,
    fov::{
        traits::{FovAlgorithm, FovProvider, FovReceiver},
        utils::{distance::DistanceAlgorithm, slope::Slope},
    },
};

/// Advanced shadowcasting algorithm implementation
pub struct Shadowcast;

impl FovAlgorithm for Shadowcast {
    fn compute_fov<P: FovProvider, R: FovReceiver>(
        origin: (i32, i32),
        vision_type: u8,
        range: u32,
        provider: &P,
        receiver: &mut R,
    ) {
        receiver.clear_visible();
        receiver.set_visible(origin);

        // Process all 8 cardinal and diagonal directions
        for direction in Direction::iter_cardinal_ordinal() {
            let mut quadrant = Quadrant::new(direction, origin, vision_type, provider, receiver);
            let mut first_row = Row::new(1, Slope::new(-1, 1), Slope::new(1, 1));
            Self::scan_recursive(range, &mut quadrant, &mut first_row);
        }
    }
}

impl Shadowcast {
    pub fn compute_direction<P: FovProvider, R: FovReceiver>(
        origin: (i32, i32),
        vision_type: u8,
        range: u32,
        provider: &P,
        receiver: &mut R,
        direction: Direction,
    ) {
        receiver.clear_visible();
        receiver.set_visible(origin);

        let mut quadrant = Quadrant::new(direction, origin, vision_type, provider, receiver);
        let mut first_row = Row::new(1, Slope::new(-1, 1), Slope::new(1, 1));
        Self::scan_recursive(range, &mut quadrant, &mut first_row);
    }

    fn scan_recursive<P: FovProvider, R: FovReceiver>(
        range: u32,
        quadrant: &mut Quadrant<P, R>,
        row: &mut Row,
    ) {
        let mut prev_tile = None;
        let distance_alg = DistanceAlgorithm::Euclidean;

        for tile in row.tiles() {
            // Check if tile is within range (using Euclidean distance to avoid double-squaring)
            if !distance_alg.within_range_of_origin(tile, range as f32) {
                continue;
            }

            // Determine if we should reveal this tile
            let should_reveal = quadrant.is_opaque(tile) || row.is_symmetric(tile);

            if should_reveal {
                quadrant.set_visible(tile);
            }

            // Handle shadow casting based on the previous tile
            if let Some(prev_tile) = prev_tile {
                // Transition from opaque to clear: start of new visible area
                if quadrant.is_opaque(prev_tile) && quadrant.is_clear(tile) {
                    row.calc_starting_slope(tile);
                }

                // Transition from clear to opaque: end of visible area, start recursion
                if quadrant.is_clear(prev_tile) && quadrant.is_opaque(tile) {
                    let mut next_row = row.next();
                    next_row.calc_ending_slope(tile);
                    Self::scan_recursive(range, quadrant, &mut next_row);
                }
            }

            prev_tile = Some(tile);
        }

        // If the last tile was clear, continue scanning the next row
        if let Some(prev_tile) = prev_tile {
            if quadrant.is_clear(prev_tile) {
                Self::scan_recursive(range, quadrant, &mut row.next());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fov::traits::{FovProvider, FovReceiver};
    use std::collections::HashSet;

    // Test implementations
    struct TestProvider {
        opaque_positions: HashSet<(i32, i32)>,
    }

    impl TestProvider {
        fn new() -> Self { Self { opaque_positions: HashSet::new() } }

        fn add_opaque(&mut self, position: (i32, i32)) { self.opaque_positions.insert(position); }
    }

    impl FovProvider for TestProvider {
        fn is_opaque(&self, position: (i32, i32), _vision_type: u8) -> bool {
            self.opaque_positions.contains(&position)
        }
    }

    struct TestReceiver {
        visible_positions: HashSet<(i32, i32)>,
    }

    impl TestReceiver {
        fn new() -> Self { Self { visible_positions: HashSet::new() } }

        fn get_visible_count(&self) -> usize { self.visible_positions.len() }

        fn get_max_distance_from_origin(&self) -> f32 {
            self.visible_positions.iter().map(|&(x, y)| ((x * x + y * y) as f32).sqrt()).fold(0.0, f32::max)
        }
    }

    impl FovReceiver for TestReceiver {
        fn set_visible(&mut self, position: (i32, i32)) { self.visible_positions.insert(position); }

        fn get_visible(&self, position: (i32, i32)) -> bool { self.visible_positions.contains(&position) }

        fn clear_visible(&mut self) { self.visible_positions.clear(); }
    }

    #[test]
    fn test_shadowcast_range_calculation() {
        let provider = TestProvider::new();
        let mut receiver = TestReceiver::new();

        // Test with range 5 - should see tiles up to distance 5
        let range = 5;
        Shadowcast::compute_fov((0, 0), 0, range, &provider, &mut receiver);

        // Origin should be visible
        assert!(receiver.get_visible((0, 0)));

        // Tiles at exactly range 5 should be visible
        assert!(receiver.get_visible((5, 0)), "Tile at distance 5 should be visible");
        assert!(receiver.get_visible((0, 5)), "Tile at distance 5 should be visible");
        assert!(receiver.get_visible((3, 4)), "Tile at distance 5 should be visible"); // 3² + 4² = 25, √25 = 5

        // Tiles beyond range should not be visible
        assert!(!receiver.get_visible((6, 0)), "Tile at distance 6 should not be visible");
        assert!(!receiver.get_visible((0, 6)), "Tile at distance 6 should not be visible");

        // Check that the maximum distance is approximately the range
        let max_distance = receiver.get_max_distance_from_origin();
        assert!(
            max_distance <= range as f32 + 0.1,
            "Maximum visible distance {} should not exceed range {}",
            max_distance,
            range
        );
    }

    #[test]
    fn test_shadowcast_basic() {
        let provider = TestProvider::new();
        let mut receiver = TestReceiver::new();

        // Compute FOV from origin with range 3
        Shadowcast::compute_fov((0, 0), 0, 3, &provider, &mut receiver);

        // Origin should be visible
        assert!(receiver.get_visible((0, 0)));

        // Adjacent tiles should be visible
        assert!(receiver.get_visible((1, 0)));
        assert!(receiver.get_visible((0, 1)));
        assert!(receiver.get_visible((-1, 0)));
        assert!(receiver.get_visible((0, -1)));

        // Should have visible tiles
        assert!(receiver.get_visible_count() > 1);
    }

    #[test]
    fn test_shadowcast_with_obstacles() {
        let mut provider = TestProvider::new();
        let mut receiver = TestReceiver::new();

        // Add an obstacle at (1, 0)
        provider.add_opaque((1, 0));

        // Compute FOV from origin with range 3
        Shadowcast::compute_fov((0, 0), 0, 3, &mut provider, &mut receiver);

        // Origin should be visible
        assert!(receiver.get_visible((0, 0)));

        // Obstacle should be visible (walls are visible)
        assert!(receiver.get_visible((1, 0)));

        // Tiles behind the obstacle should be shadowed
        assert!(!receiver.get_visible((2, 0)), "Tile behind obstacle should be shadowed");
    }
}
