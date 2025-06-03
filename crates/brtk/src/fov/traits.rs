//! Core traits for the FOV system
//!
//! These traits provide a flexible interface for implementing FOV algorithms
//! that can work with different map representations and visibility storage systems.

/// Trait for objects that can provide opacity information for FOV calculations
///
/// This trait abstracts the map or world representation, allowing FOV algorithms
/// to work with different underlying data structures.
pub trait FovProvider {
    /// Check if a position blocks vision
    ///
    /// # Arguments
    /// * `position` - The (x, y) coordinates to check
    /// * `vision_type` - Type of vision (for future extensibility, e.g., infrared, magical)
    ///
    /// # Returns
    /// `true` if the position blocks vision, `false` otherwise
    fn is_opaque(&self, position: (i32, i32), vision_type: u8) -> bool;
}

/// Trait for objects that can receive and store visibility information
///
/// This trait abstracts the visibility storage system, allowing different
/// implementations for storing which tiles are visible.
pub trait FovReceiver {
    /// Mark a position as visible
    ///
    /// # Arguments
    /// * `position` - The (x, y) coordinates to mark as visible
    fn set_visible(&mut self, position: (i32, i32));

    /// Check if a position is currently visible
    ///
    /// # Arguments
    /// * `position` - The (x, y) coordinates to check
    ///
    /// # Returns
    /// `true` if the position is visible, `false` otherwise
    fn get_visible(&self, position: (i32, i32)) -> bool;

    /// Clear all visibility information
    ///
    /// This is typically called at the start of each FOV computation
    fn clear_visible(&mut self);
}

/// Trait for FOV algorithms
///
/// This trait defines the interface that all FOV algorithms must implement.
/// It allows for easy swapping between different algorithms while maintaining
/// a consistent interface.
pub trait FovAlgorithm {
    /// Compute field of view from an origin point
    ///
    /// # Arguments
    /// * `origin` - The (x, y) coordinates of the observer
    /// * `vision_type` - Type of vision being used
    /// * `range` - Maximum range of vision
    /// * `provider` - Object that provides opacity information
    /// * `receiver` - Object that receives visibility information
    fn compute_fov<P: FovProvider, R: FovReceiver>(
        origin: (i32, i32),
        vision_type: u8,
        range: u32,
        provider: &P,
        receiver: &mut R,
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    // Test implementations for the traits
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
    }

    impl FovReceiver for TestReceiver {
        fn set_visible(&mut self, position: (i32, i32)) { self.visible_positions.insert(position); }

        fn get_visible(&self, position: (i32, i32)) -> bool { self.visible_positions.contains(&position) }

        fn clear_visible(&mut self) { self.visible_positions.clear(); }
    }

    #[test]
    fn test_provider_trait() {
        let mut provider = TestProvider::new();
        assert!(!provider.is_opaque((0, 0), 0));

        provider.add_opaque((1, 1));
        assert!(provider.is_opaque((1, 1), 0));
        assert!(!provider.is_opaque((0, 0), 0));
    }

    #[test]
    fn test_receiver_trait() {
        let mut receiver = TestReceiver::new();
        assert!(!receiver.get_visible((0, 0)));

        receiver.set_visible((0, 0));
        assert!(receiver.get_visible((0, 0)));

        receiver.clear_visible();
        assert!(!receiver.get_visible((0, 0)));
    }
}
