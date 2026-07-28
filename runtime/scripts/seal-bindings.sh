#!/usr/bin/env bash
# Narrow the generated FFI bindings to Kotlin `internal` / Swift `package` so they cannot become host public API. Usage: ./scripts/seal-bindings.sh [bindings-dir]
set -euo pipefail

cd "$(dirname "$0")/.."

BINDINGS="${1:-bindings}"

IFS= read -r -d '' SEALER <<'PERL_SEALER' || true
use strict;
use warnings;

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
                if ($mode eq 'seal') {
                    $line =~ s/\A(\s*(?:$ANN\s+)*)((?:(?:$MOD)\s+)*(?:$DECL)\b)/${1}internal $2/;
                }
                else { push @$leaks, "    line $n: $line" }
            }
            elsif (!$has_vis && $mode eq 'check' && $code =~ /\b(?:$DECL)\b/) {
                push @$leaks, "    line $n: $line";
            }
        }
        push @out, $line;
    }
    return @out;
}

sub seal_swift {
    my ($lines, $mode, $leaks) = @_;

    my $ATTR = qr/\@[\w.]+(?:\([^)]*\))?/;
    my $MOD  = qr/final|required|static|class|override|convenience|dynamic
                 |indirect|mutating|nonmutating|lazy|weak|unowned|nonisolated
                 |distributed|borrowing|consuming|prefix|postfix|infix|optional
                 |unsafe/x;

    my $shim_done = grep { /\Apackage import / } @$lines;

    my $st = { block => 0 };
    my @out;

    for my $n (1 .. scalar @$lines) {
        my $line = $lines->[$n - 1];
        my $in_comment = $st->{block};
        my $code = strip($line, $st, 0);

        if (!$in_comment) {
            if ($mode eq 'seal') {
                $line =~ s/\A(\s*(?:(?:$ATTR|$MOD)\s+)*)(?:public|open)\b/${1}package/;

                if (!$shim_done && $line =~ /\Aimport (\w+CFFI)\s*\z/) {
                    $line = "#if compiler(>=6.0)\npackage import $1\n#else\nimport $1\n#endif\n";
                    $shim_done = 1;
                }
            }
            elsif ($code =~ /\b(?:public|open)\b/) {
                push @$leaks, "    line $n: $line";
            }
        }
        push @out, $line;
    }
    return @out;
}
PERL_SEALER

seal() {
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
