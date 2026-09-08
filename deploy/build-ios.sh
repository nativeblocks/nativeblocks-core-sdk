#!/usr/bin/env bash
# Assemble the distributable iOS SDK: three XCFrameworks plus the compiler
# sources, wrapped in a single SwiftPM package a consumer can actually depend on.
#
#   deploy/build-ios.sh                       # rebuild everything, emit local package
#   deploy/build-ios.sh --skip-build          # reuse platforms/ios/*/output
#   deploy/build-ios.sh --registry <base-url> # emit url:+checksum: manifest instead
#
# Edit versions.toml and run this; the version is fanned out automatically.
#
# Two manifest flavours come out of the same generator:
#
#   local     .binaryTarget(path:)  — consumable straight off disk, no hosting.
#             This is what you test with and what a tester gets handed.
#   registry  .binaryTarget(url:checksum:) — what gets published, pointing at
#             dist.nativeblocks.io. Identical in every other respect.
#
# The four packages under platforms/ios are wired together with .package(path:),
# which cannot be published. This flattens them into one package whose binary
# modules are XCFrameworks and whose compiler half stays source, because SwiftPM
# has no way to load a macro from a binary target.

source "$(dirname "${BASH_SOURCE[0]}")/lib/common.sh"

IOS_DIR="$NB_ROOT/platforms/ios"
SKIP_BUILD=0
REGISTRY_BASE=""

while [ $# -gt 0 ]; do
    case "$1" in
        --skip-build) SKIP_BUILD=1; shift ;;
        --registry)   REGISTRY_BASE="${2:?--registry needs a base URL}"; shift 2 ;;
        *)            nb_die "unknown argument: $1" ;;
    esac
done

VERSION="$(nb_version ios)"
OUT="$NB_ROOT/deploy/build/ios/$VERSION"
PKG="$OUT/Nativeblocks"

# name : bundle id : source directory
ARTIFACTS=(
    "NativeblocksRuntime:io.nativeblocks.runtime:runtime"
    "NativeblocksFoundation:io.nativeblocks.foundation:foundation"
    "NativeblocksDevkit:io.nativeblocks.devkit:devkit"
)

nb_require swift xcodebuild zip python3

# versions.toml is the only file anyone edits by hand. The xcframework builds
# stamp CFBundleShortVersionString from each package's VERSION file, so fan the
# version out before building rather than discovering the mismatch after a full
# rebuild.
"$NB_DEPLOY_DIR/sync-versions.sh" || nb_die "could not apply versions.toml"

source "$IOS_DIR/nb-xcframework.sh"

# ---------------------------------------------------------------------------
# Build
# ---------------------------------------------------------------------------

if [ "$SKIP_BUILD" = "0" ]; then
    for artifact in "${ARTIFACTS[@]}"; do
        IFS=: read -r name _ dir <<< "$artifact"
        nb_step "building $name"
        ( cd "$IOS_DIR/$dir" && ./build_xcframework.sh )
    done
else
    nb_warn "--skip-build: reusing platforms/ios/*/output"
fi

# ---------------------------------------------------------------------------
# Verify and stage the binaries
# ---------------------------------------------------------------------------

nb_step "staging $VERSION"
rm -rf "$OUT"
mkdir -p "$PKG/XCFrameworks"

declare -a CHECKSUMS=()   # name<TAB>checksum, in ARTIFACTS order

for artifact in "${ARTIFACTS[@]}"; do
    IFS=: read -r name bundle_id dir <<< "$artifact"
    src="$IOS_DIR/$dir/output/$name.xcframework"
    zip="$IOS_DIR/$dir/output/$name.xcframework.zip"

    [ -d "$src" ] || nb_die "$src not found — run without --skip-build"
    [ -f "$zip" ] || nb_die "$zip not found — run without --skip-build"

    # Re-verify what is actually being shipped rather than trusting that the
    # output directory still holds what the build script last checked. This is
    # also what catches a VERSION bump that has not been rebuilt.
    nb_set_version
    nb_verify "$name" "$bundle_id" "$src"

    cp -R "$src" "$PKG/XCFrameworks/"
    cp "$zip" "$OUT/"
    CHECKSUMS+=("$name	$(swift package compute-checksum "$zip")")
done

