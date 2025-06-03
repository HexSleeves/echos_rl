use bevy::{platform::collections::HashMap, prelude::*};
use bevy_ecs_tilemap::prelude::*;
use brtk::prelude::*;

use crate::{
    core::{components::Position, constants::ModelConstants},
    gameplay::world::components::TerrainType,
};

/// Represents a single tile in the map with all its properties
#[derive(Debug, Clone, Reflect)]
pub struct Tile {
    pub terrain: TerrainType,
    pub tile_entity: Option<Entity>,
    pub actor: Option<Entity>,
    pub visible: bool,
    pub explored: bool,
}

impl Default for Tile {
    fn default() -> Self {
        Self { terrain: TerrainType::Wall, tile_entity: None, actor: None, visible: false, explored: false }
    }
}

#[derive(Reflect, Clone, Resource)]
pub struct Map {
    pub size: (u32, u32),
    /// Single source of truth for all tile data
    pub tiles: Grid<Tile>,
    /// Fast reverse lookup: entity -> position
    pub actor_positions: HashMap<Entity, Position>,
    /// Tilemap storage for rendering (derived from tiles)
    pub tile_storage: TileStorage,
}

impl FromWorld for Map {
    fn from_world(_world: &mut World) -> Self {
        let size = (ModelConstants::MAP_WIDTH, ModelConstants::MAP_HEIGHT);
        Self::new(size)
    }
}

impl FovProvider for Map {
    fn is_opaque(&self, position: (i32, i32), _vision_type: u8) -> bool {
        let pos = Position::new(position.0, position.1);

        // Out of bounds is opaque
        if !self.in_bounds(pos) {
            return true;
        }

        // Check if terrain blocks vision
        self.get_terrain(pos).map(|terrain| terrain.blocks_vision()).unwrap_or(true)
    }
}

impl Map {
    pub fn new(size: (u32, u32)) -> Self {
        let tiles = Grid::new_fill(size, Tile::default());
        Self {
            size,
            tiles,
            actor_positions: HashMap::new(),
            tile_storage: TileStorage::empty(TilemapSize::new(size.0, size.1)),
        }
    }

    // Position utilities
    pub fn pos_to_idx(&self, position: Position) -> usize {
        self.tiles.position_to_index_unchecked(position.into())
    }

    pub fn idx_to_pos(&self, idx: usize) -> Option<Position> {
        self.tiles.index_to_position(idx).map(|(x, y)| Position::new(x, y))
    }

    pub fn in_bounds(&self, position: Position) -> bool {
        let (x, y) = position.into();
        x >= 0 && y >= 0 && x < self.size.0 as i32 && y < self.size.1 as i32
    }

    // Terrain operations
    pub fn get_terrain(&self, position: Position) -> Option<TerrainType> {
        self.tiles.get(position.into()).map(|tile| tile.terrain)
    }

    pub fn set_terrain(&mut self, position: Position, terrain: TerrainType) {
        if let Some(tile) = self.tiles.get_mut(position.into()) {
            tile.terrain = terrain;
        }
    }

    pub fn is_walkable(&self, position: Position) -> bool {
        self.get_terrain(position).map(|terrain| terrain != TerrainType::Wall).unwrap_or(false)
    }

    // Actor management with bidirectional lookup
    pub fn get_actor(&self, position: Position) -> Option<Entity> {
        self.tiles.get(position.into()).and_then(|tile| tile.actor)
    }

    pub fn get_actor_position(&self, entity: Entity) -> Option<Position> {
        self.actor_positions.get(&entity).copied()
    }

    pub fn place_actor(&mut self, position: Position, actor: Entity) -> Result<(), String> {
        if !self.in_bounds(position) {
            return Err("Position out of bounds".to_string());
        }

        if !self.is_walkable(position) {
            return Err("Position not walkable".to_string());
        }

        if self.get_actor(position).is_some() {
            return Err("Position already occupied".to_string());
        }

        // Remove actor from previous position if it exists
        if let Some(old_pos) = self.actor_positions.get(&actor)
            && let Some(tile) = self.tiles.get_mut((*old_pos).into())
        {
            tile.actor = None;
        }

        // Place actor at new position
        if let Some(tile) = self.tiles.get_mut(position.into()) {
            tile.actor = Some(actor);
            self.actor_positions.insert(actor, position);
        }

        Ok(())
    }

