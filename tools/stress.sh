#!/usr/bin/env bash
set -euo pipefail

usage() {
  echo "usage: stress [-r] [-n <count>] <problem> <naive> <gen>" >&2
  exit 2
}

profile="dev"
count=1000
while [ "$#" -gt 0 ]; do
  case "$1" in
    -r)
      profile="release"
      shift
      ;;
    -n)
      [ "$#" -ge 2 ] || usage
      count="$2"
      shift 2
      ;;
    -*) usage ;;
    *) break ;;
  esac
done

if [ "$#" -ne 3 ]; then
  usage
fi

p="$1"
naive="$2"
gen="$3"

if ! root="$(git rev-parse --show-toplevel 2>/dev/null)"; then
  echo "error: run inside the git repository" >&2
  exit 1
fi
if [ "$(dirname "$(dirname "$PWD")")" != "$root" ]; then
  echo "error: run inside a contest directory (e.g. atcoder/abc001/)" >&2
  exit 1
fi

for f in "$p" "$naive" "$gen"; do
  if [ ! -f "src/bin/$f.rs" ]; then
    echo "error: src/bin/$f.rs not found" >&2
    exit 1
  fi
done

if [ "$profile" = "release" ]; then
  cargo build --release --bin "$p" --bin "$naive" --bin "$gen"
  dir="$(cargo metadata --format-version 1 --no-deps | jq -r '.target_directory')/release"
else
  cargo build --bin "$p" --bin "$naive" --bin "$gen"
  dir="$(cargo metadata --format-version 1 --no-deps | jq -r '.target_directory')/debug"
fi

out="stress/$p"
mkdir -p "$out"
in="$(mktemp)"
trap 'rm -f "$in"' EXIT

for ((k = 1; k <= count; k++)); do
  if ! "$dir/$gen" "$k" > "$in"; then
    echo "generator stopped at $k"
    exit 0
  fi

  if ! got="$("$dir/$p" < "$in")"; then
    echo "RE: $p"
    cat "$in"
    exit 1
  fi
  if ! want="$("$dir/$naive" < "$in")"; then
    echo "RE: $naive"
    cat "$in"
    exit 1
  fi

  if [ "$got" != "$want" ]; then
    i=1
    while [ -e "$out/$i.in" ]; do
      i=$((i + 1))
    done
    cp "$in" "$out/$i.in"
    printf '%s\n' "$want" > "$out/$i.out"
    echo "WA at $k (saved as $out/$i.in)"
    echo "--- input"
    cat "$in"
    echo "--- $p"
    printf '%s\n' "$got"
    echo "--- $naive"
    printf '%s\n' "$want"
    exit 1
  fi
done

echo "AC $count cases"