# The three frameworks have to link together, with every object forced in, for
# every slice. A stripped static archive passes every plist and symbol check and
# can still fail here.
nb_step "link check"
for slice in "${NB_SLICES[@]}"; do
    nb_link_check "$PKG/XCFrameworks" "$slice" \
        NativeblocksRuntime NativeblocksFoundation NativeblocksDevkit
done

# ---------------------------------------------------------------------------
# Stage the compiler sources
# ---------------------------------------------------------------------------
# The macro plugin and NativeblocksTool are macOS host tools the consumer's
# toolchain builds itself, so they arrive as source. swift-syntax stays pinned
# at 509 so consumers on older Xcode can still compile it.

nb_step "staging compiler sources"
for item in Sources Plugins; do
    [ -e "$IOS_DIR/compiler/$item" ] || nb_die "compiler/$item not found"
    cp -R "$IOS_DIR/compiler/$item" "$PKG/"
done

if [ -e "$IOS_DIR/compiler/Package.resolved" ]; then
    cp "$IOS_DIR/compiler/Package.resolved" "$PKG/"
    nb_info "  swift-syntax pinned from compiler/Package.resolved"
else
    nb_warn "no compiler/Package.resolved — swift-syntax will float within 509.x"
    nb_warn "run 'swift package resolve' in platforms/ios/compiler first"
fi

find "$PKG" -name '.DS_Store' -delete

# ---------------------------------------------------------------------------
# Generate Package.swift
# ---------------------------------------------------------------------------

emit_binary_target() {
    local name="$1" checksum="$2"
    if [ -n "$REGISTRY_BASE" ]; then
        cat <<EOF
        .binaryTarget(
            name: "$name",
            url: "${REGISTRY_BASE%/}/$VERSION/$name.xcframework.zip",
            checksum: "$checksum"
        ),
EOF
    else
        cat <<EOF
        .binaryTarget(
            name: "$name",
            path: "XCFrameworks/$name.xcframework"
        ),
EOF
    fi
}

checksum_for() {
    local want="$1" line
    for line in "${CHECKSUMS[@]}"; do
        [ "${line%%	*}" = "$want" ] && { printf '%s\n' "${line#*	}"; return; }
    done
    nb_die "no checksum recorded for $want"
}

