#!/usr/bin/env bash
# Publish every Android artifact into deploy/build/maven — a static Maven
# repository laid out exactly as it will sit on R2. Nothing leaves this machine.
#
#   deploy/build-android.sh
#
# Edit versions.toml and run this; the version is fanned out automatically.
#
# The five Gradle builds under platforms/android are independent and couple
# through mavenLocal(), so order is forced: compiler and the plugin first, then
# runtime, then foundation and devkit which resolve the earlier ones. Each
# module is published twice — once to mavenLocal so the next build can resolve
# it, once into the staging tree that gets uploaded.

source "$(dirname "${BASH_SOURCE[0]}")/lib/common.sh"

ANDROID_DIR="$NB_ROOT/platforms/android"
VERSION="$(nb_version android)"
STAGING="$NB_ROOT/deploy/build/maven"
GROUP_PATH="io/nativeblocks"

# gradle build dir : gradle project path
MODULES=(
    "compiler:"                 # publishes both :compiler and :gradle-plugin
    "runtime::core"
    "foundation::foundation"
    "devkit::devkit"
)

nb_require java

# versions.toml is the only file anyone edits by hand. Gradle reads the version
# from each package's VERSION file and from the consumer catalogs instead, so
# fan it out here rather than making it a separate command someone has to
# remember: a stale VERSION file publishes at the wrong coordinates and only
# surfaces as a confusing summary at the very end.
"$NB_DEPLOY_DIR/sync-versions.sh" || nb_die "could not apply versions.toml"

nb_step "publishing android $VERSION -> deploy/build/maven"

for entry in "${MODULES[@]}"; do
    build="${entry%%:*}"
    project="${entry#*:}"
    dir="$ANDROID_DIR/$build"

    [ -x "$dir/gradlew" ] || nb_die "no gradlew in $dir"

    nb_info ""
    nb_info "  ── $build ${project:+($project)}"

    # mavenLocal first: foundation and devkit resolve runtime/compiler from it.
    ( cd "$dir" && ./gradlew ${project:+$project:}publishToMavenLocal \
        --console=plain -q ) || nb_die "$build: publishToMavenLocal failed"

    ( cd "$dir" && ./gradlew ${project:+$project:}publishAllPublicationsToStagingRepository \
        -PnbStagingRepo="$STAGING" --console=plain -q ) \
        || nb_die "$build: staging publish failed"
done

# ---------------------------------------------------------------------------
# BOM
# ---------------------------------------------------------------------------
# A java-platform publication is a POM and nothing else, so it is emitted
# directly rather than standing up a sixth Gradle build to produce one file.

nb_step "generating bom"

BOM_DIR="$STAGING/$GROUP_PATH/bom-android/$VERSION"
BOM_POM="$BOM_DIR/bom-android-$VERSION.pom"
mkdir -p "$BOM_DIR"

{
    cat <<EOF
<?xml version="1.0" encoding="UTF-8"?>
<project xmlns="http://maven.apache.org/POM/4.0.0"
         xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
         xsi:schemaLocation="http://maven.apache.org/POM/4.0.0 https://maven.apache.org/xsd/maven-4.0.0.xsd">
  <modelVersion>4.0.0</modelVersion>
  <groupId>io.nativeblocks</groupId>
  <artifactId>bom-android</artifactId>
  <version>$VERSION</version>
  <packaging>pom</packaging>
  <name>bom-android</name>
  <description>Nativeblocks bill of materials for Android</description>
  <url>https://nativeblocks.io</url>
  <licenses>
    <license>
      <name>NATIVEBLOCKS TERMS OF SERVICE</name>
      <url>https://nativeblocks.io/terms-of-service</url>
    </license>
  </licenses>
  <developers>
    <developer>
      <name>Nativeblocks</name>
      <email>dev@nativeblocks.io</email>
    </developer>
  </developers>
  <dependencyManagement>
    <dependencies>
EOF
    for artifact in runtime-android foundation-android compiler-android devkit-android gradle-plugin-android; do
        cat <<EOF
      <dependency>
        <groupId>io.nativeblocks</groupId>
        <artifactId>$artifact</artifactId>
        <version>$VERSION</version>
      </dependency>
EOF
    done
    cat <<EOF
    </dependencies>
  </dependencyManagement>
</project>
EOF
} > "$BOM_POM"

# Maven resolvers fetch these sidecars alongside every artifact.
for algo in md5 sha1 sha256 sha512; do
    case "$algo" in
        md5) md5 -q "$BOM_POM" | tr -d '\n' > "$BOM_POM.$algo" ;;
        *)   shasum -a "${algo#sha}" "$BOM_POM" | awk '{printf "%s", $1}' > "$BOM_POM.$algo" ;;
    esac
done

nb_info "  $GROUP_PATH/bom-android/$VERSION/bom-android-$VERSION.pom"

# The BOM is generated rather than published by Gradle, so it misses the
# publishToMavenLocal pass the other modules get. Anything resolving through
# mavenLocal() — the sample app, most notably — needs it there too.
LOCAL_BOM="$HOME/.m2/repository/$GROUP_PATH/bom-android/$VERSION"
mkdir -p "$LOCAL_BOM"
cp "$BOM_POM" "$LOCAL_BOM/bom-android-$VERSION.pom"
nb_info "  ~/.m2/.../bom-android/$VERSION/bom-android-$VERSION.pom"

# ---------------------------------------------------------------------------
# Summary
# ---------------------------------------------------------------------------

nb_step "published artifacts"
missing=0
for artifact in runtime-android foundation-android compiler-android devkit-android gradle-plugin-android bom-android; do
    dir="$STAGING/$GROUP_PATH/$artifact/$VERSION"
    if [ -d "$dir" ]; then
        primary="$(find "$dir" -maxdepth 1 -name "$artifact-$VERSION.*" \
            ! -name '*.md5' ! -name '*.sha*' ! -name '*.module' | head -1)"
        nb_info "  io.nativeblocks:$artifact:$VERSION  ($(basename "${primary:-pom only}"))"
    else
        nb_warn "io.nativeblocks:$artifact:$VERSION — MISSING"
        missing=$((missing + 1))
    fi
done

# The marker's coordinates derive from the plugin id, so find it rather than
# hardcoding a path that silently goes stale when the id changes.
marker="$(find "$STAGING" -type d -name '*.gradle.plugin' -print -quit)"
if [ -n "$marker" ] && [ -d "$marker/$VERSION" ]; then
    nb_info "  $(basename "$marker" .gradle.plugin) (plugin marker)"
else
    nb_warn "plugin marker missing"
    missing=$((missing + 1))
fi

[ "$missing" -eq 0 ] || nb_die "$missing artifact(s) missing from the staging repo"

nb_ok "android $VERSION staged at deploy/build/maven ($(du -sh "$STAGING" | cut -f1))"
