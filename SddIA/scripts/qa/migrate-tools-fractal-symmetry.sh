#!/usr/bin/env bash
set -euo pipefail
REPO_ROOT="$(cd "$(dirname "$0")/../../.." && pwd)"
cd "$REPO_ROOT"
TARGETS=(
  "SddIA_1/.SddIA"
  "SddIA_2/.SddIA"
  "SddIA_3/.SddIA"
  "SddIA_4/.SddIA"
  "SddIA/scripts/starter-kit/.SddIA"
)
for rel in "${TARGETS[@]}"; do
  legacy="$rel/Tools"
  canonical="$rel/tools"
  [[ -d "$legacy" ]] || continue
  mkdir -p "$canonical"
  shopt -s dotglob nullglob
  for f in "$legacy"/*; do
    base="$(basename "$f")"
    dest="$canonical/$base"
    if [[ -e "$dest" ]]; then
      [[ "$base" == "index.md" ]] && cp -f "$f" "$dest"
    else
      mv -f "$f" "$dest"
    fi
  done
  shopt -u dotglob nullglob
  rmdir "$legacy" 2>/dev/null || rm -rf "$legacy"
  git rm -rf --ignore-unmatch -- "$legacy" 2>/dev/null || true
  git add -- "$canonical" 2>/dev/null || true
  echo "OK $rel"
done
