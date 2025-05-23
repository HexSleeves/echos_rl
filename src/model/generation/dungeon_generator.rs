use bevy::prelude::*;
use brtk::grid::Grid;

use crate::model::{
    ModelConstants,
    components::{Description, Position, TerrainType},
};

use super::Room;

pub struct DungeonGenerator {
    pub width: usize,
    pub height: usize,
    pub min_room_size: i32,
    pub max_room_size: i32,
    pub max_rooms: usize,
    pub rooms: Vec<Room>,
}

impl Default for DungeonGenerator {
    fn default() -> Self {
        Self {
            width: ModelConstants::MAP_WIDTH,
            height: ModelConstants::MAP_HEIGHT,
            min_room_size: 5,
            max_room_size: 10,
            max_rooms: 15,
            rooms: Vec::new(),
        }
    }
}

impl DungeonGenerator {
    /// Create a new dungeon generator with the specified dimensions and default settings
    pub fn new(width: usize, height: usize) -> Self { Self { width, height, ..Default::default() } }

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

        // Add doors between rooms and corridors
        // self.place_doors(&mut grid, rng);

        // Place stairs
        if !self.rooms.is_empty() {
            self.place_stairs(&mut grid, rng);
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
        let max_attempts = self.max_rooms * 3; // Allow for placement failures

        while self.rooms.len() < self.max_rooms && attempts < max_attempts {
            let room =
                Room::random(rng, self.min_room_size, self.max_room_size, (self.width as i32, self.height as i32));

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

    /// Place doors at suitable locations between rooms and corridors
    fn place_doors(&self, grid: &mut Grid<TerrainType>, rng: &mut fastrand::Rng) {
        let mut door_candidates = Vec::new();

        // Check each room's border for potential entryways
        for room in &self.rooms {
            // Get all border positions of the room
            for border_pos in room.border_positions() {
                // Skip if not a floor (only floors can be doors)
                if !grid.in_bounds(border_pos) || grid[border_pos] != TerrainType::Floor {
                    continue;
                }

                // Check each direction for adjacent floor tiles outside the room
                let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];
                let mut floor_connections_outside = 0;
                let mut floor_connections_inside = 0;

                for &(dx, dy) in &directions {
                    let adjacent_pos = (border_pos.0 + dx, border_pos.1 + dy);

                    if grid.in_bounds(adjacent_pos) && grid[adjacent_pos] == TerrainType::Floor {
                        if room.contains(adjacent_pos) {
                            floor_connections_inside += 1;
                        } else {
                            floor_connections_outside += 1;
                        }
                    }
                }

                // A good doorway should have connections both inside and outside the room
                if floor_connections_outside > 0 && floor_connections_inside > 0 {
                    // Calculate door quality score - we prefer doors that connect well
                    let score = floor_connections_inside + floor_connections_outside;
                    door_candidates.push((border_pos.0, border_pos.1, score));
                }
            }
        }

        log::info!("Found {} door candidates", door_candidates.len());

        // Sort candidates by score (higher is better)
        door_candidates.sort_by(|a, b| b.2.cmp(&a.2));

        // Determine how many doors to place - roughly one per room, with some randomness
        let base_door_count = self.rooms.len();
        let additional_doors = (base_door_count / 2).min(door_candidates.len() / 4);
        let door_count =
            if additional_doors > 0 { base_door_count + rng.usize(0..=additional_doors) } else { base_door_count };

        // Apply a minimum distance between doors to avoid clustered doors
        let min_door_distance: i32 = 3; // Minimum tiles between doors
        let mut placed_doors = Vec::new();

        // Start with the best candidates
        for &(x, y, _) in &door_candidates {
            // Skip if we've placed enough doors
            if placed_doors.len() >= door_count {
                break;
            }

            // Check if this door is too close to another door
            let too_close = placed_doors.iter().any(|&(door_x, door_y)| {
                let x_diff: i32 = door_x - x;
                let y_diff: i32 = door_y - y;
                let manhattan_distance: i32 = x_diff.abs() + y_diff.abs();
                manhattan_distance < min_door_distance
            });

            if !too_close {
                grid[(x, y)] = TerrainType::Door;
                placed_doors.push((x, y));
            }
        }

        log::info!("Placed {} doors", placed_doors.len());
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
    ///
    /// This method creates Bevy entities for each cell in the terrain grid, applying:
    /// - The appropriate TerrainType component
    /// - A descriptive name as a Description component
    /// - A Position component based on the grid coordinates
    ///
    /// Returns a Grid<Entity> with the entity IDs corresponding to each position,
    /// which can be used to reference these entities later.
    pub fn generate_entities(&self, commands: &mut Commands, terrain_grid: &Grid<TerrainType>) -> Grid<Entity> {
        Grid::new_fn((self.width, self.height), |_index, (x, y)| {
            // Convert to i32 for grid indexing
            let terrain_type = terrain_grid[(x as i32, y as i32)].clone();
            let description = Description::new(terrain_type.description());
            commands.spawn((terrain_type, description, Position::new(x as i32, y as i32))).id()
        })
    }
}
