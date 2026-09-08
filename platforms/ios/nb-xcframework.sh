#!/usr/bin/env bash
# Shared helpers for the four Nativeblocks host artifacts.
#
# Every build_xcframework.sh sources this so that name, bundle id, version,
# deployment target, symbol stripping and verification are decided in exactly
# one place. Source it, then
#   nb_set_version
#   ... package-specific build ...
#   nb_finalize <name> <bundle-id> <xcframework-path>
#   nb_verify   <name> <bundle-id> <xcframework-path>
#   nb_package  <name> <xcframework-path> <output-dir>

# All four iOS artifacts move in lockstep, versioned by [versions].ios in
# versions.toml at the repo root — the single place anyone edits. Everything
# below reads NB_VERSION, so call nb_set_version before nb_finalize / nb_verify.
NB_VERSION=""

# Resolved from this file's own location so the helpers work from any cwd and
# without the deploy/ tooling being present.
NB_IOS_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
NB_VERSIONS_TOML="$NB_IOS_DIR/../../versions.toml"

nb_set_version() {
  [ -f "$NB_VERSIONS_TOML" ] || { echo "❌ not found: $NB_VERSIONS_TOML"; return 1; }

  # Scoped to the [versions] table so a like-named key elsewhere cannot win.
  NB_VERSION="$(awk '
    /^[[:space:]]*\[/ { in_versions = ($0 ~ /^[[:space:]]*\[versions\][[:space:]]*$/); next }
    !in_versions { next }
    /^[[:space:]]*ios[[:space:]]*=/ {
      line = $0
      sub(/#.*/, "", line)
      sub(/^[[:space:]]*ios[[:space:]]*=[[:space:]]*/, "", line)
      gsub(/^"|"[[:space:]]*$/, "", line)
      gsub(/[[:space:]]+$/, "", line)
      print line
      exit
    }
  ' "$NB_VERSIONS_TOML")"

  [ -n "$NB_VERSION" ] || { echo "❌ no [versions].ios in $NB_VERSIONS_TOML"; return 1; }
}

# Must match the platforms declared in every Package.swift.
NB_IOS_MIN="15.0"
NB_MACOS_MIN="13.0"

# The slices every artifact ships. Kept identical across packages so a consumer
# never finds a slice in one framework that is missing from another.
NB_SLICES=("ios-arm64" "ios-arm64-simulator")

nb_require() {
  local missing=0 tool
  for tool in "$@"; do
    command -v "$tool" >/dev/null || { echo "❌ required tool not found: $tool"; missing=1; }
  done
  [ "$missing" = "0" ] || exit 1
}

# SwiftPM leaves the object of a deleted source file behind in .build, and
# xccache's `libtool -static` sweeps up whatever it finds — so a file you removed
# months ago still ships, still referencing APIs that no longer exist. That is
# how NativeSqliteStore.swift.o kept dragging the old FFI CacheProvider into the
# runtime. Wipe the compiled products before every build; dependency checkouts
# stay so resolving is still cheap.
nb_clean_build() {
  local dir="$1"
  [ -d "$dir/.build" ] || return 0
  find "$dir/.build" -mindepth 1 -maxdepth 1 \
    ! -name checkouts ! -name repositories ! -name artifacts ! -name registry \
    ! -name workspace-state.json \
    -exec rm -rf {} + 2>/dev/null || true
}

# Belt and braces for the above: every object in the framework must trace back to
# a source file that exists right now.
#   nb_check_sources <module-source-dir> <framework-binary>
nb_check_sources() {
  local src_dir="$1" binary="$2"
  local members sources stale

  members=$(mktemp); sources=$(mktemp)
  ar t "$binary" 2>/dev/null | grep '\.o$' | sed 's/\.swift\.o$//; s/\.o$//' | sort -u > "$members"
  find "$src_dir" -name "*.swift" -exec basename {} .swift \; 2>/dev/null | sort -u > "$sources"
  # resource_bundle_accessor is synthesised by SwiftPM, never a file on disk.
  stale=$(comm -23 "$members" "$sources" | grep -v '^resource_bundle_accessor$' || true)
  rm -f "$members" "$sources"

  if [ -n "$stale" ]; then
    echo "   ❌ $(basename "$binary"): objects with no source file — stale .build:"
    echo "$stale" | sed 's/^/      /'
    return 1
  fi
}

nb_build() {
  # nb_build <output-dir> <target> [target...]
  local out="$1"; shift
  echo "⚙️  Building $* (release, library evolution)..."
  xccache pkg build "$@" \
    --sdk=iphoneos,iphonesimulator \
    --config=release \
    --library-evolution \
    --no-merge-slices \
    --out="$out"
}

# --- Info.plist -------------------------------------------------------------
# xccache writes an XFWK-typed plist with an AvailableLibraries array into each
# .framework, which is the *xcframework* shape and carries no version at all.
# Each slice gets a real framework plist written from scratch instead.
nb_write_plist() {
  local name="$1" bundle_id="$2" slice="$3" plist="$4"
  local supported min_key min_value

  case "$slice" in
    ios-arm64-simulator) supported="iPhoneSimulator"; min_key="MinimumOSVersion";     min_value="$NB_IOS_MIN" ;;
    ios-*)               supported="iPhoneOS";        min_key="MinimumOSVersion";     min_value="$NB_IOS_MIN" ;;
    macos-*)             supported="MacOSX";          min_key="LSMinimumSystemVersion"; min_value="$NB_MACOS_MIN" ;;
    *) echo "❌ unknown slice id: $slice"; return 1 ;;
  esac

  cat > "$plist" <<EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
	<key>CFBundleDevelopmentRegion</key>
	<string>en</string>
	<key>CFBundleExecutable</key>
	<string>$name</string>
	<key>CFBundleIdentifier</key>
	<string>$bundle_id</string>
	<key>CFBundleInfoDictionaryVersion</key>
	<string>6.0</string>
	<key>CFBundleName</key>
	<string>$name</string>
	<key>CFBundlePackageType</key>
	<string>FMWK</string>
	<key>CFBundleShortVersionString</key>
	<string>$NB_VERSION</string>
	<key>CFBundleVersion</key>
	<string>$NB_VERSION</string>
	<key>CFBundleSupportedPlatforms</key>
	<array>
		<string>$supported</string>
	</array>
	<key>$min_key</key>
	<string>$min_value</string>
