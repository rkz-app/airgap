#!/bin/bash
set -e

echo "====================================="
echo "Building Airgap for ARM Cortex-M"
echo "====================================="

OUTPUT_DIR="target/arm"
mkdir -p "${OUTPUT_DIR}"

# thumbv8m.main-none-eabihf:  Cortex-M33 (STM32L5/U5/H5)
# thumbv7em-none-eabihf:      Cortex-M4F, M7 (STM32F4/F7/H7/G4/L4)
# thumbv7m-none-eabi:         Cortex-M3 (STM32F1/F2)
# thumbv6m-none-eabi:         Cortex-M0, M0+ (STM32F0/G0)

for triple in \
  thumbv8m.main-none-eabihf \
  thumbv7em-none-eabihf \
  thumbv7m-none-eabi \
  thumbv6m-none-eabi
do
  echo ""
  echo "Building for ${triple}..."
  rustup target add "${triple}" 2>/dev/null || true
  cargo build --release --no-default-features --target "${triple}"  --config 'rustflags="-C panic=abort"'
  name="${triple%%-none-eabi*}"
  name="${name%%-none-eabihf*}"
  cp "target/${triple}/release/libairgap.a" "${OUTPUT_DIR}/libairgap-${name}.a"
  echo "  -> ${OUTPUT_DIR}/libairgap-${name}.a"
done

cp include/airgap.h "${OUTPUT_DIR}/"

echo ""
echo "====================================="
echo "ARM BUILD COMPLETE"
echo "====================================="
ls -la "${OUTPUT_DIR}/"