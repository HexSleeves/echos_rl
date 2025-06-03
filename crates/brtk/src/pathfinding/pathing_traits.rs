//! Core traits for the pathfinding system
//!
//! These traits provide a flexible interface for implementing pathfinding algorithms
//! that can work with different map representations and movement systems.

/// Trait for objects that can provide walkability and cost information for pathfinding
///
/// This trait abstracts the map or world representation, allowing pathfinding algorithms
/// to work with different underlying data structures.
pub trait PathProvider {
    /// Check if a position is walkable for the given movement type
    ///
    /// # Arguments
    /// * `position` - The (x, y) coordinates to check
    /// * `movement_type` - Type of movement (for different entity types, e.g., flying, walking)
    ///
    /// # Returns
    /// `true` if the position is walkable, `false` otherwise
    fn is_walkable(&mut self, position: (i32, i32), movement_type: u8) -> bool;

    /// Get the movement cost for a position
    ///
    /// # Arguments
    /// * `position` - The (x, y) coordinates to check
    /// * `movement_type` - Type of movement
    ///
    /// # Returns
    /// Movement cost as a u32 (higher values are more expensive)
    fn cost(&mut self, position: (i32, i32), movement_type: u8) -> u32;
}

/// Trait for pathfinding algorithms
///
/// This trait defines the interface that all pathfinding algorithms must implement.
/// It allows for easy swapping between different algorithms while maintaining
/// a consistent interface.
pub trait PathAlgorithm {
    /// Compute a path from origin to destination
    ///
    /// # Arguments
    /// * `origin` - The (x, y) coordinates of the start point
    /// * `destination` - The (x, y) coordinates of the end point
    /// * `movement_type` - Type of movement being used
    /// * `partial_path_on_failure` - Whether to return a partial path if full path fails
    /// * `provider` - Object that provides walkability and cost information
    ///
    /// # Returns
    /// `Some(Vec<(i32, i32)>)` with the path (excluding start point), or `None` if no path found
    fn compute_path<P: PathProvider>(
        origin: (i32, i32),
        destination: (i32, i32),
        movement_type: u8,
        partial_path_on_failure: bool,
        provider: &mut P,
    ) -> Option<Vec<(i32, i32)>>;
}
