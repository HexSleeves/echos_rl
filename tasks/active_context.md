# Active Context: Data-Driven Entity System Implementation

## Current Session Overview
**Date**: Current session
**Focus**: Planning and initial implementation of data-driven entity spawning system
**Mode**: PLAN MODE → Ready to transition to ACT MODE

## Current State Analysis

### What We've Discovered
1. **Existing System**: Well-structured Bevy 0.16 roguelike with hardcoded entity spawning
2. **Current Spawning**: Located in `src/model/systems/spawner.rs` with fixed component assignments
3. **Asset Pipeline**: Already uses `bevy_asset_loader` and RON format for textures
4. **Components**: Clean ECS structure with Position, TurnActor, ViewShed, TileSprite, etc.

### Current Architecture Strengths
- ✅ Modern Bevy 0.16 with proper ECS patterns
- ✅ Existing asset loading system using RON format
- ✅ Well-organized module structure
- ✅ Turn-based system with proper component integration
- ✅ Working gameplay loop with player/enemy spawning

### Current Pain Points
- ❌ Hardcoded entity spawning limits variety and flexibility
- ❌ No designer-friendly way to modify entity properties
- ❌ Requires code changes for new entity types
- ❌ Difficult to balance or test entity variations

## Planning Complete - Ready for Implementation

### Comprehensive Plan Developed
1. **Product Requirements**: Clear scope and success criteria defined
2. **Technical Architecture**: Detailed component mapping and data flow
3. **System Design**: Integration points with existing Bevy systems identified
4. **Task Breakdown**: 13 detailed tasks across 4 phases with time estimates
5. **Risk Assessment**: High/medium risks identified with mitigation strategies

### Key Architectural Decisions Made
1. **Data Format**: RON (existing format) for entity definitions
2. **Integration**: Extend existing `bevy_asset_loader` system
3. **Migration Strategy**: Incremental replacement with fallback mechanisms
4. **Performance**: Maintain parity with current hardcoded approach
5. **Compatibility**: Preserve all existing component behaviors

## Immediate Next Steps (Ready for ACT MODE)

### Phase 1 Priority: Foundation Implementation
**Next Action**: Task 1.1 - Create Entity Definition Data Structures
**Estimated Time**: 1 hour
**No Dependencies**: Can start immediately

#### Specific Implementation Tasks Ready:
1. **Create module structure**: `src/model/entities/`
2. **Implement core structs**: `EntityDefinition`, `EntityComponents`
3. **Add component data types**: `TurnActorData`, `ViewShedData`, `TileSpriteData`
4. **Set up serde**: Serialization/deserialization with error handling
5. **Write unit tests**: Validate data structures and conversions

### Files Ready to Create
```
src/model/entities/
├── mod.rs              # Module declaration and exports
├── definition.rs       # EntityDefinition and core structs
├── components.rs       # Component data types and conversions
├── loader.rs          # Asset loading integration (Task 1.2)
└── spawner.rs         # Data-driven spawning logic (Task 2.1)
```

### Asset Files Ready to Create
```
assets/entities/
├── player.ron          # Player definition matching current hardcoded values
├── enemies/
│   ├── whale.ron      # Current whale enemy definition
│   └── basic_enemy.ron # Generic enemy template
```

## Current Code Analysis - Key Integration Points

### Existing Components to Support
From `src/model/systems/spawner.rs`:
```rust
// Player components (exact match needed):
PlayerTag,
AwaitingInput,
TurnActor::new(100),
ViewShed { radius: 8 },
TileSprite {
    tile_coords: (10, 18),
    tile_size: Vec2::splat(ViewConstants::TILE_SIZE),
}

// Enemy components (exact match needed):
AITag,
TurnActor::new(120),
TileSprite {
    tile_coords: (0, 16),
    tile_size: Vec2::splat(ViewConstants::TILE_SIZE),
}
```

### Asset Loading Pattern (from `assets/textures.ron`)
```ron
({
    "kenny_textures": Folder(
        path: "textures/kenny",
    ),
    // Add entity_definitions here
})
```

### System Integration Point (from `src/view/screens/gameplay.rs`)
```rust
app.add_systems(
    OnEnter(ScreenState::Gameplay),
    (spawn_map, spawn_player).chain().in_set(GameplaySystemSet::Initialization),
);
```

## Development Environment Ready

### Dependencies Available
- ✅ Bevy 0.16 with full ECS support
- ✅ serde with derive features for serialization
- ✅ bevy_asset_loader 0.23.0 for asset management
- ✅ Existing reflection and type registration patterns

### Hot Reloading Setup
- ✅ `bevy/file_watcher` feature enabled for dev builds
- ✅ Asset hot reloading already working for textures
- ✅ Development environment configured properly

## Validation Criteria for Next Phase

### Task 1.1 Success Criteria
- [ ] All data structures compile without errors
- [ ] RON serialization round-trip works perfectly
- [ ] Component conversion maintains exact data fidelity
- [ ] Unit tests demonstrate correct behavior
- [ ] No performance regression from data structures

