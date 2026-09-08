#!/usr/bin/env bash
# Publish the iOS SDK to R2: XCFrameworks to the dist domain, and an SE-0292
# Swift Package Registry to the swift domain.
#
#   deploy/deploy-ios.sh              # guard, upload, verify
#   deploy/deploy-ios.sh --dry-run    # build the registry files, upload nothing
#   deploy/deploy-ios.sh --force      # overwrite an already-published version
#
# Reads deploy/.env. Run deploy/build-ios.sh first.
#
# This re-runs build-ios.sh --skip-build --registry to regenerate Package.swift
# with url:+checksum: binary targets instead of local paths. The XCFrameworks
# themselves are reused, so what was verified locally is what ships.

source "$(dirname "${BASH_SOURCE[0]}")/lib/common.sh"
source "$(dirname "${BASH_SOURCE[0]}")/lib/r2.sh"
nb_load_env

SWIFT_PUBLIC_URL="${SWIFT_PUBLIC_URL:-https://binaries.nativeblocks.io}"
DIST_PUBLIC_URL="${DIST_PUBLIC_URL:-https://dist.nativeblocks.io}"

# Package identity. Scope and name split on the first dot, which is what
# decides the registry URL path.
PACKAGE_SCOPE="nativeblocks"
PACKAGE_NAME="sdk"
PACKAGE_ID="$PACKAGE_SCOPE.$PACKAGE_NAME"

DRY_RUN=0
NB_FORCE=0
while [ $# -gt 0 ]; do
    case "$1" in
        --dry-run) DRY_RUN=1; shift ;;
        --force)   NB_FORCE=1; shift ;;
        *)         nb_die "unknown argument: $1" ;;
    esac
done

VERSION="$(nb_version ios)"
OUT="$NB_ROOT/deploy/build/ios/$VERSION"
PKG="$OUT/Nativeblocks"

nb_require aws curl swift python3 zip

[ -d "$PKG" ] || nb_die "nothing staged — run deploy/build-ios.sh first"

nb_r2_init

REGISTRY_PREFIX="$PACKAGE_SCOPE/$PACKAGE_NAME"
BINARY_PREFIX="ios/$VERSION"

nb_step "target"
nb_info "  bucket:   s3://$R2_BUCKET"
nb_info "  package:  $PACKAGE_ID @ $VERSION"
nb_info "  binaries: $DIST_PUBLIC_URL/$BINARY_PREFIX/"
nb_info "  registry: $SWIFT_PUBLIC_URL/$REGISTRY_PREFIX"

nb_r2_guard "$VERSION" "$BINARY_PREFIX/" "$REGISTRY_PREFIX/$VERSION"

# --- regenerate the manifest against the dist domain ------------------------

nb_step "regenerating Package.swift with remote binary targets"
"$NB_DEPLOY_DIR/build-ios.sh" --skip-build --registry "$DIST_PUBLIC_URL/ios" >/dev/null \
    || nb_die "could not regenerate the registry manifest"
grep -q 'url: "https://' "$PKG/Package.swift" \
    || nb_die "generated manifest still uses local paths"
nb_ok "manifest points at $DIST_PUBLIC_URL/ios/$VERSION"

# --- source archive ---------------------------------------------------------
# SwiftPM requires exactly one top-level directory in the archive: zero or two
# is a hard error, and root-level files next to one directory are silently
# discarded. The XCFrameworks are excluded — consumers fetch those by URL.

nb_step "building the source archive"
ARCHIVE="$OUT/$VERSION.zip"
rm -f "$ARCHIVE"

archive_src="$(mktemp -d)"
cp -R "$PKG" "$archive_src/Nativeblocks"
rm -rf "$archive_src/Nativeblocks/XCFrameworks"
find "$archive_src" -name '.DS_Store' -delete
( cd "$archive_src" && zip -ry -q "$ARCHIVE" Nativeblocks ) || nb_die "could not build the source archive"
rm -rf "$archive_src"

roots="$(unzip -Z1 "$ARCHIVE" | awk -F/ '{print $1}' | sort -u | wc -l | tr -d ' ')"
[ "$roots" = "1" ] || nb_die "archive must have exactly one top-level directory, found $roots"

# Lowercase hex sha256, which is what SwiftPM compares and pins.
CHECKSUM="$(swift package compute-checksum "$ARCHIVE")"
nb_info "  $(basename "$ARCHIVE")  $(du -h "$ARCHIVE" | cut -f1)  sha256 $CHECKSUM"

# --- registry metadata ------------------------------------------------------

