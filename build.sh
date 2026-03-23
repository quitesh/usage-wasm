#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")"

# Initialize submodule if needed
git submodule update --init

# Apply patches (idempotent — reset first)
git -C usage checkout -- .
for patch in patches/*.patch; do
  [ -f "$patch" ] && git -C usage apply "../$patch"
done

# Build WASM
cargo build --release --target wasm32-unknown-unknown

# Run wasm-bindgen
wasm-bindgen \
  --target web \
  --out-dir pkg/dist \
  target/wasm32-unknown-unknown/release/usage_wasm.wasm

# Copy wasm binary to package root for ?url imports
cp pkg/dist/usage_wasm_bg.wasm pkg/

# Remove wasm-bindgen generated .gitignore
rm -f pkg/dist/.gitignore

echo "Build complete. Output in pkg/"
