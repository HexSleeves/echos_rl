pub enum Distance {
    /// Use the Pythagoras algorithm for determining distance - sqrt(A^2 + B^2)
    Pythagoras,
    /// Use the Pythagoras algorithm for distance, but omitting the square-root for a faster
    /// but squared result.
    PythagorasSquared,
    /// Use Manhattan distance (distance up plus distance along)
    Manhattan,
    /// Use Chebyshev distance (maximum of absolute differences in each dimension)
    Chebyshev,
    /// Use a diagonal distance, the max of the x and y distances
    Diagonal,
    /// Use Euclidean distance (sqrt(A^2 + B^2))
    Euclidean,
    /// Use a diagonal distance, the max of the x and y distances
    DiagonalWithCosts(f32, f32),
}

impl Distance {
    pub fn calculate(self, start: (f32, f32), end: (f32, f32)) -> f32 {
        match self {
            Distance::Pythagoras => pythagoras(start, end),
            Distance::PythagorasSquared => pythagoras_squared(start, end),
            Distance::Manhattan => manhattan(start, end),
            Distance::Euclidean => euclidean(start, end),
            Distance::Chebyshev => chebyshev(start, end),
            Distance::Diagonal => diagonal(start, end),
            Distance::DiagonalWithCosts(d1, d2) => diagonal_with_costs(start, end, d1, d2),
        }
    }
}

fn pythagoras(start: (f32, f32), end: (f32, f32)) -> f32 { pythagoras_squared(start, end).sqrt() }

fn pythagoras_squared(start: (f32, f32), end: (f32, f32)) -> f32 {
    let dx = (start.0.max(end.0) - start.0.min(end.0)).powf(2.0);
    let dy = (start.1.max(end.1) - start.1.min(end.1)).powf(2.0);
    dx + dy
}

fn manhattan(start: (f32, f32), end: (f32, f32)) -> f32 {
    let dx = start.0.max(end.0) - start.0.min(end.0);
    let dy = start.1.max(end.1) - start.1.min(end.1);
    dx + dy
}

fn chebyshev(start: (f32, f32), end: (f32, f32)) -> f32 {
    (start.0 - end.0).abs().max((start.1 - end.1).abs())
}

fn diagonal(start: (f32, f32), end: (f32, f32)) -> f32 { diagonal_with_costs(start, end, 1.0, 1.0) }

fn diagonal_with_costs(start: (f32, f32), end: (f32, f32), cardinal_cost: f32, ordinal_cost: f32) -> f32 {
    let dx = (start.0 - end.0).abs();
    let dy = (start.1 - end.1).abs();
    cardinal_cost.mul_add(dx.max(dy), (ordinal_cost - cardinal_cost) * dx.min(dy))
}

/// Calculate Euclidean distance between two points
fn euclidean(a: (f32, f32), b: (f32, f32)) -> f32 {
    let dx = a.0 - b.0;
    let dy = a.1 - b.1;
    (dx * dx + dy * dy).sqrt()
}

#[cfg(test)]
mod tests {
    use super::Distance;

    #[test]
    fn test_distances() {
        let pythagoras = Distance::Pythagoras.calculate((0.0, 0.0), (10.0, 10.0));
        let difference = pythagoras - f32::sqrt(200.0);
        assert!(difference <= f32::EPSILON);

        let pythagoras_squared = Distance::PythagorasSquared.calculate((0.0, 0.0), (10.0, 10.0));
        let difference = pythagoras_squared - 200.0;
        assert!(difference <= f32::EPSILON);

        let manhattan = Distance::Manhattan.calculate((0.0, 0.0), (10.0, 10.0));
        let difference = manhattan - 20.0;
        assert!(difference <= f32::EPSILON);

        let chebyshev = Distance::Chebyshev.calculate((0.0, 0.0), (10.0, 10.0));
        let difference = chebyshev - 10.0;
        assert!(difference <= f32::EPSILON);

        let diagonal = Distance::Diagonal.calculate((0.0, 0.0), (10.0, 10.0));
        let difference = diagonal - 10.0;
        assert!(difference <= f32::EPSILON);

        let diagonal_with_costs = Distance::DiagonalWithCosts(2.0, 2.0).calculate((0.0, 0.0), (10.0, 10.0));
        let difference = diagonal_with_costs - 20.0;
        assert!(difference <= f32::EPSILON);
    }
}
