#!/usr/bin/env bash
# Puerta Argos pre-commit: verify-process-integrity + audit EDA (Ola 5 — Rust).
set -euo pipefail
# shellcheck source=hook_common.sh
source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/hook_common.sh"

GENOME_PREFIXES=(
  "SddIA/skills/"
  "SddIA/events/"
  "SddIA/process/"
  "SddIA/agents/"
  "SddIA/tools/"
  "SddIA/actions/"
  "SddIA/library/norms/"
  "SddIA/library/codexes/"
  ".SddIA/"
)

staged_paths() {
  git_run diff --cached --name-only --diff-filter=ACMR | while IFS= read -r line; do
    [[ -n "$line" ]] || continue
    line="${line//\\//}"
    printf '%s\n' "$line"
  done
}

staged_touches_genome() {
  local path prefix
  while IFS= read -r path; do
    [[ -n "$path" ]] || continue
    for prefix in "${GENOME_PREFIXES[@]}"; do
      if [[ "$path" == "$prefix"* ]]; then
        return 0
      fi
    done
  done < <(staged_paths)
  return 1
}

main() {
  if skip_hooks; then
    echo "SddIA pre-commit: SKIPPED (SDDIA_SKIP_HOOKS=1)" >&2
    exit 0
  fi

  resolve_orchestrator

  if ! "$SDDIA_EXECUTE_PROCESS_BIN" --verify-process-integrity; then
    echo "SddIA pre-commit: BLOCKED — verify-process-integrity failed" >&2
    exit 1
  fi

  local report orphan_count
  report=$("$SDDIA_EXECUTE_PROCESS_BIN" --audit-eda-coverage --scan --json) || {
    echo "SddIA pre-commit: BLOCKED — audit-entity-eda-coverage error" >&2
    exit 1
  }

  orphan_count=$(printf '%s' "$report" | sed -n 's/.*"orphan_count"[[:space:]]*:[[:space:]]*\([0-9][0-9]*\).*/\1/p' | head -1)
  orphan_count="${orphan_count:-0}"

  if [[ "$orphan_count" -gt 0 ]] && staged_touches_genome; then
    echo "SddIA pre-commit: BLOCKED — Argos orphan_count=${orphan_count}" >&2
    printf '%s' "$report" | sed -n 's/.*"entity_class"[[:space:]]*:[[:space:]]*"\([^"]*\)".*"entity_name"[[:space:]]*:[[:space:]]*"\([^"]*\)".*"artifact_path"[[:space:]]*:[[:space:]]*"\([^"]*\)".*/  - \1\/\2 → \3/p' >&2 || true
    exit 1
  fi

  resolve_sddia_qa || exit 1
  local qa_json qa_rc=0 success="" exit_code=""
  qa_json=$("$SDDIA_QA_BIN" gate-evolution --json) || qa_rc=$?
  success=$(printf '%s' "$qa_json" | sed -n 's/.*"success"[[:space:]]*:[[:space:]]*\(true\|false\).*/\1/p' | tail -1)
  exit_code=$(printf '%s' "$qa_json" | sed -n 's/.*"exitCode"[[:space:]]*:[[:space:]]*\(-\{0,1\}[0-9][0-9]*\).*/\1/p' | tail -1)
  if [[ "$success" == "false" && "${exit_code:-$qa_rc}" -gt 0 ]]; then
    echo "SddIA pre-commit: BLOCKED — evolution gate success=false exitCode=${exit_code:-$qa_rc}" >&2
    printf '%s\n' "$qa_json" >&2
    exit 1
  fi
  if [[ -z "$success" && "$qa_rc" -ne 0 ]]; then
    echo "SddIA pre-commit: BLOCKED — evolution gate CLI rc=${qa_rc} (sobre ilegible)" >&2
    printf '%s\n' "$qa_json" >&2
    exit 1
  fi

  exit 0
}

main "$@"
