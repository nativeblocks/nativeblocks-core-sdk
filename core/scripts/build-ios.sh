#!/usr/bin/env bash
# Build the iOS artifact: a dynamic .xcframework (device + simulator) plus the
# Swift binding, staged under dist/ios/ ready to drop into an Xcode project.
#
# We ship the cdylib (.dylib), not the staticlib (.a). A static archive is an
# un-dead-stripped bag of every object file (~58 MB/slice); the linked+stripped
# dynamic library is the same code with unused sections gone (~1.3 MB/slice).
# Consumers "Embed & Sign" it instead of "Do Not Embed".
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

# Artifact name comes from [lib] name in Cargo.toml, not the package name.
DYLIB="libnativeblocks_runtime.dylib"
OUT="dist/ios"
# Both names come from uniffi.toml [bindings.swift]. They are deliberately NOT
# the host's name (NativeblocksRuntime), which stays free for its public API:
#   SWIFT_MOD  generated Swift bindings, sealed to `package` visibility
#   C_MOD      low-level C module — header, modulemap and the xcframework
SWIFT_MOD="NativeblocksRuntimeFFI"
C_MOD="NativeblocksRuntimeCFFI"
XCF="$OUT/$C_MOD.xcframework"

# rquickjs ships no prebuilt bindings for the apple-ios triples, so generate
# them with libclang (--features script-quickjs-bindgen). Point bindgen at the
# right SDK per target; the simulator Rust triple (…-ios-sim) isn't a valid
# clang triple, so append a real …-simulator triple (bindgen's env args win).
# Pin a modern deployment target so QuickJS's stack probes link (iOS >= 12).
export BINDGEN_EXTRA_CLANG_ARGS_aarch64_apple_ios="-isysroot $(xcrun --sdk iphoneos --show-sdk-path)"
export BINDGEN_EXTRA_CLANG_ARGS_aarch64_apple_ios_sim="-isysroot $(xcrun --sdk iphonesimulator --show-sdk-path) --target=arm64-apple-ios13.0-simulator"
export IPHONEOS_DEPLOYMENT_TARGET=13.0

echo "==> Building dynamic lib for device + simulator targets (arm64 only)"
cargo build --release --target aarch64-apple-ios     --features script-quickjs-bindgen
cargo build --release --target aarch64-apple-ios-sim --features script-quickjs-bindgen

mkdir -p "$OUT/headers"

# Stage the dylibs and rewrite their install name to @rpath so the loader finds
# them once Xcode embeds them under the app's Frameworks/ dir. Cargo stamps the
# absolute build path as LC_ID_DYLIB, which would not resolve on-device. Staged
# under target/ so the intermediates never ship in dist/.
STAGE="target/ios-dylibs"
DEV_DYLIB="$STAGE/device/$DYLIB"
SIM_DYLIB="$STAGE/sim/$DYLIB"
rm -rf "$STAGE"
mkdir -p "$STAGE/device" "$STAGE/sim"
cp "target/aarch64-apple-ios/release/$DYLIB"     "$DEV_DYLIB"
cp "target/aarch64-apple-ios-sim/release/$DYLIB" "$SIM_DYLIB"

echo "==> Rewriting install names to @rpath (required for embedding)"
install_name_tool -id "@rpath/$DYLIB" "$DEV_DYLIB"
install_name_tool -id "@rpath/$DYLIB" "$SIM_DYLIB"

echo "==> Generating Swift bindings + headers dir"
./scripts/generate-bindings.sh >/dev/null
cp "bindings/swift/${C_MOD}.h"         "$OUT/headers/"
cp "bindings/swift/${C_MOD}.modulemap" "$OUT/headers/module.modulemap"

echo "==> Assembling xcframework"
rm -rf "$XCF"
xcodebuild -create-xcframework \
  -library "$DEV_DYLIB" -headers "$OUT/headers" \
  -library "$SIM_DYLIB" -headers "$OUT/headers" \
  -output "$XCF"

cp "bindings/swift/${SWIFT_MOD}.swift" "$OUT/"

echo "==> Done. iOS artifacts in $OUT/"
echo "    - add $XCF to the package as a binaryTarget named $C_MOD (Embed & Sign)"
echo "    - add $OUT/${SWIFT_MOD}.swift to the $SWIFT_MOD target, NOT to the host target"