nb_step "generating registry metadata"
META="$OUT/release-metadata.json"
python3 - "$META" "$PACKAGE_ID" "$VERSION" "$CHECKSUM" <<'PY'
import json, sys
path, pkg_id, version, checksum = sys.argv[1:5]
# publishedAt is deliberately omitted: SwiftPM decodes dates with .iso8601,
# which rejects fractional seconds, and a bad value fails the whole decode.
doc = {
    "id": pkg_id,
    "version": version,
    "resources": [
        {"name": "source-archive", "type": "application/zip", "checksum": checksum}
    ],
    "metadata": {
        "description": "Nativeblocks Server-Driven UI SDK for iOS",
        "licenseURL": "https://nativeblocks.io/terms-of-service",
        "repositoryURLs": [],
    },
}
with open(path, "w") as fh:
    json.dump(doc, fh, indent=2)
PY

# The release list accumulates every version, so merge into whatever is already
# published rather than overwriting it with just this release.
LIST="$OUT/release-list.json"
EXISTING="$OUT/.release-list-remote.json"
rm -f "$EXISTING"
nb_r2 s3api get-object --bucket "$R2_BUCKET" --key "$REGISTRY_PREFIX" "$EXISTING" >/dev/null 2>&1 || true

python3 - "$LIST" "$EXISTING" "$VERSION" "$SWIFT_PUBLIC_URL/$REGISTRY_PREFIX" <<'PY'
import json, os, sys
out, existing, version, base = sys.argv[1:5]
releases = {}
if os.path.exists(existing):
    try:
        releases = json.load(open(existing)).get("releases", {}) or {}
    except Exception:
        releases = {}
releases[version] = {"url": f"{base}/{version}"}
with open(out, "w") as fh:
    json.dump({"releases": releases}, fh, indent=2)
print("  versions in list: " + ", ".join(sorted(releases)))
PY
rm -f "$EXISTING"

if [ "$DRY_RUN" = "1" ]; then
    nb_ok "dry run — built archive and metadata, uploaded nothing"
    nb_info "  $ARCHIVE"
    nb_info "  $META"
    nb_info "  $LIST"
    exit 0
fi

# --- upload, in spec order --------------------------------------------------
# The release list goes last, always: no version becomes discoverable before
# every file it points at exists.

nb_step "1/5 binaries"
for zipfile in "$OUT"/*.xcframework.zip; do
    nb_r2_put "$zipfile" "$BINARY_PREFIX/$(basename "$zipfile")" "application/zip"
done

nb_step "2/5 source archive"
nb_r2_put "$ARCHIVE" "$REGISTRY_PREFIX/$VERSION.zip" "application/zip"

nb_step "3/5 manifest"
nb_r2_put "$PKG/Package.swift" "$REGISTRY_PREFIX/$VERSION/Package.swift" "text/x-swift"

nb_step "4/5 release metadata"
nb_r2_put "$META" "$REGISTRY_PREFIX/$VERSION" "application/json"

nb_step "5/5 release list"
nb_r2_put "$LIST" "$REGISTRY_PREFIX" "application/json" "$NB_CACHE_INDEX"

# --- verify over the public domains -----------------------------------------

nb_step "checking the public endpoints"
nb_http_check "$SWIFT_PUBLIC_URL/$REGISTRY_PREFIX"                     "release list"     "application/json"
nb_http_check "$SWIFT_PUBLIC_URL/$REGISTRY_PREFIX/$VERSION"            "release metadata" "application/json"
nb_http_check "$SWIFT_PUBLIC_URL/$REGISTRY_PREFIX/$VERSION/Package.swift" "manifest"      "text/x-swift"
nb_http_check "$SWIFT_PUBLIC_URL/$REGISTRY_PREFIX/$VERSION.zip"        "source archive"   "application/zip"
for zipfile in "$OUT"/*.xcframework.zip; do
    check "$DIST_PUBLIC_URL/$BINARY_PREFIX/$(basename "$zipfile")" "$(basename "$zipfile")" "application/zip"
done

version_header="$(curl -sI "$SWIFT_PUBLIC_URL/$REGISTRY_PREFIX" | awk -F': ' 'tolower($1)=="content-version"{print $2}' | tr -d '\r')"
if [ "$version_header" = "1" ]; then
    nb_info "  Content-Version: 1"
else
    nb_warn "Content-Version is '${version_header:-<missing>}', expected '1' — SwiftPM will reject this"
    NB_CHECK_FAILED=$((NB_CHECK_FAILED + 1))
fi

[ "$NB_CHECK_FAILED" -eq 0 ] || nb_die "$NB_CHECK_FAILED check(s) failed — see above"

nb_ok "ios $VERSION published"
nb_info ""
nb_info "  consumers run once:"
nb_info "    swift package-registry set $SWIFT_PUBLIC_URL"
nb_info ""
nb_info "  then depend on it:"
nb_info "    .package(id: \"$PACKAGE_ID\", exact: \"$VERSION\")"
