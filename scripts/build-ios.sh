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
# Swift module name configured in uniffi.toml ([bindings.swift] module_name).
MOD="NativeblocksCoreEngine"

# rquickjs ships no precompiled bindings for the apple-ios triples, so build
# them with script-quickjs-bindgen (libclang). Unlike cargo-ndk, nothing wires
# the SDK sysroot for us, so point bindgen at the right SDK per target via the
# per-target BINDGEN_EXTRA_CLANG_ARGS_* env var (bindgen reads it automatically).
IOS_SDK="$(xcrun --sdk iphoneos --show-sdk-path)"
SIM_SDK="$(xcrun --sdk iphonesimulator --show-sdk-path)"
# Device's Rust triple is a valid clang triple, so it only needs the sysroot.
# The simulator Rust triples (…-ios-sim, x86_64…-ios) are NOT valid clang
# triples — rquickjs-sys passes the Rust triple to clang as --target=, and
# clang rejects "ios-sim". Append a real …-simulator triple; bindgen adds env
# args after the builtin --target, so the last (ours) wins.
export BINDGEN_EXTRA_CLANG_ARGS_aarch64_apple_ios="-isysroot $IOS_SDK"
export BINDGEN_EXTRA_CLANG_ARGS_aarch64_apple_ios_sim="-isysroot $SIM_SDK --target=arm64-apple-ios13.0-simulator"
export BINDGEN_EXTRA_CLANG_ARGS_x86_64_apple_ios="-isysroot $SIM_SDK --target=x86_64-apple-ios13.0-simulator"
# QuickJS's C uses stack probes (___chkstk_darwin), unavailable below iOS 12;
# pin a modern deployment target so the cc-compiled objects link.
export IPHONEOS_DEPLOYMENT_TARGET=13.0

echo "==> Building static lib for device + simulator targets"
cargo build --release --target aarch64-apple-ios     --features script-quickjs-bindgen
cargo build --release --target aarch64-apple-ios-sim --features script-quickjs-bindgen
cargo build --release --target x86_64-apple-ios      --features script-quickjs-bindgen

mkdir -p "$OUT/headers"

echo "==> Fusing simulator slices (arm64 + x86_64) into one fat lib"
lipo -create \
  "target/aarch64-apple-ios-sim/release/$LIB" \
  "target/x86_64-apple-ios/release/$LIB" \
  -output "$OUT/libnativeblocks_core_sdk-sim.a"

echo "==> Generating Swift bindings + headers dir"
./scripts/generate-bindings.sh >/dev/null
cp "bindings/swift/${MOD}FFI.h"         "$OUT/headers/"
cp "bindings/swift/${MOD}FFI.modulemap" "$OUT/headers/module.modulemap"

echo "==> Assembling xcframework"
rm -rf "$XCF"
xcodebuild -create-xcframework \
  -library "target/aarch64-apple-ios/release/$LIB" -headers "$OUT/headers" \
  -library "$OUT/libnativeblocks_core_sdk-sim.a"   -headers "$OUT/headers" \
  -output "$XCF"

cp "bindings/swift/${MOD}.swift" "$OUT/"

echo "==> Done. iOS artifacts in $OUT/"
echo "    - add $XCF to your target (Do Not Embed)"
echo "    - add $OUT/${MOD}.swift to your target"
