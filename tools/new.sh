#!/usr/bin/env bash
set -euo pipefail

if [ "$#" -lt 2 ]; then
  echo "usage: new <contest> <problem>..." >&2
  echo "example: new abc001 {a..g}" >&2
  echo "         new cf0001 a b1 b2 c" >&2
  exit 1
fi

contest="$1"
shift
problems=("$@")

if ! root="$(git rev-parse --show-toplevel 2>/dev/null)"; then
  echo "error: run inside the git repository" >&2
  exit 1
fi

site="$(basename "$PWD")"
if [ "$(dirname "$PWD")" != "$root" ]; then
  echo "error: run directly under a site directory (e.g. atcoder/)" >&2
  exit 1
fi
case "$site" in
  tools | archive | .*)
    echo "error: $site is not a site directory" >&2
    exit 1
    ;;
esac

if ! [[ "$contest" =~ ^[a-z0-9][a-z0-9_-]*$ ]]; then
  echo "error: invalid contest name: $contest" >&2
  exit 1
fi

for p in "${problems[@]}"; do
  if ! [[ "$p" =~ ^[a-z0-9][a-z0-9_]*$ ]]; then
    echo "error: invalid problem id: $p" >&2
    exit 1
  fi
done

pkg="$site-$contest"
dir="$PWD/$contest"
created=()

if [ ! -e "$dir" ]; then
  tpl_cargo="$root/tools/templates/${site}/Cargo.toml"
  if [ ! -f "$tpl_cargo" ]; then
    echo "error: template not found: $tpl_cargo" >&2
    exit 1
  fi
  mkdir -p "$dir"
  sed "s/@NAME@/$pkg/g" "$tpl_cargo" > "$dir/Cargo.toml"
  created+=("$site/$contest/Cargo.toml")
fi

tpl_main="$root/tools/templates/${site}/main.rs"
if [ ! -f "$tpl_main" ]; then
  echo "error: template not found: $tpl_main" >&2
  exit 1
fi
mkdir -p "$dir/src/bin"
for p in "${problems[@]}"; do
  if [ ! -f "$dir/src/bin/$p.rs" ]; then
    cp "$tpl_main" "$dir/src/bin/$p.rs"
    created+=("$site/$contest/src/bin/$p.rs")
  else
    echo "skip (already exists): $site/$contest/src/bin/$p.rs"
  fi
done

echo "created:"
for f in "${created[@]}"; do
  echo "  $f"
done
