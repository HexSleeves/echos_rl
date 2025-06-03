//\! Distance calculation algorithms for FOV systems

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DistanceAlgorithm {
    Euclidean,
    EuclideanSquared,
    Manhattan,
    Chebyshev,
}

impl DistanceAlgorithm {
    pub fn distance_2d(&self, from: (i32, i32), to: (i32, i32)) -> f32 {
        let dx = (to.0 - from.0) as f32;
        let dy = (to.1 - from.1) as f32;
        
        match self {
            Self::Euclidean => (dx * dx + dy * dy).sqrt(),
            Self::EuclideanSquared => dx * dx + dy * dy,
            Self::Manhattan => dx.abs() + dy.abs(),
            Self::Chebyshev => dx.abs().max(dy.abs()),
        }
    }

    pub fn within_range_of_origin(&self, point: (i32, i32), range: f32) -> bool {
        match self {
            Self::EuclideanSquared => {
                self.distance_2d((0, 0), point) <= range * range
            }
            _ => self.distance_2d((0, 0), point) <= range,
        }
    }
}

impl Default for DistanceAlgorithm {
    fn default() -> Self {
        Self::EuclideanSquared
    }
}
