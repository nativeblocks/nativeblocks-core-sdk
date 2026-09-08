#!/usr/bin/env bash
# Cloudflare R2 access over the S3-compatible API. Source, do not execute.
#
# Requires lib/common.sh to be sourced first, and reads deploy/.env via
# nb_load_env for R2_URL, R2_BUCKET, R2_ACCESS_KEY_ID, R2_SECRET_ACCESS_KEY.
#
# Gradle's built-in s3:// transport cannot be used: it resolves AWS endpoints
# and offers no override. Everything here uploads an already-built local tree
# instead, which is also what makes the immutability guard possible.

# Everything under a version path is immutable; the indexes that change on every
# release are not.
NB_CACHE_IMMUTABLE="public, max-age=31536000, immutable"
NB_CACHE_INDEX="public, max-age=60"

# nb_r2_init — validate configuration and prepare the aws environment.
nb_r2_init() {
    nb_require aws

    R2_URL="${R2_URL:-}"
    R2_ACCESS_KEY_ID="${R2_ACCESS_KEY_ID:-}"
    R2_SECRET_ACCESS_KEY="${R2_SECRET_ACCESS_KEY:-}"

    [ -n "$R2_URL" ] || nb_die "R2_URL is unset — copy deploy/.env.example to deploy/.env"
    case "$R2_URL" in
        *xxxx*) nb_die "R2_URL still has the placeholder account id — edit deploy/.env" ;;
    esac

    # The bucket may be on the URL path or in R2_BUCKET. Cloudflare's dashboard
    # shows the account-level endpoint with no bucket, so accept both rather
    # than making the pasted value wrong.
    local scheme rest host path
    scheme="${R2_URL%%://*}"
    rest="${R2_URL#*://}"
    host="${rest%%/*}"
    path="${rest#"$host"}"
    path="${path#/}"

    NB_R2_ENDPOINT="$scheme://$host"
    if [ -n "${path%%/*}" ]; then R2_BUCKET="${path%%/*}"; else R2_BUCKET="${R2_BUCKET:-}"; fi

    # aws only reports a bad bucket as an unreadable regex failure deep inside
    # the upload, so validate here where the message can be useful.
    case "$R2_BUCKET" in
        "") nb_die "no bucket — append it to R2_URL (…r2.cloudflarestorage.com/<bucket>) or set R2_BUCKET in deploy/.env" ;;
        *[!a-zA-Z0-9._-]*) nb_die "invalid bucket name '$R2_BUCKET' — check R2_URL / R2_BUCKET in deploy/.env" ;;
    esac

    [ -n "$R2_ACCESS_KEY_ID" ]     && export AWS_ACCESS_KEY_ID="$R2_ACCESS_KEY_ID"
    [ -n "$R2_SECRET_ACCESS_KEY" ] && export AWS_SECRET_ACCESS_KEY="$R2_SECRET_ACCESS_KEY"
    local var
    for var in AWS_ACCESS_KEY_ID AWS_SECRET_ACCESS_KEY; do
        [ -n "${!var:-}" ] || nb_die "no credentials — set R2_ACCESS_KEY_ID and R2_SECRET_ACCESS_KEY in deploy/.env"
    done

    export AWS_DEFAULT_REGION="auto"
    # aws-cli 2.17+ sends integrity headers R2 rejects; without these you get
    # opaque 400s partway through an upload.
    export AWS_REQUEST_CHECKSUM_CALCULATION="when_required"
    export AWS_RESPONSE_CHECKSUM_VALIDATION="when_required"

    export NB_R2_ENDPOINT R2_BUCKET

    # deploy/.env is gitignored, but a tracked copy would be one `git add -A`
    # away from publishing the token.
    if [ -f "$NB_DEPLOY_DIR/.env" ] && \
       git -C "$NB_ROOT" ls-files --error-unmatch "$NB_DEPLOY_DIR/.env" >/dev/null 2>&1; then
        nb_warn "deploy/.env is tracked by git — it holds credentials, untrack it"
    fi
}

nb_r2() { aws --endpoint-url "$NB_R2_ENDPOINT" "$@"; }

# nb_r2_count <prefix> — objects under a prefix.
# This is also the first call that touches the bucket, so a failure here means
# bad credentials or a wrong endpoint, not an empty prefix. Distinguishing them
# matters: otherwise the immutability guard silently passes on a broken config.
nb_r2_count() {
    local prefix="$1" out
    if ! out="$(nb_r2 s3api list-objects-v2 --bucket "$R2_BUCKET" --prefix "$prefix" \
                --query 'length(Contents || `[]`)' --output text 2>&1)"; then
        nb_warn "$out"
        nb_die "cannot list s3://$R2_BUCKET — check R2_URL, R2_BUCKET and the API token in deploy/.env"
    fi
    printf '%s\n' "$out"
}

# nb_r2_guard <version> <prefix>...  — refuse to publish over an existing release.
#
# Overwriting a published version is the worst failure in this pipeline. Gradle
# caches by coordinate and SwiftPM pins the source-archive checksum into every
# consumer's Package.resolved, so a replaced artifact does not error — it
# silently disagrees with what consumers already trust. There is no yank: a bad
# release gets a patch release, never a takedown.
#
# Set NB_FORCE=1 to warn and continue instead.
nb_r2_guard() {
    local version="$1"; shift
    local prefix count occupied=0

    nb_step "checking $version is unpublished"
    for prefix in "$@"; do
        count="$(nb_r2_count "$prefix")"
        if [ "$count" -gt 0 ]; then
            nb_warn "$prefix — $count object(s) already there"
            occupied=$((occupied + 1))
        fi
    done

    if [ "$occupied" -gt 0 ]; then
        [ "${NB_FORCE:-0}" = "1" ] || \
            nb_die "$version is already published — bump versions.toml, or pass --force"
        nb_warn "--force: overwriting $version anyway"
    else
        nb_ok "$version is unpublished"
    fi
}

# nb_r2_put <file> <key> <content-type> [cache-control]
nb_r2_put() {
    local file="$1" key="$2" ctype="$3" cache="${4:-$NB_CACHE_IMMUTABLE}"
    [ -f "$file" ] || nb_die "nb_r2_put: no such file: $file"
    nb_r2 s3api put-object --bucket "$R2_BUCKET" --key "$key" --body "$file" \
        --content-type "$ctype" --cache-control "$cache" >/dev/null \
        || nb_die "upload failed: $key"
    nb_info "  ↑ $key"
}

# nb_http_check <url> <label> [expected-content-type]
# Verifies over the public domain. Sets NB_CHECK_FAILED to the running count.
NB_CHECK_FAILED=0
nb_http_check() {
    local url="$1" label="$2" want="${3:-}" head code ctype
    head="$(curl -sI --max-time 20 "$url" 2>/dev/null)"
    code="$(printf '%s' "$head" | awk 'NR==1{print $2}')"
    ctype="$(printf '%s' "$head" | awk -F': ' 'tolower($1)=="content-type"{print $2}' | tr -d '\r')"

    if [ "$code" != "200" ]; then
        nb_warn "${code:-no response}  $label — $url"
        NB_CHECK_FAILED=$((NB_CHECK_FAILED + 1))
        return
    fi
    if [ -n "$want" ]; then
        case "$ctype" in
            "$want"|"$want;"*) ;;
            *) nb_warn "200  $label but Content-Type is '$ctype', expected '$want'"
               NB_CHECK_FAILED=$((NB_CHECK_FAILED + 1)); return ;;
        esac
    fi
    nb_info "  200  $label${want:+  ($ctype)}"
}
