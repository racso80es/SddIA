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
_sddia_augment_operator_path

resolve_sddia_qa() {
  if [[ -n "${SDDIA_QA_BIN:-}" ]] && _sddia_is_native_elf "$SDDIA_QA_BIN"; then
    export SDDIA_QA_BIN
    return 0
  fi
  local debug_bin release_bin d_ok=0 r_ok=0 dm rm_
  debug_bin="$REPO/SddIA/target/debug/sddia-qa"
  release_bin="$REPO/SddIA/target/release/sddia-qa"
  if _sddia_is_native_elf "$debug_bin"; then d_ok=1; fi
  if _sddia_is_native_elf "$release_bin"; then r_ok=1; fi
  # F-DEP-07: release salvo debug estrictamente más nuevo.
  if [[ "$d_ok" -eq 1 && "$r_ok" -eq 1 ]]; then
    dm="$(_sddia_elf_mtime "$debug_bin")"
    rm_="$(_sddia_elf_mtime "$release_bin")"
    if [[ -n "$dm" && -n "$rm_" && "$dm" -gt "$rm_" ]]; then
      SDDIA_QA_BIN="$debug_bin"
    else
      SDDIA_QA_BIN="$release_bin"
    fi
    export SDDIA_QA_BIN
    return 0
  fi
  if [[ "$r_ok" -eq 1 ]]; then
    SDDIA_QA_BIN="$release_bin"
    export SDDIA_QA_BIN
    return 0
  fi
  if [[ "$d_ok" -eq 1 ]]; then
    SDDIA_QA_BIN="$debug_bin"
    export SDDIA_QA_BIN
    return 0
  fi
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
  if [[ "$ref" == "HEAD" ]]; then
    git -C "$REPO" symbolic-ref --short HEAD 2>/dev/null || printf '%s' "$ref"
    return 0
  fi
  printf '%s' "$ref"
}

is_main_ref() {
  [[ "$(ref_to_branch "$1")" == "main" ]]
}

# Espejo de project_binding::main_push_allowed.
# Core (cumulo presente) siempre veta. Cliente trunk_direct no veta.
main_push_veto() {
  local top manifest
  top=$(git rev-parse --show-toplevel 2>/dev/null || printf '%s' "$REPO")
  if [[ -f "$top/SddIA/core/cumulo.paths.json" ]]; then
    return 0
  fi
  manifest="$top/.SddIA/project.md"
  if [[ -f "$manifest" ]] && grep -Eq '^delivery_mode:[[:space:]]*["'\'']?trunk_direct["'\'']?[[:space:]]*$' "$manifest"; then
    return 1
  fi
  return 0
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
  if scan_presented_for_branch "$branch"; then
    return 0
  fi
  local state
  state=$(_gh_pr_state_for_branch "$branch")
  [[ "$state" == "OPEN" || "$state" == "MERGED" ]]
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
  local tmp err_file rc=0 phase_hint
  tmp=$(_write_ephemeral_json "hook-${process_name}" "$payload")
  err_file=$(mktemp "${TMPDIR:-/tmp}/hook-${process_name}.XXXXXX.stderr")
  export SDDIA_HOOK_DELIVERY_CLOSE=1
  export SDDIA_AGENT_RUNTIME_TIMEOUT_SECS="${SDDIA_AGENT_RUNTIME_TIMEOUT_SECS:-180}"
  hook_timing_record_process "$process_name"
  "$SDDIA_EXECUTE_PROCESS_BIN" --process "$process_name" --inputs-file "$tmp" 2>"$err_file" || rc=$?
  if [[ -s "$err_file" ]]; then
    cat "$err_file" >&2
  fi
  if [[ "$rc" -ne 0 ]]; then
    phase_hint=$(
      python3 - "$err_file" 2>/dev/null <<'PY' || true
import json, re, sys
text = open(sys.argv[1], encoding="utf-8", errors="replace").read()
for line in reversed(text.splitlines()):
    line = line.strip()
    if not line:
        continue
    try:
        obj = json.loads(line)
    except json.JSONDecodeError:
        m = re.search(r'"phase_name"\s*:\s*"([^"]+)"', line)
        if m:
            print(m.group(1))
            break
        continue
    v = obj.get("phase_name")
    if isinstance(v, str) and v.strip():
        print(v.strip())
        sys.exit(0)
    data = obj.get("data") if isinstance(obj.get("data"), dict) else {}
    v = data.get("phase_name")
    if isinstance(v, str) and v.strip():
        print(v.strip())
        sys.exit(0)
PY
    )
    if [[ -n "$phase_hint" ]]; then
      echo "SddIA hook: proceso «${process_name}» falló en fase «${phase_hint}» (exit ${rc})" >&2
    else
      echo "SddIA hook: proceso «${process_name}» falló (exit ${rc})" >&2
    fi
  fi
  rm -f "$tmp" "$err_file"
  return "$rc"
}

