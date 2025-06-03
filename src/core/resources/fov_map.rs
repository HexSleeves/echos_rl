use bevy::prelude::*;
use bitvec::prelude::*;
use brtk::fov::{FovAlgorithm as BrtkFovAlgorithm, FovReceiver, Shadowcast};

use crate::core::{components::Position, constants::ModelConstants, resources::Map};

/// Field of view map using bit-level storage for memory efficiency.
/// This implementation uses the BitVec crate to store boolean values as individual bits.
#[derive(Resource)]
pub struct FovMap {
    width: usize,
    height: usize,
    visible: BitVec,
    revealed: BitVec,
}

impl FromWorld for FovMap {
    fn from_world(_world: &mut World) -> Self {
        let size = (ModelConstants::MAP_WIDTH as usize, ModelConstants::MAP_HEIGHT as usize);
        Self::new(size.0, size.1)
    }
}

impl FovMap {
    pub fn new(width: usize, height: usize) -> Self {
        let size = width * height;

        Self { width, height, revealed: bitvec![0; size], visible: bitvec![0; size] }
    }

    /// Converts 2D coordinates to a 1D index
    fn coords_to_index(&self, x: i32, y: i32) -> Option<usize> {
        if x < 0 || y < 0 || x >= self.width as i32 || y >= self.height as i32 {
            return None;
        }
        Some(y as usize * self.width + x as usize)
    }

    /// Checks if a position is revealed (has been seen before)
    pub fn is_revealed(&self, pos: Position) -> bool {
        let (x, y) = pos.into();
        self.coords_to_index(x, y).map(|idx| self.revealed[idx]).unwrap_or(false)
    }

    /// Checks if a position is currently visible
    pub fn is_visible(&self, pos: Position) -> bool {
        let (x, y) = pos.into();
        self.coords_to_index(x, y).map(|idx| self.visible[idx]).unwrap_or(false)
    }

    /// Marks a position as revealed
    pub fn set_revealed(&mut self, pos: Position, value: bool) {
        let (x, y) = pos.into();
        if let Some(idx) = self.coords_to_index(x, y) {
            self.revealed.set(idx, value);
        }
    }

    /// Marks a position as visible
    pub fn set_visible(&mut self, pos: Position, value: bool) {
        let (x, y) = pos.into();
        if let Some(idx) = self.coords_to_index(x, y) {
            self.visible.set(idx, value);
            if value {
                self.revealed.set(idx, true);
            }
        }
    }

    /// Clears all visibility flags (called at the start of each turn)
    pub fn clear_visibility(&mut self) { self.visible.fill(false); }

    /// Updates the FOV for an entity at the given position with the given radius
    pub fn compute_fov(&mut self, map: &Map, origin: Position, radius: u8) {
        self.clear_visibility();

        // Always mark the origin as visible
        self.set_visible(origin, true);

        // Dispatch to the appropriate algorithm
        self.compute_fov_advanced_shadowcasting(map, origin, radius as u32)
    }
}

// ADVANCED SHADOWCASTING
impl FovMap {
    /// Advanced shadowcasting FOV implementation using the brtk FOV system
    fn compute_fov_advanced_shadowcasting(&mut self, map: &Map, origin: Position, radius: u32) {
        // Use the advanced shadowcasting algorithm from brtk
        let origin_coords = (origin.x(), origin.y());
        Shadowcast::compute_fov(origin_coords, 0, radius, map, self);
    }
}

impl FovReceiver for FovMap {
    fn set_visible(&mut self, position: (i32, i32)) {
        let pos = Position::new(position.0, position.1);
        self.set_visible(pos, true);
    }

    fn get_visible(&self, position: (i32, i32)) -> bool {
        let pos = Position::new(position.0, position.1);
        self.is_visible(pos)
    }

    fn clear_visible(&mut self) { self.clear_visibility(); }
}
