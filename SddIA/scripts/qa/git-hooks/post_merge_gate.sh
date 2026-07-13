#!/usr/bin/env bash
# Puerta post-merge Ola B: accept-pr con merge_already_done (Ola 5).
set -euo pipefail
# shellcheck source=hook_common.sh
source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/hook_common.sh"

main() {
  if skip_hooks; then
    echo "SddIA post-merge: SKIPPED (SDDIA_SKIP_HOOKS=1)" >&2
    exit 0
  fi

  local head
  head=$(git_run symbolic-ref -q HEAD 2>/dev/null || true)
  [[ "$head" == "refs/heads/main" ]] || exit 0

  local source_branch
  source_branch=$(infer_merged_branch || true)
  if [[ -z "$source_branch" ]]; then
    echo "SddIA post-merge: no merge branch inferred — no-op" >&2
    exit 0
  fi

  local author correlation_id payload
  author=$(git_config user.email "unknown@sddia.local")
  correlation_id=$(uuidgen 2>/dev/null || cat /proc/sys/kernel/random/uuid 2>/dev/null || echo "local-$(date +%s)")
  payload=$(printf '{"source_branch":"%s","author":"%s","correlation_id":"%s","merge_already_done":true}' \
    "$source_branch" "$author" "$correlation_id")

  if ! invoke_process "accept-pr" "$payload"; then
    echo "SddIA post-merge: BLOCKED — accept-pr failed for ${source_branch}" >&2
    exit 1
  fi
}

main "$@"
