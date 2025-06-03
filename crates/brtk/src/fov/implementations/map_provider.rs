//! Map provider implementation for integration with game maps

use crate::fov::traits::FovProvider;

/// A generic map provider that wraps a map and provides opacity information
pub struct MapProvider<M, F>
where
    F: FnMut(&M, (i32, i32), u8) -> bool,
{
    map: M,
    opacity_fn: F,
}

impl<M, F> MapProvider<M, F>
where
    F: FnMut(&M, (i32, i32), u8) -> bool,
{
    pub fn new(map: M, opacity_fn: F) -> Self {
        Self { map, opacity_fn }
    }

    pub fn map(&self) -> &M {
        &self.map
    }

    pub fn map_mut(&mut self) -> &mut M {
        &mut self.map
    }
}

impl<M, F> FovProvider for MapProvider<M, F>
where
    F: FnMut(&M, (i32, i32), u8) -> bool,
{
    fn is_opaque(&mut self, position: (i32, i32), vision_type: u8) -> bool {
        (self.opacity_fn)(&self.map, position, vision_type)
    }
}

/// A simple grid-based map provider for testing and basic use cases
pub struct GridMapProvider {
    grid: Vec<Vec<bool>>,
    width: usize,
    height: usize,
}

impl GridMapProvider {
    pub fn new(width: usize, height: usize, default_opaque: bool) -> Self {
        let grid = vec![vec![default_opaque; width]; height];
        Self {
            grid,
            width,
            height,
        }
    }

    pub fn set_opaque(&mut self, x: i32, y: i32, opaque: bool) -> bool {
        if self.is_valid_position(x, y) {
            self.grid[y as usize][x as usize] = opaque;
            true
        } else {
            false
        }
    }

    fn is_valid_position(&self, x: i32, y: i32) -> bool {
        x >= 0 && y >= 0 && (x as usize) < self.width && (y as usize) < self.height
    }

    pub fn dimensions(&self) -> (usize, usize) {
        (self.width, self.height)
    }
}

impl FovProvider for GridMapProvider {
    fn is_opaque(&mut self, position: (i32, i32), _vision_type: u8) -> bool {
        let (x, y) = position;
        
        if !self.is_valid_position(x, y) {
            return true;
        }
        
        self.grid[y as usize][x as usize]
    }
}
