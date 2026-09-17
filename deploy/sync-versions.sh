#!/usr/bin/env bash
# Apply versions.toml to the one place that cannot read it itself.
#
#   deploy/sync-versions.sh            # write the catalogs
#   deploy/sync-versions.sh --check    # exit 1 if anything is out of date
#
# build-android.sh and build-ios.sh call this themselves, so you normally only
# edit versions.toml and run a build.
#
# Gradle build scripts and the iOS helpers both parse versions.toml directly.
# Only Gradle *version catalogs* cannot — they are static TOML with no way to
# read another file — so their nativeblocks* entries are rewritten here.

source "$(dirname "${BASH_SOURCE[0]}")/lib/common.sh"

CHECK_ONLY=false
[ "${1:-}" = "--check" ] && CHECK_ONLY=true

drift=0
wrote=0

# The Gradle builds consume each other through version catalogs, and a catalog
# is static TOML that cannot read a VERSION file. Those `nativeblocks*` entries
# are therefore a third copy of the version and have to be rewritten here, or
# foundation goes looking for a plugin version that was never published.
nb_step "android catalogs"
android_version="$(nb_version android)"

for catalog in "$NB_ROOT"/platforms/android/{foundation,devkit,sample}/gradle/libs.versions.toml; do
    [ -f "$catalog" ] || continue
    rel="${catalog#"$NB_ROOT"/}"

    stale="$(awk -v want="$android_version" '
        /^nativeblocks[A-Za-z]*[[:space:]]*=[[:space:]]*"/ {
            line = $0
            sub(/^[^=]*=[[:space:]]*"/, "", line)
            sub(/".*$/, "", line)
            if (line != want) print
        }' "$catalog")"

    if [ -z "$stale" ]; then
        nb_info "  $rel — $android_version"
        continue
    fi

    if $CHECK_ONLY; then
        while IFS= read -r line; do
            nb_warn "$rel: ${line} (expected $android_version)"
            drift=$((drift + 1))
        done <<< "$stale"
        continue
    fi

    python3 - "$catalog" "$android_version" <<'PY'
import re, sys
path, want = sys.argv[1], sys.argv[2]
with open(path) as fh:
    text = fh.read()
new = re.sub(r'^(nativeblocks[A-Za-z]*\s*=\s*)"[^"]*"',
             lambda m: f'{m.group(1)}"{want}"', text, flags=re.M)
with open(path, "w") as fh:
    fh.write(new)
PY
    count="$(printf '%s\n' "$stale" | wc -l | tr -d ' ')"
    nb_info "  $rel — $count entr(y|ies) -> $android_version"
    wrote=$((wrote + 1))
done

nb_step "ios SDKConfig"
ios_version="$(nb_version ios)"
sdk_config="$NB_ROOT/platforms/ios/runtime/Sources/NativeblocksRuntime/api/util/SDKConfig.swift"
rel="${sdk_config#"$NB_ROOT"/}"
ios_have="$(sed -nE 's/.*SDK_VERSION: String = "([^"]*)".*/\1/p' "$sdk_config")"

if [ "$ios_have" = "$ios_version" ]; then
    nb_info "  $rel — $ios_version"
elif $CHECK_ONLY; then
    nb_warn "$rel: '$ios_have' (expected $ios_version)"
    drift=$((drift + 1))
else
    sed -i '' -E "s/(SDK_VERSION: String = )\"[^\"]*\"/\1\"$ios_version\"/" "$sdk_config"
    nb_info "  $rel — $ios_have -> $ios_version"
    wrote=$((wrote + 1))
fi

# The Rust core is versioned by Cargo.toml, which we assert against rather than
# rewrite — editing a Cargo manifest from a shell script invites a bad day.
nb_step "core"
core_want="$(nb_version core)"
core_have="$(nb_cargo_version)"
if [ "$core_want" = "$core_have" ]; then
    nb_info "  runtime/Cargo.toml — $core_have"
else
    nb_warn "runtime/Cargo.toml is '$core_have', versions.toml says '$core_want'"
    nb_warn "edit one of them by hand so they agree"
    drift=$((drift + 1))
fi

if $CHECK_ONLY; then
    [ "$drift" -eq 0 ] || nb_die "$drift version file(s) out of sync — run deploy/sync-versions.sh"
    nb_ok "all version files match versions.toml"
else
    [ "$drift" -eq 0 ] || nb_die "$drift version(s) need a manual fix (see above)"
    nb_ok "synced ${wrote} file(s) from versions.toml"
fi
