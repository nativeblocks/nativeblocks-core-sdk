#!/usr/bin/env bash
# Release a platform end to end: sync versions, check, build, verify, deploy.
#
#   deploy/release.sh android
#   deploy/release.sh ios
#   deploy/release.sh all
#   deploy/release.sh ios --dry-run    # everything except the upload
#   deploy/release.sh android --force  # overwrite an already-published version
#
# Edit versions.toml first. The Rust core is not built here: run
# runtime/scripts/release.sh and copy runtime/dist/ into platforms/ by hand
# beforehand when the core changed (see ANDROID.md / IOS.md).
#
# Stops at the first failing step, so nothing is uploaded unless the build
# verified.

source "$(dirname "${BASH_SOURCE[0]}")/lib/common.sh"

TARGET="${1:-}"
[ $# -gt 0 ] && shift
DEPLOY_ARGS=("$@")

case "$TARGET" in
    android|ios) PLATFORMS=("$TARGET") ;;
    all)         PLATFORMS=(android ios) ;;
    *)           nb_die "usage: $(basename "$0") <android|ios|all> [--dry-run] [--force]" ;;
esac

for arg in "${DEPLOY_ARGS[@]+"${DEPLOY_ARGS[@]}"}"; do
    case "$arg" in
        --dry-run|--force) ;;
        *) nb_die "unknown argument: $arg" ;;
    esac
done

nb_step "syncing versions"
"$NB_DEPLOY_DIR/sync-versions.sh"

for platform in "${PLATFORMS[@]}"; do
    version="$(nb_version "$platform")"

    nb_step "[$platform $version] check"
    "$NB_DEPLOY_DIR/check-version.sh" "$platform/v$version"

    nb_step "[$platform $version] build"
    "$NB_DEPLOY_DIR/build-$platform.sh"

    nb_step "[$platform $version] verify"
    "$NB_DEPLOY_DIR/verify-$platform.sh"

    nb_step "[$platform $version] deploy"
    "$NB_DEPLOY_DIR/deploy-$platform.sh" "${DEPLOY_ARGS[@]+"${DEPLOY_ARGS[@]}"}"

    nb_ok "$platform $version released"
done
