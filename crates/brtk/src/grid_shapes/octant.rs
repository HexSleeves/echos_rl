/// An octant
#[derive(Debug, Clone)]
pub struct Octant(pub u8);

impl Octant {
    /// adapted from <http://codereview.stackexchange.com/a/95551>
    /// converts a position into a coordinate relative `Octant(0)` offset
    #[inline]
    pub fn to_offset(&self, position: (i32, i32)) -> (i32, i32) {
        match self.0 {
            0 => (position.0, position.1),
            1 => (position.1, position.0),
            2 => (position.1, -position.0),
            3 => (-position.0, position.1),
            4 => (-position.0, -position.1),
            5 => (-position.1, -position.0),
            6 => (-position.1, position.0),
            7 => (position.0, -position.1),
            _ => unreachable!(),
        }
    }

    /// converts from a `Octant(0)` relative coordinate into a `(i32, i32)`
    #[inline]
    #[allow(clippy::wrong_self_convention)]
    pub fn from_offset(&self, offset: (i32, i32)) -> (i32, i32) {
        let p = match self.0 {
            0 => (offset.0, offset.1),
            1 => (offset.1, offset.0),
            2 => (-offset.1, offset.0),
            3 => (-offset.0, offset.1),
            4 => (-offset.0, -offset.1),
            5 => (-offset.1, -offset.0),
            6 => (offset.1, -offset.0),
            7 => (offset.0, -offset.1),
            _ => unreachable!(),
        };
        (p.0, p.1)
    }

    /// Creates a new `Octant` from two points
    #[inline(always)]
    pub const fn new(position: (i32, i32), other: (i32, i32)) -> Self {
        // adapted from <http://codereview.stackexchange.com/a/95551>
        let mut dx = other.0 - position.0;
        let mut dy = other.1 - position.1;
        let mut octant = 0;
        if dy < 0 {
            dx = -dx;
            dy = -dy;
            octant += 4;
        }
        if dx < 0 {
            let tmp = dx;
            dx = dy;
            dy = -tmp;
            octant += 2;
        }
        if dx < dy {
            octant += 1;
        }

        Self(octant)
    }
}
