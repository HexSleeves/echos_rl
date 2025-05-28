#!/bin/bash
# build.sh - Intelligent Rust build script with Cranelift detection

set -e # Exit on any error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
  echo -e "${GREEN}[INFO]${NC} $1"
}

print_warning() {
  echo -e "${YELLOW}[WARN]${NC} $1"
}

print_error() {
  echo -e "${RED}[ERROR]${NC} $1"
}

# Check if Cranelift is available
check_cranelift() {
  if rustc -Z codegen-backend=cranelift --version >/dev/null 2>&1; then
    return 0 # Success - Cranelift available
  else
    return 1 # Failure - Cranelift not available
  fi
}

# Main build logic
main() {
  print_status "Starting build process..."

  # Parse command line arguments
  BUILD_MODE="dev"
  EXTRA_ARGS=""

  while [[ $# -gt 0 ]]; do
    case $1 in
    --release)
      BUILD_MODE="release"
      EXTRA_ARGS="$EXTRA_ARGS --release"
      shift
      ;;
    --profile)
      BUILD_MODE="$2"
      EXTRA_ARGS="$EXTRA_ARGS --profile $2"
      shift 2
      ;;
    *)
      EXTRA_ARGS="$EXTRA_ARGS $1"
      shift
      ;;
    esac
  done

  print_status "Build mode: $BUILD_MODE"

  # Check Cranelift availability
  if check_cranelift; then
    print_status "Cranelift codegen backend detected and available"

    # Only use Cranelift for dev builds (it's not optimized for release)
    if [[ "$BUILD_MODE" == "dev" ]]; then
      print_status "Using Cranelift for faster dev compilation"

      # Create temporary cargo config to enable Cranelift
      TEMP_CONFIG=$(mktemp)
      cat >"$TEMP_CONFIG" <<'EOF'
[profile.dev]
codegen-backend = "cranelift"

[profile.dev.package."*"]
codegen-backend = "llvm"
EOF

      # Build with Cranelift
      CARGO_CONFIG_PROFILE_DEV_CODEGEN_BACKEND="cranelift" \
        cargo build $EXTRA_ARGS

      # Clean up
      rm -f "$TEMP_CONFIG"
    else
      print_warning "Cranelift available but not used for $BUILD_MODE builds (LLVM is better for optimized builds)"
      cargo build $EXTRA_ARGS
    fi
  else
    print_warning "Cranelift codegen backend not available, using standard LLVM backend"
    print_status "To install Cranelift: rustup component add rustc-codegen-cranelift-preview"

    # Ensure no Cranelift config is used
    CARGO_CONFIG_PROFILE_DEV_CODEGEN_BACKEND="" cargo build $EXTRA_ARGS
  fi

  print_status "Build completed successfully!"
}

# Help function
show_help() {
  cat <<EOF
Usage: $0 [OPTIONS]

Build script with automatic Cranelift detection for faster dev builds.

OPTIONS:
    --release           Build in release mode (disables Cranelift)
    --profile PROFILE   Use specific build profile
    -h, --help         Show this help message

EXAMPLES:
    $0                 # Dev build with Cranelift if available
    $0 --release       # Release build with LLVM
    $0 --profile dist  # Distribution build with LLVM

NOTES:
    - Cranelift is only used for dev builds (faster compilation)
    - Release builds always use LLVM (better optimization)
    - Install Cranelift: rustup component add rustc-codegen-cranelift-preview
EOF
}

# Check for help flag
if [[ "$1" == "-h" || "$1" == "--help" ]]; then
  show_help
  exit 0
fi

# Run main function
main "$@"
