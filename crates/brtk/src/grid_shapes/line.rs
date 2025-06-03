use std::{collections::HashSet, fmt::Display};

use bevy::prelude::Reflect;
use serde::{Deserialize, Serialize};

use crate::grid_shapes::{
    iter::{BresenhamLineInclusiveIter, BresenhamLineIter},
    BoxedShape, BoxedShapeIter, Shape, ShapeIter,
};

/// A line.
#[derive(Serialize, Deserialize, Reflect, Debug, Clone, Copy, Eq, PartialEq)]
pub struct Line {
    end: (i32, i32),
    start: (i32, i32),
}

impl Line {
    /// Creates a new line.
    #[inline(always)]
    pub const fn new(start: (i32, i32), end: (i32, i32)) -> Self {
        Self { start, end }
    }

    #[allow(dead_code)]
    #[inline]
    fn iter_exlusive(&self) -> BresenhamLineIter {
        BresenhamLineIter::new(self.start, self.end)
    }
}

impl Shape for Line {
    #[inline]
    fn get_count(&self) -> u32 {
        (self.end.0 - self.start.0)
            .abs()
            .max((self.end.1 - self.start.1).abs()) as u32
    }

    #[inline]
    fn contains(&self, position: (i32, i32)) -> bool {
        self.get_positions().contains(&position)
    }

    #[inline]
    fn get_positions(&self) -> HashSet<(i32, i32)> {
        self.iter().collect()
    }

    #[inline]
    fn boxed_iter(&self) -> BoxedShapeIter {
        Box::new(self.into_iter())
    }
}

impl ShapeIter for Line {
    type Iterator = BresenhamLineInclusiveIter;

    #[inline]
    fn iter(&self) -> Self::Iterator {
        self.into_iter()
    }
}

impl IntoIterator for Line {
    type IntoIter = BresenhamLineInclusiveIter;
    type Item = (i32, i32);

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        BresenhamLineInclusiveIter::new(self.start, self.end)
    }
}

impl Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Line {{Start: ({}, {}), End: ({}, {})}}",
            self.start.0, self.start.1, self.end.0, self.end.1
        )
    }
}

impl From<Line> for BoxedShape {
    fn from(value: Line) -> Self {
        Box::new(value)
    }
}
