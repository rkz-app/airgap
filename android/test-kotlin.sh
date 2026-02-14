#!/bin/bash
set -e

echo "🧪 Airgap Kotlin/JVM Test Runner"
echo "=================================="
echo ""

# Determine target based on macOS architecture
ARCH=$(uname -m)
if [ "$ARCH" = "arm64" ]; then
    TARGET="aarch64-apple-darwin"
elif [ "$ARCH" = "x86_64" ]; then
    TARGET="x86_64-apple-darwin"
else
    echo "Unsupported architecture: $ARCH"
    exit 1
fi

# Build the library for macOS (JNI now works on all JVM targets!)
echo "Building native library for macOS ($TARGET)..."
cd ..
cargo build --release --target $TARGET
cd android

# Create jniLibs directory for tests
JNILIBS_DIR="airgap/src/test/jniLibs"
mkdir -p "$JNILIBS_DIR"

# Copy the dylib to jniLibs (JVM will look for libairgap.dylib)
echo "Copying native library to test jniLibs..."
cp "../target/$TARGET/release/libairgap.dylib" "$JNILIBS_DIR/"

# Compile classes using gradle
echo "Compiling Kotlin classes with gradle..."
./gradlew :airgap:compileDebugKotlin :airgap:compileDebugUnitTestKotlin --quiet

# Find all Kotlin jars in gradle cache
KOTLIN_JARS=$(find ~/.gradle/caches -name "kotlin-stdlib*.jar" -o -name "kotlin-test*.jar" | tr '\n' ':')
if [ -z "$KOTLIN_JARS" ]; then
    echo "Error: Could not find Kotlin jars in gradle cache"
    exit 1
fi

# Set up classpath
MAIN_CLASSES="airgap/build/tmp/kotlin-classes/debug"
TEST_CLASSES="airgap/build/tmp/kotlin-classes/debugUnitTest"

# Run the tests using the main() function in AirgapTests.kt
echo "Running Kotlin/JVM tests..."
echo ""

java \
    -Djava.library.path="$JNILIBS_DIR" \
    -classpath "$MAIN_CLASSES:$TEST_CLASSES:$KOTLIN_JARS" \
    app.rkz.airgap.AirgapTestsKt

echo ""
echo "================================"
echo "✅ All Kotlin/JVM tests passed!"
echo ""