parse_pre_push_stdin() {
  local line local_ref local_sha remote_ref remote_sha
  while IFS= read -r line || [[ -n "$line" ]]; do
    [[ -n "$line" ]] || continue
    read -r local_ref local_sha remote_ref remote_sha <<< "$line"
    [[ -n "$local_ref" ]] || continue
    printf '%s|%s|%s|%s\n' "$local_ref" "$local_sha" "$remote_ref" "$remote_sha"
  done
}

# Argumento: SHA **local** del stdin pre-push. Ceros = delete. SHA remoto cero = ref nueva (no delete).
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

resolve_eda_proofs_dir() {
  local rel=".SddIA/proofs"
  if [[ -f "$CUMULO_PATH" ]]; then
    local parsed
    parsed=$(sed -n 's/.*"proofs"[[:space:]]*:[[:space:]]*"\([^"]*\)".*/\1/p' "$CUMULO_PATH" | head -1)
    if [[ -n "$parsed" ]]; then
      rel="${parsed#./}"
    fi
  fi
  printf '%s\n' "$REPO/$rel"
}

hook_timing_now_ms() {
  python3 -c 'import time; print(int(time.time() * 1000))'
}

HOOK_TIMING_START_MS=""
HOOK_TIMING_INVOKED_JSON='[]'

hook_timing_begin() {
  HOOK_TIMING_START_MS=$(hook_timing_now_ms)
  HOOK_TIMING_INVOKED_JSON='[]'
}

hook_timing_record_process() {
  local name="$1"
  [[ -n "$name" ]] || return 0
  HOOK_TIMING_INVOKED_JSON=$(
    python3 -c 'import json,sys; a=json.loads(sys.argv[1]); a.append(sys.argv[2]); print(json.dumps(a))' \
      "$HOOK_TIMING_INVOKED_JSON" "$name"
  )
}

append_hook_timing_jsonl() {
  local hook_name="$1"
  local branch="$2"
  local total_ms="$3"
  local invoked_json="${4:-[]}"
  local proofs dir line recorded_at
  proofs=$(resolve_eda_proofs_dir)
  dir="$proofs/hook-timings"
  mkdir -p "$dir"
  recorded_at=$(date -u +"%Y-%m-%dT%H:%M:%SZ" 2>/dev/null || python3 -c 'from datetime import datetime,timezone; print(datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ"))')
  line=$(
    python3 -c 'import json,sys; print(json.dumps({"hook":sys.argv[1],"branch":sys.argv[2],"total_ms":int(sys.argv[3]),"invoked_processes":json.loads(sys.argv[4]),"delta_class":None,"attestation_hit":None,"recorded_at":sys.argv[5]}, separators=(",",":")))' \
      "$hook_name" "$branch" "$total_ms" "$invoked_json" "$recorded_at"
  )
  printf '%s\n' "$line" >> "$dir/hook-timings.jsonl"
}

hook_timing_flush() {
  local hook_name="$1"
  local branch="${2:-}"
  local end_ms total_ms
  [[ -n "${HOOK_TIMING_START_MS:-}" ]] || return 0
  end_ms=$(hook_timing_now_ms)
  total_ms=$((end_ms - HOOK_TIMING_START_MS))
  if [[ -z "$branch" ]]; then
    branch=$(git_run symbolic-ref --short HEAD 2>/dev/null || printf '_unknown')
  fi
  append_hook_timing_jsonl "$hook_name" "$branch" "$total_ms" "$HOOK_TIMING_INVOKED_JSON"
  HOOK_TIMING_START_MS=""
  HOOK_TIMING_INVOKED_JSON='[]'
}
