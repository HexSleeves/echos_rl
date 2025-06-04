use bevy::prelude::*;
use rand::SeedableRng;
use rand_pcg::Pcg64;
use serde::{Deserialize, Serialize};

use crate::random::Dice;

#[derive(Resource, Serialize, Deserialize, Reflect, Clone)]
pub struct Random {
    #[reflect(ignore, default = "default_pcg")]
    pub random: Pcg64,
}

fn default_pcg() -> Pcg64 { Pcg64::from_os_rng() }

impl Default for Random {
    fn default() -> Self { Self { random: Pcg64::from_os_rng() } }
}

impl Random {
    pub fn new(seed: u64) -> Self { Self { random: Pcg64::seed_from_u64(seed) } }

    pub fn roll(&mut self, dice: Dice) -> i32 { dice.roll(&mut self.random) }

    /// Generate a random u32 value
    pub fn u32(&mut self, range: std::ops::Range<u32>) -> u32 {
        use rand::Rng;
        self.random.random_range(range)
    }

    /// Generate a random usize value
    pub fn usize(&mut self, range: std::ops::Range<usize>) -> usize {
        use rand::Rng;
        self.random.random_range(range)
    }

    /// Generate a random i32 value
    pub fn i32(&mut self, range: std::ops::RangeInclusive<i32>) -> i32 {
        use rand::Rng;
        self.random.random_range(range)
    }

    /// Generate a random boolean
    pub fn bool(&mut self) -> bool {
        use rand::Rng;
        self.random.random_bool(0.5)
    }

    /// Create a fastrand::Rng from the current state
    /// This allows compatibility with existing code that expects fastrand::Rng
    pub fn to_fastrand(&mut self) -> fastrand::Rng {
        use rand::Rng;
        // Generate a seed from the current RNG state
        let seed = self.random.random::<u64>();
        fastrand::Rng::with_seed(seed)
    }

    /// Returns a mutable reference to the underlying RNG core.
    ///
    /// Use this when you need direct access to the RNG for compatibility
    /// with external libraries that expect a specific RNG type.
    /// Prefer the structured methods (roll, u32, i32, etc.) when possible.
    pub fn as_rng_core(&mut self) -> &mut Pcg64 { &mut self.random }
}
