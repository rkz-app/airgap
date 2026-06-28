#!/bin/bash
set -e

UNAME_S="$(uname -s)"
OUTPUT_DIR="target/sim"
mkdir -p "${OUTPUT_DIR}"

echo "====================================="
echo "Building Airgap simulator for ${UNAME_S}"
echo "====================================="

case "${UNAME_S}" in
  Darwin)
    rustup target add aarch64-apple-darwin x86_64-apple-darwin 2>/dev/null || true

    cargo build --release --no-default-features --target aarch64-apple-darwin
    cargo build --release --no-default-features --target x86_64-apple-darwin

    lipo -create \
      target/aarch64-apple-darwin/release/libairgap.a \
      target/x86_64-apple-darwin/release/libairgap.a \
      -output "${OUTPUT_DIR}/libairgap-macos.a"
    echo "  -> ${OUTPUT_DIR}/libairgap-macos.a (universal)"
    ;;

  Linux)
    rustup target add x86_64-unknown-linux-gnu aarch64-unknown-linux-gnu 2>/dev/null || true

    cargo build --release --no-default-features --target x86_64-unknown-linux-gnu
    cp target/x86_64-unknown-linux-gnu/release/libairgap.a "${OUTPUT_DIR}/libairgap-linux-x64.a"
    echo "  -> ${OUTPUT_DIR}/libairgap-linux-x64.a"

    if command -v aarch64-linux-gnu-gcc &>/dev/null || [ -d "$(rustc --print sysroot)/lib/rustlib/aarch64-unknown-linux-gnu" ]; then
      cargo build --release --no-default-features --target aarch64-unknown-linux-gnu
      cp target/aarch64-unknown-linux-gnu/release/libairgap.a "${OUTPUT_DIR}/libairgap-linux-arm64.a"
      echo "  -> ${OUTPUT_DIR}/libairgap-linux-arm64.a"
    else
      echo "  (aarch64-unknown-linux-gnu linker not available, skipping)"
    fi
    ;;

  *)
    echo "Unsupported host: ${UNAME_S}"
    exit 1
    ;;
esac

cp include/airgap.h "${OUTPUT_DIR}/"

echo ""
echo "====================================="
echo "SIM BUILD COMPLETE"
echo "====================================="
ls -la "${OUTPUT_DIR}/"