#!/usr/bin/env bash
# Assert the release tag agrees with versions.toml and with every VERSION file
# that the builds actually read. Run before anything is built or uploaded.
#
#   deploy/check-version.sh android/v1.0.0-beta1
#   deploy/check-version.sh ios              # resolve version from versions.toml

source "$(dirname "${BASH_SOURCE[0]}")/lib/common.sh"

nb_parse_tag "${1:-}"

nb_step "$NB_PLATFORM $NB_VERSION"

declared="$(nb_version "$NB_PLATFORM")"
if [ "$NB_VERSION" != "$declared" ]; then
    nb_die "tag says '$NB_VERSION' but versions.toml [versions].$NB_PLATFORM says '$declared'"
fi
nb_ok "versions.toml agrees"

if [ "$NB_PLATFORM" = "core" ]; then
    exit 0
fi

# Neither platform keeps VERSION files any more: the Gradle scripts and the iOS
# helpers both parse versions.toml directly, so the tag matching versions.toml
# above is the whole check. Only the catalogs can drift, and sync-versions.sh
# owns those.
"$NB_DEPLOY_DIR/sync-versions.sh" --check >/dev/null 2>&1 \
    || nb_die "version catalogs are stale — run deploy/sync-versions.sh"
nb_ok "version catalogs agree"
nb_ok "embedded core version $(nb_version core)"
