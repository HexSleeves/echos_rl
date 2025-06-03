use std::collections::HashSet;

use bevy::prelude::Reflect;
use serde::{Deserialize, Serialize};

use crate::grid_shapes::{BoxedShape, BoxedShapeIter, Line, Shape, ShapeIter};

/// A Grid based circle
#[derive(Serialize, Deserialize, Reflect, Debug, Clone, Copy, Eq, PartialEq)]
pub struct Circle {
    center: (i32, i32),
    radius: u32,
}

impl Circle {
    /// Creates a new circle.
    pub fn new<R: Into<u32>>(center: (i32, i32), radius: R) -> Self {
        Self {
            center,
            radius: radius.into(),
        }
    }
}

impl Circle {
    /// Get the center of the circle
    #[inline]
    pub const fn center(&self) -> (i32, i32) {
        self.center
    }

    /// Get the left point of the circle
    #[inline]
    pub fn left(&self) -> (i32, i32) {
        (self.center.0 - self.radius as i32, self.center.1)
    }

    /// Get the right point of the circle
    #[inline]
    pub fn right(&self) -> (i32, i32) {
        (self.center.0 + self.radius as i32, self.center.1)
    }

    /// Get the top point of the circle
    #[inline]
    pub fn top(&self) -> (i32, i32) {
        (self.center.0, self.center.1 + self.radius as i32)
    }

    /// Get the bottom point of the circle
    #[inline]
    pub fn bottom(&self) -> (i32, i32) {
        (self.center.0, self.center.1 - self.radius as i32)
    }

    /// Return a line from the left to the right of the circle
    #[inline]
    pub fn as_horizontal_line(&self) -> Line {
        Line::new(self.left(), self.right())
    }

    /// Return a line from the top to the bottom of the circle
    #[inline]
    pub fn as_vertical_line(&self) -> Line {
        Line::new(self.bottom(), self.top())
    }

    /// Get the circumfrence of the circle
    pub fn get_circumfrence(&self) -> HashSet<(i32, i32)> {
        let mut discovered = HashSet::new();
        let mut d = (5 - (self.radius as i32 * 4)) / 4;
        let mut x = 0;
        let mut y = self.radius as i32;

        loop {
            // Circumfrence
            discovered.insert((self.center.0 + x, self.center.1 + y));
            discovered.insert((self.center.0 + x, self.center.1 - y));
            discovered.insert((self.center.0 - x, self.center.1 + y));
            discovered.insert((self.center.0 - x, self.center.1 - y));
            discovered.insert((self.center.0 + y, self.center.1 + x));
            discovered.insert((self.center.0 + y, self.center.1 - x));
            discovered.insert((self.center.0 - y, self.center.1 + x));
            discovered.insert((self.center.0 - y, self.center.1 - x));

            if d < 0 {
                d += (2 * x) + 1;
            } else {
                d += (2 * (x - y)) + 1;
                y -= 1;
            }
            x += 1;

            if x > y {
                break;
            }
        }
        discovered
    }
}

impl Shape for Circle {
    #[inline]
    // FIX: PERF??
    fn get_count(&self) -> u32 {
        self.get_positions().len() as u32
    }

    #[inline]
    fn contains(&self, position: (i32, i32)) -> bool {
        self.get_positions().contains(&position)
    }

    // TODO: Move to iter()
    #[inline]
    fn get_positions(&self) -> HashSet<(i32, i32)> {
        let mut discovered = HashSet::new();
        let mut d = (5 - (self.radius as i32 * 4)) / 4;
        let mut x = 0;
        let mut y = self.radius as i32;

        let mut start;
        let mut end;
        let mut line;
        loop {
            start = (self.center.0 + x, self.center.1 + y);
            end = (self.center.0 + x, self.center.1 - y);
            line = Line::new(start, end);
            for point in line.get_positions() {
                discovered.insert(point);
            }

            start = (self.center.0 - x, self.center.1 + y);
            end = (self.center.0 - x, self.center.1 - y);
            line = Line::new(start, end);
            for point in line.get_positions() {
                discovered.insert(point);
            }

            start = (self.center.0 + y, self.center.1 + x);
            end = (self.center.0 + y, self.center.1 - x);
            line = Line::new(start, end);
            for point in line.get_positions() {
                discovered.insert(point);
            }

            start = (self.center.0 - y, self.center.1 + x);
            end = (self.center.0 - y, self.center.1 - x);
            line = Line::new(start, end);
            for point in line.get_positions() {
                discovered.insert(point);
            }

            //// Circumfrence
            // discovered.insert((self.center.0 + x, self.center.1 + y));
            // discovered.insert((self.center.0 + x, self.center.1 - y));
            // discovered.insert((self.center.0 - x, self.center.1 + y));
            // discovered.insert((self.center.0 - x, self.center.1 - y));
            // discovered.insert((self.center.0 + y, self.center.1 + x));
            // discovered.insert((self.center.0 + y, self.center.1 - x));
            // discovered.insert((self.center.0 - y, self.center.1 + x));
            // discovered.insert((self.center.0 - y, self.center.1 - x));

            if d < 0 {
                d += (2 * x) + 1;
            } else {
                d += (2 * (x - y)) + 1;
                y -= 1;
            }
            x += 1;

            if x > y {
                break;
            }
        }
        discovered
    }

    #[inline]
    fn boxed_iter(&self) -> BoxedShapeIter {
        Box::new(self.into_iter())
    }
}

impl ShapeIter for Circle {
    type Iterator = std::collections::hash_set::IntoIter<(i32, i32)>;

    #[inline]
    fn iter(&self) -> Self::Iterator {
        self.get_positions().into_iter()
    }
}

impl IntoIterator for Circle {
    type IntoIter = std::collections::hash_set::IntoIter<(i32, i32)>;
    type Item = (i32, i32);

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.get_positions().into_iter()
    }
}

impl From<Circle> for BoxedShape {
    fn from(value: Circle) -> Self {
        Box::new(value)
    }
}
