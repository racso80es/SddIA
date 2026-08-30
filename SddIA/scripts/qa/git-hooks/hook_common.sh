#!/usr/bin/env bash
# Utilidades compartidas hooks Git SddIA (Ola 5 — sin Python).
set -euo pipefail

HOOK_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
REPO=$(cd "$HOOK_DIR/../../../.." && pwd)
QA="$REPO/SddIA/scripts/qa"
CUMULO_PATH="$REPO/SddIA/core/cumulo.paths.json"
HOOK_DELIVERY_CLOSE_ENV="SDDIA_HOOK_DELIVERY_CLOSE"
BRANCH_PREFIXES=(feat/ fix/ refactor/ hotfix/)
MAIN_GUARD_MSG="Violación de Soberanía: main solo muta mediante el proceso accept-pr (PR merge). Push bloqueado."

# shellcheck source=/dev/null
source "$REPO/SddIA/scripts/common/sddia_shell_lib.sh"

resolve_sddia_qa() {
  local candidate
  for candidate in \
    "$REPO/SddIA/target/debug/sddia-qa" \
    "$REPO/SddIA/target/release/sddia-qa"; do
    if [[ -x "$candidate" ]]; then
      SDDIA_QA_BIN="$candidate"
      return 0
    fi
  done
  echo "SddIA pre-commit: sddia-qa no encontrado (compilar: cd SddIA && cargo build -p sddia-qa)" >&2
  return 1
}

skip_hooks() {
  [[ "${SDDIA_SKIP_HOOKS:-}" == "1" ]]
}

in_delivery_close_cycle() {
  [[ "${SDDIA_HOOK_DELIVERY_CLOSE:-}" == "1" ]]
}

# AEL-CA9: el hook corre gate-evolution solo si DCC no va a invocarse (cero ramas nuevas).
pre_push_hook_runs_evolution_gate() {
  local n="${1:-0}"
  [[ "$n" -eq 0 ]]
}

ref_to_branch() {
  local ref="$1"
  ref="${ref#refs/heads/}"
  printf '%s' "$ref"
}

is_main_ref() {
  [[ "$(ref_to_branch "$1")" == "main" ]]
}

branch_slug() {
  local name="$1"
  name="${name#"${name%%[![:space:]]*}"}"
  name="${name%"${name##*[![:space:]]}"}"
  local prefix
  for prefix in "${BRANCH_PREFIXES[@]}"; do
    if [[ "$name" == "$prefix"* ]]; then
      printf '%s' "${name#"$prefix"}"
      return 0
    fi
  done
  if [[ "$name" == */* ]]; then
    printf '%s' "${name#*/}"
    return 0
  fi
  printf '%s' "$name"
}

resolve_persist_ref() {
  local branch_name="$1"
  local slug kind candidate
  slug=$(branch_slug "$branch_name")
  [[ -n "$slug" ]] || return 0
  for kind in features fixes; do
    candidate="$REPO/docs/$kind/$slug"
    if [[ -d "$candidate" ]]; then
      printf 'docs/%s/%s' "$kind" "$slug"
      return 0
    fi
  done
}

eda_bus_dirs() {
  local key rel default
  for key in pending processing processed; do
    case "$key" in
      pending) default=".events/pending" ;;
      processing) default=".events/processing" ;;
      processed) default=".events/processed" ;;
    esac
    rel="$default"
    if [[ -f "$CUMULO_PATH" ]]; then
      local parsed
      parsed=$(sed -n "s/.*\"${key}\"[[:space:]]*:[[:space:]]*\"\([^\"]*\)\".*/\1/p" "$CUMULO_PATH" | head -1)
      if [[ -n "$parsed" ]]; then
        rel="${parsed#./}"
      fi
    fi
    if [[ -d "$REPO/$rel" ]]; then
      printf '%s\n' "$REPO/$rel"
    fi
  done
}

scan_presented_for_branch() {
  local target="$1"
  local bus_dir path
  while IFS= read -r bus_dir; do
    [[ -d "$bus_dir" ]] || continue
    for path in "$bus_dir"/*.json; do
      [[ -f "$path" ]] || continue
      if grep -q "\"event_type\"[[:space:]]*:[[:space:]]*\"PullRequest_Presented\"" "$path" 2>/dev/null \
        && grep -q "\"branch\"[[:space:]]*:[[:space:]]*\"${target}\"" "$path" 2>/dev/null; then
        return 0
      fi
    done
  done < <(eda_bus_dirs)
  return 1
}

_gh_pr_state_for_branch() {
  local branch="$1"
  local state
  state=$(gh pr view "$branch" --json state -q .state 2>/dev/null || true)
  printf '%s' "${state^^}"
}

gh_pr_open_for_branch() {
  [[ "$(_gh_pr_state_for_branch "$1")" == "OPEN" ]]
}

gh_pr_merged_for_branch() {
  [[ "$(_gh_pr_state_for_branch "$1")" == "MERGED" ]]
}

should_skip_pre_push_present() {
  local branch="$1"
  local state
  state=$(_gh_pr_state_for_branch "$branch")
  [[ "$state" == "OPEN" || "$state" == "MERGED" ]] && return 0
  scan_presented_for_branch "$branch"
}

git_run() {
  git -C "$REPO" "$@"
}

git_config() {
  local key="$1"
  local default="${2:-}"
  local value
  value=$(git_run config --get "$key" 2>/dev/null || true)
  if [[ -n "$value" ]]; then
    printf '%s' "$value"
  else
    printf '%s' "$default"
  fi
}

_write_ephemeral_json() {
  local prefix="$1"
  local payload="$2"
  local tmp
  tmp=$(mktemp "${TMPDIR:-/tmp}/${prefix}.XXXXXX.json")
  printf '%s' "$payload" > "$tmp"
  printf '%s' "$tmp"
}

invoke_process() {
  local process_name="$1"
  local payload="$2"
  _sddia_resolve_orchestrator "$REPO"
  local tmp rc=0
  tmp=$(_write_ephemeral_json "hook-${process_name}" "$payload")
  export SDDIA_HOOK_DELIVERY_CLOSE=1
  "$SDDIA_EXECUTE_PROCESS_BIN" --process "$process_name" --inputs-file "$tmp" >&2 || rc=$?
  rm -f "$tmp"
  return "$rc"
}

parse_pre_push_stdin() {
  local line local_ref local_sha remote_ref remote_sha
  while IFS= read -r line; do
    [[ -n "$line" ]] || continue
    read -r local_ref local_sha remote_ref remote_sha <<< "$line"
    [[ -n "$local_ref" ]] || continue
    printf '%s|%s|%s|%s\n' "$local_ref" "$local_sha" "$remote_ref" "$remote_sha"
  done
}

is_delete_push() {
  [[ "$1" =~ ^0+$ ]]
}

infer_merged_branch() {
  git_run rev-parse --verify HEAD^2 >/dev/null 2>&1 || return 1
  local msg branch
  msg=$(git_run log -1 --pretty=%B 2>/dev/null || true)
  if [[ "$msg" =~ Merge\ branch\ \'([^\']+)\' ]]; then
    printf '%s' "${BASH_REMATCH[1]}"
    return 0
  fi
  branch=$(git_run name-rev --name-only HEAD^2 2>/dev/null || true)
  branch="${branch#remotes/origin/}"
  branch="${branch#remotes/}"
  branch="${branch//\~}"
  branch="${branch//^}"
  [[ -n "$branch" ]] && printf '%s' "$branch"
}

resolve_orchestrator() {
  _sddia_resolve_orchestrator "$REPO"
}
