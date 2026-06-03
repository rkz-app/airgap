#!/bin/bash
set -e

echo "====================================="
echo "Building Airgap library for Embedded"
echo "====================================="

# --------------------------------------------------------
# Add ARM Cortex-M targets
# --------------------------------------------------------
# thumbv7em-none-eabihf: Cortex-M4F, M7 (STM32F4/F7/H7/G4/L4 - most common)
# thumbv7m-none-eabi:    Cortex-M3 (STM32F1/F2)
# thumbv6m-none-eabi:    Cortex-M0, M0+ (STM32F0/G0)
echo "Adding embedded targets..."
rustup target add \
  thumbv7em-none-eabihf \
  thumbv7m-none-eabi \
  thumbv6m-none-eabi 2>/dev/null || true

# --------------------------------------------------------
# Clean output directory
# --------------------------------------------------------
OUTPUT_DIR="target/embedded"
mkdir -p "${OUTPUT_DIR}"

# --------------------------------------------------------
# Build for each target with no-default-features
# --------------------------------------------------------
build_target() {
    local target=$1
    local triple=$2
    echo ""
    echo "Building for ${target}..."
    cargo build --release --no-default-features --target "${triple}"
    cp "target/${triple}/release/libairgap.a" "${OUTPUT_DIR}/libairgap-${target}.a"
    echo "  -> ${OUTPUT_DIR}/libairgap-${target}.a"
}

build_target "armv7em"  "thumbv7em-none-eabihf"
build_target "armv7m"   "thumbv7m-none-eabi"
build_target "armv6m"   "thumbv6m-none-eabi"

# --------------------------------------------------------
# Copy header (embedded-unavailable functions are guarded
# by #ifndef AIRGAP_EMBEDDED — see build.rs)
# --------------------------------------------------------
cp include/airgap.h "${OUTPUT_DIR}/"

echo ""
echo "====================================="
echo "EMBEDDED BUILD COMPLETE"
echo "====================================="
echo ""
echo "Output files in ${OUTPUT_DIR}/:"
ls -la "${OUTPUT_DIR}/"
echo ""
echo "Usage in STM32CubeIDE:"
echo "  1. Add ${OUTPUT_DIR}/airgap.h to your include path"
echo "  2. Add the appropriate .a file to your linker libraries"
echo "  3. Provide a global allocator in your firmware"
echo "  4. See airgap.h for the full C API"
echo ""
echo "Available static libraries:"
echo "  libairgap-armv7em.a  - Cortex-M4F, M7 (STM32F4/F7/H7)"
echo "  libairgap-armv7m.a   - Cortex-M3 (STM32F1/F2)"
echo "  libairgap-armv6m.a   - Cortex-M0, M0+ (STM32F0/G0)"
echo ""
