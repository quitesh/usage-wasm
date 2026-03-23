#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")"

# Initialize submodule if needed
git submodule update --init

# Ensure rustup-managed toolchain is used (Homebrew cargo lacks wasm32 target)
if command -v rustup &>/dev/null; then
  CARGO="rustup run stable cargo"
else
  CARGO="cargo"
fi

# Build WASM
$CARGO build --release --target wasm32-unknown-unknown

# Run wasm-bindgen
wasm-bindgen \
  --target web \
  --out-dir dist \
  target/wasm32-unknown-unknown/release/usage_wasm.wasm

# Copy wasm binary to package root for ?url imports
cp dist/usage_wasm_bg.wasm .

# Remove wasm-bindgen generated .gitignore
rm -f dist/.gitignore

echo "Build complete. Output in dist/"
