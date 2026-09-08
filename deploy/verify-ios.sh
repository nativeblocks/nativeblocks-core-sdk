#!/usr/bin/env bash
# Verify the staged iOS package the way a consumer meets it: resolve it, build
# it for a real device slice, and confirm the Rust ABI stayed unreachable.
#
#   deploy/verify-ios.sh
#   deploy/verify-ios.sh --keep    # leave derived data for inspection

source "$(dirname "${BASH_SOURCE[0]}")/lib/common.sh"

KEEP=0
[ "${1:-}" = "--keep" ] && KEEP=1

VERSION="$(nb_version ios)"
PKG="$NB_ROOT/deploy/build/ios/$VERSION/Nativeblocks"

[ -d "$PKG" ] || nb_die "no staged package — run deploy/build-ios.sh first"
nb_require xcodebuild swift

WORK="$(mktemp -d)"
cleanup() { [ "$KEEP" = "1" ] || rm -rf "$WORK"; }
trap cleanup EXIT

# ---------------------------------------------------------------------------
# 1. The manifest resolves
# ---------------------------------------------------------------------------

nb_step "resolving $PKG"
( cd "$PKG" && swift package resolve ) || nb_die "package failed to resolve"
nb_ok "resolved"

# ---------------------------------------------------------------------------
# 2. It builds for a device slice, macro and all
# ---------------------------------------------------------------------------
# Foundation is the useful scheme here: it pulls in the Runtime binary target
# and the source macro at once, so one build covers both halves of the package.

nb_step "building NativeblocksFoundation for iOS"
( cd "$PKG" && xcodebuild -scheme NativeblocksFoundation \
    -destination 'generic/platform=iOS' \
    -derivedDataPath "$WORK/dd" -skipMacroValidation build ) \
    > "$WORK/build.log" 2>&1 || {
        tail -30 "$WORK/build.log" >&2
        nb_die "build failed — see above"
    }
nb_ok "built for iphoneos"

[ -d "$WORK/dd/Build/Products/Debug-iphoneos/NativeblocksFoundation.framework" ] \
    || nb_die "no NativeblocksFoundation.framework for iphoneos"

# The macro is a host executable the compiler loads. If it is missing, the
# macro silently did not participate in the build.
[ -f "$WORK/dd/Build/Products/Debug/NativeblocksCompilerMacros" ] \
    || nb_die "the macro plugin executable was never built"
nb_ok "macro plugin built and loaded"

# ---------------------------------------------------------------------------
# 3. The FFI module stayed sealed
# ---------------------------------------------------------------------------
# The Rust ABI is merged into the framework binary but its .swiftmodule is never
# shipped, so downstream code cannot import it. Two independent checks, because
# either one alone can pass on a broken build.

nb_step "checking the FFI seal"

leaks=0
while IFS= read -r interface; do
    if grep -q "NativeblocksRuntimeFFI" "$interface"; then
        nb_warn "FFI leaked into $(basename "$interface")"
        leaks=$((leaks + 1))
    fi
done < <(find "$PKG/XCFrameworks" -name '*.swiftinterface')
[ "$leaks" -eq 0 ] || nb_die "$leaks public interface(s) reference the FFI module"
nb_ok "no FFI references in any public interface"

if find "$PKG/XCFrameworks" -name 'NativeblocksRuntimeFFI.swiftmodule' | grep -q .; then
    nb_die "NativeblocksRuntimeFFI.swiftmodule is being shipped"
fi
nb_ok "FFI swiftmodule not shipped"

# The real test: importing it must not compile.
mkdir -p "$WORK/seal"
cat > "$WORK/seal/main.swift" <<'EOF'
import NativeblocksRuntimeFFI
EOF

SDK="$(xcrun --sdk iphoneos --show-sdk-path)"
if xcrun --sdk iphoneos swiftc -target arm64-apple-ios15.0 -sdk "$SDK" \
        -F "$PKG/XCFrameworks/NativeblocksRuntime.xcframework/ios-arm64" \
        -emit-module -o /dev/null "$WORK/seal/main.swift" 2>/dev/null; then
    nb_die "import NativeblocksRuntimeFFI compiled — the Rust ABI is reachable"
fi
nb_ok "import NativeblocksRuntimeFFI does not resolve"

nb_ok "ios $VERSION verified"
