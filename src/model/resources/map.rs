use bevy::{platform::collections::HashMap, prelude::*};
use bevy_ecs_tilemap::prelude::*;
use brtk::prelude::*;

use crate::model::{
    ModelConstants,
    components::{Position, TerrainType},
};

#[derive(Reflect, Clone, Resource)]
pub struct Map {
    pub size: (usize, usize),
    pub terrain: Grid<TerrainType>,
    pub tile_storage: TileStorage,
    pub actors: HashMap<Position, Entity>,
}

impl FromWorld for Map {
    fn from_world(_world: &mut World) -> Self {
        let size = (ModelConstants::MAP_WIDTH, ModelConstants::MAP_HEIGHT);
        Self {
            size,
            actors: HashMap::new(),
            terrain: Grid::new_fill(size, TerrainType::Wall),
            tile_storage: TileStorage::empty(TilemapSize::new(size.0 as u32, size.1 as u32)),
        }
    }
}

impl Map {
    pub fn new(size: (usize, usize)) -> Self {
        let terrain = Grid::new_fill(size, TerrainType::Wall);
        Self {
            size,
            terrain,
            actors: HashMap::new(),
            tile_storage: TileStorage::empty(TilemapSize::new(size.0 as u32, size.1 as u32)),
        }
    }

    pub fn pos_to_idx(&self, position: Position) -> usize { self.terrain.position_to_index_unchecked(position.into()) }
    pub fn idx_to_pos(&self, idx: usize) -> Option<(i32, i32)> { self.terrain.index_to_position(idx) }

    // Helper method to check if a position is in bounds
    pub fn in_bounds(&self, position: Position) -> bool {
        let (x, y) = position.into();
        x >= 0 && y >= 0 && x < self.size.0 as i32 && y < self.size.1 as i32
    }

    pub fn get_terrain(&self, position: Position) -> Option<TerrainType> { self.terrain.get(position.into()).copied() }
    pub fn get_terrain_entity(&self, position: Position) -> Option<Entity> {
        self.tile_storage.get(&TilePos::new(position.x() as u32, position.y() as u32))
    }
    // pub fn get_mut_terrain(&mut self, position: Position) -> Option<&mut TerrainType> {
    //     self.terrain.get_mut(position.into())
    // }

    pub fn get_actor(&self, position: Position) -> Option<Entity> { self.actors.get(&position).copied() }
    pub fn get_actor_mut(&mut self, position: Position) -> Option<&mut Entity> { self.actors.get_mut(&position) }
    pub fn set_actor(&mut self, position: Position, actor: Option<Entity>) {
        if let Some(actor) = actor {
            self.actors.insert(position, actor);
        } else {
            self.actors.remove(&position);
        }
    }
}
