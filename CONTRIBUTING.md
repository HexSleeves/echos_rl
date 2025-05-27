# Contributing to Echos in the Dark

Thank you for your interest in contributing to **Echos in the Dark**! This document provides guidelines and information for contributors.

## 🚀 Getting Started

### Prerequisites

- [Rust](https://rustlang.org/) (latest stable version)
- [Git](https://git-scm.com/)
- Basic familiarity with [Bevy Engine](https://bevyengine.org/)
- Understanding of ECS (Entity Component System) architecture

### Development Setup

1. **Fork and Clone**
   ```bash
   git clone https://github.com/YOUR_USERNAME/echos_rl.git
   cd echos_rl
   ```

2. **Install Dependencies**
   ```bash
   cargo build
   ```

3. **Run Tests**
   ```bash
   cargo test
   ```

4. **Start Development Server**
   ```bash
   cargo run --features dev_native
   ```

## 📋 How to Contribute

### 1. Choose an Issue

- Check the [Issues](https://github.com/lecoqjacob/echos_rl/issues) page
- Look for issues labeled `good first issue` for beginners
- Comment on the issue to let others know you're working on it

### 2. Create a Branch

```bash
git checkout -b feature/your-feature-name
# or
git checkout -b fix/bug-description
```

### 3. Make Your Changes

- Follow our [coding standards](#coding-standards)
- Write tests for new functionality
- Update documentation as needed

### 4. Test Your Changes

```bash
# Run all tests
cargo test

# Check formatting
cargo fmt --check

# Run linter
cargo clippy -- -D warnings

# Test in development mode
cargo run --features dev_native
```

### 5. Submit a Pull Request

- Push your branch to your fork
- Create a Pull Request with a clear description
- Link any related issues
- Wait for review and address feedback

## 🎯 Areas for Contribution

### 🎮 Gameplay Features
- **Combat System**: Implement turn-based combat mechanics
- **Inventory Management**: Item handling and equipment systems
- **Character Progression**: Leveling, skills, and abilities
- **AI Behavior**: Enemy AI patterns and decision making

### 🔧 Technical Improvements
- **Performance Optimization**: Profiling and optimization
- **Error Handling**: Robust error recovery systems
- **Testing**: Unit tests, integration tests, benchmarks
- **Documentation**: Code documentation and tutorials

### 🎨 Content Creation
- **Entity Definitions**: New enemies, items, and characters
- **Level Design**: Map layouts and encounter design
- **Art Assets**: Sprites, tilesets, and animations
- **Audio**: Sound effects and music integration

### 🛠️ Tools and Utilities
- **Entity Editor**: Visual entity definition editor
- **Map Editor**: Level design tools
- **Asset Pipeline**: Improved asset processing
- **Debug Tools**: Development and debugging utilities

## 📝 Coding Standards

### Rust Guidelines

- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `cargo fmt` for consistent formatting
- Ensure `cargo clippy` passes without warnings
- Write idiomatic Rust code with proper error handling

### Code Style

```rust
// ✅ Good: Clear, documented function
/// Spawns a player entity at the specified position
///
/// # Arguments
/// * `commands` - Bevy commands for entity creation
/// * `position` - World position for the player
///
/// # Returns
/// * `Entity` - The spawned player entity ID
pub fn spawn_player(commands: &mut Commands, position: Position) -> Entity {
    commands.spawn_player(position)
}

// ❌ Bad: Unclear, undocumented function
pub fn sp(c: &mut Commands, p: Position) -> Entity {
    c.spawn_player(p)
}
```

### Project Structure

- **Model**: Game logic, components, resources
- **View**: Rendering, UI, visual systems
- **Controller**: Input handling, game control
- **Assets**: Data files, textures, configurations

### Entity System Guidelines

```rust
// ✅ Good: Data-driven entity definition
EntityDefinition(
    name: "Fire Elemental",
    description: Some("A creature of pure flame"),
    components: EntityComponents(
        turn_actor: Some(TurnActorData(speed: 80)),
        tile_sprite: Some(TileSpriteData(tile_coords: (5, 12))),
        is_ai: Some(true),
        // Add custom components as needed
    ),
)

// ✅ Good: Component with clear purpose
#[derive(Component, Serialize, Deserialize, Debug, Clone, Reflect)]
pub struct FireDamageData {
    /// Damage dealt per turn
    pub damage_per_turn: u32,
    /// Number of turns the effect lasts
    pub duration: u32,
}
```

## 🧪 Testing Guidelines

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entity_spawning() {
        // Arrange
        let mut world = World::new();
        let mut commands = world.commands();

        // Act
        let entity = spawn_player(&mut commands, Position::new(0, 0));

        // Assert
        assert!(world.get::<PlayerTag>(entity).is_some());
    }
}
```

### Integration Tests

```rust
// tests/integration/entity_system.rs
use echos_rl::*;

#[test]
fn test_full_entity_pipeline() {
    // Test complete entity creation from RON to ECS
}
```

### Asset Tests

```rust
#[test]
fn test_ron_file_parsing() {
    let player_ron = include_str!("../assets/entities/player.ron");
    let entity_def: EntityDefinition = ron::from_str(player_ron)
        .expect("Player RON file should parse correctly");

    assert_eq!(entity_def.name, "Player");
    assert!(entity_def.is_player());
}
```

## 📚 Documentation

### Code Documentation

- Use `///` for public API documentation
- Include examples in doc comments
- Document all public functions, structs, and modules
- Explain complex algorithms and design decisions

### Asset Documentation

```ron
// assets/entities/example.ron
EntityDefinition(
    name: "Example Entity",
    description: Some("This is an example entity for documentation"),
    // Document component purposes
    components: EntityComponents(
        // Speed determines turn frequency (higher = more turns)
        turn_actor: Some(TurnActorData(speed: 100)),
        // Sprite coordinates in the tileset (x, y)
        tile_sprite: Some(TileSpriteData(tile_coords: (0, 0))),
    ),
)
```

## 🐛 Bug Reports

When reporting bugs, please include:

1. **Description**: Clear description of the issue
2. **Steps to Reproduce**: Exact steps to trigger the bug
3. **Expected Behavior**: What should happen
4. **Actual Behavior**: What actually happens
5. **Environment**: OS, Rust version, game version
6. **Logs**: Relevant error messages or logs

### Bug Report Template

```markdown
## Bug Description
Brief description of the bug

## Steps to Reproduce
1. Step one
2. Step two
3. Step three

## Expected Behavior
What should happen

## Actual Behavior
What actually happens

## Environment
- OS: [e.g., Windows 11, macOS 14, Ubuntu 22.04]
- Rust Version: [e.g., 1.75.0]
- Game Version: [e.g., 0.1.0]

## Additional Context
Any other relevant information
```

## 💡 Feature Requests

For feature requests, please:

1. Check if the feature already exists or is planned
2. Describe the feature and its benefits
3. Provide examples or mockups if applicable
4. Consider implementation complexity
5. Discuss with maintainers before starting work

## 🔄 Pull Request Process

### Before Submitting

- [ ] Code follows project style guidelines
- [ ] Tests pass (`cargo test`)
- [ ] Linting passes (`cargo clippy`)
- [ ] Code is formatted (`cargo fmt`)
- [ ] Documentation is updated
- [ ] Commit messages are clear and descriptive

### PR Description Template

```markdown
## Description
Brief description of changes

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Testing
- [ ] Unit tests added/updated
- [ ] Integration tests added/updated
- [ ] Manual testing completed

## Checklist
- [ ] Code follows style guidelines
- [ ] Self-review completed
- [ ] Documentation updated
- [ ] Tests pass
```

### Review Process

1. **Automated Checks**: CI/CD runs tests and linting
2. **Code Review**: Maintainers review code quality and design
3. **Testing**: Manual testing of new features
4. **Approval**: At least one maintainer approval required
5. **Merge**: Squash and merge to main branch

## 🏷️ Commit Guidelines

### Commit Message Format

```
type(scope): brief description

Longer description if needed

Fixes #123
```

### Types

- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

### Examples

```bash
feat(entities): add fire elemental enemy type

Add new fire elemental enemy with burning damage over time.
Includes RON definition and custom FireDamage component.

Fixes #45

fix(spawning): resolve entity command borrowing issue

Replace closure-based spawning with direct conditional logic
to avoid multiple mutable borrows of world resources.

docs(readme): update installation instructions

Add development mode setup and hot reloading information.
```

## 🤝 Community Guidelines

### Code of Conduct

- Be respectful and inclusive
- Welcome newcomers and help them learn
- Focus on constructive feedback
- Respect different opinions and approaches
- Follow the [Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct)

### Communication

- **GitHub Issues**: Bug reports and feature requests
- **Pull Requests**: Code contributions and discussions
- **Discussions**: General questions and community chat

## 📞 Getting Help

- **Documentation**: Check the [docs/](docs/) directory
- **Issues**: Search existing issues for solutions
- **Discussions**: Ask questions in GitHub Discussions
- **Code**: Read the source code and tests for examples

## 🎉 Recognition

Contributors will be:
- Listed in the project's contributors
- Mentioned in release notes for significant contributions
- Invited to join the core team for sustained contributions

Thank you for contributing to **Echos in the Dark**! 🌟
