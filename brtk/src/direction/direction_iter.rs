use crate::direction::{Direction, DIRECTION_TABLE};

pub struct DirectionIter {
    current: usize,
    end: usize,
}

impl DirectionIter {
    /// Returns an iterator over the [`Direction`]s [`NORTH`, `NORTH_EAST`, `EAST`,
    /// `SOUTH_EAST`, `SOUTH`, `SOUTH_WEST`, `WEST`, `NORTH_WEST`]
    pub const fn cardinal_ordinal() -> Self { Self { current: 0, end: 7 } }

    /// Returns an iterator over the [`Direction`]s [`NORTH`, `NORTH_EAST`, `EAST`,
    /// `SOUTH_EAST`, `SOUTH`, `SOUTH_WEST`, `WEST`, `NORTH_WEST`, `UP`, `DOWN`]
    pub const fn cardinal_ordinal_vertical() -> Self { Self { current: 0, end: 9 } }

    /// Returns an iterator over every [`Direction`] including every mutation of [`NORTH`,
    /// `EAST`, `SOUTH`, `WEST`, `UP`, `DOWN`]
    pub const fn all_3d() -> Self { Self { current: 0, end: 25 } }

    /// Returns an iterator over the [`Direction`]s [`NORTH`, `EAST`, `SOUTH`, `WEST`]
    pub const fn cardinal() -> Self { Self { current: 0, end: 3 } }

    /// Returns an iterator over the [`Direction`]s [`NORTH_EAST`, `SOUTH_EAST`, `SOUTH_WEST`,
    /// `NORTH_WEST`]
    pub const fn ordinal() -> Self { Self { current: 4, end: 7 } }

    /// Returns an iterator over the [`Direction`]s (`UP`, `DOWN`)
    pub const fn vertical() -> Self { Self { current: 8, end: 9 } }
}

impl Iterator for DirectionIter {
    type Item = Direction;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.current;
        self.current += 1;
        if next > self.end {
            None
        } else {
            Some(DIRECTION_TABLE[next])
        }
    }
}
