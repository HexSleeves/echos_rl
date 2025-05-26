use bevy::prelude::*;

use crate::model::{
    components::{Position, TerrainType},
    resources::CurrentMap,
    systems::{ActorMovedEvent, MoveActorEvent},
};

/// Example system showing common map operations for a roguelike
pub fn example_map_usage(
    mut commands: Commands,
    mut current_map: ResMut<CurrentMap>,
    mut move_events: EventWriter<MoveActorEvent>,
) {
    // Example 1: Check if a position is walkable
    let player_pos = Position::new(5, 5);
    if current_map.is_walkable(player_pos) {
        info!("Position {:?} is walkable", player_pos);
    }

    // Example 2: Find all walkable neighbors for pathfinding
    let walkable_neighbors = current_map.get_walkable_neighbors(player_pos);
    info!("Walkable neighbors: {:?}", walkable_neighbors);

    // Example 3: Place an actor safely
    let new_actor = commands
        .spawn((
            Position::new(10, 10),
            // Add other components like Actor, Health, etc.
        ))
        .id();

    match current_map.place_actor(Position::new(10, 10), new_actor) {
        Ok(()) => info!("Actor placed successfully"),
        Err(e) => warn!("Failed to place actor: {}", e),
    }

    // Example 4: Move an actor using the event system
    move_events.write(MoveActorEvent {
        actor: new_actor,
        direction: Position::new(1, 0), // Move right
    });

    // Example 5: Find actors in a radius (for AI, spells, etc.)
    let nearby_actors = current_map.get_actors_in_radius(player_pos, 3);
    for (pos, entity) in nearby_actors {
        info!("Actor {:?} found at {:?}", entity, pos);
    }

    // Example 6: Update visibility (for fog of war)
    current_map.clear_visibility();
    for neighbor in current_map.get_neighbors(player_pos) {
        current_map.set_visible(neighbor, true);
    }

    // Example 7: Check exploration status
    if current_map.is_explored(player_pos) {
        info!("Player has been here before");
    }

    // Example 8: Modify terrain (for destructible walls, doors, etc.)
    current_map.set_terrain(Position::new(15, 15), TerrainType::Floor);

    // Example 9: Get reverse lookup - find where an actor is
    if let Some(actor_position) = current_map.get_actor_position(new_actor) {
        info!("Actor is at position {:?}", actor_position);
    }
}

/// Example pathfinding using the map's walkable neighbors
pub fn simple_pathfinding_example(
    current_map: Res<CurrentMap>,
    start: Position,
    goal: Position,
) -> Option<Vec<Position>> {
    // Simple breadth-first search using the map's walkable neighbors
    use std::collections::{HashMap, VecDeque};

    let mut queue = VecDeque::new();
    let mut came_from = HashMap::new();
    let mut visited = std::collections::HashSet::new();

    queue.push_back(start);
    visited.insert(start);

    while let Some(current) = queue.pop_front() {
        if current == goal {
            // Reconstruct path
            let mut path = Vec::new();
            let mut current = goal;

            while current != start {
                path.push(current);
                current = came_from[&current];
            }
            path.push(start);
            path.reverse();
            return Some(path);
        }

        for neighbor in current_map.get_walkable_neighbors(current) {
            if !visited.contains(&neighbor) {
                visited.insert(neighbor);
                came_from.insert(neighbor, current);
                queue.push_back(neighbor);
            }
        }
    }

    None // No path found
}
