use bevy::prelude::*;
use brtk::distance::Distance;

/// Resource for configuring distance calculation algorithms used throughout the game
#[derive(Resource, Reflect, Debug, Clone)]
#[reflect(Resource)]
pub struct DistanceSettings {
    /// Distance algorithm used for AI detection and behavior scoring
    /// Default: Manhattan (optimal for grid-based movement)
    pub ai_detection: Distance,

    /// Distance algorithm used for field of view and visibility calculations
    /// Default: Euclidean (realistic vision range)
    pub fov_range: Distance,

    /// Distance algorithm used for pathfinding and movement planning
    /// Default: Chebyshev (accounts for diagonal movement)
    pub pathfinding: Distance,

    /// Distance algorithm used for tactical gameplay mechanics
    /// Default: Diagonal (balanced between Manhattan and Euclidean)
    pub tactical: Distance,
}

impl Default for DistanceSettings {
    fn default() -> Self {
        Self {
            ai_detection: Distance::Manhattan,
            fov_range: Distance::Pythagoras,
            pathfinding: Distance::Chebyshev,
            tactical: Distance::Diagonal,
        }
    }
}

impl DistanceSettings {
    /// Create new distance settings with default algorithms
    pub fn new() -> Self { Self::default() }

    /// Create distance settings optimized for performance (faster algorithms)
    pub fn performance_optimized() -> Self {
        Self {
            ai_detection: Distance::Manhattan,
            fov_range: Distance::Manhattan,
            pathfinding: Distance::Manhattan,
            tactical: Distance::Manhattan,
        }
    }

    /// Create distance settings optimized for accuracy (more realistic algorithms)
    pub fn accuracy_optimized() -> Self {
        Self {
            ai_detection: Distance::Pythagoras,
            fov_range: Distance::Pythagoras,
            pathfinding: Distance::Pythagoras,
            tactical: Distance::Pythagoras,
        }
    }

    /// Create distance settings for classic roguelike feel (grid-based algorithms)
    pub fn classic_roguelike() -> Self {
        Self {
            ai_detection: Distance::Manhattan,
            fov_range: Distance::Chebyshev,
            pathfinding: Distance::Chebyshev,
            tactical: Distance::Manhattan,
        }
    }

    /// Get a human-readable name for a distance algorithm
    pub fn distance_name(distance: &Distance) -> &'static str {
        match distance {
            Distance::Pythagoras => "Pythagoras",
            Distance::PythagorasSquared => "Pythagoras Squared",
            Distance::Manhattan => "Manhattan",
            Distance::Chebyshev => "Chebyshev",
            Distance::Diagonal => "Diagonal",
            Distance::DiagonalWithCosts(_, _) => "Diagonal with Costs",
        }
    }

    /// Get a description of what a distance algorithm is good for
    pub fn distance_description(distance: &Distance) -> &'static str {
        match distance {
            Distance::Pythagoras => "Standard geometric distance with square root",
            Distance::PythagorasSquared => "Fast geometric distance without square root",
            Distance::Manhattan => "Grid-based distance (no diagonals)",
            Distance::Chebyshev => "Maximum of horizontal/vertical distances",
            Distance::Diagonal => "Allows diagonal movement at same cost",
            Distance::DiagonalWithCosts(_, _) => "Diagonal movement with custom costs",
        }
    }

    /// Get all available distance algorithms for UI selection
    pub fn available_distances() -> Vec<Distance> {
        vec![
            Distance::Manhattan,
            Distance::Chebyshev,
            Distance::Diagonal,
            Distance::Pythagoras,
            Distance::PythagorasSquared,
        ]
    }
}
