#!/usr/bin/env bash
# Smokes LAB-CA1–CA4, CA6–CA9 (T6). Ejecutar desde raíz del repo.
set -euo pipefail
REPO_ROOT="$(cd "$(dirname "$0")/../../../.." && pwd)"
cd "$REPO_ROOT"

echo "== build =="
( cd "$REPO_ROOT/SddIA" && CARGO_TARGET_DIR="$REPO_ROOT/SddIA/target" cargo build -p execute-process -q )

EP="$REPO_ROOT/SddIA/target/debug/execute-process"
fail=0
pass() { echo "PASS $1"; }
fail_msg() { echo "FAIL $1: $2"; fail=1; }

# LAB-CA1 / CA2: relay flag → simulated rápido
export SDDIA_AGENT_RELAY_IDE=1
export SDDIA_AGENT_RUNTIME_COMMAND="sleep 999"
export SDDIA_LAB_SKIP_PBI_ARCHIVE=1
export SDDIA_LAB_SKIP_DELIVERY_CLOSE=1
export SDDIA_LAB_SKIP_GIT=1
t0=$(date +%s)
out=$(./sddia-run.sh --process feature --inputs '{"feature_name":"smoke-relay","branch_name":"feat/smoke-relay","persist_ref":"docs/features/kaizen-feature-lab-init-frictions/.tmp/smoke-relay","base_branch":"main"}' 2>&1)
t1=$(date +%s)
elapsed=$((t1 - t0))
if echo "$out" | grep -q '"success":true' && echo "$out" | grep -q '"status":"simulated"' && [[ "$elapsed" -lt 15 ]]; then
  pass LAB-CA1
else
  fail_msg LAB-CA1 "elapsed=${elapsed}s out=${out:0:200}"
fi
unset SDDIA_AGENT_RELAY_IDE SDDIA_AGENT_RUNTIME_COMMAND

# LAB-CA3: paridad shell setdefault
(
  export SDDIA_LAB_SIMULATE_IOTA=0
  export SDDIA_FOO_PARITY_TEST=from_env
  # shellcheck source=SddIA/scripts/common/sddia_shell_lib.sh
  source "$REPO_ROOT/SddIA/scripts/common/sddia_shell_lib.sh"
  _sddia_load_vault "$REPO_ROOT" 2>/dev/null || true
  if [[ "${SDDIA_FOO_PARITY_TEST:-}" == "from_env" ]]; then
    pass LAB-CA3-setdefault
  else
    fail_msg LAB-CA3 "env var fue pisada"
  fi
)

# LAB-CA4: timeout motor (cargo test)
if ( cd "$REPO_ROOT/SddIA" && CARGO_TARGET_DIR="$REPO_ROOT/SddIA/target" cargo test -p execute-process timeout_kills_hanging_command -- --nocapture -q 2>/dev/null ); then
  pass LAB-CA4
else
  fail_msg LAB-CA4 "test timeout_kills_hanging_command"
fi

# LAB-CA6: execution_id en acuse
out2=$(SDDIA_LAB_SKIP_GIT=1 SDDIA_LAB_SKIP_PBI_ARCHIVE=1 SDDIA_LAB_SKIP_DELIVERY_CLOSE=1 SDDIA_AGENT_RUNTIME_COMMAND="" \
  ./sddia-run.sh --process feature --inputs '{"feature_name":"smoke-eid","branch_name":"feat/smoke-eid","persist_ref":"docs/features/kaizen-feature-lab-init-frictions/.tmp/smoke-eid"}' 2>&1)
eid=$(echo "$out2" | python3 -c 'import sys,json; d=json.loads(sys.stdin.read().split("{",1)[-1].rsplit("}",1)[0] if "{" in sys.stdin.read() else "{}"); print(d.get("data",{}).get("execution_id",""))' 2>/dev/null || true)
if [[ -z "$eid" ]]; then
  eid=$(echo "$out2" | grep -oP '"execution_id":"[^"]+"' | head -1 | cut -d'"' -f4)
fi
if [[ -n "$eid" && -d "$REPO_ROOT/.SddIA/workspaces/feature/$eid" ]]; then
  pass LAB-CA6
else
  fail_msg LAB-CA6 "eid=$eid workspace missing"
fi

# LAB-CA9: censo índice daemons
count=$(grep -c '| `.*\.md` |' "$REPO_ROOT/SddIA/daemons/index.md" || true)
if grep -q "$count Centinelas catalogados" "$REPO_ROOT/SddIA/daemons/index.md"; then
  pass LAB-CA9
else
  fail_msg LAB-CA9 "censo no coincide con $count filas"
fi

# LAB-CA8: entity-manager acepta daemon (dry invoke PILOT)
if grep -q '"daemon"' "$REPO_ROOT/SddIA/engine/execute-process/src/engine/entity_manager.rs"; then
  pass LAB-CA8-pilot
else
  fail_msg LAB-CA8 "daemon no en PILOT_CLASSES"
fi

# LAB-CA5: sin huérfanos del grupo tras timeout
count_orphans() { { pgrep -f 'sddia-agent-to' 2>/dev/null || true; } | wc -l | tr -d ' '; }
before=$(count_orphans)
( cd "$REPO_ROOT/SddIA" && CARGO_TARGET_DIR="$REPO_ROOT/SddIA/target" \
  cargo test -p execute-process timeout_kills_hanging_command -q >/dev/null 2>&1 ) || true
after=$(count_orphans)
if [[ "$after" -le "$before" ]]; then
  pass LAB-CA5
else
  fail_msg LAB-CA5 "huérfanos: antes=$before después=$after"
fi

# LAB-CA10: gate dirty-worktree al inicio (sin SKIP_GIT sobre árbol sucio)
if [[ -n "$(git -C "$REPO_ROOT" status --porcelain)" ]]; then
  out3=$(cd "$REPO_ROOT" && env -u SDDIA_LAB_SKIP_GIT SDDIA_LAB_SKIP_PBI_ARCHIVE=1 SDDIA_LAB_SKIP_DELIVERY_CLOSE=1 \
    SDDIA_AGENT_RELAY_IDE=1 ./sddia-run.sh --process feature \
    --inputs '{"feature_name":"smoke-dirty","branch_name":"feat/smoke-dirty","persist_ref":"docs/features/kaizen-feature-lab-init-frictions/.tmp/smoke-dirty"}' 2>&1 || true)
  if echo "$out3" | grep -q 'dirty-worktree'; then
    pass LAB-CA10
  else
    fail_msg LAB-CA10 "gate no disparó: ${out3:0:200}"
  fi
else
  echo "SKIP LAB-CA10 (árbol limpio)"
fi

# LAB-CA11: preservación de ?? docs/todos/ ajenos (cobertura unitaria)
if ( cd "$REPO_ROOT/SddIA" && CARGO_TARGET_DIR="$REPO_ROOT/SddIA/target" \
     cargo test -p execute-process filter_snapshot_commit_files_skips_untracked_todos_ajeno -q >/dev/null 2>&1 ); then
  pass LAB-CA11
else
  fail_msg LAB-CA11 "test de preservación"
fi

# Purga de artefactos de laboratorio
rm -rf "$REPO_ROOT/docs/features/kaizen-feature-lab-init-frictions/.tmp/smoke-relay" \
       "$REPO_ROOT/docs/features/kaizen-feature-lab-init-frictions/.tmp/smoke-eid" \
       "$REPO_ROOT/docs/features/kaizen-feature-lab-init-frictions/.tmp/smoke-dirty"

echo "== resultado: fail=$fail =="
exit "$fail"
