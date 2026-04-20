#!/bin/bash
set -e

echo "====================================="
echo "Building Airgap library for JS/WASM"
echo "====================================="

# --------------------------------------------------------
# Check wasm-pack is installed
# --------------------------------------------------------
if ! command -v wasm-pack &> /dev/null; then
    echo "wasm-pack not found. Installing..."
    curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
fi

# --------------------------------------------------------
# Add wasm32 target
# --------------------------------------------------------
rustup target add wasm32-unknown-unknown 2>/dev/null || true

# --------------------------------------------------------
# Build
# --------------------------------------------------------
echo ""
echo "Building WASM package..."
wasm-pack build --release --target web --out-dir pkg

# --------------------------------------------------------
# Output
# --------------------------------------------------------
echo ""
echo "====================================="
echo "WASM BUILD COMPLETE"
echo "====================================="
echo ""
echo "Package files:"
echo "  pkg/airgap.js        - ES module loader"
echo "  pkg/airgap_bg.wasm   - compiled WASM binary"
echo "  pkg/airgap.d.ts      - TypeScript types"
echo ""
echo "Usage:"
echo "  import init, { WasmEncoder, WasmDecoder } from './pkg/airgap.js';"
echo "  await init();"
echo ""