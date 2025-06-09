# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build Commands

### Development

```bash
# Quick dev build (uses Cranelift if available)
make build
cargo run

# Dev build with hot reloading and debug features
cargo run --features dev_native

# Run with debug logging
cargo run --features dev_log
```

### Release Builds

```bash
# Release build
make build-release
cargo run --release

# Distribution build (maximum optimization)
make build-dist
cargo build --profile dist
```

### Testing and Quality

```bash
# Run all tests
cargo test

# Format code
cargo fmt

# Run linter
cargo clippy

# Check without building
cargo check
```

## Core Architecture

### Turn-Based Game Engine

This is a Bevy-based roguelike with a sophisticated turn-based system:

- **GameState Flow**: `GatherActions` → `ProcessTurns` cycle
- **TurnQueue**: Priority-based scheduling using `BinaryHeap<Reverse<(u64, Entity)>>`
- **Actions**: Polymorphic action system with `ActionType` enum and `GameAction` trait
- **Time Management**: Speed-based timing with automatic overflow handling

### Map and Spatial Systems

- **CurrentMap Resource**: Central map state with integrated pathfinding and FOV
- **Grid-based Tiles**: Single source of truth with bidirectional actor lookup
- **Field of View**: Bit-optimized shadowcasting implementation
- **Pathfinding**: A\* integration with terrain-aware cost calculations

### Entity System

- **Data-Driven Approach**: Entities defined in RON files under `assets/entities/`
- **Command Pattern**: Spawn commands as components (`SpawnPlayerCommand`, `SpawnAICommand`)
- **Component Architecture**: Core components (Position, FieldOfView), Tag components (PlayerTag, AITag)

### Module Structure

- **`src/core/`**: Fundamental game mechanics (turns, movement, FOV, pathfinding)
- **`src/gameplay/`**: Higher-level game logic (player systems, AI, world generation)
- **`src/rendering/`**: Display and tilemap rendering
- **`src/ui/`**: User interface and camera systems
- **`crates/brtk/`**: Custom roguelike toolkit (FOV, pathfinding, grid utilities)
- **`crates/echos_assets/`**: Asset loading and entity definition system

### Key Resources

- **TurnQueue**: Manages entity turn order and timing
- **CurrentMap**: Central map state with spatial queries
- **FovMap**: Bit-packed field of view calculations
- **DistanceSettings**: Configurable distance algorithms for different contexts

### Development Features

- **Hot Reloading**: Entity definitions reload automatically in dev mode
- **Debug Systems**: Comprehensive logging and debug visualization
- **Performance Monitoring**: Turn queue cleanup tracking and optimization

### Asset Organization

- **Entity Definitions**: `assets/entities/*.ron` - Data-driven entity configurations
- **Textures**: `assets/textures/` - Tileset and sprite assets
- **Settings**: `assets/settings/` - Configuration files

### Error Handling

- **GameError Enum**: Comprehensive error types for game operations
- **Graceful Degradation**: Fallback behaviors for asset loading failures
- **Action Validation**: Pre-execution checks with automatic rollback

## Development Notes

- Use `make` targets for convenient building with optimized settings
- Entity definitions support hot reloading - modify RON files for instant changes
- Turn-based architecture enables deterministic gameplay and easy debugging
- Spatial systems are optimized for performance with bit-packed storage
- Follow existing patterns in core modules when adding new systems
- Always run `cargo build` to check progress in rust