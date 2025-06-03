//! Advanced shadowcasting algorithm implementation

use super::{quadrant::Quadrant, row::Row};
use crate::{
    direction::Direction,
    fov::{
        traits::{FovAlgorithm, FovProvider, FovReceiver},
        utils::{distance::DistanceAlgorithm, slope::Slope},
    },
};

/// Advanced shadowcasting algorithm implementation
pub struct Shadowcast;

impl FovAlgorithm for Shadowcast {
    fn compute_fov<P: FovProvider, R: FovReceiver>(
        origin: (i32, i32),
        vision_type: u8,
        range: u32,
        provider: &mut P,
        receiver: &mut R,
    ) {
        receiver.clear_visible();
        receiver.set_visible(origin);

        // Process all 8 cardinal and diagonal directions
        let directions = [
            Direction::North,
            Direction::NorthEast,
            Direction::East,
            Direction::SouthEast,
            Direction::South,
            Direction::SouthWest,
            Direction::West,
            Direction::NorthWest,
        ];

        for direction in directions {
            let mut quadrant = Quadrant::new(direction, origin, vision_type, provider, receiver);
            let mut first_row = Row::new(1, Slope::new(-1, 1), Slope::new(1, 1));
            Self::scan_recursive(range, &mut quadrant, &mut first_row);
        }
    }
}

impl Shadowcast {
    pub fn compute_direction<P: FovProvider, R: FovReceiver>(
        origin: (i32, i32),
        vision_type: u8,
        range: u32,
        provider: &mut P,
        receiver: &mut R,
        direction: Direction,
    ) {
        receiver.clear_visible();
        receiver.set_visible(origin);

        let mut quadrant = Quadrant::new(direction, origin, vision_type, provider, receiver);
        let mut first_row = Row::new(1, Slope::new(-1, 1), Slope::new(1, 1));
        Self::scan_recursive(range, &mut quadrant, &mut first_row);
    }

    fn scan_recursive<P: FovProvider, R: FovReceiver>(
        range: u32,
        quadrant: &mut Quadrant<P, R>,
        row: &mut Row,
    ) {
        let mut prev_tile = None;
        let distance_alg = DistanceAlgorithm::EuclideanSquared;

        for tile in row.tiles() {
            // Check if tile is within range
            if !distance_alg.within_range_of_origin(tile, range as f32) {
                continue;
            }

            // Determine if we should reveal this tile
            let should_reveal = quadrant.is_opaque(tile) || row.is_symmetric(tile);
            
            if should_reveal {
                quadrant.set_visible(tile);
            }

            // Handle shadow casting based on the previous tile
            if let Some(prev_tile) = prev_tile {
                // Transition from opaque to clear: start of new visible area
                if quadrant.is_opaque(prev_tile) && quadrant.is_clear(tile) {
                    row.calc_starting_slope(tile);
                }
                
                // Transition from clear to opaque: end of visible area, start recursion
                if quadrant.is_clear(prev_tile) && quadrant.is_opaque(tile) {
                    let mut next_row = row.next();
                    next_row.calc_ending_slope(tile);
                    Self::scan_recursive(range, quadrant, &mut next_row);
                }
            }

            prev_tile = Some(tile);
        }

        // If the last tile was clear, continue scanning the next row
        if let Some(prev_tile) = prev_tile {
            if quadrant.is_clear(prev_tile) {
                Self::scan_recursive(range, quadrant, &mut row.next());
            }
        }
    }
}
