use bevy::prelude::*;
use rand::SeedableRng;
use rand_pcg::Pcg64;
use serde::{Deserialize, Serialize};

use crate::random::Dice;

#[derive(Serialize, Deserialize, Reflect, Clone)]
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
}
