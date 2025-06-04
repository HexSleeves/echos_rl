use std::ops::{Add, AddAssign};

use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::TilePos;
use brtk::distance::Distance;

use crate::core::resources::DistanceSettings;

/// Position component for entities in the game world
#[derive(Component, Reflect, Default, Debug, Clone, Copy, PartialEq, Eq, Hash, Deref, DerefMut)]
#[reflect(Component)]
pub struct Position(pub IVec2);

impl Position {
    pub fn new(x: i32, y: i32) -> Self { Self(IVec2::new(x, y)) }
    pub fn x(&self) -> i32 { self.0.x }
    pub fn y(&self) -> i32 { self.0.y }

    /// Convert Position to (f32, f32) tuple for distance calculations
    pub fn as_f32_tuple(&self) -> (f32, f32) { (self.0.x as f32, self.0.y as f32) }

    /// Legacy distance method using Bevy's built-in calculation (preserved for compatibility)
    pub fn distance(&self, other: &Position) -> f32 { self.0.as_vec2().distance(other.0.as_vec2()) }

    pub fn distance_squared(&self, other: &Position) -> i32 { self.0.distance_squared(other.0) }

    /// Distance calculation optimized for AI detection and behavior scoring
    /// Uses Manhattan distance for grid-based movement patterns
    pub fn ai_detection_distance(&self, other: &Position) -> f32 {
        Distance::Manhattan.calculate(self.as_f32_tuple(), other.as_f32_tuple())
    }

    /// Distance calculation for field of view and visibility checks
    /// Uses Euclidean distance for realistic vision range calculations
    pub fn fov_range_distance(&self, other: &Position) -> f32 {
        Distance::Euclidean.calculate(self.as_f32_tuple(), other.as_f32_tuple())
    }

    /// Distance calculation for pathfinding and movement planning
    /// Uses Chebyshev distance to account for diagonal movement
    pub fn pathfinding_distance(&self, other: &Position) -> f32 {
        Distance::Chebyshev.calculate(self.as_f32_tuple(), other.as_f32_tuple())
    }

    /// Configurable distance calculation for tactical gameplay mechanics
    /// Uses Diagonal distance as a balanced option between Manhattan and Euclidean
    pub fn tactical_distance(&self, other: &Position) -> f32 {
        Distance::Diagonal.calculate(self.as_f32_tuple(), other.as_f32_tuple())
    }

    /// High-performance distance calculation without square root
    /// Uses PythagorasSquared for when you only need relative distances
    pub fn fast_distance_squared(&self, other: &Position) -> f32 {
        Distance::PythagorasSquared.calculate(self.as_f32_tuple(), other.as_f32_tuple())
    }

    /// Configurable AI detection distance using DistanceSettings
    pub fn ai_detection_distance_configured(&self, other: &Position, settings: &DistanceSettings) -> f32 {
        settings.ai_detection.calculate(self.as_f32_tuple(), other.as_f32_tuple())
    }

    /// Configurable FOV range distance using DistanceSettings
    pub fn fov_range_distance_configured(&self, other: &Position, settings: &DistanceSettings) -> f32 {
        settings.fov_range.calculate(self.as_f32_tuple(), other.as_f32_tuple())
    }

    /// Configurable pathfinding distance using DistanceSettings
    pub fn pathfinding_distance_configured(&self, other: &Position, settings: &DistanceSettings) -> f32 {
        settings.pathfinding.calculate(self.as_f32_tuple(), other.as_f32_tuple())
    }

    /// Configurable tactical distance using DistanceSettings
    pub fn tactical_distance_configured(&self, other: &Position, settings: &DistanceSettings) -> f32 {
        settings.tactical.calculate(self.as_f32_tuple(), other.as_f32_tuple())
    }

    /// Generic distance calculation with custom algorithm
    pub fn distance_with_algorithm(&self, other: &Position, algorithm: Distance) -> f32 {
        algorithm.calculate(self.as_f32_tuple(), other.as_f32_tuple())
    }
}

impl From<IVec2> for Position {
    fn from(vec: IVec2) -> Self { Self(vec) }
}

impl From<Position> for (i32, i32) {
    fn from(value: Position) -> Self { (value.0.x, value.0.y) }
}

impl From<TilePos> for Position {
    fn from(tile_pos: TilePos) -> Self { Self(IVec2::new(tile_pos.x as i32, tile_pos.y as i32)) }
}

impl From<(i32, i32)> for Position {
    fn from((x, y): (i32, i32)) -> Self { Self(IVec2::new(x, y)) }
}

impl Add<Position> for Position {
    type Output = Self;

    fn add(self, rhs: Position) -> Self::Output { Self(self.0 + rhs.0) }
}

impl AddAssign<Position> for Position {
    fn add_assign(&mut self, rhs: Position) { self.0 += rhs.0; }
}

impl Add<IVec2> for Position {
    type Output = Self;

    fn add(self, rhs: IVec2) -> Self::Output { Self(self.0 + rhs) }
}

impl AddAssign<IVec2> for Position {
    fn add_assign(&mut self, rhs: IVec2) { self.0 += rhs; }
}

impl Add<(i32, i32)> for Position {
    type Output = Self;

    fn add(self, rhs: (i32, i32)) -> Self::Output { Self(IVec2::new(self.0.x + rhs.0, self.0.y + rhs.1)) }
}

impl AddAssign<(i32, i32)> for Position {
    fn add_assign(&mut self, rhs: (i32, i32)) {
        self.0.x += rhs.0;
        self.0.y += rhs.1;
    }
}