nb_step "generating Package.swift"
{
cat <<'EOF'
// swift-tools-version: 5.9
//
// Generated by deploy/build-ios.sh — do not edit.
//
// One package, four modules. Runtime, Foundation and Devkit are prebuilt
// XCFrameworks; the compiler half stays source because SwiftPM cannot load a
// macro implementation from a binary target.
//
// Note the multi-target library products: a binary target cannot declare its
// own dependencies, so each product must name every module it links against.

import CompilerPluginSupport
import PackageDescription

let package = Package(
    name: "Nativeblocks",
    platforms: [.iOS(.v15), .macOS(.v13)],
    products: [
        .library(
            name: "NativeblocksRuntime",
            targets: ["NativeblocksRuntime"]
        ),
        .library(
            name: "NativeblocksFoundation",
            targets: ["NativeblocksFoundation", "NativeblocksRuntime", "NativeblocksCompiler"]
        ),
        .library(
            name: "NativeblocksDevkit",
            targets: ["NativeblocksDevkit", "NativeblocksRuntime"]
        ),
        .library(
            name: "NativeblocksCompiler",
            targets: ["NativeblocksCompiler"]
        ),
        .plugin(name: "GenerateProvider", targets: ["GenerateProvider"]),
        .plugin(name: "Sync", targets: ["Sync"]),
        .plugin(name: "PrepareSchema", targets: ["PrepareSchema"]),
        .executable(name: "NativeblocksTool", targets: ["NativeblocksTool"]),
    ],
    dependencies: [
        .package(url: "https://github.com/apple/swift-syntax.git", from: "509.0.0")
    ],
    targets: [
EOF

for artifact in "${ARTIFACTS[@]}"; do
    IFS=: read -r name _ _ <<< "$artifact"
    emit_binary_target "$name" "$(checksum_for "$name")"
done

cat <<'EOF'
        .macro(
            name: "NativeblocksCompilerMacros",
            dependencies: [
                .product(name: "SwiftSyntaxMacros", package: "swift-syntax"),
                .product(name: "SwiftCompilerPlugin", package: "swift-syntax"),
                "_NativeblocksCompilerCommon",
            ]
        ),

        .plugin(
            name: "GenerateProvider",
            capability: .command(
                intent: .custom(
                    verb: "GenerateProvider",
                    description: "Generate `{Target}ActionProvider.swift` and `{Target}BlockProvider.swift` from Block and Action macros."),
                permissions: [
                    .writeToPackageDirectory(
                        reason: "This command write the new Provider to the source root.")
                ]
            ),
            dependencies: [
                .target(name: "NativeblocksTool")
            ]
        ),

        .plugin(
            name: "Sync",
            capability: .command(
                intent: .custom(
                    verb: "Sync",
                    description: "Synchronize JSON blocks and actions with the Nativeblocks Studio."),
                permissions: [
                    .writeToPackageDirectory(
                        reason: "This command writes the new JSON files to the source root."),
                    .allowNetworkConnections(
                        scope: PluginNetworkPermissionScope.all(),
                        reason: "This command synchronizes JSONs with the Nativeblocks server."),
                ]
            ),
            dependencies: [
                .target(name: "NativeblocksTool")
            ]
        ),

        .plugin(
            name: "PrepareSchema",
            capability: .command(
                intent: .custom(
                    verb: "PrepareSchema",
                    description: "Generate local JSON schemas for Blocks and Actions in the `.nativeblocks` directory."
                ),
                permissions: [
                    .writeToPackageDirectory(
                        reason: "This command write the new json blocks to the source root.")
                ]
            ),
            dependencies: [
                .target(name: "NativeblocksTool")
            ]
        ),

        .executableTarget(
            name: "NativeblocksTool",
            dependencies: [
                .product(name: "SwiftSyntax", package: "swift-syntax"),
                .product(name: "SwiftParser", package: "swift-syntax"),
                .product(name: "SwiftSyntaxBuilder", package: "swift-syntax"),
                "_NativeblocksCompilerCommon",
            ]
        ),

        .target(
            name: "NativeblocksCompiler",
            dependencies: ["NativeblocksCompilerMacros"]
        ),

        .target(
            name: "_NativeblocksCompilerCommon",
            dependencies: [
                .product(name: "SwiftSyntax", package: "swift-syntax"),
                .product(name: "SwiftParser", package: "swift-syntax"),
                .product(name: "SwiftSyntaxBuilder", package: "swift-syntax"),
            ]
        ),
    ]
)
EOF
} > "$PKG/Package.swift"

# A registry manifest must never carry unsafeFlags: SwiftPM refuses to resolve a
# versioned dependency whose targets use them, and the exemption explicitly
# excludes registry downloads. The source packages all set
# -enable-library-evolution this way, which is exactly why they ship as binaries.
if grep -q 'unsafeFlags' "$PKG/Package.swift"; then
    nb_die "generated manifest contains unsafeFlags — SwiftPM will refuse to resolve it"
fi

# ---------------------------------------------------------------------------
# Record what was built
# ---------------------------------------------------------------------------

{
    echo "{"
    echo "  \"version\": \"$VERSION\","
    echo "  \"builtAt\": \"$(date -u +%Y-%m-%dT%H:%M:%SZ)\","
    echo "  \"coreVersion\": \"$(nb_cargo_version)\","
    echo "  \"swift\": \"$(swift --version 2>/dev/null | head -1 | tr -d '"')\","
    echo "  \"xcode\": \"$(xcodebuild -version 2>/dev/null | head -1)\","
    echo "  \"flavour\": \"$([ -n "$REGISTRY_BASE" ] && echo registry || echo local)\","
    echo "  \"slices\": [$(printf '"%s", ' "${NB_SLICES[@]}" | sed 's/, $//')],"
    echo "  \"checksums\": {"
    printf '    "%s": "%s",\n' $(printf '%s\n' "${CHECKSUMS[@]}" | tr '\t' ' ') | sed '$ s/,$//'
    echo "  }"
    echo "}"
} > "$OUT/MANIFEST.json"

python3 -m json.tool "$OUT/MANIFEST.json" >/dev/null || nb_die "MANIFEST.json is not valid JSON"

nb_step "validating the generated package"
( cd "$PKG" && swift package dump-package >/dev/null ) \
    || nb_die "the generated Package.swift does not parse"
nb_ok "manifest parses"

nb_ok "iOS $VERSION staged at deploy/build/ios/$VERSION"
nb_info ""
nb_info "  consume it with:"
nb_info "    .package(path: \"$PKG\")"
