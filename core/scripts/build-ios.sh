#!/usr/bin/env bash
# Build the iOS artifact: a static .xcframework (device + simulator) plus the
# Swift binding, staged under dist/ios/ ready to drop into the host package.
#
# We ship the staticlib (.a), not the cdylib. The host SDK is distributed as ONE
# self-contained NativeblocksRuntime.xcframework, so the Rust code has to be
# linkable INTO that framework's binary rather than sitting beside it as a second
# artifact consumers must Embed & Sign. A static archive is an un-dead-stripped
# bag of every object file (~58 MB/slice), but nothing of that reaches the app:
# the consumer's linker drops the unreferenced sections, landing at roughly the
# same size the dynamic library would have been.
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
STATICLIB="libnativeblocks_runtime.a"
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

# Built with the ios-static profile, not release: release turns on lto and strip,
# either of which silently empties a static archive of its extern "C" entry
# points. See the comment on [profile.ios-static] in Cargo.toml.
PROFILE="ios-static"

echo "==> Building static lib for device + simulator targets (arm64 only)"
cargo build --profile "$PROFILE" --target aarch64-apple-ios     --features script-quickjs-bindgen
cargo build --profile "$PROFILE" --target aarch64-apple-ios-sim --features script-quickjs-bindgen

DEV_LIB="target/aarch64-apple-ios/$PROFILE/$STATICLIB"
SIM_LIB="target/aarch64-apple-ios-sim/$PROFILE/$STATICLIB"

# A staticlib built with the wrong profile fails silently: the archive is still
# ~60 MB and still contains the crate's object, but LTO has internalized every
# entry point, so it resolves nothing at link time and the error only surfaces
# in a consumer's app. Check here, where the cause is one line away.
echo "==> Verifying the archives export the UniFFI entry points"
for lib in "$DEV_LIB" "$SIM_LIB"; do
  # `|| true`: nm exits non-zero on members with no symbols, which is normal.
  count=$(nm -gU "$lib" 2>/dev/null | grep -cE "_(uniffi|ffi)_nativeblocks_runtime" || true)
  if [ "$count" -eq 0 ]; then
    echo "❌ $lib exports no UniFFI entry points."
    echo "   [profile.$PROFILE] must keep lto and strip off — see Cargo.toml."
    exit 1
  fi
  echo "    $(basename "$(dirname "$lib")")/$STATICLIB: $count entry points"
done

mkdir -p "$OUT/headers"

echo "==> Generating Swift bindings + headers dir"
./scripts/generate-bindings.sh >/dev/null
cp "bindings/swift/${C_MOD}.h"         "$OUT/headers/"
cp "bindings/swift/${C_MOD}.modulemap" "$OUT/headers/module.modulemap"

echo "==> Assembling xcframework"
rm -rf "$XCF"
xcodebuild -create-xcframework \
  -library "$DEV_LIB" -headers "$OUT/headers" \
  -library "$SIM_LIB" -headers "$OUT/headers" \
  -output "$XCF"

cp "bindings/swift/${SWIFT_MOD}.swift" "$OUT/"

echo "==> Done. iOS artifacts in $OUT/"
echo "    - copy $XCF into the host package's Frameworks/ (binaryTarget $C_MOD)"
echo "    - copy $OUT/${SWIFT_MOD}.swift into the $SWIFT_MOD target, NOT the host target"
