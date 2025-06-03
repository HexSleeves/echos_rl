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
    ├── visibility_map.rs  # HashSet-based FovReceiver implementation
    └── map_provider.rs    # Generic map provider adapters
```

### Algorithm Comparison

| Algorithm | Complexity | Accuracy | Performance | Use Case |
|-----------|------------|----------|-------------|----------|
| Raycasting | O(r²) | High | Good | Reliable wall blocking |
| Traditional Shadowcasting | O(r²) | Good | Excellent | Large view distances |
| Advanced Shadowcasting | O(r²) | Excellent | Excellent | Artifact-free precision |

### Advanced Shadowcasting Features

The new advanced shadowcasting algorithm provides:

- **Precise slope calculations** using rational numbers to avoid floating-point errors
- **Symmetric visibility** ensuring consistent results
- **Artifact-free rendering** with proper shadow casting
- **Octant-based scanning** for complete 360° coverage
- **Efficient recursion** with proper shadow boundary handling

## Integration with echos_rl

### FovMap Enhancement

The existing `FovMap` resource has been enhanced with:

- New `AdvancedShadowcasting` algorithm option (now default)
- Adapter classes (`EchosMapProvider`, `EchosFovReceiver`) for seamless integration
- Backward compatibility with existing API
- Enhanced algorithm toggling (F key cycles through all three algorithms)

### Usage Example

```rust
// Create FOV map with advanced shadowcasting (default)
let mut fov_map = FovMap::new(width, height);

// Compute FOV for player
fov_map.compute_fov(&map, player_position, view_radius);

// Check visibility
if fov_map.is_visible(some_position) {
    // Tile is visible to player
}

// Toggle algorithms at runtime
fov_map.set_algorithm(FovAlgorithm::AdvancedShadowcasting);
```

## Performance Characteristics

### Memory Usage

- **BitVec storage** for visibility flags (1 bit per tile)
- **HashSet-based** visibility maps for flexible implementations
- **Ray caching** for raycasting algorithm optimization

### Computational Complexity

- **O(r²)** time complexity for all algorithms where r is the vision radius
- **Constant memory** overhead regardless of map size
- **Cache-friendly** access patterns for better performance

## Testing

The implementation includes comprehensive tests:

- **Algorithm consistency** tests comparing all three algorithms
- **Edge case handling** for boundaries and obstacles
- **Performance benchmarks** for algorithm comparison
- **Integration tests** with the existing game systems

### Running Tests

```bash
# Test the brtk FOV system
cargo test fov -p brtk

# Test integration with echos_rl
cargo test fov_map

# Run performance benchmarks
cargo test --ignored test_performance_comparison
```

## Future Enhancements

### Planned Features

1. **Vision Types** - Support for different vision modes (infrared, magical, etc.)
2. **Asymmetric FOV** - Support for directional lighting and vision cones
3. **Multi-level FOV** - Support for 3D environments with multiple floors
4. **Dynamic Lighting** - Integration with lighting systems for realistic shadows

### Extension Points

The trait-based design makes it easy to:

- Add new FOV algorithms by implementing `FovAlgorithm`
- Create custom map providers for different game types
- Implement specialized visibility storage systems
- Add vision modifiers and effects

## Migration Guide

### From Previous Implementation

The new system is fully backward compatible. Existing code will continue to work unchanged, but you can opt into the new advanced shadowcasting by:

```rust
// Enable advanced shadowcasting (now default)
fov_map.set_algorithm(FovAlgorithm::AdvancedShadowcasting);
```

### Best Practices

1. **Use AdvancedShadowcasting** for the best visual quality
2. **Cache FOV results** when possible to avoid redundant calculations
3. **Consider vision radius** impact on performance for large values
4. **Test algorithm behavior** with your specific map layouts

## Technical References

- [Albert Ford's Shadowcasting Algorithm](https://www.albertford.com/shadowcasting/)
- [Roguelike FOV Algorithms](http://www.roguebasin.com/index.php?title=Field_of_Vision)
- [Symmetric Shadowcasting](http://www.roguebasin.com/index.php?title=Symmetric_Shadowcasting)

## Conclusion

The new FOV system provides a robust, flexible, and high-performance foundation for line-of-sight calculations in echos_rl. The trait-based architecture ensures extensibility while the advanced shadowcasting algorithm delivers artifact-free, precise visibility calculations that enhance the player experience.
