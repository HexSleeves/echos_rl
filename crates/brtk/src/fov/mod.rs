//! Field of View (FOV) system for roguelike games
//!
//! This module provides a trait-based FOV system that supports multiple algorithms
//! and can be adapted to different game architectures. The design is inspired by
//! modern shadowcasting techniques and provides both flexibility and performance.

pub mod algorithms;
pub mod implementations;
pub mod traits;
pub mod utils;

// Re-export main types for convenience
pub use algorithms::shadowcast::Shadowcast;
pub use implementations::{map_provider::MapProvider, visibility_map::VisibilityMap};
pub use traits::{FovAlgorithm, FovProvider, FovReceiver};
pub use utils::slope::Slope;

/// Main FOV algorithm selector
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FovAlgorithmType {
    /// Advanced shadowcasting algorithm with precise slope calculations
    Shadowcast,
    /// Directional shadowcasting for cone-based vision
    ShadowcastDirection(crate::direction::Direction),
}

impl Default for FovAlgorithmType {
    fn default() -> Self { Self::Shadowcast }
}

impl FovAlgorithmType {
    /// Compute FOV using the selected algorithm
    pub fn compute<P: FovProvider, R: FovReceiver>(
        &self,
        origin: (i32, i32),
        vision_type: u8,
        range: u32,
        provider: &mut P,
        receiver: &mut R,
    ) {
        match self {
            Self::Shadowcast => {
                Shadowcast::compute_fov(origin, vision_type, range, provider, receiver);
            }
            Self::ShadowcastDirection(direction) => {
                Shadowcast::compute_direction(origin, vision_type, range, provider, receiver, *direction);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::direction::Direction;

    #[test]
    fn test_fov_algorithm_type_default() {
        assert_eq!(FovAlgorithmType::default(), FovAlgorithmType::Shadowcast);
    }

    #[test]
    fn test_fov_algorithm_type_variants() {
        let shadowcast = FovAlgorithmType::Shadowcast;
        let directional = FovAlgorithmType::ShadowcastDirection(Direction::NORTH);

        assert_ne!(shadowcast, directional);
    }
}
