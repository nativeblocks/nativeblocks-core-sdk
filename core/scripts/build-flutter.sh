#!/usr/bin/env bash
# Build the Flutter artifacts: per-platform dynamic libs + the Dart binding,
# staged under dist/flutter/ ready to drop into a Flutter FFI plugin.
#
# Prereqs:
#   - cargo install cargo-ndk          (+ Android NDK)  for Android
#   - cargo install uniffi-bindgen-dart --version 0.1.3
#   - rustup target add aarch64-apple-ios               for iOS (on macOS)
#
# Usage:  ./scripts/build-flutter.sh
set -euo pipefail

cd "$(dirname "$0")/.."

OUT="dist/flutter"
mkdir -p "$OUT"

echo "==> Android: cdylib .so per ABI"
# rquickjs has no prebuilt bindings for the Android triples; generate them at
# build time (--features script-quickjs-bindgen). cargo-ndk supplies the sysroot.
cargo ndk \
  -t arm64-v8a -t armeabi-v7a -t x86_64 -t x86 \
  -o "$OUT/android/jniLibs" \
  build --release --features script-quickjs-bindgen

# iOS dylib only builds on macOS; skip elsewhere so the script stays portable.
if [[ "$(uname -s)" == "Darwin" ]]; then
  echo "==> iOS: cdylib for device"
  # Point bindgen at the iOS SDK; pin a modern deployment target (QuickJS >= 12).
  export BINDGEN_EXTRA_CLANG_ARGS_aarch64_apple_ios="-isysroot $(xcrun --sdk iphoneos --show-sdk-path)"
  export IPHONEOS_DEPLOYMENT_TARGET=13.0
  cargo build --release --target aarch64-apple-ios --features script-quickjs-bindgen
  mkdir -p "$OUT/ios"
  cp target/aarch64-apple-ios/release/libcore_nativeblocks.a "$OUT/ios/" 2>/dev/null || true
fi

echo "==> Generating Dart binding"
./scripts/generate-bindings.sh >/dev/null
mkdir -p "$OUT/lib"
cp bindings/dart/NativeblocksCoreEngine.dart "$OUT/lib/"

echo "==> Done. Flutter artifacts in $OUT/"
echo "    - copy $OUT/android/jniLibs/* into  <plugin>/android/src/main/jniLibs/"
echo "    - copy $OUT/lib/*           into  <plugin>/lib/"
echo "    - embed the iOS lib in the plugin's ios/ target"
