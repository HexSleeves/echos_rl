# OpenCode - Echos RL

## Build Commands
- `make build` - Dev build with Cranelift if available
- `make build-release` - Release build with LLVM
- `make run` - Build and run dev version
- `cargo test` - Run all tests
- `cargo test -- --nocapture` - Run tests with output
- `cargo test test_name` - Run a single test

## Lint & Format
- `make clippy` - Run clippy lints
- `make fmt` - Format code with rustfmt
- `cargo check` - Check code without building

## Code Style Guidelines
- **Max line width**: 110 characters
- **Imports**: Group by crate (`imports_granularity = "Crate"`)
- **Formatting**: Single-line functions and if/else statements when possible
- **Error handling**: Use `anyhow` for general errors, `thiserror` for library errors
- **Naming**: Follow Rust conventions (snake_case for variables/functions, CamelCase for types)
- **Comments**: Wrap at 100 characters, format code in doc comments
- **Bevy ECS**: Use small, focused components; specific system queries with filters
- **Macros**: Use `[]` braces for `bevy_ecs::children!` macro

## Project Structure
- Core game code in `src/`
- Reusable components in `crates/`
- Assets in `assets/`