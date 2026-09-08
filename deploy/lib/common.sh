#!/usr/bin/env bash
# Shared helpers for the release scripts. Source, do not execute.
#
#   source "$(dirname "$0")/lib/common.sh"

set -euo pipefail

# ---------------------------------------------------------------------------
# Paths
# ---------------------------------------------------------------------------

# Resolve the repo root from this file's location, so scripts work from any cwd.
NB_LIB_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
NB_DEPLOY_DIR="$(cd "$NB_LIB_DIR/.." && pwd)"
NB_ROOT="$(cd "$NB_DEPLOY_DIR/.." && pwd)"
NB_VERSIONS_TOML="$NB_ROOT/versions.toml"

# iOS has no VERSION files: nb_set_version in platforms/ios/nb-xcframework.sh
# reads [versions].ios from versions.toml directly.

# ---------------------------------------------------------------------------
# Output
# ---------------------------------------------------------------------------

if [ -t 1 ]; then
    NB_C_RED=$'\033[0;31m'; NB_C_GREEN=$'\033[0;32m'
    NB_C_YELLOW=$'\033[0;33m'; NB_C_DIM=$'\033[2m'; NB_C_OFF=$'\033[0m'
else
    NB_C_RED=""; NB_C_GREEN=""; NB_C_YELLOW=""; NB_C_DIM=""; NB_C_OFF=""
fi

nb_info()  { printf '%s\n' "$*"; }
nb_step()  { printf '\n%s==>%s %s\n' "$NB_C_DIM" "$NB_C_OFF" "$*"; }
nb_ok()    { printf '%s✓%s %s\n' "$NB_C_GREEN" "$NB_C_OFF" "$*"; }
nb_warn()  { printf '%s!%s %s\n' "$NB_C_YELLOW" "$NB_C_OFF" "$*" >&2; }
nb_die()   { printf '%s✗%s %s\n' "$NB_C_RED" "$NB_C_OFF" "$*" >&2; exit 1; }

# nb_require <tool>...  — fail listing every missing tool, not just the first.
nb_require() {
    local missing=() tool
    for tool in "$@"; do
        command -v "$tool" >/dev/null 2>&1 || missing+=("$tool")
    done
    [ ${#missing[@]} -eq 0 ] || nb_die "missing required tool(s): ${missing[*]}"
}

# ---------------------------------------------------------------------------
# Environment
# ---------------------------------------------------------------------------

# nb_load_env — read deploy/.env into the environment if it exists.
# Anything already exported wins, so CI can set variables without editing files.
nb_load_env() {
    local envfile="$NB_DEPLOY_DIR/.env"
    [ -f "$envfile" ] || return 0

    local key value line
    while IFS= read -r line || [ -n "$line" ]; do
        case "$line" in ''|'#'*) continue ;; esac
        line="${line%$'\r'}"                    # tolerate CRLF files
        line="${line#export }"
        key="${line%%=*}"
        value="${line#*=}"
        [ "$key" = "$line" ] && continue

        key="${key%"${key##*[![:space:]]}"}"    # trim space around the key
        key="${key#"${key%%[![:space:]]*}"}"
        value="${value#"${value%%[![:space:]]*}"}"

        # strip one layer of matching quotes, else drop any trailing comment
        case "$value" in
            \"*\") value="${value#\"}"; value="${value%\"}" ;;
            \'*\') value="${value#\'}"; value="${value%\'}" ;;
            *)      value="${value%%[[:space:]]#*}" ;;
        esac
        value="${value%"${value##*[![:space:]]}"}"

        case "$key" in ''|*[!A-Za-z0-9_]*) continue ;; esac

        [ -n "${!key:-}" ] || export "$key=$value"
    done < "$envfile"
}

# ---------------------------------------------------------------------------
# versions.toml
# ---------------------------------------------------------------------------

# nb_version <android|ios|core>  — echo the value from [versions].
# Scoped to the [versions] table so a like-named key elsewhere cannot win.
nb_version() {
    local key="$1" value
    [ -f "$NB_VERSIONS_TOML" ] || nb_die "not found: $NB_VERSIONS_TOML"

    value="$(awk -v key="$key" '
        /^[[:space:]]*\[/ { in_versions = ($0 ~ /^[[:space:]]*\[versions\][[:space:]]*$/); next }
        !in_versions { next }
        {
            line = $0
            sub(/#.*/, "", line)
            if (match(line, "^[[:space:]]*" key "[[:space:]]*=")) {
                sub("^[[:space:]]*" key "[[:space:]]*=[[:space:]]*", "", line)
                gsub(/^"|"[[:space:]]*$/, "", line)
                gsub(/[[:space:]]+$/, "", line)
                print line
                exit
            }
        }
    ' "$NB_VERSIONS_TOML")"

    [ -n "$value" ] || nb_die "no [versions].$key in $NB_VERSIONS_TOML"
    printf '%s\n' "$value"
}

# nb_cargo_version — the version declared in runtime/Cargo.toml [package].
nb_cargo_version() {
    local cargo="$NB_ROOT/runtime/Cargo.toml" value
    [ -f "$cargo" ] || nb_die "not found: $cargo"

    value="$(awk '
        /^[[:space:]]*\[/ { in_package = ($0 ~ /^[[:space:]]*\[package\][[:space:]]*$/); next }
        !in_package { next }
        /^[[:space:]]*version[[:space:]]*=/ {
            line = $0
            sub(/#.*/, "", line)
            sub(/^[[:space:]]*version[[:space:]]*=[[:space:]]*/, "", line)
            gsub(/^"|"[[:space:]]*$/, "", line)
            gsub(/[[:space:]]+$/, "", line)
            print line
            exit
        }
    ' "$cargo")"

    [ -n "$value" ] || nb_die "no [package].version in $cargo"
    printf '%s\n' "$value"
}

# ---------------------------------------------------------------------------
# Tags
# ---------------------------------------------------------------------------

# nb_parse_tag <tag>  — split "android/v1.0.0-beta1" into NB_PLATFORM/NB_VERSION.
# A bare platform name also works, resolving the version from versions.toml, so
# the scripts can be run ad hoc without inventing a tag.
nb_parse_tag() {
    local tag="${1:-}"
    [ -n "$tag" ] || nb_die "usage: $(basename "${0:-script}") <android|ios|core>[/v<version>]"

    case "$tag" in
        */v*) NB_PLATFORM="${tag%%/*}"; NB_VERSION="${tag#*/v}" ;;
        */*)  nb_die "malformed tag '$tag' — expected <platform>/v<version>" ;;
        *)    NB_PLATFORM="$tag"; NB_VERSION="$(nb_version "$tag")" ;;
    esac

    case "$NB_PLATFORM" in
        android|ios|core) ;;
        *) nb_die "unknown platform '$NB_PLATFORM' — expected android, ios or core" ;;
    esac

    [ -n "$NB_VERSION" ] || nb_die "empty version in tag '$tag'"
    export NB_PLATFORM NB_VERSION
}
