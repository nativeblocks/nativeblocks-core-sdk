#!/usr/bin/env bash
# Build the iOS artifact: a static .xcframework (device + simulator) plus the
# Swift binding, staged under dist/ios/ ready to drop into an Xcode project.
#
# Prereqs (macOS + Xcode):
#   - rustup target add aarch64-apple-ios aarch64-apple-ios-sim x86_64-apple-ios
#
# Usage:  ./scripts/build-ios.sh
set -euo pipefail

cd "$(dirname "$0")/.."

LIB="libnativeblocks_core_sdk.a"
OUT="dist/ios"
XCF="$OUT/NativeblocksCoreSdk.xcframework"

echo "==> Building static lib for device + simulator targets"
cargo build --release --target aarch64-apple-ios
cargo build --release --target aarch64-apple-ios-sim
cargo build --release --target x86_64-apple-ios

mkdir -p "$OUT/headers"

echo "==> Fusing simulator slices (arm64 + x86_64) into one fat lib"
lipo -create \
  "target/aarch64-apple-ios-sim/release/$LIB" \
  "target/x86_64-apple-ios/release/$LIB" \
  -output "$OUT/libnativeblocks_core_sdk-sim.a"

echo "==> Generating Swift bindings + headers dir"
./scripts/generate-bindings.sh >/dev/null
cp bindings/swift/nativeblocks_core_sdkFFI.h         "$OUT/headers/"
cp bindings/swift/nativeblocks_core_sdkFFI.modulemap "$OUT/headers/module.modulemap"

echo "==> Assembling xcframework"
rm -rf "$XCF"
xcodebuild -create-xcframework \
  -library "target/aarch64-apple-ios/release/$LIB" -headers "$OUT/headers" \
  -library "$OUT/libnativeblocks_core_sdk-sim.a"   -headers "$OUT/headers" \
  -output "$XCF"

cp bindings/swift/nativeblocks_core_sdk.swift "$OUT/"

echo "==> Done. iOS artifacts in $OUT/"
echo "    - add $XCF to your target (Do Not Embed)"
echo "    - add $OUT/nativeblocks_core_sdk.swift to your target"
