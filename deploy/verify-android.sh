#!/usr/bin/env bash
# Build a throwaway app against nothing but deploy/build/maven, the way a
# customer would. Everything here exists to make the test honest:
#
#   * a scratch GRADLE_USER_HOME, so a stale ~/.gradle cache cannot answer
#   * no mavenLocal(), so the artifacts we just published locally cannot mask a
#     broken staging repo — this is the failure this script exists to catch
#   * the exact repository setup we hand consumers, content filter included, so
#     this proves the published snippet rather than some easier arrangement
#
#   deploy/verify-android.sh
#   deploy/verify-android.sh --keep    # leave the scratch project for inspection

source "$(dirname "${BASH_SOURCE[0]}")/lib/common.sh"

KEEP=0
[ "${1:-}" = "--keep" ] && KEEP=1

VERSION="$(nb_version android)"
STAGING="$NB_ROOT/deploy/build/maven"
SDK_DIR="${ANDROID_HOME:-${ANDROID_SDK_ROOT:-$HOME/Library/Android/sdk}}"

[ -d "$STAGING" ]  || nb_die "no staging repo — run deploy/build-android.sh first"
[ -d "$SDK_DIR" ]  || nb_die "Android SDK not found at $SDK_DIR (set ANDROID_HOME)"

WORK="$(mktemp -d)/verify-android"
cleanup() { [ "$KEEP" = "1" ] || rm -rf "$(dirname "$WORK")"; }
trap cleanup EXIT

mkdir -p "$WORK/app/src/main/java/io/nativeblocks/verify"
mkdir -p "$WORK/app/src/main/res/values"

nb_step "scratch project for android $VERSION"
nb_info "  repo: $STAGING"
nb_info "  work: $WORK"

# --- project scaffolding ----------------------------------------------------

cat > "$WORK/settings.gradle.kts" <<EOF
pluginManagement {
    repositories {
        maven {
            url = uri("file://$STAGING")
            content { includeGroupByRegex("io\\\\.nativeblocks.*") }
        }
        gradlePluginPortal()
        google()
        mavenCentral()
    }
}
dependencyResolutionManagement {
    repositoriesMode.set(RepositoriesMode.FAIL_ON_PROJECT_REPOS)
    repositories {
        maven {
            url = uri("file://$STAGING")
            content { includeGroupByRegex("io\\\\.nativeblocks.*") }
        }
        google()
        mavenCentral()
    }
}
rootProject.name = "verify"
include(":app")
EOF

cat > "$WORK/local.properties" <<EOF
sdk.dir=$SDK_DIR
EOF

cat > "$WORK/gradle.properties" <<'EOF'
org.gradle.jvmargs=-Xmx3072m -Dfile.encoding=UTF-8
android.useAndroidX=true
android.nonTransitiveRClass=true
EOF

cat > "$WORK/build.gradle.kts" <<'EOF'
plugins {
    id("com.android.application") version "8.13.2" apply false
    id("org.jetbrains.kotlin.android") version "2.2.21" apply false
    id("com.google.devtools.ksp") version "2.2.21-2.0.5" apply false
}
EOF

cat > "$WORK/app/build.gradle.kts" <<EOF
plugins {
    id("com.android.application")
    id("org.jetbrains.kotlin.android")
    id("com.google.devtools.ksp")
    id("io.nativeblocks.gradle-plugin-android") version "$VERSION"
}

android {
    namespace = "io.nativeblocks.verify"
    compileSdk = 35
    defaultConfig {
        applicationId = "io.nativeblocks.verify"
        minSdk = 26
        targetSdk = 35
    }
    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }
}

kotlin { compilerOptions { jvmTarget.set(org.jetbrains.kotlin.gradle.dsl.JvmTarget.JVM_17) } }

dependencies {
    implementation(platform("io.nativeblocks:bom-android:$VERSION"))
    implementation("io.nativeblocks:runtime-android")
    implementation("io.nativeblocks:foundation-android")
    implementation("io.nativeblocks:compiler-android")
    debugImplementation("io.nativeblocks:devkit-android")

    // ksp does not extend implementation, so the platform has to be added to it
    // separately or the version never resolves.
    ksp(platform("io.nativeblocks:bom-android:$VERSION"))
    ksp("io.nativeblocks:compiler-android")
}
EOF

cat > "$WORK/app/src/main/AndroidManifest.xml" <<'EOF'
<?xml version="1.0" encoding="utf-8"?>
<manifest xmlns:android="http://schemas.android.com/apk/res/android">
    <application android:label="verify" />
</manifest>
EOF

cat > "$WORK/app/src/main/java/io/nativeblocks/verify/Smoke.kt" <<'EOF'
package io.nativeblocks.verify

import io.nativeblocks.compiler.type.Action
import io.nativeblocks.compiler.type.ActionFunction
import io.nativeblocks.compiler.type.ActionParameter
import io.nativeblocks.runtime.api.util.SDKConfig

