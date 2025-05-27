# 🌟 Echos in the Dark

A modern roguelike game built with [Bevy Engine](https://bevyengine.org/) featuring a data-driven entity system and turn-based gameplay.

![Bevy Version](https://img.shields.io/badge/Bevy-0.16-blue)
![Rust Edition](https://img.shields.io/badge/Rust-2024-orange)
![License](https://img.shields.io/badge/License-CC0%201.0-green)
![Build Status](https://img.shields.io/badge/Build-Passing-brightgreen)

## 🎮 About

**Echos in the Dark** is a turn-based roguelike that emphasizes flexible, data-driven design. The game features a sophisticated entity system where all game objects (players, enemies, items) are defined through RON (Rust Object Notation) files, making it easy to modify and extend without touching code.

### ✨ Key Features

- **🔧 Data-Driven Architecture**: All entities defined in RON files for easy modification
- **⚡ Modern Bevy ECS**: Built on Bevy 0.16 with full ECS architecture
- **🎯 Turn-Based Combat**: Strategic turn-based gameplay with action queuing
- **👁️ Field of View**: Dynamic lighting and vision system
- **🗺️ Procedural Generation**: Randomly generated maps and encounters
- **🔄 Hot Reloading**: Real-time asset reloading during development
- **🎨 Tilemap Rendering**: Efficient tilemap-based graphics
- **🎵 Audio System**: Integrated audio with Kira Audio

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
├── controller/          # Input handling and game control
│   └── systems/        # Controller systems
├── model/              # Game logic and data
│   ├── components/     # ECS components
│   ├── entities/       # Data-driven entity system
│   ├── resources/      # Game resources
│   ├── commands/       # Entity commands
│   └── systems/        # Game logic systems
└── view/               # Rendering and UI
    ├── screens/        # Game screens
    ├── systems/        # Rendering systems
    └── resources/      # View resources

assets/
├── entities/           # Entity definitions (RON files)
│   ├── player.ron     # Player definition
│   └── enemies/       # Enemy definitions
├── textures/          # Game textures
└── settings/          # Configuration files
```

### Data-Driven Entity System

Entities are defined using RON files, making the game highly moddable:

```ron
// assets/entities/player.ron
EntityDefinition(
    name: "Player",
    description: Some("The player character"),
    components: EntityComponents(
        turn_actor: Some(TurnActorData(speed: 100)),
        view_shed: Some(ViewShedData(radius: 8)),
        tile_sprite: Some(TileSpriteData(tile_coords: (10, 18))),
        is_player: Some(true),
        is_ai: Some(false),
    ),
)
```

### Command-Based Spawning

The game uses a command-based entity spawning system:

```rust
// Queue entity spawning
commands.spawn_player(position);
commands.spawn_enemy("whale", position);
commands.spawn_random_enemy(position);

// Commands are processed by the spawn system with automatic fallback
```

## 🎯 Game Features

### Turn-Based System
- **Action Queue**: Queue multiple actions per turn
- **Speed-Based Timing**: Faster entities act more frequently
- **Strategic Depth**: Plan your moves carefully

### Entity System
- **Flexible Components**: Mix and match components for unique entities
- **Data-Driven**: Modify entities without code changes
- **Hot Reloading**: See changes instantly during development

### Procedural Generation
- **Random Maps**: Each playthrough features unique layouts
- **Dynamic Spawning**: Enemies spawn based on level and conditions
- **Balanced Encounters**: Weighted spawn systems for fair gameplay

## 🛠️ Development

### Building

```bash
# Development build (faster compilation)
cargo build

# Release build (optimized)
cargo build --release

# Distribution build (maximum optimization)
cargo build --profile dist
```

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

### Phase 2: Core Gameplay 🚧
- [ ] Combat system
- [ ] Inventory management
- [ ] Character progression
- [ ] Save/load system

### Phase 3: Content 📋
- [ ] Multiple enemy types
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
- **Email**: lecoqjacob@gmail.com
- **Project**: [https://github.com/lecoqjacob/echos_rl](https://github.com/lecoqjacob/echos_rl)

---

<div align="center">

**[⭐ Star this repo](https://github.com/lecoqjacob/echos_rl)** if you find it useful!

Made with ❤️ and [Rust](https://rust-lang.org/) 🦀

</div>
