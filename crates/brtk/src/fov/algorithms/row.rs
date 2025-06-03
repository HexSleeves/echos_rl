//! Row iteration for shadowcasting algorithm

use crate::fov::utils::Slope;

/// Represents a row in the shadowcasting algorithm
pub struct Row {
    pub depth: u32,
    pub start_slope: Slope,
    pub end_slope: Slope,
}

impl Row {
    pub const fn new(depth: u32, start_slope: Slope, end_slope: Slope) -> Self {
        Self {
            depth,
            start_slope,
            end_slope,
        }
    }

    pub const fn next(&self) -> Self {
        Self {
            depth: self.depth + 1,
            start_slope: self.start_slope,
            end_slope: self.end_slope,
        }
    }

    pub fn calc_starting_slope(&mut self, tile: (i32, i32)) {
        self.start_slope = Self::slope(tile);
    }

    pub fn calc_ending_slope(&mut self, tile: (i32, i32)) {
        self.end_slope = Self::slope(tile);
    }

    const fn slope(tile: (i32, i32)) -> Slope {
        Slope::new(2 * tile.1 - 1, 2 * tile.0)
    }

    pub fn tiles(&self) -> RowIter {
        let start_col = (self.depth as f64 * self.start_slope.value() + 0.5).floor() as i32;
        let end_col = (self.depth as f64 * self.end_slope.value() - 0.5).ceil() as i32;

        RowIter {
            depth: self.depth,
            current_col: start_col,
            max_col: end_col,
        }
    }

    pub fn is_symmetric(&self, tile: (i32, i32)) -> bool {
        let tile_y = tile.1 as f64;
        let depth = self.depth as f64;
        
        tile_y >= depth * self.start_slope.value() && tile_y <= depth * self.end_slope.value()
    }
}

pub struct RowIter {
    depth: u32,
    max_col: i32,
    current_col: i32,
}

impl Iterator for RowIter {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_col > self.max_col {
            None
        } else {
            let tile = (self.depth as i32, self.current_col);
            self.current_col += 1;
            Some(tile)
        }
    }
}
