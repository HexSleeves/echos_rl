use crate::core::components::Position;

/// Calculate the Euclidean distance between two positions
pub fn calculate_distance(a: Position, b: Position) -> f32 {
    let dx = (a.x() - b.x()) as f32;
    let dy = (a.y() - b.y()) as f32;
    (dx * dx + dy * dy).sqrt()
}

/// Calculate the Manhattan distance between two positions
pub fn manhattan_distance(a: Position, b: Position) -> u32 {
    ((a.x() - b.x()).abs() + (a.y() - b.y()).abs()) as u32
}

/// Calculate the squared distance (faster than Euclidean when you only need comparison)
pub fn distance_squared(a: Position, b: Position) -> f32 {
    let dx = (a.x() - b.x()) as f32;
    let dy = (a.y() - b.y()) as f32;
    dx * dx + dy * dy
}

/// Check if two positions are adjacent (including diagonally)
pub fn are_adjacent(a: Position, b: Position) -> bool {
    let dx = (a.x() - b.x()).abs();
    let dy = (a.y() - b.y()).abs();
    dx <= 1 && dy <= 1 && (dx + dy) > 0
}

/// Check if two positions are cardinally adjacent (not diagonal)
pub fn are_cardinally_adjacent(a: Position, b: Position) -> bool {
    let dx = (a.x() - b.x()).abs();
    let dy = (a.y() - b.y()).abs();
    (dx == 1 && dy == 0) || (dx == 0 && dy == 1)
}

/// Get all positions within a given radius
pub fn positions_in_radius(center: Position, radius: u32) -> Vec<Position> {
    let mut positions = Vec::new();
    let radius = radius as i32;

    for dx in -radius..=radius {
        for dy in -radius..=radius {
            if dx == 0 && dy == 0 {
                continue; // Skip center
            }

            let squared_distance = dx * dx + dy * dy;
            if squared_distance <= (radius * radius) {
                positions.push(Position::new(center.x() + dx, center.y() + dy));
            }
        }
    }

    positions
}

/// Get positions in a line between two points (Bresenham's line algorithm)
pub fn line_between(start: Position, end: Position) -> Vec<Position> {
    let mut positions = Vec::new();
    
    let mut x0 = start.x();
    let mut y0 = start.y();
    let x1 = end.x();
    let y1 = end.y();

    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx - dy;

    loop {
        positions.push(Position::new(x0, y0));

        if x0 == x1 && y0 == y1 {
            break;
        }

        let e2 = 2 * err;
        if e2 > -dy {
            err -= dy;
            x0 += sx;
        }
        if e2 < dx {
            err += dx;
            y0 += sy;
        }
    }

    positions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_distance() {
        let a = Position::new(0, 0);
        let b = Position::new(3, 4);
        assert_eq!(calculate_distance(a, b), 5.0);
    }

    #[test]
    fn test_manhattan_distance() {
        let a = Position::new(0, 0);
        let b = Position::new(3, 4);
        assert_eq!(manhattan_distance(a, b), 7);
    }

    #[test]
    fn test_are_adjacent() {
        let center = Position::new(5, 5);
        assert!(are_adjacent(center, Position::new(4, 4))); // Diagonal
        assert!(are_adjacent(center, Position::new(5, 4))); // Cardinal
        assert!(!are_adjacent(center, Position::new(3, 3))); // Too far
    }

    #[test]
    fn test_are_cardinally_adjacent() {
        let center = Position::new(5, 5);
        assert!(!are_cardinally_adjacent(center, Position::new(4, 4))); // Diagonal
        assert!(are_cardinally_adjacent(center, Position::new(5, 4))); // Cardinal
        assert!(!are_cardinally_adjacent(center, Position::new(3, 3))); // Too far
    }

    #[test]
    fn test_line_between() {
        let start = Position::new(0, 0);
        let end = Position::new(2, 2);
        let line = line_between(start, end);
        
        assert!(line.contains(&Position::new(0, 0)));
        assert!(line.contains(&Position::new(1, 1)));
        assert!(line.contains(&Position::new(2, 2)));
    }
}
