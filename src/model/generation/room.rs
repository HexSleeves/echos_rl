// No imports needed

use bevy::reflect::Reflect;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect)]
pub struct Room {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl Room {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self { Self { x, y, width, height } }

    pub fn random(rng: &mut fastrand::Rng, min_size: i32, max_size: i32, bounds: (i32, i32)) -> Self {
        let width = rng.i32(min_size..=max_size);
        let height = rng.i32(min_size..=max_size);
        let x = rng.i32(1..(bounds.0 - width - 1));
        let y = rng.i32(1..(bounds.1 - height - 1));

        Self { x, y, width, height }
    }

    pub fn center(&self) -> (i32, i32) { (self.x + self.width / 2, self.y + self.height / 2) }

    pub fn intersects(&self, other: &Room) -> bool {
        // Add a buffer of 1 to ensure rooms aren't directly adjacent
        self.x <= other.x + other.width + 1
            && self.x + self.width + 1 >= other.x
            && self.y <= other.y + other.height + 1
            && self.y + self.height + 1 >= other.y
    }

    pub fn positions(&self) -> impl Iterator<Item = (i32, i32)> + '_ {
        (self.x..self.x + self.width).flat_map(move |x| (self.y..self.y + self.height).map(move |y| (x, y)))
    }

    pub fn border_positions(&self) -> impl Iterator<Item = (i32, i32)> + '_ {
        let x_range = self.x..self.x + self.width;
        // y_range not directly used, just kept for symmetry with x_range
        let _y_range = self.y..self.y + self.height;

        // Top and bottom borders
        let horizontal_borders = x_range.clone().flat_map(move |x| {
            [
                (x, self.y),                   // Top border
                (x, self.y + self.height - 1), // Bottom border
            ]
        });

        // Left and right borders (excluding corners which are already in horizontal_borders)
        let vertical_borders = (self.y + 1..self.y + self.height - 1).flat_map(move |y| {
            [
                (self.x, y),                  // Left border
                (self.x + self.width - 1, y), // Right border
            ]
        });

        horizontal_borders.chain(vertical_borders)
    }

    pub fn inner_positions(&self) -> impl Iterator<Item = (i32, i32)> + '_ {
        ((self.x + 1)..(self.x + self.width - 1))
            .flat_map(move |x| ((self.y + 1)..(self.y + self.height - 1)).map(move |y| (x, y)))
    }

    /// Checks if a given position is inside the room
    pub fn contains(&self, pos: (i32, i32)) -> bool {
        pos.0 >= self.x && pos.0 < self.x + self.width && pos.1 >= self.y && pos.1 < self.y + self.height
    }
}
