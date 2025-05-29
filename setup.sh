#!/bin/bash
set -e

echo "Setting up Rust Bevy game development environment..."

# Install Rust if not present
if ! command -v rustc &>/dev/null; then
  echo "Installing Rust..."
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain nightly-2025-05-01
  source $HOME/.cargo/env
  echo 'export PATH="$HOME/.cargo/bin:$PATH"' >>$HOME/.profile
else
  echo "Rust is already installed"
  # Ensure we have the right toolchain
  rustup toolchain install nightly-2025-05-01
  rustup default nightly-2025-05-01
fi

# Install Cranelift component for faster dev builds
echo "Installing Cranelift component..."
rustup component add rustc-codegen-cranelift-preview

# Add cargo bin to PATH
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >>$HOME/.profile
source $HOME/.profile

# Navigate to workspace
cd /mnt/persist/workspace

# Try to install minimal system dependencies that might be available
echo "Attempting to install available system dependencies..."
sudo apt-get install -y libasound2-dev libudev-dev pkg-config || echo "Some packages may not be available, continuing..."

# Check what compilers are available and configure accordingly
echo "Checking available compilers..."
if command -v gcc &>/dev/null; then
  echo "Found GCC, configuring Rust to use it"
  export CC=gcc
  export CXX=g++

  # Modify the existing .cargo/config.toml to use gcc instead of clang
  echo "Modifying .cargo/config.toml to use gcc..."
  if [ -f .cargo/config.toml ]; then
    # Backup original
    cp .cargo/config.toml .cargo/config.toml.backup
    # Replace clang with gcc
    sed -i 's/linker = "clang"/linker = "gcc"/' .cargo/config.toml
    # Remove mold linker which requires clang
    sed -i '/mold/d' .cargo/config.toml
    # Remove clang-specific flags
    sed -i '/fuse-ld=/d' .cargo/config.toml
  fi

  # Also configure user cargo config
  mkdir -p ~/.cargo
  cat >~/.cargo/config.toml <<EOF
[target.x86_64-unknown-linux-gnu]
linker = "gcc"
EOF
fi

echo "Setup complete! Environment is ready for testing."
