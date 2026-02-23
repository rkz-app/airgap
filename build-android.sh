#!/bin/bash
set -e

echo "Building Airgap library for Android..."

# ---------------------------------------------------------
# Verify NDK
# ---------------------------------------------------------
if [ -z "$ANDROID_NDK_HOME" ]; then
    echo "ANDROID_NDK_HOME not set"
    echo "Example:"
    echo "export ANDROID_NDK_HOME=~/Library/Android/sdk/ndk/27.0.12077973"
    exit 1
fi

# ---------------------------------------------------------
# Detect NDK host (Apple Silicon safe)
# ---------------------------------------------------------
if [[ "$OSTYPE" == "darwin"* ]]; then
    NDK_HOST="darwin-x86_64"
elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
    NDK_HOST="linux-x86_64"
else
    echo "Unsupported host: $OSTYPE"
    exit 1
fi

echo "Using NDK host: $NDK_HOST"

# ---------------------------------------------------------
# Add targets
# ---------------------------------------------------------
rustup target add \
aarch64-linux-android \
armv7-linux-androideabi \
i686-linux-android \
x86_64-linux-android 2>/dev/null || true

# ---------------------------------------------------------
# Isolated Android target directory
# ---------------------------------------------------------
export CARGO_TARGET_DIR=target-android

# ---------------------------------------------------------
# Common linker flags (16KB page size)
# ---------------------------------------------------------
RUSTFLAGS="-C link-arg=-Wl,-z,max-page-size=16384"

# ---------------------------------------------------------
# Build arm64 (primary ABI)
# ---------------------------------------------------------
echo "Building arm64-v8a..."
CC_aarch64_linux_android=$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/$NDK_HOST/bin/aarch64-linux-android21-clang \
AR_aarch64_linux_android=$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/$NDK_HOST/bin/llvm-ar \
RUSTFLAGS="$RUSTFLAGS" \
cargo build --release --target aarch64-linux-android

# ---------------------------------------------------------
# Build armeabi-v7a
# ---------------------------------------------------------
echo "Building armeabi-v7a..."
CC_armv7_linux_androideabi=$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/$NDK_HOST/bin/armv7a-linux-androideabi21-clang \
AR_armv7_linux_androideabi=$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/$NDK_HOST/bin/llvm-ar \
RUSTFLAGS="$RUSTFLAGS" \
cargo build --release --target armv7-linux-androideabi

# ---------------------------------------------------------
# Build x86
# ---------------------------------------------------------
echo "Building x86..."
CC_i686_linux_android=$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/$NDK_HOST/bin/i686-linux-android21-clang \
AR_i686_linux_android=$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/$NDK_HOST/bin/llvm-ar \
RUSTFLAGS="$RUSTFLAGS" \
cargo build --release --target i686-linux-android

# ---------------------------------------------------------
# Build x86_64
# ---------------------------------------------------------
echo "Building x86_64..."
CC_x86_64_linux_android=$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/$NDK_HOST/bin/x86_64-linux-android21-clang \
AR_x86_64_linux_android=$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/$NDK_HOST/bin/llvm-ar \
RUSTFLAGS="$RUSTFLAGS" \
cargo build --release --target x86_64-linux-android

# ---------------------------------------------------------
# Copy into jniLibs
# ---------------------------------------------------------
JNI_DIR=android/airgap/src/main/jniLibs

mkdir -p \
$JNI_DIR/arm64-v8a \
$JNI_DIR/armeabi-v7a \
$JNI_DIR/x86 \
$JNI_DIR/x86_64

cp target-android/aarch64-linux-android/release/libairgap.so $JNI_DIR/arm64-v8a/
cp target-android/armv7-linux-androideabi/release/libairgap.so $JNI_DIR/armeabi-v7a/
cp target-android/i686-linux-android/release/libairgap.so $JNI_DIR/x86/
cp target-android/x86_64-linux-android/release/libairgap.so $JNI_DIR/x86_64/

# ---------------------------------------------------------
# Verify 16KB alignment (arm64 only)
# ---------------------------------------------------------
echo ""
echo "Verifying 16KB page alignment..."

$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/$NDK_HOST/bin/llvm-readelf \
-l $JNI_DIR/arm64-v8a/libairgap.so

# ---------------------------------------------------------
# Build AAR
# ---------------------------------------------------------
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
echo "✅ Android build complete"
echo "AAR:"
echo "android/airgap/build/outputs/aar/airgap-release.aar"