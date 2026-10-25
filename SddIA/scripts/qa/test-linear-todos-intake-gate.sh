#!/usr/bin/env bash
# Lab: gate pre-commit + helpers linear todos intake (AUT-14).
set -euo pipefail
HOOK_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/git-hooks" && pwd)"
# shellcheck source=git-hooks/hook_common.sh
source "$HOOK_DIR/hook_common.sh"

if ! tracker_provider_is_linear; then
  echo "SKIP: manifiesto sin provider linear" >&2
  exit 0
fi

assert_blocked_path() {
  local path="$1"
  if path_under_todos_bucket "$path"; then
    return 0
  fi
  echo "FAIL: se esperaba bucket todos para $path" >&2
  exit 1
}

assert_open_path() {
  local path="$1"
  if path_under_todos_bucket "$path"; then
    echo "FAIL: no debería ser bucket todos: $path" >&2
    exit 1
  fi
}

assert_blocked_path "docs/todos/pending/foo.md"
assert_blocked_path "docs/todos/done/bar.md"
assert_open_path "docs/features/x/objectives.md"

echo "OK: test-linear-todos-intake-gate"
