#!/usr/bin/env bash
set -euo pipefail

usage() {
  echo "usage: stress [-a] [-r] [-n <count>] <problem>" >&2
  exit 2
}

assert=0
profile="dev"
count=1000
while [ "$#" -gt 0 ]; do
  case "$1" in
    -a)
      assert=1
      shift
      ;;
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

if [ "$#" -ne 1 ]; then
  usage
fi

p="$1"
naive="${p}_naive"
gen="${p}_gen"

if ! root="$(git rev-parse --show-toplevel 2>/dev/null)"; then
  echo "error: run inside the git repository" >&2
  exit 1
fi
if [ "$(dirname "$(dirname "$PWD")")" != "$root" ]; then
  echo "error: run inside a contest directory (e.g. atcoder/abc001/)" >&2
  exit 1
fi

if [ "$assert" -eq 1 ]; then
  bins=("$naive" "$gen")
else
  bins=("$p" "$naive" "$gen")
fi
for f in "${bins[@]}"; do
  if [ ! -f "src/bin/$f.rs" ]; then
    echo "error: src/bin/$f.rs not found (run sn $p first)" >&2
    exit 1
  fi
done

build=()
for f in "${bins[@]}"; do
  build+=(--bin "$f")
done
if [ "$profile" = "release" ]; then
  cargo build --release "${build[@]}"
  dir="$(cargo metadata --format-version 1 --no-deps | jq -r '.target_directory')/release"
else
  cargo build "${build[@]}"
  dir="$(cargo metadata --format-version 1 --no-deps | jq -r '.target_directory')/debug"
fi

in="$(mktemp)"
err="$(mktemp)"
trap 'rm -f "$in" "$err"' EXIT

save() {
  mkdir -p "$1"
  i=1
  while [ -e "$1/$i.in" ]; do
    i=$((i + 1))
  done
  cp "$in" "$1/$i.in"
  saved="$1/$i.in"
}

for ((k = 1; k <= count; k++)); do
  if ! "$dir/$gen" "$k" > "$in"; then
    echo "generator stopped at $k"
    exit 0
  fi

  if [ "$assert" -eq 1 ]; then
    if ! "$dir/$naive" < "$in" > /dev/null 2> "$err"; then
      save "stress/$naive"
      echo "RE: $naive at $k (saved as $saved)"
      echo "--- input"
      cat "$in"
      echo "--- stderr"
      cat "$err"
      exit 1
    fi
    continue
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
    save "stress/$p"
    printf '%s\n' "$want" > "${saved%.in}.out"
    echo "WA at $k (saved as $saved)"
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