@Action(keyType = "VERIFY_SMOKE", name = "Smoke", description = "release verification action")
class SmokeAction {

    @ActionParameter
    data class Param(val value: String = "")

    @ActionFunction
    fun run(param: Param) {
        check(SDKConfig.SDK_VERSION.isNotEmpty() && param.value.isEmpty())
    }
}
EOF

# --- build ------------------------------------------------------------------
# A scratch GRADLE_USER_HOME is the point: it guarantees nothing is answered
# from a warm cache, so the staging repo has to stand on its own.

# Separate from ~/.gradle so the user's cache cannot answer, but persistent
# across runs so the Gradle distribution is not re-downloaded every time.
export GRADLE_USER_HOME="$NB_ROOT/deploy/build/.gradle-verify"
WRAPPER="$NB_ROOT/platforms/android/sample/gradlew"
[ -x "$WRAPPER" ] || nb_die "no gradle wrapper at $WRAPPER"
cp -R "$NB_ROOT/platforms/android/sample/gradle" "$WORK/"
cp "$WRAPPER" "$WORK/"

# Realizing the whole task graph is what an IDE sync does, and it is a stricter
# gate than assembling: a task whose configuration reads a file eagerly builds
# fine but breaks Android Studio for anyone without that file. The scratch
# project deliberately has no nativeblocks.json.
nb_step "realizing the task graph (stands in for an IDE sync)"
( cd "$WORK" && ./gradlew :app:tasks --all --no-daemon --console=plain -q >/dev/null ) \
    || nb_die "task realization failed — the plugin reads config too eagerly"
nb_ok "task graph realizes without a nativeblocks.json"

nb_step "assembling debug + release"
( cd "$WORK" && ./gradlew assembleDebug assembleRelease \
    --no-daemon --console=plain --refresh-dependencies ) \
    || nb_die "the scratch app failed to build against the staging repo"

# --- assertions -------------------------------------------------------------

nb_step "checking ksp output"

# The processor is the half a plain dependency check cannot prove. If it did not
# run, the app still builds and the SDK is quietly useless.
generated="$(find "$WORK/app/build/generated/ksp" -name '*.kt' 2>/dev/null | head -5)"
[ -n "$generated" ] || nb_die "ksp generated nothing — the processor did not run"
find "$WORK/app/build/generated/ksp" -name '*.kt' | sed "s|$WORK/app/build/generated/ksp/||;s|^|  |"

grep -rqs "VERIFY_SMOKE" "$WORK/app/build/generated/ksp" \
    || nb_die "ksp ran but did not pick up the @Action annotation"
nb_ok "ksp generated provider sources"

nb_step "checking the built apk"

APK="$(find "$WORK/app/build/outputs/apk/release" -name '*.apk' | head -1)"
[ -n "$APK" ] || nb_die "no release apk produced"

abis="$(unzip -l "$APK" | grep -o 'lib/[a-z0-9_-]*' | sort -u | sed 's|lib/||')"
nb_info "  ABIs: $(echo "$abis" | tr '\n' ' ')"

for expected in arm64-v8a armeabi-v7a x86_64; do
    echo "$abis" | grep -qx "$expected" || nb_die "missing ABI in apk: $expected"
done

# The 16 KB page-size requirement applies to 64-bit ABIs only; 32-bit stays 4 KB.
unzip -o -q "$APK" 'lib/*/libnativeblocks_runtime.so' -d "$WORK/abi"
python3 - "$WORK/abi" <<'PY'
import struct, sys, pathlib
bad = []
for so in sorted(pathlib.Path(sys.argv[1]).rglob("libnativeblocks_runtime.so")):
    d = so.read_bytes()
    is64 = d[4] == 2
    e = '<' if d[5] == 1 else '>'
    if is64:
        phoff, = struct.unpack_from(e + 'Q', d, 0x20)
        phes,  = struct.unpack_from(e + 'H', d, 0x36)
        phn,   = struct.unpack_from(e + 'H', d, 0x38)
    else:
        phoff, = struct.unpack_from(e + 'I', d, 0x1C)
        phes,  = struct.unpack_from(e + 'H', d, 0x2A)
        phn,   = struct.unpack_from(e + 'H', d, 0x2C)
    aligns = []
    for i in range(phn):
        o = phoff + i * phes
        t, = struct.unpack_from(e + 'I', d, o)
        if t == 1:
            a, = struct.unpack_from(e + 'Q', d, o + 48) if is64 else struct.unpack_from(e + 'I', d, o + 28)
            aligns.append(a)
    align = max(aligns) if aligns else 0
    abi = so.parent.name
    if is64 and align < 16384:
        bad.append(f"{abi} align=0x{align:x}")
    print(f"  {abi}: LOAD align 0x{align:x} ({'64-bit' if is64 else '32-bit'})")
if bad:
    sys.exit("16 KB alignment failure: " + ", ".join(bad))
PY

nb_ok "android $VERSION verified against the staging repo only"
