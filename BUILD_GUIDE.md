# Build Guide - Intelligent Cranelift Detection

This project includes an intelligent build system that automatically detects and uses the Cranelift codegen backend for faster development builds while falling back to LLVM when Cranelift is unavailable.

## Quick Start

### Using the Build Script (Recommended)

```bash
# Development build with automatic Cranelift detection
./build.sh

# Release build (always uses LLVM for optimization)
./build.sh --release

# Distribution build with maximum optimization
./build.sh --profile dist
```

### Using Make Targets

```bash
# Show all available targets
make help

# Development build
make build

# Release build
make build-release

# Build and run
make run
```

### Traditional Cargo (Manual)

```bash
# Standard cargo build (may or may not use Cranelift depending on config)
cargo build

# Force LLVM backend
CARGO_CONFIG_PROFILE_DEV_CODEGEN_BACKEND="" cargo build

# Force Cranelift backend (if available)
CARGO_CONFIG_PROFILE_DEV_CODEGEN_BACKEND="cranelift" cargo build
```

## Cranelift Backend

### What is Cranelift?

Cranelift is an alternative Rust codegen backend that prioritizes fast compilation over runtime optimization. It's perfect for development builds where you want quick iteration cycles.

**Benefits:**
- ⚡ **Faster compilation** - Significantly reduces build times for dev builds
- 🔄 **Quick iteration** - Faster feedback loop during development
- 🛠️ **Development focused** - Optimized for the edit-compile-test cycle

**Trade-offs:**
- 🐌 **Slower runtime** - Generated code is less optimized than LLVM
- 🎯 **Dev builds only** - Not recommended for release builds
- 🧪 **Experimental** - Still in preview, may have compatibility issues

### Installation

Install Cranelift codegen backend:

```bash
# Install via rustup
rustup component add rustc-codegen-cranelift-preview

# Or use our make target
make install-cranelift
```

### Checking Availability

```bash
# Check if Cranelift is available
make check-cranelift

# Manual check
rustc -Z codegen-backend=cranelift --version
```

## Build System Architecture

### Automatic Detection Logic

The build system uses a three-tier detection approach:

1. **`build.rs`** - Compile-time detection that sets environment variables
2. **`build.sh`** - Runtime detection with intelligent backend selection
3. **`.cargo/config.toml`** - Static configuration with dynamic overrides

### Decision Matrix

| Build Type | Cranelift Available | Backend Used | Reason |
|------------|-------------------|--------------|---------|
| `dev` | ✅ Yes | Cranelift | Faster compilation |
| `dev` | ❌ No | LLVM | Fallback |
| `release` | ✅ Yes | LLVM | Better optimization |
| `release` | ❌ No | LLVM | Standard |
| `dist` | ✅ Yes | LLVM | Maximum optimization |
| `dist` | ❌ No | LLVM | Standard |

### File Structure

```
├── build.rs              # Compile-time Cranelift detection
├── build.sh              # Intelligent build script (bash)
├── build_rust.rs         # Cross-platform build script (Rust)
├── Makefile              # Convenient build targets
├── .cargo/config.toml    # Cargo configuration
└── BUILD_GUIDE.md        # This file
```

## Build Scripts

### `build.sh` (Primary)

Feature-rich bash script with:
- ✅ Automatic Cranelift detection
- ✅ Colored output and status messages
- ✅ Command-line argument parsing
- ✅ Help documentation
- ✅ Error handling

```bash
./build.sh [OPTIONS]

OPTIONS:
    --release           Build in release mode
    --profile PROFILE   Use specific build profile
    -h, --help         Show help message
```

### `build_rust.rs` (Alternative)

Cross-platform Rust script using `rust-script`:
- ✅ Platform independent
- ✅ Rust-native argument parsing
- ✅ Colored terminal output
- ✅ Same logic as bash script

```bash
# Requires rust-script: cargo install rust-script
rust-script build_rust.rs --help
```

## Configuration Files

### `.cargo/config.toml`

Contains the base Cranelift configuration:

```toml
[unstable]
codegen-backend = true

# Cranelift for main crate (when enabled)
# [profile.dev]
# codegen-backend = "cranelift"

# LLVM for dependencies (better compatibility)
[profile.dev.package."*"]
codegen-backend = "llvm"
```

### `build.rs`

Compile-time detection that:
- Checks Cranelift availability
- Sets environment variables
- Provides build warnings/info

## Environment Variables

The build system uses these environment variables:

| Variable | Purpose | Values |
|----------|---------|---------|
| `CARGO_CONFIG_PROFILE_DEV_CODEGEN_BACKEND` | Override dev backend | `"cranelift"`, `"llvm"`, `""` |
| `CRANELIFT_AVAILABLE` | Detection result | `"1"`, `"0"` |
| `USE_CRANELIFT` | Usage decision | `"1"`, `"0"` |

## Troubleshooting

### Cranelift Not Working

1. **Check installation:**
   ```bash
   make check-cranelift
   ```

2. **Reinstall component:**
   ```bash
   rustup component remove rustc-codegen-cranelift-preview
   rustup component add rustc-codegen-cranelift-preview
   ```

3. **Force LLVM backend:**
   ```bash
   CARGO_CONFIG_PROFILE_DEV_CODEGEN_BACKEND="" cargo build
   ```

### macOS Issues

Cranelift may have compatibility issues on macOS. If you encounter problems:

1. **Disable Cranelift in config:**
   ```toml
   # Comment out in .cargo/config.toml
   # [unstable]
   # codegen-backend = true
   ```

2. **Use LLVM-only builds:**
   ```bash
   ./build.sh --release  # Always uses LLVM
   ```

### Build Failures

1. **Clean and rebuild:**
   ```bash
   make clean
   make build
   ```

2. **Check Rust version:**
   ```bash
   rustc --version  # Should be nightly
   ```

3. **Verify toolchain:**
   ```bash
   rustup show
   ```

## Performance Comparison

Typical compilation time improvements with Cranelift:

| Project Size | LLVM Time | Cranelift Time | Improvement |
|--------------|-----------|----------------|-------------|
| Small (< 10k LOC) | 30s | 15s | ~50% faster |
| Medium (10-50k LOC) | 2m | 45s | ~60% faster |
| Large (> 50k LOC) | 5m | 1.5m | ~70% faster |

*Note: Runtime performance will be slower with Cranelift*

## Best Practices

### Development Workflow

1. **Use Cranelift for dev builds:**
   ```bash
   ./build.sh  # Fast compilation
   ```

2. **Test with LLVM periodically:**
   ```bash
   ./build.sh --release  # Check optimized performance
   ```

3. **Always release with LLVM:**
   ```bash
   ./build.sh --profile dist  # Maximum optimization
   ```

### CI/CD Integration

```yaml
# Example GitHub Actions
- name: Build (Dev)
  run: ./build.sh

- name: Build (Release)
  run: ./build.sh --release
```

### IDE Integration

Update your IDE build tasks to use the intelligent build script:

```json
// .vscode/tasks.json
{
    "label": "Build (Intelligent)",
    "type": "shell",
    "command": "./build.sh",
    "group": {
        "kind": "build",
        "isDefault": true
    }
}
```

## Migration Guide

### From Standard Cargo

If you were using standard `cargo build`:

1. **Replace with build script:**
   ```bash
   # Old
   cargo build

   # New
   ./build.sh
   ```

2. **Update scripts/aliases:**
   ```bash
   # Add to .bashrc/.zshrc
   alias dev-build="./build.sh"
   alias release-build="./build.sh --release"
   ```

### From Manual Cranelift

If you were manually configuring Cranelift:

1. **Remove manual config** from `.cargo/config.toml`
2. **Use the build script** for automatic detection
3. **Keep environment overrides** for special cases

## Contributing

When contributing to this project:

1. **Use the build script** for development
2. **Test both backends** before submitting PRs
3. **Update this guide** if you modify the build system
4. **Document any new build requirements**

---

For more information, see the main [README.md](README.md) or open an issue.