### Quality Gates
- [ ] Clippy passes with project standards
- [ ] Tests achieve >80% coverage for new code
- [ ] Documentation is clear and complete
- [ ] Error handling covers malformed data cases

## Risk Monitoring

### High-Priority Risks to Watch
1. **Component Compatibility**: Ensure exact behavioral match during conversion
2. **Performance**: Monitor data deserialization overhead
3. **Type Safety**: Catch serde errors early in development

### Mitigation Ready
- Comprehensive unit testing planned
- Fallback mechanisms designed
- Incremental rollout strategy defined

## PHASE 1 COMPLETED ✅

**Status**: ✅ PHASE 1 COMPLETE - READY FOR PHASE 2
**Completed**: Foundation (Core Data Structures)
**Total Time**: 2 hours
**Quality**: All tests passing (11/11), full asset loading integration

### Task 1.1 COMPLETED ✅ (45 minutes)
- ✅ Created `src/model/entities/` module structure
- ✅ Implemented `EntityDefinition` and `EntityComponents` structs
- ✅ Created component data types: `TurnActorData`, `ViewShedData`, `TileSpriteData`
- ✅ Added serde serialization/deserialization support
- ✅ Implemented conversion traits to ECS components
- ✅ Added comprehensive unit tests (8 tests, all passing)
- ✅ Created RON entity files: player.ron, whale.ron, basic_enemy.ron
- ✅ Verified RON parsing with integration tests
- ✅ Registered types with Bevy reflection system

### Task 1.2 COMPLETED ✅ (1 hour)
- ✅ Created `src/model/entities/loader.rs` with EntityDefinitions resource
- ✅ Integrated with existing bevy_asset_loader system
- ✅ Updated `assets/textures.ron` to include entity_definitions folder
- ✅ Modified loading screen to load EntityDefinitions resource
- ✅ Registered EntityDefinition as Bevy asset type
- ✅ Added comprehensive tests including integration tests
- ✅ Verified hot reloading capability (dev mode ready)
- ✅ All 11 tests passing in entities module

### Task 1.3 COMPLETED ✅ (15 minutes)
- ✅ Created RON entity files with proper directory structure
- ✅ Verified all files parse correctly as EntityDefinition assets
- ✅ Validated component data matches hardcoded values

### Current System State:
- ✅ **Asset Loading**: EntityDefinitions resource loads at startup
- ✅ **Hot Reloading**: File watcher enabled for dev builds
- ✅ **Type Safety**: All types registered with Bevy reflection
- ✅ **Error Handling**: Robust via bevy_asset_loader
- ✅ **Testing**: Comprehensive unit and integration tests
- ✅ **Performance**: No regression, efficient HashMap lookups

## TASK 2.1 COMPLETED ✅

**Status**: ✅ TASK 2.1 COMPLETE - DATA-DRIVEN SPAWNING SYSTEM
**Completed**: Core spawning system migration with fallback support
**Time Taken**: 2 hours
**Quality**: All tests passing (15/15), robust error handling

### What Was Accomplished:
- ✅ Created `src/model/entities/spawner.rs` with data-driven spawning functions
- ✅ Implemented `spawn_entity_from_definition()` core function
- ✅ Added specialized functions: `spawn_player_from_definition()`, `spawn_random_enemy_from_definition()`
- ✅ Built robust error handling with `SpawnError` enum
- ✅ Created fallback module with exact hardcoded behavior preservation
- ✅ Refactored `src/model/systems/spawner.rs` to use data-driven approach with fallback
- ✅ Added comprehensive tests including RON file integration tests
- ✅ Verified component compatibility with existing hardcoded system

### Key Features Implemented:
- ✅ **Graceful Fallback**: System automatically falls back to hardcoded spawning if assets aren't loaded
- ✅ **Error Handling**: Comprehensive error types for all failure scenarios
- ✅ **Component Compatibility**: Exact same components as hardcoded system
- ✅ **Turn Integration**: Seamless integration with existing turn queue system
- ✅ **Map Integration**: Proper entity placement on game map
- ✅ **Logging**: Informative logs for debugging and monitoring

### Current System State:
- ✅ **Spawning**: Both data-driven and fallback spawning work correctly
- ✅ **Asset Loading**: EntityDefinitions resource loads at startup
- ✅ **Component Creation**: Exact behavioral match with hardcoded system
- ✅ **Error Recovery**: Robust fallback mechanisms prevent crashes
- ✅ **Testing**: 15/15 tests passing, including integration tests

### Next Action: Task 2.2 - Player Spawning Migration
**Estimated Time**: 1 hour
**Goal**: Complete player spawning migration and remove hardcoded dependencies
**Dependencies**: Task 2.1 complete ✅

### Ready for Task 2.2: Player Spawning Migration
**Focus**: Finalize player spawning system and ensure full compatibility
**Current Status**: Player spawning already works with data-driven system + fallback
**Remaining Work**: Testing, validation, and cleanup
