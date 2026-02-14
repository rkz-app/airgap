#!/bin/bash
set -e

echo "🧪 Running Airgap Kotlin Tests"
echo "================================"

# Build the library for macOS (for JVM testing)
echo "Building native library for macOS..."
cd ..

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

# Build for macOS
cargo build --release --target $TARGET > /dev/null 2>&1
cd android

# Compile classes using gradle
echo "Compiling Kotlin classes with gradle..."
./gradlew :airgap:compileDebugKotlin > /dev/null 2>&1
./gradlew :airgap:compileDebugUnitTestKotlin > /dev/null 2>&1

# Find all Kotlin jars in gradle cache
KOTLIN_JARS=$(find ~/.gradle/caches -name "kotlin-stdlib*.jar" -o -name "kotlin-test*.jar" | tr '\n' ':')
if [ -z "$KOTLIN_JARS" ]; then
    echo "Error: Could not find Kotlin jars in gradle cache"
    exit 1
fi

# Set up classpath
MAIN_CLASSES="airgap/build/tmp/kotlin-classes/debug"
TEST_CLASSES="airgap/build/tmp/kotlin-classes/debugUnitTest"

# Use the macOS target directory
NATIVE_LIB_PATH="../target/$TARGET/release"

# Run the tests
echo "Running tests..."
echo ""

java \
    -Djava.library.path="$NATIVE_LIB_PATH" \
    -classpath "$MAIN_CLASSES:$TEST_CLASSES:$KOTLIN_JARS" \
    app.rkz.airgap.AirgapTestsKt

echo ""
echo "================================"
echo "✅ All Kotlin tests completed!"
