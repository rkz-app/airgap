#!/bin/bash
set -e

echo "🧪 Building and running Kotlin/JVM tests..."

# Build Rust library as cdylib for current platform
echo "Building Rust library for host platform..."
cargo build --release --lib

# Detect platform-specific library extension
if [[ "$OSTYPE" == "darwin"* ]]; then
    LIB_EXT="dylib"
    LIB_PREFIX="lib"
elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
    LIB_EXT="so"
    LIB_PREFIX="lib"
else
    echo "Unsupported platform: $OSTYPE"
    exit 1
fi

# Find the Rust library
RUST_LIB="target/release/${LIB_PREFIX}airgap.${LIB_EXT}"
if [ ! -f "$RUST_LIB" ]; then
    echo "Error: Rust library not found at $RUST_LIB"
    exit 1
fi

# Set up directories
KOTLIN_SRC="android/airgap/src/main/kotlin"
TEST_SRC="android/airgap/src/test/kotlin"
BUILD_DIR="target/kotlin-test"
CLASSES_DIR="$BUILD_DIR/classes"
LIB_DIR="$BUILD_DIR/lib"

mkdir -p "$CLASSES_DIR"
mkdir -p "$LIB_DIR"

# Copy Rust library to lib directory
cp "$RUST_LIB" "$LIB_DIR/"

# Check if kotlinc is available
if ! command -v kotlinc &> /dev/null; then
    echo "Error: kotlinc not found. Please install Kotlin compiler."
    echo "You can install it via:"
    echo "  brew install kotlin (macOS)"
    echo "  sdk install kotlin (SDKMan)"
    exit 1
fi

# Compile Kotlin sources
echo "Compiling Kotlin sources..."
kotlinc \
    "$KOTLIN_SRC/app/rkz/airgap/AirgapException.kt" \
    "$KOTLIN_SRC/app/rkz/airgap/QRResult.kt" \
    "$KOTLIN_SRC/app/rkz/airgap/AirgapEncoder.kt" \
    "$KOTLIN_SRC/app/rkz/airgap/AirgapDecoder.kt" \
    "$TEST_SRC/app/rkz/airgap/AirgapTests.kt" \
    -include-runtime \
    -d "$BUILD_DIR/airgap-tests.jar"

echo ""
echo "Running tests..."
echo ""

# Run tests with library path
java -Djava.library.path="$LIB_DIR" \
     -jar "$BUILD_DIR/airgap-tests.jar"

TEST_RESULT=$?

# Cleanup
echo ""
echo "Cleaning up..."
rm -rf "$BUILD_DIR"

if [ $TEST_RESULT -eq 0 ]; then
    echo "✅ Done!"
    exit 0
else
    echo "❌ Tests failed!"
    exit $TEST_RESULT
fi