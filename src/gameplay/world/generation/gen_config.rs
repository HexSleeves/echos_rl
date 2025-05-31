use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use brtk::grid::Grid;

use crate::{
    core::{components::Description, constants::ModelConstants},
    gameplay::world::components::{TerrainType, UndergroundType},
};

use super::Room;

/// Configuration for level generation based on depth
#[derive(Debug, Clone, Reflect, Resource)]
pub struct GenConfig {
    pub depth: usize,
    pub width: u32,
    pub height: u32,
    pub underground_type: UndergroundType,

    // Room generation
    pub rooms: Vec<Room>,
    pub room_count: (usize, usize), // min and max rooms
    pub room_size: (usize, usize),  // min and max room size

    // Ore generation
    pub ore_density: f32,            // 0.0 to 1.0
    pub hazard_density: f32,         // 0.0 to 1.0
    pub special_feature_chance: f32, // 0.0 to 1.0
    pub echo_chamber_chance: f32,    // 0.0 to 1.0

    pub player_spawn_point: Option<(u32, u32)>,
}

impl Default for GenConfig {
    fn default() -> Self {
        Self {
            depth: 1,
            ore_density: 0.3,
            room_size: (4, 8),
            rooms: Vec::new(),
            room_count: (3, 6),
            hazard_density: 0.1,
            echo_chamber_chance: 0.1,
            special_feature_chance: 0.05,
            width: ModelConstants::MAP_WIDTH,
            height: ModelConstants::MAP_HEIGHT,
            underground_type: UndergroundType::Mine,
            player_spawn_point: None,
        }
    }
}

impl GenConfig {
    /// Create a new dungeon generator with the specified dimensions and default settings
    pub fn new(depth: usize, width: u32, height: u32) -> Self {
        let underground_type = match depth {
            d if d <= 5 => UndergroundType::Mine,
            d if d <= 10 => UndergroundType::Cave,
            _ => UndergroundType::Cave, // or introduce a new enum variant
        };

        // Adjust generation parameters based on depth
        let (room_count, room_size, ore_density, hazard_density, special_feature_chance, echo_chamber_chance) =
            match depth {
                d if d <= 3 => ((3, 6), (4, 8), 0.3, 0.1, 0.05, 0.1), // Easy start
                d if d <= 7 => ((5, 8), (3, 7), 0.5, 0.2, 0.1, 0.2),  // Getting harder
                d if d <= 12 => ((7, 10), (3, 6), 0.6, 0.3, 0.15, 0.3),
                _ => ((8, 12), (3, 5), 0.7, 0.4, 0.2, 0.4), // Very deep = more dangerous
            };

        Self {
            width,
            height,
            depth,
            underground_type,
            room_count,
            room_size,
            ore_density,
            hazard_density,
            special_feature_chance,
            echo_chamber_chance,
            ..Default::default()
        }
    }

    /// Generate a complete dungeon map with rooms, corridors, doors, and stairs
    ///
    /// This method orchestrates the entire dungeon generation process, including:
    /// - Creating a grid filled with walls
    /// - Placing rooms of varying sizes
    /// - Carving corridors to connect rooms
    /// - Adding doors at appropriate locations
    /// - Placing up and down stairs in distant rooms
    ///
    /// Returns a Grid<TerrainType> representing the completed dungeon
    pub fn generate(&mut self, rng: &mut fastrand::Rng) -> Grid<TerrainType> {
        // Initialize grid with walls
        let mut grid = Grid::new_fill((self.width, self.height), TerrainType::Wall);

        // Clear existing rooms
        self.rooms.clear();

        // Generate new rooms
        self.generate_rooms(rng);

        // Carve rooms
        for room in &self.rooms {
            self.carve_room(&mut grid, room);
        }

        // Connect rooms
        if !self.rooms.is_empty() {
            let mut rooms = self.rooms.clone();
            // Sort rooms for consistent corridor generation
            rooms.sort_by_key(|room| room.center().0 + room.center().1);

            // Connect each room to the next one
            for i in 0..rooms.len() - 1 {
                let from = rooms[i].center();
                let to = rooms[i + 1].center();
                self.carve_corridor(&mut grid, from, to);
            }
        }

        // Place stairs
        if !self.rooms.is_empty() {
            self.place_stairs(&mut grid, rng);
        }

        // Find player spawn point
        if let Some(player_spawn_point) = self.find_valid_position(&grid) {
            self.player_spawn_point = Some(player_spawn_point);
        }

        grid
    }

