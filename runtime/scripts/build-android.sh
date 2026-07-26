#!/usr/bin/env bash
# Build the Android artifacts: a release .so per ABI + the Kotlin bindings,
# staged under dist/android/ ready to drop into an Android library module.
#
# Prereqs:
#   - Android NDK installed (ANDROID_NDK_HOME set)
#   - cargo install cargo-ndk
#   - rustup target add aarch64-linux-android armv7-linux-androideabi \
#                       i686-linux-android x86_64-linux-android
#
# Usage:  ./scripts/build-android.sh
set -euo pipefail

cd "$(dirname "$0")/.."

OUT="dist/android"
JNILIBS="$OUT/jniLibs"
# Mirrors [bindings.kotlin] package_name in uniffi.toml. The `ffi` leaf keeps the
# generated code out of the host's own io.nativeblocks.runtime package; it is
# sealed to `internal`, so it must be compiled in the same Gradle module as the
# wrapper (README.md).
KT_PKG="io/nativeblocks/runtime/ffi"

echo "==> Cross-compiling release .so for all ABIs"
# rquickjs has no prebuilt bindings for the Android triples; generate them at
# build time (--features script-quickjs-bindgen). cargo-ndk exports the
# per-target sysroot that bindgen needs.
cargo ndk \
  -t arm64-v8a -t armeabi-v7a -t x86_64 -t x86 \
  -o "$JNILIBS" \
  build --release --features script-quickjs-bindgen

echo "==> Generating Kotlin bindings"
./scripts/generate-bindings.sh >/dev/null

echo "==> Staging Kotlin binding"
mkdir -p "$OUT/java/$KT_PKG"
cp "bindings/kotlin/$KT_PKG/NativeblocksRuntime.kt" "$OUT/java/$KT_PKG/"

echo "==> Done. Android artifacts in $OUT/"
echo "    - copy $JNILIBS/* into  <module>/src/main/jniLibs/"
echo "    - copy $OUT/java/*  into <module>/src/main/java/  (same module as the wrapper)"
echo "    - add dependency:  net.java.dev.jna:jna:5.14.0@aar"
