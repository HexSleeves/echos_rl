use crate::grid_shapes::Rectangle;

/// An iterator over a rectangle.
#[derive(Debug, Clone)]
pub struct RectIter {
    offset: (i32, i32),
    max_offset: (i32, i32),

    /// The minimum corner point of the rect.
    min: (i32, i32),
}

impl RectIter {
    /// Creates a new rectangle iterator.
    pub fn new(min: (i32, i32), max: (i32, i32)) -> Self {
        let size = (max.0 - min.0, max.1 - min.1);
        Self {
            min,
            max_offset: size,
            offset: (0, 0),
        }
    }
}

impl Iterator for RectIter {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.offset.1 > self.max_offset.1 {
            return None;
        }
        let p = self.offset;
        self.offset.0 += 1;
        if self.offset.0 > self.max_offset.0 {
            self.offset.0 = 0;
            self.offset.1 += 1;
        }
        Some((self.min.0 + p.0, self.min.1 + p.1))
    }
}

impl From<Rectangle> for RectIter {
    fn from(rect: Rectangle) -> Self {
        rect.into_iter()
    }
}
