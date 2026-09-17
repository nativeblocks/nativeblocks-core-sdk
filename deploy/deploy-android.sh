#!/usr/bin/env bash
# Upload the staged Android Maven repository to Cloudflare R2.
#
#   deploy/deploy-android.sh              # guard, upload, verify
#   deploy/deploy-android.sh --dry-run    # show what would change, upload nothing
#   deploy/deploy-android.sh --force      # overwrite an already-published version
#
# Reads deploy/.env for R2_URL, R2_ACCESS_KEY_ID and R2_SECRET_ACCESS_KEY.
#
# Run deploy/build-android.sh first. This only uploads what is already staged in
# deploy/build/maven — it never builds, so what you verified is what ships.
#
# Android needs no Worker: a Maven repository is static files fetched by exact
# path, and Gradle does not check Content-Type. Public read on the bucket is the
# whole requirement.

source "$(dirname "${BASH_SOURCE[0]}")/lib/common.sh"
source "$(dirname "${BASH_SOURCE[0]}")/lib/r2.sh"

# Configuration comes from deploy/.env — copy deploy/.env.example and fill it
# in. Real environment variables win over the file, so CI needs no edits.
nb_load_env

nb_require_env MAVEN_PUBLIC_URL

DRY_RUN=0
NB_FORCE=0
while [ $# -gt 0 ]; do
    case "$1" in
        --dry-run) DRY_RUN=1; shift ;;
        --force)   NB_FORCE=1; shift ;;
        *)         nb_die "unknown argument: $1" ;;
    esac
done

VERSION="$(nb_version android)"
STAGING="$NB_ROOT/deploy/build/maven"

nb_require aws curl

[ -d "$STAGING/io" ] || nb_die "nothing staged — run deploy/build-android.sh first"

nb_r2_init

nb_step "target"
nb_info "  bucket:   s3://$R2_BUCKET"
nb_info "  endpoint: $NB_R2_ENDPOINT"
nb_info "  version:  $VERSION"
nb_info "  files:    $(find "$STAGING" -type f | wc -l | tr -d ' ') ($(du -sh "$STAGING" | cut -f1))"

nb_r2_guard "$VERSION" \
    "io/nativeblocks/runtime-android/$VERSION/" \
    "io/nativeblocks/foundation-android/$VERSION/" \
    "io/nativeblocks/compiler-android/$VERSION/" \
    "io/nativeblocks/devkit-android/$VERSION/" \
    "io/nativeblocks/gradle-plugin-android/$VERSION/" \
    "io/nativeblocks/bom-android/$VERSION/"

# ---------------------------------------------------------------------------
# Upload
# ---------------------------------------------------------------------------
# Two passes, because everything under a version directory is immutable but
# maven-metadata.xml accumulates the version list and must stay fresh.

SYNC_ARGS=(--endpoint-url "$NB_R2_ENDPOINT")
[ "$DRY_RUN" = "1" ] && SYNC_ARGS+=(--dryrun)

nb_step "uploading artifacts"
aws "${SYNC_ARGS[@]}" s3 sync "$STAGING/" "s3://$R2_BUCKET/" \
    --exclude "*maven-metadata*" \
    --exclude "*.DS_Store" \
    --cache-control "$NB_CACHE_IMMUTABLE" \
    || nb_die "artifact upload failed"

nb_step "uploading maven-metadata"
aws "${SYNC_ARGS[@]}" s3 sync "$STAGING/" "s3://$R2_BUCKET/" \
    --exclude "*" --include "*maven-metadata*" \
    --cache-control "$NB_CACHE_INDEX" \
    || nb_die "metadata upload failed"

if [ "$DRY_RUN" = "1" ]; then
    nb_ok "dry run only — nothing uploaded"
    exit 0
fi

# ---------------------------------------------------------------------------
# Verify over the public domain
# ---------------------------------------------------------------------------
# The plugin marker is checked explicitly: it is the one object whose absence
# breaks the consumer's plugins {} block rather than a dependency.

nb_step "checking the public domain"
nb_http_check "$MAVEN_PUBLIC_URL/io/nativeblocks/runtime-android/$VERSION/runtime-android-$VERSION.aar" "runtime aar"
nb_http_check "$MAVEN_PUBLIC_URL/io/nativeblocks/foundation-android/$VERSION/foundation-android-$VERSION.aar" "foundation aar"
nb_http_check "$MAVEN_PUBLIC_URL/io/nativeblocks/devkit-android/$VERSION/devkit-android-$VERSION.aar" "devkit aar"
nb_http_check "$MAVEN_PUBLIC_URL/io/nativeblocks/compiler-android/$VERSION/compiler-android-$VERSION.jar" "compiler jar"
nb_http_check "$MAVEN_PUBLIC_URL/io/nativeblocks/bom-android/$VERSION/bom-android-$VERSION.pom" "bom pom"
nb_http_check "$MAVEN_PUBLIC_URL/io/nativeblocks/gradle-plugin-android/io.nativeblocks.gradle-plugin-android.gradle.plugin/$VERSION/io.nativeblocks.gradle-plugin-android.gradle.plugin-$VERSION.pom" "plugin marker"

[ "$NB_CHECK_FAILED" -eq 0 ] || \
    nb_die "$NB_CHECK_FAILED object(s) not reachable — check the bucket's public custom domain"

nb_ok "android $VERSION published"

cat <<EOF

── consumer setup ─────────────────────────────────────────────────────────────

settings.gradle.kts — scope the repository so only io.nativeblocks resolves
from it. Without the content filter every dependency in the build queries R2
first, which is slower and makes an outage yours instead of Maven Central's.
The regex matters: the plugin marker lives under the group
io.nativeblocks.gradle-plugin-android, not io.nativeblocks.

pluginManagement {
    repositories {
        maven {
            url = uri("$MAVEN_PUBLIC_URL")
            content { includeGroupByRegex("io\\\\.nativeblocks.*") }
        }
        gradlePluginPortal()
        google()
        mavenCentral()
    }
}

dependencyResolutionManagement {
    repositories {
        maven {
            url = uri("$MAVEN_PUBLIC_URL")
            content { includeGroupByRegex("io\\\\.nativeblocks.*") }
        }
        google()
        mavenCentral()
    }
}

app/build.gradle.kts:

plugins {
    id("com.google.devtools.ksp") version "2.2.21-2.0.5"
    id("io.nativeblocks.gradle-plugin-android") version "$VERSION"
}

dependencies {
    implementation(platform("io.nativeblocks:bom-android:$VERSION"))
    implementation("io.nativeblocks:runtime-android")
    implementation("io.nativeblocks:foundation-android")
    implementation("io.nativeblocks:compiler-android")
    debugImplementation("io.nativeblocks:devkit-android")

    // The BOM must be repeated here: ksp is its own configuration and does not
    // extend implementation, so without this line the version resolves empty
    // and the build fails with "Could not find io.nativeblocks:compiler-android:".
    ksp(platform("io.nativeblocks:bom-android:$VERSION"))
    ksp("io.nativeblocks:compiler-android")
}

───────────────────────────────────────────────────────────────────────────────
EOF
