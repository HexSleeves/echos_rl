# Technical Specifications: Data-Driven Entity System

## Technology Stack
- **Game Engine**: Bevy 0.16
- **Language**: Rust (2024 edition)
- **Data Format**: RON (Rust Object Notation)
- **Asset Loading**: bevy_asset_loader 0.23.0
- **Serialization**: serde 1.x with derive features
- **Additional Dependencies**:
  - bevy_ecs_tilemap 0.16 (current tilemap system)
  - brtk (custom library for game utilities)

## Current Architecture Analysis

### Existing Entity System
```rust
// Current hardcoded spawning in src/model/systems/spawner.rs
pub fn spawn_player(
    mut commands: Commands,
    // ... parameters
) {
    let player_id = commands
        .spawn((
            player_position,
            PlayerTag,
            AwaitingInput,
            TurnActor::new(100),
            ViewShed { radius: 8 },
            TileSprite {
                tile_coords: (10, 18),
                tile_size: Vec2::splat(ViewConstants::TILE_SIZE),
                ..Default::default()
            },
        ))
        .id();
}
```

### Current Components
- `Position`: Entity location in world
- `PlayerTag`: Marker for player entity
- `AITag`: Marker for AI entities
- `TurnActor`: Turn-based action system with speed/timing
- `ViewShed`: Vision/FOV with radius
- `TileSprite`: Rendering data (coordinates, size)
- `AwaitingInput`: Player input state marker

## Proposed Architecture

### 1. Entity Definition System
```rust
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EntityDefinition {
    pub name: String,
    pub description: Option<String>,
    pub components: EntityComponents,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EntityComponents {
    // Core components
    pub turn_actor: Option<TurnActorData>,
    pub view_shed: Option<ViewShedData>,
    pub tile_sprite: Option<TileSpriteData>,

    // Tags
    pub is_player: Option<bool>,
    pub is_ai: Option<bool>,

    // Additional properties
    pub spawn_weight: Option<f32>,
    pub level_range: Option<(u32, u32)>,
}
```

### 2. Asset Loading Integration
- Leverage existing `bevy_asset_loader` system
- Use `StandardDynamicAssets` for RON file loading
- Integrate with current texture asset pipeline

### 3. Spawning System Refactor
```rust
pub fn spawn_entity_from_definition(
    commands: &mut Commands,
    definition: &EntityDefinition,
    position: Position,
) -> Entity;

pub fn spawn_player_from_data(
    mut commands: Commands,
    entity_definitions: Res<EntityDefinitions>,
    // ... other parameters
);
```

## Implementation Strategy

### Phase 1: Foundation
1. **Create data structures for entity definitions**
   - Define `EntityDefinition` and component data structs
   - Implement serde serialization/deserialization
   - Add RON support

2. **Asset loading setup**
   - Extend existing asset loading system
   - Add entity definition assets
   - Implement hot-reloading support

### Phase 2: Core Migration
1. **Player spawning conversion**
   - Create player RON definition
   - Refactor `spawn_player` to use data
   - Maintain exact component compatibility

2. **Enemy spawning conversion**
   - Create enemy RON definitions
   - Replace hardcoded enemy spawning
   - Add support for multiple enemy types

### Phase 3: Enhancement
1. **Template system**
   - Add entity inheritance/templates
   - Implement variant support
   - Add randomization options

2. **Tool integration**
   - Hot-reloading for development
   - Validation and error handling
   - Performance optimization

## Data Structure Design

### Entity Definition RON Format
```ron
// assets/entities/player.ron
EntityDefinition(
    name: "Player",
    description: Some("The main character"),
    components: EntityComponents(
        turn_actor: Some(TurnActorData(
            speed: 100,
            action_queue_size: 5,
        )),
        view_shed: Some(ViewShedData(
            radius: 8,
        )),
        tile_sprite: Some(TileSpriteData(
            tile_coords: (10, 18),
            tile_size: (12.0, 12.0),
        )),
        is_player: Some(true),
        is_ai: Some(false),
    ),
)
```

### Directory Structure
```
assets/
├── entities/
│   ├── player.ron
│   ├── enemies/
│   │   ├── whale.ron
│   │   ├── basic_enemy.ron
│   │   └── ...
│   └── items/
│       └── ...
├── textures.ron (existing)
└── ...
```

## Integration Points

### 1. Asset Loading System
- Integrate with existing `TextureAssets` resource
- Add `EntityDefinitions` resource
- Use same loading patterns as current system

### 2. Component Registration
- Ensure all component types are registered for reflection
- Add new data types to Bevy's type registry
- Maintain serialization compatibility

### 3. Spawning System Hooks
- Replace calls in `src/view/screens/gameplay.rs`
- Maintain same system ordering and dependencies
- Preserve turn queue integration

## Performance Considerations

### Loading Strategy
- Load all entity definitions at startup
- Cache compiled spawning functions
- Minimal runtime deserialization

### Memory Usage
- Share common component data
- Use Bevy's asset system for deduplication
- Efficient storage of definition variants

### Runtime Performance
- Pre-compile entity bundle creation
- Avoid runtime reflection where possible
- Benchmark against current hardcoded approach

## Error Handling

### Asset Loading Errors
- Graceful fallback to default definitions
- Clear error messages for malformed RON
- Development-time validation

### Runtime Errors
- Validate component compatibility
- Handle missing definitions gracefully
- Comprehensive logging for debugging

## Testing Strategy

### Unit Tests
- Entity definition parsing
- Component data conversion
- Spawning system functionality

### Integration Tests
- Full entity spawning pipeline
- Asset loading integration
- Performance benchmarks

### Development Tools
- RON syntax validation
- Entity definition viewer
- Hot-reloading verification

## Migration Path

### Backward Compatibility
- Maintain existing component interfaces
- Keep same entity behavior
- Preserve save/load functionality

### Incremental Rollout
1. Add data-driven system alongside existing
2. Migrate player spawning first
3. Gradually convert enemy types
4. Remove hardcoded spawning last

### Risk Mitigation
- Feature flags for new system
- Fallback to hardcoded spawning
- Comprehensive testing before migration
