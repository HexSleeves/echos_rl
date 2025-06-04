//! Quadrant handling for shadowcasting algorithm

use crate::{
    direction::Direction,
    fov::traits::{FovProvider, FovReceiver},
};

/// Represents one quadrant/octant in the shadowcasting algorithm
pub struct Quadrant<'a, P: FovProvider, R: FovReceiver> {
    direction: Direction,
    origin: (i32, i32),
    vision_type: u8,
    provider: &'a P,
    receiver: &'a mut R,
}

impl<'a, P: FovProvider, R: FovReceiver> Quadrant<'a, P, R> {
    pub fn new(
        direction: Direction,
        origin: (i32, i32),
        vision_type: u8,
        provider: &'a P,
        receiver: &'a mut R,
    ) -> Self {
        Self { direction, origin, vision_type, provider, receiver }
    }

    fn transform(&self, tile: (i32, i32)) -> (i32, i32) {
        let offset = match self.direction {
            Direction::NORTH => (tile.1, -tile.0),
            Direction::SOUTH => (tile.1, tile.0),
            Direction::EAST => (tile.0, tile.1),
            Direction::WEST => (-tile.0, tile.1),
            Direction::NORTH_EAST => {
                if tile.0 >= tile.1 {
                    (tile.0, -tile.1)
                } else {
                    (tile.1, -tile.0)
                }
            }
            Direction::NORTH_WEST => {
                if tile.0 >= tile.1 {
                    (-tile.0, -tile.1)
                } else {
                    (-tile.1, -tile.0)
                }
            }
            Direction::SOUTH_EAST => {
                if tile.0 >= tile.1 {
                    (tile.0, tile.1)
                } else {
                    (tile.1, tile.0)
                }
            }
            Direction::SOUTH_WEST => {
                if tile.0 >= tile.1 {
                    (-tile.0, tile.1)
                } else {
                    (-tile.1, tile.0)
                }
            }
            _ => {
                // Invalid direction - return origin to avoid silent failures
                debug_assert!(false, "Invalid direction in transform: {:?}", self.direction);
                (0, 0)
            }
        };

        (self.origin.0 + offset.0, self.origin.1 + offset.1)
    }

    pub fn set_visible(&mut self, tile: (i32, i32)) {
        let global_pos = self.transform(tile);
        self.receiver.set_visible(global_pos);
    }

    pub fn is_opaque(&mut self, tile: (i32, i32)) -> bool {
        let global_pos = self.transform(tile);
        self.provider.is_opaque(global_pos, self.vision_type)
    }

    pub fn is_clear(&mut self, tile: (i32, i32)) -> bool { !self.is_opaque(tile) }
}
