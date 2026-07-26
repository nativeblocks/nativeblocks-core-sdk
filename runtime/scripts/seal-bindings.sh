#!/usr/bin/env bash
# Seal the generated FFI bindings so the host SDK cannot re-export them.
#
# UniFFI emits everything `public`: every converter, handle, record and callback
# interface. Copied as-is into a host SDK, all of it becomes host public API and
# consumers could bind straight to the Rust ABI. This narrows the generated
# sources to the tightest visibility that still lets the hand-written wrapper
# use them:
#
#   Kotlin  every top-level declaration -> `internal`  (visible in the module)
#   Swift   `public`/`open`             -> `package`   (visible in the package)
#
# The payoff is not just hiding: a wrapper that returns or accepts a generated
# type from a public function then FAILS TO COMPILE ("public function exposes its
# internal return type"), so a leak cannot reach a release.
#
# Sealing PARTIALLY is worse than not sealing at all: leftovers do not widen the
# host API, they break its build, because a declaration left public ends up
# referring to types that are now internal. Two ways a line-anchored regex misses
# a declaration, both of which cost a real build:
#
#   1. UniFFI indents some top-level declarations (` fun provideLogger(...)`), so
#      an `^fun` anchor skips them -> "'public' function exposes its 'internal'
#      parameter type 'Logger'".
#   2. A modifier the rewrite does not know sits in front of the keyword
#      (`inline fun ... T.use`) -> "Public-API inline function cannot access
#      non-public-API function".
#
# Hence: scope comes from brace/paren depth rather than indentation (the generated
# code puts object members at column 0 and `data class` constructor properties at
# brace depth 0, which is what the paren counter excludes), the Kotlin modifier
# set is carried in full, and every file is re-read in check mode with a
# deliberately BROADER rule than the rewrite uses. A declaration behind an unknown
# modifier therefore fails HERE, instead of turning into a compile error in the
# host SDK that has to be patched by hand in generated code.
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

# The rewriter, read into a variable rather than embedded in $(...): bash 3.2
# rescans a command substitution for quotes and backticks even inside a heredoc,
# and this program contains both. `read` returns non-zero at EOF, hence `|| true`.
IFS= read -r -d '' SEALER <<'PERL_SEALER' || true
use strict;
use warnings;

# SEAL_LANG=kotlin|swift  SEAL_MODE=seal|check  argv[0]=file
my $lang = $ENV{SEAL_LANG} || '';
my $mode = $ENV{SEAL_MODE} || 'seal';
my $path = $ARGV[0] || '';
die "seal: need SEAL_LANG=kotlin|swift SEAL_MODE=seal|check and a file\n"
    unless $lang =~ /\A(?:kotlin|swift)\z/ && $mode =~ /\A(?:seal|check)\z/ && length $path;

open my $fh, '<', $path or die "seal: cannot read $path: $!\n";
my @lines = <$fh>;
close $fh;

my @leaks;
my @out = $lang eq 'kotlin'
    ? seal_kotlin(\@lines, $mode, \@leaks)
    : seal_swift(\@lines, $mode, \@leaks);

if ($mode eq 'seal') {
    open my $w, '>', $path or die "seal: cannot write $path: $!\n";
    print {$w} @out;
    close $w or die "seal: cannot close $path: $!\n";
}
elsif (@leaks) {
    print STDERR "ERROR: $lang bindings not fully sealed - $path\n";
    print STDERR @leaks;
    exit 1;
}
exit 0;

# Remove comments, string literals and (Kotlin) backticked identifiers, so brace
# and paren counting cannot be thrown off by a brace inside a string template,
# and so the word "public" in prose is never rewritten or reported. $st carries
# the comment / raw-string state across lines.
sub strip {
    my ($s, $st, $kt) = @_;
    my ($out, $i, $n) = ('', 0, length $s);
    while ($i < $n) {
        my $two = substr $s, $i, 2;
        if ($st->{block}) {
            if ($two eq '*/') { $st->{block} = 0; $i += 2 } else { $i++ }
            next;
        }
        if ($kt && $st->{raw}) {
            if (substr($s, $i, 3) eq '"""') { $st->{raw} = 0; $i += 3 } else { $i++ }
            next;
        }
        if ($two eq '/*') { $st->{block} = 1; $i += 2; next }
        if ($two eq '//') { last }
        if ($kt && substr($s, $i, 3) eq '"""') { $st->{raw} = 1; $i += 3; next }

        my $c = substr $s, $i, 1;
        my $quote = ($c eq '"') || ($kt && ($c eq "'" || $c eq '`'));
        if ($quote) {
            $i++;
            while ($i < $n) {
                my $d = substr $s, $i, 1;
                if ($d eq "\\") { $i += 2; next }
                if ($d eq $c)   { $i++; last }
                $i++;
            }
            next;
        }
        $out .= $c;
        $i++;
    }
    return $out;
}

