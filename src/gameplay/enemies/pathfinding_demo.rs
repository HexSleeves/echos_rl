use bevy::prelude::*;

use crate::{
    core::{
        components::Position,
        pathfinding::{Pathfinder, PathfindingAlgorithm},
        resources::CurrentMap,
    },
    gameplay::enemies::{
        components::AIBehaviorType,
        enhanced_spawning::{spawn_enhanced_ai_entity, AIPathfindingConfig},
    },
};

/// Demo system to showcase enhanced pathfinding capabilities
pub fn pathfinding_demo_system(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    current_map: Res<CurrentMap>,
    mut config: ResMut<AIPathfindingConfig>,
) {
    // Toggle pathfinding algorithm with 'P' key
    if keyboard_input.just_pressed(KeyCode::KeyP) {
        config.default_algorithm = match config.default_algorithm {
            PathfindingAlgorithm::AStar => {
                info!("Switched to Dijkstra pathfinding algorithm");
                PathfindingAlgorithm::Dijkstra
            }
            PathfindingAlgorithm::Dijkstra => {
                info!("Switched to A* pathfinding algorithm");
                PathfindingAlgorithm::AStar
            }
        };
    }

    // Toggle diagonal movement with 'D' key
    if keyboard_input.just_pressed(KeyCode::KeyD) {
        config.allow_diagonal = !config.allow_diagonal;
        info!("Diagonal movement: {}", if config.allow_diagonal { "enabled" } else { "disabled" });
    }

    // Spawn test AI entities with 'T' key
    if keyboard_input.just_pressed(KeyCode::KeyT) {
        spawn_test_ai_entities(&mut commands, &current_map);
    }

    // Run pathfinding benchmark with 'B' key
    if keyboard_input.just_pressed(KeyCode::KeyB) {
        run_pathfinding_benchmark(&current_map);
    }
}

/// Spawn test AI entities to demonstrate pathfinding
fn spawn_test_ai_entities(commands: &mut Commands, map: &CurrentMap) {
    let test_positions = vec![
        Position::new(10, 10),
        Position::new(15, 15),
        Position::new(20, 20),
        Position::new(25, 25),
    ];

    let behavior_types = vec![
        AIBehaviorType::Hostile,
        AIBehaviorType::Passive,
        AIBehaviorType::Neutral,
        AIBehaviorType::Hostile,
    ];

    for (i, (&position, &behavior_type)) in test_positions.iter().zip(behavior_types.iter()).enumerate() {
        if map.is_walkable(position) && map.get_actor(position).is_none() {
            let entity = spawn_enhanced_ai_entity(
                commands,
                position,
                behavior_type,
                8, // detection range
                100, // speed
                Some(format!("TestAI_{}", i + 1)),
            );
            
            info!("Spawned test AI entity {:?} at {:?} with behavior {:?}", 
                  entity, position, behavior_type);
        } else {
            warn!("Cannot spawn AI at {:?} - position not walkable or occupied", position);
        }
    }
}

/// Run a pathfinding benchmark to compare algorithms
fn run_pathfinding_benchmark(map: &CurrentMap) {
    let start = Position::new(5, 5);
    let goal = Position::new(45, 45);
    
    if !map.is_walkable(start) || !map.is_walkable(goal) {
        warn!("Benchmark positions are not walkable");
        return;
    }

    info!("Running pathfinding benchmark from {:?} to {:?}", start, goal);

    // Benchmark A*
    let astar_pathfinder = Pathfinder::new(PathfindingAlgorithm::AStar)
        .with_max_iterations(10000)
        .with_diagonal(false);

    let start_time = std::time::Instant::now();
    let astar_result = astar_pathfinder.find_path(start, goal, map);
    let astar_duration = start_time.elapsed();

    // Benchmark Dijkstra
    let dijkstra_pathfinder = Pathfinder::new(PathfindingAlgorithm::Dijkstra)
        .with_max_iterations(10000)
        .with_diagonal(false);

    let start_time = std::time::Instant::now();
    let dijkstra_result = dijkstra_pathfinder.find_path(start, goal, map);
    let dijkstra_duration = start_time.elapsed();

    // Report results
    info!("=== Pathfinding Benchmark Results ===");
    info!("A* Algorithm:");
    info!("  - Path length: {} steps", astar_result.path.len());
    info!("  - Path cost: {}", astar_result.cost);
    info!("  - Nodes explored: {}", astar_result.nodes_explored);
    info!("  - Time taken: {:?}", astar_duration);
    
    info!("Dijkstra Algorithm:");
    info!("  - Path length: {} steps", dijkstra_result.path.len());
    info!("  - Path cost: {}", dijkstra_result.cost);
    info!("  - Nodes explored: {}", dijkstra_result.nodes_explored);
    info!("  - Time taken: {:?}", dijkstra_duration);

    if !astar_result.is_empty() && !dijkstra_result.is_empty() {
        let speed_ratio = dijkstra_duration.as_nanos() as f64 / astar_duration.as_nanos() as f64;
        info!("A* is {:.2}x faster than Dijkstra", speed_ratio);
    }

    // Test pathfinding to nearest
    info!("=== Testing pathfinding to nearest position ===");
    let blocked_goal = Position::new(1, 1); // Likely blocked by walls
    
    let start_time = std::time::Instant::now();
    let nearest_result = astar_pathfinder.find_path_to_nearest(start, blocked_goal, map, 5);
    let nearest_duration = start_time.elapsed();

    if !nearest_result.is_empty() {
        info!("Found path to nearest position:");
        info!("  - Target was: {:?}", blocked_goal);
        info!("  - Reached: {:?}", nearest_result.path.last());
        info!("  - Path length: {} steps", nearest_result.path.len());
        info!("  - Time taken: {:?}", nearest_duration);
    } else {
        info!("Could not find path to any nearby position");
    }
}

