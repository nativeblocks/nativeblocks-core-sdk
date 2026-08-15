#!/bin/bash
set -euo pipefail

# =========================================================
# CONFIG
# =========================================================
FRAMEWORK_NAME="NativeblocksDevkit"
VERSION="1.0.0"

PACKAGE_PATH=$(pwd)
SOURCE_DIR="$PACKAGE_PATH/Sources/$FRAMEWORK_NAME"
RESOURCE_DIR="$SOURCE_DIR/Resources"

OUTPUT_DIR="output"
XCFRAMEWORK_NAME="$FRAMEWORK_NAME.xcframework"
ZIP_NAME="$XCFRAMEWORK_NAME.zip"
XCFRAMEWORK_PATH="$OUTPUT_DIR/$XCFRAMEWORK_NAME"
DSYM_DIR="$OUTPUT_DIR/dSYMs"

# =========================================================
# CLEANUP
# =========================================================
echo "🧹 Cleaning previous artifacts..."
rm -rf "$OUTPUT_DIR" "$RESOURCE_DIR" "$XCFRAMEWORK_NAME"
mkdir -p "$OUTPUT_DIR" "$DSYM_DIR"

# =========================================================
# BUILD XCFRAMEWORK (MODULE STABLE)
# =========================================================
echo "⚙️ Building XCFramework (library evolution enabled)..."
xccache pkg build "$FRAMEWORK_NAME" \
  --sdk=iphoneos,iphonesimulator \
  --config=release \
  --library-evolution

# =========================================================
# PATCH FRAMEWORK SLICES (SAFE, NON-DESTRUCTIVE)
# =========================================================
echo "🔧 Patching framework slices..."

for platform in ios-arm64 ios-arm64-simulator; do
  FRAMEWORK_DIR="$XCFRAMEWORK_NAME/$platform/$FRAMEWORK_NAME.framework"

  if [[ ! -d "$FRAMEWORK_DIR" ]]; then
    echo "⚠️ Slice not found: $FRAMEWORK_DIR"
    continue
  fi

  PLIST="$FRAMEWORK_DIR/Info.plist"

  echo "➡️ Processing $platform"

  # ---- Versioning (do NOT overwrite plist) ----
  /usr/libexec/PlistBuddy -c "Add :CFBundleShortVersionString string $VERSION" "$PLIST" 2>/dev/null || \
  /usr/libexec/PlistBuddy -c "Set :CFBundleShortVersionString $VERSION" "$PLIST"

  /usr/libexec/PlistBuddy -c "Add :CFBundleVersion string $VERSION" "$PLIST" 2>/dev/null || \
  /usr/libexec/PlistBuddy -c "Set :CFBundleVersion $VERSION" "$PLIST"

  # ---- Minimum OS (non-destructive) ----
  /usr/libexec/PlistBuddy -c "Add :MinimumOSVersion string 15.6" "$PLIST" 2>/dev/null || true

  # ---- Validation (XCFramework-aware) ----
  PACKAGE_TYPE=$(/usr/libexec/PlistBuddy -c "Print :CFBundlePackageType" "$PLIST" 2>/dev/null || echo "MISSING")
  EXECUTABLE=$(/usr/libexec/PlistBuddy -c "Print :CFBundleExecutable" "$PLIST" 2>/dev/null || echo "MISSING")

  if [[ "$PACKAGE_TYPE" == "MISSING" ]]; then
    echo "❌ ERROR: CFBundlePackageType missing"
    exit 1
  fi

  if [[ "$EXECUTABLE" != "$FRAMEWORK_NAME" ]]; then
    echo "❌ ERROR: CFBundleExecutable invalid ($EXECUTABLE)"
    exit 1
  fi

  echo "ℹ️ CFBundlePackageType=$PACKAGE_TYPE, Executable=$EXECUTABLE"
  echo "✅ $platform OK"
done

# =========================================================
# COLLECT dSYMs (BEFORE STRIPPING)
# =========================================================
echo "📦 Collecting dSYMs..."
find "$XCFRAMEWORK_NAME" -name "*.dSYM" -exec cp -R {} "$DSYM_DIR/" \;
echo "✅ dSYMs stored in $DSYM_DIR"

# =========================================================
# STRIP SYMBOLS (SAFE FOR SWIFT)
# =========================================================
echo "🧹 Stripping symbols from framework binaries..."

find "$XCFRAMEWORK_NAME" -type f -name "$FRAMEWORK_NAME" | while read -r BIN; do
  echo "➡️ Stripping $(basename "$BIN")"
  strip -x "$BIN"
done

echo "✅ Symbol stripping complete"

# =========================================================
# MOVE XCFRAMEWORK TO OUTPUT
# =========================================================
echo "📦 Moving XCFramework to output directory..."
mv "$XCFRAMEWORK_NAME" "$XCFRAMEWORK_PATH"

# =========================================================
# ZIP XCFRAMEWORK
# =========================================================
echo "🗜️ Zipping XCFramework..."
cd "$OUTPUT_DIR"
zip -r "$ZIP_NAME" "$XCFRAMEWORK_NAME" > /dev/null
cd ..

# =========================================================
# COMPUTE CHECKSUM
# =========================================================
echo "🔍 Computing SwiftPM checksum..."
CHECKSUM=$(swift package compute-checksum "$OUTPUT_DIR/$ZIP_NAME")

# =========================================================
# VERIFY MODULE STABILITY
# =========================================================
echo "🔍 Verifying Swift interfaces..."
INTERFACE_COUNT=$(find "$XCFRAMEWORK_PATH" -name "*.swiftinterface" | wc -l | tr -d ' ')
if [[ "$INTERFACE_COUNT" -eq 0 ]]; then
  echo "❌ ERROR: No .swiftinterface files found"
  exit 1
fi
echo "✅ Found $INTERFACE_COUNT .swiftinterface files"

# =========================================================
# DONE
# =========================================================
echo ""
echo "✅ BUILD COMPLETE"
echo "📦 XCFramework: $OUTPUT_DIR/$ZIP_NAME"
echo "🔑 Checksum: $CHECKSUM"
echo "📁 dSYMs: $DSYM_DIR"
echo ""
echo "📄 SwiftPM Package.swift snippet:"
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
            url: "<YOUR_RELEASE_URL>/$ZIP_NAME",
            checksum: "$CHECKSUM"
        )
    ]
)
EOF
