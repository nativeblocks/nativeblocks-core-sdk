#!/usr/bin/env bash
# Regenerate FFI bindings for Kotlin, Swift, and Dart from the Rust core.
#
# Prereqs:
#   - Rust toolchain (cargo)
#   - Dart generator:  cargo install uniffi-bindgen-dart --version 0.1.3
#
# Usage:  ./scripts/generate-bindings.sh
set -euo pipefail

cd "$(dirname "$0")/.."

LIB_NAME="nativeblocks_core_sdk"

case "$(uname -s)" in
  Darwin) EXT="dylib" ;;
  Linux)  EXT="so" ;;
  *)      echo "Unsupported host OS: $(uname -s)" >&2; exit 1 ;;
esac

LIB="target/debug/lib${LIB_NAME}.${EXT}"

echo "==> Building Rust library"
cargo build

echo "==> Generating Kotlin bindings -> bindings/kotlin"
cargo run --quiet --bin uniffi-bindgen -- \
  generate --library "$LIB" --language kotlin --out-dir bindings/kotlin

echo "==> Generating Swift bindings -> bindings/swift"
cargo run --quiet --bin uniffi-bindgen -- \
  generate --library "$LIB" --language swift --out-dir bindings/swift

echo "==> Generating Dart bindings -> bindings/dart"
if command -v uniffi-bindgen-dart >/dev/null 2>&1; then
  uniffi-bindgen-dart generate --library "$LIB" --out-dir bindings/dart
else
  echo "    SKIPPED: uniffi-bindgen-dart not installed." >&2
  echo "    Run: cargo install uniffi-bindgen-dart --version 0.1.3" >&2
fi

echo "==> Done."