# Kotlin: every top-level declaration -> `internal`, so the hand-written wrapper
# in the same Gradle module can still use it and nothing outside can.
sub seal_kotlin {
    my ($lines, $mode, $leaks) = @_;

    my $ANN  = qr/\@[\w.:]+(?:\([^)]*\))?/;
    my $MOD  = qr/open|final|abstract|sealed|data|value|annotation|enum|inner
                 |companion|inline|noinline|crossinline|external|const|lateinit
                 |suspend|operator|infix|tailrec|expect|actual|override|vararg
                 |reified/x;
    my $DECL = qr/class|interface|object|fun|val|var|typealias/;
    my $VIS  = qr/public|private|internal|protected/;

    my $st = { block => 0, raw => 0 };
    my ($brace, $paren) = (0, 0);
    my @out;

    for my $n (1 .. scalar @$lines) {
        my $line = $lines->[$n - 1];

        # Depth as of the START of the line is what decides top level.
        my ($b, $p, $in_comment) = ($brace, $paren, $st->{block} || $st->{raw});
        my $code = strip($line, $st, 1);
        $brace += ($code =~ tr/{//) - ($code =~ tr/}//);
        $paren += ($code =~ tr/(//) - ($code =~ tr/)//);

        if (!$in_comment && $b == 0 && $p == 0) {
            my $has_vis = $line =~ /\A\s*(?:(?:$ANN|$MOD)\s+)*(?:$VIS)\b/;

            if ($line =~ /\A\s*(?:(?:$ANN|$MOD)\s+)*public\b/) {
                if ($mode eq 'seal') {
                    $line =~ s/\A(\s*(?:(?:$ANN|$MOD)\s+)*)public\b/${1}internal/;
                }
                else { push @$leaks, "    line $n: $line" }
            }
            elsif (!$has_vis
                && $line =~ /\A\s*(?:$ANN\s+)*(?:(?:$MOD)\s+)*(?:$DECL)\b/) {
                # No modifier at all - Kotlin defaults to public. `internal` goes
                # after any annotations, before the remaining modifiers.
                if ($mode eq 'seal') {
                    $line =~ s/\A(\s*(?:$ANN\s+)*)((?:(?:$MOD)\s+)*(?:$DECL)\b)/${1}internal $2/;
                }
                else { push @$leaks, "    line $n: $line" }
            }
            elsif (!$has_vis && $mode eq 'check' && $code =~ /\b(?:$DECL)\b/) {
                # Deliberately broader than the rewrite: ANY top-level line with a
                # declaration keyword and no visibility modifier is a leak, however
                # it is spelled. A modifier the rewrite does not know
                # (`context(String) fun ...`) is skipped above and would otherwise
                # ship public - this is the net that catches it.
                push @$leaks, "    line $n: $line";
            }
        }
        push @out, $line;
    }
    return @out;
}

# Swift: `public`/`open` -> `package`, which spans the targets of one SwiftPM
# package and stops at its edge. Declarations with no modifier already default to
# internal, which is narrower, so they are left alone.
sub seal_swift {
    my ($lines, $mode, $leaks) = @_;

    my $ATTR = qr/\@[\w.]+(?:\([^)]*\))?/;
    my $MOD  = qr/final|required|static|class|override|convenience|dynamic
                 |indirect|mutating|nonmutating|lazy|weak|unowned|nonisolated
                 |distributed|borrowing|consuming|prefix|postfix|infix|optional
                 |unsafe/x;

    # The C-shim import is downgraded once. Its #else branch is itself a plain
    # `import`, which would match again and nest on every re-run.
    my $shim_done = grep { /\Apackage import / } @$lines;

    my $st = { block => 0 };
    my @out;

    for my $n (1 .. scalar @$lines) {
        my $line = $lines->[$n - 1];
        my $in_comment = $st->{block};
        my $code = strip($line, $st, 0);

        if (!$in_comment) {
            if ($mode eq 'seal') {
                # `open` is replaced rather than narrowed: it cannot coexist with a
                # reduced access level, and nothing outside the package needs to
                # subclass a generated type.
                $line =~ s/\A(\s*(?:(?:$ATTR|$MOD)\s+)*)(?:public|open)\b/${1}package/;

                if (!$shim_done && $line =~ /\Aimport (\w+CFFI)\s*\z/) {
                    # Access levels on imports are Swift 6; the guard keeps Xcode 15
                    # building, where `package` declarations already work.
                    $line = "#if compiler(>=6.0)\npackage import $1\n#else\nimport $1\n#endif\n";
                    $shim_done = 1;
                }
            }
            elsif ($code =~ /\b(?:public|open)\b/) {
                # Any surviving token in real code, not only one in modifier
                # position - catches `required public init` and friends.
                push @$leaks, "    line $n: $line";
            }
        }
        push @out, $line;
    }
    return @out;
}
PERL_SEALER

seal() { # $1=kotlin|swift  $2=file
  SEAL_LANG="$1" SEAL_MODE=seal  perl -e "$SEALER" "$2"
  SEAL_LANG="$1" SEAL_MODE=check perl -e "$SEALER" "$2"
}

sealed=0
while IFS= read -r f; do
  echo "    sealing $f (internal)"
  seal kotlin "$f"
  sealed=$((sealed + 1))
done < <(find "$BINDINGS/kotlin" -name '*.kt' 2>/dev/null)

while IFS= read -r f; do
  echo "    sealing $f (package)"
  seal swift "$f"
  sealed=$((sealed + 1))
done < <(find "$BINDINGS/swift" -name '*.swift' 2>/dev/null)

if [[ "$sealed" -eq 0 ]]; then
  echo "ERROR: no Kotlin/Swift bindings under $BINDINGS/ — run generate-bindings.sh first" >&2
  exit 1
fi

echo "    sealed $sealed file(s)"
