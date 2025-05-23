use crate::gameplay::map::tile::TileType;
use bevy::prelude::*;

use super::tile::UndergroundType;

/// Represents a level in the game world
#[derive(Component, Debug, Reflect)]
pub struct GameMap {
    pub depth: i32,
    pub map: Vec<TileType>,
    pub width: i32,
    pub height: i32,
    pub underground_type: Option<UndergroundType>,
}

impl GameMap {
    pub fn new(width: i32, height: i32, depth: i32, underground_type: Option<UndergroundType>) -> Self {
        let map = vec![TileType::Wall; (width * height) as usize];
        Self { depth, underground_type, map, width, height }
    }

    pub fn in_bounds(&self, x: i32, y: i32) -> bool { x >= 0 && x < self.width && y >= 0 && y < self.height }
    pub fn get_index(&self, x: i32, y: i32) -> usize { (y * self.width + x) as usize }

    pub fn get_tile(&self, x: i32, y: i32) -> TileType {
        if !self.in_bounds(x, y) {
            return TileType::Wall;
        }

        self.map[self.get_index(x, y)]
    }

    pub fn set_tile(&mut self, x: i32, y: i32, tile: TileType) {
        if !self.in_bounds(x, y) {
            return;
        }

        let idx = self.get_index(x, y);
        self.map[idx] = tile;
    }
}
