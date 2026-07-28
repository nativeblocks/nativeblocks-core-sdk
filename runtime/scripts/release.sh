#!/usr/bin/env bash
# Build every available platform and stage copy-ready artifacts under dist/. Usage: ./scripts/release.sh [android] [ios]
set -euo pipefail

cd "$(dirname "$0")/.."

DIST="dist"
VERSION="$(grep -m1 '^version' Cargo.toml | sed -E 's/.*"(.*)".*/\1/')"
OS="$(uname -s)"

REQUESTED=("$@")
[[ ${#REQUESTED[@]} -eq 0 ]] && REQUESTED=(android ios)

for p in "${REQUESTED[@]}"; do
  case "$p" in
    android|ios) ;;
    *)
      echo "ERROR: unknown platform '$p' (expected: android, ios)" >&2
      exit 2 ;;
  esac
done

want() { printf '%s\n' "${REQUESTED[@]}" | grep -qx "$1"; }

BUILT=()
SKIPPED=()

have() { command -v "$1" >/dev/null 2>&1; }
target_installed() { rustup target list --installed 2>/dev/null | grep -qx "$1"; }

echo "=================================================="
echo " nativeblocks-core-sdk release  v$VERSION  ($OS)"
echo "=================================================="
echo "==> Cleaning $DIST/"
rm -rf "$DIST"
mkdir -p "$DIST"
echo "$VERSION" > "$DIST/VERSION"

run_platform() {
  local name="$1" builder="$2" reason="$3" ready="$4"
  if [[ "$ready" != "1" ]]; then
    SKIPPED+=("$name ($reason)")
    return
  fi
  echo "==> [$name] building"
  if "$builder"; then
    BUILT+=("$name")
  else
    rm -rf "${DIST:?}/$name"
    SKIPPED+=("$name (build failed — see output above)")
  fi
}

if want android; then
  ready=0
  if have cargo-ndk && target_installed aarch64-linux-android; then ready=1; fi
  run_platform android ./scripts/build-android.sh \
    "install: cargo install cargo-ndk + rustup target add the android triples" "$ready"
  [[ -d "$DIST/android" ]] && cat > "$DIST/android/COPY-INSTRUCTIONS.txt" <<'EOF'
COPY INTO YOUR ANDROID LIBRARY MODULE:

  dist/android/jniLibs/*   ->  <module>/src/main/jniLibs/
  dist/android/java/*      ->  <module>/src/main/java/

THEN add to the module's build.gradle.kts dependencies:

  implementation("net.java.dev.jna:jna:5.14.0@aar")
EOF
fi

if want ios; then
  ready=0
  if [[ "$OS" == "Darwin" ]] && have xcodebuild && target_installed aarch64-apple-ios; then ready=1; fi
  run_platform ios ./scripts/build-ios.sh \
    "needs macOS + Xcode + rustup target add the apple-ios triples" "$ready"
  [[ -d "$DIST/ios" ]] && cat > "$DIST/ios/COPY-INSTRUCTIONS.txt" <<'EOF'
ADD TO YOUR XCODE TARGET:

  dist/ios/NativeblocksRuntimeCFFI.xcframework  ->  binaryTarget "NativeblocksRuntimeCFFI"
  dist/ios/NativeblocksRuntimeFFI.swift         ->  Sources/NativeblocksRuntimeFFI/

The generated Swift is sealed to `package` visibility: put it in its own
NativeblocksRuntimeFFI target, in the SAME package as the host target. The host
target (NativeblocksRuntime) is the only product consumers can import.
EOF
fi

cat > "$DIST/COPY-GUIDE.md" <<EOF
# Release v$VERSION — copy guide

Built on $OS. Each platform folder has a COPY-INSTRUCTIONS.txt with exact paths.

| Platform | Artifact folder       | Copy into the host project                       |
| -------- | --------------------- | ------------------------------------------------ |
| Android  | \`dist/android/\`     | \`jniLibs/\` + \`java/\` of a library module     |
| iOS      | \`dist/ios/\`         | the \`.xcframework\` + \`.swift\` into the target |

See README.md for the copy steps and the host SDK boundary rules.
EOF

echo
echo "=================================================="
echo " DONE  v$VERSION"
echo "=================================================="
[[ ${#BUILT[@]}   -gt 0 ]] && printf '  built:   %s\n' "${BUILT[*]}"
[[ ${#SKIPPED[@]} -gt 0 ]] && for s in "${SKIPPED[@]}"; do printf '  skipped: %s\n' "$s"; done
echo
echo "  Artifacts in ./$DIST/  — read $DIST/COPY-GUIDE.md"
