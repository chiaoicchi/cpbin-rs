#!/usr/bin/env bash
set -euo pipefail

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

case "$site" in
  atcoder) url="https://atcoder.jp/contests/$contest" ;;
  codeforces) url="https://codeforces.com/contest/$contest" ;;
  *)
    echo "error: no contest URL rule for site: $site (add one to fetch.sh)" >&2
    exit 1
    ;;
esac

errlog="$(mktemp)"
trap 'rm -f "$errlog"' EXIT

json="$(oj-api get-contest "$url" 2> "$errlog")"
if [ "$(jq -r '.status' <<< "$json")" != "ok" ]; then
  echo "error: oj-api get-contest failed for $url" >&2
  exit 1
fi

count=0
while IFS=$'\t' read -r alphabet purl; do
  p="$(tr '[:upper:]' '[:lower:]' <<< "$alphabet")"
  dest="testcases/$p"
  if [ -d "$dest" ]; then
    echo "skip (already downloaded): $dest"
    continue
  fi
  if [ "$count" -gt 0 ]; then sleep 1; fi
  echo "downloading: $alphabet -> $dest"
  if ! oj download -d "$dest" "$purl" >> "$errlog" 2>&1; then
    cat "$errlog" >&2
    echo "error: oj download failed for $purl" >&2
    exit 1
  fi
  count=$((count + 1))
done < <(jq -r '.result.problems[] | [.context.alphabet, .url] | @tsv' <<< "$json")

echo "done: $count donwloaded"
