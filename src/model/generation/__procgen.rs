use bevy::{platform::collections::HashSet, prelude::*};

use rand::{Rng, prelude::*};

use super::{
    gamemap::GameMap,
    level_config::LevelConfig,
    tile::{TileType, UndergroundType},
};

/// Places a cluster of ore tiles around the given position
fn place_ore_cluster(level: &mut GameMap, x: i32, y: i32, ore_type: TileType, cluster_size: i32, rng: &mut ThreadRng) {
    let mut placed = 0;
    let mut positions = vec![(x, y)];
    let mut visited = HashSet::new();
    visited.insert((x, y));

    while !positions.is_empty() && placed < cluster_size {
        let (cx, cy) = positions.remove(0);

        // Place ore if it's a wall tile
        if level.get_tile(cx, cy) == TileType::CaveWall || level.get_tile(cx, cy) == TileType::MineWall {
            level.set_tile(cx, cy, ore_type);
            placed += 1;

            // Add adjacent positions
            for (dx, dy) in &[(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let nx = cx + dx;
                let ny = cy + dy;
                if nx > 0 && nx < level.width - 1 && ny > 0 && ny < level.height - 1 && !visited.contains(&(nx, ny)) {
                    visited.insert((nx, ny));
                    positions.push((nx, ny));
                }
            }
        }
    }
}

/// Add sound propagation properties to the level
fn add_sound_properties(level: &mut GameMap, rng: &mut ThreadRng, config: &LevelConfig) {
    // Add echo chambers (amplify sound)
    if config.depth > 3 && rng.random_ratio((config.echo_chamber_chance * 100.0) as u32, 100) {
        let x = rng.random_range(5..level.width - 5);
        let y = rng.random_range(5..level.height - 5);

        // Create a circular chamber
        for dy in -4..=4 {
            for dx in -4..=4 {
                let nx = x + dx;
                let ny = y + dy;
                let dist_sq = dx * dx + dy * dy;

                if dist_sq <= 16 && level.get_tile(nx, ny).is_walkable() {
                    // Mark as echo chamber (could use a special tile type)
                    level.set_tile(nx, ny, TileType::CaveFloor);
                }
            }
        }
    }
}

/// Adds hazards to the level
fn add_hazards(level: &mut GameMap, rng: &mut ThreadRng, config: &LevelConfig) {
    // Add unstable walls that can collapse
    let unstable_walls = (level.width * level.height) as f32 * config.hazard_density * 0.5;
    for _ in 0..unstable_walls as i32 {
        let x = rng.random_range(1..level.width - 1);
        let y = rng.random_range(1..level.height - 1);

        if level.get_tile(x, y) == TileType::MineWall || level.get_tile(x, y) == TileType::CaveWall {
            // Check if adjacent to walkable space
            let mut adjacent_walkable = false;
            for dy in -1..=1 {
                for dx in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    if level.in_bounds(x + dx, y + dy) && level.get_tile(x + dx, y + dy).is_walkable() {
                        adjacent_walkable = true;
                        break;
                    }
                }
                if adjacent_walkable {
                    break;
                }
            }

            if adjacent_walkable {
                level.set_tile(x, y, TileType::UnstableWall);
            }
        }
    }

    // Add gas pockets
    let gas_pockets = (level.width * level.height) as f32 * config.hazard_density * 0.3;
    for _ in 0..gas_pockets as i32 {
        let x = rng.random_range(1..level.width - 1);
        let y = rng.random_range(1..level.height - 1);

        if level.get_tile(x, y).is_walkable() {
            level.set_tile(x, y, TileType::GasPocket);
        }
    }
}

