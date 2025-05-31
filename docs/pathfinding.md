# Enhanced Pathfinding System

The enhanced pathfinding system in echos_rl provides sophisticated AI navigation capabilities inspired by the atrl project but built with modern Bevy patterns and improved performance.

## Features

### Core Pathfinding Algorithms

- **A* Algorithm**: Optimal pathfinding with heuristic guidance
- **Dijkstra's Algorithm**: Guaranteed shortest path, useful for multiple targets
- **Partial Pathfinding**: Find paths to nearby positions when direct paths fail
- **Multi-target Pathfinding**: Find paths to any of multiple goal positions

### Performance Optimizations

- **Path Caching**: Automatic caching of computed paths with configurable TTL
- **Iterative Limits**: Configurable maximum iterations to prevent infinite loops
- **Periodic Cleanup**: Automatic removal of expired cache entries
- **Memory Management**: Efficient data structures with size limits

### AI Integration

- **Enhanced AI Components**: `AIPathfinding` component for sophisticated navigation
- **Behavior-Specific Algorithms**: Different algorithms for different AI types
- **Path Validation**: Automatic path invalidation when obstacles change
- **Recalculation Strategies**: Smart path recalculation based on game state

## Usage

### Basic Pathfinding

```rust
use crate::core::pathfinding::{Pathfinder, PathfindingAlgorithm};

// Create a pathfinder
let pathfinder = Pathfinder::new(PathfindingAlgorithm::AStar)
    .with_max_iterations(1000)
    .with_diagonal(false);

// Find a path
let result = pathfinder.find_path(start_pos, goal_pos, &map);

if !result.is_empty() {
    println!("Found path with {} steps", result.path.len());
    for step in result.path {
        println!("Step: {:?}", step);
    }
}
```

### Enhanced AI Entities

```rust
use crate::gameplay::enemies::enhanced_spawning::*;

// Spawn an AI entity with pathfinding
let entity = spawn_enhanced_ai_entity(
    &mut commands,
    Position::new(10, 10),
    AIBehaviorType::Hostile,
    8, // detection range
    100, // speed
    Some("Guard".to_string()),
);

// Or use the bundle directly
commands.spawn(EnhancedAIBundle::hostile(
    Position::new(15, 15),
    6, // detection range
    120, // speed
));
```

### Path Caching

```rust
use crate::core::pathfinding::path_cache::*;

// Create a cached pathfinder
let mut cached_pathfinder = CachedPathfinder::new(
    Pathfinder::new(PathfindingAlgorithm::AStar)
).with_cache_config(
    1000, // max entries
    Duration::from_secs(30), // max age
);

// Use it like a regular pathfinder
let result = cached_pathfinder.find_path(start, goal, &map);

// Check cache statistics
let stats = cached_pathfinder.cache_stats();
println!("Cache hit rate: {:.2}%", stats.hit_rate * 100.0);
```

## Configuration

### Global Configuration

```rust
use crate::gameplay::enemies::enhanced_spawning::AIPathfindingConfig;

// Configure pathfinding globally
app.insert_resource(AIPathfindingConfig {
    default_algorithm: PathfindingAlgorithm::AStar,
    max_iterations: 1000,
    allow_diagonal: false,
    recalculation_threshold: 5,
    enable_path_caching: true,
});
```

### Per-Entity Configuration

```rust
// Customize pathfinding for specific entities
let custom_ai = EnhancedAIBundle::hostile(position, 8, 100)
    .with_pathfinding_config(
        PathfindingAlgorithm::Dijkstra,
        true, // allow diagonal
        2000, // max iterations
    );
```

## Algorithm Comparison

| Algorithm | Best For | Performance | Memory Usage |
|-----------|----------|-------------|--------------|
| A* | Single target, known goal | Fast | Low |
| Dijkstra | Multiple targets, exploration | Slower | Higher |

### When to Use A*

- Chasing a specific target (player)
- Direct navigation to known positions
- Performance-critical scenarios

### When to Use Dijkstra

- Exploring multiple options
- Finding nearest of several targets
- When heuristics are unreliable

## Demo and Testing

### Interactive Demo

The pathfinding demo system provides interactive testing:

- **P**: Toggle between A* and Dijkstra algorithms
- **D**: Toggle diagonal movement
- **T**: Spawn test AI entities
- **B**: Run pathfinding benchmark
- **V**: Toggle debug visualization

### Benchmarking

```rust
// Run performance benchmarks
run_pathfinding_benchmark(&map);
```

This will compare A* vs Dijkstra performance and provide detailed metrics.

## Advanced Features

### Path Invalidation

```rust
// Invalidate paths when map changes
path_cache.invalidate_paths_through(changed_position);
path_cache.invalidate_paths_involving(entity_position);
```

### Custom Pathfinding

```rust
// Implement custom pathfinding behavior
impl AIPathfinding {
    pub fn custom_pathfinding(&mut self, start: Position, target: Position, map: &CurrentMap) -> bool {
        // Custom logic here
        self.calculate_path(start, target, map, current_turn)
    }
}
```

### Multi-Target Pathfinding

```rust
use crate::core::pathfinding::dijkstra::find_path_to_any;

let goals = vec![pos1, pos2, pos3];
let result = find_path_to_any(start, &goals, &map, 1000, false);
```

## Performance Considerations

### Memory Usage

- Path cache: ~1000 entries by default
- Each cached path: ~100-500 bytes
- Total memory: ~100-500 KB

### CPU Usage

- A* pathfinding: ~0.1-1ms per path
- Cache lookup: ~0.001ms
- Path validation: ~0.01ms

### Optimization Tips

1. **Use caching** for frequently requested paths
2. **Limit iterations** to prevent performance spikes
3. **Choose appropriate algorithms** for different scenarios
4. **Invalidate caches** when map changes
5. **Use partial pathfinding** for distant targets

## Integration with Existing Systems

The enhanced pathfinding system is designed to work alongside existing AI systems:

- **Backward Compatible**: Existing AI entities are automatically upgraded
- **Configurable**: Can be enabled/disabled per entity
- **Performance Aware**: Minimal impact on existing systems

## Troubleshooting

### Common Issues

1. **No path found**: Check if start/goal positions are walkable
2. **Performance issues**: Reduce max_iterations or enable caching
3. **Memory usage**: Reduce cache size or TTL
4. **Incorrect paths**: Verify map data and walkability checks

### Debug Information

Enable debug visualization to see:
- Current AI paths (green lines)
- Target positions (red circles)
- Current positions (blue circles)
- Path recalculation events

## Future Enhancements

Planned improvements include:

- **Hierarchical Pathfinding**: For large maps
- **Dynamic Obstacles**: Real-time obstacle avoidance
- **Group Pathfinding**: Coordinated movement for multiple entities
- **Path Smoothing**: More natural movement curves
- **Async Pathfinding**: Background path computation
