#!/bin/sh
# Structural check for `evidence/`: the repository's audit trail of verification
# that was actually performed. Content is a human question; layout is not, and
# layout is what rots silently. Three rules, all mechanical:
#
#   1. flat            — a subdirectory means someone started filing by topic,
#                        and the date-first ordering stops working across it
#   2. Markdown only   — anything else is a raw artifact that belongs summarized
#                        in a record, not committed beside it
#   3. YYYY-MM-DD-topic — the date prefix is what makes `ls` chronological
#
# Plain POSIX sh with no runtime to install: this repository has no Node, Ruby,
# or Cargo toolchain, and a check that needed one would not run at all.
#
# A missing `evidence/` directory is not a failure. The convention is opt-in per
# repository and the directory appears on first real use.

set -eu

# Resolved from this script, not from cwd, so the check gives the same answer
# from a git hook, from CI, or by hand from a subdirectory.
repo_root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
evidence_dir="$repo_root/evidence"

[ -d "$evidence_dir" ] || exit 0

# Every violation, not just the first: fixing six names one round trip at a time
# is what makes a hook something people bypass.
problems=0
report() {
  problems=$((problems + 1))
  printf '  %s\n' "$1" >&2
}

for path in "$evidence_dir"/* "$evidence_dir"/.*; do
  name=${path##*/}
  # The globs above are literal when nothing matches, and `.`/`..` always match
  # the second one.
  case $name in
    '*' | '.*' | '.' | '..') [ -e "$path" ] || continue ;;
  esac

  if [ -d "$path" ]; then
    report "evidence/$name/ — evidence/ must be flat; no subdirectories"
    continue
  fi
  if [ ! -f "$path" ]; then
    report "evidence/$name — not a regular file"
    continue
  fi
  case $name in
    *.md) ;;
    *)
      report "evidence/$name — only .md files are allowed; summarize the artifact in a record instead of committing it"
      continue
      ;;
  esac

  # `expr` rather than a bashism: the month and day alternations are spelled out
  # because `2026-13-45-x.md` is exactly what a lazier [0-9][0-9] lets through.
  if ! expr "$name" : '[0-9][0-9][0-9][0-9]-\(0[1-9]\|1[0-2]\)-\(0[1-9]\|[12][0-9]\|3[01]\)-[a-z0-9]\{1,\}\(-[a-z0-9]\{1,\}\)*\.md$' >/dev/null; then
    report "evidence/$name — expected YYYY-MM-DD-<topic>.md with a lowercase hyphenated topic"
  fi
done

if [ "$problems" -gt 0 ]; then
  printf 'evidence/ layout check failed (%s). See the Evidence section of AGENTS.md.\n' "$problems" >&2
  exit 1
fi

printf 'evidence/ layout OK\n'
