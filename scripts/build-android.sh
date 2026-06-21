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
KT_PKG="io/nativeblocks/core/engine"

echo "==> Cross-compiling release .so for all ABIs"
# script-quickjs-bindgen: rquickjs has no precompiled bindings for the Android
# triples, so generate them at build time. cargo-ndk exports the per-target
# BINDGEN_EXTRA_CLANG_ARGS_* (sysroot) that bindgen needs.
cargo ndk \
  -t arm64-v8a -t armeabi-v7a -t x86_64 -t x86 \
  -o "$JNILIBS" \
  build --release --features script-quickjs-bindgen

echo "==> Generating Kotlin bindings"
./scripts/generate-bindings.sh >/dev/null

echo "==> Staging Kotlin binding"
mkdir -p "$OUT/java/$KT_PKG"
cp "bindings/kotlin/$KT_PKG/nativeblocks_core_sdk.kt" "$OUT/java/$KT_PKG/"

echo "==> Done. Android artifacts in $OUT/"
echo "    - copy $JNILIBS/* into  <module>/src/main/jniLibs/"
echo "    - copy $OUT/java/*  into <module>/src/main/java/"
echo "    - add dependency:  net.java.dev.jna:jna:5.14.0@aar"