/// Generates a mine level with rooms and corridors
pub fn generate_mine(level: &mut GameMap, rng: &mut ThreadRng, config: &LevelConfig) {
    // Clear the map first with appropriate wall type
    let wall_type = match config.underground_type {
        UndergroundType::Mine => TileType::MineWall,
        UndergroundType::Cave => TileType::CaveWall,
        UndergroundType::DarkAdventure => {
            if rng.random_bool(0.7) {
                TileType::CaveWall
            } else {
                TileType::MineWall
            }
        }
    };

    for i in 0..level.map.len() {
        level.map[i] = wall_type;
    }

    // Determine number of rooms based on config
    let num_rooms = rng.random_range(config.room_count.0..=config.room_count.1);
    let (min_room_size, max_room_size) = config.room_size;

    let mut rooms = Vec::new();

    // Generate rooms
    for _ in 0..num_rooms {
        let w = rng.random_range(min_room_size..=max_room_size);
        let h = rng.random_range(min_room_size..=max_room_size);
        let x = rng.random_range(1..level.width - w - 1);
        let y = rng.random_range(1..level.height - h - 1);

        let room = Rect::new(x, y, w, h);

        let mut overlap = false;
        for other_room in &rooms {
            if room.intersect(other_room) {
                overlap = true;
                break;
            }
        }

        if !overlap {
            apply_room_to_map(level, &room, TileType::MineFloor);
            rooms.push(room);
        }
    }

    // Connect rooms with corridors
    for i in 0..rooms.len() {
        if i == 0 {
            continue;
        }

        let (prev_x, prev_y) = rooms[i - 1].center();
        let (new_x, new_y) = rooms[i].center();

        // 50% chance to draw corridors in different order
        if rng.random_bool(0.5) {
            apply_horizontal_tunnel(level, prev_x, new_x, prev_y, TileType::MineFloor);
            apply_vertical_tunnel(level, prev_y, new_y, new_x, TileType::MineFloor);
        } else {
            apply_vertical_tunnel(level, prev_y, new_y, prev_x, TileType::MineFloor);
            apply_horizontal_tunnel(level, prev_x, new_x, new_y, TileType::MineFloor);
        }
    }

    // Add ore veins based on config
    add_ore_veins(level, rng, config);

    // Add stairs and special features if we have rooms
    if !rooms.is_empty() {
        // Add stairs down in the last room
        if let Some(last_room) = rooms.last() {
            let (x, y) = last_room.center();
            level.set_tile(x, y, TileType::StairsDown);
        }

        // Add stairs up in the first room
        if let Some(first_room) = rooms.first() {
            let (x, y) = first_room.center();
            level.set_tile(x, y, TileType::StairsUp);
        }

        // Add special features
        if rng.random_ratio((config.special_feature_chance * 100.0) as u32, 100) {
            add_special_features(level, rng, config, &rooms);
        }
    }

    // Add sound propagation properties
    add_sound_properties(level, rng, config);

    // Add hazards
    add_hazards(level, rng, config);
}

/// Generates a cave level using cellular automata
pub fn generate_cave(level: &mut GameMap, rng: &mut ThreadRng, config: &LevelConfig) {
    // Initialize with random walls and floors
    let fill_percent = 55 - ((1.0 - config.ore_density) * 10.0) as i32; // Higher ore density = more open caves

    for y in 0..level.height {
        for x in 0..level.width {
            if x == 0 || x == level.width - 1 || y == 0 || y == level.height - 1 {
                level.set_tile(x, y, TileType::CaveWall);
            } else {
                let roll = rng.random_range(0..100);
                if roll < fill_percent {
                    level.set_tile(x, y, TileType::CaveWall);
                } else {
                    level.set_tile(x, y, TileType::CaveFloor);
                }
            }
        }
    }

    // Run cellular automata to smooth the caves
    let iterations = 5;
    for _ in 0..iterations {
        let mut new_map = level.map.clone();

        for y in 1..level.height - 1 {
            for x in 1..level.width - 1 {
                let neighbors = count_walls(level, x, y);
                let idx = level.get_index(x, y);

                if level.map[idx] == TileType::CaveWall {
                    // If a wall has few wall neighbors, convert to floor
                    if neighbors < 4 {
                        new_map[idx] = TileType::CaveFloor;
                    }
                } else {
                    // If a floor has many wall neighbors, convert to wall
                    if neighbors > 5 {
                        new_map[idx] = TileType::CaveWall;
                    }
                }
            }
        }

        level.map = new_map;
    }

    // Add ore veins based on config
    add_ore_veins(level, rng, config);

    // Add sound propagation properties
    add_sound_properties(level, rng, config);

    // Add hazards
    add_hazards(level, rng, config);

    // Add stairs
    add_stairs_to_cave(level, rng);

    // Add special features if enabled
    if rng.random_ratio((config.special_feature_chance * 100.0) as u32, 100) {
        add_special_features_to_cave(level, rng, config);
    }
}

