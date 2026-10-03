#!/usr/bin/env bash
# Puerta pre-push Ola B: guarda main, idempotencia PR, delivery-close-cycle (Ola 5).
set -euo pipefail
# shellcheck source=hook_common.sh
source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/hook_common.sh"

run_evolution_gate() {
  resolve_sddia_qa || return 1
  hook_timing_record_process "gate-evolution"
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

  hook_timing_begin

  local stdin_text line local_ref local_sha remote_ref remote_sha
  # $(cat) recorta el \n final; el marcador conserva el payload de git pre-push.
  stdin_text=$(cat; printf x)
  stdin_text="${stdin_text%x}"
  if [[ -z "$stdin_text" ]]; then
    hook_timing_flush "pre-push"
    exit 0
  fi

  local branches=()
  local branch_shas=()
  while IFS='|' read -r local_ref local_sha remote_ref remote_sha || [[ -n "${local_ref:-}" ]]; do
    [[ -n "$local_ref" ]] || continue
    if is_delete_push "$local_sha"; then
      continue
    fi
    if is_main_ref "$local_ref"; then
      if main_push_veto; then
        hook_timing_flush "pre-push"
        echo "$MAIN_GUARD_MSG" >&2
        exit 1
      fi
      continue
    fi
    local branch
    branch=$(ref_to_branch "$local_ref")
    if [[ -z "$branch" || "$branch" == "main" ]]; then
      if main_push_veto; then
        hook_timing_flush "pre-push"
        echo "$MAIN_GUARD_MSG" >&2
        exit 1
      fi
      continue
    fi
    if should_skip_pre_push_present "$branch"; then
      continue
    fi
    branches+=("$branch")
    branch_shas+=("${branch}|${local_sha}|${remote_sha}")
  done < <(printf '%s' "$stdin_text" | parse_pre_push_stdin)

  if pre_push_hook_runs_evolution_gate "${#branches[@]}"; then
    run_evolution_gate || {
      hook_timing_flush "pre-push" "_evolution_gate"
      exit 1
    }
    hook_timing_flush "pre-push" "_evolution_gate"
    exit 0
  fi

  local exit_code=0 branch persist_ref slug payload qa_payload timing_branch=""
  local entry local_sha remote_sha paths delta_class qa_profile
  for entry in "${branch_shas[@]}"; do
    IFS='|' read -r branch local_sha remote_sha <<< "$entry"
    timing_branch="$branch"
    paths=$(delta_paths_for_push "$remote_sha" "$local_sha" || true)
    delta_class=$(delta_class_for_paths "$paths")
    HOOK_TIMING_DELTA_CLASS="$delta_class"
    if [[ "$delta_class" == "passive" ]]; then
      qa_profile="docs-only"
    else
      qa_profile="full"
    fi
    qa_payload=$(
      python3 -c 'import json,sys; print(json.dumps({"event_type":"Local_QA_Requested","blocking":True,"emitter_agent":"git-hook-pre-push","payload":{"branch":sys.argv[1],"qa_profile":sys.argv[2]}}))' \
        "$branch" "$qa_profile"
    )
    if ! invoke_process "route-domain-event" "$qa_payload"; then
      echo "SddIA pre-push: BLOCKED — Local_QA_Requested failed for ${branch}" >&2
      exit_code=1
      continue
    fi

    persist_ref=$(resolve_persist_ref "$branch" || true)
    slug=$(branch_slug "$branch")
    payload=$(
      python3 -c 'import json,sys; b,s,pref,q=sys.argv[1],sys.argv[2],sys.argv[3],sys.argv[4]; d={"source_process":"git-hook-pre-push","branch_name":b,"pr_title":f"feat: {s or b}","pr_body":"Presentación automática vía hook pre-push (PBI-005 Ola B).","target_branch":"main","qa_profile":q}; d["persist_ref"]=pref if pref else None; print(json.dumps(d))' \
        "$branch" "${slug:-$branch}" "${persist_ref:-}" "$qa_profile"
    )

    if ! invoke_process "delivery-close-cycle" "$payload"; then
      echo "SddIA pre-push: BLOCKED — delivery-close-cycle failed for ${branch}" >&2
      exit_code=1
    fi
  done

  if [[ ${#branches[@]} -gt 1 ]]; then
    timing_branch="_multi"
  fi
  hook_timing_flush "pre-push" "$timing_branch"
  exit "$exit_code"
}

main "$@"
