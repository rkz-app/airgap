#!/bin/bash
set -e

echo "🧪 Building and running ObjC tests..."

# Build Rust library first
echo "Building Rust library..."
cargo build --release

# Create temp directory for object files
TEMP_DIR="target/objc-test-temp"
mkdir -p "$TEMP_DIR"

# Compile ObjC sources
echo "Compiling ObjC sources..."

# Compile AGQRResult.m
clang -c \
    -I./include \
    -I./objc \
    -fobjc-arc \
    -fmodules \
    -o "$TEMP_DIR/AGQRResult.o" \
    objc/AGQRResult.m

# Compile AGEncoder.m
clang -c \
    -I./include \
    -I./objc \
    -fobjc-arc \
    -fmodules \
    -o "$TEMP_DIR/AGEncoder.o" \
    objc/AGEncoder.m

# Compile AGDecoder.m
clang -c \
    -I./include \
    -I./objc \
    -fobjc-arc \
    -fmodules \
    -o "$TEMP_DIR/AGDecoder.o" \
    objc/AGDecoder.m

# Compile test file
clang -c \
    -I./include \
    -I./objc \
    -fobjc-arc \
    -fmodules \
    -o "$TEMP_DIR/AirgapTests.o" \
    objc/Tests/AirgapTests.m

# Link everything together
echo "Linking test executable..."
clang \
    -fobjc-arc \
    -fobjc-link-runtime \
    -framework Foundation \
    -L./target/release \
    -lairgap \
    "$TEMP_DIR/AGQRResult.o" \
    "$TEMP_DIR/AGEncoder.o" \
    "$TEMP_DIR/AGDecoder.o" \
    "$TEMP_DIR/AirgapTests.o" \
    -o "$TEMP_DIR/AirgapTests"

# Run tests
echo ""
echo "Running tests..."
"$TEMP_DIR/AirgapTests"

# Cleanup
echo ""
echo "Cleaning up..."
rm -rf "$TEMP_DIR"

echo "✅ Done!"