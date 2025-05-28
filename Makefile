# Makefile for Echos in the Dark
# Provides convenient build targets with automatic Cranelift detection

.PHONY: help build build-release build-dist run run-release test clean check fmt clippy install-cranelift

# Default target
help:
	@echo "Available targets:"
	@echo "  build          - Dev build with Cranelift if available"
	@echo "  build-release  - Release build with LLVM"
	@echo "  build-dist     - Distribution build with maximum optimization"
	@echo "  run            - Build and run dev version"
	@echo "  run-release    - Build and run release version"
	@echo "  test           - Run all tests"
	@echo "  check          - Check code without building"
	@echo "  fmt            - Format code"
	@echo "  clippy         - Run clippy lints"
	@echo "  clean          - Clean build artifacts"
	@echo "  install-cranelift - Install Cranelift codegen backend"

# Development build with Cranelift detection
build:
	@echo "Building with intelligent Cranelift detection..."
	@./build.sh

# Release build (always uses LLVM)
build-release:
	@echo "Building release version..."
	@./build.sh --release

# Distribution build
build-dist:
	@echo "Building distribution version..."
	@./build.sh --profile dist

# Run development version
run: build
	@echo "Running development version..."
	@bevy run --features dev_native

# Run release version
run-release: build-release
	@echo "Running release version..."
	@bevy run --release

# Run tests
test:
	@echo "Running tests..."
	@cargo test

# Check code without building
check:
	@echo "Checking code..."
	@cargo check

# Format code
fmt:
	@echo "Formatting code..."
	@cargo fmt

# Run clippy lints
clippy:
	@echo "Running clippy..."
	@cargo clippy

# Clean build artifacts
clean:
	@echo "Cleaning build artifacts..."
	@cargo clean

# Install Cranelift codegen backend
install-cranelift:
	@echo "Installing Cranelift codegen backend..."
	@rustup component add rustc-codegen-cranelift-preview

# Check if Cranelift is available
check-cranelift:
	@echo "Checking Cranelift availability..."
	@if rustc -Z codegen-backend=cranelift --version >/dev/null 2>&1; then \
		echo "✅ Cranelift is available"; \
	else \
		echo "❌ Cranelift is not available"; \
		echo "Install with: make install-cranelift"; \
	fi
