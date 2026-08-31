#!/usr/bin/env bash
set -euo pipefail

if [ "$#" -ne 1 ]; then
  echo "usage: bd <problem>" >&2
  exit 1
fi

p="$1"

if ! root="$(git rev-parse --show-toplevel 2>/dev/null)"; then
  echo "error: run inside the git repository" >&2
  exit 1
fi
if [ "$(dirname "$(dirname "$PWD")")" != "$root" ]; then
  echo "error: run inside a contest directory (e.g. atcoder/abc001/)" >&2
  exit 1
fi

if [ ! -f "src/bin/$p.rs" ]; then
  echo "error: src/bin/$p.rs not found" >&2
  exit 1
fi

lib="$root/../cplib-rs"
if [ ! -f "$lib/Cargo.toml" ]; then
  echo "error: $lib not found:" >&2
  echo "  git clone git@github.com:chiaoicchi/cplib-rs.git ~/src/cplib-rs" >&2
  exit 1
fi

edition="$(sed -n 's/^edition *= *"\([0-9]*\)".*/\1/p' Cargo.toml | head -n 1)"

out="$(bundle-rs --lib "$lib" "src/bin/$p.rs" | rustfmt --edition "$edition")"
printf '%s\n' "$out" | wl-copy
bytes="$(printf '%s\n' "$out" | wc -c)"
echo "copied: src/bin/$p.rs -> clipboard ($bytes bytes)"
