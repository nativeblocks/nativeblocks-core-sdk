#!/usr/bin/env bash
# Package the host SDK as ONE self-contained NativeblocksRuntime.xcframework.
#
# The package has three targets, but consumers must see exactly one module:
#
#   NativeblocksRuntimeCFFI   Rust static library + C header   (binaryTarget)
#   NativeblocksRuntimeFFI    generated UniFFI Swift bindings   -> folded in
#   NativeblocksRuntime       the host SDK, the public API      -> the framework
#
# So this script builds the two Swift targets, then `libtool`-merges their object
# files together with the Rust archive into a single static framework binary.
# Only NativeblocksRuntime.swiftmodule is shipped, so `import
# NativeblocksRuntimeFFI` does not resolve for anyone downstream and the Rust ABI
# stays unreachable.
#
# The generated bindings are already sealed to `package` visibility by
# core/scripts/seal-bindings.sh, so no FFI type can appear in the host's public
# API. This script VERIFIES that (see "Sealing check") rather than trusting it:
# the only thing the host interface may say about the FFI module is its import
# line, which is then dropped along with the module itself.
#
# Prereqs:
#   - xccache  (gem install xccache)
#   - Frameworks/NativeblocksRuntimeCFFI.xcframework, copied from core/dist/ios/
#     after running core/scripts/build-ios.sh
#
# Usage:  ./build_xcframework.sh
set -euo pipefail

cd "$(dirname "$0")"

FRAMEWORK_NAME="NativeblocksRuntime"
FFI_MODULE="NativeblocksRuntimeFFI"
C_MODULE="NativeblocksRuntimeCFFI"

OUTPUT_DIR="output"
STAGE_DIR="$OUTPUT_DIR/stage"
SLICES_DIR="$OUTPUT_DIR/slices"
XCFRAMEWORK_PATH="$OUTPUT_DIR/$FRAMEWORK_NAME.xcframework"
ZIP_NAME="$FRAMEWORK_NAME.xcframework.zip"

# xcframework slice ids, shared by the Swift builds and the vendored Rust one.
SLICES=("ios-arm64" "ios-arm64-simulator")

VENDORED_CFFI="Frameworks/$C_MODULE.xcframework"

# =============================================================================
# PREFLIGHT
# =============================================================================

command -v xccache >/dev/null || {
  echo "❌ xccache not found. Install it with: gem install xccache"
  exit 1
}

if [ ! -d "$VENDORED_CFFI" ]; then
  echo "❌ $VENDORED_CFFI not found."
  echo "   Run core/scripts/build-ios.sh, then copy core/dist/ios/$C_MODULE.xcframework here."
  exit 1
fi

