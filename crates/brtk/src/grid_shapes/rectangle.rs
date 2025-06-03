use bevy::prelude::Reflect;
use serde::{Deserialize, Serialize};

use crate::grid_shapes::iter::RectIter;

/// A 2D rectangle.
#[derive(Serialize, Deserialize, Reflect, Debug, Clone, Copy, Eq, PartialEq)]
pub struct Rectangle {
    /// The minimum coordinates of the rectangle.
    pub min: (i32, i32),
    /// The maximum coordinates of the rectangle.
    pub max: (i32, i32),
}

impl Default for Rectangle {
    fn default() -> Self {
        Self::new_with_size((0, 0), (0, 0))
    }
}

impl Rectangle {
    /// Creates a new rectangle.
    #[inline]
    pub fn new(min: (i32, i32), max: (i32, i32)) -> Self {
        Self {
            min: min.min(max),
            max: min.max(max),
        }
    }

    /// Creates a new rectangle with the given size.
    #[inline]
    pub fn new_with_size(min: (i32, i32), dimensions: (u32, u32)) -> Self {
        let max = (min.0 + dimensions.0 as i32, min.1 + dimensions.1 as i32);
        Self::new(min, max)
    }
}

impl Rectangle {
    /// Get the width of the rectangle.
    #[inline]
    pub const fn width(&self) -> i32 {
        self.max.0 - self.min.0
    }

    /// Get the height of the rectangle.
    #[inline]
    pub const fn height(&self) -> i32 {
        self.max.1 - self.min.1
    }

    /// Get the minimum point of the rectangle.
    #[inline]
    pub const fn min(&self) -> (i32, i32) {
        self.min
    }

    /// Get the maximum point of the rectangle.
    #[inline]
    pub const fn max(&self) -> (i32, i32) {
        self.max
    }

    /// Check if the rectangle is square.
    #[inline]
    pub fn is_square(&self) -> bool {
        let diff = (self.max.0 - self.min.0, self.max.1 - self.min.1);
        diff.0 == diff.1
    }
}

impl Rectangle {
    #[inline]
    pub const fn center(&self) -> (i32, i32) {
        ((self.min.0 + self.max.0) / 2, (self.min.1 + self.max.1) / 2)
    }

    #[inline]
    pub fn left(&self) -> i32 {
        self.min.0.min(self.max.0)
    }

    #[inline]
    pub fn right(&self) -> i32 {
        self.min.0.max(self.max.0)
    }

    #[inline]
    pub fn top(&self) -> i32 {
        self.min.1.max(self.max.1)
    }

    #[inline]
    pub fn bottom(&self) -> i32 {
        self.min.1.min(self.max.1)
    }

    /// Check if this rectangle intersects another rectangle.
    #[inline]
    pub const fn intersects(&self, other: Self) -> bool {
        // (self.min.cmple(other.max) & self.max.cmpge(other.min)).all()
        self.min.0 <= other.max.0
            && self.max.0 >= other.min.0
            && self.min.1 <= other.max.1
            && self.max.1 >= other.min.1
    }

    /// Calls a function for each x/y point in the rectangle
    pub fn for_each<F>(&self, f: F)
    where
        F: FnMut((i32, i32)),
    {
        RectIter::new(self.min, self.max).for_each(f);
    }
}

impl IntoIterator for Rectangle {
    type IntoIter = RectIter;
    type Item = (i32, i32);

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        RectIter::new(self.min, self.max)
    }
}
