# 🌟 Echos in the Dark

A modern roguelike game built with [Bevy Engine](https://bevyengine.org/) featuring a data-driven entity system and turn-based gameplay.

![Bevy Version](https://img.shields.io/badge/Bevy-0.16-blue)
![Rust Edition](https://img.shields.io/badge/Rust-2024-orange)
![License](https://img.shields.io/badge/License-CC0%201.0-green)
![Build Status](https://img.shields.io/badge/Build-Passing-brightgreen)
![CodeRabbit Pull Request Reviews](https://img.shields.io/coderabbit/prs/github/HexSleeves/echos_rl?utm_source=oss&utm_medium=github&utm_campaign=HexSleeves%2Fechos_rl&labelColor=171717&color=FF570A&link=https%3A%2F%2Fcoderabbit.ai&label=CodeRabbit+Reviews)

## 🎮 About

**Echos in the Dark** is a turn-based roguelike that emphasizes flexible, data-driven design. The game features a sophisticated entity system where all game objects (players, enemies, items) are defined through RON (Rust Object Notation) files, making it easy to modify and extend without touching code.

### ✨ Key Features

- **🔧 Data-Driven Architecture**: All entities defined in RON files for easy modification
- **⚡ Modern Bevy ECS**: Built on Bevy 0.16 with full ECS architecture
- **🎯 Turn-Based Combat**: Strategic turn-based gameplay with sophisticated action system
- **👁️ Field of View**: Bit-optimized shadowcasting FOV system
- **🗺️ Procedural Generation**: Room-based map generation with configurable parameters
- **🤖 Advanced AI**: Multi-behavior AI system (chase, flee, wander, idle) with scoring
- **⚔️ Combat System**: Health, stats, and damage calculations with event-driven architecture
- **🧭 Pathfinding**: A* pathfinding with terrain-aware cost calculations and caching
- **🔄 Hot Reloading**: Real-time asset reloading during development
- **🎨 Tilemap Rendering**: Efficient tilemap-based graphics with multiple tilesets

## 🚀 Quick Start

### Prerequisites

- [Rust](https://rustlang.org/) (latest stable)
- [Git](https://git-scm.com/)

### Installation

```bash
# Clone the repository
git clone https://github.com/lecoqjacob/echos_rl.git
cd echos_rl

# Run the game in development mode
cargo run

# Or build for release
cargo run --release
```

### Development Mode

For development with hot reloading and debug features:

```bash
# Run with development features
cargo run --features dev_native

# Run with logging
cargo run --features dev_log
```

## 🏗️ Architecture

### Project Structure

```
src/
├── core/               # Core game mechanics
│   ├── actions/        # Action system (walk, attack, wait, teleport)
│   ├── commands/       # Entity spawn commands
│   ├── components/     # Core ECS components (position, health, stats)
│   ├── resources/      # Game resources (maps, FOV, turn queue)
│   ├── systems/        # Core systems (combat, FOV)
│   └── types/          # Core types and error handling
├── gameplay/           # High-level game logic
│   ├── enemies/        # AI system with multiple behaviors
│   ├── player/         # Player input and actions
│   ├── turns/          # Turn-based system management
│   └── world/          # Map generation and spawning
├── rendering/          # Display and graphics
│   ├── screens/        # Game screens (loading, gameplay)
│   └── systems/        # Rendering systems
├── ui/                 # User interface
│   ├── components/     # UI components (camera, interaction)
│   ├── systems/        # UI systems
│   └── utils/          # UI utilities and widgets
└── utils/              # General utilities

crates/
├── brtk/               # Custom roguelike toolkit
│   ├── fov/            # Field of view algorithms
│   ├── pathfinding/    # A* pathfinding with caching
│   ├── grid/           # Grid utilities and shapes
│   └── random/         # Dice rolling system
└── echos_assets/       # Asset loading system

assets/
├── entities/           # Entity definitions (RON files)
│   ├── player.definition.ron
│   └── enemies/        # Enemy definitions
├── textures/           # Game textures and tilesets
└── settings/           # Configuration files
```

### Data-Driven Entity System

Entities are defined using RON files, making the game highly moddable:

```ron
// assets/entities/player.definition.ron
EntityDefinition(
    name: "Player",
    description: Some("The player character"),
    components: EntityComponents(
        turn_actor: Some(TurnActorData(speed: 100)),
        field_of_view: Some(FieldOfViewData(radius: 8)),
        tile_sprite: Some(TileSpriteData(tile_coords: (10, 18))),
        health: Some(HealthData(max_health: 30, current_health: 30)),
        stats: Some(StatsData(strength: 3, defense: 2)),
        player_tag: Some(true),
    ),
)
```

### Command-Based Spawning

The game uses a command-based entity spawning system:

```rust
// Queue entity spawning
commands.spawn(SpawnPlayerCommand { position });
commands.spawn(SpawnAICommand { 
    entity_key: "hostile_guard".to_string(), 
    position 
});

// Commands are processed by the spawn system with automatic fallback
// to default entities if the specified entity is not found
```

## 🎯 Game Features

### Turn-Based System

- **Priority Queue**: Binary heap-based turn scheduling with overflow handling
- **Speed-Based Timing**: Faster entities act more frequently based on speed stats
- **Action Types**: Walk, attack, wait, and teleport actions with validation
- **Strategic Depth**: Plan your moves carefully with full turn preview

### Advanced AI System

- **Behavior Scoring**: AI entities score different behaviors (chase, flee, wander, idle)
- **Dynamic Decision Making**: AI chooses best action based on current situation
- **Configurable Behaviors**: Each enemy type has different behavioral parameters
- **Performance Optimized**: Efficient pathfinding with caching and distance-based updates

### Procedural Generation

- **Room-Based Maps**: Configurable room generation with corridors
- **Dynamic Spawning**: Enemies spawn based on room size and type
- **Weighted Encounters**: Different enemy types spawn with configurable probabilities
- **Configurable Parameters**: Map size, room count, and generation rules easily modified

## 🛠️ Development

### Building

This project includes an intelligent build system with automatic Cranelift detection for faster development builds:

```bash
# Development build with automatic Cranelift detection (recommended)
./build.sh

# Release build (optimized with LLVM)
./build.sh --release

# Distribution build (maximum optimization)
./build.sh --profile dist

# Or use convenient make targets
make build          # Dev build
make build-release  # Release build
make run           # Build and run
```

**Traditional cargo still works:**

```bash
cargo build         # Standard build
cargo build --release
```

For detailed information about the build system, see [BUILD_GUIDE.md](BUILD_GUIDE.md).

### Testing

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test module
cargo test entities
```

### Code Quality

```bash
# Format code
cargo fmt

# Run linter
cargo clippy

# Check for security vulnerabilities
cargo audit
```

### Asset Development

Entity definitions support hot reloading in development mode:

1. Modify any `.ron` file in `assets/entities/`
2. Changes are automatically loaded in-game
3. No restart required

## 📦 Dependencies

### Core Dependencies

- **[Bevy](https://bevyengine.org/)** `0.16` - Game engine
- **[bevy_ecs_tilemap](https://github.com/StarArawn/bevy_ecs_tilemap)** `0.16` - Tilemap rendering
- **[bevy_asset_loader](https://github.com/NiklasEi/bevy_asset_loader)** `0.23` - Asset management
- **[bevy_kira_audio](https://github.com/NiklasEi/bevy_kira_audio)** `0.23` - Audio system

### Utility Dependencies

- **[serde](https://serde.rs/)** `1.0` - Serialization framework
- **[ron](https://github.com/ron-rs/ron)** `0.10` - Rust Object Notation
- **[fastrand](https://github.com/smol-rs/fastrand)** `2.0` - Fast random number generation
- **[thiserror](https://github.com/dtolnay/thiserror)** `2.0` - Error handling

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guidelines](CONTRIBUTING.md) for details.

### Development Setup

1. **Fork the repository**
2. **Create a feature branch**: `git checkout -b feature/amazing-feature`
3. **Make your changes** following our coding standards
4. **Add tests** for new functionality
5. **Run the test suite**: `cargo test`
6. **Commit your changes**: `git commit -m 'Add amazing feature'`
7. **Push to the branch**: `git push origin feature/amazing-feature`
8. **Open a Pull Request**

### Coding Standards

- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `cargo fmt` for formatting
- Ensure `cargo clippy` passes without warnings
- Write tests for new functionality
- Document public APIs with doc comments
- Follow the existing project structure

### Areas for Contribution

- 🎮 **Gameplay Features**: New mechanics, abilities, items
- 🎨 **Art & Assets**: Sprites, tilesets, animations
- 🔧 **Tools**: Development tools, editors, utilities
- 📚 **Documentation**: Tutorials, guides, API docs
- 🐛 **Bug Fixes**: Performance improvements, bug fixes
- 🧪 **Testing**: Unit tests, integration tests, benchmarks

## 📋 Roadmap

### Phase 1: Foundation ✅

- [x] Data-driven entity system
- [x] Command-based spawning
- [x] Asset loading integration
- [x] Hot reloading support

### Phase 2: Core Gameplay ✅

- [x] Combat system with health and damage
- [x] Advanced AI with multiple behaviors
- [x] Turn-based action system
- [x] Field of view and pathfinding
- [ ] Inventory management
- [ ] Character progression
- [ ] Save/load system

### Phase 3: Content 🚧

- [x] Multiple enemy types (hostile guards, wanderers, critters)
- [x] Procedural map generation
- [ ] Items and equipment
- [ ] Special abilities
- [ ] Multiple levels/areas

### Phase 4: Polish 📋

- [ ] UI improvements
- [ ] Audio integration
- [ ] Performance optimization
- [ ] Accessibility features

## 📄 License

This project is licensed under the **CC0 1.0 Universal** license - see the [LICENSE](LICENSE) file for details.

This means you can:

- ✅ Use the code for any purpose
- ✅ Modify and distribute
- ✅ Use commercially
- ✅ No attribution required

## 🙏 Acknowledgments

- **[Bevy Community](https://bevyengine.org/community/)** - For the amazing game engine
- **[Rust Community](https://www.rust-lang.org/community)** - For the fantastic language and ecosystem
- **[Roguelike Development Community](https://www.reddit.com/r/roguelikedev/)** - For inspiration and guidance

## 📞 Contact

- **Author**: Jacob L ([@lecoqjacob](https://github.com/lecoqjacob))
- **Email**: <lecoqjacob@gmail.com>
- **Project**: [https://github.com/lecoqjacob/echos_rl](https://github.com/lecoqjacob/echos_rl)

---

<div align="center">

**[⭐ Star this repo](https://github.com/lecoqjacob/echos_rl)** if you find it useful!

Made with ❤️ and [Rust](https://rust-lang.org/) 🦀

</div>