    /// Generate random non-overlapping rooms within the dungeon boundaries
    ///
    /// This method creates rooms with random sizes and positions within the dungeon,
    /// ensuring they don't overlap with existing rooms. It attempts room placement
    /// multiple times if needed to reach the desired number of rooms.
    fn generate_rooms(&mut self, rng: &mut fastrand::Rng) {
        let mut attempts = 0;

        // Determine number of rooms based on config
        let max_rooms = rng.usize(self.room_count.0..=self.room_count.1);
        let max_attempts = max_rooms * 3; // Allow for placement failures
        let (min_room_size, max_room_size) = self.room_size;

        while self.rooms.len() < max_rooms && attempts < max_attempts {
            let room = Room::random(
                rng,
                min_room_size as i32,
                max_room_size as i32,
                (self.width as i32, self.height as i32),
            );

            // Check if room overlaps with existing rooms
            let is_valid = !self.rooms.iter().any(|r| r.intersects(&room));

            if is_valid {
                self.rooms.push(room);
            }

            attempts += 1;
        }
    }

    /// Carve a room into the dungeon grid by setting all cells within the room to Floor
    ///
    /// This method modifies the grid in-place, changing all cells within the room's
    /// boundaries from walls to floor tiles.
    fn carve_room(&self, grid: &mut Grid<TerrainType>, room: &Room) {
        for (x, y) in room.positions() {
            if let Some(cell) = grid.get_mut((x, y)) {
                *cell = TerrainType::Floor;
            }
        }
    }

    /// Carve a corridor between two points, using either horizontal-first or vertical-first
    /// approach
    fn carve_corridor(&self, grid: &mut Grid<TerrainType>, from: (i32, i32), to: (i32, i32)) {
        let (mut x, mut y) = from;

        // Alternate between horizontal-first and vertical-first corridors
        // Using a simple deterministic approach rather than random
        let horizontal_first = (from.0 + from.1) % 2 == 0;

        if horizontal_first {
            // Horizontal then vertical
            while x != to.0 {
                x += (to.0 - x).signum();
                let position = (x, y);
                if grid.in_bounds(position) {
                    grid[position] = TerrainType::Floor;
                }
            }

            while y != to.1 {
                y += (to.1 - y).signum();
                let position = (x, y);
                if grid.in_bounds(position) {
                    grid[position] = TerrainType::Floor;
                }
            }
        } else {
            // Vertical then horizontal
            while y != to.1 {
                y += (to.1 - y).signum();
                let position = (x, y);
                if grid.in_bounds(position) {
                    grid[position] = TerrainType::Floor;
                }
            }

            while x != to.0 {
                x += (to.0 - x).signum();
                let position = (x, y);
                if grid.in_bounds(position) {
                    grid[position] = TerrainType::Floor;
                }
            }
        }
    }

