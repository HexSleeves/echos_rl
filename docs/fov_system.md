# Field of View (FOV) System Documentation

## Overview

The echos_rl project now features a comprehensive, trait-based Field of View system that provides advanced shadowcasting algorithms for precise line-of-sight calculations in roguelike games. This implementation is based on Albert Ford's shadowcasting algorithm and provides significant improvements over traditional approaches.

## Architecture

### Core Components

The FOV system is built around three main traits in the `brtk` crate:

1. **`FovProvider`** - Abstracts map/world representation for opacity queries
2. **`FovReceiver`** - Handles visibility information storage and retrieval
3. **`FovAlgorithm`** - Defines the interface for FOV calculation algorithms

### Key Features

- **Trait-based design** for maximum flexibility and reusability
- **Advanced shadowcasting** with precise rational slope calculations
- **Multiple algorithm support** (raycasting, traditional shadowcasting, advanced shadowcasting)
- **Directional FOV** support for cone-based vision
- **Memory-efficient** bit-level storage for visibility data
- **Performance optimized** with caching and efficient algorithms
- **Entity-to-entity visibility** utility functions for AI systems

## Entity Visibility Utilities

### New Utility Functions

The `FovMap` now provides utility functions for checking entity-to-entity visibility:

#### `can_see_position(observer_pos, target_pos, range, map) -> bool`

Checks if an observer at one position can see a target position within a given range.

```rust
use crate::core::resources::FovMap;

let observer_pos = Position::new(5, 5);
let target_pos = Position::new(7, 6);
let range = 8;

if FovMap::can_see_position(observer_pos, target_pos, range, &map) {
    println!("Observer can see the target!");
}
```

#### `can_see_entity(observer_pos, observer_range, target_pos, map) -> bool`

Semantic wrapper for entity-to-entity visibility checks.

```rust
// AI checking if it can see the player
if FovMap::can_see_entity(*ai_pos, ai_behavior.detection_range, player_pos, &current_map) {
    // AI can see player, start chasing!
    ai_behavior.update_player_sighting(player_pos, current_turn);
}
```

#### `compute_temporary_fov(observer_pos, range, map) -> VisibilityMap`

Computes a temporary FOV for multiple visibility queries from the same observer.

```rust
let visibility_map = FovMap::compute_temporary_fov(observer_pos, range, &map);

// Check multiple targets efficiently
for target_pos in potential_targets {
    if visibility_map.get_visible((target_pos.x(), target_pos.y())) {
        // Target is visible
    }
}
```

### AI Integration

The AI systems now use these utility functions to properly check their own field of view:

```rust
// Before: Incorrectly checking player's FOV
if fov_map.is_visible(*player_pos) { ... }

// After: Correctly checking AI's FOV
if FovMap::can_see_entity(*ai_pos, ai_behavior.detection_range, *player_pos, &current_map) { ... }
```

This fixes the fundamental issue where AI entities were checking if the player position was visible in the player's FOV, rather than checking if the AI could see the player from its own position.

## Implementation Details

### brtk Crate Structure

```
crates/brtk/src/fov/
├── mod.rs                 # Main module exports and FovAlgorithmType enum
├── traits.rs              # Core traits (FovProvider, FovReceiver, FovAlgorithm)
├── algorithms/
│   ├── mod.rs
│   ├── shadowcast.rs      # Advanced shadowcasting implementation
│   ├── quadrant.rs        # Quadrant/octant coordinate transformations
│   └── row.rs             # Row iteration for shadowcasting
├── utils/
│   ├── mod.rs
│   ├── slope.rs           # Rational number slope calculations
│   └── distance.rs        # Distance calculation algorithms
└── implementations/
    ├── mod.rs
    ├── visibility_map.rs  # HashSet-based visibility storage
    └── map_provider.rs    # Generic map provider wrapper
```

### Game Integration

```
src/core/resources/
└── fov_map.rs            # Main FovMap resource with utility functions

src/core/systems.rs      # Player FOV computation system
src/gameplay/enemies/systems/
├── chase.rs              # AI chase behavior using entity visibility
└── flee.rs               # AI flee behavior using entity visibility
```

## Performance Considerations

### Utility Function Performance

The new utility functions compute FOV on-demand using temporary storage:

- **Memory**: Uses `VisibilityMap` (HashSet-based) for temporary storage
- **CPU**: Computes full FOV for each query (optimizable with caching)
- **Scalability**: Suitable for current AI entity counts, can be optimized later

### Optimization Opportunities

1. **FOV Caching**: Cache FOV results per entity per turn
2. **Incremental Updates**: Only recompute FOV when entity moves
3. **Spatial Partitioning**: Group nearby entities for batch FOV computation
4. **Range Culling**: Skip FOV computation for entities beyond interaction range

## Testing

The system includes comprehensive tests covering:

- Basic visibility checks
- Range limitations
- Wall occlusion
- Utility function correctness

Run tests with:

```bash
cargo test fov_map::tests
```

## Usage Examples

### AI Enemy Detection

```rust
// In AI scorer system
if ai_behavior.behavior_type == AIBehaviorType::Hostile {
    if FovMap::can_see_entity(*ai_pos, ai_behavior.detection_range, *player_pos, &current_map) {
        let distance = crate::utils::calculate_distance(*ai_pos, *player_pos);
        if distance <= ai_behavior.detection_range as f32 {
            ai_behavior.update_player_sighting(*player_pos, current_turn);
            chase_score = 1.0;
        }
    }
}
```

### Multi-Target Visibility

```rust
// Check visibility to multiple targets efficiently
let visibility_map = FovMap::compute_temporary_fov(observer_pos, range, &map);

for enemy in enemies.iter() {
    if visibility_map.get_visible((enemy.pos.x(), enemy.pos.y())) {
        // Enemy is visible, add to target list
        visible_enemies.push(enemy);
    }
}
```

## Future Enhancements

1. **Component-based FOV**: Give each entity its own FOV component
2. **Directional Vision**: Implement cone-based vision for more realistic AI
3. **Vision Types**: Support different vision types (normal, infrared, magical)
4. **Performance Optimization**: Implement caching and incremental updates
5. **Advanced AI**: Use FOV for more sophisticated AI behaviors

## Migration Guide

### Updating AI Systems

If you have existing AI systems that check visibility:

1. **Replace global FOV checks**:

   ```rust
   // Old
   if fov_map.is_visible(target_pos) { ... }

   // New
   if FovMap::can_see_entity(observer_pos, range, target_pos, &map) { ... }
   ```

2. **Add CurrentMap resource** to system parameters if not already present

3. **Update function signatures** to include the map parameter

### Performance Considerations

- The new utility functions compute FOV on-demand
- For high-frequency visibility checks, consider using `compute_temporary_fov`
- Monitor performance and implement caching if needed

## Conclusion

The enhanced FOV system provides a solid foundation for entity-to-entity visibility checks while maintaining the existing player FOV functionality. The utility functions offer a clean API for AI systems and can be extended for future gameplay features.