</dict>
</plist>
EOF
  plutil -lint "$plist" >/dev/null
}

# --- Finalize ---------------------------------------------------------------
# Stamp identity onto every slice and drop debug info.
#
# `-S` (debug symbols) is the ONLY safe strip for a static archive, and is what
# Xcode's own "Strip Style: Debugging Symbols" does for static libraries. Do not
# add `-x`: it deletes the local `l_got.*` labels that relocations point at, and
# the consumer's link then fails with "Undefined symbol: protocol descriptor for
# NativeblocksRuntimeFFI.CacheProvider" and friends. Nothing in the symbol table
# shows that damage — only an actual link does, which is why nb_link_check runs.
nb_finalize() {
  local name="$1" bundle_id="$2" xcf="$3" slice fw binary before after

  echo "🔖 Stamping $name $NB_VERSION and stripping debug info..."
  for slice in "$xcf"/*/; do
    slice="$(basename "$slice")"
    fw="$xcf/$slice/$name.framework"
    [ -d "$fw" ] || { echo "❌ missing slice framework: $fw"; return 1; }

    nb_write_plist "$name" "$bundle_id" "$slice" "$fw/Info.plist"

    binary="$fw/$name"
    before=$(du -k "$binary" | cut -f1)
    strip -S "$binary"
    after=$(du -k "$binary" | cut -f1)

    # `package` and `private` interfaces serve same-package and @_spi clients.
    # Neither applies to a binary consumer.
    rm -f "$fw/Modules/$name.swiftmodule"/*.package.swiftinterface
    rm -f "$fw/Modules/$name.swiftmodule"/*.private.swiftinterface

    echo "   ✅ $slice  $((before / 1024))M → $((after / 1024))M"
  done
}

# --- Verify -----------------------------------------------------------------
nb_verify() {
  local name="$1" bundle_id="$2" xcf="$3" slice fw plist debug interfaces value

  echo "🔍 Verifying $name..."
  for slice in "${NB_SLICES[@]}"; do
    fw="$xcf/$slice/$name.framework"
    [ -d "$fw" ] || { echo "   ❌ $slice: slice missing from the xcframework"; return 1; }
    plist="$fw/Info.plist"

    nb_expect "$plist" CFBundlePackageType        FMWK          "$slice" || return 1
    nb_expect "$plist" CFBundleExecutable         "$name"       "$slice" || return 1
    nb_expect "$plist" CFBundleName               "$name"       "$slice" || return 1
    nb_expect "$plist" CFBundleIdentifier         "$bundle_id"  "$slice" || return 1
    nb_expect "$plist" CFBundleShortVersionString "$NB_VERSION" "$slice" || return 1
    nb_expect "$plist" CFBundleVersion            "$NB_VERSION" "$slice" || return 1

    # A single __debug section anywhere in the archive means strip did not run.
    debug=$(otool -l "$fw/$name" 2>/dev/null | grep -c '__debug' || true)
    if [ "$debug" != "0" ]; then
      echo "   ❌ $slice: $debug __debug sections still present"
      return 1
    fi

    # Without a .swiftinterface the framework only compiles against the exact
    # Swift version that built it.
    interfaces=$(find "$fw/Modules" -name "*.swiftinterface" 2>/dev/null | wc -l | tr -d ' ')
    if [ "$interfaces" = "0" ]; then
      echo "   ❌ $slice: no .swiftinterface — library evolution did not take"
      return 1
    fi

    echo "   ✅ $slice: $name $NB_VERSION, $interfaces interface(s), no debug info"
  done
}

nb_expect() {
  local plist="$1" key="$2" want="$3" slice="$4" got
  got=$(/usr/libexec/PlistBuddy -c "Print :$key" "$plist" 2>/dev/null || echo "<missing>")
  if [ "$got" != "$want" ]; then
    echo "   ❌ $slice: $key is '$got', expected '$want'"
    return 1
  fi
}

# --- Link check -------------------------------------------------------------
# The only honest test of a stripped static archive. Symbol-table inspection
# cannot see broken relocations, so this force-loads every object in every
# framework and asks the real linker to resolve them, exactly as the consumer's
# app will. Everything is linked together because foundation and devkit resolve
# their runtime symbols from NativeblocksRuntime.
#
#   nb_link_check <xcframeworks-dir> <slice> <module> [module...]
nb_link_check() {
  local xcf_dir="$1" slice="$2"; shift 2
  local modules=("$@")
  local sdk target module args=() work

  case "$slice" in
    ios-arm64-simulator) sdk="iphonesimulator"; target="arm64-apple-ios$NB_IOS_MIN-simulator" ;;
    ios-arm64)           sdk="iphoneos";        target="arm64-apple-ios$NB_IOS_MIN" ;;
    *) echo "   ⚠️  no link check for slice $slice"; return 0 ;;
  esac

  # Deliberately no `import`: this is a LINK test. Importing would compile the
  # .swiftinterface too, and NativeblocksFoundation's interface says
  # `import NativeblocksCompiler`, which lives in the source package and is not
  # built here. force_load already drags in every object, which is what makes
  # the linker resolve every reference — the thing we actually need proved.
  work=$(mktemp -d)
  : > "$work/main.swift"
  for module in "${modules[@]}"; do
    args+=(-Xlinker -force_load -Xlinker "$xcf_dir/$module.xcframework/$slice/$module.framework/$module")
  done

  if xcrun --sdk "$sdk" swiftc -target "$target" "${args[@]}" \
       -lsqlite3 "$work/main.swift" -o "$work/probe" 2>"$work/err"; then
    echo "   ✅ $slice: links clean (${#modules[@]} frameworks, all objects forced)"
    rm -rf "$work"
  else
    echo "   ❌ $slice: the shipped frameworks do not link."
    grep -E '^  "|^ld: ' "$work/err" 2>/dev/null | head -12 | sed 's/^/      /' || true
    echo "      full log: $work/err"
    return 1
  fi
}

# --- Package ----------------------------------------------------------------
nb_package() {
  local name="$1" xcf="$2" out="$3"
  local zip_name="$name.xcframework.zip"
  echo "🗜️  Packaging $zip_name..."
  (cd "$out" && rm -f "$zip_name" && zip -r -q "$zip_name" "$name.xcframework")
  NB_CHECKSUM=$(swift package compute-checksum "$out/$zip_name")
  NB_ZIP="$out/$zip_name"

  echo ""
  echo "✅ $name $NB_VERSION"
  echo "   📦 $NB_ZIP ($(du -h "$NB_ZIP" | cut -f1))"
  echo "   🔐 $NB_CHECKSUM"
  echo "   The framework is static — consumers set it to \"Do Not Embed\"."
  echo ""
}