/// Description component for entities
#[derive(Component, Reflect, Default, Debug, Clone, Deref, DerefMut)]
#[reflect(Component)]
pub struct Description(pub String);

impl Description {
    pub fn new(description: impl ToString) -> Self { Self(description.to_string()) }
}

#[derive(Reflect, Component, Deref, DerefMut)]
#[reflect(Component)]
pub struct FieldOfView(pub u8);

impl Default for FieldOfView {
    fn default() -> Self { Self(4) }
}

impl FieldOfView {
    pub fn new(radius: u8) -> Self { Self(radius) }
}

/// Component that marks an entity as the player
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
#[require(Description, Position)]
pub struct PlayerTag;

/// Component that marks an entity as AI-controlled
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct AITag;

/// Component that marks an entity as dead
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct DeadTag;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Mob;

#[cfg(test)]
mod tests {
    use super::*;
    use brtk::distance::Distance;

    #[test]
    fn test_position_creation() {
        let pos = Position::new(5, 10);
        assert_eq!(pos.x(), 5);
        assert_eq!(pos.y(), 10);
    }

    #[test]
    fn test_as_f32_tuple() {
        let pos = Position::new(3, 7);
        assert_eq!(pos.as_f32_tuple(), (3.0, 7.0));
    }

    #[test]
    fn test_distance_methods_consistency() {
        let pos1 = Position::new(0, 0);
        let pos2 = Position::new(3, 4);

        // Test that all distance methods return reasonable values
        let ai_distance = pos1.ai_detection_distance(&pos2);
        let fov_distance = pos1.fov_range_distance(&pos2);
        let pathfinding_distance = pos1.pathfinding_distance(&pos2);
        let tactical_distance = pos1.tactical_distance(&pos2);

        // All distances should be positive
        assert!(ai_distance > 0.0);
        assert!(fov_distance > 0.0);
        assert!(pathfinding_distance > 0.0);
        assert!(tactical_distance > 0.0);

        // Manhattan distance should be 7 (3 + 4)
        assert_eq!(ai_distance, 7.0);

        // Euclidean distance should be 5 (sqrt(3^2 + 4^2))
        assert_eq!(fov_distance, 5.0);

        // Chebyshev distance should be 4 (max(3, 4))
        assert_eq!(pathfinding_distance, 4.0);
    }

    #[test]
    fn test_configurable_distance_methods() {
        let pos1 = Position::new(0, 0);
        let pos2 = Position::new(6, 8);
        let settings = DistanceSettings::new();

        let ai_distance = pos1.ai_detection_distance_configured(&pos2, &settings);
        let fov_distance = pos1.fov_range_distance_configured(&pos2, &settings);
        let pathfinding_distance = pos1.pathfinding_distance_configured(&pos2, &settings);
        let tactical_distance = pos1.tactical_distance_configured(&pos2, &settings);

        // Should match the default algorithms
        assert_eq!(ai_distance, pos1.ai_detection_distance(&pos2));
        assert_eq!(fov_distance, pos1.fov_range_distance(&pos2));
        assert_eq!(pathfinding_distance, pos1.pathfinding_distance(&pos2));
        assert_eq!(tactical_distance, pos1.tactical_distance(&pos2));
    }

    #[test]
    fn test_distance_with_algorithm() {
        let pos1 = Position::new(0, 0);
        let pos2 = Position::new(3, 4);

        // Test different algorithms
        let manhattan = pos1.distance_with_algorithm(&pos2, Distance::Manhattan);
        let euclidean = pos1.distance_with_algorithm(&pos2, Distance::Euclidean);
        let chebyshev = pos1.distance_with_algorithm(&pos2, Distance::Chebyshev);

        assert_eq!(manhattan, 7.0);
        assert_eq!(euclidean, 5.0);
        assert_eq!(chebyshev, 4.0);
    }

    #[test]
    fn test_backward_compatibility() {
        let pos1 = Position::new(0, 0);
        let pos2 = Position::new(3, 4);

        // Legacy distance method should still work
        let legacy_distance = pos1.distance(&pos2);
        assert_eq!(legacy_distance, 5.0); // Euclidean distance

        // Distance squared should still work
        let distance_squared = pos1.distance_squared(&pos2);
        assert_eq!(distance_squared, 25); // 3^2 + 4^2 = 25
    }

    #[test]
    fn test_same_position_distances() {
        let pos = Position::new(5, 5);

        // All distance calculations should return 0 for same position
        assert_eq!(pos.ai_detection_distance(&pos), 0.0);
        assert_eq!(pos.fov_range_distance(&pos), 0.0);
        assert_eq!(pos.pathfinding_distance(&pos), 0.0);
        assert_eq!(pos.tactical_distance(&pos), 0.0);
        assert_eq!(pos.distance(&pos), 0.0);
        assert_eq!(pos.distance_squared(&pos), 0);
    }

    #[test]
    fn test_performance_distance_methods() {
        let pos1 = Position::new(0, 0);
        let pos2 = Position::new(5, 12);

        // Fast distance squared should not use square root
        let fast_squared = pos1.fast_distance_squared(&pos2);
        assert_eq!(fast_squared, 169.0); // 5^2 + 12^2 = 169

        // Should be faster than regular distance (no sqrt)
        let regular_distance = pos1.fov_range_distance(&pos2);
        assert_eq!(regular_distance, 13.0); // sqrt(169) = 13
    }
}
