//\! Slope calculations for precise shadowcasting

/// Represents a slope as a rational number Y/X
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Slope {
    pub y: i32,
    pub x: i32,
}

impl Slope {
    #[inline]
    pub const fn new(y: i32, x: i32) -> Self {
        Self { y, x }
    }

    #[inline]
    pub fn value(&self) -> f64 {
        if self.x == 0 {
            if self.y > 0 {
                f64::INFINITY
            } else if self.y < 0 {
                f64::NEG_INFINITY
            } else {
                f64::NAN
            }
        } else {
            self.y as f64 / self.x as f64
        }
    }
}
