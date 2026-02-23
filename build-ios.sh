#!/bin/bash
set -e

echo "Building Airgap framework for iOS..."

# Add iOS targets if not already added
echo "Adding iOS targets..."
rustup target add aarch64-apple-ios aarch64-apple-ios-sim x86_64-apple-ios 2>/dev/null || true

# Build for iOS device (arm64)
echo "Building for iOS device (arm64)..."
cargo build --release --target aarch64-apple-ios --lib

# Build for iOS simulator (arm64)
echo "Building for iOS simulator (arm64)..."
cargo build --release --target aarch64-apple-ios-sim --lib

# Build for iOS simulator (x86_64)
echo "Building for iOS simulator (x86_64)..."
cargo build --release --target x86_64-apple-ios --lib

# Helper function to compile ObjC wrapper and merge with Rust static lib
compile_and_merge_objc() {
    local arch=$1
    local sdk=$2
    local rust_static_lib=$3
    local output_static_lib=$4

    echo "Compiling ObjC wrapper for ${arch} and merging into static library..."

    local temp_dir="target/temp_objc_${arch}"
    mkdir -p "${temp_dir}"

    # Compile AGQRResult.m
    xcrun -sdk ${sdk} clang -c \
        -arch ${arch} \
        -I./include \
        -I./objc \
        -fobjc-arc \
        -fmodules \
        -o "${temp_dir}/AGQRResult.o" \
        objc/AGQRResult.m

    # Compile AGEncoder.m
    xcrun -sdk ${sdk} clang -c \
        -arch ${arch} \
        -I./include \
        -I./objc \
        -fobjc-arc \
        -fmodules \
        -o "${temp_dir}/AGEncoder.o" \
        objc/AGEncoder.m

    # Compile AGDecoder.m
    xcrun -sdk ${sdk} clang -c \
        -arch ${arch} \
        -I./include \
        -I./objc \
        -fobjc-arc \
        -fmodules \
        -o "${temp_dir}/AGDecoder.o" \
        objc/AGDecoder.m

    # Create static library combining Rust static lib and ObjC object files
    # Use libtool to create the combined static library
    xcrun -sdk ${sdk} libtool -static \
        -o "${output_static_lib}" \
        "${rust_static_lib}" \
        "${temp_dir}/AGQRResult.o" \
        "${temp_dir}/AGEncoder.o" \
        "${temp_dir}/AGDecoder.o"

    # Clean up temp files
    rm -rf "${temp_dir}"
}

# Helper function to create framework
create_framework() {
    local framework_name=$1
    local static_lib_path=$2
    local output_dir=$3

    echo "Creating framework: ${framework_name}"

    local framework_dir="${output_dir}/${framework_name}.framework"
    rm -rf "${framework_dir}"
    mkdir -p "${framework_dir}/Headers"

    # Copy static library and rename to framework name
    cp "${static_lib_path}" "${framework_dir}/${framework_name}"

    # Copy C headers
    cp -r include/* "${framework_dir}/Headers/"

    # Copy ObjC headers
    cp objc/AGQRResult.h "${framework_dir}/Headers/"
    cp objc/AGEncoder.h "${framework_dir}/Headers/"
    cp objc/AGDecoder.h "${framework_dir}/Headers/"

    # Create umbrella header
    cat > "${framework_dir}/Headers/Airgap.h" <<UMBRELLA
//
//  Airgap.h
//  Airgap
//

#import <Foundation/Foundation.h>

#import "AGQRResult.h"
#import "AGEncoder.h"
#import "AGDecoder.h"
UMBRELLA

    # Create Modules directory and modulemap
    mkdir -p "${framework_dir}/Modules"
    cat > "${framework_dir}/Modules/module.modulemap" <<MODULEMAP
framework module ${framework_name} {
    umbrella header "Airgap.h"
    export *
    module * { export * }
}
MODULEMAP

    # Create Info.plist
    cat > "${framework_dir}/Info.plist" <<EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleDevelopmentRegion</key>
    <string>en</string>
    <key>CFBundleExecutable</key>
    <string>${framework_name}</string>
    <key>CFBundleIdentifier</key>
    <string>app.rkz.${framework_name}</string>
    <key>CFBundleInfoDictionaryVersion</key>
    <string>6.0</string>
    <key>CFBundleName</key>
    <string>${framework_name}</string>
    <key>CFBundlePackageType</key>
    <string>FMWK</string>
    <key>CFBundleShortVersionString</key>
    <string>0.1.0</string>
    <key>CFBundleVersion</key>
    <string>1</string>
    <key>MinimumOSVersion</key>
    <string>12.0</string>
</dict>
</plist>
EOF
}

# Compile ObjC wrappers and merge with Rust static libs
echo "Compiling ObjC wrappers and creating combined static libraries..."
mkdir -p target/combined-static

# Device (arm64)
compile_and_merge_objc \
    "arm64" \
    "iphoneos" \
    "target/aarch64-apple-ios/release/libairgap.a" \
    "target/combined-static/libairgap-device.a"

# Simulator arm64
compile_and_merge_objc \
    "arm64" \
    "iphonesimulator" \
    "target/aarch64-apple-ios-sim/release/libairgap.a" \
    "target/combined-static/libairgap-sim-arm64.a"

# Simulator x86_64
compile_and_merge_objc \
    "x86_64" \
    "iphonesimulator" \
    "target/x86_64-apple-ios/release/libairgap.a" \
    "target/combined-static/libairgap-sim-x86_64.a"

# Create universal simulator static library
echo "Creating universal simulator static library..."
lipo -create \
    target/combined-static/libairgap-sim-arm64.a \
    target/combined-static/libairgap-sim-x86_64.a \
    -output target/combined-static/libairgap-simulator.a

# Create framework directories
echo "Creating frameworks..."
mkdir -p target/frameworks/device
mkdir -p target/frameworks/simulator

# Create frameworks
create_framework "Airgap" "target/combined-static/libairgap-device.a" "target/frameworks/device"
create_framework "Airgap" "target/combined-static/libairgap-simulator.a" "target/frameworks/simulator"

# Create XCFramework
echo "Creating XCFramework..."
rm -rf Airgap.xcframework
xcodebuild -create-xcframework \
    -framework target/frameworks/device/Airgap.framework \
    -framework target/frameworks/simulator/Airgap.framework \
    -output Airgap.xcframework

echo "XCFramework created at Airgap.xcframework"