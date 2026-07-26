#!/usr/bin/env bash
# Regenerate FFI bindings for Kotlin and Swift from the Rust core.
#
# Kotlin and Swift are the only FFI surfaces.
#
# Prereqs:
#   - Rust toolchain (cargo)
#
# Usage:  ./scripts/generate-bindings.sh
set -euo pipefail

cd "$(dirname "$0")/.."

# Matches [lib] name in Cargo.toml (the shipped artifact name, not the
# crate/package name `nativeblocks-runtime`).
LIB_NAME="nativeblocks_runtime"

case "$(uname -s)" in
  Darwin) EXT="dylib" ;;
  Linux)  EXT="so" ;;
  *)      echo "Unsupported host OS: $(uname -s)" >&2; exit 1 ;;
esac

LIB="target/debug/lib${LIB_NAME}.${EXT}"

echo "==> Building Rust library"
cargo build --features script-quickjs-bindgen

echo "==> Generating Kotlin bindings -> bindings/kotlin"
cargo run --quiet --bin uniffi-bindgen -- \
  generate --library "$LIB" --language kotlin --out-dir bindings/kotlin

echo "==> Generating Swift bindings -> bindings/swift"
cargo run --quiet --bin uniffi-bindgen -- \
  generate --library "$LIB" --language swift --out-dir bindings/swift

# Visibility is narrowed here, before anything is staged or copied into a host
# SDK, so there is never an unsealed copy of the bindings to pick up by mistake.
echo "==> Sealing bindings (generated FFI must not become host public API)"
./scripts/seal-bindings.sh

echo "==> Done."
