#!/usr/bin/env bash
# Seal the generated FFI bindings so the host SDK cannot re-export them.
#
# UniFFI emits everything `public`: every converter, handle, record and callback
# interface. Copied as-is into a host SDK, all of it becomes host public API and
# consumers could bind straight to the Rust ABI. This rewrites the generated
# sources to the narrowest visibility that still lets the hand-written wrapper
# use them:
#
#   Kotlin  every top-level declaration -> `internal`  (visible in the module)
#   Swift   `public`/`open`             -> `package`   (visible in the package)
#
# The payoff is not just hiding: a wrapper that returns or accepts a generated
# type from a public function then FAILS TO COMPILE ("public function exposes its
# internal return type"), so a leak cannot reach a release.
#
# Layout each rewrite assumes (see README.md):
#   Kotlin  generated + wrapper in the SAME Gradle module — `internal` is
#           module-scoped, so a separate :runtime-ffi module would lock the
#           wrapper out too.
#   Swift   generated in its own NativeblocksRuntimeFFI target, wrapper in
#           NativeblocksRuntime, both in ONE SwiftPM package — `package`
#           visibility spans targets of a package but stops at its edge.
#
# The C-shim import is downgraded to `package import` but no further: the
# generated package-level declarations take C types (RustBuffer et al) in their
# signatures, so `internal import` is rejected by the compiler.
#
# Idempotent — safe to re-run. Called by generate-bindings.sh.
#
# Usage:  ./scripts/seal-bindings.sh [bindings-dir]   (default: bindings)
set -euo pipefail

cd "$(dirname "$0")/.."

BINDINGS="${1:-bindings}"

# Keywords that can open a top-level Kotlin declaration, plus the non-visibility
# modifiers that may precede them.
KT_DECL='class|interface|object|fun|val|var|typealias'
KT_MODS='open |abstract |sealed |data |value |annotation |enum '

fail_if_leaked() { # $1=file  $2=perl-condition  $3=label
  local leaked
  leaked="$(perl -ne "print \"    line \$.: \$_\" if $2" "$1")"
  if [[ -n "$leaked" ]]; then
    echo "ERROR: $3 survived sealing in $1:" >&2
    printf '%s' "$leaked" >&2
    exit 1
  fi
}

seal_kotlin() {
  local f="$1"
  # 1. explicit `public` -> `internal`
  # 2. modifier-less top-level declarations default to public -> prefix `internal`.
  #    Lines that already carry a visibility modifier are skipped, which is what
  #    makes the pass idempotent.
  perl -i -pe "
    s/^public /internal /;
    s/^((?:${KT_MODS})*)(${KT_DECL})\b/internal \$1\$2/ unless /^(?:internal|private|public)\b/;
  " "$f"
  fail_if_leaked "$f" "/^(?!internal |private )(?:${KT_MODS})*(?:${KT_DECL})\b/" "public declarations"
}

seal_swift() {
  local f="$1"
  # `open` cannot coexist with narrowed access, so it is replaced rather than
  # downgraded; nothing outside the package needs to subclass generated types.
  # The compiler(>=6.0) guard keeps Xcode 15 working, where access levels on
  # imports (SE-0409) do not exist yet but `package` declarations already do.
  perl -i -pe '
    s/^(\s*)public\b/$1package/;
    s/^(\s*)open\b/$1package/;
  ' "$f"

  # Guarded separately: the #else branch it emits is itself a plain `import`,
  # which would match again and nest on every re-run.
  if ! grep -q '^package import ' "$f"; then
    perl -i -pe '
      if (/^import (\w+CFFI)$/) {
        $_ = "#if compiler(>=6.0)\npackage import $1\n#else\nimport $1\n#endif\n";
      }
    ' "$f"
  fi
  fail_if_leaked "$f" '/^\s*(public|open)\b/' "public declarations"
}

sealed=0
while IFS= read -r f; do
  echo "    sealing $f (internal)"
  seal_kotlin "$f"
  sealed=$((sealed + 1))
done < <(find "$BINDINGS/kotlin" -name '*.kt' 2>/dev/null)

while IFS= read -r f; do
  echo "    sealing $f (package)"
  seal_swift "$f"
  sealed=$((sealed + 1))
done < <(find "$BINDINGS/swift" -name '*.swift' 2>/dev/null)

if [[ "$sealed" -eq 0 ]]; then
  echo "ERROR: no Kotlin/Swift bindings under $BINDINGS/ — run generate-bindings.sh first" >&2
  exit 1
fi

echo "    sealed $sealed file(s)"