    pub fn remove_actor(&mut self, actor: Entity) -> Option<Position> {
        if let Some(position) = self.actor_positions.remove(&actor) {
            if let Some(tile) = self.tiles.get_mut(position.into()) {
                tile.actor = None;
            }
            Some(position)
        } else {
            None
        }
    }

    pub fn move_actor(&mut self, actor: Entity, new_position: Position) -> Result<Position, String> {
        let old_position = self.get_actor_position(actor).ok_or("Actor not found on map")?;

        self.remove_actor(actor);
        match self.place_actor(new_position, actor) {
            Ok(()) => Ok(old_position),
            Err(e) => {
                // Restore actor to old position if move failed
                let _ = self.place_actor(old_position, actor);
                Err(e)
            }
        }
    }

    // Visibility and exploration
    pub fn set_visible(&mut self, position: Position, visible: bool) {
        if let Some(tile) = self.tiles.get_mut(position.into()) {
            tile.visible = visible;
            if visible {
                tile.explored = true;
            }
        }
    }

    pub fn is_visible(&self, position: Position) -> bool {
        self.tiles.get(position.into()).map(|tile| tile.visible).unwrap_or(false)
    }

    pub fn is_explored(&self, position: Position) -> bool {
        self.tiles.get(position.into()).map(|tile| tile.explored).unwrap_or(false)
    }

    pub fn clear_visibility(&mut self) {
        for tile in self.tiles.iter_mut() {
            tile.visible = false;
        }
    }

    // Spatial queries for roguelike features
    pub fn get_neighbors(&self, position: Position) -> Vec<Position> {
        let mut neighbors = Vec::new();
        let (x, y) = position.into();

        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let neighbor = Position::new(x + dx, y + dy);
                if self.in_bounds(neighbor) {
                    neighbors.push(neighbor);
                }
            }
        }
        neighbors
    }

    pub fn get_actors_in_radius(&self, center: Position, radius: i32) -> Vec<(Position, Entity)> {
        let mut actors = Vec::new();
        let (cx, cy) = center.into();

        for x in (cx - radius)..=(cx + radius) {
            for y in (cy - radius)..=(cy + radius) {
                let pos = Position::new(x, y);
                if self.in_bounds(pos) {
                    let distance_sq = (x - cx).pow(2) + (y - cy).pow(2);
                    if distance_sq <= radius.pow(2)
                        && let Some(actor) = self.get_actor(pos)
                    {
                        actors.push((pos, actor));
                    }
                }
            }
        }
        actors
    }

    // Tile entity management for rendering
    pub fn get_tile_entity(&self, position: Position) -> Option<Entity> {
        self.tiles.get(position.into()).and_then(|tile| tile.tile_entity)
    }

    pub fn set_tile_entity(&mut self, position: Position, entity: Entity) {
        if let Some(tile) = self.tiles.get_mut(position.into()) {
            tile.tile_entity = Some(entity);
        }
    }

    // Pathfinding support
    pub fn get_walkable_neighbors(&self, position: Position) -> Vec<Position> {
        self.get_neighbors(position)
            .into_iter()
            .filter(|&pos| self.is_walkable(pos) && self.get_actor(pos).is_none())
            .collect()
    }

    /// Get a random walkable position on the map
    ///
    /// This is a simple implementation that iterates over all positions and checks if they are
    /// walkable. It is not efficient for large maps.
    ///
    /// Returns `None` if no walkable positions are found.
    pub fn get_random_walkable_position(&self) -> Option<Position> {
        let mut rng = fastrand::Rng::new();
        let mut positions = Vec::new();

        for (x, y) in self.tiles.position_iter() {
            if self.is_walkable(Position::new(x, y)) {
                positions.push(Position::new(x, y));
            }
        }

        if positions.is_empty() {
            return None;
        }

        Some(positions[rng.usize(0..positions.len())])
    }
}
