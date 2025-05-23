use bevy::prelude::*;
use rand::prelude::*;

use super::{gamemap::GameMap, tile::TileType};

/// Generates a mine level with rooms and corridors
pub fn generate_mine(level: &mut GameMap, rng: &mut ThreadRng, complexity: f32, depth: i32) {
    // Clear the map first
    for i in 0..level.map.len() {
        level.map[i] = TileType::MineWall;
    }

    // Determine number of rooms based on complexity
    let num_rooms = (5.0 + complexity * 10.0) as i32;
    let min_room_size = 3;
    let max_room_size = 8;

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

    // Add ore veins based on depth
    add_ore_veins(level, rng, depth);

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
    add_special_features(level, rng, depth, &rooms);
}

/// Generates a cave level using cellular automata
pub fn generate_cave(level: &mut GameMap, rng: &mut ThreadRng, complexity: f32, depth: i32) {
    // Initialize with random walls and floors
    let fill_percent = 55 - (complexity * 10.0) as i32; // Higher complexity = more open caves

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

    // Add ore veins based on depth
    add_ore_veins(level, rng, depth);

    // Add stairs
    add_stairs_to_cave(level, rng);

    // Add special features
    add_special_features_to_cave(level, rng, depth);
}

/// Generates a dark adventure area with unique challenges
pub fn generate_dark_adventure(level: &mut GameMap, rng: &mut ThreadRng, depth: i32) {
    // Start with a mix of mine and cave generation
    if rng.random_bool(0.5) {
        generate_mine(level, rng, 0.7, depth);

        // Replace mine tiles with appropriate dark adventure tiles
        for i in 0..level.map.len() {
            if level.map[i] == TileType::MineWall {
                level.map[i] = TileType::CaveWall;
            } else if level.map[i] == TileType::MineFloor {
                level.map[i] = TileType::CaveFloor;
            }
        }
    } else {
        generate_cave(level, rng, 0.7, depth);
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

    // Add more exotic ores
    let num_exotic_ores = rng.random_range(5..10);
    for _ in 0..num_exotic_ores {
        let x = rng.random_range(1..level.width - 1);
        let y = rng.random_range(1..level.height - 1);

        if level.get_tile(x, y) == TileType::CaveWall {
            level.set_tile(x, y, TileType::WhisperingIronOre);
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

/// Adds ore veins to the level based on depth
fn add_ore_veins(level: &mut GameMap, rng: &mut ThreadRng, depth: i32) {
    // Basic ores (iron, copper) are more common at shallow depths
    let iron_chance = 15 - depth.min(10);
    let copper_chance = 10 - depth.min(8);

    // Special ores become more common with depth
    let sonorite_chance = depth.min(15);
    let glimmerstone_chance = depth.min(12);
    let whispering_iron_chance = if depth > 10 { depth - 10 } else { 0 };

    for y in 1..level.height - 1 {
        for x in 1..level.width - 1 {
            let idx = level.get_index(x, y);

            // Only replace wall tiles
            if level.map[idx] == TileType::MineWall || level.map[idx] == TileType::CaveWall {
                let roll = rng.random_range(0..100);

                if roll < iron_chance {
                    level.map[idx] = TileType::IronOre;
                } else if roll < iron_chance + copper_chance {
                    level.map[idx] = TileType::CopperOre;
                } else if roll < iron_chance + copper_chance + sonorite_chance {
                    level.map[idx] = TileType::SonoriteOre;
                } else if roll < iron_chance + copper_chance + sonorite_chance + glimmerstone_chance {
                    level.map[idx] = TileType::GlimmerstoneOre;
                } else if roll
                    < iron_chance + copper_chance + sonorite_chance + glimmerstone_chance + whispering_iron_chance
                {
                    level.map[idx] = TileType::WhisperingIronOre;
                }
            }
        }
    }
}

/// Adds special features to the level
fn add_special_features(level: &mut GameMap, rng: &mut ThreadRng, depth: i32, rooms: &[Rect]) {
    // Add unstable walls that can collapse
    let unstable_wall_chance = depth.min(20);

    for y in 1..level.height - 1 {
        for x in 1..level.width - 1 {
            let idx = level.get_index(x, y);

            // Only replace wall tiles that aren't ores
            if (level.map[idx] == TileType::MineWall || level.map[idx] == TileType::CaveWall)
                && !level.map[idx].is_ore()
            {
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
        if let Some(room) = rooms.choose(&mut *rng) {
            let (x, y) = room.random_point(rng);

            // Only place gas pockets on floor tiles
            if level.get_tile(x, y) == TileType::MineFloor {
                level.set_tile(x, y, TileType::GasPocket);
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
fn add_special_features_to_cave(level: &mut GameMap, rng: &mut ThreadRng, depth: i32) {
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
