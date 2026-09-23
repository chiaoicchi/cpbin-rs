#!/usr/bin/env bash
set -euo pipefail

profile="dev"
stress=0
while [ "$#" -gt 0 ]; do
  case "$1" in
    -r)
      profile="release"
      shift
      ;;
    -s)
      stress=1
      shift
      ;;
    -*)
      echo "usage: ck [-r] [-s] <problem>" >&2
      exit 1
      ;;
    *) break ;;
  esac
done

if [ "$#" -ne 1 ]; then
  echo "usage: ck [-r] [-s] <problem>" >&2
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
if [ ! -d "testcases/$p" ]; then
  echo "error: testcases/$p not found (run fetch first)" >&2
  exit 1
fi

if [ "$profile" = "release" ]; then
  cargo build --release --bin "$p"
  bin="$(cargo metadata --format-version 1 --no-deps | jq -r '.target_directory')/release/$p"
else
  cargo build --bin "$p"
  bin="$(cargo metadata --format-version 1 --no-deps | jq -r '.target_directory')/debug/$p"
fi

oj test -c "$bin" -d "testcases/$p"

if [ "$stress" -eq 1 ]; then
  if [ ! -d "stress/$p" ]; then
    echo "error: stress/$p not found (run stress first)" >&2
    exit 1
  fi
  echo "--- stress cases"
  oj test -c "$bin" -d "stress/$p"
fi