/// Generates a dark adventure area with unique challenges
pub fn generate_dark_adventure(level: &mut GameMap, rng: &mut ThreadRng, config: &LevelConfig) {
    // Create a modified config with higher density for dark adventure areas
    let mut dark_config = config.clone();
    dark_config.ore_density = (dark_config.ore_density * 1.3).min(0.9);
    dark_config.hazard_density = (dark_config.hazard_density * 1.5).min(0.8);
    dark_config.special_feature_chance = (dark_config.special_feature_chance * 1.5).min(0.7);
    dark_config.echo_chamber_chance = (dark_config.echo_chamber_chance * 2.0).min(0.8);

    // Start with a mix of mine and cave generation
    if rng.random_bool(0.5) {
        generate_mine(level, rng, &dark_config);

        // Replace mine tiles with appropriate dark adventure tiles
        for i in 0..level.map.len() {
            if level.map[i] == TileType::MineWall {
                level.map[i] = TileType::CaveWall;
            } else if level.map[i] == TileType::MineFloor {
                level.map[i] = TileType::CaveFloor;
            }
        }
    } else {
        generate_cave(level, rng, &dark_config);
    }

    // Add more special features and ancient machinery
    let num_ancient_machinery = rng.random_range(3..7);
    for _ in 0..num_ancient_machinery {
        let x = rng.random_range(1..level.width - 1);
        let y = rng.random_range(1..level.height - 1);

        if level.get_tile(x, y) == TileType::CaveFloor {
            level.set_tile(x, y, TileType::AncientMachinery);
        }
    }

    // Add more exotic ores in clusters
    let num_exotic_ores = rng.random_range(5..10);
    for _ in 0..num_exotic_ores {
        let x = rng.random_range(1..level.width - 1);
        let y = rng.random_range(1..level.height - 1);

        if !level.get_tile(x, y).is_walkable() {
            // Place a cluster of exotic ores
            place_ore_cluster(level, x, y, TileType::WhisperingIronOre, 2, rng);
        }
    }

    // Add echo crystals in dark adventure areas
    let num_echo_crystals = rng.random_range(2..5);
    for _ in 0..num_echo_crystals {
        let x = rng.random_range(1..level.width - 1);
        let y = rng.random_range(1..level.height - 1);

        if !level.get_tile(x, y).is_walkable() {
            level.set_tile(x, y, TileType::SonoriteOre);
        }
    }
}

/// Counts the number of wall tiles around a position
fn count_walls(level: &GameMap, x: i32, y: i32) -> i32 {
    let mut count = 0;

    for nx in -1..=1 {
        for ny in -1..=1 {
            if nx == 0 && ny == 0 {
                continue;
            }

            let tile = level.get_tile(x + nx, y + ny);
            if tile == TileType::CaveWall || tile == TileType::MineWall || tile == TileType::Wall {
                count += 1;
            }
        }
    }

    count
}

/// Adds ore veins to the level based on depth and config
fn add_ore_veins(level: &mut GameMap, rng: &mut ThreadRng, config: &LevelConfig) {
    let total_tiles = (level.width * level.height) as f32;
    let target_ore_tiles = (total_tiles * config.ore_density * 0.3) as i32; // 30% of density for ores

    // Determine ore distribution based on depth
    let ore_weights = match config.depth {
        d if d <= 5 => vec![(TileType::IronOre, 70), (TileType::CopperOre, 30)],
        d if d <= 10 => vec![
            (TileType::IronOre, 40),
            (TileType::CopperOre, 40),
            (TileType::SonoriteOre, 15),
            (TileType::GlimmerstoneOre, 5),
        ],
        _ => vec![
            (TileType::IronOre, 20),
            (TileType::CopperOre, 20),
            (TileType::SonoriteOre, 25),
            (TileType::GlimmerstoneOre, 15),
            (TileType::WhisperingIronOre, 20),
        ],
    };

    // Place ores in clusters
    let mut placed_ores = 0;
    while placed_ores < target_ore_tiles {
        let x = rng.random_range(1..level.width - 1);
        let y = rng.random_range(1..level.height - 1);

        // Only place ores in walls
        if !level.get_tile(x, y).is_walkable() {
            // Choose ore type based on weights
            let total_weight: i32 = ore_weights.iter().map(|&(_, w)| w).sum();
            let mut choice = rng.random_range(0..total_weight);
            let mut ore_type = TileType::IronOre; // Default fallback

            for &(tile, weight) in &ore_weights {
                choice -= weight;
                if choice <= 0 {
                    ore_type = tile;
                    break;
                }
            }

            // Place the ore and some surrounding ores
            let cluster_size = rng.random_range(1..=3);
            place_ore_cluster(level, x, y, ore_type, cluster_size, rng);
            placed_ores += cluster_size * 2; // Approximate count
        }
    }
}

/// Adds special features to the level
fn add_special_features(level: &mut GameMap, rng: &mut ThreadRng, config: &LevelConfig, rooms: &[Rect]) {
    let depth = config.depth;

    // Add ancient machinery in rooms
    if depth > 3 && rng.random_ratio((config.special_feature_chance * 100.0) as u32, 100) {
        if let Some(room) = rooms.choose(rng) {
            let (x, y) = room.random_point(rng);
            if level.get_tile(x, y).is_walkable() {
                level.set_tile(x, y, TileType::AncientMachinery);
            }
        }
    }

    // Add echo crystals in deeper levels
    if depth > 7 && rng.random_ratio((config.special_feature_chance * 50.0) as u32, 100) {
        let x = rng.random_range(1..level.width - 1);
        let y = rng.random_range(1..level.height - 1);

        if !level.get_tile(x, y).is_walkable() {
            level.set_tile(x, y, TileType::WhisperingIronOre);

            // Create a small cluster of echo crystals
            for dy in -1..=1 {
                for dx in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    let nx = x + dx;
                    let ny = y + dy;

                    if level.in_bounds(nx, ny) && !level.get_tile(nx, ny).is_walkable() {
                        level.set_tile(nx, ny, TileType::SonoriteOre);
                    }
                }
            }
        }
    }
}

