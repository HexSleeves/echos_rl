use std::{fmt::Display, str::FromStr, sync::LazyLock};

use bevy::prelude::Reflect;
use regex::Captures;

use crate::random::DiceError;

// https://regex101.com/
static DICE_REGEX: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(
        r"(?xi)
^
(?P<count> [1-9]\d*)?
d
(?P<sides> [1-9]\d*)
\+?
(?P<modifier> [1-9]\d*)?
$",
    )
    .expect("Couldn't compile regex")
});

#[derive(Debug, Reflect, Clone, Copy, PartialEq, Eq)]
pub struct Dice {
    pub count: i32,
    pub sides: i32,
    pub modifier: i32,
}

impl Dice {
    pub fn new(count: i32, sides: i32, modifier: i32) -> Self { Self { count, sides, modifier } }

    pub fn roll(&self, random: &mut impl rand::Rng) -> i32 {
        let roll = random.random_range(self.count..=self.count * self.sides);
        roll + self.modifier
    }

    pub fn min(&self) -> i32 { self.count + self.modifier }

    pub fn max(&self) -> i32 { self.count * self.sides + self.modifier }
}

impl Display for Dice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.modifier == 0 {
            write!(f, "{}d{}", self.count, self.sides)
        } else {
            write!(f, "{}d{}+{}", self.count, self.sides, self.modifier)
        }
    }
}

impl FromStr for Dice {
    type Err = DiceError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut caps = DICE_REGEX.captures_iter(s);
        let word = caps.next().ok_or(DiceError::Unparseable)?;
        Dice::try_from(word)
    }
}

impl TryFrom<Captures<'_>> for Dice {
    type Error = DiceError;

    fn try_from(value: Captures<'_>) -> Result<Self, Self::Error> {
        let count = value
            .name("count")
            .map(|m| m.as_str())
            .unwrap_or("1")
            .parse::<i32>()
            .map_err(|_| DiceError::ParseCount)?;

        let sides = value
            .name("sides")
            .map(|m| m.as_str())
            .ok_or(DiceError::MissingSides)?
            .parse::<i32>()
            .map_err(|_| DiceError::ParseSides)?;

        let modifier = value
            .name("modifier")
            .map(|m| m.as_str())
            .unwrap_or("0")
            .parse::<i32>()
            .map_err(|_| DiceError::ParseModifier)?;

        Ok(Self { count, sides, modifier })
    }
}
