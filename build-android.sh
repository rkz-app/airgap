#!/bin/bash
set -e

echo "Building Airgap library for Android..."

# Add Android targets if not already added
echo "Adding Android targets..."
rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android 2>/dev/null || true

# Set up Android NDK path (update this to your NDK location)
if [ -z "$ANDROID_NDK_HOME" ]; then
    echo "Error: ANDROID_NDK_HOME environment variable not set"
    echo "Please set it to your Android NDK installation path"
    echo "Example: export ANDROID_NDK_HOME=~/Library/Android/sdk/ndk/26.1.10909125"
    exit 1
fi

# Create cargo config for Android
mkdir -p .cargo
cat > .cargo/config.toml << EOF
[target.aarch64-linux-android]
ar = "$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/darwin-x86_64/bin/llvm-ar"
linker = "$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/darwin-x86_64/bin/aarch64-linux-android30-clang"

[target.armv7-linux-androideabi]
ar = "$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/darwin-x86_64/bin/llvm-ar"
linker = "$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/darwin-x86_64/bin/armv7a-linux-androideabi30-clang"

[target.i686-linux-android]
ar = "$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/darwin-x86_64/bin/llvm-ar"
linker = "$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/darwin-x86_64/bin/i686-linux-android30-clang"

[target.x86_64-linux-android]
ar = "$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/darwin-x86_64/bin/llvm-ar"
linker = "$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/darwin-x86_64/bin/x86_64-linux-android30-clang"
EOF

# Build for Android targets
echo "Building for arm64-v8a..."
cargo build --release --target aarch64-linux-android

echo "Building for armeabi-v7a..."
cargo build --release --target armv7-linux-androideabi

echo "Building for x86..."
cargo build --release --target i686-linux-android

echo "Building for x86_64..."
cargo build --release --target x86_64-linux-android

# Create jniLibs directory structure
echo "Creating jniLibs directory..."
mkdir -p android/airgap/src/main/jniLibs/arm64-v8a
mkdir -p android/airgap/src/main/jniLibs/armeabi-v7a
mkdir -p android/airgap/src/main/jniLibs/x86
mkdir -p android/airgap/src/main/jniLibs/x86_64

# Copy libraries
cp target/aarch64-linux-android/release/libairgap.so android/airgap/src/main/jniLibs/arm64-v8a/
cp target/armv7-linux-androideabi/release/libairgap.so android/airgap/src/main/jniLibs/armeabi-v7a/
cp target/i686-linux-android/release/libairgap.so android/airgap/src/main/jniLibs/x86/
cp target/x86_64-linux-android/release/libairgap.so android/airgap/src/main/jniLibs/x86_64/

echo ""
echo "Native libraries built successfully!"
echo ""

# Build AAR using Gradle
echo "Building AAR with Gradle..."
cd android

# Make gradlew executable if it exists
if [ -f "./gradlew" ]; then
    chmod +x ./gradlew
    ./gradlew :airgap:assembleRelease
else
    echo "gradlew not found, using system gradle..."
    gradle :airgap:assembleRelease
fi

cd ..

echo ""
echo "Build complete!"
echo ""
echo "AAR file: android/airgap/build/outputs/aar/airgap-release.aar"
echo "Native libraries: android/airgap/src/main/jniLibs/"
echo "Kotlin source: android/airgap/src/main/kotlin/"
echo ""