/// Adds stairs to a cave level
fn add_stairs_to_cave(level: &mut GameMap, rng: &mut ThreadRng) {
    // Find a suitable location for stairs down
    let mut attempts = 0;
    let max_attempts = 100;

    while attempts < max_attempts {
        let x = rng.random_range(1..level.width - 1);
        let y = rng.random_range(1..level.height - 1);

        if level.get_tile(x, y) == TileType::CaveFloor {
            level.set_tile(x, y, TileType::StairsDown);
            break;
        }

        attempts += 1;
    }

    // Find a suitable location for stairs up (far from stairs down)
    attempts = 0;

    while attempts < max_attempts {
        let x = rng.random_range(1..level.width - 1);
        let y = rng.random_range(1..level.height - 1);

        if level.get_tile(x, y) == TileType::CaveFloor {
            level.set_tile(x, y, TileType::StairsUp);
            break;
        }

        attempts += 1;
    }
}

/// Adds special features to a cave level
fn add_special_features_to_cave(level: &mut GameMap, rng: &mut ThreadRng, config: &LevelConfig) {
    let depth = config.depth;
    // Add unstable walls that can collapse
    let unstable_wall_chance = depth.min(20);

    for y in 1..level.height - 1 {
        for x in 1..level.width - 1 {
            let idx = level.get_index(x, y);

            // Only replace wall tiles that aren't ores
            if level.map[idx] == TileType::CaveWall && !level.map[idx].is_ore() {
                let roll = rng.random_range(0..100);

                if roll < unstable_wall_chance {
                    level.map[idx] = TileType::UnstableWall;
                }
            }
        }
    }

    // Add gas pockets
    let num_gas_pockets = depth / 3;
    for _ in 0..num_gas_pockets {
        let x = rng.random_range(1..level.width - 1);
        let y = rng.random_range(1..level.height - 1);

        // Only place gas pockets on floor tiles
        if level.get_tile(x, y) == TileType::CaveFloor {
            level.set_tile(x, y, TileType::GasPocket);
        }
    }

    // Add ancient machinery
    let num_ancient_machinery = depth / 5;
    for _ in 0..num_ancient_machinery {
        let x = rng.random_range(1..level.width - 1);
        let y = rng.random_range(1..level.height - 1);

        // Only place ancient machinery on floor tiles
        if level.get_tile(x, y) == TileType::CaveFloor {
            level.set_tile(x, y, TileType::AncientMachinery);
        }
    }
}

/// A simple rectangle for room generation
#[derive(Clone, Copy, Debug)]
pub struct Rect {
    pub x1: i32,
    pub y1: i32,
    pub x2: i32,
    pub y2: i32,
}

impl Rect {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self { Self { x1: x, y1: y, x2: x + w, y2: y + h } }

    pub fn center(&self) -> (i32, i32) { ((self.x1 + self.x2) / 2, (self.y1 + self.y2) / 2) }

    pub fn intersect(&self, other: &Rect) -> bool {
        self.x1 <= other.x2 && self.x2 >= other.x1 && self.y1 <= other.y2 && self.y2 >= other.y1
    }

    pub fn random_point(&self, rng: &mut ThreadRng) -> (i32, i32) {
        let x = rng.random_range(self.x1 + 1..self.x2);
        let y = rng.random_range(self.y1 + 1..self.y2);
        (x, y)
    }
}

/// Applies a room to the map
fn apply_room_to_map(level: &mut GameMap, room: &Rect, tile_type: TileType) {
    for y in room.y1 + 1..room.y2 {
        for x in room.x1 + 1..room.x2 {
            level.set_tile(x, y, tile_type);
        }
    }
}

/// Applies a horizontal tunnel
fn apply_horizontal_tunnel(level: &mut GameMap, x1: i32, x2: i32, y: i32, tile_type: TileType) {
    for x in x1.min(x2)..=x1.max(x2) {
        level.set_tile(x, y, tile_type);
    }
}

/// Applies a vertical tunnel
fn apply_vertical_tunnel(level: &mut GameMap, y1: i32, y2: i32, x: i32, tile_type: TileType) {
    for y in y1.min(y2)..=y1.max(y2) {
        level.set_tile(x, y, tile_type);
    }
}
