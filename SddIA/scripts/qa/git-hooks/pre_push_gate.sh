#!/usr/bin/env bash
# Puerta pre-push Ola B: guarda main, idempotencia PR, delivery-close-cycle (Ola 5).
set -euo pipefail
# shellcheck source=hook_common.sh
source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/hook_common.sh"

run_evolution_gate() {
  resolve_sddia_qa || return 1
  if ! "$SDDIA_QA_BIN" gate-evolution --json --range --if-touched --sync-base; then
    echo "SddIA pre-push: BLOCKED — evolution gate (--range --if-touched) failed" >&2
    return 1
  fi
}

main() {
  if skip_hooks; then
    echo "SddIA pre-push: SKIPPED (SDDIA_SKIP_HOOKS=1)" >&2
    exit 0
  fi

  if in_delivery_close_cycle; then
    echo "SddIA pre-push: SKIPPED (delivery-close-cycle guard)" >&2
    exit 0
  fi

  local stdin_text line local_ref local_sha remote_ref remote_sha
  stdin_text=$(cat)
  [[ -n "$stdin_text" ]] || exit 0

  local branches=()
  while IFS='|' read -r local_ref local_sha remote_ref remote_sha; do
    [[ -n "$local_ref" ]] || continue
    if is_delete_push "$remote_sha"; then
      continue
    fi
    if is_main_ref "$local_ref"; then
      echo "$MAIN_GUARD_MSG" >&2
      exit 1
    fi
    local branch
    branch=$(ref_to_branch "$local_ref")
    if [[ -z "$branch" || "$branch" == "main" ]]; then
      echo "$MAIN_GUARD_MSG" >&2
      exit 1
    fi
    if should_skip_pre_push_present "$branch"; then
      continue
    fi
    branches+=("$branch")
  done < <(printf '%s' "$stdin_text" | parse_pre_push_stdin)

  run_evolution_gate || exit 1

  if [[ "${#branches[@]}" -eq 0 ]]; then
    exit 0
  fi

  local exit_code=0 branch persist_ref slug payload qa_payload
  for branch in "${branches[@]}"; do
    qa_payload=$(printf '{"event_type":"Local_QA_Requested","blocking":true,"emitter_agent":"git-hook-pre-push","payload":{"branch":"%s"}}' "$branch")
    if ! invoke_process "route-domain-event" "$qa_payload"; then
      echo "SddIA pre-push: BLOCKED — Local_QA_Requested failed for ${branch}" >&2
      exit_code=1
      continue
    fi

    persist_ref=$(resolve_persist_ref "$branch" || true)
    slug=$(branch_slug "$branch")
    if [[ -n "$persist_ref" ]]; then
      payload=$(printf '{"source_process":"git-hook-pre-push","branch_name":"%s","pr_title":"feat: %s","pr_body":"Presentación automática vía hook pre-push (PBI-005 Ola B).","target_branch":"main","persist_ref":"%s"}' \
        "$branch" "${slug:-$branch}" "$persist_ref")
    else
      payload=$(printf '{"source_process":"git-hook-pre-push","branch_name":"%s","pr_title":"feat: %s","pr_body":"Presentación automática vía hook pre-push (PBI-005 Ola B).","target_branch":"main","persist_ref":null}' \
        "$branch" "${slug:-$branch}")
    fi

    if ! invoke_process "delivery-close-cycle" "$payload"; then
      echo "SddIA pre-push: BLOCKED — delivery-close-cycle failed for ${branch}" >&2
      exit_code=1
    fi
  done

  exit "$exit_code"
}

main "$@"
