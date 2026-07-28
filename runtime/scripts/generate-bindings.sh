#!/usr/bin/env bash
# Regenerate the Kotlin and Swift FFI bindings from the Rust runtime, then seal them.
set -euo pipefail

cd "$(dirname "$0")/.."

LIB_NAME="nativeblocks_runtime"

case "$(uname -s)" in
  Darwin) EXT="dylib" ;;
  Linux)  EXT="so" ;;
  *)      echo "Unsupported host OS: $(uname -s)" >&2; exit 1 ;;
esac

LIB="target/debug/lib${LIB_NAME}.${EXT}"

FEATURES="script-quickjs-bindgen,uniffi-cli"

echo "==> Building Rust library"
cargo build --features "$FEATURES"

echo "==> Generating Kotlin bindings -> bindings/kotlin"
cargo run --quiet --features "$FEATURES" --bin uniffi-bindgen -- \
  generate --library "$LIB" --language kotlin --out-dir bindings/kotlin

echo "==> Generating Swift bindings -> bindings/swift"
cargo run --quiet --features "$FEATURES" --bin uniffi-bindgen -- \
  generate --library "$LIB" --language swift --out-dir bindings/swift

echo "==> Sealing bindings (generated FFI must not become host public API)"
./scripts/seal-bindings.sh

echo "==> Done."
