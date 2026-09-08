#!/usr/bin/env bash
# Package NativeblocksFoundation — the block, action and modifier library — as a static xcframework.
#
# Name, bundle id, version, deployment target, symbol stripping and the
# verification pass all live in ../nb-xcframework.sh so that this artifact
# is stamped identically to the other three.
#
# Prereqs:  xccache  (gem install xccache)
# Usage:    ./build_xcframework.sh
set -euo pipefail

cd "$(dirname "$0")"
source ../nb-xcframework.sh
nb_set_version

FRAMEWORK_NAME="NativeblocksFoundation"
BUNDLE_ID="io.nativeblocks.foundation"

OUTPUT_DIR="output"
XCFRAMEWORK_PATH="$OUTPUT_DIR/$FRAMEWORK_NAME.xcframework"

nb_require xccache strip plutil zip swift

echo "🧹 Cleaning previous build artifacts..."
rm -rf "$OUTPUT_DIR"
mkdir -p "$OUTPUT_DIR"
nb_clean_build .

nb_build "$OUTPUT_DIR" "$FRAMEWORK_NAME"

[ -d "$XCFRAMEWORK_PATH" ] || {
  echo "❌ xccache did not produce $XCFRAMEWORK_PATH"
  exit 1
}

echo "🔍 Checking every object traces to a source file..."
for slice in "${NB_SLICES[@]}"; do
  nb_check_sources "Sources/$FRAMEWORK_NAME" \
    "$XCFRAMEWORK_PATH/$slice/$FRAMEWORK_NAME.framework/$FRAMEWORK_NAME"
done

nb_finalize "$FRAMEWORK_NAME" "$BUNDLE_ID" "$XCFRAMEWORK_PATH"
nb_verify   "$FRAMEWORK_NAME" "$BUNDLE_ID" "$XCFRAMEWORK_PATH"
nb_package  "$FRAMEWORK_NAME" "$XCFRAMEWORK_PATH" "$OUTPUT_DIR"