/// System to visualize pathfinding debug information
pub fn pathfinding_debug_system(
    ai_query: Query<(Entity, &Position, &crate::gameplay::enemies::pathfinding::AIPathfinding), With<crate::core::components::AITag>>,
    mut gizmos: Gizmos,
) {
    for (entity, position, pathfinding) in ai_query.iter() {
        // Draw current path
        if !pathfinding.current_path.is_empty() {
            let path_color = Color::srgb(0.0, 1.0, 0.0); // Green for path
            
            for i in 0..pathfinding.current_path.len() - 1 {
                let from = pathfinding.current_path[i];
                let to = pathfinding.current_path[i + 1];
                
                gizmos.line_2d(
                    Vec2::new(from.x() as f32 * 32.0, from.y() as f32 * 32.0),
                    Vec2::new(to.x() as f32 * 32.0, to.y() as f32 * 32.0),
                    path_color,
                );
            }

            // Draw target position
            if let Some(target) = pathfinding.target_position {
                gizmos.circle_2d(
                    Vec2::new(target.x() as f32 * 32.0, target.y() as f32 * 32.0),
                    16.0,
                    Color::srgb(1.0, 0.0, 0.0), // Red for target
                );
            }

            // Draw current position
            gizmos.circle_2d(
                Vec2::new(position.x() as f32 * 32.0, position.y() as f32 * 32.0),
                12.0,
                Color::srgb(0.0, 0.0, 1.0), // Blue for current position
            );
        }
    }
}

/// Resource for pathfinding demo configuration
#[derive(Resource, Default)]
pub struct PathfindingDemoConfig {
    pub show_debug_visualization: bool,
    pub auto_spawn_test_entities: bool,
    pub benchmark_interval: f32,
    pub last_benchmark: f32,
}

/// System to handle demo configuration
pub fn pathfinding_demo_config_system(
    mut config: ResMut<PathfindingDemoConfig>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    // Toggle debug visualization with 'V' key
    if keyboard_input.just_pressed(KeyCode::KeyV) {
        config.show_debug_visualization = !config.show_debug_visualization;
        info!("Pathfinding debug visualization: {}", 
              if config.show_debug_visualization { "enabled" } else { "disabled" });
    }

    // Auto-benchmark every 30 seconds if enabled
    if config.benchmark_interval > 0.0 {
        config.last_benchmark += time.delta_secs();
        if config.last_benchmark >= config.benchmark_interval {
            config.last_benchmark = 0.0;
            // Trigger benchmark (would need map access here)
            info!("Auto-benchmark triggered (every {} seconds)", config.benchmark_interval);
        }
    }
}

/// Plugin for pathfinding demo functionality
pub struct PathfindingDemoPlugin;

impl Plugin for PathfindingDemoPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PathfindingDemoConfig>()
            .add_systems(
                Update,
                (
                    pathfinding_demo_system,
                    pathfinding_demo_config_system,
                    pathfinding_debug_system.run_if(|config: Res<PathfindingDemoConfig>| {
                        config.show_debug_visualization
                    }),
                ),
            );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_demo_config() {
        let config = PathfindingDemoConfig::default();
        assert!(!config.show_debug_visualization);
        assert!(!config.auto_spawn_test_entities);
        assert_eq!(config.benchmark_interval, 0.0);
    }
}
