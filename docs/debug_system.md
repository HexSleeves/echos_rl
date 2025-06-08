# Debug System Documentation

## Overview

The Echos RL debug system provides comprehensive, categorized debugging capabilities integrated into the development tools. It offers granular control over logging, file output for bug reports, and runtime configuration through a unified development interface.

## Features

- **🎯 Categorized Debug Logging**: 8 distinct categories (AI, Turns, Combat, World, Input, Rendering, Performance, General)
- **🛠️ Unified Development Interface**: Single window combining performance metrics, debug controls, and file logging
- **📁 File Logging**: Automatic log rotation with separate files per category
- **🐞 Bug Report Generation**: Comprehensive reports with recent log entries
- **⚡ Zero-Cost Abstraction**: No performance impact when debug features are disabled
- **🔧 Runtime Controls**: Toggle debug categories on/off during gameplay
- **🌍 Environment Variable Support**: Quick enable/disable via environment variables

## Quick Start

### Enable Debug Features

```bash
# Build with debug features
cargo run --features debug

# Enable specific categories via environment variables
DEBUG_AI=1 cargo run --features debug
DEBUG_TURNS=1 DEBUG_AI=1 cargo run --features debug
```

### Access Development Tools

1. Run the game with debug features enabled
2. The unified "🛠️ Development Tools" window provides access to:
   - **📊 Performance Metrics**: FPS and frame time monitoring
   - **🐛 Debug Logging**: Category controls and status
   - **📁 File Logging**: Configuration and management
   - **🐞 Bug Reports**: One-click bug report generation
   - **💡 Quick Tips**: Keyboard shortcuts and usage examples

### Keyboard Shortcuts

- **`Escape`**: Toggle World Inspector (entity browser)
- **`` ` ``** (backtick): Toggle UI Debug Overlay

## Debug Categories

| Category    | Macro                                     | Environment Variable | Purpose                                   |
| ----------- | ----------------------------------------- | -------------------- | ----------------------------------------- |
| AI          | `debug_ai!()`, `warn_ai!()`               | `DEBUG_AI`           | AI behavior, pathfinding, decision making |
| Turns       | `debug_turns!()`, `warn_turns!()`         | `DEBUG_TURNS`        | Turn processing, queue management         |
| Combat      | `debug_combat!()`, `warn_combat!()`       | `DEBUG_COMBAT`       | Combat calculations, damage, effects      |
| World       | `debug_world!()`, `warn_world!()`         | `DEBUG_WORLD`        | World generation, map operations          |
| Input       | `debug_input!()`, `warn_input!()`         | `DEBUG_INPUT`        | Input handling, key processing            |
| Rendering   | `debug_rendering!()`, `warn_rendering!()` | `DEBUG_RENDERING`    | Graphics, UI, visual systems              |
| Performance | `debug_perf!()`, `warn_perf!()`           | `DEBUG_PERF`         | Performance monitoring, optimization      |
| General     | `debug_general!()`, `warn_general!()`     | `DEBUG_GENERAL`      | General purpose debugging                 |

## Usage Examples

### Basic Debug Logging

```rust
use crate::{debug_ai, warn_ai, debug_ai_time};

fn ai_system() {
    debug_ai!("AI system starting turn processing");

    // Time a critical operation
    debug_ai_time!("pathfinding", {
        let path = calculate_path();
        debug_ai!("Generated path with {} steps", path.len());
    });

    if error_condition {
        warn_ai!("Failed to find valid path for entity");
    }
}
```

### Environment Variable Control

```bash
# Enable only AI debugging
DEBUG_AI=1 cargo run --features debug

# Enable multiple categories
DEBUG_AI=1 DEBUG_TURNS=1 DEBUG_COMBAT=1 cargo run --features debug

# Enable all categories
DEBUG_ALL=1 cargo run --features debug
```

### Runtime Configuration

The unified development tools window allows you to:

- Toggle individual debug categories on/off
- Enable/disable file logging
- Configure log file settings (directory, size limits, rotation)
- Generate bug reports with recent log entries
- View performance metrics in real-time

## File Logging

### Log Files

When file logging is enabled, separate log files are created for each category:

- `echos_rl_ai.log` - AI system logs
- `echos_rl_turns.log` - Turn processing logs
- `echos_rl_combat.log` - Combat system logs
- `echos_rl_world.log` - World generation logs
- `echos_rl_input.log` - Input handling logs
- `echos_rl_rendering.log` - Rendering system logs
- `echos_rl_perf.log` - Performance monitoring logs
- `echos_rl_general.log` - General purpose logs

### Log Rotation

- Automatic rotation when files exceed configured size (default: 10MB)
- Configurable number of rotated files to keep (default: 5)
- Oldest files are automatically deleted

### Bug Reports

The "Generate Bug Report" button creates a comprehensive report including:

- System information and timestamp
- Recent entries from all enabled debug categories
- Current configuration settings
- Saved to `bug_report.txt` in the project root

## Migration Guide

### From Standard Logging

Replace standard logging calls with category-specific debug macros:

```rust
// Before
info!("AI entity {} starting wander behavior", entity_name);
warn!("Failed to find path for entity {:?}", entity);

// After
debug_ai!("AI entity {} starting wander behavior", entity_name);
warn_ai!("Failed to find path for entity {:?}", entity);
```

### Conditional Compilation

Debug macros automatically handle conditional compilation:

```rust
// This code only compiles when debug features are enabled
debug_ai!("Expensive debug calculation: {}", expensive_calculation());

// Zero cost when debug features are disabled - expensive_calculation() is never called
```

## Configuration

### Cargo Features

- `debug`: Enables debug UI and runtime controls
- `debug-logging`: Enables file logging capabilities (automatically included with `debug`)

### Runtime Configuration

The `DebugConfig` resource can be modified at runtime through the development tools UI or programmatically:

```rust
fn configure_debug_system(mut debug_config: ResMut<DebugConfig>) {
    debug_config.file_logging_enabled = true;
    debug_config.max_log_file_size = 20 * 1024 * 1024; // 20MB
    debug_config.max_log_files = 10;
}
```

## Performance Impact

### Zero-Cost Abstraction

When debug features are disabled:

- Debug macros compile to nothing (zero runtime cost)
- No file I/O operations
- No string formatting
- No memory allocations for debug data

### With Debug Features Enabled

- Minimal overhead for enabled categories
- File I/O is buffered and asynchronous where possible
- Debug UI updates only when window is open

## Best Practices

1. **Use Appropriate Categories**: Choose the most specific category for your debug messages
2. **Avoid Expensive Operations**: Debug macros should not perform expensive calculations
3. **Use Timing Macros**: Use `debug_*_time!()` macros for performance-critical sections
4. **Environment Variables**: Use environment variables for quick debugging sessions
5. **File Logging**: Enable file logging when you need persistent debug information
6. **Bug Reports**: Generate bug reports when encountering issues for comprehensive debugging

## Integration with Development Tools

The debug system is fully integrated with the existing development infrastructure:

- **World Inspector**: Browse and modify entities in real-time (Escape key)
- **UI Debug Overlay**: Toggle UI debugging information (backtick key)
- **Performance Metrics**: Real-time FPS and frame time monitoring
- **State Transitions**: Automatic logging of game state changes

This unified approach provides a comprehensive development environment for debugging, profiling, and understanding the game's behavior at runtime.
