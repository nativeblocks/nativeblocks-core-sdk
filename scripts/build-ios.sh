#!/usr/bin/env bash
# Build the iOS artifact: a static .xcframework (device + simulator) plus the
# Swift binding, staged under dist/ios/ ready to drop into an Xcode project.
#
# Prereqs (macOS + Xcode):
#   - rustup target add aarch64-apple-ios aarch64-apple-ios-sim
#
# The simulator slice is arm64-only (Apple Silicon / M-series). Intel-Mac
# simulators are intentionally not supported.
#
# Usage:  ./scripts/build-ios.sh
set -euo pipefail

cd "$(dirname "$0")/.."

LIB="libnativeblocks_core_sdk.a"
OUT="dist/ios"
XCF="$OUT/NativeblocksCoreSdk.xcframework"
# Swift module name configured in uniffi.toml ([bindings.swift] module_name).
MOD="NativeblocksCoreEngine"

echo "==> Building static lib for device + simulator targets (arm64 only)"
cargo build --release --target aarch64-apple-ios
cargo build --release --target aarch64-apple-ios-sim

mkdir -p "$OUT/headers"

# Simulator slice is the arm64 build directly — M-series only, no x86_64 fuse.
SIM_LIB="target/aarch64-apple-ios-sim/release/$LIB"

echo "==> Generating Swift bindings + headers dir"
./scripts/generate-bindings.sh >/dev/null
cp "bindings/swift/${MOD}FFI.h"         "$OUT/headers/"
cp "bindings/swift/${MOD}FFI.modulemap" "$OUT/headers/module.modulemap"

echo "==> Assembling xcframework"
rm -rf "$XCF"
xcodebuild -create-xcframework \
  -library "target/aarch64-apple-ios/release/$LIB" -headers "$OUT/headers" \
  -library "$SIM_LIB"                              -headers "$OUT/headers" \
  -output "$XCF"

cp "bindings/swift/${MOD}.swift" "$OUT/"

echo "==> Done. iOS artifacts in $OUT/"
echo "    - add $XCF to your target (Do Not Embed)"
echo "    - add $OUT/${MOD}.swift to your target"
