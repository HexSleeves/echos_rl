pub struct PointIterRowMajor {
    point: (i32, i32),
    size: (u32, u32),
}

impl PointIterRowMajor {
    pub const fn new(size: (u32, u32)) -> Self {
        Self {
            point: (0, 0),
            size,
        }
    }
}

impl Iterator for PointIterRowMajor {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.point.1 == self.size.0 as i32 {
            return None;
        }
        let point = self.point;
        self.point.0 += 1;
        if self.point.0 == self.size.1 as i32 {
            self.point.0 = 0;
            self.point.1 += 1;
        }
        Some(point)
    }
}