    /// Place up and down stairs in different rooms, away from walls
    fn place_stairs(&self, grid: &mut Grid<TerrainType>, rng: &mut fastrand::Rng) {
        if self.rooms.len() < 2 {
            return; // Need at least 2 rooms for stairs
        }

        // Choose two distant rooms for stairs
        let mut room_indices: Vec<usize> = (0..self.rooms.len()).collect();
        room_indices.sort_by_key(|&i| {
            let room = &self.rooms[i];
            let center = room.center();
            // Use Manhattan distance from top-left to create a consistent ordering
            center.0 + center.1
        });

        // Get first and last rooms (most distant)
        let first_room_idx = room_indices[0];
        let last_room_idx = room_indices[room_indices.len() - 1];

        // Find suitable positions in first room for up stairs
        let mut up_stair_candidates = Vec::new();
        let first_room = &self.rooms[first_room_idx];
        for (x, y) in first_room.inner_positions() {
            if grid.in_bounds((x, y)) && grid[(x, y)] == TerrainType::Floor {
                // Check if position is away from walls (has mostly floor neighbors)
                let floor_neighbors = [(-1, -1), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1)]
                    .iter()
                    .filter(|&(dx, dy)| {
                        let nx = x + dx;
                        let ny = y + dy;
                        grid.in_bounds((nx, ny)) && grid[(nx, ny)] == TerrainType::Floor
                    })
                    .count();

                // If position has at least 6 floor neighbors, it's good for stairs
                if floor_neighbors >= 6 {
                    up_stair_candidates.push((x as usize, y as usize));
                }
            }
        }

        // Find suitable positions in last room for down stairs
        let mut down_stair_candidates = Vec::new();
        let last_room = &self.rooms[last_room_idx];
        for (x, y) in last_room.inner_positions() {
            if grid.in_bounds((x, y)) && grid[(x, y)] == TerrainType::Floor {
                // Check if position is away from walls (has mostly floor neighbors)
                let floor_neighbors = [(-1, -1), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1)]
                    .iter()
                    .filter(|&(dx, dy)| {
                        let nx = x + dx;
                        let ny = y + dy;
                        grid.in_bounds((nx, ny)) && grid[(nx, ny)] == TerrainType::Floor
                    })
                    .count();

                // If position has at least 6 floor neighbors, it's good for stairs
                if floor_neighbors >= 6 {
                    down_stair_candidates.push((x as usize, y as usize));
                }
            }
        }

        // Place stairs if we have candidates
        if !up_stair_candidates.is_empty() {
            let (x, y) = up_stair_candidates[rng.usize(0..up_stair_candidates.len())];
            // Convert to i32 for grid indexing
            grid[(x as i32, y as i32)] = TerrainType::StairsUp;
        }

        if !down_stair_candidates.is_empty() {
            let (x, y) = down_stair_candidates[rng.usize(0..down_stair_candidates.len())];
            // Convert to i32 for grid indexing
            grid[(x as i32, y as i32)] = TerrainType::StairsDown;
        }
    }

    /// Convert the terrain grid into actual game entities
    pub fn generate_tile_storage(
        &self,
        mut commands: Commands,
        tilemap_id: TilemapId,
        terrain_grid: &Grid<TerrainType>,
    ) -> TileStorage {
        let mut tile_storage = TileStorage::empty(TilemapSize::new(self.width, self.height));

        for y in 0..self.height {
            for x in 0..self.width {
                let tile_pos = TilePos { x, y };

                let terrain_type = terrain_grid[(x as i32, y as i32)];
                let description = Description::new(terrain_type.description());
                let texture_index = TileTextureIndex(terrain_type.texture_index());

                let tile_entity = commands
                    .spawn((
                        description,
                        terrain_type,
                        TileBundle { tilemap_id, position: tile_pos, texture_index, ..Default::default() },
                    ))
                    .id();
                tile_storage.set(&tile_pos, tile_entity);
            }
        }

        tile_storage
    }

    // Helper function to find a valid floor position
    fn find_valid_position(&self, grid: &Grid<TerrainType>) -> Option<(u32, u32)> {
        let mut rng = fastrand::Rng::new();
        let mut valid_positions = Vec::new();

        for (x, y) in grid.position_iter() {
            if grid.in_bounds((x, y)) && grid[(x, y)].is_walkable() {
                valid_positions.push((x as u32, y as u32));
            }
        }

        if valid_positions.is_empty() {
            return None;
        }

        Some(valid_positions[rng.usize(0..valid_positions.len())])
    }
}
