#!/bin/bash
set -e

echo "Building Airgap dynamic framework for iOS..."

rustup target add aarch64-apple-ios aarch64-apple-ios-sim x86_64-apple-ios 2>/dev/null || true

# Clean to avoid old static contamination
cargo clean

# ---------------------------------------------------------
# Build cdylib for device
# ---------------------------------------------------------
cargo build --release --target aarch64-apple-ios

# ---------------------------------------------------------
# Build cdylib for simulator (arm64)
# ---------------------------------------------------------
cargo build --release --target aarch64-apple-ios-sim

# ---------------------------------------------------------
# Build cdylib for simulator (x86_64)
# ---------------------------------------------------------
cargo build --release --target x86_64-apple-ios

# ---------------------------------------------------------
# Create frameworks
# ---------------------------------------------------------
rm -rf build
mkdir -p build/device build/simulator

create_framework() {
    local dylib_path=$1
    local output_dir=$2

    mkdir -p "$output_dir/Airgap.framework/Headers"
    mkdir -p "$output_dir/Airgap.framework/Modules"

    cp "$dylib_path" "$output_dir/Airgap.framework/Airgap"
    cp include/airgap.h "$output_dir/Airgap.framework/Headers/"

    cat > "$output_dir/Airgap.framework/Modules/module.modulemap" <<EOF
framework module Airgap {
    umbrella header "airgap.h"
    export *
    module * { export * }
}
EOF

    cat > "$output_dir/Airgap.framework/Info.plist" <<EOF
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
    <string>12.0</string>
</dict>
</plist>
EOF
}

# Device
create_framework \
target/aarch64-apple-ios/release/libairgap.dylib \
build/device

# Simulator (merge both arches)
lipo -create \
target/aarch64-apple-ios-sim/release/libairgap.dylib \
target/x86_64-apple-ios/release/libairgap.dylib \
-output build/simulator/libairgap_sim.dylib

create_framework \
build/simulator/libairgap_sim.dylib \
build/simulator

# ---------------------------------------------------------
# Create XCFramework
# ---------------------------------------------------------
rm -rf Airgap.xcframework

xcodebuild -create-xcframework \
-framework build/device/Airgap.framework \
-framework build/simulator/Airgap.framework \
-output Airgap.xcframework

echo "✅ XCFramework created"