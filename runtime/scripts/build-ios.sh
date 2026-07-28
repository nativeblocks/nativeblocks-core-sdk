#!/usr/bin/env bash
# Build the iOS xcframework (arm64 device + arm64 simulator) plus the Swift binding into dist/ios/.
set -euo pipefail

cd "$(dirname "$0")/.."

STATICLIB="libnativeblocks_runtime.a"
OUT="dist/ios"
SWIFT_MOD="NativeblocksRuntimeFFI"
C_MOD="NativeblocksRuntimeCFFI"
XCF="$OUT/$C_MOD.xcframework"

export BINDGEN_EXTRA_CLANG_ARGS_aarch64_apple_ios="-isysroot $(xcrun --sdk iphoneos --show-sdk-path)"
export BINDGEN_EXTRA_CLANG_ARGS_aarch64_apple_ios_sim="-isysroot $(xcrun --sdk iphonesimulator --show-sdk-path) --target=arm64-apple-ios13.0-simulator"
export IPHONEOS_DEPLOYMENT_TARGET=13.0

PROFILE="ios-static"

echo "==> Building static lib for device + simulator targets (arm64 only)"
cargo build --profile "$PROFILE" --target aarch64-apple-ios     --features script-quickjs-bindgen
cargo build --profile "$PROFILE" --target aarch64-apple-ios-sim --features script-quickjs-bindgen

DEV_LIB="target/aarch64-apple-ios/$PROFILE/$STATICLIB"
SIM_LIB="target/aarch64-apple-ios-sim/$PROFILE/$STATICLIB"

echo "==> Verifying the archives export the UniFFI entry points"
for lib in "$DEV_LIB" "$SIM_LIB"; do
  count=$(nm -gU "$lib" 2>/dev/null | grep -cE "_(uniffi|ffi)_nativeblocks_runtime" || true)
  if [ "$count" -eq 0 ]; then
    echo "❌ $lib exports no UniFFI entry points."
    echo "   [profile.$PROFILE] must keep lto and strip off — see Cargo.toml."
    exit 1
  fi
  echo "    $(basename "$(dirname "$lib")")/$STATICLIB: $count entry points"
done

OBJCOPY="$(rustc --print sysroot)/lib/rustlib/$(rustc -vV | sed -n 's/^host: //p')/bin/rust-objcopy"
if [ ! -x "$OBJCOPY" ]; then
  echo "❌ rust-objcopy not found at $OBJCOPY"
  echo "   install it with: rustup component add llvm-tools"
  exit 1
fi

echo "==> Stripping debug info and embedded bitcode"
for lib in "$DEV_LIB" "$SIM_LIB"; do
  before=$(du -m "$lib" | cut -f1)
  strip -S "$lib"
  "$OBJCOPY" --remove-section=__LLVM,__bitcode --remove-section=__LLVM,__cmdline "$lib"

  count=$(nm -gU "$lib" 2>/dev/null | grep -cE "_(uniffi|ffi)_nativeblocks_runtime" || true)
  if [ "$count" -eq 0 ]; then
    echo "❌ $lib lost its UniFFI entry points while stripping."
    exit 1
  fi
  echo "    $(basename "$(dirname "$lib")")/$STATICLIB: ${before} MB -> $(du -m "$lib" | cut -f1) MB, $count entry points"
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
