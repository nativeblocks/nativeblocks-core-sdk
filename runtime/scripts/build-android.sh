#!/usr/bin/env bash
# Build the Android .so for every ABI plus the Kotlin bindings into dist/android/.
set -euo pipefail

cd "$(dirname "$0")/.."

OUT="dist/android"
JNILIBS="$OUT/jniLibs"
KT_PKG="io/nativeblocks/runtime/ffi"

echo "==> Cross-compiling release .so for all ABIs"
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
