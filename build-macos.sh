#!/bin/bash
set -e

echo "====================================="
echo "Building Airgap library for macOS"
echo "====================================="

# --------------------------------------------------------
# Add macOS targets
# --------------------------------------------------------
echo "Adding macOS targets..."
rustup target add \
  aarch64-apple-darwin \
  x86_64-apple-darwin 2>/dev/null || true

# --------------------------------------------------------
# Clean output directory
# --------------------------------------------------------
OUTPUT_DIR="target/macos"
mkdir -p "${OUTPUT_DIR}"

# --------------------------------------------------------
# Build for each architecture with default features
# --------------------------------------------------------
echo ""
echo "Building for aarch64-apple-darwin (Apple Silicon)..."
cargo build --release --target aarch64-apple-darwin

echo ""
echo "Building for x86_64-apple-darwin (Intel)..."
cargo build --release --target x86_64-apple-darwin

# --------------------------------------------------------
# Create universal binary via lipo
# --------------------------------------------------------
echo ""
echo "Creating universal binary..."
lipo -create \
  "target/aarch64-apple-darwin/release/libairgap.a" \
  "target/x86_64-apple-darwin/release/libairgap.a" \
  -output "${OUTPUT_DIR}/libairgap-macos.a"

echo "  -> ${OUTPUT_DIR}/libairgap-macos.a"

# --------------------------------------------------------
# Copy header
# --------------------------------------------------------
cp include/airgap.h "${OUTPUT_DIR}/"
echo "  -> ${OUTPUT_DIR}/airgap.h"

echo ""
echo "====================================="
echo "MACOS BUILD COMPLETE"
echo "====================================="
echo ""
echo "Output files in ${OUTPUT_DIR}/:"
ls -la "${OUTPUT_DIR}/"
echo ""
echo "Usage with LVGL simulator:"
echo "  1. Add ${OUTPUT_DIR}/airgap.h to your include path"
echo "  2. Link ${OUTPUT_DIR}/libairgap-macos.a"
echo "  3. See airgap.h for the full C API"
echo ""
echo "Available functions:"
echo "  - airgap_encoder_new()              (full features)"
echo "  - airgap_encoder_new_with_session_id()"
echo "  - airgap_encoder_get_qr_string()"
echo "  - airgap_encoder_generate_png()     (QR code PNG generation)"
echo "  - airgap_decoder_*()               (all decoder functions)"
echo ""
