#!/usr/bin/env bash
set -euo pipefail

if [ "$#" -lt 1 ]; then
  echo "usage: sn <problem>..." >&2
  echo "example: sn f" >&2
  exit 1
fi

problems=("$@")

if ! root="$(git rev-parse --show-toplevel 2>/dev/null)"; then
  echo "error: run inside the git repository" >&2
  exit 1
fi

contest="$(basename "$PWD")"
site="$(basename "$(dirname "$PWD")")"
if [ "$(dirname "$(dirname "$PWD")")" != "$root" ]; then
  echo "error: run inside a contest directory (e.g. atcoder/abc001/)" >&2
  exit 1
fi

for p in "${problems[@]}"; do
  if ! [[ "$p" =~ ^[a-z0-9][a-z0-9_]*$ ]]; then
    echo "error: invalid problem id: $p" >&2
    exit 1
  fi
  if [ ! -f "src/bin/$p.rs" ]; then
    echo "error: src/bin/$p.rs not found" >&2
    exit 1
  fi
done

tpl_naive="$root/tools/templates/${site}/naive.rs"
tpl_gen="$root/tools/templates/${site}/gen.rs"
for f in "$tpl_naive" "$tpl_gen"; do
  if [ ! -f "$f" ]; then
    echo "error: template not found: $f" >&2
    exit 1
  fi
done

created=()
for p in "${problems[@]}"; do
  for kind in naive gen; do
    dst="src/bin/${p}_${kind}.rs"
    if [ -f "$dst" ]; then
      echo "skip (already exists): $site/$contest/$dst"
      continue
    fi
    if [ "$kind" = "naive" ]; then
      cp "$tpl_naive" "$dst"
    else
      cp "$tpl_gen" "$dst"
    fi
    created+=("$site/$contest/$dst")
  done
done

if [ "${#created[@]}" -eq 0 ]; then
  exit 0
fi

echo "created:"
for f in "${created[@]}"; do
  echo "  $f"
done
