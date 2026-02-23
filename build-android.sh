#!/bin/bash
set -e

echo "Building Airgap library for Android..."

# Add Android targets
rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android 2>/dev/null || true

if [ -z "$ANDROID_NDK_HOME" ]; then
    echo "Error: ANDROID_NDK_HOME not set"
    exit 1
fi

# Detect host platform properly
HOST_TAG=$(basename "$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/"*)
echo "Using NDK host: $HOST_TAG"

CLANG="$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/$HOST_TAG/bin"

# Ensure .cargo exists but DO NOT overwrite existing config
mkdir -p .cargo

if [ ! -f ".cargo/config.toml" ]; then
cat > .cargo/config.toml <<EOF
[target.aarch64-linux-android]
linker = "$CLANG/aarch64-linux-android21-clang"
rustflags = ["-C", "link-arg=-Wl,-z,max-page-size=16384"]

[target.armv7-linux-androideabi]
linker = "$CLANG/armv7a-linux-androideabi21-clang"
rustflags = ["-C", "link-arg=-Wl,-z,max-page-size=16384"]

[target.i686-linux-android]
linker = "$CLANG/i686-linux-android21-clang"

[target.x86_64-linux-android]
linker = "$CLANG/x86_64-linux-android21-clang"
EOF
fi

# Build
echo "Building arm64-v8a..."
cargo build --release --target aarch64-linux-android

echo "Building armeabi-v7a..."
cargo build --release --target armv7-linux-androideabi

echo "Building x86..."
cargo build --release --target i686-linux-android

echo "Building x86_64..."
cargo build --release --target x86_64-linux-android

# Create jniLibs
mkdir -p android/airgap/src/main/jniLibs/{arm64-v8a,armeabi-v7a,x86,x86_64}

cp target/aarch64-linux-android/release/libairgap.so android/airgap/src/main/jniLibs/arm64-v8a/
cp target/armv7-linux-androideabi/release/libairgap.so android/airgap/src/main/jniLibs/armeabi-v7a/
cp target/i686-linux-android/release/libairgap.so android/airgap/src/main/jniLibs/x86/
cp target/x86_64-linux-android/release/libairgap.so android/airgap/src/main/jniLibs/x86_64/

# Verify 16KB page alignment (arm64 only)
if [ -f "$CLANG/llvm-readelf" ]; then
    echo ""
    echo "Checking arm64 alignment..."
    "$CLANG/llvm-readelf" -l android/airgap/src/main/jniLibs/arm64-v8a/libairgap.so | grep -i align || true
fi

echo ""
echo "Android native libraries built successfully."