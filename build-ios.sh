#!/bin/bash
set -e

TARGET=12.0

export IPHONEOS_DEPLOYMENT_TARGET=$TARGET
export RUSTFLAGS="-C link-arg=-miphoneos-version-min=${TARGET}"

echo "Building dynamic Airgap framework..."

rustup target add aarch64-apple-ios aarch64-apple-ios-sim x86_64-apple-ios 2>/dev/null || true

# Build Rust dylibs
cargo build --release --target aarch64-apple-ios
cargo build --release --target aarch64-apple-ios-sim
cargo build --release --target x86_64-apple-ios

build_dynamic() {
    local arch=$1
    local sdk=$2
    local rust_static=$3
    local output_dylib=$4

    if [ "$sdk" = "iphoneos" ]; then
        MIN_FLAG="-miphoneos-version-min=${TARGET}"
    else
        MIN_FLAG="-mios-simulator-version-min=${TARGET}"
    fi

    local temp_dir="target/temp_${arch}"
    mkdir -p "${temp_dir}"

    echo "Compiling ObjC for ${arch}..."

    xcrun -sdk ${sdk} clang -c \
        -arch ${arch} \
        ${MIN_FLAG} \
        -fobjc-arc \
        -fmodules \
        -I./include \
        -I./objc \
        objc/AGQRResult.m \
        -o "${temp_dir}/AGQRResult.o"

    xcrun -sdk ${sdk} clang -c \
        -arch ${arch} \
        ${MIN_FLAG} \
        -fobjc-arc \
        -fmodules \
        -I./include \
        -I./objc \
        objc/AGEncoder.m \
        -o "${temp_dir}/AGEncoder.o"

    xcrun -sdk ${sdk} clang -c \
        -arch ${arch} \
        ${MIN_FLAG} \
        -fobjc-arc \
        -fmodules \
        -I./include \
        -I./objc \
        objc/AGDecoder.m \
        -o "${temp_dir}/AGDecoder.o"

    echo "Linking dynamic library for ${arch}..."

    xcrun -sdk ${sdk} clang \
        -arch ${arch} \
        -dynamiclib \
        ${MIN_FLAG} \
        -install_name @rpath/Airgap.framework/Airgap \
        -framework Foundation \
        -o "${output_dylib}" \
        "${rust_static}" \
        "${temp_dir}/AGQRResult.o" \
        "${temp_dir}/AGEncoder.o" \
        "${temp_dir}/AGDecoder.o" \
        -Wl,-force_load,"${rust_static}"
}

mkdir -p target/dynamic

# Device
build_dynamic \
    arm64 \
    iphoneos \
    target/aarch64-apple-ios/release/libairgap.a \
    target/dynamic/Airgap-device

# Simulator arm64
build_dynamic \
    arm64 \
    iphonesimulator \
    target/aarch64-apple-ios-sim/release/libairgap.a \
    target/dynamic/Airgap-sim-arm64

# Simulator x86_64
build_dynamic \
    x86_64 \
    iphonesimulator \
    target/x86_64-apple-ios/release/libairgap.a \
    target/dynamic/Airgap-sim-x86_64

# Create universal simulator dylib
lipo -create \
    target/dynamic/Airgap-sim-arm64 \
    target/dynamic/Airgap-sim-x86_64 \
    -output target/dynamic/Airgap-simulator

create_framework() {
    local dylib_path=$1
    local output_dir=$2

    local framework_dir="${output_dir}/Airgap.framework"
    rm -rf "${framework_dir}"
    mkdir -p "${framework_dir}/Headers"
    mkdir -p "${framework_dir}/Modules"

    cp "${dylib_path}" "${framework_dir}/Airgap"

    cp objc/*.h "${framework_dir}/Headers/"

    cat > "${framework_dir}/Headers/Airgap.h" <<EOF
#import <Foundation/Foundation.h>
#import "AGQRResult.h"
#import "AGEncoder.h"
#import "AGDecoder.h"
EOF

    cat > "${framework_dir}/Modules/module.modulemap" <<EOF
framework module Airgap {
    umbrella header "Airgap.h"
    export *
    module * { export * }
}
EOF

cat > "${framework_dir}/Info.plist" <<EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN"
 "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleExecutable</key>
    <string>Airgap</string>
    <key>CFBundleIdentifier</key>
    <string>app.rkz.Airgap</string>
    <key>CFBundlePackageType</key>
    <string>FMWK</string>
    <key>MinimumOSVersion</key>
    <string>${TARGET}</string>
</dict>
</plist>
EOF
}

mkdir -p target/frameworks/device
mkdir -p target/frameworks/simulator

create_framework target/dynamic/Airgap-device target/frameworks/device
create_framework target/dynamic/Airgap-simulator target/frameworks/simulator

rm -rf Airgap.xcframework

xcodebuild -create-xcframework \
    -framework target/frameworks/device/Airgap.framework \
    -framework target/frameworks/simulator/Airgap.framework \
    -output Airgap.xcframework

echo "Dynamic XCFramework created."