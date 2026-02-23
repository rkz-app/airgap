#!/bin/bash
set -e

echo "====================================="
echo "Building Airgap library for Android"
echo "====================================="

# --------------------------------------------------------
# Add Android targets
# --------------------------------------------------------
rustup target add \
  aarch64-linux-android \
  armv7-linux-androideabi \
  i686-linux-android \
  x86_64-linux-android 2>/dev/null || true

if [ -z "$ANDROID_NDK_HOME" ]; then
    echo "Error: ANDROID_NDK_HOME not set"
    exit 1
fi

# --------------------------------------------------------
# Detect NDK host correctly
# --------------------------------------------------------
if [[ "$OSTYPE" == "darwin"* ]]; then
    if [[ "$(uname -m)" == "arm64" ]]; then
        HOST_TAG="darwin-arm64"
    else
        HOST_TAG="darwin-x86_64"
    fi
elif [[ "$OSTYPE" == "linux"* ]]; then
    HOST_TAG="linux-x86_64"
else
    echo "Unsupported host: $OSTYPE"
    exit 1
fi

echo "Using NDK host: $HOST_TAG"

CLANG="$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/$HOST_TAG/bin"

# --------------------------------------------------------
# Configure Cargo (only if not already configured)
# --------------------------------------------------------
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

# --------------------------------------------------------
# Build native libraries
# --------------------------------------------------------
echo ""
echo "Building arm64-v8a..."
cargo build --release --target aarch64-linux-android

echo "Building armeabi-v7a..."
cargo build --release --target armv7-linux-androideabi

echo "Building x86..."
cargo build --release --target i686-linux-android

echo "Building x86_64..."
cargo build --release --target x86_64-linux-android

# --------------------------------------------------------
# Copy to jniLibs
# --------------------------------------------------------
mkdir -p android/airgap/src/main/jniLibs/{arm64-v8a,armeabi-v7a,x86,x86_64}

cp target/aarch64-linux-android/release/libairgap.so android/airgap/src/main/jniLibs/arm64-v8a/
cp target/armv7-linux-androideabi/release/libairgap.so android/airgap/src/main/jniLibs/armeabi-v7a/
cp target/i686-linux-android/release/libairgap.so android/airgap/src/main/jniLibs/x86/
cp target/x86_64-linux-android/release/libairgap.so android/airgap/src/main/jniLibs/x86_64/

# --------------------------------------------------------
# Verify 16KB alignment (arm64)
# --------------------------------------------------------
if [ -f "$CLANG/llvm-readelf" ]; then
    echo ""
    echo "Checking 16KB alignment (arm64)..."
    "$CLANG/llvm-readelf" -l android/airgap/src/main/jniLibs/arm64-v8a/libairgap.so | grep -i align || true
fi

echo ""
echo "Native libraries built successfully."

# --------------------------------------------------------
# Build AAR
# --------------------------------------------------------
echo ""
echo "Building AAR..."

cd android

if [ -f "./gradlew" ]; then
    chmod +x ./gradlew
    ./gradlew :airgap:assembleRelease
else
    gradle :airgap:assembleRelease
fi

cd ..

echo ""
echo "====================================="
echo "AAR BUILD COMPLETE"
echo "====================================="
echo ""
echo "AAR file:"
echo "android/airgap/build/outputs/aar/airgap-release.aar"
echo ""