for slice in "${SLICES[@]}"; do
  if ! ls "$VENDORED_CFFI/$slice"/*.a >/dev/null 2>&1; then
    echo "❌ $VENDORED_CFFI/$slice holds no static library (.a)."
    echo "   The Rust side has to be a staticlib for it to link INTO the framework."
    echo "   Re-run core/scripts/build-ios.sh."
    exit 1
  fi
done

echo "🧹 Cleaning previous build artifacts..."
rm -rf "$OUTPUT_DIR"
mkdir -p "$STAGE_DIR" "$SLICES_DIR"

# =============================================================================
# BUILD THE SWIFT TARGETS
# =============================================================================

echo "⚙️  Building $FRAMEWORK_NAME and $FFI_MODULE with library evolution..."
xccache pkg build "$FRAMEWORK_NAME" "$FFI_MODULE" \
  --sdk=iphoneos,iphonesimulator \
  --config=release \
  --library-evolution \
  --out="$STAGE_DIR"

for module in "$FRAMEWORK_NAME" "$FFI_MODULE"; do
  [ -d "$STAGE_DIR/$module.xcframework" ] || {
    echo "❌ xccache did not produce $STAGE_DIR/$module.xcframework"
    exit 1
  }
done

# =============================================================================
# MERGE EACH SLICE INTO ONE FRAMEWORK
# =============================================================================

for slice in "${SLICES[@]}"; do
  echo "🔗 Merging $slice..."

  HOST_FW="$STAGE_DIR/$FRAMEWORK_NAME.xcframework/$slice/$FRAMEWORK_NAME.framework"
  FFI_FW="$STAGE_DIR/$FFI_MODULE.xcframework/$slice/$FFI_MODULE.framework"
  RUST_LIB=$(ls "$VENDORED_CFFI/$slice"/*.a | head -1)

  OUT_FW="$SLICES_DIR/$slice/$FRAMEWORK_NAME.framework"
  mkdir -p "$OUT_FW"

  # Host Swift + generated bindings + Rust, in one archive.
  libtool -static -o "$OUT_FW/$FRAMEWORK_NAME" \
    "$HOST_FW/$FRAMEWORK_NAME" \
    "$FFI_FW/$FFI_MODULE" \
    "$RUST_LIB" 2>/dev/null

  # Everything besides the binary comes from the host framework, so the FFI
  # module's .swiftmodule is simply never copied — that is what makes it
  # unimportable downstream.
  cp -R "$HOST_FW/Headers" "$OUT_FW/" 2>/dev/null || true
  cp -R "$HOST_FW/Modules" "$OUT_FW/"
  cp "$HOST_FW/Info.plist" "$OUT_FW/Info.plist"

  SWIFTMODULE="$OUT_FW/Modules/$FRAMEWORK_NAME.swiftmodule"

  # `package` and `private` interfaces serve same-package and @_spi clients.
  # Neither applies to a binary consumer, and both name the FFI module, so drop
  # them rather than ship a dangling reference.
  rm -f "$SWIFTMODULE"/*.package.swiftinterface
  rm -f "$SWIFTMODULE"/*.private.swiftinterface

  # ---- Sealing check -------------------------------------------------------
  # If the FFI module appears in the public interface anywhere other than its
  # own import line, a declaration leaked a generated type into the host API.
  # That would ship an xcframework nobody can compile against, so fail here.
  for interface in "$SWIFTMODULE"/*.swiftinterface; do
    [ -e "$interface" ] || continue
    leaks=$(grep -n "$FFI_MODULE" "$interface" | grep -vE "^[0-9]+:import $FFI_MODULE$" || true)
    if [ -n "$leaks" ]; then
      echo "❌ $FFI_MODULE leaked into the public interface of $slice:"
      echo "$leaks"
      echo "   Re-run core/scripts/seal-bindings.sh and rebuild."
      exit 1
    fi
    # Sealed clean: the import is the only mention, and the module it names is
    # not shipped, so the line has to go too.
    sed -i '' "/^import $FFI_MODULE\$/d" "$interface"
  done

  echo "   ✅ $slice merged ($(du -h "$OUT_FW/$FRAMEWORK_NAME" | cut -f1))"
done

# =============================================================================
# ASSEMBLE THE XCFRAMEWORK
# =============================================================================

echo "📦 Assembling $FRAMEWORK_NAME.xcframework..."
CREATE_ARGS=()
for slice in "${SLICES[@]}"; do
  CREATE_ARGS+=(-framework "$SLICES_DIR/$slice/$FRAMEWORK_NAME.framework")
done
xcodebuild -create-xcframework "${CREATE_ARGS[@]}" -output "$XCFRAMEWORK_PATH" >/dev/null

# =============================================================================
# VERIFY
# =============================================================================

echo "🔍 Verifying the artifact is self-contained..."

# Swift mangles a module reference as <length><name>, e.g. 22NativeblocksRuntimeFFI.
FFI_MANGLED="${#FFI_MODULE}$FFI_MODULE"

for slice in "${SLICES[@]}"; do
  FW="$XCFRAMEWORK_PATH/$slice/$FRAMEWORK_NAME.framework"
  BINARY="$FW/$FRAMEWORK_NAME"
  MODULES="$FW/Modules"

  # `nm` reports undefineds per object file, so a symbol one member imports from
  # another still shows up as undefined. Subtract what the archive defines to get
  # the references that would actually reach the consumer's linker.
  DEFINED=$(mktemp)
  UNDEFINED=$(mktemp)
  # `|| true`: nm exits non-zero when any archive member has no symbols, which
  # is normal here and would otherwise trip pipefail.
  nm -gU "$BINARY" 2>/dev/null | awk '$2 ~ /^[A-TV-Z]$/ {print $3}' | sort -u > "$DEFINED"   || true
  nm -gu "$BINARY" 2>/dev/null | awk '$1 == "U" {print $2}'          | sort -u > "$UNDEFINED" || true
  dangling=$(comm -23 "$UNDEFINED" "$DEFINED" | grep -c "$FFI_MANGLED" || true)
  rust_defined=$(grep -cE "_(uniffi|ffi)_nativeblocks_runtime" "$DEFINED" || true)
  rm -f "$DEFINED" "$UNDEFINED"

  # Nothing may still be looking for the folded-in bindings module.
  if [ "$dangling" != "0" ]; then
    echo "   ❌ $slice has $dangling unresolved $FFI_MODULE symbols"
    exit 1
  fi

  # The Rust entry points must be defined here, not expected from elsewhere.
  # Zero usually means the archive was built with a stripping profile — see
  # [profile.ios-static] in core/Cargo.toml.
  if [ "$rust_defined" = "0" ]; then
    echo "   ❌ $slice contains no Rust entry points — the static library did not merge"
    exit 1
  fi

  # Exactly one importable module.
  shipped=$(ls "$MODULES" | grep -c '\.swiftmodule$' || true)
  if [ "$shipped" != "1" ] || [ ! -d "$MODULES/$FRAMEWORK_NAME.swiftmodule" ]; then
    echo "   ❌ $slice ships $shipped modules; expected only $FRAMEWORK_NAME"
    ls "$MODULES"
    exit 1
  fi

  echo "   ✅ $slice: $rust_defined Rust entry points, 1 module, nothing dangling"
done

# =============================================================================
# PACKAGE
# =============================================================================

echo "🗜️  Creating distribution archive..."
(cd "$OUTPUT_DIR" && zip -r -q "$ZIP_NAME" "$FRAMEWORK_NAME.xcframework")

CHECKSUM=$(swift package compute-checksum "$OUTPUT_DIR/$ZIP_NAME")

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "✅ Build completed successfully!"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "📦 XCFramework:  $OUTPUT_DIR/$ZIP_NAME ($(du -h "$OUTPUT_DIR/$ZIP_NAME" | cut -f1))"
echo "🔐 Checksum:     $CHECKSUM"
echo ""
echo "The framework is static, so consumers set it to \"Do Not Embed\"."
echo "One artifact, one module: import $FRAMEWORK_NAME"
echo ""
cat <<EOF
// swift-tools-version: 5.9
import PackageDescription

let package = Package(
    name: "$FRAMEWORK_NAME",
    platforms: [.iOS(.v15)],
    products: [
        .library(name: "$FRAMEWORK_NAME", targets: ["$FRAMEWORK_NAME"])
    ],
    targets: [
        .binaryTarget(
            name: "$FRAMEWORK_NAME",
            url: "<your-hosted-url>/$ZIP_NAME",
            checksum: "$CHECKSUM"
        )
    ]
)
EOF
